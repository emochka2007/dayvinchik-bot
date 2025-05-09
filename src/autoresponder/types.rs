use clap::builder::Str;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ExportedChat {
    pub name: String,
    #[serde(rename = "type")]
    pub chat_type: String,
    pub id: i64,
    pub messages: Vec<Message>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub id: i64,
    #[serde(rename = "type")]
    pub message_type: String,
    pub date: String,
    pub date_unixtime: String,
    pub from: Option<String>,
    pub from_id: Option<String>,
    pub text: TextOrTextList,
    pub text_entities: Vec<TextEntity>,
    pub edited: Option<String>,
    pub edited_unixtime: Option<String>,
    pub reply_to_message_id: Option<i64>,

    // Optional media fields
    pub file: Option<String>,
    pub file_name: Option<String>,
    pub file_size: Option<u64>,
    pub photo: Option<String>,
    pub photo_file_size: Option<u64>,
    pub thumbnail: Option<String>,
    pub thumbnail_file_size: Option<u64>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub duration_seconds: Option<u32>,
    pub media_type: Option<String>,
    pub sticker_emoji: Option<String>,
    pub mime_type: Option<String>,

    pub reactions: Option<Vec<Reaction>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TextOrTextList {
    Single(String),
    List(Vec<TextOrTextPart>),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TextOrTextPart {
    Single(String),
    TextPart(TextPart),
}

impl TextOrTextList {
    pub fn to_string(&self) -> String {
        match self {
            TextOrTextList::Single(single) => {
                single.clone()
            }
            TextOrTextList::List(list) => {
                let mut joined_text = String::new();
                for text in list {
                    match text {
                        TextOrTextPart::Single(single) => {
                            joined_text.push_str(single)
                        }
                        TextOrTextPart::TextPart(text_part) => {
                            joined_text.push_str(text_part.text.as_str())
                        }
                    }
                }
                joined_text
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TextPart {
    #[serde(rename = "type")]
    pub part_type: String,
    pub text: String,
    pub document_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TextEntity {
    #[serde(rename = "type")]
    pub entity_type: String,
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Reaction {
    #[serde(rename = "type")]
    pub reaction_type: String,
    pub count: u32,
    pub emoji: String,
    pub recent: Vec<RecentReaction>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RecentReaction {
    pub from: String,
    pub from_id: String,
    pub date: String,
}
