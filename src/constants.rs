use crate::common::ChatId;
use crate::td::td_request::RequestKeys;
use lazy_static::lazy_static;
use log::{error, info};
use std::sync::Mutex;

lazy_static! {
    // todo what is static ref
    static ref LAST_REQUEST: Mutex<RequestKeys> = Mutex::new(RequestKeys::Unknown);
}
pub const VINCHIK_CHAT: &str = "1234060895";
pub const VINCHIK_CHAT_INT: ChatId = 1234060895;

pub fn get_last_request() -> RequestKeys {
    *LAST_REQUEST.lock().expect("Error")
}

pub fn update_last_request(value: RequestKeys) {
    // error!("updating last tdlib {:?}", value);
    let mut last_msg = LAST_REQUEST.lock().unwrap();
    *last_msg = value;
}
