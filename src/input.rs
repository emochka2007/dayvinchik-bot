use std::thread::sleep;
use std::time::Duration;
use rust_tdlib::types::{GetChatHistory, GetChats};
use crate::chats::{get_chat_history, get_public_chats};
use crate::constants::VINCHIK_CHAT;
use crate::message::{CustomGetMe, SendMessage};
use crate::td_file::td_file_download;
use crate::tdjson::{send, ClientId};

pub fn match_input(input: String, client_id: ClientId) {
    println!("input - {input}");
    match input.to_uppercase().as_str().trim() {
        "C" => {
            get_public_chats(client_id);
        }
        "R" => {
            // Latest message id read
            get_chat_history(client_id, VINCHIK_CHAT);
        }
        "S" => {
            let sendText = SendMessage::like();
            let message = serde_json::to_string(&sendText).unwrap();
            send(client_id, &message);
            sleep(Duration::new(1, 0));
            get_chat_history(client_id, VINCHIK_CHAT);
        }
        "M" => {
            let getMeMsg = &CustomGetMe::builder();
            let message = serde_json::to_string(&getMeMsg).unwrap();
            send(client_id, &message);
        }
        "F" => {
            let downloadMsg = td_file_download(1230);
            let message = serde_json::to_string(&downloadMsg).unwrap();
            send(client_id, &message);
        }
        _ => {}
    }
}