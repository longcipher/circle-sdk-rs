//! Wallet resource models for the Circle Developer-Controlled Wallets API.
//!
//! Contains request parameters and response types for wallet management
//! endpoints including balances and NFTs.

use super::common::{AccountType, Blockchain, CustodyType, PageParams, TokenStandard, WalletState};

/// NFT token standard.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum NftStandard {
    /// ERC-721 non-fungible token.
    #[serde(rename = "ERC721")]
    Erc721,
    /// ERC-1155 multi-token.
    #[serde(rename = "ERC1155")]
    Erc1155,
}

/// Fungible token standard (for wallet balance queries).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum FtStandard {
    /// Native coin (no ERC standard).
    #[serde(rename = "")]
    Native,
    /// ERC-20 fungible token.
    #[serde(rename = "ERC20")]
    Erc20,
}

/// Smart Contract Account core implementation variant.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ScaCore {
    /// Circle ERC-4337 v1 implementation.
    #[serde(rename = "circle_4337_v1")]
    Circle4337V1,
    /// Circle ERC-6900 single-owner v1 implementation.
    #[serde(rename = "circle_6900_singleowner_v1")]
    Circle6900SingleownerV1,
    /// Circle ERC-6900 single-owner v2 implementation.
    #[serde(rename = "circle_6900_singleowner_v2")]
    Circle6900SingleownerV2,
    /// Circle ERC-6900 single-owner v3 implementation.
    #[serde(rename = "circle_6900_singleowner_v3")]
    Circle6900SingleownerV3,
}

/// A blockchain token definition.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    /// Unique token ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Blockchain network the token lives on.
    pub blockchain: Blockchain,
    /// Whether this token is the native coin of its chain.
    pub is_native: bool,
    /// Token name (e.g. `"USD Coin"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Token standard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<TokenStandard>,
    /// Number of decimal places.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimals: Option<i32>,
    /// Chain symbol (e.g. `"USDC"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Contract address (absent for native coins).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_address: Option<String>,
    /// ISO-8601 last-update timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<String>,
    /// ISO-8601 creation timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
}

/// A single fungible token balance entry.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    /// Token amount as a decimal string.
    pub amount: String,
    /// Token definition.
    pub token: Token,
    /// ISO-8601 last-update timestamp.
    pub update_date: String,
}

/// Wallet metadata for creation requests.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletMetadata {
    /// Human-readable name for the wallet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// External reference ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
}

/// A developer-controlled wallet resource.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wallet {
    /// Unique wallet ID (UUID).
    pub id: String,
    /// On-chain wallet address.
    pub address: String,
    /// Blockchain network this wallet is on.
    pub blockchain: Blockchain,
    /// ISO-8601 creation timestamp.
    pub create_date: String,
    /// ISO-8601 last-update timestamp.
    pub update_date: String,
    /// Custody type.
    pub custody_type: CustodyType,
    /// Human-readable name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// External reference ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
    /// Wallet lifecycle state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<WalletState>,
    /// User ID (user-controlled wallets).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Wallet set ID the wallet belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_set_id: Option<String>,
    /// Initial public key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_public_key: Option<String>,
    /// Account type (SCA or EOA).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<AccountType>,
    /// SCA core implementation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sca_core: Option<ScaCore>,
    /// Token balances (populated when requested via `include_all` or balance endpoints).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_balances: Option<Vec<Balance>>,
}

/// Inner data of a list-wallets response.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletsData {
    /// Wallets matching the query.
    pub wallets: Vec<Wallet>,
}

/// Response wrapper for the list-wallets and create-wallets endpoints.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Wallets {
    /// Response data.
    pub data: WalletsData,
}

/// Inner data of a single-wallet response.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletData {
    /// The wallet.
    pub wallet: Wallet,
}

/// Response wrapper for get/update wallet endpoints.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WalletResponse {
    /// Response data.
    pub data: WalletData,
}

/// Inner data of the list-wallet-balances (developer-level) response.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletsWithBalancesData {
    /// Wallets with their token balance details.
    pub wallets: Vec<Wallet>,
}

/// Response wrapper for the list-developer-wallet-balances endpoint.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WalletsWithBalances {
    /// Response data.
    pub data: WalletsWithBalancesData,
}

/// Inner data of single-wallet token-balances response.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalancesData {
    /// Token balances for the wallet.
    pub token_balances: Vec<Balance>,
}

/// Response wrapper for the per-wallet balances endpoint.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Balances {
    /// Response data.
    pub data: BalancesData,
}

/// A single NFT holding.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nft {
    /// NFT amount (ERC-1155 can have quantity > 1).
    pub amount: String,
    /// Token definition.
    pub token: Token,
    /// ISO-8601 last-update timestamp.
    pub update_date: String,
    /// On-chain token ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nft_token_id: Option<String>,
    /// IPFS or HTTP URI of the NFT metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
}

/// Inner data of a list-wallet-NFTs response.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NftsData {
    /// NFTs matching the query.
    pub nfts: Vec<Nft>,
}

/// Response wrapper for wallet NFT endpoints.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Nfts {
    /// Response data.
    pub data: NftsData,
}

