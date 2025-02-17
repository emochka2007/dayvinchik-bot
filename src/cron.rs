use crate::pg::pg::PgClient;
use crate::td::td_json::ClientId;
use crate::td::td_manager::TdManager;
use log::{error, info};
use std::str::FromStr;
use std::time::Duration;
use tokio::time::{interval, sleep};

pub async fn cron_manager(client_id: ClientId, pg_client: &PgClient) {
    let mut interval = interval(Duration::from_secs(1));
    interval.tick().await;
    let td_manager = TdManager::init(client_id);
    loop {
        interval.tick().await;
        match td_manager.send_request(pg_client).await {
            Ok(_) => info!("Cron job executed"),
            // Err(e) => error!("Cron job failed {:?}", e)
            Err(e) => {}
        }
    }
}
