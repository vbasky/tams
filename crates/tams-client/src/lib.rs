//! HTTP client for the TAMS API.

use reqwest::Client as HttpClient;
use tams_core::{Flow, Source};
use thiserror::Error;
use url::Url;
use uuid::Uuid;

/// Errors that can occur when interacting with a TAMS store.
#[derive(Debug, Error)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("API error: {status} - {message}")]
    Api { status: u16, message: String },

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, Error>;

/// A client for interacting with a TAMS store.
#[derive(Debug, Clone)]
pub struct Client {
    base_url: Url,
    http: HttpClient,
}

impl Client {
    /// Create a new TAMS client.
    pub fn new(base_url: impl AsRef<str>) -> Result<Self> {
        let base_url = Url::parse(base_url.as_ref())
            .map_err(|e| Error::Other(format!("Invalid base URL: {e}")))?;
        let http = HttpClient::builder()
            .user_agent("tams/0.1.0")
            .build()?;
        Ok(Self { base_url, http })
    }

    /// Get a Source by ID.
    pub async fn get_source(&self, id: Uuid) -> Result<Source> {
        let url = self
            .base_url
            .join(&format!("sources/{id}"))
            .map_err(|e| Error::Other(format!("URL error: {e}")))?;
        let resp = self.http.get(url).send().await?;
        if !resp.status().is_success() {
            return Err(Error::Api {
                status: resp.status().as_u16(),
                message: resp.text().await.unwrap_or_default(),
            });
        }
        Ok(resp.json().await?)
    }

    /// Get a Flow by ID.
    pub async fn get_flow(&self, id: Uuid) -> Result<Flow> {
        let url = self
            .base_url
            .join(&format!("flows/{id}"))
            .map_err(|e| Error::Other(format!("URL error: {e}")))?;
        let resp = self.http.get(url).send().await?;
        if !resp.status().is_success() {
            return Err(Error::Api {
                status: resp.status().as_u16(),
                message: resp.text().await.unwrap_or_default(),
            });
        }
        Ok(resp.json().await?)
    }
}
