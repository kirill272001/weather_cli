use clap::Parser;
use reqwest::Client;
use serde::Deserialize;

#[derive(Parser, Debug)]
#[command(
    author = "Kyrylo",
    version = "0.1.0",
    about = "Simple CLI weather app",
    long_about = None
)]
struct Args {
    city: String,
}

#[derive(Deserialize, Debug)]
struct CurrentWeather {
    temperature: f64,
    windspeed: f64,
}

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    current_weather: CurrentWeather,
}
fn get_coords(city: &str) -> (f64, f64) {
    match city.to_lowercase().as_str() {
        "kyiv" | "kiev" | "київ" => (50.45, 30.52),
        "lviv" | "львів"        => (49.84, 24.03),
        "odesa" | "odessa" | "одеса" => (46.48, 30.73),
        _ => {
            println!("Не знаю такого міста, використовую Київ за замовчуванням");
            (50.45, 30.52)
        }
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

   println!("Місто з аргументів: {}", args.city);


    let (lat, lon) = get_coords(&args.city);

    println!("Використовуємо координати: lat = {}, lon = {}\n", lat, lon);

    if let Err(err) = fetch_weather_for_city(lat, lon).await {
        eprintln!("Помилка при отриманні погоди: {}", err);
}
}

async fn fetch_weather_for_city(
    latitude: f64,
    longitude: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true",
        latitude, longitude
    );

    let resp = client.get(&url).send().await?;
    let data: WeatherResponse = resp.json().await?;

    println!("Поточна температура: {} °C", data.current_weather.temperature);
    println!("Швидкість вітру: {} м/с", data.current_weather.windspeed);

    Ok(())
}