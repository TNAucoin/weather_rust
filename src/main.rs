use clap::Parser;
use serde::Deserialize;

use weather_rust::{args::Args, location::client::LocationClient};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    println!("{:#?}", args.zip);
    let location = LocationClient::new().get_location(&args.zip)?;
    println!("{:#?}", location);
    // let resp = reqwest::get("https://geocoding-api.open-meteo.com/v1/search?name=LFT&count=1")
    //     .await?
    //     .json::<Test>()
    //     .await?;
    // println!("{:#?}", resp);
    Ok(())
}
