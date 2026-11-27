//! Complete HTTP client for the TAMS API (v8.2).
//!
//! Covers all operations defined in the official OpenAPI specification.

use reqwest::{header, Client as HttpClient, Method, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use tams_core::*;
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

    /// Create a client with a bearer token.
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

    // ==================== Root & Service ====================

    /// GET / — list root endpoints.
    pub async fn get_root(&self) -> Result<Vec<String>> {
        self.get("").await
    }

    /// GET /service
    pub async fn get_service(&self) -> Result<Service> {
        self.get("service").await
    }

    /// POST /service — update service info.
    pub async fn update_service(&self, update: &serde_json::Value) -> Result<()> {
        self.request_empty(Method::POST, "service", Some(update)).await
    }

    /// GET /service/storage-backends
    pub async fn list_storage_backends(&self) -> Result<Vec<StorageBackend>> {
        self.get("service/storage-backends").await
    }

    /// GET /service/profiles
    pub async fn list_profiles(&self) -> Result<Vec<Profile>> {
        self.get("service/profiles").await
    }

    /// GET /service/profiles/{id}
    pub async fn get_profile(&self, id: Uuid) -> Result<Profile> {
        self.get(&format!("service/profiles/{id}")).await
    }

    /// POST /service/profiles/{id} — create a profile.
    pub async fn create_profile(&self, id: Uuid, profile: &Profile) -> Result<Profile> {
        self.post(&format!("service/profiles/{id}"), profile).await
    }

    // ==================== Webhooks ====================

    /// GET /service/webhooks
    pub async fn list_webhooks(&self) -> Result<Vec<Webhook>> {
        self.get("service/webhooks").await
    }

    /// POST /service/webhooks — register a webhook.
    pub async fn create_webhook(&self, webhook: &WebhookCreate) -> Result<Webhook> {
        self.post("service/webhooks", webhook).await
    }

    /// GET /service/webhooks/{id}
    pub async fn get_webhook(&self, id: Uuid) -> Result<Webhook> {
        self.get(&format!("service/webhooks/{id}")).await
    }

    /// PUT /service/webhooks/{id}
    pub async fn update_webhook(&self, id: Uuid, webhook: &WebhookUpdate) -> Result<Webhook> {
        self.put(&format!("service/webhooks/{id}"), webhook).await
    }

    /// DELETE /service/webhooks/{id}
    pub async fn delete_webhook(&self, id: Uuid) -> Result<()> {
        self.delete(&format!("service/webhooks/{id}")).await
    }

    // ==================== Sources ====================

    /// GET /sources
    pub async fn list_sources(&self) -> Result<Vec<Source>> {
        self.get("sources").await
    }

    /// GET /sources/{id}
    pub async fn get_source(&self, id: Uuid) -> Result<Source> {
        self.get(&format!("sources/{id}")).await
    }

    /// PUT /sources/{id}/label
    pub async fn set_source_label(&self, id: Uuid, label: &str) -> Result<()> {
        self.put_empty(&format!("sources/{id}/label"), &serde_json::json!(label))
            .await
    }

    /// DELETE /sources/{id}/label
    pub async fn delete_source_label(&self, id: Uuid) -> Result<()> {
        self.delete(&format!("sources/{id}/label")).await
    }

    /// GET /sources/{id}/label
    pub async fn get_source_label(&self, id: Uuid) -> Result<String> {
        self.get(&format!("sources/{id}/label")).await
    }

    /// PUT /sources/{id}/description
    pub async fn set_source_description(&self, id: Uuid, description: &str) -> Result<()> {
        self.put_empty(
            &format!("sources/{id}/description"),
            &serde_json::json!(description),
        )
        .await
    }

    /// DELETE /sources/{id}/description
    pub async fn delete_source_description(&self, id: Uuid) -> Result<()> {
        self.delete(&format!("sources/{id}/description")).await
    }

    /// GET /sources/{id}/description
    pub async fn get_source_description(&self, id: Uuid) -> Result<String> {
        self.get(&format!("sources/{id}/description")).await
    }

    /// GET /sources/{id}/tags
    pub async fn get_source_tags(&self, id: Uuid) -> Result<serde_json::Map<String, serde_json::Value>> {
        self.get(&format!("sources/{id}/tags")).await
    }

    /// GET /sources/{id}/tags/{name}
    pub async fn get_source_tag(&self, id: Uuid, name: &str) -> Result<serde_json::Value> {
        self.get(&format!("sources/{id}/tags/{name}")).await
    }

    /// PUT /sources/{id}/tags/{name}
    pub async fn set_source_tag(&self, id: Uuid, name: &str, value: &serde_json::Value) -> Result<()> {
        self.put_empty(&format!("sources/{id}/tags/{name}"), value)
            .await
    }

    /// DELETE /sources/{id}/tags/{name}
    pub async fn delete_source_tag(&self, id: Uuid, name: &str) -> Result<()> {
        self.delete(&format!("sources/{id}/tags/{name}")).await
    }

    // ==================== Flows ====================

    /// GET /flows
    pub async fn list_flows(&self) -> Result<Vec<Flow>> {
        self.get("flows").await
    }

    /// GET /flows/{id}
    pub async fn get_flow(&self, id: Uuid) -> Result<Flow> {
        self.get(&format!("flows/{id}")).await
    }

    /// PUT /flows/{id}
    pub async fn put_flow(&self, flow: &Flow) -> Result<Flow> {
        self.put(&format!("flows/{}", flow.id), flow).await
    }

    /// DELETE /flows/{id}
    pub async fn delete_flow(&self, id: Uuid) -> Result<()> {
        self.delete(&format!("flows/{id}")).await
    }

    /// PUT /flows/{id}/label
    pub async fn set_flow_label(&self, id: Uuid, label: &str) -> Result<()> {
        self.put_empty(&format!("flows/{id}/label"), &serde_json::json!(label))
            .await
    }

    /// DELETE /flows/{id}/label
    pub async fn delete_flow_label(&self, id: Uuid) -> Result<()> {
        self.delete(&format!("flows/{id}/label")).await
    }

    /// GET /flows/{id}/label
    pub async fn get_flow_label(&self, id: Uuid) -> Result<String> {
        self.get(&format!("flows/{id}/label")).await
    }

    /// PUT /flows/{id}/description
    pub async fn set_flow_description(&self, id: Uuid, description: &str) -> Result<()> {
        self.put_empty(
            &format!("flows/{id}/description"),
            &serde_json::json!(description),
        )
        .await
    }

    /// DELETE /flows/{id}/description
    pub async fn delete_flow_description(&self, id: Uuid) -> Result<()> {
        self.delete(&format!("flows/{id}/description")).await
    }

    /// GET /flows/{id}/description
    pub async fn get_flow_description(&self, id: Uuid) -> Result<String> {
        self.get(&format!("flows/{id}/description")).await
    }

    /// GET /flows/{id}/tags
    pub async fn get_flow_tags(&self, id: Uuid) -> Result<serde_json::Map<String, serde_json::Value>> {
        self.get(&format!("flows/{id}/tags")).await
    }

    /// GET /flows/{id}/tags/{name}
    pub async fn get_flow_tag(&self, id: Uuid, name: &str) -> Result<serde_json::Value> {
        self.get(&format!("flows/{id}/tags/{name}")).await
    }

    /// PUT /flows/{id}/tags/{name}
    pub async fn set_flow_tag(&self, id: Uuid, name: &str, value: &serde_json::Value) -> Result<()> {
        self.put_empty(&format!("flows/{id}/tags/{name}"), value)
            .await
    }

    /// DELETE /flows/{id}/tags/{name}
    pub async fn delete_flow_tag(&self, id: Uuid, name: &str) -> Result<()> {
        self.delete(&format!("flows/{id}/tags/{name}")).await
    }

    /// PUT /flows/{id}/status
    pub async fn set_flow_status(&self, id: Uuid, status: &str) -> Result<()> {
        self.put_empty(&format!("flows/{id}/status"), &serde_json::json!(status))
            .await
    }

    /// DELETE /flows/{id}/status
    pub async fn delete_flow_status(&self, id: Uuid) -> Result<()> {
        self.delete(&format!("flows/{id}/status")).await
    }

    /// GET /flows/{id}/status
    pub async fn get_flow_status(&self, id: Uuid) -> Result<String> {
        self.get(&format!("flows/{id}/status")).await
    }

    /// PUT /flows/{id}/read-only
    pub async fn set_flow_read_only(&self, id: Uuid, read_only: bool) -> Result<()> {
        self.put_empty(
            &format!("flows/{id}/read-only"),
            &serde_json::json!(read_only),
        )
        .await
    }

    /// GET /flows/{id}/read-only
    pub async fn get_flow_read_only(&self, id: Uuid) -> Result<bool> {
        self.get(&format!("flows/{id}/read-only")).await
    }

    /// PUT /flows/{id}/avg-bit-rate
    pub async fn set_flow_avg_bit_rate(&self, id: Uuid, rate: u64) -> Result<()> {
        self.put_empty(
            &format!("flows/{id}/avg-bit-rate"),
            &serde_json::json!(rate),
        )
        .await
    }

    /// DELETE /flows/{id}/avg-bit-rate
    pub async fn delete_flow_avg_bit_rate(&self, id: Uuid) -> Result<()> {
        self.delete(&format!("flows/{id}/avg-bit-rate")).await
    }

    /// GET /flows/{id}/avg-bit-rate
    pub async fn get_flow_avg_bit_rate(&self, id: Uuid) -> Result<u64> {
        self.get(&format!("flows/{id}/avg-bit-rate")).await
    }

    /// PUT /flows/{id}/max-bit-rate
    pub async fn set_flow_max_bit_rate(&self, id: Uuid, rate: u64) -> Result<()> {
        self.put_empty(
            &format!("flows/{id}/max-bit-rate"),
            &serde_json::json!(rate),
        )
        .await
    }

    /// DELETE /flows/{id}/max-bit-rate
    pub async fn delete_flow_max_bit_rate(&self, id: Uuid) -> Result<()> {
        self.delete(&format!("flows/{id}/max-bit-rate")).await
    }

    /// GET /flows/{id}/max-bit-rate
    pub async fn get_flow_max_bit_rate(&self, id: Uuid) -> Result<u64> {
        self.get(&format!("flows/{id}/max-bit-rate")).await
    }

    /// PUT /flows/{id}/flow-collection
    pub async fn set_flow_collection(&self, id: Uuid, collection: &serde_json::Value) -> Result<()> {
        self.put_empty(&format!("flows/{id}/flow-collection"), collection)
            .await
    }

    /// DELETE /flows/{id}/flow-collection
    pub async fn delete_flow_collection(&self, id: Uuid) -> Result<()> {
        self.delete(&format!("flows/{id}/flow-collection")).await
    }

    /// GET /flows/{id}/flow-collection
    pub async fn get_flow_collection(&self, id: Uuid) -> Result<serde_json::Value> {
        self.get(&format!("flows/{id}/flow-collection")).await
    }

    // ==================== Segments ====================

    /// GET /flows/{id}/segments
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

    /// POST /flows/{id}/segments — register segments.
    pub async fn create_segments(
        &self,
        flow_id: Uuid,
        segments: &[FlowSegmentCreate],
    ) -> Result<Vec<FlowSegment>> {
        self.post(&format!("flows/{flow_id}/segments"), &segments)
            .await
    }

    /// DELETE /flows/{id}/segments
    pub async fn delete_segments(&self, flow_id: Uuid, timerange: Option<&str>) -> Result<()> {
        let mut url = self
            .base_url
            .join(&format!("flows/{flow_id}/segments"))
            .map_err(|e| Error::Other(format!("URL error: {e}")))?;
        if let Some(tr) = timerange {
            url.query_pairs_mut().append_pair("timerange", tr);
        }
        let resp = self.http.delete(url).send().await?;
        if !resp.status().is_success() && resp.status() != StatusCode::NO_CONTENT {
            return Err(Error::Api {
                status: resp.status().as_u16(),
                message: resp.text().await.unwrap_or_default(),
            });
        }
        Ok(())
    }

    // ==================== Storage ====================

    /// POST /flows/{id}/storage — allocate storage.
    pub async fn allocate_storage(
        &self,
        flow_id: Uuid,
        request: &StorageAllocationRequest,
    ) -> Result<StorageAllocation> {
        self.post(&format!("flows/{flow_id}/storage"), request)
            .await
    }

    // ==================== Objects ====================

    /// GET /objects
    pub async fn list_objects(&self) -> Result<Vec<Object>> {
        self.get("objects").await
    }

    /// POST /objects/instances
    pub async fn create_object_instances(&self, instances: &serde_json::Value) -> Result<serde_json::Value> {
        self.post("objects/instances", instances).await
    }

    /// DELETE /objects/instances
    pub async fn delete_object_instances(&self) -> Result<()> {
        self.delete("objects/instances").await
    }

    // ==================== Flow Delete Requests ====================

    /// GET /flow-delete-requests
    pub async fn list_flow_delete_requests(&self) -> Result<Vec<FlowDeleteRequest>> {
        self.get("flow-delete-requests").await
    }

    /// GET /flow-delete-requests/{id}
    pub async fn get_flow_delete_request(&self, id: Uuid) -> Result<FlowDeleteRequest> {
        self.get(&format!("flow-delete-requests/{id}")).await
    }

    // ==================== Internal helpers ====================

    async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        self.request(Method::GET, path, None::<&()>).await
    }

    async fn put<B: Serialize, R: DeserializeOwned>(&self, path: &str, body: &B) -> Result<R> {
        self.request(Method::PUT, path, Some(body)).await
    }

    async fn put_empty<B: Serialize>(&self, path: &str, body: &B) -> Result<()> {
        self.request_empty(Method::PUT, path, Some(body)).await
    }

    async fn post<B: Serialize, R: DeserializeOwned>(&self, path: &str, body: &B) -> Result<R> {
        self.request(Method::POST, path, Some(body)).await
    }

    async fn delete(&self, path: &str) -> Result<()> {
        self.request_empty(Method::DELETE, path, None::<&()>).await
    }

    async fn request<B: Serialize, R: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<R> {
        let url = self.join(path)?;
        let mut req = self.http.request(method, url);
        if let Some(b) = body {
            req = req.json(b);
        }
        let resp = req.send().await?;
        self.handle_response(resp).await
    }

    async fn request_empty<B: Serialize>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<()> {
        let url = self.join(path)?;
        let mut req = self.http.request(method, url);
        if let Some(b) = body {
            req = req.json(b);
        }
        let resp = req.send().await?;
        if !resp.status().is_success() && resp.status() != StatusCode::NO_CONTENT {
            return Err(Error::Api {
                status: resp.status().as_u16(),
                message: resp.text().await.unwrap_or_default(),
            });
        }
        Ok(())
    }

    async fn handle_response<T: DeserializeOwned>(&self, resp: reqwest::Response) -> Result<T> {
        if !resp.status().is_success() {
            return Err(Error::Api {
                status: resp.status().as_u16(),
                message: resp.text().await.unwrap_or_default(),
            });
        }
        Ok(resp.json().await?)
    }

    fn join(&self, path: &str) -> Result<Url> {
        self.base_url
            .join(path)
            .map_err(|e| Error::Other(format!("URL error: {e}")))
    }
}
