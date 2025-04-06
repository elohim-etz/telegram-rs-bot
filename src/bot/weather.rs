use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct WeatherResponse {
    pub location: Location,
    pub current: CurrentWeather,
}

#[derive(Debug, Deserialize)]
pub struct Location {
    pub name: String,
    pub region: String,
    pub country: String,
    pub localtime: String,
}

#[derive(Debug, Deserialize)]
pub struct CurrentWeather {
    pub temp_c: f32,
    pub condition: WeatherCondition,
    pub wind_kph: f32,
    pub humidity: i32,
    pub feelslike_c: f32,
}

#[derive(Debug, Deserialize)]
pub struct WeatherCondition {
    pub text: String,
    pub icon: String,
}

#[derive(Error, Debug)]
pub enum WeatherError {
    #[error("Request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Location not found")]
    LocationNotFound,
}

pub async fn get_weather(location: &str) -> Result<WeatherResponse, WeatherError> {
    let api_key = "1bb7e1fa17ac48d994d160049250204"; // Consider moving to config
    let url = format!(
        "https://api.weatherapi.com/v1/current.json?key={}&q={}",
        api_key, location
    );

    let response = reqwest::get(&url).await?;
    
    if response.status().is_success() {
        Ok(response.json::<WeatherResponse>().await?)
    } else {
        Err(WeatherError::LocationNotFound)
    }
}