use crate::error::Result;
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::time::Duration;

#[macro_export]
macro_rules! github_latest_release_url {
    ($owner:expr, $repo:expr) => {
        format!(
            "https://api.github.com/repos/{}/{}/releases/latest",
            $owner, $repo
        )
    };
}

pub struct RequestClient {
    pub client: Client,
}

impl RequestClient {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("grab/0.1.0")
            .build()?;

        Ok(Self { client })
    }

    pub async fn get_json<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        let response = self.client.get(url).send().await?;
        let json = response.json::<T>().await?;
        Ok(json)
    }

    pub async fn download_bytes(&self, url: &str) -> Result<Vec<u8>> {
        let response = self.client.get(url).send().await?;
        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }
}
