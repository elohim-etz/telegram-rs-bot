use serde::Deserialize;
use thiserror::Error;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct WaifuImage {
    pub url: String,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Deserialize)]
pub struct Tag {
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct WaifuResponse {
    pub images: Vec<WaifuImage>,
}

#[derive(Error, Debug)]
pub enum WaifuError {
    #[error("Request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Invalid URL: {0}")]
    UrlError(#[from] url::ParseError),
    #[error("No images found")]
    NoImages,
}

pub async fn get_waifu_image(tag: &str) -> Result<(String, String, String), WaifuError> {
    // Build URL with tag
    let mut url = Url::parse("https://api.waifu.im/search")?;
    if !tag.is_empty() {
        url.query_pairs_mut().append_pair("included_tags", tag);
    }

    // Make request
    let response = reqwest::get(url)
        .await?
        .json::<WaifuResponse>()
        .await?;

    // Get first image
    let image = response.images.first().ok_or(WaifuError::NoImages)?;
    let caption = image.tags.first()
        .and_then(|t| t.description.as_ref())
        .map(|d| d.as_str())
        .unwrap_or("Here's your waifu!");

    Ok((image.url.clone(), caption.to_string(), image.url.clone()))
}