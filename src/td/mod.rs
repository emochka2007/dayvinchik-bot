use crate::file::get_project_root;
use crate::td::td_json::{send, ClientId};
use crate::vault::vault_kv;
use serde::Serialize;
use serde_json::Value;
use std::env;

pub mod read;
pub mod td_chats;
pub mod td_command_map;
pub mod td_file;
pub mod td_json;
pub mod td_message;
pub mod td_request;
pub mod td_response;
use anyhow::Result;

#[derive(Serialize)]
pub struct TDLibParams {
    pub(crate) use_test_dc: bool,
    pub(crate) database_directory: Option<String>,
    pub(crate) files_directory: Option<String>,
    pub(crate) use_file_database: bool,
    pub(crate) use_chat_info_database: bool,
    pub(crate) use_message_database: bool,
    pub(crate) use_secret_chats: bool,
    pub(crate) api_id: i32,
    pub(crate) api_hash: String,
    pub(crate) system_language_code: String,
    pub(crate) device_model: String,
    pub(crate) system_version: Option<String>,
    pub(crate) application_version: String,
    pub(crate) enable_storage_optimizer: bool,
    pub(crate) ignore_file_names: bool,
}
pub fn init_tdlib_params(client_id: ClientId) -> Result<()> {
    // use custom dir for storing artefacts that tdlib creates in dev
    let root = get_project_root()?;
    let artefacts_dir = format!("{}/td/tdlib_artefacts", root.to_str().unwrap());
    let tg_hash = vault_kv("tg_hash");
    let tg_id = vault_kv("tg_id").parse::<i32>()?;

    // set tdlib params
    let params = TDLibParams {
        use_test_dc: false,
        database_directory: Some(artefacts_dir),
        files_directory: None,
        use_file_database: false,
        use_chat_info_database: true,
        use_message_database: true,
        use_secret_chats: false,
        api_id: tg_id,
        api_hash: tg_hash,
        system_language_code: "en".to_string(),
        device_model: "MacBook Pro".to_string(),
        system_version: None,
        application_version: "0.1.0".to_string(),
        enable_storage_optimizer: false,
        ignore_file_names: false,
    };

    let params_value = serde_json::to_value(params)?;

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
    Ok(())
}
