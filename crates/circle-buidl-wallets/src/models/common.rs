//! Common types shared across the Buidl Wallets API.
//!
//! Includes the [`Blockchain`] enum, pagination parameters, and the API error
//! response type.

/// Blockchain network identifier.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE")]
pub enum Blockchain {
    /// Ethereum mainnet.
    Eth,
    /// Ethereum Sepolia testnet.
    #[serde(rename = "ETH-SEPOLIA")]
    EthSepolia,
    /// Polygon PoS mainnet.
    Matic,
    /// Polygon Amoy testnet.
    #[serde(rename = "MATIC-AMOY")]
    MaticAmoy,
    /// Arbitrum One mainnet.
    Arb,
    /// Arbitrum Sepolia testnet.
    #[serde(rename = "ARB-SEPOLIA")]
    ArbSepolia,
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
    /// Avalanche C-Chain mainnet.
    Avax,
    /// Avalanche Fuji testnet.
    #[serde(rename = "AVAX-FUJI")]
    AvaxFuji,
    /// ARC testnet.
    #[serde(rename = "ARC-TESTNET")]
    ArcTestnet,
    /// Monad mainnet.
    Monad,
    /// Monad testnet.
    #[serde(rename = "MONAD-TESTNET")]
    MonadTestnet,
}

/// Pagination cursor parameters shared across list endpoints.
#[derive(Debug, Default, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageParams {
    /// Start of date-time range (ISO-8601, inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// End of date-time range (ISO-8601, inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    /// Cursor for the previous page (exclusive end).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_before: Option<String>,
    /// Cursor for the next page (exclusive start).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_after: Option<String>,
    /// Maximum number of items to return (1–50, default 10).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
}

/// Error response body returned by the Circle API on non-2xx status codes.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct ApiErrorBody {
    /// Numeric error code.
    pub code: i32,
    /// Human-readable error message.
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blockchain_serializes_to_screaming_kebab() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string(&Blockchain::EthSepolia)?;
        assert_eq!(json, "\"ETH-SEPOLIA\"");
        let json2 = serde_json::to_string(&Blockchain::Eth)?;
        assert_eq!(json2, "\"ETH\"");
        Ok(())
    }

    #[test]
    fn blockchain_deserializes() -> Result<(), Box<dyn std::error::Error>> {
        let b: Blockchain = serde_json::from_str("\"MATIC-AMOY\"")?;
        assert_eq!(b, Blockchain::MaticAmoy);
        Ok(())
    }
}
