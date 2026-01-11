use crate::client::RequestClient;
use crate::error::Result;
use crate::models::App;

pub async fn load_apps(client: &RequestClient, url: &str) -> Result<Vec<App>> {
    let apps: Vec<App> = client.get_json(url).await?;
    Ok(apps)
}
