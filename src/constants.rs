use std::sync::OnceLock;

pub const VINCHIK_CHAT:&str="1234060895";
static LAST_MESSAGE: OnceLock<i64> = OnceLock::new();

pub fn update_last_message(value: i64) {
    LAST_MESSAGE.set(value).unwrap()
}

pub fn get_last_message() -> i64{
    *LAST_MESSAGE.get().unwrap_or(&0)
}