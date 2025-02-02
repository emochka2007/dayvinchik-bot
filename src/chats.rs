use rust_tdlib::types::{GetChatHistory, GetChats};
use serde::{Deserialize, Serialize};
use crate::constants::VINCHIK_CHAT;
use crate::tdjson::{send, ClientId};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct Chats {
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_type: String,
    total_count: i32,
    chat_ids: Vec<i32>,
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: String,
    client_id: String,
}

pub fn parse_chats(chats_json: String) -> Chats {
    serde_json::from_str(&chats_json).expect("Get Chats Error Parsing")
}
pub fn get_public_chats(client_id: ClientId) {
    let publicChats = GetChats::builder().limit(100).build();
    let message = serde_json::to_string(&publicChats).unwrap();
    send(client_id, &message);
}
pub fn get_chat_history(client_id: ClientId, chat_id: i64) {
    let message = GetChatHistory::builder().chat_id(chat_id)
        .limit(2).build();
    let chat_history_msg = serde_json::to_string(&message).unwrap();
    send(client_id, &chat_history_msg)
}
