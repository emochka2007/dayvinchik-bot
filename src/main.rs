#![feature(error_generic_member_access)]
mod auth;
mod common;
mod constants;
mod cron;
mod embeddings;
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
mod td;
mod viuer;

use crate::common::BotError::OllamaError;
use crate::common::{env_init, BotError};
use crate::cron::cron_manager;
use crate::embeddings::ollama::OllamaVision;
use crate::entities::actor::{Actor, ActorType};
use crate::entities::chat_responder::ChatResponder;
use crate::entities::dv_bot::DvBot;
use crate::entities::image_embeddings::{get_and_store_embedding, ImageEmbeddings};
use crate::entities::profile_reviewer::ProfileReviewer;
use crate::file::image_to_base64;
use crate::helpers::input;
use crate::input::match_input;
use crate::pg::pg::{DbQuery, PgConnect};
use crate::td::init_tdlib_params;
use crate::td::read::parse_message;
use crate::td::td_json::{new_client, receive};
use crate::viuer::display_image_in_terminal;
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
    // display_image_in_terminal("alt_images/popusk.jpg");
    //
    // return Ok(());
    let pool = PgConnect::create_pool_from_env()?;
    let client = pool.get().await?;
    PgConnect::run_migrations(&client).await?;
    PgConnect::clean_db(&client).await?;

    match dotenvy::var("EDU") {
        Ok(_value) => {
            get_and_store_embedding(&client).await.unwrap_or_else(|e| {
                error!("Get and store embedding {:?}", e);
            });
        }
        Err(e) => {
            debug!("EDU var is not set {:?}", e);
        }
    }

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
                debug!("X -> {x}");
                parse_message(&x, &client)
                    .await
                    .unwrap_or_else(|e| error!("Parse message {:?}", e));
            }
        }
    });

    // todo wait for register -> change to func state checker
    // IF QR AUTH NEEDED
    // qr_auth_init(client_id);

    if let Ok(client) = pool.get().await {
        tokio::spawn(async move {
            cron_manager(client_id, &client).await;
        });
    }
    let client = pool
        .get()
        .await
        .unwrap_or_else(|e| panic!("Couldn't get the client from pool {:?}", e));

    tokio::spawn(async move {
        let dv_bot = DvBot::new(&client);
        match dotenvy::var("UPDATE_CHAT") {
            Ok(_value) => {
                dv_bot.on_init().await.unwrap_or_else(|e| {
                    error!("DV_BOT init error {:?}", e);
                });
            }
            Err(e) => {
                debug!("UPDATE_CHAT var is not set {:?}", e);
            }
        }
        Actor::new(ActorType::Default, 50)
            .analyze(&client)
            .await
            .unwrap_or_else(|e| {
                error!("Actor analyze error {:?}", e);
            });
    });

    let client = pool.get().await?;
    tokio::spawn(async move {
        //todo move loop internally
        loop {
            ProfileReviewer::start(&client)
                .await
                .unwrap_or_else(|e| error!("ProfileReviewer start {:?}", e));
            sleep(Duration::from_secs(10)).await;
        }
    });

    // let client = pool.get().await?;
    // tokio::spawn(async move {
    //     //todo move loop internally
    //     loop {
    //         ChatResponder::start(&client)
    //             .await
    //             .unwrap_or_else(|e| error!("Chat Responder start {:?}", e));
    //         sleep(Duration::from_secs(30)).await;
    //     }
    // });

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
