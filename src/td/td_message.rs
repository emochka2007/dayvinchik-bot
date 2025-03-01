use crate::common::{BotError, ChatId, FileId, MessageId};
use crate::constants::PROCESSED_MESSAGE_IDS;
use crate::entities::dv_bot::DvBot;
use crate::entities::profile_reviewer::{ProcessingStatus, ProfileReviewer};
use crate::entities::superlike::SuperLike;
use crate::entities::task::Task;
use crate::pg::pg::{DbQuery, PgClient};
use crate::td::td_file::td_file_download;
use crate::td::td_request::RequestKeys;
use crate::td::td_response::ResponseKeys;
use async_trait::async_trait;
use log::{debug, error, info};
use rust_tdlib::types::{
    GetChatHistory, Message, MessageContent, Messages, TextEntity, TextEntityType,
};
use serde_json::Value;
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct MessageMeta {
    id: String,
    message_id: MessageId,
    is_read: bool,
    text: String,
    created_at: i32,
    chat_id: ChatId,
    url: Option<String>,
    file_ids: Option<Vec<FileId>>,
    processed: bool,
}

#[async_trait]
impl DbQuery for MessageMeta {
    async fn insert<'a>(&'a self, pg_client: &'a PgClient) -> Result<(), BotError> {
        let id = &Uuid::parse_str(&self.id).unwrap();
        let query = "INSERT INTO messages (\
        id, \
        chat_id,\
        message_id,\
        is_read,\
        text,\
        url,\
        processed) \
        VALUES ($1, $2, $3, $4, $5, $6, $7) \
        ON CONFLICT (message_id) \
        DO NOTHING";
        pg_client
            .query(
                query,
                &[
                    &id,
                    &self.chat_id,
                    &self.message_id,
                    &self.is_read,
                    &self.text,
                    &self.url,
                    &self.processed,
                ],
            )
            .await?;
        Ok(())
    }

    fn from_sql(row: Row) -> Result<Self, BotError>
    where
        Self: Sized,
    {
        Ok(Self {
            id: row.try_get("id")?,
            message_id: row.try_get("message_id")?,
            is_read: row.try_get("is_read")?,
            text: row.try_get("text")?,
            created_at: row.try_get("created_at")?,
            chat_id: row.try_get("chat_id")?,
            url: Some(row.try_get("url")?),
            file_ids: None,
            processed: false,
        })
    }
}

impl MessageMeta {
    pub fn from_message(msg: &Message) -> Result<Self, BotError> {
        let is_read = true;
        let parsed_content = match_message_content(msg.content())?;
        Ok(Self {
            id: Uuid::new_v4().to_string(),
            message_id: msg.id(),
            chat_id: msg.chat_id(),
            is_read,
            text: parsed_content.text,
            created_at: msg.date(),
            url: parsed_content.url,
            file_ids: parsed_content.file_ids,
            processed: false,
        })
    }
    pub fn is_match(&self) -> bool {
        //todo take from config
        let match_string = "Есть взаимная симпатия!";
        self.text.contains(match_string)
    }
    pub fn url(&self) -> &Option<String> {
        &self.url
    }
    pub fn is_read(&self) -> &bool {
        &self.is_read
    }
    pub fn chat_id(&self) -> &ChatId {
        &self.chat_id
    }
    pub fn message_id(&self) -> &MessageId {
        &self.message_id
    }
    pub fn text(&self) -> &String {
        &self.text
    }
    pub fn file_ids(&self) -> &Option<Vec<i32>> {
        &self.file_ids
    }
    pub async fn get_all_unprocessed(pg_client: &PgClient) -> Result<Vec<Self>, BotError> {
        let mut unprocessed_messages = Vec::new();
        let query = "SELECT * from messages WHERE processed <> true AND url IS NOT NULL ";
        let rows = pg_client.query(query, &[]).await?;
        for row in rows {
            unprocessed_messages.push(Self::from_sql(row)?)
        }
        Ok(unprocessed_messages)
    }
    pub async fn process(&self, pg_client: &PgClient) -> Result<(), BotError> {
        let query = "UPDATE messages SET processed = true WHERE id = $1";
        pg_client.query(query, &[&self.id]).await?;
        Ok(())
    }
}

