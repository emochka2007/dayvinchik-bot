use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::td::td_command_map::Commands;

lazy_static! {
    static ref LAST_MESSAGE: Mutex<i64> = Mutex::new(0);
    // todo what is static ref
    static ref LAST_TDLIB_CALL: Mutex<Commands> = Mutex::new(Commands::Unknown);
}
pub const VINCHIK_CHAT:&str="1234060895";

pub fn update_last_message(value: i64) {
    let mut last_msg = LAST_MESSAGE.lock().unwrap();
    *last_msg = value;
}

pub fn get_last_message() -> i64{
    *LAST_MESSAGE.lock().unwrap()
}

pub fn get_last_tdlib_call() -> Commands {
    *LAST_TDLIB_CALL.lock().expect("Error")
}

pub fn update_last_tdlib_call(value: Commands) {
    let mut last_msg = LAST_TDLIB_CALL.lock().unwrap();
    *last_msg = value;
}
