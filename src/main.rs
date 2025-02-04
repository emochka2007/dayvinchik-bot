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

use std::collections::HashMap;
use serde::Serialize;
use std::time::Duration;
use log::LevelFilter::{Info, Trace};
use rust_tdlib::tdjson::set_log_verbosity_level;
use crate::helpers::input;
use crate::input::match_input;
use crate::td::read::parse_message;
use crate::td::tdjson::{new_client, receive};
use crate::td::{init_tdlib_params};

#[tokio::main]
async fn main() {
    set_log_verbosity_level(0);
    env_logger::init();
    log::info!("Start");
    dotenvy::dotenv().unwrap();

    let client_id = new_client();


    tokio::spawn(async move {
        while let res = receive(2.0) {
            // println!("in receive");
            match res {
                Some(x) => {
                    println!("{}", x);
                    parse_message(&x, client_id).expect("TODO: panic message");
                }
                None => {}
            }
        }
    });

    init_tdlib_params(client_id);

    // wait for register -> change to func state checker
    tokio::time::sleep(Duration::new(3, 0)).await;

    // IF QR AUTH NEEDED
    // qr_auth_init(client_id);

    loop {
        let input = input().unwrap();
        match_input(input, client_id).await;
        tokio::time::sleep(Duration::new(1, 0)).await;
    }
}


