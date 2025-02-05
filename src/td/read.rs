use rust_tdlib::types::{Chat, Chats, Messages};
use serde_json::Value;
use crate::chats::{get_chat_info, get_messages, ChatMeta};
use crate::file::{file_log, log_append};
use crate::td::td_message::{match_message_content, MessageMeta};
use crate::td::tdjson::ClientId;
use crate::UnreadChats;

pub fn parse_message(json_str: &str, client_id: ClientId, mut unread_chats: &UnreadChats) -> std::io::Result<()> {
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
        // log::debug!("{:?}", messages);
        // log::info!("{}", json_str);
        for message in messages.messages() {
            let message = message.as_ref().unwrap();
            let parsed_message = MessageMeta::from_message(message, None);
        }
    } else if parsed["@type"] == "chat" {
        let chat: Chat = serde_json::from_value(parsed)?;
        let last_message = chat.last_message().as_ref().unwrap().clone();
        let parsed_message = MessageMeta::from_message(&last_message, Some(chat.last_read_inbox_message_id()));
        // log::error!("l_m.chat_id {} l_m.id {} chat.id {:?} c.l_r_i {} title {} is_read {} msg_outgoing {}", last_message.chat_id(), last_message.id(),
        //     chat.id(), chat.last_read_inbox_message_id(), chat.title(), parsed_message.is_read(), last_message.is_outgoing());
        if *parsed_message.is_read() == false {
            // todo if chat.id() is in BLOCK_LIST
            let chat_meta = ChatMeta::new(chat.id(), parsed_message);
            unread_chats.lock().unwrap().insert(chat.id(), chat_meta);
        }
    }
    Ok(())
}