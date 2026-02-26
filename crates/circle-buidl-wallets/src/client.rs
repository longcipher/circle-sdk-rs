//! HTTP client for the Buidl Wallets API.

use crate::{
    error::Error,
    models::{
        common::ApiErrorBody,
        transfer::{ListTransfersParams, TransferId, Transfers},
        user_op::{ListUserOpsParams, UserOpId, UserOps},
        wallet::{Balances, ListWalletBalancesParams, ListWalletNftsParams, Nfts},
    },
};

/// Async HTTP client for the Circle W3S Buidl Wallets API.
pub struct BuidlWalletsClient {
    base_url: String,
    api_key: String,
    http: hpx::Client,
}

impl std::fmt::Debug for BuidlWalletsClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BuidlWalletsClient")
            .field("base_url", &self.base_url)
            .field("api_key", &"<redacted>")
            .finish_non_exhaustive()
    }
}

impl BuidlWalletsClient {
    /// Creates a new client using the Circle production base URL.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self::with_base_url(api_key, "https://api.circle.com")
    }

    /// Creates a new client with a custom base URL (useful for Prism mock servers).
    pub fn with_base_url(api_key: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self { base_url: base_url.into(), api_key: api_key.into(), http: hpx::Client::new() }
    }

    /// Dispatch a GET request and decode the JSON response.
    async fn get<T, P>(&self, path: &str, params: &P) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned,
        P: serde::Serialize + ?Sized,
    {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .http
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("X-Request-Id", uuid::Uuid::new_v4().to_string())
            .query(params)
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

    // ── Transfers ──────────────────────────────────────────────────────────

    /// List transfers matching the given filters.
    ///
    /// `params.wallet_addresses` is required by the Circle API.
    pub async fn list_transfers(&self, params: &ListTransfersParams) -> Result<Transfers, Error> {
        self.get("/v1/w3s/buidl/transfers", params).await
    }

    /// Retrieve a single transfer by its UUID.
    pub async fn get_transfer(&self, id: &str) -> Result<TransferId, Error> {
        let path = format!("/v1/w3s/buidl/transfers/{}", id);
        self.get(&path, &[("", "")][..0]).await
    }

    // ── UserOps ────────────────────────────────────────────────────────────

    /// List user operations matching the given filters.
    pub async fn list_user_ops(&self, params: &ListUserOpsParams) -> Result<UserOps, Error> {
        self.get("/v1/w3s/buidl/userOps", params).await
    }

    /// Retrieve a single user operation by its UUID.
    pub async fn get_user_op(&self, id: &str) -> Result<UserOpId, Error> {
        let path = format!("/v1/w3s/buidl/userOps/{}", id);
        self.get(&path, &[("", "")][..0]).await
    }

    // ── Wallets ────────────────────────────────────────────────────────────

    /// Retrieve token balances for a wallet by its UUID.
    pub async fn list_wallet_balances_by_id(
        &self,
        wallet_id: &str,
        params: &ListWalletBalancesParams,
    ) -> Result<Balances, Error> {
        let path = format!("/v1/w3s/buidl/wallets/{}/balances", wallet_id);
        self.get(&path, params).await
    }

    /// Retrieve NFTs held by a wallet by its UUID.
    pub async fn list_wallet_nfts_by_id(
        &self,
        wallet_id: &str,
        params: &ListWalletNftsParams,
    ) -> Result<Nfts, Error> {
        let path = format!("/v1/w3s/buidl/wallets/{}/nfts", wallet_id);
        self.get(&path, params).await
    }

    /// Retrieve token balances for a wallet by blockchain + address.
    pub async fn list_wallet_balances_by_address(
        &self,
        blockchain: &str,
        address: &str,
        params: &ListWalletBalancesParams,
    ) -> Result<Balances, Error> {
        let path = format!("/v1/w3s/buidl/wallets/{}/{}/balances", blockchain, address);
        self.get(&path, params).await
    }

    /// Retrieve NFTs for a wallet by blockchain + address.
    pub async fn list_wallet_nfts_by_address(
        &self,
        blockchain: &str,
        address: &str,
        params: &ListWalletNftsParams,
    ) -> Result<Nfts, Error> {
        let path = format!("/v1/w3s/buidl/wallets/{}/{}/nfts", blockchain, address);
        self.get(&path, params).await
    }
}
