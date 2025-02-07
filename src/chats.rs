use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use rust_tdlib::types::{Chats, GetChat, GetChatHistory, GetChats};
use crate::constants::get_last_message;
use crate::td::td_message::MessageMeta;
use crate::td::tdjson::{send, ClientId};

pub type UnreadChats = Arc<Mutex<HashMap<i64, ChatMeta>>>;

pub struct ChatMeta  {
    chat_id: i64,
    last_message: MessageMeta,
}
impl ChatMeta {
    pub fn new(chat_id: i64, last_message: MessageMeta) -> Self {
        Self {
            chat_id,
            last_message
        }
    }
    pub fn last_message_text(&self) -> &String {
        &self.last_message.text()
    }
}

pub fn parse_chats(chats_json: String) -> Chats {
    serde_json::from_str(&chats_json).expect("Get Chats Error Parsing")
}
pub fn get_public_chats(client_id: ClientId) {
    let publicChats = GetChats::builder().limit(100).build();
    let message = serde_json::to_string(&publicChats).unwrap();
    send(client_id, &message);
}
pub fn get_messages(client_id: ClientId, chat_id: i64, limit: i32) {
    let last_msg_id = get_last_message();
        let message = GetChatHistory::builder()
            .chat_id(chat_id)
            .from_message_id(last_msg_id)
            // .offset(i * -1)
            .limit(limit).build();
        let chat_history_msg = serde_json::to_string(&message).unwrap();
        log::debug!("{chat_history_msg}");
        send(client_id, &chat_history_msg)
}
pub fn get_chat_info(client_id: ClientId, chat_id: i64) {
    let message = GetChat::builder().chat_id(chat_id).build();
    let chat_history_msg = serde_json::to_string(&message).unwrap();
    send(client_id, &chat_history_msg)
}

//return none if not match and link if it does
pub fn identify_match(text: &str) -> std::io::Result<String> {
    let match_to_contain = "Есть взаимная симпатия!";
    // if text.contains(match_to_contain) {
    //
    // }
    Ok(match_to_contain.to_string())
}