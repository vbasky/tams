use thiserror::Error;

/// Errors that can occur when interacting with a TAMS store.
#[derive(Debug, Error)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("API error: {status} - {message}")]
    Api { status: u16, message: String },

    #[error("Invalid timerange: {0}")]
    InvalidTimerange(String),

    #[error("{0}")]
    Other(String),
}
