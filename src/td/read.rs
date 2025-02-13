use std::thread::sleep;
use std::time::Duration;
use log::{debug, info};
use rust_tdlib::types::{Chat, Chats, Messages, UpdateFile};
use serde_json::Value;
use crate::chats::{td_chat_history, td_chat_info, ChatMeta};
use crate::constants::{get_last_tdlib_call, update_last_message};
use crate::entities::profile_match::{ProfileMatch};
use crate::entities::profile_reviewer::{ProfileReviewer, ProfileReviewerStatus};
use crate::file::{move_file};
use crate::pg::pg::PgClient;
use crate::td::td_command_map::{Commands, ResponseKeys};
use crate::td::td_file::td_file_download;
use crate::td::td_message::{MessageMeta};
use crate::td::td_json::ClientId;

pub async fn parse_message(json_str: &str, client_id: ClientId, pg_client: &PgClient) -> std::io::Result<()> {
    let parsed: Value = serde_json::from_str(json_str)?;
    let td_type = parsed["type"].as_str().unwrap();
    let response_key: ResponseKeys = ResponseKeys::from_str(td_type)?;
    let last_tdlib_call = get_last_tdlib_call();
    match last_tdlib_call {
        Commands::GetChats => {
            if response_key == ResponseKeys::ChatIds {
                info!("GetChats");
                let chats: Chats = serde_json::from_value(parsed)?;
                let chats_list = chats.chat_ids();
                for chat in chats_list {
                    debug!("{chat}");
                    td_chat_info(client_id, *chat);
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    // td_chat_history(client_id, *chat, 1);
                }
            }
        }
        Commands::GetChatHistory => {
            if parsed["@type"] == "messages" {
                let messages: Messages = serde_json::from_value(parsed)?;
                for message in messages.messages() {
                    let message = message.as_ref().unwrap();
                    let parsed_message = MessageMeta::from_message(message, None);
                    // debug!("{:?}", message);
                    parsed_message.insert_db(pg_client).await.unwrap();
                    if parsed_message.is_match() {
                        let profile_match = ProfileMatch {
                            url: parsed_message.url().as_ref().unwrap().to_string(),
                            full_text: parsed_message.text().to_string(),
                        };
                        profile_match.insert_db(pg_client).await.unwrap();
                    }
                    debug!("{:?}", parsed_message);
                    //todo if profile_reviwer active
                    let file_ids = parsed_message.file_ids();
                    if !parsed_message.text().is_empty() && file_ids.is_some() {
                        if file_ids.clone().unwrap().iter().len() > 0 {
                            let mut profile_reviewer = ProfileReviewer::new(
                                message.chat_id(), parsed_message.text(), ProfileReviewerStatus::PENDING);
                            profile_reviewer.set_file_ids(Some(parsed_message.file_ids().as_ref().unwrap().clone()));
                            profile_reviewer.insert_db(pg_client).await.expect("TODO: panic message");
                            td_file_download(client_id, profile_reviewer.main_file())?;
                        }
                    }
                    update_last_message(message.id());
                }
            }
        }
        Commands::GetChat => {
            if parsed["@type"] == "chat" {
                let chat: Chat = serde_json::from_value(parsed)?;
                match chat.last_message().as_ref() {
                    Some(last_message) => {
                        let parsed_message = MessageMeta::from_message(last_message, Some(chat.last_read_inbox_message_id()));
                        let chat_meta = ChatMeta::new(client_id, chat.id(), parsed_message.clone());
                        chat_meta.insert_db(pg_client).await;
                        parsed_message.insert_db(pg_client).await.unwrap();
                        if parsed_message.is_match() {
                            let profile_match = ProfileMatch {
                                url: parsed_message.url().as_ref().unwrap().to_string(),
                                full_text: parsed_message.text().to_string(),
                            };
                            profile_match.insert_db(pg_client).await.unwrap();
                        }
                        debug!("{:?}", parsed_message);
                        let file_ids = parsed_message.file_ids();
                        if !parsed_message.text().is_empty() && file_ids.is_some() {
                            if file_ids.clone().unwrap().iter().len() > 0 {
                                let mut profile_reviewer = ProfileReviewer::new(
                                    *parsed_message.chat_id(), parsed_message.text(), ProfileReviewerStatus::PENDING);
                                profile_reviewer.set_file_ids(Some(parsed_message.file_ids().as_ref().unwrap().clone()));
                                profile_reviewer.insert_db(pg_client).await.expect("TODO: panic message");
                                td_file_download(client_id, profile_reviewer.main_file())?;
                            }
                        }
                        // update_last_message(parsed_message);
                    }
                    None => {
                        debug!("{:?}", chat);
                    }
                }
            }
        }
        Commands::SearchPublicChat => {
            if parsed["@type"] == "chat" {
                // let chat: Chat = serde_json::from_value(parsed)?;
                // let id = chat.id();
            }
        }
        Commands::DownloadFile => {
            //todo overwrite in case of multi-file support
            if parsed["@type"] == "updateFile" {
                let last_pending = ProfileReviewer::get_waiting(pg_client).await.unwrap();
                let update_file: UpdateFile = serde_json::from_value(parsed)?;
                let path = update_file.file().local().path();
                debug!("Path {path}");
                if !path.is_empty() {
                    let new_path = format!(
                        "profile_images/{}.png", last_pending.main_file());
                    move_file(path, &new_path)?;
                }
            }
        }
        _ => {
            // error!("Unknown last_tdlib_call {}", last_tdlib_call.as_str());
        }
    }
    Ok(())
}