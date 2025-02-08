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

use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};
use serde::Serialize;
use std::time::Duration;
use rust_tdlib::tdjson::set_log_verbosity_level;
use rust_tdlib::types::Chat;
use crate::chats::{ChatMeta, UnreadChats};
use crate::entities::profile_match::{ProfileMatch};
use crate::helpers::input;
use crate::input::match_input;
use crate::pg::pg::{connect_pg_from_env, run_migrations, PgConnect};
use crate::td::read::parse_message;
use crate::td::tdjson::{new_client, receive};
use crate::td::{init_tdlib_params};

//todo ListOfMatches
#[tokio::main]
async fn main() {
    set_log_verbosity_level(0);
    env_logger::init();
    log::info!("Start");
    dotenvy::dotenv().unwrap();
    // Connect postgres
    let pg_client = connect_pg_from_env().await.unwrap();
    run_migrations(&pg_client).await;
    let client_id = new_client();
    tokio::spawn(async move {
        loop {
            let res = receive(2.0);
            if let Some(x) = res {
                // println!("X -> {x}");
                parse_message(&x, client_id, &pg_client).await.unwrap();
            }
        }
    });

    init_tdlib_params(client_id);

    // wait for register -> change to func state checker
    tokio::time::sleep(Duration::new(1, 0)).await;

    // IF QR AUTH NEEDED
    // qr_auth_init(client_id);

    loop {
        let input = input().unwrap();
        match_input(input, client_id).await;
        tokio::time::sleep(Duration::new(1, 0)).await;
    }
}


