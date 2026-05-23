use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct IpInfo {
    city: Option<String>,
    loc: Option<String>,
}

pub async fn get_location() -> anyhow::Result<(f64, f64, String)> {
    let resp: IpInfo = reqwest::get("https://ipinfo.io/json").await?.json().await?;

    let loc = resp
        .loc
        .ok_or_else(|| anyhow::anyhow!("Поле 'loc' отсутствует"))?;
    let parts: Vec<&str> = loc.split(',').collect();

    let lat = parts[0].parse::<f64>()?;
    let lon = parts[1].parse::<f64>()?;

    let city = resp.city.unwrap_or("Unknown".into());

    Ok((lat, lon, city))
}
