use log::error;
use rust_tdlib::types::{Message, MessageContent, TextEntity, TextEntityType};
use crate::td::td_file::td_file_download;

#[derive(Debug)]
pub struct MessageMeta {
    id: i64,
    is_read: bool,
    text: String,
    created_at: i32,
    chat_id: i64,
    url: Option<String>,
    file_ids: Option<Vec<i32>>
}

impl MessageMeta {
    pub fn from_message(msg: &Message, last_read_inbox_message_id: Option<i64>) -> Self {
        let mut is_read = true;
        if !msg.is_outgoing() {
            if last_read_inbox_message_id.is_some() && msg.id() > last_read_inbox_message_id.unwrap() {
                is_read = false;
            }
        }
        let parsed_content = match_message_content(msg.content().clone()).unwrap();
        Self {
            id: msg.id(),
            chat_id: msg.chat_id(),
            is_read,
            text: parsed_content.text,
            created_at: msg.date(),
            url: parsed_content.url,
            file_ids: parsed_content.file_ids,
        }
    }
    pub fn is_match(&self) -> bool {
        //todo take from config
        let match_string = "Есть взаимная симпатия!";
        self.text.contains(match_string)
    }
    pub fn id(&self) -> &i64 {
        &self.id
    }
    pub fn url(&self) -> &Option<String> {
        &self.url
    }
    pub fn is_read(&self) -> &bool {
        &self.is_read
    }
    pub fn chat_id(&self) -> &i64 {
        &self.chat_id
    }
    pub fn text(&self) -> &String {
        &self.text
    }
    pub fn file_ids(&self) -> &Option<Vec<i32>> {
        &self.file_ids
    }
}

pub struct ParseMessageContent {
    text: String,
    url: Option<String>,
    file_ids: Option<Vec<i32>>
}
pub fn match_message_content(msg_content: MessageContent) -> std::io::Result<ParseMessageContent> {
    let _path_to_append = "profile.log";
    let mut parsed_content = ParseMessageContent {
        url: None,
        file_ids: None,
        text: String::from("unmatched")
    };
    let mut file_ids = Vec::new();
    match msg_content {
        // If video just send only caption
        MessageContent::MessageVideo(mut content) => {
            parsed_content.text = content.caption().text().clone();
            let entities = content.caption().entities();
            get_url_entity(entities, &mut parsed_content);
        }
        MessageContent::MessagePhoto(content) => {
            parsed_content.text = content.caption().text().clone();
            let largest_size = content.photo().sizes().last().unwrap();
            file_ids.push(largest_size.photo().id());
            let entities = content.caption().entities();
            get_url_entity(entities, &mut parsed_content);
        }
        MessageContent::MessageText(content) => {
            parsed_content.text = content.text().text().to_string();
            let entities = content.text().entities();
            get_url_entity(entities, &mut parsed_content);
        }
        _ => {
            error!("Unknown message content {:?}", msg_content);
        }
    }
    parsed_content.file_ids = Some(file_ids);
    Ok(parsed_content)
}
fn get_url_entity(entities: &Vec<TextEntity>, content: &mut ParseMessageContent) {
    for entity in entities {
        match entity.type_() {
            TextEntityType::TextUrl(url) => {
                content.url = Some(url.url().to_string());
            },
            _ => {}
        }
    }
}