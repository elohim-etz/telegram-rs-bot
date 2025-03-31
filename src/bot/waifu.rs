use serde::Deserialize;
use thiserror::Error;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct WaifuResponse {
    pub url: String,
}

#[derive(Error, Debug)]
pub enum WaifuError {
    #[error("Invalid category")]
    InvalidCategory,
    #[error("Request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Parse error: {0}")]
    ParseError(#[from] serde_json::Error),
    #[error("Invalid URL: {0}")]
    UrlError(#[from] url::ParseError),
}

pub async fn get_waifu_image(category: &str) -> Result<Url, WaifuError> {
    let valid_categories = [
        "waifu", "neko", "shinobu", "megumin", "bully", "cuddle", "cry", 
        "hug", "awoo", "kiss", "lick", "pat", "smug", "bonk", "yeet", 
        "blush", "smile", "wave", "highfive", "handhold", "nom", "bite", 
        "glomp", "slap", "kill", "kick", "happy", "wink", "poke", "dance", "cringe"
    ];
    
    if !valid_categories.contains(&category) {
        return Err(WaifuError::InvalidCategory);
    }

    let api_url = format!("https://api.waifu.pics/sfw/{}", category);
    let response = reqwest::get(&api_url).await?.json::<WaifuResponse>().await?;
    Ok(Url::parse(&response.url)?)
}