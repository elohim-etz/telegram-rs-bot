use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct Slip {
    pub advice: String,
}

#[derive(Debug, Deserialize)]
pub struct AdviceResponse {
    pub slip: Slip,
}

#[derive(Error, Debug)]
pub enum AdviceError {
    #[error("Missing slip field in response")]
    MissingSlip,
    #[error("Request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Parse error: {0}")]
    ParseError(#[from] serde_json::Error),
}

pub async fn get_daily_advice() -> Result<String, AdviceError> {
    let response = reqwest::get("https://api.adviceslip.com/advice")
        .await?
        .json::<serde_json::Value>()
        .await?;
    
    if let Some(slip) = response.get("slip") {
        Ok(serde_json::from_value::<Slip>(slip.clone())?.advice)
    } else {
        Err(AdviceError::MissingSlip)
    }
}