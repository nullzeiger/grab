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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identical_versions_are_latest() {
        let compare = Compare::new("v1.2.3", "1.2.3").unwrap();
        assert!(compare.is_latest);
    }

    #[test]
    fn test_different_versions_are_not_latest() {
        let compare = Compare::new("v1.3.0", "1.2.3").unwrap();
        assert!(!compare.is_latest);
    }

    #[test]
    fn test_release_with_prefix_and_suffix() {
        let compare = Compare::new("release-v2.0.1-stable", "myapp 2.0.1 (build 42)").unwrap();
        assert!(compare.is_latest);
    }

    #[test]
    fn test_older_local_version() {
        let compare = Compare::new("v2.0.0", "v1.9.9").unwrap();
        assert!(!compare.is_latest);
    }

    #[test]
    fn test_newer_local_version_is_not_latest() {
        // is_latest is a string equality check, not a semver comparison
        let compare = Compare::new("v1.0.0", "v2.0.0").unwrap();
        assert!(!compare.is_latest);
    }

    #[test]
    fn test_version_extracted_from_multiline_output() {
        let compare = Compare::new("v0.5.10", "mycli 0.5.10\nbuilt on linux").unwrap();
        assert!(compare.is_latest);
    }

    #[test]
    fn test_invalid_release_returns_error() {
        let result = Compare::new("no-version-here", "1.2.3");
        assert!(matches!(
            result,
            Err(GrabError::InvalidRegexInput(msg)) if msg.contains("release")
        ));
    }

    #[test]
    fn test_invalid_local_version_returns_error() {
        let result = Compare::new("v1.2.3", "no-version-here");
        assert!(matches!(
            result,
            Err(GrabError::InvalidRegexInput(msg)) if msg.contains("current")
        ));
    }

    #[test]
    fn test_both_invalid_returns_release_error_first() {
        let result = Compare::new("invalid", "also-invalid");
        assert!(matches!(
            result,
            Err(GrabError::InvalidRegexInput(msg)) if msg.contains("release")
        ));
    }

    #[test]
    fn test_empty_strings_return_error() {
        assert!(Compare::new("", "").is_err());
    }

    #[test]
    fn test_partial_version_not_matched() {
        // "1.2" alone has no third segment — should error
        let result = Compare::new("v1.2", "1.2.3");
        assert!(matches!(result, Err(GrabError::InvalidRegexInput(_))));
    }

    #[test]
    fn test_only_first_semver_match_is_used() {
        // release has two semver strings — regex finds the first one (1.0.0)
        let compare = Compare::new("v1.0.0 or v2.0.0", "1.0.0").unwrap();
        assert!(compare.is_latest);
    }
}
