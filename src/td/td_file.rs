use rust_tdlib::types::DownloadFile;
use serde_json::Error;
use crate::constants::update_last_tdlib_call;
use crate::td::tdjson::{send, ClientId};

pub fn td_file_download(client_id: ClientId, file_id: i32) -> Result<(), Error> {
    let download_msg = DownloadFile::builder().file_id(file_id).limit(1)
        .priority(16).build();
    let message = serde_json::to_string(&download_msg)?;
    send(client_id, &message);
    update_last_tdlib_call("DownloadFile".to_string());
    Ok(())
}