use crate::common::{BotError, ChatId};
use crate::constants::VINCHIK_CHAT_INT;
use crate::entities::chat_meta::ChatMeta;
use crate::entities::profile_reviewer::ProcessingStatus;
use crate::entities::task::Task;
use crate::messages::message::SendMessage;
use crate::openapi::llm_api::OpenAI;
use crate::openapi::llm_api::OpenAIType::Chat;
use crate::pg::pg::{DbQuery, DbStatusQuery, PgClient};
use crate::prompts::Prompt;
use crate::td::td_chats::td_get_chats;
use crate::td::td_request::RequestKeys;
use crate::td::td_response::ResponseKeys;
use async_trait::async_trait;
use log::info;
use tokio_postgres::Row;
use uuid::Uuid;

/// 1. Update chats
/// 2. Get unread chats
/// 3. Get chat last_message
/// 4. Gets the response with llm with Actor behavior how to react to it
///    todo: Check if this chat in matches?
pub struct ChatResponder {
    id: Uuid,
    status: ProcessingStatus,
    chat_id: ChatId,
    msg_from: String,
    msg_to: Option<String>,
}

#[async_trait]
impl DbQuery for ChatResponder {
    const DB_NAME: &'static str = "chat_responders";
    async fn insert<'a>(&'a self, pg_client: &'a PgClient) -> Result<(), BotError> {
        let query = "INSERT into chat_responders (\
        id,
        status,\
        chat_id, \
        from, \
        VALUES ($1,$2,$3,$4)";
        pg_client
            .query(
                query,
                &[
                    &self.id,
                    &ProcessingStatus::Waiting.to_str()?,
                    &self.chat_id,
                    &self.msg_from,
                ],
            )
            .await?;
        Ok(())
    }

    fn from_sql(row: Row) -> Result<Self, BotError>
    where
        Self: Sized,
    {
        Ok(Self {
            id: row.try_get("id")?,
            chat_id: row.try_get("chat_id")?,
            status: row.try_get("status")?,
            msg_from: row.try_get("msg_from")?,
            msg_to: row.try_get("msg_to")?,
        })
    }
}

#[async_trait]
impl DbStatusQuery for ChatResponder {
    type Status = ProcessingStatus;

    async fn update_status<'a>(
        &'a self,
        pg_client: &'a PgClient,
        status: Self::Status,
    ) -> Result<(), BotError> {
        let query = "UPDATE chat_responders SET status=$1 WHERE id=$2";
        pg_client
            .query(query, &[&status.to_str()?, &self.id])
            .await?;
        Ok(())
    }

    async fn get_by_status_one(
        pg_client: &PgClient,
        status: Self::Status,
    ) -> Result<Option<Self>, BotError> {
        let query = "SELECT * from chat_responders WHERE status = $1 LIMIT 1";
        let row_opt = pg_client.query_opt(query, &[&status.to_str()?]).await?;
        match row_opt {
            Some(row) => Ok(Some(Self::from_sql(row)?)),
            None => Ok(None),
        }
    }
}
impl ChatResponder {
    pub fn new(chat_id: ChatId, from: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            status: ProcessingStatus::Waiting,
            chat_id,
            msg_from: from.to_string(),
            msg_to: None,
        }
    }
    pub async fn start(pg_client: &PgClient) -> Result<(), BotError> {
        let chats = ChatMeta::get_all_unread(pg_client).await?;
        for chat in chats {
            if *chat.chat_id() != VINCHIK_CHAT_INT {
                let open_ai = OpenAI::new(Chat)?;
                let prompt = Prompt::chat_responder(chat.last_message_text());
                let response = open_ai.send_user_message(prompt.user).await?;
                info!("OpenAI response: {response}");
                let chat_id_str = chat.chat_id().to_string();
                let send_message = SendMessage::text_message(&response, &chat_id_str);
                let message = serde_json::to_string(&send_message)?;
                Task::new(
                    message,
                    RequestKeys::SendMessage,
                    ResponseKeys::UpdateChatReadInbox,
                    pg_client,
                )
                .await?;
                td_get_chats(pg_client).await?;
            }
        }
        Ok(())
    }
    // pub fn update_to(&self, to: &str) -> Result<(), BotError> {}
}
