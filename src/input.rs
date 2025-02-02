use std::time::Duration;
use crate::chats::{get_chat_history, get_public_chats};
use crate::constants::VINCHIK_CHAT;
use crate::llm_api::OpenAI;
use crate::message::{CustomGetMe, SendMessage};
use crate::superlike::SuperLike;
use crate::td_file::td_file_download;
use crate::tdjson::{send, ClientId};

pub async fn match_input(input: String, client_id: ClientId) {
    println!("input - {input}");
    match input.to_uppercase().as_str().trim() {
        "O" => {
            let open_ai = OpenAI::new();
            let review = open_ai.profile_check("none".to_string()).await;
        }
        "C" => {
            get_public_chats(client_id);
        }
        "R" => {
            // Latest message id read
            get_chat_history(client_id, VINCHIK_CHAT);
        }
        "L" => {
            let constructed_message = SendMessage::like();
            let message = serde_json::to_string(&constructed_message).unwrap();
            send(client_id, &message);
            tokio::time::sleep(Duration::new(1, 0)).await;
            get_chat_history(client_id, VINCHIK_CHAT);
        }
        // Flow of superlike
        // Send superlike message
        // Gets the data and image
        // Ask openai for superlike_message
        // Send superlike_message
        "S" => {
            let constructed_message = SendMessage::super_like();
            let message = serde_json::to_string(&constructed_message).unwrap();
            send(client_id, &message);

            tokio::time::sleep(Duration::new(2, 0)).await;
            // Send
            let superlikes = SuperLike::get_from_file().unwrap();
            let text_to_use = superlikes.nyash.as_str();
            let superlike_message = SendMessage::text_message(text_to_use);
            let message = serde_json::to_string(&superlike_message).unwrap();
            send(client_id, &message);
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