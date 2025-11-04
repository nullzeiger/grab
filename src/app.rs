use crate::client::RequestClient;
use crate::compare_release_version::Compare;
use crate::error::{GrabError, Result};
use crate::github_release;
use crate::github_version::Version;
use crate::models::App;
use crate::storage;
use std::process::Output;
use std::sync::Arc;
use tokio::process::Command;

async fn command(name: &str, flag: &str) -> Result<Output> {
    let output = Command::new(name)
        .args([flag])
        .output()
        .await
        .map_err(GrabError::Io)?;

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(GrabError::CommandFailed {
            command: name.to_string(),
            exit_code: output.status.code().unwrap_or(-1),
            stderr: error_message,
        });
    }

    Ok(output)
}

pub fn add_app(app: App) -> Result<()> {
    let mut apps = storage::load_apps()?;
    apps.push(app);
    storage::save_apps(&apps)
}

pub async fn check_apps() -> Result<()> {
    let request_client = RequestClient::new()?;
    let client = Arc::new(request_client);
    let apps = storage::load_apps()?;
    let mut tasks = tokio::task::JoinSet::new();

    for app in apps {
        let client = Arc::clone(&client);

        tasks.spawn(async move {
            let version_output = command(&app.name, &app.version_flag).await?;
            let latest_version = Version::latest(&app, &client).await?;
            let local_version = String::from_utf8_lossy(&version_output.stdout)
                .trim()
                .to_string();

            let compare = Compare::new(&latest_version.tag_name, &local_version)?;

            println!(
                "Local version of {}: {}\nLatest version in GitHub repo: {}\nURL: {}\n",
                app.name, local_version, latest_version.tag_name, latest_version.html_url
            );

            if !compare.is_latest {
                println!(
                    "Update available for {} — downloading latest release...\n",
                    app.name
                );
                github_release::download_latest_asset(
                    &client,
                    &app.owner,
                    &app.repo,
                    &app.asset_pattern,
                )
                .await?;
            } else {
                println!("{} is already up to date.\n", app.name);
            }

            Ok::<(), GrabError>(())
        });
    }

    while let Some(result) = tasks.join_next().await {
        result.map_err(GrabError::TaskJoin)??;
    }

    Ok(())
}

pub async fn download_apps(app_name: Option<&App>) -> Result<()> {
    let request_client = RequestClient::new()?;
    let client = Arc::new(request_client);

    if let Some(app) = app_name {
        github_release::download_latest_asset(&client, &app.owner, &app.repo, &app.asset_pattern)
            .await?;
        return Ok(());
    }

    let apps = storage::load_apps()?;
    let mut tasks = tokio::task::JoinSet::new();

    for app in apps {
        let client = Arc::clone(&client);
        tasks.spawn(async move {
            github_release::download_latest_asset(
                &client,
                &app.owner,
                &app.repo,
                &app.asset_pattern,
            )
            .await
        });
    }

    while let Some(result) = tasks.join_next().await {
        result.map_err(GrabError::TaskJoin)??;
    }

    Ok(())
}

pub fn remove_app(index: usize) -> Result<()> {
    if index == 0 {
        return Err(GrabError::InvalidInput(
            "Index must be greater than 0".to_string(),
        ));
    }

    let mut apps = storage::load_apps()?;
    if index > apps.len() {
        return Err(GrabError::NotFound(format!(
            "No app found at index {}",
            index
        )));
    }

    apps.remove(index - 1);
    storage::save_apps(&apps)
}

pub fn search_apps(query: &str) -> Result<Vec<(usize, App)>> {
    let apps = storage::load_apps()?;
    let query_lower = query.to_lowercase();

    let results = apps
        .into_iter()
        .enumerate()
        .filter_map(|(i, app)| {
            if app.name.to_lowercase().contains(&query_lower)
                || app.owner.to_lowercase().contains(&query_lower)
                || app.repo.to_lowercase().contains(&query_lower)
            {
                Some((i + 1, app))
            } else {
                None
            }
        })
        .collect();

    Ok(results)
}

pub fn list_apps(limit: Option<usize>) -> Result<Vec<(usize, App)>> {
    let apps = storage::load_apps()?;

    let mut results: Vec<(usize, App)> = apps
        .into_iter()
        .enumerate()
        .map(|(i, app)| (i + 1, app))
        .collect();

    if let Some(lim) = limit {
        results.truncate(lim);
    }

    Ok(results)
}
