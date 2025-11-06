use crate::client::RequestClient;
use crate::error::{GrabError, Result};
use crate::client::github_latest_release_url;
use serde::Deserialize;
use tokio::fs::File as TokioFile;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Deserialize)]
pub struct Release {
    pub assets: Vec<Asset>,
}

#[derive(Debug, Deserialize)]
pub struct Asset {
    pub name: String,
    pub browser_download_url: String,
}

async fn get_latest_release(client: &RequestClient, owner: &str, repo: &str) -> Result<Release> {
    let url = github_latest_release_url(owner, repo);
    let release: Release = client.get_json(&url).await?;
    Ok(release)
}

pub async fn download_latest_asset(
    client: &RequestClient,
    owner: &str,
    repo: &str,
    asset_pattern: &str,
) -> Result<()> {
    let release = get_latest_release(client, owner, repo).await?;

    let asset = release
        .assets
        .iter()
        .find(|a| a.name.contains(asset_pattern))
        .ok_or_else(|| GrabError::AssetNotFound(asset_pattern.to_string()))?;

    println!("Downloading: {}", asset.name);

    let content = client.download_bytes(&asset.browser_download_url).await?;

    let mut file = TokioFile::create(&asset.name).await?;
    file.write_all(&content).await?;

    println!("Download completed: {}", asset.name);
    Ok(())
}
