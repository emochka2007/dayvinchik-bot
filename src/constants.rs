use crate::common::ChatId;
use crate::td::td_request::RequestKeys;
use lazy_static::lazy_static;
use log::info;
use std::sync::Mutex;

lazy_static! {
    static ref LAST_MESSAGE: Mutex<i64> = Mutex::new(0);
    // todo what is static ref
    static ref LAST_TDLIB_CALL: Mutex<RequestKeys> = Mutex::new(RequestKeys::Unknown);
}
pub const VINCHIK_CHAT: &str = "1234060895";
pub const VINCHIK_CHAT_INT: ChatId = 1234060895;

pub fn get_last_tdlib_call() -> RequestKeys {
    *LAST_TDLIB_CALL.lock().expect("Error")
}

pub fn update_last_tdlib_call(value: RequestKeys) {
    info!("updating last tdlib {:?}", value);
    let mut last_msg = LAST_TDLIB_CALL.lock().unwrap();
    *last_msg = value;
}
