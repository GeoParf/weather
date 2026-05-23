use serde::Deserialize;

//
// --- Геокодинг города через Open‑Meteo ---
//

#[derive(Deserialize, Debug)]
struct GeoResult {
    latitude: f64,
    longitude: f64,
    name: String,
}

#[derive(Deserialize, Debug)]
struct GeoResponse {
    results: Option<Vec<GeoResult>>,
}

pub async fn geocode_city(city: &str) -> anyhow::Result<(f64, f64, String)> {
    let url = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={}&count=1",
        city
    );

    let resp: GeoResponse = reqwest::get(&url).await?.json().await?;

    let result = resp
        .results
        .and_then(|mut v| v.pop())
        .ok_or_else(|| anyhow::anyhow!("Город '{}' не найден", city))?;

    Ok((result.latitude, result.longitude, result.name))
}

//
// --- Погода через Open‑Meteo ---
//

#[derive(Deserialize, Debug)]
pub struct WeatherResponse {
    pub current: CurrentWeather,
}

#[derive(Deserialize, Debug)]
pub struct CurrentWeather {
    pub temperature_2m: f64,
    pub wind_speed_10m: f64,
}

pub async fn get_weather(lat: f64, lon: f64) -> anyhow::Result<WeatherResponse> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,wind_speed_10m",
        lat, lon
    );

    let resp = reqwest::get(&url).await?.json().await?;
    Ok(resp)
}
