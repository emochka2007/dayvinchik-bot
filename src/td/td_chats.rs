use crate::common::BotError;
use crate::entities::task::Task;
use crate::pg::pg::PgClient;
use crate::td::td_request::RequestKeys;
use crate::td::td_response::ResponseKeys;
use rust_tdlib::types::GetChats;

pub async fn td_get_chats(pg_client: &PgClient) -> Result<(), BotError> {
    let public_chats = GetChats::builder().limit(100).build();
    let message = serde_json::to_string(&public_chats)?;
    Task::new(
        message,
        RequestKeys::GetChats,
        ResponseKeys::Chats,
        pg_client,
    )
    .await?;
    Ok(())
}
