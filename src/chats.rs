use crate::common::{ChatId, MessageId};
use crate::pg::pg::PgClient;
use crate::td::td_json::{send, ClientId};
use crate::td::td_manager::Task;
use crate::td::td_message::MessageMeta;
use crate::td::td_request::RequestKeys;
use crate::td::td_response::ResponseKeys;
use log::{debug, error, info};
use rust_tdlib::types::{
    Chat, GetChat as TdGetChat, GetChatHistory as TdGetChatHistory, GetChats, Message, OpenChat,
};
use serde_json::{Error as SerdeError, Value};
use tokio_postgres::Error;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ChatMeta {
    id: Uuid,
    chat_id: ChatId,
    last_read_message_id: MessageId,
    title: String,
    last_message_id: MessageId,
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
    pub async fn insert_db(&self, client: &PgClient) -> () {
        let query = "INSERT INTO chats (\
        id, \
        chat_id, \
        last_read_message_id,\
        last_message_id,\
        title) \
    VALUES ($1,$2, $3, $4,$5) ON CONFLICT (chat_id) \
   DO UPDATE SET last_read_message_id = EXCLUDED.last_read_message_id, \
   last_message_id = EXCLUDED.last_message_id ";
        client
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
            .await
            .unwrap();
    }
    pub async fn select_by_chat_id(chat_id: i64, client: &PgClient) -> Result<Self, Error> {
        let query = "SELECT * from chats WHERE chat_id = $1 LIMIT 1";
        let row = client.query_one(query, &[&chat_id]).await?;
        Ok(Self {
            id: row.try_get("id")?,
            title: row.try_get("title")?,
            last_read_message_id: row.try_get("last_read_message_id")?,
            chat_id: row.try_get("chat_id")?,
            last_message_id: row.try_get("last_message_id")?,
        })
    }
}

pub async fn td_get_chats(pg_client: &PgClient) {
    let public_chats = GetChats::builder().limit(100).build();
    let message = serde_json::to_string(&public_chats).unwrap();
    Task::new(
        message,
        RequestKeys::GetChats,
        ResponseKeys::Chats,
        pg_client,
    )
        .await
        .unwrap();
}

pub async fn td_get_last_message(
    pg_client: &PgClient,
    chat_id: ChatId,
    limit: i32,
) -> Result<(), SerdeError> {
    let history_message = TdGetChatHistory::builder()
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
        .await
        .unwrap();
    Ok(())
}

pub async fn td_chat_info(pg_client: &PgClient, chat_id: ChatId) {
    let message = TdGetChat::builder().chat_id(chat_id).build();
    let chat_history_msg = serde_json::to_string(&message).unwrap();
    Task::new(
        chat_history_msg,
        RequestKeys::GetChat,
        ResponseKeys::Chat,
        pg_client,
    )
        .await
        .unwrap();
}

pub async fn get_chat(
    json_str: Value,
    pg_client: &PgClient,
) -> Result<Option<ChatMeta>, SerdeError> {
    let chat: Chat = serde_json::from_value(json_str)?;
    info!("Get chat");
    // if chat.id < 0 - it's a channel we cannot write to
    if chat.id() < 0 {
        return Ok(None);
    }
    let last_message = chat.last_message().as_ref().unwrap();
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
    chat_meta.insert_db(pg_client).await;
    debug!("{:?}", chat_meta);
    Ok(Some(chat_meta))
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
