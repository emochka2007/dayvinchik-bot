use std::time::Duration;
use log::debug;
use rust_tdlib::types::{ChatInviteLink, SearchPublicChat};
use serde_json::Error;
use tokio::time::sleep;
use crate::chats::{td_chat_history, td_get_chats};
use crate::constants::{update_last_tdlib_call, VINCHIK_CHAT};
use crate::message::{CustomGetMe, SendMessage};
use crate::openapi::llm_api::OpenAI;
use crate::superlike::SuperLike;
use crate::td::td_file::td_file_download;
use crate::td::tdjson::{send, ClientId};

pub async fn match_input(input: String, client_id: ClientId) -> Result<(), Error> {
    println!("input - {input}");
    let vinchik = VINCHIK_CHAT.parse::<i64>().unwrap();
    match input.to_uppercase().as_str().trim() {
        // main flow with analyze
        // Send /start - STARTED
        // Send 1 - VIEW_PROFILES
        // Get photo and store - GETTING PHOTO
        // Get profile and store - GETTING_PROFILE
        // Ask gpt with prompt - ASKING_LLM
        // Receive number and store it. - APPROVED
        "M" => {
            let start_message = SendMessage::text_message("/start", VINCHIK_CHAT);
            let message = serde_json::to_string(&start_message)?;
            send(client_id, &message);
            // todo check if answer received
            sleep(Duration::new(2, 0)).await;
            let view_profiles = SendMessage::text_message("1", VINCHIK_CHAT);
            let message = serde_json::to_string(&view_profiles)?;
            send(client_id, &message);
            sleep(Duration::new(2, 0)).await;
            td_chat_history(client_id, vinchik, 1);

        }
        "O" => {
            let open_ai = OpenAI::new();
            let review = open_ai.send_user_message("none".to_string()).await;
        }
        "C" => {
            // get_chat_info(client_id, VINCHIK_CHAT);
            td_get_chats(client_id);
        }
        "R" => {
            // Latest message id read
            td_chat_history(client_id, vinchik, 2);
        }
        // Get Last 100 messages from Vinchik chat
        "L" => {
            td_chat_history(client_id, vinchik, 100);
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
                // let open_ai = OpenAI::new();
                // let ai_response = open_ai.generate_response(chat.last_message_text().to_string()).await.expect("TODO: panic message");
                // log::debug!("Ai response {}", ai_response);
                // let constructed_message = SendMessage::text_message("s", id.to_string().as_str());
                // let message = serde_json::to_string(&constructed_message).unwrap();
                // send(client_id, &message);
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
            update_last_tdlib_call("DownloadFile".to_string());
        }
        // From Link to chat_id
        "Z" => {
            let url = "prfckfechenizst";
            let chat_invite_link = SearchPublicChat::builder().username(url).build();
            let message = serde_json::to_string(&chat_invite_link).unwrap();
            send(client_id, &message);
            update_last_tdlib_call("SearchPublicChat".to_string());
        }
        _ => {}
    }
    Ok(())
}