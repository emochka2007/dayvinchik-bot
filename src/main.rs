mod auth;
mod chats;
mod common;
mod constants;
mod cron;
mod entities;
mod errors;
mod file;
mod helpers;
mod input;
mod messages;
mod openapi;
mod pg;
mod prompts;
mod start_phrases;
mod superlike;
mod td;

use crate::cron::cron_manager;
use crate::entities::actor::{Actor, ActorType};
use crate::entities::profile_reviewer::ProfileReviewer;
use crate::helpers::input;
use crate::input::match_input;
use crate::pg::pg::{PgClient, PgConnect};
use crate::td::init_tdlib_params;
use crate::td::read::parse_message;
use crate::td::td_json::{new_client, receive};
use crate::td::td_manager::TdManager;
use log::{debug, error, info};
use rust_tdlib::tdjson::set_log_verbosity_level;
use std::env;
use std::time::Duration;
use tokio::runtime::Handle;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    // tokio: console enable console_subscriber::init();
    set_log_verbosity_level(0);
    env_logger::init();
    dotenvy::dotenv().unwrap();
    info!("Start");

    let client_id = new_client();
    init_tdlib_params(client_id);

    let pool = PgConnect::create_pool_from_env().await.unwrap();

    let client = pool.get().await.unwrap();
    PgConnect::run_migrations(&client).await;

    tokio::spawn(async move {
        loop {
            let msg = tokio::task::spawn_blocking(|| {
                receive(0.1) // td_receive
            })
                .await
                .expect("spawn_blocking failed");

            if let Some(x) = msg {
                // info!("X -> {x}");
                parse_message(&x, client_id, &client)
                    .await
                    .unwrap_or_else(|e| error!("{:?}", e));
            }
        }
    });

    // todo wait for register -> change to func state checker
    info!("Tdlib init");
    // IF QR AUTH NEEDED
    // qr_auth_init(client_id);

    if let Ok(client) = pool.get().await {
        tokio::spawn(async move {
            cron_manager(client_id, &client).await;
        });
    }

    if let Ok(client) = pool.get().await {
        tokio::spawn(async move {
            Actor::new(ActorType::DEFAULT)
                .analyze(&client)
                .await
                .expect("TODO: panic message");
        });
    }

    if let Ok(client) = pool.get().await {
        tokio::spawn(async move {
            //todo move loop internally
            loop {
                ProfileReviewer::start(&client)
                    .await
                    .unwrap_or_else(|e| error!("{:?}", e));
                sleep(Duration::from_secs(5)).await;
            }
        });
    }

    let mode = env::var("MODE").unwrap();
    if mode == "CRON" {} else {
        loop {
            if let Ok(input) = input() {
                if let Ok(client) = pool.get().await {
                    tokio::spawn(
                        async move { match_input(input, client_id, &client).await.unwrap() },
                    );
                }
            } else {
                log::error!("Failed to get input");
            }
        }
    }
}
