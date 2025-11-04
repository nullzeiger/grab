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
