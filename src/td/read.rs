use log::{error, info};
use rust_tdlib::types::{Chat, Chats, MessageContent, Messages};
use serde_json::Value;
use crate::chats::{get_chat_info, get_messages};
use crate::file::{file_log, log_append};
use crate::td::td_message::{match_message_content, MessageMeta};
use crate::td::tdjson::ClientId;

pub fn parse_message(json_str: &str, client_id: ClientId) -> std::io::Result<()> {
    let parsed: Value = serde_json::from_str(json_str)?;
    // Extract the link
    if parsed["authorization_state"].is_string() {
        let link = parsed["authorization_state"]["link"].as_str();
        // println!("✅ Extracted Telegram Link: {}", link);
        // generate_qr_code(link);
    } else if parsed["chat_ids"].is_array() {
        let chats: Chats = serde_json::from_value(parsed)?;
        let chats_list = chats.chat_ids();
        for chat in chats_list {
            let chat_info = get_chat_info(client_id, *chat);
            get_messages(client_id, *chat, 1);
        }
        file_log(serde_json::to_string(&chats)?);
    } else if parsed["@type"] == "messages" {
        let messages: Messages = serde_json::from_value(parsed)?;
        let last_message = messages.messages().get(0).unwrap();
        let last_message = last_message.clone().unwrap();
        let parsed_message = MessageMeta::from_message(&last_message, None);
        log::debug!("{:?}", parsed_message);
    } else if parsed["@type"] == "chat" {
        let chat: Chat = serde_json::from_value(parsed)?;
        let last_message = chat.last_message().as_ref().unwrap().clone();
        let parsed_message = MessageMeta::from_message(&last_message, Some(chat.id()));
        log::debug!("{:?}", parsed_message);
    }
    Ok(())
}