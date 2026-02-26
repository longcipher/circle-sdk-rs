//! Transaction resource models for the Circle Developer-Controlled Wallets API.
//!
//! Contains request parameters and response types for transaction management
//! endpoints including transfers, contract execution, signing, and fee estimation.

use super::common::{Blockchain, CustodyType, FeeLevel, TransactionFee};

/// Transaction lifecycle state.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionState {
    /// Transaction was cancelled.
    Cancelled,
    /// Transaction has received required confirmations.
    Confirmed,
    /// Transaction is complete.
    Complete,
    /// Transaction was denied by screening.
    Denied,
    /// Transaction failed on-chain.
    Failed,
    /// Transaction was just initiated.
    Initiated,
    /// Transaction cleared pre-chain checks.
    Cleared,
    /// Transaction is queued for broadcast.
    Queued,
    /// Transaction was sent to the network.
    Sent,
    /// Transaction is stuck (e.g. low gas).
    Stuck,
}

/// Transaction directional type.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionType {
    /// Incoming transaction.
    Inbound,
    /// Outgoing transaction.
    Outbound,
}

/// The operation performed by the transaction.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Operation {
    /// Token or native asset transfer.
    Transfer,
    /// Smart contract function call.
    ContractExecution,
    /// Smart contract deployment.
    ContractDeployment,
}

/// Risk score label.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RiskScore {
    /// Risk is unknown.
    Unknown,
    /// Low risk.
    Low,
    /// Medium risk.
    Medium,
    /// High risk.
    High,
    /// Severe risk.
    Severe,
    /// Address is on a blocklist.
    Blocklist,
}

/// Risk category label.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RiskCategory {
    /// Sanctions-related risk.
    Sanctions,
    /// Child sexual abuse material.
    Csam,
    /// Illicit behavior.
    IllicitBehavior,
    /// Gambling.
    Gambling,
    /// Terrorist financing.
    TerroristFinancing,
    /// Unsupported.
    Unsupported,
    /// Frozen address.
    Frozen,
    /// Other risk.
    Other,
    /// High-risk industry.
    HighRiskIndustry,
    /// Politically exposed person.
    Pep,
    /// Trusted entity.
    Trusted,
    /// Hacking.
    Hacking,
    /// Human trafficking.
    HumanTrafficking,
    /// Special measures.
    SpecialMeasures,
}

/// Risk exposure type.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RiskType {
    /// Direct ownership.
    Ownership,
    /// Counterparty relationship.
    Counterparty,
    /// Indirect relationship.
    Indirect,
}

/// Recommended action for a screened transaction.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RiskAction {
    /// Approve the transaction.
    Approve,
    /// Flag for manual review.
    Review,
    /// Freeze the wallet.
    FreezeWallet,
    /// Deny the transaction.
    Deny,
}

/// An individual risk signal from the screening evaluation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskSignal {
    /// Risk signal source (e.g. screening provider name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Source-specific value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_value: Option<String>,
    /// Risk score for this signal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_score: Option<RiskScore>,
    /// Risk categories for this signal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_categories: Option<Vec<RiskCategory>>,
    /// Risk exposure type.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_type: Option<RiskType>,
}

/// Transaction screening and compliance decision.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionScreeningDecision {
    /// ISO-8601 timestamp when screening occurred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screening_date: Option<String>,
    /// Name of the compliance rule that triggered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    /// Recommended actions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<RiskAction>>,
    /// Reasons for the decision (risk signals).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasons: Option<Vec<RiskSignal>>,
}

/// A developer-controlled transaction resource.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    /// Unique transaction ID.
    pub id: String,
    /// Current lifecycle state.
    pub state: TransactionState,
    /// Blockchain network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockchain: Option<Blockchain>,
    /// Transaction direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<TransactionType>,
    /// ISO-8601 creation timestamp.
    pub create_date: String,
    /// ISO-8601 last-update timestamp.
    pub update_date: String,
    /// ABI function signature (contract executions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abi_function_signature: Option<String>,
    /// ABI parameters (contract executions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abi_parameters: Option<Vec<serde_json::Value>>,
    /// Token amounts being transferred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amounts: Option<Vec<String>>,
    /// Transfer amount in USD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_in_usd: Option<String>,
    /// Block hash of the confirming block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<String>,
    /// Block height of the confirming block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_height: Option<u64>,
    /// Contract address (for contract executions/deployments).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_address: Option<String>,
    /// Custody type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custody_type: Option<CustodyType>,
    /// Destination wallet address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_address: Option<String>,
    /// Human-readable error reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_reason: Option<String>,
    /// Detailed error description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<String>,
    /// Fee estimate used at submission time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_fee: Option<TransactionFee>,
    /// Fee priority level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_level: Option<FeeLevel>,
    /// ISO-8601 first confirmation timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_confirm_date: Option<String>,
    /// Actual network fee paid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_fee: Option<String>,
    /// Network fee denominated in USD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_fee_in_usd: Option<String>,
    /// NFT token IDs being transferred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfts: Option<Vec<String>>,
    /// Operation type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
    /// External reference ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
    /// Source wallet address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_address: Option<String>,
    /// Token ID for the asset being transferred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    /// On-chain transaction hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    /// User ID (user-controlled wallets).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Source wallet ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_id: Option<String>,
    /// Compliance screening evaluation result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_screening_evaluation: Option<TransactionScreeningDecision>,
}

