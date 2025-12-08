
use clap::Parser;

mod weather;

#[derive(Parser, Debug)]
#[command(
    author = "Kyrylo",
    version = "0.1.0",
    about = "Simple CLI weather app (like Gismeteo) on Rust",
    long_about = None
)]
struct Args {
    /// Назва міста
    city: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!("Місто з аргументів: {}", args.city);

    match weather::get_weather_for_city(&args.city).await {
        Ok(info) => {
            println!("\nПогода для: {}", info.city);
            println!("Температура: {} °C", info.temperature);
            println!("Опис: {}", info.description);
            println!("Час вимірювання: {}", info.time);
        }
        Err(err) => {
            eprintln!("Помилка: {}", err);
        }
    }
}