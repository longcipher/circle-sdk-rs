//! Wallet resource models for the Circle Buidl Wallets API.
//!
//! Covers token, balance, NFT, and wallet balance/NFT list-endpoint types.

use super::common::{Blockchain, PageParams};

/// Token standard (fungible tokens).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FtStandard {
    /// Native coin (no ERC standard).
    #[serde(rename = "")]
    Native,
    /// ERC-20 fungible token.
    Erc20,
}

/// Token standard (used in combined ERC-20/721/1155 contexts).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TokenStandard {
    /// ERC-20 fungible token.
    Erc20,
    /// ERC-721 non-fungible token.
    Erc721,
    /// ERC-1155 multi-token.
    Erc1155,
}

/// NFT token standard.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NftStandard {
    /// ERC-721 non-fungible token.
    Erc721,
    /// ERC-1155 multi-token.
    Erc1155,
}

/// A blockchain token definition.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    /// Blockchain network the token lives on.
    pub blockchain: Blockchain,
    /// Whether this token is the native coin of its chain.
    pub is_native: bool,
    /// Token name (e.g. `"USD Coin"`).
    pub name: Option<String>,
    /// Token standard.
    pub standard: Option<TokenStandard>,
    /// Number of decimal places.
    pub decimals: Option<i32>,
    /// Chain symbol (e.g. `"USDC"`).
    pub symbol: Option<String>,
    /// Contract address (absent for native coins).
    pub token_address: Option<String>,
}

/// A single fungible token balance entry.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    /// Token amount as a decimal string.
    pub amount: String,
    /// Token definition.
    pub token: Token,
    /// Last-update timestamp (ISO-8601).
    pub update_date: String,
}

/// Inner data of a `listWalletBalancesBy*` response.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalancesData {
    /// Token balances matching the query filters.
    pub token_balances: Vec<Balance>,
}

/// Response wrapper for `listWalletBalancesById` and `listWalletBalancesByBlockchainAddress`.
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
    /// Last-update timestamp (ISO-8601).
    pub update_date: String,
    /// On-chain token ID.
    pub nft_token_id: Option<String>,
    /// IPFS or HTTP URI of the NFT metadata.
    pub metadata: Option<String>,
}

/// Inner data of a `listWalletNFTsBy*` response.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NftsData {
    /// NFTs matching the query filters.
    pub nfts: Vec<Nft>,
}

/// Response wrapper for `listWalletNFTsById` and `listWalletNFTsByBlockchainAddress`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Nfts {
    /// Response data.
    pub data: NftsData,
}

/// Query parameters for balance list endpoints that use a wallet UUID.
#[derive(Debug, Default, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWalletBalancesParams {
    /// Filter by fungible token standard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<FtStandard>,
    /// Filter by token name substring.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Filter by token contract address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_address: Option<String>,
    /// Pagination parameters.
    #[serde(flatten)]
    pub page: PageParams,
}

/// Query parameters for NFT list endpoints that use a wallet UUID.
#[derive(Debug, Default, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWalletNftsParams {
    /// Filter by NFT standard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<NftStandard>,
    /// Filter by token name substring.
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
    fn balances_response_deserializes() -> Result<(), Box<dyn std::error::Error>> {
        let json = r#"{
            "data": {
                "tokenBalances": [
                    {
                        "amount": "6.62607015",
                        "token": {
                            "blockchain": "ETH",
                            "isNative": true
                        },
                        "updateDate": "2023-01-01T12:04:05Z"
                    }
                ]
            }
        }"#;
        let resp: Balances = serde_json::from_str(json)?;
        assert_eq!(resp.data.token_balances.len(), 1);
        assert!(resp.data.token_balances[0].token.is_native);
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
                        "updateDate": "2023-01-01T12:04:05Z",
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
}
