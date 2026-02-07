use crate::error::{GrabError, Result};
use once_cell::sync::Lazy;
use regex::Regex;

pub struct Compare {
    pub is_latest: bool,
}

static VERSION_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\d+\.\d+\.\d+").expect("Invalid regex check the version pattern syntax.")
});

impl Compare {
    pub fn new(release: &str, version: &str) -> Result<Self> {
        let release_match = VERSION_RE
            .find(release)
            .ok_or_else(|| GrabError::InvalidRegexInput("Invalid release version".to_string()))?;

        let version_match = VERSION_RE
            .find(version)
            .ok_or_else(|| GrabError::InvalidRegexInput("Invalid current version".to_string()))?;

        let is_latest = release_match.as_str() == version_match.as_str();

        Ok(Self { is_latest })
    }
}
