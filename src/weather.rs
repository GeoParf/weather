use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CurrentWeatherResponse {
    pub current: CurrentWeather,
}

#[derive(Deserialize, Debug)]
pub struct CurrentWeather {
    pub temperature_2m: f64,
    pub wind_speed_10m: f64,
    pub weather_code: u8,
}

#[derive(Deserialize, Debug)]
pub struct ForecastResponse {
    pub daily: DailyForecast,
}

#[derive(Deserialize, Debug)]
pub struct DailyForecast {
    pub time: Vec<String>,
    pub temperature_2m_max: Vec<f64>,
    pub temperature_2m_min: Vec<f64>,
}

#[derive(Deserialize, Debug)]
pub struct GeocodeResponse {
    pub results: Vec<GeocodeResult>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GeocodeResult {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub country_code: String,
}

pub struct GeoData {
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub country_code: String,
}

pub async fn geocode_city(city: &str) -> Result<GeoData> {
    let url = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={}&count=1&language=en&format=json",
        city
    );

    let resp = reqwest::get(&url).await?;
    let data: GeocodeResponse = resp.json().await?;

    let first = data
        .results
        .get(0)
        .ok_or_else(|| anyhow::anyhow!("City not found"))?;

    Ok(GeoData {
        name: first.name.clone(),
        lat: first.latitude,
        lon: first.longitude,
        country_code: first.country_code.clone(),
    })
}

pub async fn get_weather(lat: f64, lon: f64) -> Result<CurrentWeatherResponse> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,wind_speed_10m,weather_code&timezone=auto",
        lat, lon
    );

    let resp = reqwest::get(&url).await?;
    let data: CurrentWeatherResponse = resp.json().await?;
    Ok(data)
}

pub fn weather_description(code: u8) -> (&'static str, &'static str) {
    match code {
        0 => ("☀️", "Sunny"),
        1 | 2 => ("🌤️", "Partly Cloudy"),
        3 => ("☁️", "Cloudy"),

        45 | 48 => ("🌫️", "Fog"),

        51 | 53 | 55 => ("🌦️", "Drizzle"),

        61 | 63 | 65 => ("🌧️", "Rain"),

        71 | 73 | 75 => ("❄️", "Snow"),

        80 | 81 | 82 => ("🌦️", "Rain Showers"),

        95 => ("⛈️", "Thunderstorm"),

        96 | 99 => ("⛈️", "Thunderstorm with Hail"),

        _ => ("❓", "Unknown"),
    }
}

pub async fn get_forecast(lat: f64, lon: f64, days: u8) -> Result<ForecastResponse> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&daily=temperature_2m_max,temperature_2m_min&forecast_days={}&timezone=auto",
        lat, lon, days
    );

    let resp = reqwest::get(&url).await?;
    let data: ForecastResponse = resp.json().await?;
    Ok(data)
}
