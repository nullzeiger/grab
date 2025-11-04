use serde::ser::StdError;
use std::fmt;

#[derive(Debug)]
pub enum GrabError {
    Io(std::io::Error),
    Parse(String),
    NotFound(String),
    InvalidInput(String),
    Http(reqwest::Error),
    Regex(regex::Error),
    InvalidRegexInput(String),
    CommandFailed {
        command: String,
        exit_code: i32,
        stderr: String,
    },
    TaskJoin(tokio::task::JoinError),
    AssetNotFound(String),
}

impl fmt::Display for GrabError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GrabError::Io(err) => write!(f, "IO error: {}", err),
            GrabError::Parse(msg) => write!(f, "Parse error: {}", msg),
            GrabError::NotFound(msg) => write!(f, "Not found: {}", msg),
            GrabError::InvalidInput(msg) => write!(f, "Invalid format: {}", msg),
            GrabError::Http(err) => write!(f, "Error HTTP: {}", err),
            GrabError::Regex(err) => write!(f, "Regex error: {}", err),
            GrabError::InvalidRegexInput(msg) => write!(f, "Invalid regex input: {}", msg),
            GrabError::CommandFailed {
                command,
                exit_code,
                stderr,
            } => write!(
                f,
                "Command '{}' failed with exit code {}: {}",
                command, exit_code, stderr
            ),
            GrabError::TaskJoin(err) => write!(f, "Task join error: {}", err),
            GrabError::AssetNotFound(name) => write!(f, "Asset not found: {}", name),
        }
    }
}

impl StdError for GrabError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            GrabError::Io(err) => Some(err),
            GrabError::Parse(_) => self.source().as_ref().map(|e| &**e as _),
            GrabError::NotFound(_) => None,
            GrabError::InvalidInput(_) => None,
            GrabError::Http(err) => Some(err),
            GrabError::Regex(err) => Some(err),
            GrabError::InvalidRegexInput(_) => None,
            GrabError::CommandFailed { .. } => None,
            GrabError::TaskJoin(err) => Some(err),
            GrabError::AssetNotFound(_) => None,
        }
    }
}

impl From<std::io::Error> for GrabError {
    fn from(err: std::io::Error) -> Self {
        GrabError::Io(err)
    }
}

impl From<reqwest::Error> for GrabError {
    fn from(err: reqwest::Error) -> Self {
        GrabError::Http(err)
    }
}

impl From<regex::Error> for GrabError {
    fn from(err: regex::Error) -> Self {
        GrabError::Regex(err)
    }
}

impl From<tokio::task::JoinError> for GrabError {
    fn from(err: tokio::task::JoinError) -> Self {
        GrabError::TaskJoin(err)
    }
}

pub type Result<T> = std::result::Result<T, GrabError>;
