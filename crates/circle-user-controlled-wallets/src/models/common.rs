//! Common types shared across the User-Controlled Wallets API.
//!
//! Includes shared pagination, blockchain, error, and identifier types used
//! across user-controlled wallet endpoints.

use serde::{Deserialize, Serialize};

// ── Error body ──────────────────────────────────────────────────────────────

/// API error response body returned by Circle on non-2xx responses.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiErrorBody {
    /// Numeric error code from Circle.
    pub code: i32,
    /// Human-readable error message from Circle.
    pub message: String,
}

// ── Blockchain ───────────────────────────────────────────────────────────────

/// Blockchain network identifier used throughout the Circle API.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Blockchain {
    /// Ethereum mainnet.
    Eth,
    /// Ethereum Sepolia testnet.
    #[serde(rename = "ETH-SEPOLIA")]
    EthSepolia,
    /// Avalanche C-Chain mainnet.
    Avax,
    /// Avalanche Fuji testnet.
    #[serde(rename = "AVAX-FUJI")]
    AvaxFuji,
    /// Polygon (Matic) mainnet.
    Matic,
    /// Polygon Amoy testnet.
    #[serde(rename = "MATIC-AMOY")]
    MaticAmoy,
    /// Solana mainnet.
    Sol,
    /// Solana devnet.
    #[serde(rename = "SOL-DEVNET")]
    SolDevnet,
    /// Arbitrum One mainnet.
    Arb,
    /// Arbitrum Sepolia testnet.
    #[serde(rename = "ARB-SEPOLIA")]
    ArbSepolia,
    /// NEAR Protocol mainnet.
    Near,
    /// NEAR Protocol testnet.
    #[serde(rename = "NEAR-TESTNET")]
    NearTestnet,
    /// Generic EVM-compatible chain.
    Evm,
    /// Generic EVM testnet.
    #[serde(rename = "EVM-TESTNET")]
    EvmTestnet,
    /// Unichain mainnet.
    Uni,
    /// Unichain Sepolia testnet.
    #[serde(rename = "UNI-SEPOLIA")]
    UniSepolia,
    /// Base mainnet.
    Base,
    /// Base Sepolia testnet.
    #[serde(rename = "BASE-SEPOLIA")]
    BaseSepolia,
    /// Optimism mainnet.
    Op,
    /// Optimism Sepolia testnet.
    #[serde(rename = "OP-SEPOLIA")]
    OpSepolia,
    /// Aptos mainnet.
    Aptos,
    /// Aptos testnet.
    #[serde(rename = "APTOS-TESTNET")]
    AptosTestnet,
    /// ARC testnet.
    #[serde(rename = "ARC-TESTNET")]
    ArcTestnet,
    /// Monad mainnet.
    Monad,
    /// Monad testnet.
    #[serde(rename = "MONAD-TESTNET")]
    MonadTestnet,
}

// ── Token standard ───────────────────────────────────────────────────────────

/// Token standard identifier.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TokenStandard {
    /// ERC-20 fungible token.
    Erc20,
    /// ERC-721 non-fungible token.
    Erc721,
    /// ERC-1155 multi-token standard.
    Erc1155,
    /// Solana fungible token.
    Fungible,
    /// Solana fungible asset.
    FungibleAsset,
    /// Solana non-fungible token.
    NonFungible,
    /// Solana non-fungible edition.
    NonFungibleEdition,
    /// Solana programmable non-fungible token.
    ProgrammableNonFungible,
    /// Solana programmable non-fungible edition.
    ProgrammableNonFungibleEdition,
}

// ── Account / Custody types ──────────────────────────────────────────────────

/// Wallet account type.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountType {
    /// Smart Contract Account (ERC-4337 / ERC-6900).
    Sca,
    /// Externally Owned Account.
    Eoa,
}

/// Custody type for a wallet.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CustodyType {
    /// Wallet belongs to the developer.
    Developer,
    /// Wallet belongs to an end-user.
    Enduser,
}

// ── Wallet state ─────────────────────────────────────────────────────────────

/// Operational state of a wallet.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WalletState {
    /// Wallet is active and can transact.
    Live,
    /// Wallet has been frozen.
    Frozen,
}

