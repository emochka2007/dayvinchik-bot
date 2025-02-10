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

use rust_tdlib::tdjson::set_log_verbosity_level;
use crate::helpers::input;
use crate::input::match_input;
use crate::pg::pg::{create_pool_from_env, run_migrations};
use crate::td::read::parse_message;
use crate::td::tdjson::{new_client, receive};
use crate::td::{init_tdlib_params};

#[tokio::main]
async fn main() {
    set_log_verbosity_level(0);
    env_logger::init();
    log::info!("Start");
    dotenvy::dotenv().unwrap();
    // Connect postgres
    let pool = create_pool_from_env().await.unwrap();

    let client = pool.get().await.unwrap();
    run_migrations(&client).await;

    //tdlib client
    let client_id = new_client();

    tokio::spawn(async move {
        loop {
            let res = receive(2.0);
            if let Some(x) = res {
                // println!("X -> {x}");
                parse_message(&x, client_id, &client).await.unwrap();
            }
        }
    });

    init_tdlib_params(client_id);

    // wait for register -> change to func state checker
    // tokio::time::sleep(Duration::new(1, 0)).await;

    // IF QR AUTH NEEDED
    // qr_auth_init(client_id);

    // Second client for pools
    loop {
        if let Ok(input) = input() {
            if let Ok(client) = pool.get().await {
                tokio::spawn(async move { match_input(input, client_id, &client).await.unwrap() });
            }
        } else {
            log::error!("Failed to get input");
        }
    }
}


