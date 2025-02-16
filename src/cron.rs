use std::str::FromStr;
use std::time::Duration;
use chrono::{Local, Utc};
use cron::Schedule;
use log::{error, info};
use tokio::time::{interval, sleep};
use crate::pg::pg::{PgClient};
use crate::td::td_json::ClientId;
use crate::td::td_manager::TdManager;

pub async fn cron_manager(client_id: ClientId) {
    let mut interval = interval(Duration::from_secs(2));
    interval.tick().await;
    error!("Cron job 10");
    let td_manager = TdManager::init(client_id);
    error!("Cron job 12");
    loop {
        error!("Started interval");
        interval.tick().await;
        error!("Cron job awaited");
        match td_manager.send_request().await {
            Ok(_) => info!("Cron job executed"),
            Err(e) => error!("Cron job failed {:?}", e)
        }
    }
}

pub async fn cron(client_id: ClientId) {
    let expression = "0/5 * * * * *";
    let schedule = Schedule::from_str(expression).expect("Failed to parse CRON expression");
    // let client = connect_pg_from_env().await.unwrap();
    let td_manager = TdManager::init(client_id);
    loop {
        let now = Utc::now();
        error!("{:?}", now);
        if let Some(next) = schedule.upcoming(Utc).take(1).next() {
            let until_next = next - now;
            sleep(until_next.to_std().unwrap()).await;
            match td_manager.send_request().await {
                Ok(_) => info!("Cron job executed"),
                Err(e) => error!("Cron job failed {:?}", e)
            }
            println!(
                "Running every 5 seconds. Current time: {}",
                Local::now().format("%Y-%m-%d %H:%M:%S")
            );
        }
    }
}