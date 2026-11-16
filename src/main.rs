use anyhow::Result;
use clap::Parser;

mod location;
mod weather;

#[derive(Parser, Debug)]
#[command(name = "weather")]
#[command(about = "Simple CLI tool for weather information (Open-Meteo)")]
struct Args {
    /// City name for weather request
    #[arg(long)]
    city: Option<String>,

    /// Show weather forecast
    #[arg(long)]
    forecast: bool,

    /// Number of forecast days (1–7)
    #[arg(long, default_value_t = 3)]
    days: u8,

    /// Force Fahrenheit output
    #[arg(long)]
    f: bool,

    /// Force Celsius output
    #[arg(long)]
    c: bool,
}

fn detect_units(country_code: &str, force_f: bool, force_c: bool) -> &'static str {
    if force_f {
        return "F";
    }
    if force_c {
        return "C";
    }

    match country_code {
        "US" | "LR" | "MM" => "F", // United States, Liberia, Myanmar
        _ => "C",
    }
}

fn to_f(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn ms_to_mph(ms: f64) -> f64 {
    ms * 2.23694
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Get location: either by city or by IP
    let (lat, lon, city_name, country_code) = if let Some(city) = &args.city {
        let geo = weather::geocode_city(city).await?;
        (geo.lat, geo.lon, geo.name.clone(), geo.country_code.clone())
    } else {
        let loc = location::get_location().await?;
        (loc.lat, loc.lon, loc.city, loc.country)
    };

    // Determine units (C or F)
    let units = detect_units(&country_code, args.f, args.c);
    let unit_symbol = if units == "F" { "°F" } else { "°C" };

    // Forecast mode
    if args.forecast {
        let days = args.days.clamp(1, 7);
        let forecast = weather::get_forecast(lat, lon, days).await?;

        println!("Weather forecast for {} ({} days):", city_name, days);

        for i in 0..forecast.daily.time.len() {
            let t_min_c = forecast.daily.temperature_2m_min[i];
            let t_max_c = forecast.daily.temperature_2m_max[i];

            let t_min = if units == "F" { to_f(t_min_c) } else { t_min_c };
            let t_max = if units == "F" { to_f(t_max_c) } else { t_max_c };

            println!(
                "{}: from {:.1}{} to {:.1}{}",
                forecast.daily.time[i], t_min, unit_symbol, t_max, unit_symbol
            );
        }

        return Ok(());
    }

    // Current weather mode
    let current = weather::get_weather(lat, lon).await?;

    let temp_c = current.current.temperature_2m;
    let temp = if units == "F" { to_f(temp_c) } else { temp_c };

    println!("Weather in {}:", city_name);
    println!("Temperature: {:.1}{}", temp, unit_symbol);

    let wind_ms = current.current.wind_speed_10m;
    let wind = if units == "F" {
        ms_to_mph(wind_ms)
    } else {
        wind_ms
    };
    let wind_unit = if units == "F" { "mph" } else { "m/s" };

    println!("Wind: {:.1} {}", wind, wind_unit);

    let (icon, description) = weather::weather_description(current.current.weather_code);

    println!("Condition: {} {}", icon, description);
    Ok(())
}
