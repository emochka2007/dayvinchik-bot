use tokio::time::{sleep, Duration};
use log::{debug, error, info};
use rust_tdlib::types::{SearchPublicChat};
use serde_json::Error;
use crate::chats::{td_chat_history, td_chat_info, td_get_chats};
use crate::constants::{update_last_tdlib_call, VINCHIK_CHAT};
use crate::entities::profile_reviewer::ProfileReviewer;
use crate::file::{image_to_base64, move_file};
use crate::message::{SendMessage};
use crate::openapi::llm_api::OpenAI;
use crate::pg::pg::PgClient;
use crate::prompts::Prompt;
use crate::superlike::SuperLike;
use crate::td::td_file::td_file_download;
use crate::td::td_json::{send, ClientId};
use crate::td::td_manager::TdManager;

pub async fn match_input(input: String, client_id: ClientId, pg_client: &PgClient) -> Result<(), Error> {
    info!("input - {input}");
    let VINCHIK_i64 = VINCHIK_CHAT.parse::<i64>().unwrap();
    let td_manager = TdManager::init(client_id);
    match input.to_uppercase().as_str().trim() {
        // main flow with analyze
        // Send /start - STARTED
        // Send 1 - VIEW_PROFILES
        // Get photo and store - GETTING PHOTO
        // Get profile and store - GETTING_PROFILE
        // Ask gpt with prompt - ASKING_LLM
        // Receive number and store it. - APPROVED
        //todo what we implement here
        "C" => {
            td_get_chats(pg_client).await;
        }
        "E" => {
            // td_manager.send_request(pg_client).await.unwrap();
        }
        "_" => {
            let start_message = SendMessage::text_message("/start", VINCHIK_CHAT);
            let message = serde_json::to_string(&start_message)?;
            send(client_id, &message);
            // // todo check if answer received
            sleep(Duration::from_secs(1)).await;
            let view_profiles = SendMessage::text_message("1", VINCHIK_CHAT);
            let message = serde_json::to_string(&view_profiles)?;
            send(client_id, &message);
        }
        "D" => {
            let view_profiles = SendMessage::skip(VINCHIK_CHAT);
            let message = serde_json::to_string(&view_profiles)?;
            send(client_id, &message);
        }
        "X" => {
            //todo get waiting here
            loop {
                match ProfileReviewer::acquire(pg_client).await {
                    Ok(value) => {
                        let last_pending = ProfileReviewer::start(pg_client).await.unwrap();
                        let open_ai = OpenAI::new();
                        let prompt = Prompt::analyze_alt();
                        let path_to_img = format!("profile_images/{}.png", last_pending.main_file());
                        // debug!("{path_to_img}");
                        let base64_image = image_to_base64(&path_to_img).unwrap();
                        // info!("base64 {}", base64_image);
                        let response = open_ai.send_sys_image_message(prompt.system.unwrap(), prompt.user, base64_image).await.unwrap();
                        match response.parse::<i32>() {
                            Ok(score) => {
                                last_pending.finalize(pg_client, score).await.expect("TODO: panic message");
                                let reviewed_file = format!("reviewed_images/{}.png", last_pending.id());
                                move_file(&path_to_img, &reviewed_file).expect("TODO: panic message");
                            }
                            Err(e) => { error!("Response parsing error {:?}", e) }
                        }
                    }
                    Err(e) => {
                        error!("Acquiring error {:?}", e)
                    }
                }
                sleep(Duration::from_secs(2)).await;
            }
        }
        // Get Last 100 messages from Vinchik chat
        "L" => {
            td_chat_history(client_id, VINCHIK_i64, 100);
        }
        // Flow of superlike
        // Send superlike message
        // Gets the data and image
        // Ask openai for superlike_message
        // Send superlike_message
        "S" => {
            let constructed_message = SendMessage::super_like(VINCHIK_CHAT);
            let message = serde_json::to_string(&constructed_message)?;
            send(client_id, &message);

            sleep(Duration::new(2, 0)).await;
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
            // let get_me_msg = &CustomGetMe::builder();
            // let message = serde_json::to_string(&get_me_msg).unwrap();
            // send(client_id, &message);
        }
        "F" => {
            // let download_msg = td_file_download(1230);
            // let message = serde_json::to_string(&download_msg)?;
            // send(client_id, &message);
            // update_last_tdlib_call("DownloadFile".to_string());
        }
        // From Link to chat_id
        "Z" => {
            // let url = "prfckfechenizst";
            // let chat_invite_link = SearchPublicChat::builder().username(url).build();
            // let message = serde_json::to_string(&chat_invite_link)?;
            // send(client_id, &message);
            // update_last_tdlib_call("SearchPublicChat".to_string());
        }
        _ => {}
    }
    Ok(())
}