//! Wallet resource models for the Circle User-Controlled Wallets API.
//!
//! Contains request parameters and response types for wallet management
//! endpoints, including balances, NFTs, and associated token data.

use serde::{Deserialize, Serialize};

use super::common::{
    AccountType, Blockchain, CustodyType, PageParams, ScaCore, TokenStandard, WalletState,
};

// ── Wallet metadata ───────────────────────────────────────────────────────────

/// Optional name / reference metadata for a wallet.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletMetadata {
    /// Display name for the wallet.
    pub name: Option<String>,
    /// Application-defined reference identifier.
    pub ref_id: Option<String>,
}

// ── Token ─────────────────────────────────────────────────────────────────────

/// On-chain token descriptor.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    /// Circle-assigned token ID.
    pub id: String,
    /// Blockchain this token lives on.
    pub blockchain: Blockchain,
    /// `true` if this is the native coin of the chain (e.g. ETH, MATIC).
    pub is_native: bool,
    /// Human-readable token name.
    pub name: Option<String>,
    /// Token standard.
    pub standard: Option<TokenStandard>,
    /// Number of decimal places.
    pub decimals: Option<i32>,
    /// Token ticker symbol.
    pub symbol: Option<String>,
    /// On-chain contract address (absent for native coins).
    pub token_address: Option<String>,
    /// ISO 8601 last-updated timestamp.
    pub update_date: String,
    /// ISO 8601 creation timestamp.
    pub create_date: String,
}

// ── Balance ───────────────────────────────────────────────────────────────────

/// A token balance held by a wallet.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    /// Decimal string representation of the amount.
    pub amount: String,
    /// Token descriptor.
    pub token: Token,
    /// ISO 8601 timestamp of the last balance update.
    pub update_date: String,
}

/// `data` payload wrapping a list of balances.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BalancesData {
    /// Token balances held by the wallet.
    pub token_balances: Vec<Balance>,
}

/// Response envelope for list-wallet-balances.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Balances {
    /// List of balances.
    pub data: BalancesData,
}

// ── NFT ───────────────────────────────────────────────────────────────────────

/// NFT on-chain metadata.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NftMetadata {
    /// NFT display name.
    pub name: Option<String>,
    /// NFT description.
    pub description: Option<String>,
    /// URL of the NFT image.
    pub image: Option<String>,
}

/// An NFT held by a wallet.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Nft {
    /// Token descriptor for this NFT collection.
    pub token: Token,
    /// Quantity held (usually "1" for ERC-721).
    pub amount: String,
    /// ISO 8601 last-updated timestamp.
    pub update_date: String,
    /// On-chain token ID within the collection.
    pub nft_token_id: Option<String>,
    /// Off-chain metadata (name, description, image).
    pub metadata: Option<NftMetadata>,
}

/// `data` payload wrapping a list of NFTs.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NftsData {
    /// NFTs held by the wallet.
    pub nfts: Vec<Nft>,
}

/// Response envelope for list-wallet-nfts.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Nfts {
    /// List of NFTs.
    pub data: NftsData,
}

// ── Wallet ────────────────────────────────────────────────────────────────────

/// A user-controlled wallet.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Wallet {
    /// Circle-assigned wallet ID.
    pub id: String,
    /// On-chain address for this wallet.
    pub address: String,
    /// Blockchain this wallet is deployed on.
    pub blockchain: Blockchain,
    /// ISO 8601 creation timestamp.
    pub create_date: String,
    /// ISO 8601 last-updated timestamp.
    pub update_date: String,
    /// Custody model (always `Enduser` for this API).
    pub custody_type: CustodyType,
    /// Optional display name.
    pub name: Option<String>,
    /// Application-defined reference identifier.
    pub ref_id: Option<String>,
    /// Current wallet state.
    pub state: WalletState,
    /// ID of the end-user who owns this wallet.
    pub user_id: Option<String>,
    /// ID of the wallet set this wallet belongs to.
    pub wallet_set_id: String,
    /// Initial public key at wallet creation.
    pub initial_public_key: Option<String>,
    /// Account type (EOA or SCA).
    pub account_type: Option<AccountType>,
    /// SCA core version (for smart contract accounts).
    pub sca_core: Option<ScaCore>,
}

