use serde::Deserialize;
use std::collections::HashMap;
#[derive(Deserialize, Debug)]
struct Test {
    results: Vec<Contents>,
}
#[derive(Deserialize, Debug)]
struct Contents {
    id: i32,
    name: String,
    latitude: f64,
    longitude: f64,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://geocoding-api.open-meteo.com/v1/search?name=LFT&count=1")
        .await?
        .json::<Test>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}