/// Inner data of a list-transactions response.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionsData {
    /// Transactions matching the query.
    pub transactions: Vec<Transaction>,
}

/// Response wrapper for the list-transactions endpoint.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Transactions {
    /// Response data.
    pub data: TransactionsData,
}

/// Inner data of a single transaction response.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionData {
    /// The transaction.
    pub transaction: Transaction,
}

/// Response wrapper for get/create/cancel/accelerate transaction endpoints.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TransactionResponse {
    /// Response data.
    pub data: TransactionData,
}

/// Query parameters for the list-transactions endpoint.
#[derive(Debug, Default, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTransactionsParams {
    /// Filter by blockchain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockchain: Option<Blockchain>,
    /// Filter by custody type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custody_type: Option<CustodyType>,
    /// Filter by destination address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_address: Option<String>,
    /// Include all transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_all: Option<bool>,
    /// Filter by operation type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
    /// Filter by external reference ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
    /// Filter by source address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_address: Option<String>,
    /// Filter by state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<TransactionState>,
    /// Filter by token contract address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_address: Option<String>,
    /// Filter by transaction hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_hash: Option<String>,
    /// Filter by transaction type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<TransactionType>,
    /// Filter by specific wallet IDs (comma-separated).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_ids: Option<String>,
    /// Start of date-time range (ISO-8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// End of date-time range (ISO-8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    /// Cursor to page before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_before: Option<String>,
    /// Cursor to page after.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_after: Option<String>,
    /// Page size (1–50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
}

/// Request body for creating a developer-controlled transfer transaction.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransferTxRequest {
    /// Idempotency key (UUID).
    pub idempotency_key: String,
    /// Encrypted entity secret ciphertext.
    pub entity_secret_ciphertext: String,
    /// Source wallet ID.
    pub wallet_id: String,
    /// Blockchain (required for native asset transfers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockchain: Option<Blockchain>,
    /// Token ID for the asset being transferred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    /// Destination address.
    pub destination_address: String,
    /// Token amounts to transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amounts: Option<Vec<String>>,
    /// NFT token IDs to transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nft_token_ids: Option<Vec<String>>,
    /// External reference ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
    /// Fee priority level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_level: Option<FeeLevel>,
    /// Custom gas limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_limit: Option<String>,
    /// Custom gas price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<String>,
    /// Max fee per gas (EIP-1559).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fee: Option<String>,
    /// Max priority fee per gas (EIP-1559).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_fee: Option<String>,
}

/// Request body for creating a contract execution transaction.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateContractExecutionTxRequest {
    /// Idempotency key (UUID).
    pub idempotency_key: String,
    /// Encrypted entity secret ciphertext.
    pub entity_secret_ciphertext: String,
    /// Source wallet ID.
    pub wallet_id: String,
    /// Blockchain network for the call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockchain: Option<Blockchain>,
    /// Contract address to call.
    pub contract_address: String,
    /// ABI function signature (e.g. `transfer(address,uint256)`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abi_function_signature: Option<String>,
    /// ABI-encoded parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abi_parameters: Option<Vec<serde_json::Value>>,
    /// Raw call data (alternative to abi_function_signature + abi_parameters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_data: Option<String>,
    /// Fee priority level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_level: Option<FeeLevel>,
    /// Custom gas limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_limit: Option<String>,
    /// Max fee per gas (EIP-1559).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fee: Option<String>,
    /// Max priority fee per gas (EIP-1559).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_fee: Option<String>,
    /// External reference ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
    /// ETH value to send with the call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
}

/// Request body for cancelling a transaction.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelTxRequest {
    /// Encrypted entity secret ciphertext.
    pub entity_secret_ciphertext: String,
}

/// Request body for accelerating a transaction.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccelerateTxRequest {
    /// Encrypted entity secret ciphertext.
    pub entity_secret_ciphertext: String,
}

/// Request body for validating a blockchain address.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateAddressRequest {
    /// Blockchain to validate against.
    pub blockchain: Blockchain,
    /// Address string to validate.
    pub address: String,
}

/// Inner data of the validate-address response.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateAddressData {
    /// Whether the address is valid for the given blockchain.
    pub is_valid: bool,
}

