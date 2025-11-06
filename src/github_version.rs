use crate::client::RequestClient;
use crate::client::github_latest_release_url;
use crate::error::Result;
use crate::models::App;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Version {
    pub tag_name: String,
    pub html_url: String,
}

impl Version {
    pub async fn latest(app: &App, client: &RequestClient) -> Result<Version> {
        let url = github_latest_release_url(&app.owner, &app.repo);
        let version: Version = client.get_json(&url).await?;
        Ok(version)
    }
}
