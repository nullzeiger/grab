use crate::error::{GrabError, Result};
use crate::models::App;
use directories::UserDirs;
use std::fs::File;
use std::path::PathBuf;

const JSON_FILE: &str = ".apps.json";

fn get_json_path() -> Result<PathBuf> {
    if let Some(user_dirs) = UserDirs::new() {
        Ok(user_dirs.home_dir().join(JSON_FILE))
    } else {
        Err(GrabError::NotFound(
            "Could not find home directory".to_string(),
        ))
    }
}

pub fn load_apps() -> Result<Vec<App>> {
    let path = get_json_path()?;
    if !path.exists() {
        return Ok(Vec::new());
    }
    let file = File::open(path)?;
    serde_json::from_reader(file).map_err(|e| GrabError::Parse(e.to_string()))
}

pub fn save_apps(apps: &[App]) -> Result<()> {
    let path = get_json_path()?;
    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, apps).map_err(|e| GrabError::Parse(e.to_string()))
}
