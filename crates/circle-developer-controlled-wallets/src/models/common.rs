//! Common types shared across the Developer-Controlled Wallets API.
//!
//! Includes shared pagination, blockchain, error, and identifier types used
//! across developer-controlled wallet endpoints.

/// Blockchain network identifier.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Blockchain {
    /// Ethereum mainnet.
    #[serde(rename = "ETH")]
    Eth,
    /// Ethereum Sepolia testnet.
    #[serde(rename = "ETH-SEPOLIA")]
    EthSepolia,
    /// Avalanche C-Chain mainnet.
    #[serde(rename = "AVAX")]
    Avax,
    /// Avalanche Fuji testnet.
    #[serde(rename = "AVAX-FUJI")]
    AvaxFuji,
    /// Polygon PoS mainnet.
    #[serde(rename = "MATIC")]
    Matic,
    /// Polygon Amoy testnet.
    #[serde(rename = "MATIC-AMOY")]
    MaticAmoy,
    /// Solana mainnet.
    #[serde(rename = "SOL")]
    Sol,
    /// Solana devnet.
    #[serde(rename = "SOL-DEVNET")]
    SolDevnet,
    /// Arbitrum One mainnet.
    #[serde(rename = "ARB")]
    Arb,
    /// Arbitrum Sepolia testnet.
    #[serde(rename = "ARB-SEPOLIA")]
    ArbSepolia,
    /// NEAR mainnet.
    #[serde(rename = "NEAR")]
    Near,
    /// NEAR testnet.
    #[serde(rename = "NEAR-TESTNET")]
    NearTestnet,
    /// Generic EVM mainnet.
    #[serde(rename = "EVM")]
    Evm,
    /// Generic EVM testnet.
    #[serde(rename = "EVM-TESTNET")]
    EvmTestnet,
    /// Unichain mainnet.
    #[serde(rename = "UNI")]
    Uni,
    /// Unichain Sepolia testnet.
    #[serde(rename = "UNI-SEPOLIA")]
    UniSepolia,
    /// Base mainnet.
    #[serde(rename = "BASE")]
    Base,
    /// Base Sepolia testnet.
    #[serde(rename = "BASE-SEPOLIA")]
    BaseSepolia,
    /// Optimism mainnet.
    #[serde(rename = "OP")]
    Op,
    /// Optimism Sepolia testnet.
    #[serde(rename = "OP-SEPOLIA")]
    OpSepolia,
    /// Aptos mainnet.
    #[serde(rename = "APTOS")]
    Aptos,
    /// Aptos testnet.
    #[serde(rename = "APTOS-TESTNET")]
    AptosTestnet,
    /// ARC testnet.
    #[serde(rename = "ARC-TESTNET")]
    ArcTestnet,
    /// Monad mainnet.
    #[serde(rename = "MONAD")]
    Monad,
    /// Monad testnet.
    #[serde(rename = "MONAD-TESTNET")]
    MonadTestnet,
}

/// EVM-compatible blockchain network identifier.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum EvmBlockchain {
    /// Ethereum mainnet.
    #[serde(rename = "ETH")]
    Eth,
    /// Ethereum Sepolia testnet.
    #[serde(rename = "ETH-SEPOLIA")]
    EthSepolia,
    /// Avalanche C-Chain mainnet.
    #[serde(rename = "AVAX")]
    Avax,
    /// Avalanche Fuji testnet.
    #[serde(rename = "AVAX-FUJI")]
    AvaxFuji,
    /// Polygon PoS mainnet.
    #[serde(rename = "MATIC")]
    Matic,
    /// Polygon Amoy testnet.
    #[serde(rename = "MATIC-AMOY")]
    MaticAmoy,
    /// Arbitrum One mainnet.
    #[serde(rename = "ARB")]
    Arb,
    /// Arbitrum Sepolia testnet.
    #[serde(rename = "ARB-SEPOLIA")]
    ArbSepolia,
    /// Unichain mainnet.
    #[serde(rename = "UNI")]
    Uni,
    /// Unichain Sepolia testnet.
    #[serde(rename = "UNI-SEPOLIA")]
    UniSepolia,
    /// Base mainnet.
    #[serde(rename = "BASE")]
    Base,
    /// Base Sepolia testnet.
    #[serde(rename = "BASE-SEPOLIA")]
    BaseSepolia,
    /// Optimism mainnet.
    #[serde(rename = "OP")]
    Op,
    /// Optimism Sepolia testnet.
    #[serde(rename = "OP-SEPOLIA")]
    OpSepolia,
    /// Generic EVM mainnet.
    #[serde(rename = "EVM")]
    Evm,
    /// Generic EVM testnet.
    #[serde(rename = "EVM-TESTNET")]
    EvmTestnet,
    /// ARC testnet.
    #[serde(rename = "ARC-TESTNET")]
    ArcTestnet,
    /// Monad mainnet.
    #[serde(rename = "MONAD")]
    Monad,
    /// Monad testnet.
    #[serde(rename = "MONAD-TESTNET")]
    MonadTestnet,
}

