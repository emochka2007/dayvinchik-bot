use deadpool_postgres::PoolError;
use ollama_rs::error::OllamaError;
use rand::Rng;
use rust_tdlib::tdjson::set_log_verbosity_level;
use std::backtrace::Backtrace;
use std::env::VarError;
use std::num::ParseIntError;
use thiserror::Error;

pub type ChatId = i64;
pub type MessageId = i64;
pub type FileId = i32;

pub type StdResult = std::io::Result<()>;

#[derive(Error, Debug)]
pub enum BotError {
    #[error("Postgres error: {source}")]
    Postgres {
        #[from]
        source: tokio_postgres::Error,
        #[backtrace]
        backtrace: Backtrace,
    },
    #[error("IO error: {source}")]
    Io {
        #[from]
        source: std::io::Error,
        #[backtrace]
        backtrace: Backtrace,
    },
    #[error("Serde JSON error: {source}")]
    SerdeJson {
        #[from]
        source: serde_json::Error,
        #[backtrace]
        backtrace: Backtrace,
    },
    #[error("Serde JSON error: {source}")]
    EnvVar {
        #[from]
        source: VarError,
        #[backtrace]
        backtrace: Backtrace,
    },
    #[error("ParseInt error: {source}")]
    ParseError {
        #[from]
        source: ParseIntError,
        #[backtrace]
        backtrace: Backtrace,
    },
    #[error("Pool error: {source}")]
    PoolError {
        #[from]
        source: PoolError,
        #[backtrace]
        backtrace: Backtrace,
    },
    #[error("Reqwest error: {source}")]
    ReqwestError {
        #[from]
        source: reqwest::Error,
        #[backtrace]
        backtrace: Backtrace,
    },
    #[error("Ollama error: {source}")]
    OllamaError {
        #[from]
        source: OllamaError,
        #[backtrace]
        backtrace: Backtrace,
    },
}

pub fn env_init() {
    // tokio: console enable console_subscriber::init();
    set_log_verbosity_level(0);
    env_logger::init();
    dotenvy::dotenv().unwrap_or_else(|_e| {
        panic!("Not enable to initialize dotenvy");
    });
}

pub fn random_number(from: i64, to: i64) -> i64 {
    let mut rng = rand::rng();
    rng.random_range(from..to)
}
