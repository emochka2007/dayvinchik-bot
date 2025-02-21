use crate::common::BotError;
use crate::constants::VINCHIK_CHAT;
use crate::pg::pg::PgClient;
use crate::td::td_json::ClientId;
use log::info;

pub async fn match_input(
    input: String,
    _client_id: ClientId,
    _pg_client: &PgClient,
) -> Result<(), BotError> {
    info!("input - {input}");
    let _VINCHIK_i64 = VINCHIK_CHAT.parse::<i64>()?;
    match input.to_uppercase().as_str().trim() {
        _ => {}
    }
    Ok(())
}
