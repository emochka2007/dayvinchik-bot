use crate::chats::{get_chat, td_get_last_message, td_chat_info, ChatMeta};
use crate::constants::get_last_tdlib_call;
use crate::entities::profile_match::ProfileMatch;
use crate::entities::profile_reviewer::{ProfileReviewer, ProfileReviewerStatus};
use crate::file::move_file;
use crate::pg::pg::PgClient;
use crate::td::td_file::td_file_download;
use crate::td::td_json::ClientId;
use crate::td::td_manager::Task;
use crate::td::td_message::{chat_history, MessageMeta};
use crate::td::td_request::RequestKeys;
use crate::td::td_response::ResponseKeys;
use log::{debug, info};
use rust_tdlib::types::{Chat, Chats, Messages, UpdateFile};
use serde_json::Value;
use std::cmp::PartialEq;
use std::thread::sleep;
use std::time::Duration;
use tokio_postgres::Client;

pub async fn parse_message(
    json_str: &str,
    client_id: ClientId,
    pg_client: &PgClient,
) -> std::io::Result<()> {
    let json_value: Value = serde_json::from_str(json_str)?;
    match json_value.get("@type") {
        Some(td_type) => {
            let td_type = td_type.as_str().unwrap();
            info!("Td_type {td_type}");
            match ResponseKeys::from_str(td_type) {
                Ok(key) => {
                    debug!("Key {:?}", key);
                    let last_tdlib_call = get_last_tdlib_call();
                    debug!("Last tdlib call {:?}", last_tdlib_call);
                    //todo i'm sure i can improve it
                    let last_pending = Task::first_pending(pg_client).await.unwrap_or_default();
                    if *last_pending.request() == last_tdlib_call && key == *last_pending.response()
                    {
                        last_pending.to_complete(pg_client).await.unwrap();
                    }
                    match last_tdlib_call {
                        RequestKeys::GetChats => {
                            if key == ResponseKeys::Chats {
                                info!("GetChats");
                                let chats: Chats = serde_json::from_value(json_value)?;
                                let chats_list = chats.chat_ids();
                                for chat in chats_list {
                                    debug!("{chat}");
                                    td_chat_info(pg_client, *chat).await;
                                    // td_chat_history(client_id, *chat, 1);
                                }
                            }
                        }
                        RequestKeys::GetChatHistory => {
                            if key == ResponseKeys::Messages {
                                chat_history(json_value, pg_client).await?
                            }
                        }
                        RequestKeys::GetChat => {
                            if key == ResponseKeys::Chat {
                                get_chat(json_value, pg_client).await?;
                            }
                        }
                        RequestKeys::SearchPublicChat => {
                            if json_value["@type"] == "chat" {
                                // let chat: Chat = serde_json::from_value(parsed)?;
                                // let id = chat.id();
                            }
                        }
                        RequestKeys::DownloadFile => {
                            //todo overwrite in case of multi-file support
                            if key == ResponseKeys::UpdateFile {
                                let update_file: UpdateFile = serde_json::from_value(json_value)?;
                                let path = update_file.file().local().path();
                                // let last_pending =
                                //     ProfileReviewer::get_waiting(pg_client).await.unwrap();
                                debug!("Path {path}");
                                if !path.is_empty() {
                                    let new_path =
                                        format!("profile_images/{}.png", update_file.file().id());
                                    move_file(path, &new_path)?;
                                }
                            }
                        }
                        _ => {
                            // error!("Unknown last_tdlib_call {}", last_tdlib_call.as_str());
                        }
                    }
                }
                Err(..) => {}
            }
        }
        None => {}
    }
    Ok(())
}
