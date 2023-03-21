use crate::client::{ClientBuilder, ClientError};
use serde::Deserialize;
use thiserror::Error;

const LOCATION_URL: &str = "http://api.openweathermap.org/geo/1.0/";

#[derive(Deserialize, Debug, Clone)]
pub struct Location {
    name: String,
    lat: f64,
    lon: f64,
    country: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(transparent)]
pub struct LocationResponse {
    locations: Option<Location>,
}

impl From<LocationResponse> for Location {
    fn from(response: LocationResponse) -> Self {
        let location = &response.locations.unwrap();
        Self {
            name: location.name.to_string(),
            lat: location.lat,
            lon: location.lon,
            country: location.country.to_string(),
        }
    }
}

#[derive(Error, Debug)]
pub enum LocationClientError {
    #[error(transparent)]
    Client(#[from] ClientError),
}

pub struct LocationClient {
    inner: ClientBuilder,
}

impl LocationClient {
    pub fn new() -> Self {
        Self {
            inner: ClientBuilder::new(),
        }
    }

    pub fn get_location(self, zip: &u64) -> Result<Location, LocationClientError> {
        let url = format!(
            "{}zip?zip={zip_code}&limit=1&appid=71557c71f0c4da3dcf14815d39a6918c",
            LOCATION_URL,
            zip_code = zip,
        );
        println!("{}", url);
        let res: LocationResponse = self.inner.build()?.get::<LocationResponse>(&url)?;
        println!("{:#?}", res);
        Ok(res.into())
    }
}
