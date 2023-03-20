use reqwest::{Client as HttpClient, ClientBuilder as HttpCLientBuilder, Error as RequestError};
use serde::de::DeserializeOwned;
use std::{result, time::Duration};
use thiserror::Error;

pub static CLIENT_CONNECTION_TIMEOUT: u64 = 5;
pub static CLIENT_TIMEOUT: u64 = 30;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error(transparent)]
    Reqwest(#[from] RequestError),
}

pub type ClientResult<T> = result::Result<T, ClientError>;

#[derive(Clone, Debug)]
pub struct Client {
    inner: HttpClient,
}

impl Client {
    pub async fn get<T: DeserializeOwned>(&self, url: &str) -> ClientResult<T> {
        let res = self.inner.get(url).send().await?;
        if let Err(error) = res.error_for_status_ref() {
            return Err(error.into());
        }
        Ok(res.json::<T>().await?)
    }
}

#[derive(Debug)]
pub struct ClientBuilder {
    inner: HttpCLientBuilder,
}

pub type ClientBuilderResult = result::Result<Client, ClientError>;

impl ClientBuilder {
    pub fn new() -> Self {
        Self {
            inner: HttpCLientBuilder::new()
                .timeout(Duration::from_secs(CLIENT_TIMEOUT))
                .connect_timeout(Duration::from_secs(CLIENT_CONNECTION_TIMEOUT)),
            // set other client options here..
        }
    }

    pub fn build(self) -> ClientBuilderResult {
        Ok(Client {
            inner: self.inner.build()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::ClientBuilder;
    use serde::Deserialize;
    #[derive(Debug, Deserialize)]
    struct Crate {
        description: String,
    }

    #[derive(Debug, Deserialize)]
    struct Crates {
        #[serde(alias = "crate")]
        crate_: Crate,
    }

    #[tokio::test]
    async fn client_get() {
        let client = ClientBuilder::new().build().unwrap();
        let crates = client
            .get::<Crates>("https://crates.io/api/v1/crates/reqwest")
            .await
            .unwrap();
        assert_eq!(
            crates.crate_.description,
            "Crates to make HTTP network requests.".to_string()
        )
    }
}
