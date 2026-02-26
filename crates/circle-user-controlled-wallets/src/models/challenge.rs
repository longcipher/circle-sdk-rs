//! Challenge resource models for the Circle User-Controlled Wallets API.
//!
//! Contains request parameters and response types for PIN challenge and
//! security question endpoints, as well as the challenge-ID response used
//! by most write operations.

use serde::{Deserialize, Serialize};

use super::{
    common::{AccountType, Blockchain},
    wallet::WalletMetadata,
};

// ── Enums ─────────────────────────────────────────────────────────────────────

/// Type of a PIN or device challenge.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChallengeType {
    /// Initial user setup challenge.
    Initialize,
    /// Challenge for setting a PIN.
    SetPin,
    /// Challenge for changing a PIN.
    ChangePin,
    /// Challenge for setting security questions.
    SetSecurityQuestions,
    /// Challenge for creating a wallet.
    CreateWallet,
    /// Challenge for restoring a locked PIN.
    RestorePin,
    /// Challenge for creating a transaction.
    CreateTransaction,
    /// Challenge for accelerating a stuck transaction.
    AccelerateTransaction,
    /// Challenge for cancelling a transaction.
    CancelTransaction,
    /// Challenge for a smart-contract execution transaction.
    ContractExecution,
    /// Challenge for upgrading a wallet's SCA core.
    WalletUpgrade,
    /// Challenge for signing a message.
    SignMessage,
    /// Challenge for signing EIP-712 typed data.
    SignTypeddata,
    /// Challenge for signing a raw transaction.
    SignTransaction,
}

/// Status of a challenge.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChallengeStatus {
    /// Not yet started.
    Pending,
    /// Currently being processed by the mobile SDK.
    InProgress,
    /// Successfully completed.
    Complete,
    /// Finished with an error.
    Failed,
    /// Challenge has passed its expiry time.
    Expired,
}

// ── Models ────────────────────────────────────────────────────────────────────

/// A challenge record returned by the Circle API.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Challenge {
    /// Unique challenge identifier.
    pub id: String,
    /// Type of this challenge.
    #[serde(rename = "type")]
    pub challenge_type: ChallengeType,
    /// Current status.
    pub status: ChallengeStatus,
    /// IDs of resources created or modified by this challenge outcome.
    pub correlation_ids: Option<Vec<String>>,
    /// Machine-readable error code (present on failure).
    pub error_code: Option<i32>,
    /// Human-readable error message (present on failure).
    pub error_message: Option<String>,
}

/// `data` payload wrapping a list of challenges.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChallengesData {
    /// List of challenges.
    pub challenges: Vec<Challenge>,
}

/// Response envelope for list-challenges.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Challenges {
    /// Paginated challenges.
    pub data: ChallengesData,
}

/// `data` payload wrapping a single challenge.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeData {
    /// The challenge record.
    pub challenge: Challenge,
}

/// Response envelope for a single challenge lookup.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeResponse {
    /// Challenge data.
    pub data: ChallengeData,
}

/// `data` payload returned by most write operations.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeIdData {
    /// UUID of the newly created challenge.
    pub challenge_id: String,
}

/// Response envelope for operations that return a challenge ID.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeIdResponse {
    /// Challenge ID data.
    pub data: ChallengeIdData,
}

// ── Request bodies ────────────────────────────────────────────────────────────

/// Request body for `initializeUser` (sets PIN and optionally creates wallets).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetPinAndInitWalletRequest {
    /// Client-generated idempotency key (UUID).
    pub idempotency_key: String,
    /// Account type for newly created wallets.
    pub account_type: Option<AccountType>,
    /// Blockchains on which to create wallets.
    pub blockchains: Option<Vec<Blockchain>>,
    /// Optional per-wallet metadata.
    pub metadata: Option<Vec<WalletMetadata>>,
}

/// Request body for PIN set/change/restore operations.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetPinRequest {
    /// Client-generated idempotency key (UUID).
    pub idempotency_key: String,
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_type_screaming() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(serde_json::to_string(&ChallengeType::CreateWallet)?, "\"CREATE_WALLET\"");
        assert_eq!(serde_json::to_string(&ChallengeType::SignTypeddata)?, "\"SIGN_TYPEDDATA\"");
        Ok(())
    }

    #[test]
    fn challenge_status_screaming() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(serde_json::to_string(&ChallengeStatus::Complete)?, "\"COMPLETE\"");
        assert_eq!(serde_json::to_string(&ChallengeStatus::InProgress)?, "\"IN_PROGRESS\"");
        Ok(())
    }

    #[test]
    fn challenge_id_response_round_trip() -> Result<(), Box<dyn std::error::Error>> {
        let json = r#"{"data":{"challengeId":"abc-123"}}"#;
        let resp: ChallengeIdResponse = serde_json::from_str(json)?;
        assert_eq!(resp.data.challenge_id, "abc-123");
        Ok(())
    }

    #[test]
    fn challenge_type_field_renamed_to_type() -> Result<(), Box<dyn std::error::Error>> {
        let c = Challenge {
            id: "c1".to_string(),
            challenge_type: ChallengeType::SetPin,
            status: ChallengeStatus::Pending,
            correlation_ids: None,
            error_code: None,
            error_message: None,
        };
        let s = serde_json::to_string(&c)?;
        assert!(s.contains("\"type\""), "expected type key in {s}");
        Ok(())
    }
}
