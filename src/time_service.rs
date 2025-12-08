use crate::app_error::AppError;
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
    // 1. Місто → таймзона
    let timezone = city_to_timezone(city)
        .ok_or_else(|| AppError::UnknownCity(city.to_string()))?;

    // 2. Формуємо URL для WorldTimeAPI
    let url = format!("https://worldtimeapi.org/api/timezone/{}", timezone);

    // 3. Створюємо HTTP-клієнт
    let client = Client::new();

    // 4. Робимо GET-запит
    let resp = client.get(&url).send().await?;

    // 5. Парсимо JSON у нашу структуру
    let data: WorldTimeApiResponse = resp.json().await?;

    // 6. Трішки форматуємо datetime:
    // "2025-02-15T14:37:12.123456+02:00" -> "2025-02-15 14:37"
    let short = data
        .datetime
        .chars()
        .take(16)
        .collect::<String>()
        .replace('T', " ");

    // 7. Формуємо красивий текст
    let result = format!(
        "Local time in {}: {} (UTC{})",
        city,
        short,
        data.utc_offset
    );

    Ok(result)
}