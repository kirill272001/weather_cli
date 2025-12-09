use crate::error::AppError;
use reqwest::Client;
use serde::Deserialize;
#[derive(Debug)]
pub struct WeatherInfo {
    pub city: String,
    pub temperature: f64,
    pub description: String,
    pub time: String,
}

#[derive(Debug, Deserialize)]
struct CurrentWeather {
    temperature: f64,
    windspeed: f64,
    weathercode: i32,
    time: String,
}

#[derive(Debug, Deserialize)]
struct WeatherApiResponse {
    current_weather: CurrentWeather,
}

fn coords_for_city(city: &str) -> Option<(f64, f64)> {
    match city.to_lowercase().as_str() {
        "kyiv" | "kiev" | "київ" => Some((50.45, 30.52)),
        "lviv" | "львів" => Some((49.84, 24.03)),
        "odesa" | "odessa" | "одеса" => Some((46.48, 30.73)),
        "berlin" | "берлін" => Some((52.52, 13.41)),
        _ => None,
    }
}

pub async fn get_weather_for_city(city: &str) -> Result<WeatherInfo, AppError> {
    let (lat, lon) =
        coords_for_city(city).ok_or_else(|| AppError::UnknownCity(city.to_string()))?;

    let client = Client::new();

    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true",
        lat, lon
    );

    let resp = client.get(&url).send().await?;
    let api: WeatherApiResponse = resp.json().await?;

    let cw = api.current_weather;

    let description = format!("weather code: {}", cw.weathercode);

    let info = WeatherInfo {
        city: city.to_string(),
        temperature: cw.temperature,
        description,
        time: cw.time,
    };

    Ok(info)
}
