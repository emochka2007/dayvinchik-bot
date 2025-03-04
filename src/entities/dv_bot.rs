use crate::common::BotError;
use crate::constants::{VINCHIK_CHAT, VINCHIK_CHAT_INT};
use crate::entities::chat_meta::{td_chat_info, td_open_chat, ChatMeta};
use crate::entities::superlike::SuperLike;
use crate::entities::task::Task;
use crate::messages::message::SendMessage;
use crate::pg::pg::{DbQuery, PgClient};
use crate::td::td_chats::td_get_chats;
use crate::td::td_message::td_get_last_message;
use crate::td::td_request::RequestKeys;
use crate::td::td_response::ResponseKeys;
use log::error;
use uuid::Uuid;

pub struct DvBot<'a> {
    pg_client: &'a PgClient,
}

/// Only dayvinchik bot implementation
/// mb todo it in a pub trait
impl<'a> DvBot<'a> {
    pub fn new(pg_client: &'a PgClient) -> Self {
        Self { pg_client }
    }
    pub async fn send_dislike(pg_client: &PgClient) -> Result<(), BotError> {
        let send_message = SendMessage::dislike(VINCHIK_CHAT);
        let message = serde_json::to_string(&send_message)?;
        Task::new(
            message,
            RequestKeys::SendMessage,
            ResponseKeys::UpdateChatReadInbox,
            pg_client,
        )
        .await?;
        Ok(())
    }
    pub async fn send_like(pg_client: &PgClient) -> Result<(), BotError> {
        let send_message = SendMessage::like(VINCHIK_CHAT);
        let message = serde_json::to_string(&send_message)?;
        Task::new(
            message,
            RequestKeys::SendMessage,
            ResponseKeys::UpdateChatReadInbox,
            pg_client,
        )
        .await?;
        Ok(())
    }
    pub async fn send_superlike(
        pg_client: &PgClient,
        completed_reviewer_id: &Uuid,
    ) -> Result<(), BotError> {
        // Open superlike window
        let send_message = SendMessage::super_like(VINCHIK_CHAT);
        let message = serde_json::to_string(&send_message)?;
        Task::new(
            message,
            RequestKeys::SendMessage,
            ResponseKeys::UpdateChatReadInbox,
            pg_client,
        )
        .await?;
        // Send superlike text
        let starter_message = SuperLike::get_starter();
        let superlike = SuperLike::new(starter_message, *completed_reviewer_id);
        let send_message = SendMessage::text_message(superlike.message(), VINCHIK_CHAT);
        let superlike_message = serde_json::to_string(&send_message)?;
        Task::new(
            superlike_message,
            RequestKeys::SendMessage,
            ResponseKeys::UpdateChatReadInbox,
            pg_client,
        )
        .await?;
        // Insert into db
        superlike.insert(pg_client).await?;
        Ok(())
    }

    pub async fn refresh(pg_client: &PgClient) -> Result<(), BotError> {
        let send_message = SendMessage::text_message("/start", VINCHIK_CHAT);
        let message = serde_json::to_string(&send_message)?;
        Task::new(
            message,
            RequestKeys::SendMessage,
            ResponseKeys::UpdateChatReadInbox,
            pg_client,
        )
        .await?;
        let view_profiles = SendMessage::text_message("1", VINCHIK_CHAT);
        let message = serde_json::to_string(&view_profiles)?;
        Task::new(
            message,
            RequestKeys::SendMessage,
            ResponseKeys::UpdateChatReadInbox,
            pg_client,
        )
        .await?;
        Ok(())
    }

    /// Vinchik should be inside db already.
    pub async fn read_last_message(pg_client: &PgClient) -> Result<(), BotError> {
        let limit = 1;
        let chat = ChatMeta::select_by_chat_id(VINCHIK_CHAT_INT, pg_client).await?;
        match chat {
            Some(chat) => {
                td_get_last_message(pg_client, *chat.chat_id(), limit).await?;
            }
            None => {
                error!("Vinchik Chat not found");
                td_get_chats(pg_client).await?;
            }
        }
        Ok(())
    }

    pub async fn open_chat(pg_client: &PgClient) -> Result<(), BotError> {
        td_open_chat(pg_client, VINCHIK_CHAT_INT).await?;
        Ok(())
    }
    pub async fn update_bot_last_message(pg_client: &PgClient) -> Result<(), BotError> {
        td_chat_info(pg_client, VINCHIK_CHAT_INT).await?;
        Ok(())
    }

    pub async fn send_message(pg_client: &PgClient, text: &str) -> Result<(), BotError> {
        let send_message = SendMessage::text_message(text, VINCHIK_CHAT);
        let message = serde_json::to_string(&send_message)?;
        Task::new(
            message,
            RequestKeys::SendMessage,
            ResponseKeys::UpdateChatReadInbox,
            pg_client,
        )
        .await?;
        Ok(())
    }
}
