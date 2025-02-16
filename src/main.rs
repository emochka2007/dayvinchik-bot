mod pg;
mod message;
mod auth;
mod chats;
mod file;
mod input;
mod constants;
mod td;
mod prompts;
mod my_profile;
mod start_phrases;
mod errors;
mod superlike;
mod openapi;
mod helpers;
mod entities;
mod common;
mod cron;

use std::env;
use std::time::Duration;
use log::{debug, error, info};
use rust_tdlib::tdjson::set_log_verbosity_level;
use tokio::runtime::Handle;
use crate::cron::{cron, cron_manager};
use crate::helpers::input;
use crate::input::match_input;
use crate::pg::pg::{PgClient, PgConnect};
use crate::td::read::parse_message;
use crate::td::td_json::{new_client, receive};
use crate::td::{init_tdlib_params};
use crate::td::td_manager::TdManager;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    console_subscriber::init();
    set_log_verbosity_level(0);
    env_logger::init();
    dotenvy::dotenv().unwrap();
    info!("Start");

    let client_id = new_client();
    init_tdlib_params(client_id);

    // Connect postgres
    let pool = PgConnect::create_pool_from_env().await.unwrap();

    let client = pool.get().await.unwrap();
    PgConnect::run_migrations(&client).await;

    // tdlib client

    tokio::spawn(async move {
        loop {
            if let Some(x) = receive(0.1) {
                println!("X -> {x}");
                parse_message(&x, client_id, &client).await.
                    unwrap_or_else(|e| { error!("{:?}", e) })
            }
        }
    });


    // todo wait for register -> change to func state checker
    info!("Tdlib init");
    // IF QR AUTH NEEDED
    // qr_auth_init(client_id);

    cron(client_id).await;


    // let mode = env::var("MODE").unwrap();
    // if mode == "CRON" {
    //     let handle = tokio::spawn(async move {
    //     });
    //     handle.await.unwrap();
    // } else {
    //     loop {
    //         if let Ok(input) = input() {
    //             if let Ok(client) = pool.get().await {
    //                 tokio::spawn(async move {
    //                     match_input(input, client_id, &client).await.unwrap()
    //                 });
    //             }
    //         } else {
    //             log::error!("Failed to get input");
    //         }
    //     }
    // }
}


