use thiserror::Error;

#[derive(Debug, Error)]
pub enum GrabError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid format: {0}")]
    InvalidInput(String),

    #[error("Error HTTP: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),

    #[error("Invalid regex input: {0}")]
    InvalidRegexInput(String),

    #[error("Command '{command}' failed with exit code {exit_code}: {stderr}")]
    CommandFailed {
        command: String,
        exit_code: i32,
        stderr: String,
    },

    #[error("Task join error: {0}")]
    TaskJoin(#[from] tokio::task::JoinError),

    #[error("Asset not found: {0}")]
    AssetNotFound(String),
}

pub type Result<T> = color_eyre::Result<T, GrabError>;
