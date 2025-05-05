use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::fmt::Display;
use std::fs::File;
use std::path::Path;

#[derive(Deserialize, Serialize, Debug)]
pub struct RawChat {
    name: String,
    r#type: String,
    id: u32,
    messages: Vec<RawMessage>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RawMessage {
    id: i32,
    r#type: String,
    date: String,
    date_unixtime: String,
    from: String,
    from_id: String,
    text: String,
    text_entities: Vec<RawTextEntity>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RawTextEntity {
    r#type: String,
    text: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PrivateDialogue {
    messages: Vec<PrivateMessage>,
}

impl PrivateDialogue {
    pub fn add_private_message(&mut self, msg: PrivateMessage) {
        self.messages.push(msg);
    }

    pub fn get_last_message(&mut self) -> Option<&mut PrivateMessage> {
        let last = self.messages.last_mut();
        match last {
            Some(last) => Some(last),
            None => None,
        }
    }
}

impl PrivateDialogue {
    fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
enum PrivateMessageRole {
    USER,
    ASSISTANT,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PrivateMessage {
    content: String,
    role: String,
}
impl Display for PrivateMessageRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            PrivateMessageRole::ASSISTANT => "assistant".to_string(),
            PrivateMessageRole::USER => "user".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl PrivateMessage {
    pub fn concat_text(&mut self, text: &str) {
        self.content = format!("{}\n{}", self.content, text);
    }
}

pub fn transform_private_messages() -> Vec<PrivateDialogue> {
    let json_file_path = Path::new("example.json");
    let file = File::open(json_file_path).unwrap();
    let chat: RawChat = serde_json::from_reader(file).unwrap();
    println!("Chat -> {:?}", chat);
    let mut dialogues: Vec<PrivateDialogue> = Vec::new();
    for _ in 1..=10 {
        let mut dialogue = PrivateDialogue::new();
        for message in &chat.messages {
            // Sort messages by time for a single dialogue
            let role = match message.from.as_str() {
                "Maxim" => "assistant".to_string(),
                _ => "user".to_string(),
            };
            let last_message = dialogue.get_last_message();
            match last_message {
                Some(last) => {
                    // Check role
                    match last.role == role {
                        true => {
                            &last.concat_text(&message.text);
                        }
                        false => {
                            dialogue.add_private_message(PrivateMessage {
                                content: message.text.to_string(),
                                role,
                            });
                        }
                    }
                }
                None => {
                    dialogue.add_private_message(PrivateMessage {
                        content: message.text.to_string(),
                        role,
                    });
                }
            }
        }
        dialogues.push(dialogue)
    }
    dialogues
}
