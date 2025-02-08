use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref LAST_MESSAGE: Mutex<i64> = Mutex::new(0);
    static ref LAST_TDLIB_CALL: Mutex<String> = Mutex::new(String::new());
}
pub const VINCHIK_CHAT:&str="1234060895";

pub fn update_last_message(value: i64) {
    let mut last_msg = LAST_MESSAGE.lock().unwrap();
    *last_msg = value;
}

pub fn get_last_message() -> i64{
    *LAST_MESSAGE.lock().unwrap()
}

pub fn get_last_tdlib_call() -> String {
    LAST_TDLIB_CALL.lock().unwrap().to_string()
}

pub fn update_last_tdlib_call(value: String) {
    let mut last_msg = LAST_TDLIB_CALL.lock().unwrap();
    *last_msg = value;
}
