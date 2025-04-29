use crate::common::{ChatId, MessageId};
use crate::td::td_request::RequestKeys;
use anyhow::Result;
use lazy_static::lazy_static;
use std::sync::Mutex;
use tokio::sync::Mutex as TokioMutex;

lazy_static! {
    // todo what is static ref
    static ref LAST_REQUEST: Mutex<RequestKeys> = Mutex::new(RequestKeys::Unknown);
    pub static ref PROCESSED_MESSAGE_IDS: TokioMutex<Vec<MessageId>> = TokioMutex::new(Vec::new());
}
pub const VINCHIK_CHAT: &str = "1234060895";
pub const VINCHIK_CHAT_INT: ChatId = 1234060895;

pub fn get_last_request() -> Result<RequestKeys> {
    Ok(*LAST_REQUEST.lock().unwrap())
}

pub fn update_last_request(value: RequestKeys) -> Result<()> {
    let mut last_msg = LAST_REQUEST.lock().unwrap();
    *last_msg = value;
    Ok(())
}