/// Response wrapper for the validate-address endpoint.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ValidateAddressResponse {
    /// Response data.
    pub data: ValidateAddressData,
}

/// Request body for estimating transfer fees.
#[derive(Debug, Clone, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EstimateTransferFeeRequest {
    /// Source wallet address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_address: Option<String>,
    /// Blockchain network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockchain: Option<Blockchain>,
    /// Destination address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_address: Option<String>,
    /// Token amounts to estimate fees for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amounts: Option<Vec<String>>,
    /// NFT token IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfts: Option<Vec<String>>,
    /// Token ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    /// Fee priority level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_level: Option<FeeLevel>,
    /// Custom gas limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_limit: Option<String>,
    /// Custom gas price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<String>,
}

/// Fee estimate breakdown for low, medium, and high priority.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EstimateFeeData {
    /// Low-priority fee estimate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low: Option<TransactionFee>,
    /// Medium-priority fee estimate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium: Option<TransactionFee>,
    /// High-priority fee estimate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high: Option<TransactionFee>,
    /// Call gas limit for SCA transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_gas_limit: Option<String>,
    /// Verification gas limit for SCA transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_gas_limit: Option<String>,
    /// Pre-verification gas for SCA transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_verification_gas: Option<String>,
}

/// Response wrapper for fee estimation endpoints.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct EstimateFeeResponse {
    /// Response data.
    pub data: EstimateFeeData,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transaction_response_deserializes() -> Result<(), Box<dyn std::error::Error>> {
        let json = r#"{
            "data": {
                "transaction": {
                    "id": "tx-id-1",
                    "state": "COMPLETE",
                    "blockchain": "ETH",
                    "createDate": "2024-01-01T00:00:00Z",
                    "updateDate": "2024-01-02T00:00:00Z",
                    "txHash": "0xdeadbeef",
                    "operation": "TRANSFER"
                }
            }
        }"#;
        let resp: TransactionResponse = serde_json::from_str(json)?;
        assert_eq!(resp.data.transaction.id, "tx-id-1");
        assert_eq!(resp.data.transaction.state, TransactionState::Complete);
        assert_eq!(resp.data.transaction.tx_hash.as_deref(), Some("0xdeadbeef"));
        Ok(())
    }

    #[test]
    fn transactions_list_deserializes() -> Result<(), Box<dyn std::error::Error>> {
        let json = r#"{
            "data": {
                "transactions": [
                    {
                        "id": "tx-1",
                        "state": "SENT",
                        "createDate": "2024-01-01T00:00:00Z",
                        "updateDate": "2024-01-01T00:00:00Z"
                    }
                ]
            }
        }"#;
        let resp: Transactions = serde_json::from_str(json)?;
        assert_eq!(resp.data.transactions.len(), 1);
        assert_eq!(resp.data.transactions[0].state, TransactionState::Sent);
        Ok(())
    }

    #[test]
    fn validate_address_response_deserializes() -> Result<(), Box<dyn std::error::Error>> {
        let json = r#"{"data": {"isValid": true}}"#;
        let resp: ValidateAddressResponse = serde_json::from_str(json)?;
        assert!(resp.data.is_valid);
        Ok(())
    }

    #[test]
    fn create_transfer_request_serializes() -> Result<(), Box<dyn std::error::Error>> {
        let req = CreateTransferTxRequest {
            idempotency_key: "key".to_string(),
            entity_secret_ciphertext: "cipher".to_string(),
            wallet_id: "wallet-1".to_string(),
            blockchain: None,
            token_id: Some("token-1".to_string()),
            destination_address: "0xdest".to_string(),
            amounts: Some(vec!["1.0".to_string()]),
            nft_token_ids: None,
            ref_id: None,
            fee_level: Some(FeeLevel::Medium),
            gas_limit: None,
            gas_price: None,
            max_fee: None,
            priority_fee: None,
        };
        let json = serde_json::to_string(&req)?;
        assert!(json.contains("walletId"));
        assert!(json.contains("destinationAddress"));
        assert!(json.contains("MEDIUM"));
        Ok(())
    }

    #[test]
    fn transaction_state_all_variants_deserialize() -> Result<(), Box<dyn std::error::Error>> {
        for (s, expected) in [
            ("\"CANCELLED\"", TransactionState::Cancelled),
            ("\"CONFIRMED\"", TransactionState::Confirmed),
            ("\"COMPLETE\"", TransactionState::Complete),
            ("\"INITIATED\"", TransactionState::Initiated),
            ("\"STUCK\"", TransactionState::Stuck),
        ] {
            let state: TransactionState = serde_json::from_str(s)?;
            assert_eq!(state, expected);
        }
        Ok(())
    }
}
