use thiserror::Error;

#[derive(Error, Debug)]
pub enum BotError {
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Telegram API error: {0}")]
    TelegramError(String),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Generic error: {0}")]
    GenericError(String),
}

impl From<std::env::VarError> for BotError {
    fn from(err: std::env::VarError) -> Self {
        BotError::ConfigError(err.to_string())
    }
}

impl From<teloxide::RequestError> for BotError {
    fn from(err: teloxide::RequestError) -> Self {
        BotError::TelegramError(err.to_string())
    }
}

impl From<anyhow::Error> for BotError {
    fn from(err: anyhow::Error) -> Self {
        BotError::GenericError(err.to_string())
    }
}
