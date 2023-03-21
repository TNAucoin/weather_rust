use serde::Deserialize;
use thiserror::Error;

use crate::client::{ClientBuilder, ClientError};

pub const URL_WEATHER: &str = "http://api.openweathermap.org/data/2.5/weather";

#[derive(Debug, Error)]
pub enum WeatherClientError {
    #[error(transparent)]
    Client(#[from] ClientError),
}

#[derive(Debug)]
pub struct WeatherClient {
    inner: ClientBuilder,
}

impl WeatherClient {
    pub fn new() -> Self {
        Self {
            inner: ClientBuilder::new(),
        }
    }

    // pub async fn get(self, coordinates: (f64, f64)) -> Result<Weather, WeatherClientError> {
    //     let url = format!(
    //         "{}?lat={lat}&lon={lon}&appid={appid}",
    //         URL_WEATHER,
    //         lat = coordinates.0,
    //         lon = coordinates.1,
    //         appid = ""
    //     );
    //     let res = self.inner.build()?.get(&url).await?;
    // }
}
