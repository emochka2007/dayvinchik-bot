use crate::common::{BotError, ChatId, MessageId};
use crate::pg::pg::{DbQuery, PgClient};
use crate::td::td_manager::Task;
use crate::td::td_request::RequestKeys;
use crate::td::td_response::ResponseKeys;
use async_trait::async_trait;
use log::{debug, info};
use rust_tdlib::types::{Chat, GetChat as TdGetChat, OpenChat};
use serde_json::Value;
use std::io;
use std::io::ErrorKind;
use tokio_postgres::{Error, Row};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ChatMeta {
    id: Uuid,
    chat_id: ChatId,
    last_read_message_id: MessageId,
    title: String,
    last_message_id: MessageId,
}

#[async_trait]
impl DbQuery for ChatMeta {
    async fn insert<'a>(&'a self, pg_client: &'a PgClient) -> Result<(), BotError> {
        let query = "INSERT INTO chats (\
        id, \
        chat_id, \
        last_read_message_id,\
        last_message_id,\
        title) \
        VALUES ($1,$2, $3, $4,$5) ON CONFLICT (chat_id) \
        DO UPDATE SET last_read_message_id = EXCLUDED.last_read_message_id, \
        last_message_id = EXCLUDED.last_message_id ";
        pg_client
            .query(
                query,
                &[
                    &self.id,
                    &self.chat_id,
                    &self.last_read_message_id,
                    &self.last_message_id,
                    &self.title,
                ],
            )
            .await?;
        Ok(())
    }

    async fn select_one(pg_client: &PgClient, id: Uuid) -> Result<Self, BotError>
    where
        Self: Sized,
    {
        let query = "SELECT * from chats WHERE id = $1 LIMIT 1";
        let row = pg_client.query_one(query, &[&id]).await?;
        Self::from_sql(row)
    }

    fn from_sql(row: Row) -> Result<Self, BotError>
    where
        Self: Sized,
    {
        Ok(Self {
            id: row.try_get("id")?,
            title: row.try_get("title")?,
            last_read_message_id: row.try_get("last_read_message_id")?,
            chat_id: row.try_get("chat_id")?,
            last_message_id: row.try_get("last_message_id")?,
        })
    }
}
impl ChatMeta {
    pub fn chat_id(&self) -> &ChatId {
        &self.chat_id
    }

    pub fn last_read_message_id(&self) -> &MessageId {
        &self.last_read_message_id
    }
    pub fn last_message_id(&self) -> &MessageId {
        &self.last_message_id
    }

    pub fn new(
        chat_id: ChatId,
        last_read_message_id: MessageId,
        last_message_id: MessageId,
        title: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            chat_id,
            title,
            last_read_message_id,
            last_message_id,
        }
    }
    pub async fn select_by_chat_id(chat_id: i64, client: &PgClient) -> Result<Self, BotError> {
        let query = "SELECT * from chats WHERE chat_id = $1 LIMIT 1";
        let row = client.query_one(query, &[&chat_id]).await?;
        Self::from_sql(row)
    }
}

pub async fn td_chat_info(pg_client: &PgClient, chat_id: ChatId) -> Result<(), BotError> {
    let message = TdGetChat::builder().chat_id(chat_id).build();
    let chat_history_msg = serde_json::to_string(&message)?;
    Task::new(
        chat_history_msg,
        RequestKeys::GetChat,
        ResponseKeys::Chat,
        pg_client,
    )
    .await?;
    Ok(())
}
//todo mb parser for all json struct
pub async fn get_chat(json_str: Value, pg_client: &PgClient) -> Result<Option<ChatMeta>, BotError> {
    let chat: Chat = serde_json::from_value(json_str)?;
    info!("Get chat");
    // if chat.id < 0 - it's a channel we cannot write to
    if chat.id() < 0 {
        return Ok(None);
    }
    match chat.last_message().as_ref() {
        Some(last_message) => {
            let mut last_received_message_id = last_message.id();
            // if message is outgoing, then it means that we've sent
            if last_message.is_outgoing() {
                last_received_message_id = 0;
            }
            let chat_meta = ChatMeta::new(
                chat.id(),
                chat.last_read_inbox_message_id(),
                last_received_message_id,
                chat.title().to_string(),
            );
            chat_meta.insert(pg_client).await?;
            debug!("{:?}", chat_meta);
            Ok(Some(chat_meta))
        }
        None => Err(io::Error::new(ErrorKind::InvalidInput, "Last message not in chat").into()),
    }
}

pub async fn td_open_chat(pg_client: &PgClient, chat_id: ChatId) -> Result<(), Error> {
    let message = OpenChat::builder().chat_id(chat_id).build();
    let chat_history_msg = serde_json::to_string(&message).unwrap();
    Task::new(
        chat_history_msg,
        RequestKeys::OpenChat,
        ResponseKeys::Ok,
        pg_client,
    )
    .await?;
    Ok(())
}
