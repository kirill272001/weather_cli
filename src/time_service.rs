use crate::error::AppError;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct WorldTimeApiResponse {
    datetime: String,
    utc_offset: String,
    timezone: String,
}

fn city_to_timezone(city: &str) -> Option<&'static str> {
    match city.to_lowercase().as_str() {
        "kyiv" | "kiev" | "київ" | "lviv" | "львів" => Some("Europe/Kiev"),
        "london" => Some("Europe/London"),
        "berlin" | "берлін" => Some("Europe/Berlin"),
        _ => None,
    }
}

pub async fn get_time_for_city(city: &str) -> Result<String, AppError> {
    let timezone = city_to_timezone(city).ok_or_else(|| AppError::UnknownCity(city.to_string()))?;

    let url = format!("https://worldtimeapi.org/api/timezone/{}", timezone);

    let client = Client::new();

    let resp = client.get(&url).send().await?;

    let data: WorldTimeApiResponse = resp.json().await?;

    let short = data
        .datetime
        .chars()
        .take(16)
        .collect::<String>()
        .replace('T', " ");

    let result = format!("Local time in {}: {} (UTC{})", city, short, data.utc_offset);

    Ok(result)
}
