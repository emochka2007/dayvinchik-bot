use rust_tdlib::types::RequestQrCodeAuthentication;
use crate::td::tdjson::{send, ClientId};

pub fn qr_auth_init(client_id: ClientId){
    let qrCodeMessage = RequestQrCodeAuthentication::builder().build();
    let qrMsgJson = serde_json::to_string(&qrCodeMessage).unwrap();
    send(client_id, &qrMsgJson);
}