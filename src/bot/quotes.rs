use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct Anime {
    pub name: String,
    #[serde(rename = "altName")]
    pub alt_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Character {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct AnimeQuote {
    pub content: String,
    pub anime: Anime,
    pub character: Character,
}

#[derive(Error, Debug)]
pub enum QuoteError {
    #[error("Missing data field in response")]
    MissingData,
    #[error("Request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Parse error: {0}")]
    ParseError(#[from] serde_json::Error),
}

pub async fn get_random_quote() -> Result<AnimeQuote, QuoteError> {
    let response = reqwest::get("https://api.animechan.io/v1/quotes/random")
        .await?
        .json::<serde_json::Value>()
        .await?;
    
    if let Some(data) = response.get("data") {
        Ok(serde_json::from_value(data.clone())?)
    } else {
        Err(QuoteError::MissingData)
    }
}