mod auth;
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

use crate::common::{env_init, BotError};
use crate::cron::cron_manager;
use crate::entities::actor::{Actor, ActorType};
use crate::entities::dv_bot::DvBot;
use crate::entities::profile_reviewer::ProfileReviewer;
use crate::helpers::input;
use crate::input::match_input;
use crate::pg::pg::PgConnect;
use crate::td::init_tdlib_params;
use crate::td::read::parse_message;
use crate::td::td_json::{new_client, receive};
use log::{debug, error, info};
use std::env;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), BotError> {
    info!("Start");
    env_init();
    let client_id = new_client();
    init_tdlib_params(client_id);

    let pool = PgConnect::create_pool_from_env()?;

    let client = pool.get().await?;
    PgConnect::run_migrations(&client).await?;

    tokio::spawn(async move {
        loop {
            let msg = tokio::task::spawn_blocking(|| {
                receive(0.1) // td_receive
            })
            .await
            .unwrap_or_else(|e| {
                error!("{:?}", e);
                panic!("Tokio task spawn blocking");
            });

            if let Some(x) = msg {
                // info!("X -> {x}");
                parse_message(&x, &client)
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
        let dv_bot = DvBot::new(&client);
        tokio::spawn(async move {
            match dotenvy::var("UPDATE_CHAT") {
                Ok(_value) => {
                    dv_bot.on_init().await.unwrap();
                }
                Err(e) => {
                    debug!("UPDATE_CHAT var is not set {:?}", e);
                }
            }
            Actor::new(ActorType::DEFAULT, 50).analyze(&client).await?;
        });
    }

    let client = pool.get().await?;
    tokio::spawn(async move {
        //todo move loop internally
        loop {
            ProfileReviewer::start(&client)
                .await
                .unwrap_or_else(|e| error!("{:?}", e));
            sleep(Duration::from_secs(5)).await;
        }
    });

    let mode = env::var("MODE")?;
    if mode == "CRON" {
    } else {
        loop {
            if let Ok(input) = input() {
                if let Ok(client) = pool.get().await {
                    tokio::spawn(async move { match_input(input, client_id, &client).await });
                }
            } else {
                log::error!("Failed to get input");
            }
        }
    }
    Ok(())
}
