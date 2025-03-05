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
mod matches;
mod messages;
mod openapi;
mod pg;
mod prompts;
mod start_phrases;
mod td;
mod viuer;

use crate::common::{env_init, BotError, MessageId};
use crate::cron::cron_manager;
use crate::entities::actor::{Actor, ActorType};
use crate::entities::chat_responder::ChatResponder;
use crate::entities::image_embeddings::{
    get_and_store_embedding, get_score_of_image, ImageEmbeddings,
};
use crate::entities::profile_reviewer::ProfileReviewer;
use crate::file::{image_to_base64, new_base64};
use crate::helpers::input;
use crate::input::match_input;
use crate::matches::MatchAnalyzer;
use crate::openapi::llm_api::OpenAI;
use crate::pg::pg::{DbQuery, PgConnect};
use crate::prompts::Prompt;
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
    PgConnect::clean_db(&client).await?;

    match dotenvy::var("EDU") {
        Ok(_value) => {
            let chat_ai = OpenAI::new("chat/").unwrap();
            let analyze_prompt = Prompt::llava_image();
            let image_64 = new_base64("reviewed_images/40dc60f5-7d77-4a29-b5d7-942f1966a8fa.png");
            let description = chat_ai
                .send_image_with_prompt(analyze_prompt.clone(), analyze_prompt, image_64)
                .await
                .unwrap();
            let open_ai = OpenAI::new("embeddings/").unwrap();
            open_ai.embeddings(&description).await.unwrap();
            // ImageEmbeddings::get_score_of_prompt(&client, "emo girl with piercings").await?;
            get_and_store_embedding(&client).await.unwrap_or_else(|e| {
                error!("Error in get_and_store_embedding {:?}", e);
            });
        }
        Err(e) => {
            debug!("EDU var is not set {:?}", e);
        }
    }
    return Ok(());

    tokio::spawn(async move {
        loop {
            let msg = tokio::task::spawn_blocking(|| {
                receive(1.5) // td_receive
            })
            .await
            .unwrap_or_else(|e| {
                error!("{:?}", e);
                panic!("Tokio task spawn blocking");
            });

            if let Some(x) = msg {
                debug!("X -> {x}");
                parse_message(&client, &x)
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
