use crate::common::{BotError, ChatId, FileId, MessageId};
use crate::entities::profile_match::ProfileMatch;
use crate::entities::profile_reviewer::{ProfileReviewer, ProfileReviewerStatus};
use crate::pg::pg::{DbQuery, PgClient};
use crate::td::td_file::td_file_download;
use crate::td::td_manager::Task;
use crate::td::td_request::RequestKeys;
use crate::td::td_response::ResponseKeys;
use log::{debug, error};
use rust_tdlib::types::{
    GetChatHistory, Message, MessageContent, Messages, TextEntity, TextEntityType,
};
use serde_json::Value;
use tokio_postgres::Error;
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
}

impl MessageMeta {
    pub fn from_message(
        msg: &Message,
        last_read_inbox_message_id: Option<MessageId>,
    ) -> Result<Self, BotError> {
        let mut is_read = true;
        if !msg.is_outgoing()
            && last_read_inbox_message_id.is_some()
            && msg.id() > last_read_inbox_message_id.unwrap()
        {
            is_read = false;
        }
        let parsed_content = match_message_content(msg.content().clone())?;
        Ok(Self {
            id: Uuid::new_v4().to_string(),
            message_id: msg.id(),
            chat_id: msg.chat_id(),
            is_read,
            text: parsed_content.text,
            created_at: msg.date(),
            url: parsed_content.url,
            file_ids: parsed_content.file_ids,
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
    pub async fn insert_db(&self, client: &PgClient) -> Result<(), Error> {
        let id = &Uuid::parse_str(&self.id).unwrap();
        let query = "INSERT INTO messages (\
        id, \
        chat_id,\
        message_id,\
        is_read,\
        text,\
        url)\
        VALUES ($1, $2, $3, $4, $5, $6)";
        client
            .query(
                query,
                &[
                    &id,
                    &self.chat_id,
                    &self.message_id,
                    &self.is_read,
                    &self.text,
                    &self.url,
                ],
            )
            .await?;
        Ok(())
    }
}

pub struct ParseMessageContent {
    text: String,
    url: Option<String>,
    file_ids: Option<Vec<i32>>,
}
pub fn match_message_content(msg_content: MessageContent) -> std::io::Result<ParseMessageContent> {
    let _path_to_append = "profile.log";
    let mut parsed_content = ParseMessageContent {
        url: None,
        file_ids: None,
        text: String::from("unmatched"),
    };
    let mut file_ids = Vec::new();
    debug!("110: {:?}", msg_content);
    match msg_content {
        // If video just send only caption
        MessageContent::MessageVideo(content) => {
            parsed_content.text = content.caption().text().clone();
            let entities = content.caption().entities();
            get_url_entity(entities, &mut parsed_content);
        }
        MessageContent::MessagePhoto(content) => {
            parsed_content.text = content.caption().text().clone();
            // debug!("{:?}", content.photo().sizes());
            // let smallest_photo = content.photo().sizes().first().unwrap();
            // file_ids.push(smallest_photo.photo().id());
            let largest_size = content.photo().sizes().last().unwrap();
            file_ids.push(largest_size.photo().id());
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
        match entity.type_() {
            TextEntityType::TextUrl(url) => {
                content.url = Some(url.url().to_string());
            }
            _ => {}
        }
    }
}

pub async fn chat_history(json_str: Value, pg_client: &PgClient) -> Result<(), BotError> {
    let messages: Messages = serde_json::from_value(json_str)?;
    error!("messages {:?}", messages);
    for message in messages.messages() {
        match message.as_ref() {
            Some(message) => {
                let parsed_message = MessageMeta::from_message(message, None)?;
                parsed_message.insert_db(pg_client).await?;
                if parsed_message.is_match() {
                    if let Some(url) = &parsed_message.url {
                        let profile_match = ProfileMatch {
                            url: url.to_string(),
                            full_text: parsed_message.text().to_string(),
                        };
                        profile_match.insert_db(pg_client).await?;
                    }
                }
                error!("Parsed message {:?}", parsed_message);
                match parsed_message.file_ids() {
                    Some(file_ids) => {
                        // Upd: removed check for text, however it's good to verify, for some reason couldn't parse the text
                        if file_ids.len() > 0 {
                            let mut profile_reviewer = ProfileReviewer::new(
                                message.chat_id(),
                                parsed_message.text(),
                                ProfileReviewerStatus::PENDING,
                            );
                            profile_reviewer.set_file_ids(file_ids.clone());
                            profile_reviewer.insert(pg_client).await?;
                            td_file_download(pg_client, profile_reviewer.main_file()).await?;
                        }
                    }
                    None => {}
                }
            }
            None => {}
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
