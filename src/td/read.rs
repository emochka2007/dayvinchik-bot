use crate::common::BotError;
use crate::constants::get_last_request;
use crate::entities::chat_meta::{get_chat, td_chat_info};
use crate::file::move_file;
use crate::pg::pg::PgClient;
use crate::td::td_manager::Task;
use crate::td::td_message::chat_history;
use crate::td::td_request::RequestKeys;
use crate::td::td_response::ResponseKeys;
use log::{debug, error, info};
use rust_tdlib::types::{Chat, Chats, UpdateFile};
use serde_json::Value;

pub async fn parse_message(json_str: &str, pg_client: &PgClient) -> Result<(), BotError> {
    let json_value: Value = serde_json::from_str(json_str)?;

    let td_type = match json_value.get("@type").and_then(|v| v.as_str()) {
        Some(t) => t,
        None => {
            debug!("No @type field in the incoming JSON. Ignoring.");
            return Ok(());
        }
    };

    let response_key = match ResponseKeys::from_str(td_type) {
        Ok(rk) => rk,
        Err(_e) => {
            debug!("Unrecognized or unsupported td_type: {td_type}. Ignoring.");
            return Ok(());
        }
    };

    debug!("Td_type: {td_type}");
    debug!("Key: {:?}", response_key);

    // This is your “last request” logic; adjust as needed.
    let last_tdlib_call = get_last_request()?;
    debug!("Last tdlib call: {:?}", last_tdlib_call);

    // Attempt to find a “pending” Task that matches the last request + response
    let task = Task::first_pending(pg_client, &last_tdlib_call, &response_key).await?;

    if *task.request() == last_tdlib_call && *task.response() == response_key {
        task.to_complete(pg_client).await?;
    } else {
        // Skip all non-matched webhooks
        return Ok(());
    }

    match (last_tdlib_call, response_key) {
        (RequestKeys::GetChats, ResponseKeys::Chats) => {
            debug!("Processing GetChats -> Chats");
            let chats: Chats = serde_json::from_value(json_value)?;
            for chat_id in chats.chat_ids() {
                debug!("Found chat_id: {chat_id}");
                if let Err(e) = td_chat_info(pg_client, *chat_id).await {
                    error!("td_chat_info failed for chat_id {chat_id}: {e}");
                }
            }
        }

        (RequestKeys::GetChatHistory, ResponseKeys::Messages) => {
            debug!("Processing GetChatHistory -> Messages");
            if let Err(e) = chat_history(json_value, pg_client).await {
                error!("chat_history failed: {e}");
            }
        }

        (RequestKeys::GetChat, ResponseKeys::Chat) => {
            debug!("Processing GetChat -> Chat");
            if let Err(e) = get_chat(json_value, pg_client).await {
                error!("get_chat failed: {e}");
            }
        }

        (RequestKeys::SearchPublicChat, ResponseKeys::Chat) => {
            debug!("Processing SearchPublicChat -> Chat");
            let chat: Chat = serde_json::from_value(json_value)?;
            debug!("Found public chat with id {}", chat.id());
        }

        (RequestKeys::DownloadFile, ResponseKeys::UpdateFile) => {
            debug!("Processing DownloadFile -> UpdateFile");
            // Overwrite in case of multi-file support
            let update_file: UpdateFile = match serde_json::from_value(json_value) {
                Ok(update_file) => update_file,
                Err(e) => {
                    error!("Failed to parse UpdateFile: {e}");
                    return Ok(());
                }
            };

            let path = update_file.file().local().path();
            debug!("Downloaded local path: {path}");

            if !path.is_empty() {
                let new_path = format!("profile_images/{}.png", update_file.file().id());
                if let Err(e) = move_file(path, &new_path) {
                    error!("Failed to move file {path} -> {new_path}: {e}");
                }
            }
        }

        (_, _) => {}
    }

    Ok(())
}
