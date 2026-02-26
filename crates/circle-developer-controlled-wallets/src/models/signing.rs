//! Signing resource models for the Circle Developer-Controlled Wallets API.
//!
//! Contains request parameters and response types for message and transaction
//! signing endpoints.

use super::common::Blockchain;

/// Request body for signing a plain or hex-encoded message.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignMessageRequest {
    /// Source wallet ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_id: Option<String>,
    /// Blockchain network (required when wallet_id is absent).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockchain: Option<Blockchain>,
    /// Wallet address (required when wallet_id is absent).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_address: Option<String>,
    /// Message to sign (UTF-8 string or hex string when encoded_by_hex is true).
    pub message: String,
    /// When true, `message` is treated as a hex-encoded byte array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoded_by_hex: Option<bool>,
    /// Optional memo for record-keeping.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    /// Encrypted entity secret ciphertext.
    pub entity_secret_ciphertext: String,
}

/// Inner data of a sign-message response.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignatureData {
    /// Hex-encoded signature.
    pub signature: String,
}

/// Response wrapper for sign-message and sign-typed-data endpoints.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct SignatureResponse {
    /// Response data.
    pub data: SignatureData,
}

/// Request body for signing an EIP-712 typed data payload.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignTypedDataRequest {
    /// Source wallet ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_id: Option<String>,
    /// Blockchain network (required when wallet_id is absent).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockchain: Option<Blockchain>,
    /// Wallet address (required when wallet_id is absent).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_address: Option<String>,
    /// JSON-encoded EIP-712 typed data object.
    pub typed_data: String,
    /// Encrypted entity secret ciphertext.
    pub entity_secret_ciphertext: String,
}

/// Request body for signing a raw transaction.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignTransactionRequest {
    /// Source wallet ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_id: Option<String>,
    /// Blockchain network (required when wallet_id is absent).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockchain: Option<Blockchain>,
    /// Wallet address (required when wallet_id is absent).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_address: Option<String>,
    /// Hex-encoded raw unsigned transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_transaction: Option<String>,
    /// Structured transaction object (alternative to raw_transaction).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<serde_json::Value>,
    /// Encrypted entity secret ciphertext.
    pub entity_secret_ciphertext: String,
}

/// Inner data of a sign-transaction response.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignTransactionData {
    /// Hex-encoded signature.
    pub signature: String,
    /// Hex-encoded signed transaction.
    pub signed_transaction: String,
    /// Transaction hash (available after broadcast).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
}

/// Response wrapper for the sign-transaction endpoint.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct SignTransactionResponse {
    /// Response data.
    pub data: SignTransactionData,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signature_response_deserializes() -> Result<(), Box<dyn std::error::Error>> {
        let json = r#"{"data": {"signature": "0xabcdef1234"}}"#;
        let resp: SignatureResponse = serde_json::from_str(json)?;
        assert_eq!(resp.data.signature, "0xabcdef1234");
        Ok(())
    }

    #[test]
    fn sign_transaction_response_deserializes() -> Result<(), Box<dyn std::error::Error>> {
        let json = r#"{
            "data": {
                "signature": "0xsig",
                "signedTransaction": "0xsigned",
                "txHash": "0xhash"
            }
        }"#;
        let resp: SignTransactionResponse = serde_json::from_str(json)?;
        assert_eq!(resp.data.signature, "0xsig");
        assert_eq!(resp.data.signed_transaction, "0xsigned");
        assert_eq!(resp.data.tx_hash.as_deref(), Some("0xhash"));
        Ok(())
    }

    #[test]
    fn sign_message_request_serializes() -> Result<(), Box<dyn std::error::Error>> {
        let req = SignMessageRequest {
            wallet_id: Some("wallet-1".to_string()),
            blockchain: None,
            wallet_address: None,
            message: "Hello, World!".to_string(),
            encoded_by_hex: None,
            memo: None,
            entity_secret_ciphertext: "cipher".to_string(),
        };
        let json = serde_json::to_string(&req)?;
        assert!(json.contains("walletId"));
        assert!(json.contains("entitySecretCiphertext"));
        assert!(json.contains("Hello, World!"));
        Ok(())
    }

    #[test]
    fn sign_typed_data_request_serializes() -> Result<(), Box<dyn std::error::Error>> {
        let req = SignTypedDataRequest {
            wallet_id: Some("wallet-1".to_string()),
            blockchain: None,
            wallet_address: None,
            typed_data: r#"{"types":{}}"#.to_string(),
            entity_secret_ciphertext: "cipher".to_string(),
        };
        let json = serde_json::to_string(&req)?;
        assert!(json.contains("typedData"));
        Ok(())
    }
}
