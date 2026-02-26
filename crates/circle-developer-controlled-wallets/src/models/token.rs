//! Token resource models for the Circle Developer-Controlled Wallets API.
//!
//! Contains response types for the token lookup endpoint.
//! The [`Token`] struct itself is defined in [`crate::models::wallet`].

use super::wallet::Token;

/// Inner data of a get-token response.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenData {
    /// The token resource.
    pub token: Token,
}

/// Response wrapper for the get-token endpoint.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TokenResponse {
    /// Response data.
    pub data: TokenData,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::common::Blockchain;

    #[test]
    fn token_response_deserializes() -> Result<(), Box<dyn std::error::Error>> {
        let json = r#"{
            "data": {
                "token": {
                    "id": "token-id-1",
                    "blockchain": "ETH",
                    "isNative": false,
                    "name": "USD Coin",
                    "symbol": "USDC",
                    "decimals": 6,
                    "tokenAddress": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
                    "updateDate": "2024-01-01T00:00:00Z",
                    "createDate": "2024-01-01T00:00:00Z"
                }
            }
        }"#;
        let resp: TokenResponse = serde_json::from_str(json)?;
        assert_eq!(resp.data.token.blockchain, Blockchain::Eth);
        assert_eq!(resp.data.token.symbol.as_deref(), Some("USDC"));
        assert_eq!(resp.data.token.decimals, Some(6));
        Ok(())
    }

    #[test]
    fn token_response_native_deserializes() -> Result<(), Box<dyn std::error::Error>> {
        let json = r#"{
            "data": {
                "token": {
                    "blockchain": "MATIC",
                    "isNative": true,
                    "name": "MATIC",
                    "symbol": "MATIC",
                    "decimals": 18
                }
            }
        }"#;
        let resp: TokenResponse = serde_json::from_str(json)?;
        assert!(resp.data.token.is_native);
        assert_eq!(resp.data.token.blockchain, Blockchain::Matic);
        Ok(())
    }
}