/// Custody type for a wallet.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CustodyType {
    /// Wallet is under developer custody.
    Developer,
    /// Wallet is under end-user custody.
    Enduser,
}

/// Smart contract account type.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountType {
    /// Smart Contract Account (ERC-4337).
    Sca,
    /// Externally Owned Account.
    Eoa,
}

/// Wallet lifecycle state.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WalletState {
    /// Wallet is active and can send transactions.
    Live,
    /// Wallet is frozen and cannot send transactions.
    Frozen,
}

/// Transaction fee priority level.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FeeLevel {
    /// Low priority.
    Low,
    /// Medium priority.
    Medium,
    /// High priority.
    High,
}

/// Token standard.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TokenStandard {
    /// ERC-20 fungible token.
    #[serde(rename = "ERC20")]
    Erc20,
    /// ERC-721 non-fungible token.
    #[serde(rename = "ERC721")]
    Erc721,
    /// ERC-1155 multi-token.
    #[serde(rename = "ERC1155")]
    Erc1155,
    /// Solana fungible token.
    #[serde(rename = "Fungible")]
    Fungible,
    /// Aptos fungible asset.
    #[serde(rename = "FungibleAsset")]
    FungibleAsset,
    /// Solana non-fungible token.
    #[serde(rename = "NonFungible")]
    NonFungible,
    /// Solana non-fungible edition.
    #[serde(rename = "NonFungibleEdition")]
    NonFungibleEdition,
    /// Solana programmable non-fungible token.
    #[serde(rename = "ProgrammableNonFungible")]
    ProgrammableNonFungible,
    /// Solana programmable non-fungible edition.
    #[serde(rename = "ProgrammableNonFungibleEdition")]
    ProgrammableNonFungibleEdition,
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

/// Transaction fee breakdown (all fields optional / chain-dependent).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionFee {
    /// Gas limit (EVM).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_limit: Option<String>,
    /// Gas price in wei (legacy transactions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<String>,
    /// Max fee per gas (EIP-1559).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fee: Option<String>,
    /// Max priority fee per gas (EIP-1559).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_fee: Option<String>,
    /// Base fee per gas (EIP-1559).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_fee: Option<String>,
    /// Total network fee in native currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_fee: Option<String>,
    /// Raw network fee value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_fee_raw: Option<String>,
    /// L1 data fee (Optimism and Base).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l1_fee: Option<String>,
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
    fn blockchain_eth_serializes() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string(&Blockchain::Eth)?;
        assert_eq!(json, "\"ETH\"");
        Ok(())
    }

    #[test]
    fn blockchain_eth_sepolia_round_trips() -> Result<(), Box<dyn std::error::Error>> {
        let serialized = serde_json::to_string(&Blockchain::EthSepolia)?;
        assert_eq!(serialized, "\"ETH-SEPOLIA\"");
        let deserialized: Blockchain = serde_json::from_str(&serialized)?;
        assert_eq!(deserialized, Blockchain::EthSepolia);
        Ok(())
    }

    #[test]
    fn blockchain_sol_devnet_round_trips() -> Result<(), Box<dyn std::error::Error>> {
        let serialized = serde_json::to_string(&Blockchain::SolDevnet)?;
        assert_eq!(serialized, "\"SOL-DEVNET\"");
        let deserialized: Blockchain = serde_json::from_str(&serialized)?;
        assert_eq!(deserialized, Blockchain::SolDevnet);
        Ok(())
    }

    #[test]
    fn custody_type_serializes() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string(&CustodyType::Developer)?;
        assert_eq!(json, "\"DEVELOPER\"");
        Ok(())
    }

    #[test]
    fn token_standard_erc20_round_trips() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string(&TokenStandard::Erc20)?;
        assert_eq!(json, "\"ERC20\"");
        let back: TokenStandard = serde_json::from_str(&json)?;
        assert_eq!(back, TokenStandard::Erc20);
        Ok(())
    }

    #[test]
    fn fee_level_serializes() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string(&FeeLevel::Medium)?;
        assert_eq!(json, "\"MEDIUM\"");
        Ok(())
    }
}
