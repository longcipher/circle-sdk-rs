//! HTTP client for the User-Controlled Wallets API.

use crate::{
    error::Error,
    models::{
        auth::{
            DeviceTokenEmailRequest, DeviceTokenEmailResponse, DeviceTokenSocialRequest,
            DeviceTokenSocialResponse, RefreshUserTokenRequest, RefreshUserTokenResponse,
            ResendOtpRequest, ResendOtpResponse,
        },
        challenge::{
            ChallengeIdResponse, ChallengeResponse, Challenges, SetPinAndInitWalletRequest,
            SetPinRequest,
        },
        common::ApiErrorBody,
        signing::{SignMessageRequest, SignTransactionRequest, SignTypedDataRequest},
        transaction::{
            AccelerateTxRequest, CancelTxRequest, CreateContractExecutionTxRequest,
            CreateTransferTxRequest, CreateWalletUpgradeTxRequest, EstimateContractExecFeeRequest,
            EstimateTransactionFee, EstimateTransferFeeRequest, GetLowestNonceTransactionResponse,
            GetLowestNonceTxParams, ListTransactionsParams, TransactionResponse, Transactions,
            ValidateAddressRequest, ValidateAddressResponse,
        },
        user::{
            CreateUserRequest, GetUserByIdResponse, GetUserTokenRequest, ListUsersParams,
            UserResponse, UserTokenResponse, Users,
        },
        wallet::{
            Balances, CreateEndUserWalletRequest, ListWalletBalancesParams, ListWalletNftsParams,
            ListWalletsParams, Nfts, TokenResponse, UpdateWalletRequest, WalletResponse, Wallets,
        },
    },
};

/// Async HTTP client for the Circle W3S User-Controlled Wallets API.
pub struct UserWalletsClient {
    /// Base URL for the Circle API (defaults to `https://api.circle.com`).
    base_url: String,
    /// API key used in `Authorization: Bearer` header.
    api_key: String,
    /// Underlying HTTP client.
    http: hpx::Client,
}

impl std::fmt::Debug for UserWalletsClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UserWalletsClient")
            .field("base_url", &self.base_url)
            .field("api_key", &"<redacted>")
            .finish_non_exhaustive()
    }
}