pub struct ParseMessageContent {
    text: String,
    url: Option<String>,
    file_ids: Option<Vec<i32>>,
    local_path: String,
}
impl ParseMessageContent {
    pub fn text(&self) -> &String {
        &self.text
    }
}
pub fn match_message_content(msg_content: &MessageContent) -> std::io::Result<ParseMessageContent> {
    let mut parsed_content = ParseMessageContent {
        url: None,
        file_ids: None,
        text: String::from("unmatched"),
        local_path: String::new(),
    };
    let mut file_ids = Vec::new();
    match msg_content {
        // If video just send only caption
        MessageContent::MessageVideo(content) => {
            parsed_content.text = content.caption().text().clone();
            let entities = content.caption().entities();
            get_url_entity(entities, &mut parsed_content);
        }
        MessageContent::MessagePhoto(content) => {
            parsed_content.text = content.caption().text().clone();
            let largest_size = content.photo().sizes().last().unwrap();
            file_ids.push(largest_size.photo().id());
            let local_path = content
                .photo()
                .sizes()
                .last()
                .unwrap()
                .photo()
                .local()
                .path();
            parsed_content.local_path = local_path.to_string();
            let entities = content.caption().entities();
            get_url_entity(entities, &mut parsed_content);
        }
        MessageContent::MessageText(content) => {
            parsed_content.text = content.text().text().to_string();
            let entities = content.text().entities();
            get_url_entity(entities, &mut parsed_content);
        }
        _ => {
            error!("Unknown message content {:?}", msg_content);
        }
    }
    parsed_content.file_ids = Some(file_ids);
    Ok(parsed_content)
}
fn get_url_entity(entities: &Vec<TextEntity>, content: &mut ParseMessageContent) {
    for entity in entities {
        if let TextEntityType::TextUrl(url) = entity.type_() {
            content.url = Some(url.url().to_string());
        }
    }
}

//todo mb cache memory to skip processed_ids
pub async fn chat_history(json_str: Value, pg_client: &PgClient) -> Result<(), BotError> {
    let messages: Messages = serde_json::from_value(json_str)?;
    debug!("messages {:?}", messages);
    for message in messages.messages() {
        debug!("Message {:?}", message);
        if let Some(message) = message.as_ref() {
            // let processed_messages = PROCESSED_MESSAGE_IDS.blocking_lock();
            // if processed_messages.contains(&message.id()) {
            //     info!("Message skipped {}", message.id());
            //     continue;
            // } else {
            //     // Cache memory for skipping processed messages
            //     PROCESSED_MESSAGE_IDS.blocking_lock().push(message.id());
            // }

            let parsed_message = MessageMeta::from_message(message)?;
            let block_phrases = [
                "✨🔍",
                "Так выглядит твоя анкета:",
                "1",
                "никита, 21",
                "/start",
                "unmatched",
                "Бот знакомств Дайвинчик🍷",
                "Слишком много ❤️ за сегодня.",
            ];
            let is_blocked = block_phrases
                .iter()
                .any(|phrase| parsed_message.text().contains(phrase));

            if is_blocked || parsed_message.text().is_empty() {
                error!("Skipping message {}", parsed_message.text());
                continue;
            }

            parsed_message.insert(pg_client).await?;
            // Check for notification
            if SuperLike::is_superlike_notification(parsed_message.text()) {
                DvBot::send_message(pg_client, "1").await?;
                DvBot::read_last_message(pg_client).await?;
                continue;
            }
            if let Some(file_ids) = parsed_message.file_ids() {
                // Upd: removed check for text, however it's good to verify, for some reason couldn't parse the text
                if !file_ids.is_empty() {
                    let mut profile_reviewer = ProfileReviewer::new(
                        message.chat_id(),
                        parsed_message.text(),
                        ProcessingStatus::Pending,
                    );
                    profile_reviewer.set_file_ids(file_ids.clone());
                    if ProfileReviewer::acquire(pg_client).await?.is_some() {
                        profile_reviewer.insert(pg_client).await?;
                        td_file_download(pg_client, profile_reviewer.main_file().unwrap()).await?;
                    }
                } else {
                    // Send dislike if video or image without image
                    DvBot::send_dislike(pg_client).await?;
                    DvBot::refresh(pg_client).await?;
                    DvBot::read_last_message(pg_client).await?;
                }
            }
        }
    }
    Ok(())
}

pub async fn td_get_last_message(
    pg_client: &PgClient,
    chat_id: ChatId,
    limit: i32,
) -> Result<(), BotError> {
    let history_message = GetChatHistory::builder()
        .chat_id(chat_id)
        .from_message_id(0)
        .limit(limit)
        .build();
    let message = serde_json::to_string(&history_message)?;
    Task::new(
        message,
        RequestKeys::GetChatHistory,
        ResponseKeys::Messages,
        pg_client,
    )
    .await?;
    Ok(())
}
pub async fn td_read_one_from_message_id(
    pg_client: &PgClient,
    chat_id: ChatId,
    message_id: MessageId,
    offset: i32,
) -> Result<(), BotError> {
    let history_message = GetChatHistory::builder()
        .chat_id(chat_id)
        //todo 0
        .from_message_id(13981712384)
        .offset(0)
        .limit(offset)
        .build();
    let message = serde_json::to_string(&history_message)?;
    Task::new(
        message,
        RequestKeys::GetChatHistory,
        ResponseKeys::Messages,
        pg_client,
    )
    .await?;
    Ok(())
}
