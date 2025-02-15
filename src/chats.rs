use log::debug;
use rust_tdlib::types::{Chat, GetChat as TdGetChat, GetChatHistory as TdGetChatHistory, GetChats};
use serde_json::{Value, Error as SerdeError};
use tokio_postgres::Error;
use uuid::Uuid;
use crate::common::{ChatId, MessageId};
use crate::constants::{get_last_message, update_last_tdlib_call};
use crate::pg::pg::PgClient;
use crate::td::td_manager::Task;
use crate::td::td_json::{send, ClientId};
use crate::td::td_message::MessageMeta;
use crate::td::td_request::RequestKeys;
use crate::td::td_request::RequestKeys::{GetChat, GetChatHistory};
use crate::td::td_response::ResponseKeys;

#[derive(Debug, Clone)]
pub struct ChatMeta {
    id: Uuid,
    chat_id: ChatId,
    last_read_message_id: MessageId,
}
impl ChatMeta {
    pub fn new(chat_id: ChatId, last_read_message_id: MessageId) -> Self {
        Self {
            id: Uuid::new_v4(),
            chat_id,
            last_read_message_id,
        }
    }
    pub async fn insert_db(&self, client: &PgClient) -> () {
        let query = "INSERT INTO chats (\
        id, \
        chat_id, \
        last_read_message_id) \
    VALUES ($1,$2, $3) ON CONFLICT (chat_id) \
   DO UPDATE SET last_read_message_id = EXCLUDED.last_read_message_id";
        client.query(query, &[&self.id, &self.chat_id, &self.last_read_message_id]).await.unwrap();
    }
    pub async fn select_by_chat_id(chat_id: i64, client: &PgClient) -> Result<Self, Error> {
        let query = "SELECT * from chats WHERE chat_id = $1 LIMIT 1";
        let row = client.query_one(query, &[&chat_id]).await?;
        Ok(Self {
            id: row.try_get("id")?,
            last_read_message_id: row.try_get("last_read_message_id")?,
            chat_id: row.try_get("chat_id")?,
        })
    }
}

pub async fn td_get_chats(pg_client: &PgClient) {
    let public_chats = GetChats::builder().limit(100).build();
    let message = serde_json::to_string(&public_chats).unwrap();
    let task = Task::new(message, ResponseKeys::Chats, RequestKeys::GetChats);
    task.insert_db(pg_client).await.unwrap();
}

pub fn td_chat_history(client_id: ClientId, chat_id: i64, limit: i32) {
    let last_msg_id = get_last_message();
    let message = TdGetChatHistory::builder()
        .chat_id(chat_id)
        .from_message_id(last_msg_id)
        .limit(limit).build();
    let chat_history_msg = serde_json::to_string(&message).unwrap();
    send(client_id, &chat_history_msg);
    update_last_tdlib_call(GetChatHistory);
}
pub async fn td_chat_info(pg_client: &PgClient, chat_id: ChatId) {
    let message = TdGetChat::builder().chat_id(chat_id).build();
    let chat_history_msg = serde_json::to_string(&message).unwrap();
    // update_last_tdlib_call(GetChat);
    let task = Task::new(chat_history_msg, ResponseKeys::Chat, GetChat);
    task.insert_db(pg_client).await.unwrap();
}

pub async fn get_chat(json_str: Value, pg_client: &PgClient) -> Result<ChatMeta, SerdeError> {
    let chat: Chat = serde_json::from_value(json_str)?;
    let chat_meta = ChatMeta::new(chat.id(), chat.last_read_inbox_message_id());
    chat_meta.insert_db(pg_client).await;
    debug!("{:?}", chat_meta);
    Ok(chat_meta)
}