/// `data` payload wrapping a list of wallets.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletsData {
    /// List of wallets.
    pub wallets: Vec<Wallet>,
}

/// Response envelope for list-wallets.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Wallets {
    /// Paginated wallets.
    pub data: WalletsData,
}

/// `data` payload wrapping a single wallet.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletData {
    /// The wallet record.
    pub wallet: Wallet,
}

/// Response envelope for a single-wallet operation.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletResponse {
    /// Wallet data.
    pub data: WalletData,
}

// ── Token response types ──────────────────────────────────────────────────────

/// `data` payload wrapping a single token.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenData {
    /// The token descriptor.
    pub token: Token,
}

/// Response envelope for `getToken`.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenResponse {
    /// Token data.
    pub data: TokenData,
}

// ── Request / param types ─────────────────────────────────────────────────────

/// Request body for `updateWallet`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWalletRequest {
    /// New display name.
    pub name: Option<String>,
    /// New application reference ID.
    pub ref_id: Option<String>,
}

/// Query parameters for `listWallets`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWalletsParams {
    /// Filter by on-chain address.
    pub address: Option<String>,
    /// Filter by blockchain.
    pub blockchain: Option<Blockchain>,
    /// Filter by SCA core version.
    pub sca_core: Option<ScaCore>,
    /// Filter by wallet set ID.
    pub wallet_set_id: Option<String>,
    /// Filter by application reference ID.
    pub ref_id: Option<String>,
    /// Pagination cursors.
    #[serde(flatten)]
    pub page: PageParams,
}

/// Query parameters for `listWalletBalances`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWalletBalancesParams {
    /// If `true`, include all tokens even if balance is zero.
    pub include_all: Option<bool>,
    /// Filter by token name.
    pub name: Option<String>,
    /// Filter by token contract address.
    pub token_address: Option<String>,
    /// Filter by token standard.
    pub standard: Option<TokenStandard>,
    /// Pagination cursors.
    #[serde(flatten)]
    pub page: PageParams,
}

/// Query parameters for `listWalletNfts`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWalletNftsParams {
    /// If `true`, include all NFTs even if amount is zero.
    pub include_all: Option<bool>,
    /// Filter by NFT name.
    pub name: Option<String>,
    /// Filter by token contract address.
    pub token_address: Option<String>,
    /// Filter by token standard.
    pub standard: Option<TokenStandard>,
    /// Pagination cursors.
    #[serde(flatten)]
    pub page: PageParams,
}

/// Request body for `createEndUserWallet`.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEndUserWalletRequest {
    /// Client-generated idempotency key (UUID).
    pub idempotency_key: String,
    /// Blockchains on which to create wallets.
    pub blockchains: Vec<Blockchain>,
    /// Account type for the new wallet.
    pub account_type: Option<AccountType>,
    /// Optional per-wallet metadata.
    pub metadata: Option<Vec<WalletMetadata>>,
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wallet_state_screaming() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(serde_json::to_string(&WalletState::Live)?, "\"LIVE\"");
        assert_eq!(serde_json::to_string(&WalletState::Frozen)?, "\"FROZEN\"");
        Ok(())
    }

    #[test]
    fn update_wallet_request_camel_case() -> Result<(), Box<dyn std::error::Error>> {
        let req = UpdateWalletRequest { name: Some("myWallet".to_string()), ref_id: None };
        let s = serde_json::to_string(&req)?;
        assert!(s.contains("\"name\""), "expected name in {s}");
        Ok(())
    }

    #[test]
    fn create_wallet_request_camel_case() -> Result<(), Box<dyn std::error::Error>> {
        let req = CreateEndUserWalletRequest {
            idempotency_key: "abc-def".to_string(),
            blockchains: vec![Blockchain::Eth, Blockchain::EthSepolia],
            account_type: None,
            metadata: None,
        };
        let s = serde_json::to_string(&req)?;
        assert!(s.contains("idempotencyKey"), "expected idempotencyKey in {s}");
        assert!(s.contains("ETH-SEPOLIA"), "expected ETH-SEPOLIA in {s}");
        Ok(())
    }
}
