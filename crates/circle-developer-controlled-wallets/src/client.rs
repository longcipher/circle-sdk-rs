//! HTTP client for the Developer-Controlled Wallets API.

use crate::{
    error::Error,
    models::{
        common::ApiErrorBody,
        signing::{
            SignMessageRequest, SignTransactionRequest, SignTransactionResponse,
            SignTypedDataRequest, SignatureResponse,
        },
        token::TokenResponse,
        transaction::{
            AccelerateTxRequest, CancelTxRequest, CreateContractExecutionTxRequest,
            CreateTransferTxRequest, EstimateFeeResponse, EstimateTransferFeeRequest,
            ListTransactionsParams, TransactionResponse, Transactions, ValidateAddressRequest,
            ValidateAddressResponse,
        },
        wallet::{
            Balances, CreateWalletsRequest, ListWalletBalancesParams, ListWalletNftsParams,
            ListWalletsParams, Nfts, UpdateWalletRequest, WalletNftsParams, WalletResponse,
            Wallets, WalletsWithBalances,
        },
        wallet_set::{
            CreateWalletSetRequest, ListWalletSetsParams, UpdateWalletSetRequest,
            WalletSetResponse, WalletSets,
        },
    },
};

/// Async HTTP client for the Circle W3S Developer-Controlled Wallets API.
pub struct DeveloperWalletsClient {
    base_url: String,
    api_key: String,
    http: hpx::Client,
}

impl std::fmt::Debug for DeveloperWalletsClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeveloperWalletsClient")
            .field("base_url", &self.base_url)
            .field("api_key", &"<redacted>")
            .finish_non_exhaustive()
    }
}

impl DeveloperWalletsClient {
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

    /// Dispatch a POST request with a JSON body and decode the JSON response.
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

