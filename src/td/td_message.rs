use rust_tdlib::types::{Message, MessageContent, TextEntity, TextEntityType, TextEntityTypeTextUrl, TextEntityTypeUrl};

#[derive(Debug)]
pub struct MessageMeta {
    id: i64,
    is_read: bool,
    text: String,
    created_at: i32,
    chat_id: i64,
    url: Option<String>
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
}

pub struct ParseMessageContent {
    text: String,
    url: Option<String>
}
pub fn match_message_content(msg_content: MessageContent) -> std::io::Result<ParseMessageContent> {
    let _path_to_append = "profile.log";
    let mut parsed_content = ParseMessageContent {
        url: None,
        text: String::from("unmatched")
    };
    match msg_content {
        // If video just send only caption
        MessageContent::MessageVideo(mut content) => {
            parsed_content.text = content.caption().text().clone();
            for entity in content.caption().entities() {
                match entity.type_() {
                    TextEntityType::TextUrl(url) => {
                        parsed_content.url = Some(url.url().to_string());
                    },
                    _ => {}
                }
            }
            // info!("{text}");
            // log_append(text.clone(), path_to_append).expect("TODO: panic message");
            // log_append(serde_json::to_string(&last_message)?, path_to_append).expect("TODO: panic message");
        }
        MessageContent::MessagePhoto(content) => {
            parsed_content.text = content.caption().text().clone();
            for entity in content.caption().entities() {
                match entity.type_() {
                    TextEntityType::TextUrl(url) => {
                        parsed_content.url = Some(url.url().to_string());
                    },
                    _ => {}
                }
            }
            // info!("{text}");
            // log_append(text.clone(), path_to_append).expect("TODO: panic message");
            // log_append(serde_json::to_string(&last_message)?, path_to_append).expect("TODO: panic message");
        }
        MessageContent::MessageText(content) => {
            parsed_content.text = content.text().text().to_string();
            for entity in content.text().entities() {
                match entity.type_() {
                    TextEntityType::TextUrl(url) => {
                        parsed_content.url = Some(url.url().to_string());
                    },
                    _ => {}
                }
            }
            // info!("{text}");
            // log_append(text, path_to_append).expect("TODO: panic message");
            // log_append(serde_json::to_string(&last_message)?, path_to_append).expect("TODO: panic message");
        }
        _ => {
            // error!("Unknown message content {:?}", msg_content);
            // log_append(serde_json::to_string(&last_message)?, path_to_append).expect("TODO: panic message");
        }
    }
    Ok(parsed_content)
}
fn get_url_entity(msg_content: MessageContent) {
    // match msg_content {
    //     (content) => {
    //         for
    //     }
    // }
}