use clap::Parser;

mod error;
mod time_service;
mod weather;

use error::AppError;

#[derive(Parser)]
#[command(
    author = "Kyrylo Savchuk",
    version = "0.1.0",
    about = "Simple CLI app: weather + time by city"
)]
struct Args {
    city: String,
}

async fn run(city: String) -> Result<(), AppError> {
    let weather = weather::get_weather_for_city(&city).await?;
    let time = time_service::get_time_for_city(&city).await?;

    println!("==============================");
    println!("Місто: {}", city.to_uppercase());
    println!("------------------------------");
    println!("Температура: {} °C", weather.temperature);
    println!("Опис: {}", weather.description);
    println!("Час: {}", time);
    println!("==============================");

    Ok(())
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if let Err(e) = run(args.city).await {
        eprintln!("Помилка: {}", e);
    }
}