    /// Dispatch a PUT request with a JSON body and decode the JSON response.
    async fn put<T, B>(&self, path: &str, body: &B) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned,
        B: serde::Serialize + ?Sized,
    {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .http
            .put(&url)
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

    // ── Wallet Sets ────────────────────────────────────────────────────────

    /// Create a new developer-controlled wallet set.
    pub async fn create_wallet_set(
        &self,
        req: &CreateWalletSetRequest,
    ) -> Result<WalletSetResponse, Error> {
        self.post("/v1/w3s/developer/walletSets", req).await
    }

    /// Get a wallet set by its UUID.
    pub async fn get_wallet_set(&self, id: &str) -> Result<WalletSetResponse, Error> {
        let path = format!("/v1/w3s/developer/walletSets/{}", id);
        self.get(&path, &[("", "")][..0]).await
    }

    /// Update the name of a wallet set.
    pub async fn update_wallet_set(
        &self,
        id: &str,
        req: &UpdateWalletSetRequest,
    ) -> Result<WalletSetResponse, Error> {
        let path = format!("/v1/w3s/developer/walletSets/{}", id);
        self.put(&path, req).await
    }

    /// List all wallet sets belonging to the entity.
    pub async fn list_wallet_sets(
        &self,
        params: &ListWalletSetsParams,
    ) -> Result<WalletSets, Error> {
        self.get("/v1/w3s/walletSets", params).await
    }

    // ── Wallets ────────────────────────────────────────────────────────────

    /// Create one or more developer-controlled wallets.
    pub async fn create_wallets(&self, req: &CreateWalletsRequest) -> Result<Wallets, Error> {
        self.post("/v1/w3s/developer/wallets", req).await
    }

    /// List wallets matching the given filters.
    pub async fn list_wallets(&self, params: &ListWalletsParams) -> Result<Wallets, Error> {
        self.get("/v1/w3s/wallets", params).await
    }

    /// Get a wallet by its UUID.
    pub async fn get_wallet(&self, id: &str) -> Result<WalletResponse, Error> {
        let path = format!("/v1/w3s/wallets/{}", id);
        self.get(&path, &[("", "")][..0]).await
    }

    /// Update the name or reference ID of a wallet.
    pub async fn update_wallet(
        &self,
        id: &str,
        req: &UpdateWalletRequest,
    ) -> Result<WalletResponse, Error> {
        let path = format!("/v1/w3s/wallets/{}", id);
        self.put(&path, req).await
    }

    /// List developer wallets with their token balances.
    pub async fn list_wallet_balances(
        &self,
        params: &ListWalletBalancesParams,
    ) -> Result<WalletsWithBalances, Error> {
        self.get("/v1/w3s/developer/wallets/balances", params).await
    }

    /// Retrieve token balances for a single wallet by its UUID.
    pub async fn list_wallet_token_balances(
        &self,
        wallet_id: &str,
        params: &WalletNftsParams,
    ) -> Result<Balances, Error> {
        let path = format!("/v1/w3s/wallets/{}/balances", wallet_id);
        self.get(&path, params).await
    }

    /// Retrieve NFTs held by a wallet by its UUID.
    pub async fn list_wallet_nfts(
        &self,
        wallet_id: &str,
        params: &ListWalletNftsParams,
    ) -> Result<Nfts, Error> {
        let path = format!("/v1/w3s/wallets/{}/nfts", wallet_id);
        self.get(&path, params).await
    }

    // ── Signing ────────────────────────────────────────────────────────────

    /// Sign a plain or hex-encoded message.
    pub async fn sign_message(&self, req: &SignMessageRequest) -> Result<SignatureResponse, Error> {
        self.post("/v1/w3s/developer/sign/message", req).await
    }

    /// Sign an EIP-712 typed data payload.
    pub async fn sign_typed_data(
        &self,
        req: &SignTypedDataRequest,
    ) -> Result<SignatureResponse, Error> {
        self.post("/v1/w3s/developer/sign/typedData", req).await
    }

    /// Sign a raw transaction.
    pub async fn sign_transaction(
        &self,
        req: &SignTransactionRequest,
    ) -> Result<SignTransactionResponse, Error> {
        self.post("/v1/w3s/developer/sign/transaction", req).await
    }

    // ── Transactions ───────────────────────────────────────────────────────

    /// List transactions matching the given filters.
    pub async fn list_transactions(
        &self,
        params: &ListTransactionsParams,
    ) -> Result<Transactions, Error> {
        self.get("/v1/w3s/transactions", params).await
    }

    /// Get a transaction by its UUID.
    pub async fn get_transaction(&self, id: &str) -> Result<TransactionResponse, Error> {
        let path = format!("/v1/w3s/transactions/{}", id);
        self.get(&path, &[("", "")][..0]).await
    }

    /// Create a developer-controlled transfer transaction.
    pub async fn create_transfer_transaction(
        &self,
        req: &CreateTransferTxRequest,
    ) -> Result<TransactionResponse, Error> {
        self.post("/v1/w3s/developer/transactions/transfer", req).await
    }

    /// Get fee parameters for a transfer.
    pub async fn get_fee_parameters(
        &self,
        req: &CreateTransferTxRequest,
    ) -> Result<EstimateFeeResponse, Error> {
        self.post("/v1/w3s/developer/transactions/feeParameters", req).await
    }

    /// Create a developer-controlled contract execution transaction.
    pub async fn create_contract_execution_transaction(
        &self,
        req: &CreateContractExecutionTxRequest,
    ) -> Result<TransactionResponse, Error> {
        self.post("/v1/w3s/developer/transactions/contractExecution", req).await
    }

    /// Cancel a stuck or queued transaction.
    pub async fn cancel_transaction(
        &self,
        id: &str,
        req: &CancelTxRequest,
    ) -> Result<TransactionResponse, Error> {
        let path = format!("/v1/w3s/developer/transactions/{}/cancel", id);
        self.post(&path, req).await
    }

    /// Accelerate a stuck transaction by resubmitting with higher fees.
    pub async fn accelerate_transaction(
        &self,
        id: &str,
        req: &AccelerateTxRequest,
    ) -> Result<TransactionResponse, Error> {
        let path = format!("/v1/w3s/developer/transactions/{}/accelerate", id);
        self.post(&path, req).await
    }

    // ── Tokens ─────────────────────────────────────────────────────────────

    /// Get a token by its UUID.
    pub async fn get_token(&self, id: &str) -> Result<TokenResponse, Error> {
        let path = format!("/v1/w3s/tokens/{}", id);
        self.get(&path, &[("", "")][..0]).await
    }

    // ── Utilities ──────────────────────────────────────────────────────────

    /// Estimate fees for a transfer transaction.
    pub async fn estimate_transfer_fee(
        &self,
        req: &EstimateTransferFeeRequest,
    ) -> Result<EstimateFeeResponse, Error> {
        self.post("/v1/w3s/transactions/transfer/estimateFee", req).await
    }

    /// Validate a blockchain address.
    pub async fn validate_address(
        &self,
        req: &ValidateAddressRequest,
    ) -> Result<ValidateAddressResponse, Error> {
        self.post("/v1/w3s/transactions/validateAddress", req).await
    }
}

#[cfg(test)]
mod tests {
    use crate::models::transaction::TransactionState;

    /// Verify we can construct the types without panic (static / non-network tests).
    #[test]
    fn transaction_state_serde_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
        let s = serde_json::to_string(&TransactionState::Complete)?;
        assert_eq!(s, "\"COMPLETE\"");
        Ok(())
    }

    #[test]
    fn cancel_tx_request_serializes() -> Result<(), Box<dyn std::error::Error>> {
        let req = crate::models::transaction::CancelTxRequest {
            entity_secret_ciphertext: "cipher".to_string(),
        };
        let json = serde_json::to_string(&req)?;
        assert!(json.contains("entitySecretCiphertext"));
        Ok(())
    }

    #[test]
    fn accelerate_tx_request_serializes() -> Result<(), Box<dyn std::error::Error>> {
        let req = crate::models::transaction::AccelerateTxRequest {
            entity_secret_ciphertext: "cipher".to_string(),
        };
        let json = serde_json::to_string(&req)?;
        assert!(json.contains("entitySecretCiphertext"));
        Ok(())
    }
}