impl UserWalletsClient {
    /// Creates a new client using the Circle production base URL.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self::with_base_url(api_key, "https://api.circle.com")
    }

    /// Creates a new client with a custom base URL (useful for Prism mock servers).
    pub fn with_base_url(api_key: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self { base_url: base_url.into(), api_key: api_key.into(), http: hpx::Client::new() }
    }

    // ── Private HTTP helpers ──────────────────────────────────────────────

    /// Authenticated GET request, no user token.
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

        Self::decode(resp).await
    }

    /// Authenticated POST request, no user token.
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

        Self::decode(resp).await
    }

    /// Authenticated PUT request, no user token.
    #[expect(dead_code)]
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

        Self::decode(resp).await
    }

    /// Authenticated GET request with an additional `X-User-Token` header.
    async fn get_with_user_token<T, P>(
        &self,
        path: &str,
        params: &P,
        user_token: &str,
    ) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned,
        P: serde::Serialize + ?Sized,
    {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .http
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("X-User-Token", user_token)
            .header("X-Request-Id", uuid::Uuid::new_v4().to_string())
            .query(params)
            .send()
            .await
            .map_err(|e| Error::Http(e.to_string()))?;

        Self::decode(resp).await
    }

    /// Authenticated POST request with an additional `X-User-Token` header.
    async fn post_with_user_token<T, B>(
        &self,
        path: &str,
        body: &B,
        user_token: &str,
    ) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned,
        B: serde::Serialize + ?Sized,
    {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .http
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("X-User-Token", user_token)
            .header("X-Request-Id", uuid::Uuid::new_v4().to_string())
            .json(body)
            .send()
            .await
            .map_err(|e| Error::Http(e.to_string()))?;

        Self::decode(resp).await
    }

    /// Authenticated PUT request with an additional `X-User-Token` header.
    async fn put_with_user_token<T, B>(
        &self,
        path: &str,
        body: &B,
        user_token: &str,
    ) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned,
        B: serde::Serialize + ?Sized,
    {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .http
            .put(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("X-User-Token", user_token)
            .header("X-Request-Id", uuid::Uuid::new_v4().to_string())
            .json(body)
            .send()
            .await
            .map_err(|e| Error::Http(e.to_string()))?;

        Self::decode(resp).await
    }

    /// Decode a response: if 2xx parse as `T`, otherwise parse as [`ApiErrorBody`].
    async fn decode<T: serde::de::DeserializeOwned>(resp: hpx::Response) -> Result<T, Error> {
        if resp.status().is_success() {
            resp.json::<T>().await.map_err(|e| Error::Http(e.to_string()))
        } else {
            let err: ApiErrorBody = resp.json().await.map_err(|e| Error::Http(e.to_string()))?;
            Err(Error::Api { code: err.code, message: err.message })
        }
    }

    // ── User Management ───────────────────────────────────────────────────

    /// Register a new end-user.
    ///
    /// `POST /v1/w3s/users`
    pub async fn create_user(&self, req: &CreateUserRequest) -> Result<UserResponse, Error> {
        self.post("/v1/w3s/users", req).await
    }

    /// List all end-users with optional filtering and pagination.
    ///
    /// `GET /v1/w3s/users`
    pub async fn list_users(&self, params: &ListUsersParams) -> Result<Users, Error> {
        self.get("/v1/w3s/users", params).await
    }

    /// Retrieve an end-user by their Circle user ID.
    ///
    /// `GET /v1/w3s/users/{id}`
    pub async fn get_user(&self, id: &str) -> Result<GetUserByIdResponse, Error> {
        let path = format!("/v1/w3s/users/{id}");
        self.get(&path, &[("", "")][..0]).await
    }

    /// Obtain a short-lived user token for the given user ID.
    ///
    /// `POST /v1/w3s/users/token`
    pub async fn get_user_token(
        &self,
        req: &GetUserTokenRequest,
    ) -> Result<UserTokenResponse, Error> {
        self.post("/v1/w3s/users/token", req).await
    }

    // ── Session Auth ──────────────────────────────────────────────────────

    /// Obtain a device token for social sign-in flows.
    ///
    /// `POST /v1/w3s/users/social/token`
    pub async fn get_device_token_social(
        &self,
        req: &DeviceTokenSocialRequest,
    ) -> Result<DeviceTokenSocialResponse, Error> {
        self.post("/v1/w3s/users/social/token", req).await
    }

    /// Obtain a device token and OTP for email sign-in flows.
    ///
    /// `POST /v1/w3s/users/email/token`
    pub async fn get_device_token_email(
        &self,
        req: &DeviceTokenEmailRequest,
    ) -> Result<DeviceTokenEmailResponse, Error> {
        self.post("/v1/w3s/users/email/token", req).await
    }

    /// Refresh a user token using a refresh token.
    ///
    /// `POST /v1/w3s/users/token/refresh`
    pub async fn refresh_user_token(
        &self,
        user_token: &str,
        req: &RefreshUserTokenRequest,
    ) -> Result<RefreshUserTokenResponse, Error> {
        self.post_with_user_token("/v1/w3s/users/token/refresh", req, user_token).await
    }

    /// Resend a one-time password to the user's email.
    ///
    /// `POST /v1/w3s/users/email/resendOTP`
    pub async fn resend_otp(
        &self,
        user_token: &str,
        req: &ResendOtpRequest,
    ) -> Result<ResendOtpResponse, Error> {
        self.post_with_user_token("/v1/w3s/users/email/resendOTP", req, user_token).await
    }

    // ── User-token endpoint ───────────────────────────────────────────────

    /// Retrieve the end-user record associated with the supplied user token.
    ///
    /// `GET /v1/w3s/user`
    pub async fn get_user_by_token(&self, user_token: &str) -> Result<UserResponse, Error> {
        self.get_with_user_token("/v1/w3s/user", &[("", "")][..0], user_token).await
    }

    // ── PIN Challenges ────────────────────────────────────────────────────

    /// Initialize a user's PIN and optionally create wallets in a single challenge.
    ///
    /// `POST /v1/w3s/user/initialize`
    pub async fn initialize_user(
        &self,
        user_token: &str,
        req: &SetPinAndInitWalletRequest,
    ) -> Result<ChallengeIdResponse, Error> {
        self.post_with_user_token("/v1/w3s/user/initialize", req, user_token).await
    }

    /// Create a challenge for the user to set their PIN.
    ///
    /// `POST /v1/w3s/user/pin`
    pub async fn create_pin_challenge(
        &self,
        user_token: &str,
        req: &SetPinRequest,
    ) -> Result<ChallengeIdResponse, Error> {
        self.post_with_user_token("/v1/w3s/user/pin", req, user_token).await
    }

    /// Create a challenge for the user to update their PIN.
    ///
    /// `PUT /v1/w3s/user/pin`
    pub async fn update_pin_challenge(
        &self,
        user_token: &str,
        req: &SetPinRequest,
    ) -> Result<ChallengeIdResponse, Error> {
        self.put_with_user_token("/v1/w3s/user/pin", req, user_token).await
    }

    /// Create a challenge for the user to restore a locked PIN.
    ///
    /// `POST /v1/w3s/user/pin/restore`
    pub async fn restore_pin_challenge(
        &self,
        user_token: &str,
        req: &SetPinRequest,
    ) -> Result<ChallengeIdResponse, Error> {
        self.post_with_user_token("/v1/w3s/user/pin/restore", req, user_token).await
    }

    /// List challenges for the authenticated user.
    ///
    /// `GET /v1/w3s/user/challenges`
    pub async fn list_challenges(&self, user_token: &str) -> Result<Challenges, Error> {
        self.get_with_user_token("/v1/w3s/user/challenges", &[("", "")][..0], user_token).await
    }

    /// Retrieve a single challenge by its ID.
    ///
    /// `GET /v1/w3s/user/challenges/{id}`
    pub async fn get_challenge(
        &self,
        user_token: &str,
        id: &str,
    ) -> Result<ChallengeResponse, Error> {
        let path = format!("/v1/w3s/user/challenges/{id}");
        self.get_with_user_token(&path, &[("", "")][..0], user_token).await
    }

    // ── Wallets ───────────────────────────────────────────────────────────

    /// Create new wallet(s) for the authenticated user.
    ///
    /// Returns a `challengeId` — the user must complete the challenge in the
    /// mobile SDK to finalise wallet creation.
    ///
    /// `POST /v1/w3s/user/wallets`
    pub async fn create_wallet(
        &self,
        user_token: &str,
        req: &CreateEndUserWalletRequest,
    ) -> Result<ChallengeIdResponse, Error> {
        self.post_with_user_token("/v1/w3s/user/wallets", req, user_token).await
    }

    /// List wallets accessible to the authenticated user.
    ///
    /// `GET /v1/w3s/wallets`
    pub async fn list_wallets(
        &self,
        user_token: &str,
        params: &ListWalletsParams,
    ) -> Result<Wallets, Error> {
        self.get_with_user_token("/v1/w3s/wallets", params, user_token).await
    }

    /// Retrieve a single wallet by its ID.
    ///
    /// `GET /v1/w3s/wallets/{id}`
    pub async fn get_wallet(&self, user_token: &str, id: &str) -> Result<WalletResponse, Error> {
        let path = format!("/v1/w3s/wallets/{id}");
        self.get_with_user_token(&path, &[("", "")][..0], user_token).await
    }

    /// Update the name or reference ID of a wallet.
    ///
    /// `PUT /v1/w3s/wallets/{id}`
    pub async fn update_wallet(
        &self,
        user_token: &str,
        id: &str,
        req: &UpdateWalletRequest,
    ) -> Result<WalletResponse, Error> {
        let path = format!("/v1/w3s/wallets/{id}");
        self.put_with_user_token(&path, req, user_token).await
    }

    /// List token balances for a wallet.
    ///
    /// `GET /v1/w3s/wallets/{id}/balances`
    pub async fn list_wallet_balances(
        &self,
        user_token: &str,
        wallet_id: &str,
        params: &ListWalletBalancesParams,
    ) -> Result<Balances, Error> {
        let path = format!("/v1/w3s/wallets/{wallet_id}/balances");
        self.get_with_user_token(&path, params, user_token).await
    }

    /// List NFTs held by a wallet.
    ///
    /// `GET /v1/w3s/wallets/{id}/nfts`
    pub async fn list_wallet_nfts(
        &self,
        user_token: &str,
        wallet_id: &str,
        params: &ListWalletNftsParams,
    ) -> Result<Nfts, Error> {
        let path = format!("/v1/w3s/wallets/{wallet_id}/nfts");
        self.get_with_user_token(&path, params, user_token).await
    }

    // ── Transactions ──────────────────────────────────────────────────────

    /// Initiate a token transfer transaction (returns a challengeId).
    ///
    /// `POST /v1/w3s/user/transactions/transfer`
    pub async fn create_transfer_transaction(
        &self,
        user_token: &str,
        req: &CreateTransferTxRequest,
    ) -> Result<ChallengeIdResponse, Error> {
        self.post_with_user_token("/v1/w3s/user/transactions/transfer", req, user_token).await
    }

    /// Accelerate a stuck transaction (returns a challengeId).
    ///
    /// `POST /v1/w3s/user/transactions/{id}/accelerate`
    pub async fn accelerate_transaction(
        &self,
        user_token: &str,
        id: &str,
        req: &AccelerateTxRequest,
    ) -> Result<ChallengeIdResponse, Error> {
        let path = format!("/v1/w3s/user/transactions/{id}/accelerate");
        self.post_with_user_token(&path, req, user_token).await
    }

    /// Cancel a pending transaction (returns a challengeId).
    ///
    /// `POST /v1/w3s/user/transactions/{id}/cancel`
    pub async fn cancel_transaction(
        &self,
        user_token: &str,
        id: &str,
        req: &CancelTxRequest,
    ) -> Result<ChallengeIdResponse, Error> {
        let path = format!("/v1/w3s/user/transactions/{id}/cancel");
        self.post_with_user_token(&path, req, user_token).await
    }

    /// Initiate a smart-contract execution transaction (returns a challengeId).
    ///
    /// `POST /v1/w3s/user/transactions/contractExecution`
    pub async fn create_contract_execution_transaction(
        &self,
        user_token: &str,
        req: &CreateContractExecutionTxRequest,
    ) -> Result<ChallengeIdResponse, Error> {
        self.post_with_user_token("/v1/w3s/user/transactions/contractExecution", req, user_token)
            .await
    }

    /// Initiate a wallet-upgrade transaction (returns a challengeId).
    ///
    /// `POST /v1/w3s/user/transactions/walletUpgrade`
    pub async fn create_wallet_upgrade_transaction(
        &self,
        user_token: &str,
        req: &CreateWalletUpgradeTxRequest,
    ) -> Result<ChallengeIdResponse, Error> {
        self.post_with_user_token("/v1/w3s/user/transactions/walletUpgrade", req, user_token).await
    }

    /// List transactions visible to the authenticated user.
    ///
    /// `GET /v1/w3s/transactions`
    pub async fn list_transactions(
        &self,
        user_token: &str,
        params: &ListTransactionsParams,
    ) -> Result<Transactions, Error> {
        self.get_with_user_token("/v1/w3s/transactions", params, user_token).await
    }

    /// Retrieve a single transaction by its ID.
    ///
    /// `GET /v1/w3s/transactions/{id}`
    pub async fn get_transaction(
        &self,
        user_token: &str,
        id: &str,
    ) -> Result<TransactionResponse, Error> {
        let path = format!("/v1/w3s/transactions/{id}");
        self.get_with_user_token(&path, &[("", "")][..0], user_token).await
    }

    /// Retrieve the transaction with the lowest pending nonce for an address.
    ///
    /// `GET /v1/w3s/transactions/lowestNonceTransaction`
    pub async fn get_lowest_nonce_transaction(
        &self,
        params: &GetLowestNonceTxParams,
    ) -> Result<GetLowestNonceTransactionResponse, Error> {
        self.get("/v1/w3s/transactions/lowestNonceTransaction", params).await
    }

    /// Estimate transfer transaction fees.
    ///
    /// `POST /v1/w3s/transactions/transfer/estimateFee`
    pub async fn estimate_transfer_fee(
        &self,
        user_token: &str,
        req: &EstimateTransferFeeRequest,
    ) -> Result<EstimateTransactionFee, Error> {
        self.post_with_user_token("/v1/w3s/transactions/transfer/estimateFee", req, user_token)
            .await
    }

    /// Estimate contract execution transaction fees.
    ///
    /// `POST /v1/w3s/transactions/contractExecution/estimateFee`
    pub async fn estimate_contract_execution_fee(
        &self,
        user_token: &str,
        req: &EstimateContractExecFeeRequest,
    ) -> Result<EstimateTransactionFee, Error> {
        self.post_with_user_token(
            "/v1/w3s/transactions/contractExecution/estimateFee",
            req,
            user_token,
        )
        .await
    }

    /// Validate that an address is valid for a given blockchain.
    ///
    /// `POST /v1/w3s/transactions/validateAddress`
    pub async fn validate_address(
        &self,
        req: &ValidateAddressRequest,
    ) -> Result<ValidateAddressResponse, Error> {
        self.post("/v1/w3s/transactions/validateAddress", req).await
    }

    // ── Token ─────────────────────────────────────────────────────────────

    /// Retrieve token metadata by its Circle token ID.
    ///
    /// `GET /v1/w3s/tokens/{id}`
    pub async fn get_token(&self, id: &str) -> Result<TokenResponse, Error> {
        let path = format!("/v1/w3s/tokens/{id}");
        self.get(&path, &[("", "")][..0]).await
    }

    // ── Signing ───────────────────────────────────────────────────────────

    /// Request a message signing challenge (returns a challengeId).
    ///
    /// `POST /v1/w3s/user/sign/message`
    pub async fn sign_message(
        &self,
        user_token: &str,
        req: &SignMessageRequest,
    ) -> Result<ChallengeIdResponse, Error> {
        self.post_with_user_token("/v1/w3s/user/sign/message", req, user_token).await
    }

    /// Request an EIP-712 typed data signing challenge (returns a challengeId).
    ///
    /// `POST /v1/w3s/user/sign/typedData`
    pub async fn sign_typed_data(
        &self,
        user_token: &str,
        req: &SignTypedDataRequest,
    ) -> Result<ChallengeIdResponse, Error> {
        self.post_with_user_token("/v1/w3s/user/sign/typedData", req, user_token).await
    }

    /// Request a raw transaction signing challenge (returns a challengeId).
    ///
    /// `POST /v1/w3s/user/sign/transaction`
    pub async fn sign_transaction(
        &self,
        user_token: &str,
        req: &SignTransactionRequest,
    ) -> Result<ChallengeIdResponse, Error> {
        self.post_with_user_token("/v1/w3s/user/sign/transaction", req, user_token).await
    }
}
