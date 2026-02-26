//! Wallet set resource models for the Circle Developer-Controlled Wallets API.
//!
//! Contains request parameters and response types for wallet set management
//! endpoints.

use super::common::{CustodyType, PageParams};

/// A wallet set resource.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletSet {
    /// Unique wallet set ID (UUID).
    pub id: String,
    /// Custody type for wallets within this set.
    pub custody_type: CustodyType,
    /// ISO-8601 creation timestamp.
    pub create_date: String,
    /// ISO-8601 last-update timestamp.
    pub update_date: String,
    /// Human-readable name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User ID associated with this wallet set (user-controlled wallets).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// Inner data of a list-wallet-sets response.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletSetsData {
    /// Wallet sets matching the query.
    pub wallet_sets: Vec<WalletSet>,
}

/// Response wrapper for the list-wallet-sets endpoint.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WalletSets {
    /// Response data.
    pub data: WalletSetsData,
}

/// Inner data of a single wallet-set response.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletSetData {
    /// The wallet set.
    pub wallet_set: WalletSet,
}

/// Response wrapper for create/get/update wallet-set endpoints.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WalletSetResponse {
    /// Response data.
    pub data: WalletSetData,
}

/// Request body for creating a new wallet set.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWalletSetRequest {
    /// Encrypted entity secret ciphertext.
    pub entity_secret_ciphertext: String,
    /// Idempotency key (UUID) to deduplicate requests.
    pub idempotency_key: String,
    /// Optional human-readable name for the wallet set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Request body for updating a wallet set.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWalletSetRequest {
    /// New human-readable name for the wallet set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Query parameters for the list-wallet-sets endpoint.
#[derive(Debug, Default, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWalletSetsParams {
    /// Pagination parameters.
    #[serde(flatten)]
    pub page: PageParams,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wallet_set_response_deserializes() -> Result<(), Box<dyn std::error::Error>> {
        let json = r#"{
            "data": {
                "walletSet": {
                    "id": "f5d71c75-b8e3-4d1e-ae38-e8c3c3c3c3c3",
                    "custodyType": "DEVELOPER",
                    "createDate": "2024-01-01T00:00:00Z",
                    "updateDate": "2024-01-01T00:00:00Z",
                    "name": "My Wallet Set"
                }
            }
        }"#;
        let resp: WalletSetResponse = serde_json::from_str(json)?;
        assert_eq!(resp.data.wallet_set.id, "f5d71c75-b8e3-4d1e-ae38-e8c3c3c3c3c3");
        assert_eq!(resp.data.wallet_set.name.as_deref(), Some("My Wallet Set"));
        Ok(())
    }

    #[test]
    fn wallet_sets_response_deserializes() -> Result<(), Box<dyn std::error::Error>> {
        let json = r#"{
            "data": {
                "walletSets": [
                    {
                        "id": "aaa",
                        "custodyType": "DEVELOPER",
                        "createDate": "2024-01-01T00:00:00Z",
                        "updateDate": "2024-01-01T00:00:00Z"
                    }
                ]
            }
        }"#;
        let resp: WalletSets = serde_json::from_str(json)?;
        assert_eq!(resp.data.wallet_sets.len(), 1);
        assert_eq!(resp.data.wallet_sets[0].id, "aaa");
        Ok(())
    }

    #[test]
    fn create_wallet_set_request_serializes() -> Result<(), Box<dyn std::error::Error>> {
        let req = CreateWalletSetRequest {
            entity_secret_ciphertext: "cipher123".to_string(),
            idempotency_key: "key123".to_string(),
            name: Some("Test".to_string()),
        };
        let json = serde_json::to_string(&req)?;
        assert!(json.contains("entitySecretCiphertext"));
        assert!(json.contains("idempotencyKey"));
        Ok(())
    }
}
