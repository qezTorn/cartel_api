use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error{
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Failed to parse JSON: {0}")]
    JsonParseError(#[from] serde_json::Error),
}

