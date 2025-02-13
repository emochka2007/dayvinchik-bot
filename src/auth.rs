use rust_tdlib::types::RequestQrCodeAuthentication;
use crate::td::td_json::{send, ClientId};

pub fn qr_auth_init(client_id: ClientId){
    let qr_code_message = RequestQrCodeAuthentication::builder().build();
    let qr_msg_json = serde_json::to_string(&qr_code_message).unwrap();
    send(client_id, &qr_msg_json);
}