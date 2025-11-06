use crate::error::Result;
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::time::Duration;

const APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);

pub fn github_latest_release_url(owner: &str, repo: &str) -> String {
    format!(
        "https://api.github.com/repos/{}/{}/releases/latest",
        owner, repo
    )
}

#[derive(Debug, Clone)]
pub struct RequestClient {
    pub client: Client,
}

impl RequestClient {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent(APP_USER_AGENT)
            .build()?;

        Ok(Self { client })
    }

    pub async fn get_json<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        let response = self.client.get(url).send().await?;
        let response = response.error_for_status()?;
        let json = response.json::<T>().await?;
        Ok(json)
    }

    pub async fn download_bytes(&self, url: &str) -> Result<Vec<u8>> {
        let response = self.client.get(url).send().await?;
        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }
}
