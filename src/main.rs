mod tdjson;
mod message;
mod auth;
mod chats;
mod file;
mod input;
mod constants;
mod td_file;
mod llm_api;
mod prompts;
mod my_profile;
mod openai;
mod start_phrases;
mod errors;
mod superlike;

use image::Luma;
use serde::Serialize;
use serde_json::{Value};
use std::{env, io};
use std::any::Any;
use std::time::Duration;
use qrcode::QrCode;
use rust_tdlib::tdjson::set_log_verbosity_level;
use rust_tdlib::types::{Chats, MessageContent, Messages};
use tdjson::{new_client, receive, send};
use crate::file::{file_log, log_append};
use crate::input::match_input;

#[derive(Serialize)]
struct TDLibParams {
    use_test_dc: bool,
    database_directory: Option<String>,
    files_directory: Option<String>,
    use_file_database: bool,
    use_chat_info_database: bool,
    use_message_database: bool,
    use_secret_chats: bool,
    api_id: i32,
    api_hash: String,
    system_language_code: String,
    device_model: String,
    system_version: Option<String>,
    application_version: String,
    enable_storage_optimizer: bool,
    ignore_file_names: bool,
}

#[tokio::main]
async fn main() {
    set_log_verbosity_level(0);
    dotenvy::dotenv().unwrap();

    let client_id = new_client();

    tokio::spawn(async move {
        while let res = receive(2.0) {
            // println!("in receive");
            match res {
                Some(x) => {
                    println!("{}", x);
                    parse_message(&x)
                }
                None => {}
            }
        }
    });

    // use custom dir for storing artefacts that tdlib creates in dev
    let root = project_root::get_project_root().unwrap();
    let artefacts_dir = format!("{}/../td/tdlib_artefacts", root.to_str().unwrap());

    // set tdlib params
    let params = TDLibParams {
        use_test_dc: false,
        database_directory: Some(artefacts_dir),
        files_directory: None,
        use_file_database: false,
        use_chat_info_database: true,
        use_message_database: true,
        use_secret_chats: false,
        api_id: env::var("TD_API_ID").unwrap().parse().unwrap(),
        api_hash: env::var("TD_API_HASH").unwrap(),
        system_language_code: "en".to_string(),
        device_model: "MacBook Pro".to_string(),
        system_version: None,
        application_version: "0.1.0".to_string(),
        enable_storage_optimizer: false,
        ignore_file_names: false,
    };

    let params_value = serde_json::to_value(params).unwrap();

    // add @type field to json as it is invalid syntax for struct field
    // obtained from SO: https://stackoverflow.com/a/65357137
    let params_json = match params_value {
        Value::Object(m) => {
            let mut m = m.clone();
            m.insert(
                "@type".to_string(),
                Value::String("setTdlibParameters".to_string()),
            );
            Value::Object(m)
        }
        v => v.clone(),
    }
        .to_string();

    send(client_id, &params_json);

    // wait for register -> change to func state checker
    tokio::time::sleep(Duration::new(3, 0)).await;

    // IF QR AUTH NEEDED
    // qr_auth_init(client_id);
    // loop to keep getting json input for send() from developer
    loop {
        let input = input().unwrap();
        match_input(input, client_id).await;
        tokio::time::sleep(Duration::new(1, 0)).await;
    }
}

fn input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

fn generate_qr_code(link: &str) {
    let code = QrCode::new(link.as_bytes()).unwrap();
    let image = code.render::<Luma<u8>>().build();
    image.save("qr_code.png").unwrap();
    println!("✅ QR Code saved as 'qr_code.png'. Scan it with your Telegram app.");
}
fn parse_message(json_str: &str) {
    let parsed: Value = serde_json::from_str(json_str).unwrap();
    // Extract the link
    if parsed["authorization_state"].is_string() {
        let link = parsed["authorization_state"]["link"].as_str();
        // println!("✅ Extracted Telegram Link: {}", link);
        // generate_qr_code(link);
    } else if parsed["chat_ids"].is_array() {
        let chats: Chats = serde_json::from_value(parsed).unwrap();
        file_log(serde_json::to_string(&chats).unwrap());
    } else if parsed["@type"] == "messages" {
        let messages: Messages = serde_json::from_value(parsed).unwrap();
        let last_message = messages.messages().get(0).unwrap().clone();
        let msg_content: MessageContent = last_message.unwrap().content().clone();
        let path_to_append = "profile.log";
        match msg_content {
            // If video just send only caption
            MessageContent::MessageVideo(content) => {
                let text = content.caption().text();
                println!("147 {text}");
                log_append(text.clone(), path_to_append).expect("TODO: panic message");
            }
            MessageContent::MessagePhoto(content) => {
                let text = content.caption().text();
                println!("147 {text}");
                log_append(text.clone(), path_to_append).expect("TODO: panic message");
            }
            _ => {
                println!("Unknown message content {:?}", msg_content);
            }
        }
    }
}