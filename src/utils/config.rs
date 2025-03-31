use dotenv::dotenv;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub telegram_bot_token: String,
    pub log_level: String,
}

impl Config {
    pub fn new() -> anyhow::Result<Self> {
        dotenv().ok();

        Ok(Config {
            telegram_bot_token: env::var("TELEGRAM_BOT_TOKEN")?,
            log_level: env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
        })
    }
}