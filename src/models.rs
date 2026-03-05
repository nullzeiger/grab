use crate::error::{GrabError, Result};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App {
    pub name: String,
    pub owner: String,
    pub repo: String,
    pub asset_pattern: String,
    pub version_flag: String,
}

impl App {
    pub fn new(
        name: impl Into<String>,
        owner: impl Into<String>,
        repo: impl Into<String>,
        asset_pattern: impl Into<String>,
        version_flag: impl Into<String>,
    ) -> Result<Self> {
        let name = Self::validate_non_empty(name.into(), "Name")?;
        let owner = Self::validate_non_empty(owner.into(), "Owner")?;
        let repo = Self::validate_non_empty(repo.into(), "Repo")?;
        let asset_pattern = Self::validate_non_empty(asset_pattern.into(), "Asset pattern")?;
        let version_flag = Self::validate_non_empty(version_flag.into(), "Version flag")?;

        Ok(Self {
            name,
            owner,
            repo,
            asset_pattern,
            version_flag,
        })
    }

    fn validate_non_empty(field: String, field_name: &str) -> Result<String> {
        let trimmed = field.trim();
        if trimmed.is_empty() {
            Err(GrabError::InvalidInput(format!(
                "{field_name} cannot be empty"
            )))
        } else {
            Ok(trimmed.to_string())
        }
    }
}

impl fmt::Display for App {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Name: {}, Owner: {}, Repo: {}, Asset Pattern: {}, Version flag: {}",
            self.name, self.owner, self.repo, self.asset_pattern, self.version_flag
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_valid_app() -> App {
        App::new(
            "ripgrep",
            "BurntSushi",
            "ripgrep",
            "ripgrep-linux",
            "--version",
        )
        .unwrap()
    }

    #[test]
    fn test_valid_app_creates_successfully() {
        let app = make_valid_app();
        assert_eq!(app.name, "ripgrep");
        assert_eq!(app.owner, "BurntSushi");
        assert_eq!(app.repo, "ripgrep");
        assert_eq!(app.asset_pattern, "ripgrep-linux");
        assert_eq!(app.version_flag, "--version");
    }

    #[test]
    fn test_whitespace_only_name_is_invalid() {
        let result = App::new("   ", "owner", "repo", "pattern", "--version");
        assert!(matches!(result, Err(GrabError::InvalidInput(msg)) if msg.contains("Name")));
    }

    #[test]
    fn test_empty_name_is_invalid() {
        let result = App::new("", "owner", "repo", "pattern", "--version");
        assert!(matches!(result, Err(GrabError::InvalidInput(msg)) if msg.contains("Name")));
    }

    #[test]
    fn test_whitespace_only_owner_is_invalid() {
        let result = App::new("name", "   ", "repo", "pattern", "--version");
        assert!(matches!(result, Err(GrabError::InvalidInput(msg)) if msg.contains("Owner")));
    }

    #[test]
    fn test_empty_owner_is_invalid() {
        let result = App::new("name", "", "repo", "pattern", "--version");
        assert!(matches!(result, Err(GrabError::InvalidInput(msg)) if msg.contains("Owner")));
    }

    #[test]
    fn test_whitespace_only_repo_is_invalid() {
        let result = App::new("name", "owner", "\t", "pattern", "--version");
        assert!(matches!(result, Err(GrabError::InvalidInput(msg)) if msg.contains("Repo")));
    }

    #[test]
    fn test_empty_repo_is_invalid() {
        let result = App::new("name", "owner", "", "pattern", "--version");
        assert!(matches!(result, Err(GrabError::InvalidInput(msg)) if msg.contains("Repo")));
    }

    #[test]
    fn test_whitespace_only_asset_pattern_is_invalid() {
        let result = App::new("name", "owner", "repo", "  ", "--version");
        assert!(
            matches!(result, Err(GrabError::InvalidInput(msg)) if msg.contains("Asset pattern"))
        );
    }

    #[test]
    fn test_empty_asset_pattern_is_invalid() {
        let result = App::new("name", "owner", "repo", "", "--version");
        assert!(
            matches!(result, Err(GrabError::InvalidInput(msg)) if msg.contains("Asset pattern"))
        );
    }

    #[test]
    fn test_whitespace_only_version_flag_is_invalid() {
        let result = App::new("name", "owner", "repo", "pattern", "   ");
        assert!(
            matches!(result, Err(GrabError::InvalidInput(msg)) if msg.contains("Version flag"))
        );
    }

    #[test]
    fn test_empty_version_flag_is_invalid() {
        let result = App::new("name", "owner", "repo", "pattern", "");
        assert!(
            matches!(result, Err(GrabError::InvalidInput(msg)) if msg.contains("Version flag"))
        );
    }

    #[test]
    fn test_fields_are_trimmed_on_creation() {
        let app = App::new(
            "  ripgrep  ",
            "  BurntSushi  ",
            "  ripgrep  ",
            "  linux  ",
            "  --version  ",
        )
        .unwrap();
        assert_eq!(app.name, "ripgrep");
        assert_eq!(app.owner, "BurntSushi");
        assert_eq!(app.repo, "ripgrep");
        assert_eq!(app.asset_pattern, "linux");
        assert_eq!(app.version_flag, "--version");
    }
}
