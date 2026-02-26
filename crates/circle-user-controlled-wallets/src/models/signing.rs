//! Signing resource models for the Circle User-Controlled Wallets API.
//!
//! Contains request types for message and transaction signing endpoints.
//! All signing responses return a `challengeId` — the actual signing is
//! completed on the client side via the Circle Web3 Services mobile SDK.

use serde::{Deserialize, Serialize};

// ── Request bodies ────────────────────────────────────────────────────────────

/// Request body for `signMessage`.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignMessageRequest {
    /// The message to sign (plain text or hex-encoded bytes).
    pub message: String,
    /// ID of the wallet that should sign.
    pub wallet_id: String,
    /// If `true`, `message` is interpreted as a hex-encoded byte string.
    pub encoded_by_hex: Option<bool>,
    /// Optional memo stored alongside the signing request.
    pub memo: Option<String>,
}

/// Request body for `signTypedData`.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignTypedDataRequest {
    /// EIP-712 typed data as a JSON string.
    pub data: String,
    /// ID of the wallet that should sign.
    pub wallet_id: String,
    /// Optional memo stored alongside the signing request.
    pub memo: Option<String>,
}

/// Request body for `signTransaction`.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignTransactionRequest {
    /// ID of the wallet that should sign.
    pub wallet_id: String,
    /// Raw unsigned transaction bytes (hex-encoded).
    pub raw_transaction: Option<String>,
    /// Transaction fields as a JSON string.
    pub transaction: Option<String>,
    /// Optional memo stored alongside the signing request.
    pub memo: Option<String>,
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign_message_request_camel_case() -> Result<(), Box<dyn std::error::Error>> {
        let req = SignMessageRequest {
            message: "hello".to_string(),
            wallet_id: "w1".to_string(),
            encoded_by_hex: Some(false),
            memo: Some("test".to_string()),
        };
        let s = serde_json::to_string(&req)?;
        assert!(s.contains("walletId"), "{s}");
        assert!(s.contains("encodedByHex"), "{s}");
        Ok(())
    }

    #[test]
    fn sign_typed_data_request_round_trip() -> Result<(), Box<dyn std::error::Error>> {
        let req = SignTypedDataRequest {
            data: r#"{"types":{}}"#.to_string(),
            wallet_id: "w2".to_string(),
            memo: None,
        };
        let json = serde_json::to_string(&req)?;
        let decoded: SignTypedDataRequest = serde_json::from_str(&json)?;
        assert_eq!(decoded.wallet_id, "w2");
        Ok(())
    }
}
