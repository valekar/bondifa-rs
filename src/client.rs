use crate::api::API;
use crate::errors::*;
use reqwest::blocking::Response;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;

#[derive(Clone)]
struct Client {
    host: String,
    inner_client: reqwest::blocking::Client,
}

impl Client {
    pub fn new(host: String) -> Self {
        Client {
            host,
            inner_client: reqwest::blocking::Client::builder()
                .pool_idle_timeout(None)
                .build()
                .unwrap(),
        }
    }

    pub fn get<T: DeserializeOwned>(&self, end_point: API, request: Option<String>) -> Result<T> {
        let mut url: String = format!("{}{}", self.host, String::from(end_point));

        if let Some(request) = request {
            if !request.is_empty() {
                url.push_str(format!("?{}", request).as_str());
            }
        }

        let client = &self.inner_client;
        let response = client.get(url.as_str()).send()?;

        self.handler(response)
    }

    fn handler<T: DeserializeOwned>(&self, response: Response) -> Result<T> {
        match response.status() {
            StatusCode::OK => Ok(response.json::<T>()?),
            StatusCode::INTERNAL_SERVER_ERROR => {
                bail!("Internal Server Error");
            }
            StatusCode::SERVICE_UNAVAILABLE => {
                bail!("Service Unavailable");
            }
            StatusCode::UNAUTHORIZED => {
                bail!("Unauthorized");
            }
            StatusCode::BAD_REQUEST => {
                let error: String = response.json()?;
                Err(error.into())
            }
            s => {
                bail!(format!("Received response: {:?}", s));
            }
        }
    }
}
