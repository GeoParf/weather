use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct IpInfoResponse {
    city: String,
    country: String,
    loc: String, // "lat,lon"
}

pub struct Location {
    pub city: String,
    pub country: String,
    pub lat: f64,
    pub lon: f64,
}

pub async fn get_location() -> Result<Location> {
    let resp = reqwest::get("https://ipinfo.io/json").await?;
    let data: IpInfoResponse = resp.json().await?;

    let parts: Vec<&str> = data.loc.split(',').collect();
    let lat: f64 = parts[0].parse()?;
    let lon: f64 = parts[1].parse()?;

    Ok(Location {
        city: data.city,
        country: data.country,
        lat,
        lon,
    })
}
