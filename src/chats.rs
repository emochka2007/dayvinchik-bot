use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use rust_tdlib::types::{GetChat, GetChatHistory, GetChats};
use crate::constants::{get_last_message, update_last_tdlib_call};
use crate::pg::pg::PgClient;
use crate::td::td_message::MessageMeta;
use crate::td::tdjson::{send, ClientId};

pub type UnreadChats = Arc<Mutex<HashMap<i64, ChatMeta>>>;

#[derive(Debug)]
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
    pub fn last_message_text(&self) -> &String {
        &self.last_message.text()
    }
    pub async fn insert_db(&self, client: &PgClient) -> () {
        let query = "INSERT INTO chats (td_client_id, td_chat_id, is_read) \
    VALUES ($1,$2,$3) ON CONFLICT (td_chat_id) \
    DO UPDATE SET is_read=$3";
        let client_id = self.client_id as i64;
        client.query(query, &[&client_id, &self.chat_id,
            self.last_message.is_read()]).await.unwrap();
    }
}

pub fn td_get_chats(client_id: ClientId) {
    let publicChats = GetChats::builder().limit(100).build();
    let message = serde_json::to_string(&publicChats).unwrap();
    send(client_id, &message);
    update_last_tdlib_call("GetChats".to_string());
}

pub fn td_chat_history(client_id: ClientId, chat_id: i64, limit: i32) {
    let last_msg_id = get_last_message();
    let message = GetChatHistory::builder()
        .chat_id(chat_id)
        .from_message_id(last_msg_id)
        .limit(limit).build();
    let chat_history_msg = serde_json::to_string(&message).unwrap();
    send(client_id, &chat_history_msg);
    update_last_tdlib_call("GetChatHistory".to_string());
}
pub fn td_chat_info(client_id: ClientId, chat_id: i64) {
    let message = GetChat::builder().chat_id(chat_id).build();
    let chat_history_msg = serde_json::to_string(&message).unwrap();
    send(client_id, &chat_history_msg);
    update_last_tdlib_call("GetChat".to_string());
}

//return none if not match and link if it does
pub fn identify_match(text: &str) -> std::io::Result<String> {
    let match_to_contain = "Есть взаимная симпатия!";
    // if text.contains(match_to_contain) {
    //
    // }
    Ok(match_to_contain.to_string())
}
