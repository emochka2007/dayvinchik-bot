use std::collections::HashMap;
use std::time::Duration;
use rust_tdlib::types::Chat;
use rust_tdlib::types::LoginUrlInfo::Open;
use crate::chats::{get_messages, get_public_chats};
use crate::constants::VINCHIK_CHAT;
use crate::message::{CustomGetMe, SendMessage};
use crate::openapi::llm_api::OpenAI;
use crate::superlike::SuperLike;
use crate::td::td_file::td_file_download;
use crate::td::tdjson::{send, ClientId};
use crate::UnreadChats;

pub async fn match_input(input: String, client_id: ClientId, unread_chats: &UnreadChats) {
    println!("input - {input}");
    match input.to_uppercase().as_str().trim() {
        "O" => {
            let open_ai = OpenAI::new();
            let review = open_ai.generate_response("none".to_string()).await;
        }
        "C" => {
            // get_chat_info(client_id, VINCHIK_CHAT);
            get_public_chats(client_id);
        }
        "R" => {
            // Latest message id read
            // get_messages(client_id, VINCHIK_CHAT.parse::<i64>().unwrap(), 2);
        }
        // Get Last 100 messages from Vinchik chat
        "L" => {
            get_messages(client_id, VINCHIK_CHAT.parse::<i64>().unwrap(), 100);
        }
        // Flow of superlike
        // Send superlike message
        // Gets the data and image
        // Ask openai for superlike_message
        // Send superlike_message
        "S" => {
            let constructed_message = SendMessage::super_like(VINCHIK_CHAT);
            let message = serde_json::to_string(&constructed_message).unwrap();
            send(client_id, &message);

            tokio::time::sleep(Duration::new(2, 0)).await;
            // Send
            let superlikes = SuperLike::get_from_file().unwrap();
            let text_to_use = superlikes.cute();
            let superlike_message = SendMessage::text_message(text_to_use, VINCHIK_CHAT);
            let message = serde_json::to_string(&superlike_message).unwrap();
            send(client_id, &message);
        }
        // Send messages to the unread chats
        "A" => {
            for (id, chat) in unread_chats.lock().unwrap().iter() {
                let open_ai = OpenAI::new();
                let ai_response = open_ai.generate_response(chat.last_message_text().to_string()).await.expect("TODO: panic message");
                log::debug!("Ai response {}", ai_response);
                // let constructed_message = SendMessage::text_message("s", id.to_string().as_str());
                // let message = serde_json::to_string(&constructed_message).unwrap();
                // send(client_id, &message);
            }
        }
        "M" => {
            let get_me_msg = &CustomGetMe::builder();
            let message = serde_json::to_string(&get_me_msg).unwrap();
            send(client_id, &message);
        }
        "F" => {
            let download_msg = td_file_download(1230);
            let message = serde_json::to_string(&download_msg).unwrap();
            send(client_id, &message);
        }
        _ => {}
    }
}