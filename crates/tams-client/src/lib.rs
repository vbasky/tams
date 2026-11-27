//! HTTP client for the TAMS API.

use reqwest::{header, Client as HttpClient, Method};
use tams_core::{
    Flow, FlowSegment, FlowSegmentCreate, Service, Source, StorageAllocation,
    StorageAllocationRequest, StorageBackend,
};
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

    /// Create a client with a bearer token for authentication.
    pub fn with_bearer_token(base_url: impl AsRef<str>, token: impl AsRef<str>) -> Result<Self> {
        let base_url = Url::parse(base_url.as_ref())
            .map_err(|e| Error::Other(format!("Invalid base URL: {e}")))?;
        let mut headers = header::HeaderMap::new();
        let value = header::HeaderValue::from_str(&format!("Bearer {}", token.as_ref()))
            .map_err(|e| Error::Other(format!("Invalid token: {e}")))?;
        headers.insert(header::AUTHORIZATION, value);
        let http = HttpClient::builder()
            .user_agent("tams/0.1.0")
            .default_headers(headers)
            .build()?;
        Ok(Self { base_url, http })
    }

    // --- Service ---

    /// Get service information.
    pub async fn get_service(&self) -> Result<Service> {
        self.get("service").await
    }

    /// List storage backends.
    pub async fn list_storage_backends(&self) -> Result<Vec<StorageBackend>> {
        self.get("service/storage-backends").await
    }

    // --- Sources ---

    /// Get a Source by ID.
    pub async fn get_source(&self, id: Uuid) -> Result<Source> {
        self.get(&format!("sources/{id}")).await
    }

    /// List Sources.
    pub async fn list_sources(&self) -> Result<Vec<Source>> {
        self.get("sources").await
    }

    /// Create or update a Source.
    pub async fn put_source(&self, source: &Source) -> Result<Source> {
        self.put(&format!("sources/{}", source.id), source).await
    }

    // --- Flows ---

    /// Get a Flow by ID.
    pub async fn get_flow(&self, id: Uuid) -> Result<Flow> {
        self.get(&format!("flows/{id}")).await
    }

    /// List Flows.
    pub async fn list_flows(&self) -> Result<Vec<Flow>> {
        self.get("flows").await
    }

    /// Create or update a Flow.
    pub async fn put_flow(&self, flow: &Flow) -> Result<Flow> {
        self.put(&format!("flows/{}", flow.id), flow).await
    }

    // --- Segments ---

    /// List Flow Segments for a Flow, optionally filtered by timerange.
    pub async fn list_segments(
        &self,
        flow_id: Uuid,
        timerange: Option<&str>,
    ) -> Result<Vec<FlowSegment>> {
        let mut url = self
            .base_url
            .join(&format!("flows/{flow_id}/segments"))
            .map_err(|e| Error::Other(format!("URL error: {e}")))?;
        if let Some(tr) = timerange {
            url.query_pairs_mut().append_pair("timerange", tr);
        }
        let resp = self.http.get(url).send().await?;
        self.handle_response(resp).await
    }

    /// Register one or more Flow Segments.
    pub async fn create_segments(
        &self,
        flow_id: Uuid,
        segments: &[FlowSegmentCreate],
    ) -> Result<Vec<FlowSegment>> {
        self.post(&format!("flows/{flow_id}/segments"), segments)
            .await
    }

    // --- Storage ---

    /// Allocate storage and get presigned PUT URLs for uploading media objects.
    pub async fn allocate_storage(
        &self,
        flow_id: Uuid,
        request: &StorageAllocationRequest,
    ) -> Result<StorageAllocation> {
        self.post(&format!("flows/{flow_id}/storage"), request)
            .await
    }

    // --- Internal ---

    async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T> {
        self.request(Method::GET, path, None::<&()>).await
    }

    async fn put<T: serde::Serialize + serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<T> {
        self.request(Method::PUT, path, Some(body)).await
    }

    async fn post<T: serde::Serialize, R: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<R> {
        self.request(Method::POST, path, Some(body)).await
    }

    async fn request<T: serde::Serialize, R: serde::de::DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: Option<&T>,
    ) -> Result<R> {
        let url = self
            .base_url
            .join(path)
            .map_err(|e| Error::Other(format!("URL error: {e}")))?;
        let mut req = self.http.request(method, url);
        if let Some(b) = body {
            req = req.json(b);
        }
        let resp = req.send().await?;
        self.handle_response(resp).await
    }

    async fn handle_response<T: serde::de::DeserializeOwned>(
        &self,
        resp: reqwest::Response,
    ) -> Result<T> {
        if !resp.status().is_success() {
            return Err(Error::Api {
                status: resp.status().as_u16(),
                message: resp.text().await.unwrap_or_default(),
            });
        }
        Ok(resp.json().await?)
    }
}
