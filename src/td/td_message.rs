use log::{error, info};
use rust_tdlib::types::{Message, MessageContent};
use crate::file::log_append;

#[derive(Debug)]
pub struct MessageMeta {
    is_read: bool,
    text: String,
    created_at: i32,
}
impl MessageMeta {
    pub fn from_message(msg: &Message, last_read_inbox_message_id: Option<i64>) -> Self {
        let mut is_read = true;
        if last_read_inbox_message_id.is_some() && msg.id() > last_read_inbox_message_id.unwrap() {
            is_read = false;
        }
        let text = match_message_content(msg.content().clone()).unwrap();
        Self {
            is_read,
            text,
            created_at: msg.date(),
        }
    }
}

pub fn match_message_content(msg_content: MessageContent) -> std::io::Result<String> {
    let _path_to_append = "profile.log";
    let mut text = String::from("Unknown format");
    match msg_content {
        // If video just send only caption
        MessageContent::MessageVideo(mut content) => {
            text = content.caption().text().clone();
            info!("{text}");
            // log_append(text.clone(), path_to_append).expect("TODO: panic message");
            // log_append(serde_json::to_string(&last_message)?, path_to_append).expect("TODO: panic message");
        }
        MessageContent::MessagePhoto(content) => {
            text = content.caption().text().clone();
            info!("{text}");
            // log_append(text.clone(), path_to_append).expect("TODO: panic message");
            // log_append(serde_json::to_string(&last_message)?, path_to_append).expect("TODO: panic message");
        }
        MessageContent::MessageText(content) => {
            text = content.text().text().to_string();
            info!("{text}");
            // log_append(text, path_to_append).expect("TODO: panic message");
            // log_append(serde_json::to_string(&last_message)?, path_to_append).expect("TODO: panic message");
        }
        _ => {
            error!("Unknown message content {:?}", msg_content);
            // log_append(serde_json::to_string(&last_message)?, path_to_append).expect("TODO: panic message");
        }
    }
    Ok(text)
}