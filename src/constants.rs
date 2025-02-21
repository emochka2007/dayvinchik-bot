use crate::common::{BotError, ChatId};
use crate::td::td_request::RequestKeys;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    // todo what is static ref
    static ref LAST_REQUEST: Mutex<RequestKeys> = Mutex::new(RequestKeys::Unknown);
}
pub const VINCHIK_CHAT: &str = "1234060895";
pub const VINCHIK_CHAT_INT: ChatId = 1234060895;

pub fn get_last_request() -> Result<RequestKeys, BotError> {
    Ok(*LAST_REQUEST.lock()?)
}

pub fn update_last_request(value: RequestKeys) -> Result<(), BotError> {
    let mut last_msg = LAST_REQUEST.lock()?;
    *last_msg = value;
    Ok(())
}
