use crate::pg::pg::PgClient;
use crate::td::td_json::ClientId;
use anyhow::Result;
use log::info;

pub async fn match_input(input: String, _client_id: ClientId, _pg_client: &PgClient) -> Result<()> {
    info!("input - {input}");
    // let VINCHIK_i64 = VINCHIK_CHAT.parse::<i64>()?;
    // match input.to_uppercase().as_str().trim() {
    //     _ => {}
    // }
    Ok(())
}
