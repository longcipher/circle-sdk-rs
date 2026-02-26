//! UserOp (ERC-4337 user operation) request and response types for the Buidl Wallets API.

use super::common::{Blockchain, PageParams};

/// State of a user operation.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserOpState {
    /// User operation has been submitted to the bundler.
    Sent,
    /// User operation is confirmed on-chain but not finalized.
    Confirmed,
    /// User operation has reached finality.
    Complete,
    /// User operation failed.
    Failed,
}

/// Error reason for a failed user operation.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserOpErrorReason {
    /// User operation failed on-chain.
    FailedOnChain,
    /// User operation was replaced by another.
    FailedReplaced,
}

/// Raw ERC-4337 user operation fields.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserOperation {
    /// Encoded call data for the user operation.
    pub call_data: String,
    /// Nonce of the sender's account.
    pub nonce: String,
    /// Sender smart account address.
    pub sender: String,
    /// Gas limit for the `callData` execution phase.
    pub call_gas_limit: Option<String>,
    /// Factory contract address (counterfactual deployment).
    pub factory: Option<String>,
    /// Encoded factory initialization data.
    pub factory_data: Option<String>,
    /// Maximum fee per gas unit (EIP-1559).
    pub max_fee_per_gas: Option<String>,
    /// Maximum priority fee per gas unit.
    pub max_priority_fee_per_gas: Option<String>,
    /// Paymaster contract address.
    pub paymaster: Option<String>,
    /// Encoded paymaster data (v0.6).
    pub paymaster_and_data: Option<String>,
    /// Encoded paymaster data (v0.7).
    pub paymaster_data: Option<String>,
    /// Gas limit for paymaster post-operation (v0.7).
    pub paymaster_post_op_gas_limit: Option<String>,
    /// Gas limit for paymaster verification (v0.7).
    pub paymaster_verification_gas_limit: Option<String>,
    /// Gas spent outside of the main execution phase.
    pub pre_verification_gas: Option<String>,
    /// Cryptographic signature.
    pub signature: Option<String>,
    /// Gas limit for the verification phase.
    pub verification_gas_limit: Option<String>,
}

/// A single ERC-4337 user operation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserOp {
    /// System-generated unique identifier (UUID).
    pub id: String,
    /// Blockchain network.
    pub blockchain: Blockchain,
    /// Current state.
    pub state: UserOpState,
    /// EIP-4337 user operation hash.
    pub user_op_hash: String,
    /// Raw user operation data.
    pub user_operation: UserOperation,
    /// Optional caller-supplied reference identifier.
    pub ref_id: Option<String>,
    /// Actual gas cost incurred on-chain.
    pub actual_gas_cost: Option<String>,
    /// Actual gas used on-chain.
    pub actual_gas_used: Option<String>,
    /// Date the block was mined (ISO-8601).
    pub block_date: Option<String>,
    /// Block hash.
    pub block_hash: Option<String>,
    /// Block height.
    pub block_height: Option<i64>,
    /// Error reason (only when `state` is `Failed`).
    pub error_reason: Option<UserOpErrorReason>,
    /// On-chain revert reason (if any).
    pub revert_reason: Option<String>,
    /// Address the user operation executed against.
    pub to: Option<String>,
    /// Transaction hash.
    pub tx_hash: Option<String>,
    /// Creation timestamp (ISO-8601).
    pub create_date: Option<String>,
    /// Last-update timestamp (ISO-8601).
    pub update_date: Option<String>,
}

/// Inner data of the `listUserOps` response.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserOpsData {
    /// User operations matching the query filters.
    pub user_operations: Vec<UserOp>,
}

/// Response wrapper for the `listUserOps` endpoint.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserOps {
    /// Response data.
    pub data: UserOpsData,
}

/// Inner data of the `getUserOp` response.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserOpIdData {
    /// The retrieved user operation.
    pub user_operation: UserOp,
}

/// Response wrapper for the `getUserOp` endpoint.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserOpId {
    /// Response data.
    pub data: UserOpIdData,
}

/// Query parameters for [`crate::BuidlWalletsClient::list_user_ops`].
#[derive(Debug, Default, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListUserOpsParams {
    /// Filter by blockchain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockchain: Option<Blockchain>,
    /// Filter by optional reference ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
    /// Filter by sender addresses (comma-separated).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub senders: Option<String>,
    /// Filter by user operation state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<UserOpState>,
    /// Filter by transaction hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    /// Filter by user operation hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_op_hash: Option<String>,
    /// Pagination parameters.
    #[serde(flatten)]
    pub page: PageParams,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_op_state_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
        let s = serde_json::to_string(&UserOpState::Complete)?;
        assert_eq!(s, "\"COMPLETE\"");
        let parsed: UserOpState = serde_json::from_str("\"FAILED\"")?;
        assert_eq!(parsed, UserOpState::Failed);
        Ok(())
    }

    #[test]
    fn user_ops_response_deserializes() -> Result<(), Box<dyn std::error::Error>> {
        let json = r#"{
            "data": {
                "userOperations": [
                    {
                        "id": "c4d1da72-111e-4d52-bdbf-2e74a2d803d5",
                        "blockchain": "ETH-SEPOLIA",
                        "state": "SENT",
                        "userOpHash": "0x54d3cccda6ffa503bc1e554937fe67818b6ca1a5a05c7e66ebfa32bf27520152",
                        "userOperation": {
                            "callData": "0xdeadbeef",
                            "nonce": "1",
                            "sender": "0x4b6c0b0078b63f881503e7fd3a9a1061065db242"
                        }
                    }
                ]
            }
        }"#;
        let resp: UserOps = serde_json::from_str(json)?;
        assert_eq!(resp.data.user_operations.len(), 1);
        assert_eq!(resp.data.user_operations[0].state, UserOpState::Sent);
        Ok(())
    }
}