// ── Fee level ─────────────────────────────────────────────────────────────────

/// Gas fee level preference for a transaction.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FeeLevel {
    /// Low-priority fee level.
    Low,
    /// Standard fee level.
    Medium,
    /// High-priority fee level.
    High,
}

// ── SCA core version ──────────────────────────────────────────────────────────

/// Smart Contract Account core version.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
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

// ── Transaction fee details ───────────────────────────────────────────────────

/// Detailed fee break-down for a transaction.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionFee {
    /// Gas limit for the transaction.
    pub gas_limit: Option<String>,
    /// Gas price in wei.
    pub gas_price: Option<String>,
    /// EIP-1559 max fee per gas.
    pub max_fee: Option<String>,
    /// EIP-1559 max priority fee per gas.
    pub priority_fee: Option<String>,
    /// Base fee per gas at the time of estimation.
    pub base_fee: Option<String>,
    /// Total network fee amount.
    pub network_fee: Option<String>,
    /// Total network fee in raw units.
    pub network_fee_raw: Option<String>,
    /// Layer-1 data fee (for L2 networks).
    pub l1_fee: Option<String>,
}

// ── Pagination ────────────────────────────────────────────────────────────────

/// Query parameters for paginated list endpoints.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PageParams {
    /// Filter results from this date-time (ISO 8601).
    pub from: Option<String>,
    /// Filter results to this date-time (ISO 8601).
    pub to: Option<String>,
    /// Cursor for the previous page.
    #[serde(rename = "pageBefore", skip_serializing_if = "Option::is_none")]
    pub page_before: Option<String>,
    /// Cursor for the next page.
    #[serde(rename = "pageAfter", skip_serializing_if = "Option::is_none")]
    pub page_after: Option<String>,
    /// Number of items per page (max 50).
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blockchain_eth_serializes_to_eth() -> Result<(), Box<dyn std::error::Error>> {
        let s = serde_json::to_string(&Blockchain::Eth)?;
        assert_eq!(s, "\"ETH\"");
        Ok(())
    }

    #[test]
    fn blockchain_eth_sepolia_serializes_with_hyphen() -> Result<(), Box<dyn std::error::Error>> {
        let s = serde_json::to_string(&Blockchain::EthSepolia)?;
        assert_eq!(s, "\"ETH-SEPOLIA\"");
        Ok(())
    }

    #[test]
    fn blockchain_avax_fuji_serializes_with_hyphen() -> Result<(), Box<dyn std::error::Error>> {
        let s = serde_json::to_string(&Blockchain::AvaxFuji)?;
        assert_eq!(s, "\"AVAX-FUJI\"");
        Ok(())
    }

    #[test]
    fn blockchain_round_trips() -> Result<(), Box<dyn std::error::Error>> {
        let original = Blockchain::BaseSepolia;
        let json = serde_json::to_string(&original)?;
        assert_eq!(json, "\"BASE-SEPOLIA\"");
        let decoded: Blockchain = serde_json::from_str(&json)?;
        assert_eq!(decoded, original);
        Ok(())
    }

    #[test]
    fn sca_core_serializes_to_snake_case() -> Result<(), Box<dyn std::error::Error>> {
        let s = serde_json::to_string(&ScaCore::Circle4337V1)?;
        assert_eq!(s, "\"circle_4337_v1\"");
        let s2 = serde_json::to_string(&ScaCore::Circle6900SingleownerV2)?;
        assert_eq!(s2, "\"circle_6900_singleowner_v2\"");
        Ok(())
    }

    #[test]
    fn fee_level_screaming_snake_case() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(serde_json::to_string(&FeeLevel::Medium)?, "\"MEDIUM\"");
        Ok(())
    }

    #[test]
    fn page_params_camel_case_keys() -> Result<(), Box<dyn std::error::Error>> {
        let p = PageParams {
            page_before: Some("abc".to_string()),
            page_size: Some(10),
            ..Default::default()
        };
        let s = serde_json::to_string(&p)?;
        assert!(s.contains("pageBefore"), "expected pageBefore in {s}");
        assert!(s.contains("pageSize"), "expected pageSize in {s}");
        Ok(())
    }
}
