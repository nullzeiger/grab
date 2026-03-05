use crate::error::{GrabError, Result};
use crate::models::App;
use directories::UserDirs;
use std::fs::File;
use std::path::{Path, PathBuf};

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
    load_apps_from(&path)
}

pub fn save_apps(apps: &[App]) -> Result<()> {
    let path = get_json_path()?;
    save_apps_to(&path, apps)
}

pub fn load_apps_from(path: &Path) -> Result<Vec<App>> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    let file = File::open(path)?;
    serde_json::from_reader(file).map_err(|e| GrabError::Parse(e.to_string()))
}

pub fn save_apps_to(path: &Path, apps: &[App]) -> Result<()> {
    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, apps).map_err(|e| GrabError::Parse(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::App;
    use tempfile::tempdir;

    fn make_app(name: &str, owner: &str, repo: &str) -> App {
        App {
            name: name.to_string(),
            owner: owner.to_string(),
            repo: repo.to_string(),
            version_flag: "--version".to_string(),
            asset_pattern: format!("{}-linux", name),
        }
    }

    #[test]
    fn test_load_apps_from_missing_file_returns_empty_vec() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("nonexistent.json");

        let apps = load_apps_from(&path).unwrap();
        assert!(apps.is_empty());
    }

    #[test]
    fn test_save_and_load_apps_roundtrip() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("apps.json");

        let apps = vec![
            make_app("ripgrep", "BurntSushi", "ripgrep"),
            make_app("fd", "sharkdp", "fd"),
        ];

        save_apps_to(&path, &apps).unwrap();
        let loaded = load_apps_from(&path).unwrap();

        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].name, "ripgrep");
        assert_eq!(loaded[0].owner, "BurntSushi");
        assert_eq!(loaded[0].repo, "ripgrep");
        assert_eq!(loaded[1].name, "fd");
    }

    #[test]
    fn test_save_empty_vec_and_load() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("apps.json");

        save_apps_to(&path, &[]).unwrap();
        let loaded = load_apps_from(&path).unwrap();

        assert!(loaded.is_empty());
    }

    #[test]
    fn test_save_overwrites_existing_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("apps.json");

        let first = vec![make_app("bat", "sharkdp", "bat")];
        save_apps_to(&path, &first).unwrap();

        let second = vec![
            make_app("eza", "eza-community", "eza"),
            make_app("zoxide", "ajeetdsouza", "zoxide"),
        ];
        save_apps_to(&path, &second).unwrap();

        let loaded = load_apps_from(&path).unwrap();
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].name, "eza");
        assert_eq!(loaded[1].name, "zoxide");
    }

    #[test]
    fn test_load_apps_from_invalid_json_returns_error() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("bad.json");

        std::fs::write(&path, b"not valid json {{{{").unwrap();

        let result = load_apps_from(&path);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GrabError::Parse(_)));
    }

    #[test]
    fn test_saved_file_is_valid_pretty_json() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("apps.json");

        let apps = vec![make_app("delta", "dandavison", "delta")];
        save_apps_to(&path, &apps).unwrap();

        let raw = std::fs::read_to_string(&path).unwrap();
        // Pretty-printed JSON contains newlines and indentation
        assert!(raw.contains('\n'));
        assert!(raw.contains("  "));
        // Verify it parses back as valid JSON
        let parsed: serde_json::Value = serde_json::from_str(&raw).unwrap();
        assert!(parsed.is_array());
    }

    #[test]
    fn test_load_preserves_all_fields() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("apps.json");

        let app = App {
            name: "mycli".to_string(),
            owner: "myorg".to_string(),
            repo: "mycli-repo".to_string(),
            version_flag: "-V".to_string(),
            asset_pattern: "mycli-x86_64-linux".to_string(),
        };

        save_apps_to(&path, &[app]).unwrap();
        let loaded = load_apps_from(&path).unwrap();

        assert_eq!(loaded[0].name, "mycli");
        assert_eq!(loaded[0].owner, "myorg");
        assert_eq!(loaded[0].repo, "mycli-repo");
        assert_eq!(loaded[0].version_flag, "-V");
        assert_eq!(loaded[0].asset_pattern, "mycli-x86_64-linux");
    }
}
