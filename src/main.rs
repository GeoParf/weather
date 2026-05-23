use clap::Parser;

mod location;
mod weather;

#[derive(Parser, Debug)]
#[command(name = "weather")]
#[command(about = "CLI утилита для просмотра погоды (Open‑Meteo)")]
struct Args {
    /// Город для запроса погоды
    #[arg(long)]
    city: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // 1. Определяем координаты
    let (lat, lon, city_name) = if let Some(city) = args.city {
        weather::geocode_city(&city).await?
    } else {
        location::get_location().await?
    };

    // 2. Получаем погоду
    let weather = weather::get_weather(lat, lon).await?;

    println!("Погода в {}:", city_name);
    println!("Температура: {}°C", weather.current.temperature_2m);
    println!("Ветер: {} м/с", weather.current.wind_speed_10m);

    Ok(())
}
