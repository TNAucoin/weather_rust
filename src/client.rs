use reqwest::{
    blocking::Client as HttpClient, blocking::ClientBuilder as HttpCLientBuilder,
    Error as RequestError,
};
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
    pub fn get<T: DeserializeOwned>(&self, url: &str) -> ClientResult<T> {
        let res = self.inner.get(url).send()?;
        if let Err(error) = res.error_for_status_ref() {
            return Err(error.into());
        }
        Ok(res.json::<T>()?)
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
                .connect_timeout(Duration::from_secs(CLIENT_CONNECTION_TIMEOUT)), // set other client options here..
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
    struct Response {
        url: String,
    }
    #[test]
    fn client_get() {
        let client = ClientBuilder::new().build().unwrap();
        let resp = client.get::<Response>("http://httpbin.org/get").unwrap();
        assert_eq!(resp.url, "http://httpbin.org/get".to_string())
    }
}
