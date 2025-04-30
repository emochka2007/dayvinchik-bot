use log::error;
use rand::Rng;
use rust_tdlib::tdjson::set_log_verbosity_level;

pub type ChatId = i64;
pub type MessageId = i64;
pub type FileId = i32;

pub type StdResult = std::io::Result<()>;

pub fn env_init() {
    // tokio: console enable console_subscriber::init();
    set_log_verbosity_level(0);
    env_logger::init();
    dotenvy::dotenv().unwrap_or_else(|e| {
        error!("{:?}", e);
        panic!("Not enable to initialize dotenvy");
    });
}

pub fn random_number(from: i64, to: i64) -> i64 {
    let mut rng = rand::rng();
    rng.random_range(from..to)
}
