//! HTTP client for the Compliance Engine API.

use crate::{
    error::Error,
    models::{
        common::ApiErrorBody,
        screening::{
            BlockchainAddressScreeningResponse, ScreenAddressEnvelope, ScreenAddressRequest,
        },
    },
};

/// Async HTTP client for the Circle W3S Compliance Engine API.
pub struct ComplianceClient {
    base_url: String,
    api_key: String,
    http: hpx::Client,
}

impl std::fmt::Debug for ComplianceClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComplianceClient")
            .field("base_url", &self.base_url)
            .field("api_key", &"<redacted>")
            .finish_non_exhaustive()
    }
}

impl ComplianceClient {
    /// Creates a new client using the Circle production base URL.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self::with_base_url(api_key, "https://api.circle.com")
    }

    /// Creates a new client with a custom base URL (useful for Prism mock servers).
    pub fn with_base_url(api_key: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self { base_url: base_url.into(), api_key: api_key.into(), http: hpx::Client::new() }
    }

    /// Send an authenticated POST request and decode the JSON response.
    async fn post<T, B>(&self, path: &str, body: &B) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned,
        B: serde::Serialize + ?Sized,
    {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .http
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("X-Request-Id", uuid::Uuid::new_v4().to_string())
            .json(body)
            .send()
            .await
            .map_err(|e| Error::Http(e.to_string()))?;

        if resp.status().is_success() {
            resp.json::<T>().await.map_err(|e| Error::Http(e.to_string()))
        } else {
            let err: ApiErrorBody = resp.json().await.map_err(|e| Error::Http(e.to_string()))?;
            Err(Error::Api { code: err.code, message: err.message })
        }
    }

    // ── Address Screening ─────────────────────────────────────────────────

    /// Screen a blockchain address for compliance risk.
    ///
    /// This is an idempotent operation: repeating the same `idempotency_key`
    /// returns the original response without re-running the screening.
    pub async fn screen_address(
        &self,
        req: &ScreenAddressRequest,
    ) -> Result<BlockchainAddressScreeningResponse, Error> {
        let envelope: ScreenAddressEnvelope =
            self.post("/v1/w3s/compliance/screening/addresses", req).await?;
        Ok(envelope.data)
    }
}
