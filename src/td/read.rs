use std::fmt::format;
use log::{debug, error, info};
use rust_tdlib::types::{Chat, Chats, Messages, SearchPublicChat, UpdateFile};
use serde_json::Value;
use tokio_postgres::Client;
use uuid::Uuid;
use crate::chats::{td_chat_info, td_chat_history, ChatMeta};
use crate::constants::{get_last_tdlib_call, update_last_message};
use crate::entities::profile_match::{ProfileMatch};
use crate::entities::profile_reviewer::{ProfileReviewer, ProfileReviewerStatus};
use crate::file::{file_log, log_append, move_file};
use crate::td::td_message::{match_message_content, MessageMeta};
use crate::td::tdjson::ClientId;
use crate::UnreadChats;

pub async fn parse_message(json_str: &str, client_id: ClientId, pg_client: &Client) -> std::io::Result<()> {
    let parsed: Value = serde_json::from_str(json_str)?;
    let last_tdlib_call = get_last_tdlib_call();
    match last_tdlib_call.as_str() {
        // "GetChats" => {
        //     let chats: Chats = serde_json::from_value(parsed)?;
        //     let chats_list = chats.chat_ids();
        //     for chat in chats_list {
        //         let chat_info = td_chat_info(client_id, *chat);
        //         td_chat_history(client_id, *chat, 1);
        //     }
        // }
        "GetChatHistory" => {
            if parsed["@type"] == "messages" {
                let messages: Messages = serde_json::from_value(parsed)?;
                for message in messages.messages() {
                    let message = message.as_ref().unwrap();
                    let parsed_message = MessageMeta::from_message(message, None);
                    debug!("{:?}", message);
                    debug!("{:?}", parsed_message);
                    if parsed_message.is_match() {
                        let profile_match = ProfileMatch {
                            url: parsed_message.url().as_ref().unwrap().to_string(),
                            full_text: parsed_message.text().to_string(),
                        };
                        profile_match.insert_db(pg_client).await.unwrap();
                    }
                    //todo if profile_reviwer active
                    let profile_reviewer = ProfileReviewer::new(
                        message.chat_id(), parsed_message.text(), ProfileReviewerStatus::PENDING);
                    profile_reviewer.insert_db(pg_client).await.expect("TODO: panic message");
                    update_last_message(message.id());
                }
            }
        }
        "GetChat" => {
            if parsed["@type"] == "chat" {
                let chat: Chat = serde_json::from_value(parsed)?;
                match chat.last_message().as_ref() {
                    Some(last_message) => {
                        let parsed_message = MessageMeta::from_message(last_message, Some(chat.last_read_inbox_message_id()));
                        let chat_meta = ChatMeta::new(client_id, chat.id(), parsed_message);
                        chat_meta.insert_db(pg_client).await;
                    }
                    None => {
                        debug!("{:?}", chat);
                    }
                }
            }
        }
        "SearchPublicChat" => {
            if parsed["@type"] == "chat" {
                info!("last_tdlib_call {}", parsed);
                let chat: Chat = serde_json::from_value(parsed)?;
                let id = chat.id();
                info!("public id {id}");
            }
        }
        "DownloadFile" => {
            if parsed["@type"] == "updateFile" {
                let update_file: UpdateFile = serde_json::from_value(parsed)?;
                let path = update_file.file().local().path();
                debug!("Path {path}");
                if !path.is_empty() {
                    let uuid = Uuid::new_v4();
                    let new_path = format!("profile_images/{}.png",
                                           uuid.to_string());

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