/// Request body for creating one or more developer-controlled wallets.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWalletsRequest {
    /// Idempotency key (UUID) to deduplicate requests.
    pub idempotency_key: String,
    /// Encrypted entity secret ciphertext.
    pub entity_secret_ciphertext: String,
    /// Wallet set ID the wallets should belong to.
    pub wallet_set_id: String,
    /// Blockchain networks to create wallets on.
    pub blockchains: Vec<Blockchain>,
    /// Account type (SCA or EOA); defaults to EOA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<AccountType>,
    /// Number of wallets to create (default 1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
    /// Per-wallet metadata overrides.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Vec<WalletMetadata>>,
}

/// Request body for updating a wallet.
#[derive(Debug, Clone, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWalletRequest {
    /// New human-readable name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New external reference ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
}

/// Query parameters for the list-wallets endpoint.
#[derive(Debug, Default, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWalletsParams {
    /// Filter by blockchain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockchain: Option<Blockchain>,
    /// Filter by wallet address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Filter by wallet set ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_set_id: Option<String>,
    /// Filter by external reference ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
    /// Filter by wallet state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<WalletState>,
    /// Filter by custody type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custody_type: Option<CustodyType>,
    /// Pagination parameters.
    #[serde(flatten)]
    pub page: PageParams,
}

/// Query parameters for the developer list-wallet-balances endpoint.
#[derive(Debug, Default, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWalletBalancesParams {
    /// Include all wallets, not just those with balances.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_all: Option<bool>,
    /// Filter by token name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Filter by token contract address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_address: Option<String>,
    /// Filter by blockchain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockchain: Option<Blockchain>,
    /// Filter by wallet set ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_set_id: Option<String>,
    /// Filter by specific wallet IDs (comma-separated string).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_ids: Option<String>,
    /// Filter by custody type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custody_type: Option<CustodyType>,
    /// Filter by wallet address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Pagination parameters.
    #[serde(flatten)]
    pub page: PageParams,
}

/// Query parameters for per-wallet balance and NFT endpoints.
#[derive(Debug, Default, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletNftsParams {
    /// Pagination parameters.
    #[serde(flatten)]
    pub page: PageParams,
}

/// Query parameters for the list-wallet-NFTs endpoint.
#[derive(Debug, Default, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWalletNftsParams {
    /// Filter by NFT standard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<NftStandard>,
    /// Filter by token name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Filter by token contract address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_address: Option<String>,
    /// Pagination parameters.
    #[serde(flatten)]
    pub page: PageParams,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wallet_response_deserializes() -> Result<(), Box<dyn std::error::Error>> {
        let json = r#"{
            "data": {
                "wallet": {
                    "id": "wallet-id-1",
                    "address": "0x1234",
                    "blockchain": "ETH",
                    "createDate": "2024-01-01T00:00:00Z",
                    "updateDate": "2024-01-01T00:00:00Z",
                    "custodyType": "DEVELOPER",
                    "state": "LIVE"
                }
            }
        }"#;
        let resp: WalletResponse = serde_json::from_str(json)?;
        assert_eq!(resp.data.wallet.id, "wallet-id-1");
        assert_eq!(resp.data.wallet.blockchain, Blockchain::Eth);
        assert_eq!(resp.data.wallet.state, Some(WalletState::Live));
        Ok(())
    }

    #[test]
    fn wallets_response_deserializes() -> Result<(), Box<dyn std::error::Error>> {
        let json = r#"{
            "data": {
                "wallets": [
                    {
                        "id": "w1",
                        "address": "0xabc",
                        "blockchain": "MATIC",
                        "createDate": "2024-01-01T00:00:00Z",
                        "updateDate": "2024-01-01T00:00:00Z",
                        "custodyType": "DEVELOPER"
                    }
                ]
            }
        }"#;
        let resp: Wallets = serde_json::from_str(json)?;
        assert_eq!(resp.data.wallets.len(), 1);
        assert_eq!(resp.data.wallets[0].blockchain, Blockchain::Matic);
        Ok(())
    }

    #[test]
    fn nfts_response_deserializes() -> Result<(), Box<dyn std::error::Error>> {
        let json = r#"{
            "data": {
                "nfts": [
                    {
                        "amount": "1",
                        "token": {
                            "blockchain": "BASE",
                            "isNative": false
                        },
                        "updateDate": "2024-01-01T00:00:00Z",
                        "nftTokenId": "42"
                    }
                ]
            }
        }"#;
        let resp: Nfts = serde_json::from_str(json)?;
        assert_eq!(resp.data.nfts.len(), 1);
        assert_eq!(resp.data.nfts[0].nft_token_id.as_deref(), Some("42"));
        Ok(())
    }

    #[test]
    fn sca_core_serializes() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string(&ScaCore::Circle4337V1)?;
        assert_eq!(json, "\"circle_4337_v1\"");
        let back: ScaCore = serde_json::from_str(&json)?;
        assert_eq!(back, ScaCore::Circle4337V1);
        Ok(())
    }

    #[test]
    fn create_wallets_request_serializes() -> Result<(), Box<dyn std::error::Error>> {
        let req = CreateWalletsRequest {
            idempotency_key: "key".to_string(),
            entity_secret_ciphertext: "cipher".to_string(),
            wallet_set_id: "set-id".to_string(),
            blockchains: vec![Blockchain::Eth],
            account_type: None,
            count: Some(2),
            metadata: None,
        };
        let json = serde_json::to_string(&req)?;
        assert!(json.contains("walletSetId"));
        assert!(json.contains("\"ETH\""));
        Ok(())
    }
}
