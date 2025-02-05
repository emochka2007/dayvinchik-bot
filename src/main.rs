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
mod r#match;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::Serialize;
use std::time::Duration;
use log::LevelFilter::{Info, Trace};
use qrcode::EcLevel::H;
use rust_tdlib::tdjson::set_log_verbosity_level;
use rust_tdlib::types::Chat;
use crate::chats::ChatMeta;
use crate::helpers::input;
use crate::input::match_input;
use crate::td::read::parse_message;
use crate::td::tdjson::{new_client, receive};
use crate::td::{init_tdlib_params};

pub type UnreadChats = Arc<Mutex<HashMap<i64, ChatMeta>>>;
//todo ListOfMatches
pub type  = Arc<Mutex<HashMap<i64, ChatMeta>>>;
#[tokio::main]
async fn main() {
    set_log_verbosity_level(0);
    env_logger::init();
    log::info!("Start");
    dotenvy::dotenv().unwrap();

    let client_id = new_client();
    let unread_chats: UnreadChats = Arc::new(Mutex::new(HashMap::new()));
    let active_matches: UnreadChats = Arc::new(Mutex::new(HashMap::new()));


    let unread_chats = Arc::clone(&unread_chats);
    let unread_chats_clone = Arc::clone(&unread_chats);
    tokio::spawn(async move {
        loop {
            let res = receive(2.0);
            if let Some(x) = res {
                parse_message(&x, client_id, &unread_chats_clone).expect("TODO: panic message");
                // println!("X -> {x}");
            }
        }
    });

    init_tdlib_params(client_id);

    // wait for register -> change to func state checker
    tokio::time::sleep(Duration::new(3, 0)).await;

    // IF QR AUTH NEEDED
    // qr_auth_init(client_id);
    // let unread_chats = Arc::clone(&unread_chats);
    loop {
        let input = input().unwrap();
        match_input(input, client_id, &unread_chats).await;
        tokio::time::sleep(Duration::new(1, 0)).await;
    }
}


