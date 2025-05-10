use chrono::Local;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::env;
use std::fmt::Display;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use anyhow::Result;
use crate::autoresponder::types::{ExportedChat, TextOrTextList};

mod types;


#[derive(Deserialize, Serialize, Debug)]
pub struct PrivateDialogue {
    messages: Vec<PrivateMessage>,
}

impl PrivateDialogue {
    pub fn add_private_message(&mut self, msg: PrivateMessage) {
        self.messages.push(msg);
    }

    pub fn get_last_message(&mut self) -> Option<&mut PrivateMessage> {
        self.messages.last_mut()
    }
    fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }
    fn check_for_last_message(&mut self) -> anyhow::Result<()> {
        let last = self.get_last_message();
        let last = match last {
            Some(last) => last,
            None => {
                return Ok(());
            }
        };
        // Check if last message is being sent out by assistant
        match last.role.as_str() {
            "assistant" => {
                // do nothing
                Ok(())
            }
            _ => {
                let private_message = PrivateMessage {
                    content: "".to_string(),
                    role: "assistant".to_string(),
                };
                self.add_private_message(private_message);
                Ok(())
            }
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
    pub fn concat_text(&mut self, text: &TextOrTextList) {
        match text {
            TextOrTextList::Single(single) => {
                self.content = format!("{}\n{}", self.content, single);
            }
            TextOrTextList::List(list) => {
                self.content = format!("{}\n{:?}", self.content, list);
            }
        }
    }
}

// Create jsonl file
pub fn create_jsonl_file(path: &str) -> Result<()> {
    let private_dialogues = transform_private_messages(path);

    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let file_name = format!("parsed_jsonl/{}.jsonl", timestamp);
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_name)?;

    for (i, d) in private_dialogues.iter().enumerate() {
        let line = serde_json::to_string(d)?;
        if i == private_dialogues.len() - 1 {
            // Last element: write without newline
            file.write_all(line.as_bytes())?;
        } else {
            // Other elements: write with newline
            writeln!(file, "{}", line)?;
        }
    }
    Ok(())
}

pub fn transform_private_messages(path: &str) -> Vec<PrivateDialogue> {
    let json_file_path = Path::new(path);
    let file = File::open(json_file_path).unwrap();
    let chat: ExportedChat = serde_json::from_reader(file).unwrap();
    let mut dialogues: Vec<PrivateDialogue> = Vec::new();
    let assistant_name = env::var("TG_USERNAME").unwrap();
    for _ in 1..=10 {
        let mut dialogue = PrivateDialogue::new();
        for message in &chat.messages {
            // Sort messages by time for a single dialogue
            let role = match &message.from {
                Some(from_name) if from_name == &assistant_name => "assistant".to_string(),
                Some(_) => "user".to_string(),
                None => continue,
            };
            let last_message = dialogue.get_last_message();
            let text = &message.text.to_string();
            match last_message {
                Some(last) => {
                    // Check role
                    match last.role == role {
                        true => {
                            last.concat_text(&message.text);
                        }
                        false => {
                            dialogue.add_private_message(PrivateMessage {
                                content: text.clone(),
                                role,
                            });
                        }
                    }
                }
                None => {
                    dialogue.add_private_message(PrivateMessage {
                        content: text.clone(),
                        role,
                    });
                }
            }
        }
        dialogue.check_for_last_message().expect("todo error");
        dialogues.push(dialogue)
    }
    dialogues
}
