use clap::Parser;

mod app_error;
mod weather;
mod time_service;

#[derive(Parser, Debug)]
#[command(
    author = "Kyrylo",
    version = "0.1.0",
    about = "Simple CLI weather + time app",
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

            match time_service::get_time_for_city(&args.city).await {
                Ok(time_str) => {
                    println!("\n{}", time_str);
                }
                Err(err) => {
                    eprintln!("Не вдалося отримати час: {}", err);
                }
            }
        }
        Err(err) => {
            eprintln!("Помилка погоди: {}", err);
        }
    }
}