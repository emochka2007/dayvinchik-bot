mod auth;
mod autoresponder;
mod common;
mod constants;
mod cron;
mod embeddings;
mod entities;
mod errors;
mod file;
mod helpers;
mod input;
mod matches;
mod messages;
mod openapi;
mod pg;
mod prompts;
mod start_phrases;
mod td;
mod viuer;
mod cli;

use crate::common::env_init;
use crate::cron::cron_manager;
use crate::entities::actor::{Actor, ActorType};
use crate::entities::chat_responder::ChatResponder;
use crate::entities::image_embeddings::ImageEmbeddings;
use crate::entities::profile_reviewer::ProfileReviewer;
use crate::helpers::input;
use crate::input::match_input;
use crate::matches::MatchAnalyzer;
use crate::pg::pg::PgConnect;
use crate::td::init_tdlib_params;
use crate::td::td_json::new_client;
use crate::td::td_socket::start_td_socket;
use log::{debug, error, info};
use std::env;
use std::time::Duration;
use tokio::time::sleep;
use crate::cli::{is_cli_mode, start_cli};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    info!("Start");
    env_init();
    let pool = PgConnect::create_pool_from_env()?;
    let client = pool.get().await?;
    PgConnect::run_migrations(&client).await?;
    PgConnect::clean_db(&client).await?;

    let client = pool
        .get()
        .await
        .unwrap_or_else(|e| panic!("Couldn't get the client from pool {:?}", e));
    let is_cli_mode = is_cli_mode();
    if is_cli_mode {
        info!("Start in cli mode");
        return start_cli().await;
    } else {
        // Telegram connect
        let client_id = new_client();
        init_tdlib_params(client_id)?;

        tokio::spawn(async move {
            start_td_socket(client).await;
        });

        // Execute tasks
        if let Ok(client) = pool.get().await {
            tokio::spawn(async move {
                cron_manager(client_id, &client).await;
            });
        }
        if let Ok(client) = pool.get().await {
            // Spawn new tasks
            tokio::spawn(async move {
                Actor::new(ActorType::Default, 50)
                    .analyze(&client)
                    .await
                    .unwrap_or_else(|e| {
                        error!("Actor analyze error {:?}", e);
                    });
            });
        }


        // Review profiles
        let client = pool.get().await?;
        tokio::spawn(async move {
            loop {
                ProfileReviewer::start(&client)
                    .await
                    .unwrap_or_else(|e| error!("ProfileReviewer start {:?}", e));
                sleep(Duration::from_secs(10)).await;
            }
        });

        let build_env = env::var("BUILD")?;
        if build_env == "UNSTABLE" {
            if let Ok(client) = pool.get().await {
                tokio::spawn(async move {
                    MatchAnalyzer::start(&client).await.unwrap_or_else(|e| {
                        error!("Match Analyzer {:?}", e);
                    });
                });
            }
            if let Ok(client) = pool.get().await {
                // Store the vectors for reviewed images
                match dotenvy::var("EDU") {
                    Ok(_value) => {
                        // Get prompt score example
                        // ImageEmbeddings::get_score_of_prompt(&client, EMO_GIRL_DESCRIPTION).await?;
                        ImageEmbeddings::pick_and_store_reviewed_images(&client).await?;
                    }
                    Err(e) => {
                        debug!("EDU var is not set {:?}", e);
                    }
                }
            }

            if let Ok(client) = pool.get().await {
                tokio::spawn(async move {
                    //todo move loop internally
                    loop {
                        ChatResponder::start(&client)
                            .await
                            .unwrap_or_else(|e| error!("Chat Responder start {:?}", e));
                        sleep(Duration::from_secs(30)).await;
                    }
                });
            }
        }
    }

    Ok(())
}
