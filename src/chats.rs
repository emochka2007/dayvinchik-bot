use rust_tdlib::types::{GetChat, GetChatHistory, GetChats};
use uuid::Uuid;
use crate::constants::{get_last_message, update_last_tdlib_call};
use crate::pg::pg::PgClient;
use crate::td::td_command_map::Commands::{GetChat as GetChatCommand, GetChatHistory as GetChatHistoryCommand};
use crate::td::td_command_map::ResponseKeys;
use crate::td::td_manager::Task;
use crate::td::td_message::MessageMeta;
use crate::td::td_json::{send, ClientId};

#[derive(Debug, Clone)]
pub struct ChatMeta {
    client_id: ClientId,
    chat_id: i64,
    last_message: MessageMeta,
}
impl ChatMeta {
    pub fn new(client_id: ClientId, chat_id: i64, last_message: MessageMeta) -> Self {
        Self {
            client_id,
            chat_id,
            last_message,
        }
    }
    pub fn _last_message_text(&self) -> &String {
        &self.last_message.text()
    }
    pub async fn insert_db(&self, client: &PgClient) -> () {
        let id = &Uuid::new_v4();
        let query = "INSERT INTO chats (\
        id, \
        chat_id \
    VALUES ($1,$2) ON CONFLICT (chat_id) \
    DO NOTHING";
        client.query(query, &[id, &self.chat_id]).await.unwrap();
    }
}

pub async fn td_get_chats(client_id: ClientId, pg_client: &PgClient) {
    let public_chats = GetChats::builder().limit(100).build();
    let message = serde_json::to_string(&public_chats).unwrap();
    let task = Task::new(message, ResponseKeys::ChatIds);
    task.insert_db(pg_client).await.unwrap();
}

pub fn td_chat_history(client_id: ClientId, chat_id: i64, limit: i32) {
    let last_msg_id = get_last_message();
    let message = GetChatHistory::builder()
        .chat_id(chat_id)
        .from_message_id(last_msg_id)
        .limit(limit).build();
    let chat_history_msg = serde_json::to_string(&message).unwrap();
    send(client_id, &chat_history_msg);
    update_last_tdlib_call(GetChatHistoryCommand);
}
pub fn td_chat_info(client_id: ClientId, chat_id: i64) {
    let message = GetChat::builder().chat_id(chat_id).build();
    let chat_history_msg = serde_json::to_string(&message).unwrap();
    send(client_id, &chat_history_msg);
    update_last_tdlib_call(GetChatCommand);
}
