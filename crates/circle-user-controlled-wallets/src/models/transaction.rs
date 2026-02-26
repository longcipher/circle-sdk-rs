//! Transaction resource models for the Circle User-Controlled Wallets API.
//!
//! Contains request parameters and response types for transaction management
//! endpoints, including estimation, acceleration, and cancellation.

use serde::{Deserialize, Serialize};

use super::{
    common::{Blockchain, CustodyType, FeeLevel, PageParams, TransactionFee},
    wallet::Nft,
};

// ── State / type enums ────────────────────────────────────────────────────────

/// Current on-chain state of a transaction.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionState {
    /// Transaction was cancelled before broadcast.
    Cancelled,
    /// At least one confirmation received.
    Confirmed,
    /// Transaction is finalized.
    Complete,
    /// Transaction was denied by a compliance rule.
    Denied,
    /// Transaction failed on-chain.
    Failed,
    /// Submission initiated internally.
    Initiated,
    /// Mempool cleared.
    Cleared,
    /// Waiting in the internal queue.
    Queued,
    /// Broadcast to the network.
    Sent,
    /// Stuck in the mempool with insufficient gas.
    Stuck,
}

/// Direction of a transaction relative to the wallet.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionType {
    /// Incoming transaction.
    Inbound,
    /// Outgoing transaction.
    Outbound,
}

/// High-level operation type for a transaction.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Operation {
    /// Simple token or coin transfer.
    Transfer,
    /// Call to an existing smart-contract function.
    ContractExecution,
    /// Deployment of a new smart contract.
    ContractDeployment,
}

// ── Risk / screening enums ────────────────────────────────────────────────────

/// Recommended action from the compliance engine.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RiskAction {
    /// Transaction may proceed.
    Approve,
    /// Transaction requires manual review.
    Review,
    /// Originating wallet should be frozen.
    FreezeWallet,
    /// Transaction must be denied.
    Deny,
}

/// Risk severity level assigned by the screening engine.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RiskScore {
    /// Risk could not be determined.
    Unknown,
    /// Low risk.
    Low,
    /// Moderate risk.
    Medium,
    /// High risk.
    High,
    /// Severe risk.
    Severe,
    /// Address is on a block-list.
    Blocklist,
}

/// Category of risk flagged by the screening engine.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RiskCategory {
    /// OFAC or other sanctions list.
    Sanctions,
    /// Entity that facilitates sanctioned activity.
    SanctionsDesignatedFacilitator,
    /// Administratively designated sanctions target.
    SanctionsAdminDesignated,
    /// Sector-specific sanctions.
    SanctionsSector,
    /// Entity providing financial services to sanctioned counterparties.
    FinancialServiceProvider,
    /// Coin mixer or privacy wallet.
    MixerOrPrivacyWallet,
    /// Ransomware-associated address.
    Ransomware,
    /// Child exploitation-related content or activity.
    Child,
    /// Terrorist financing activity.
    TerroristFinancing,
    /// Online fraud shop.
    FraudShop,
    /// Cryptocurrency exchange.
    Exchange,
    /// Unhosted (self-custodied) wallet.
    Unhosted,
    /// Darknet market or service.
    Darknet,
    /// Gambling platform.
    Gambling,
}

/// Source type for a risk signal.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RiskType {
    /// Direct ownership risk.
    Ownership,
    /// Counterparty risk.
    Counterparty,
    /// Indirect exposure risk.
    Indirect,
}

// ── Screening structs ─────────────────────────────────────────────────────────

/// A single risk signal from the compliance screening engine.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskSignal {
    /// Source entity type (`ADDRESS`, `BLOCKCHAIN`, or `ASSET`).
    pub source: String,
    /// The actual source value (address string, chain name, etc.).
    pub source_value: String,
    /// Assigned risk score.
    pub risk_score: RiskScore,
    /// Categories of risk identified.
    pub risk_categories: Vec<RiskCategory>,
    /// Relationship type of the risk.
    #[serde(rename = "type")]
    pub risk_type: RiskType,
}

/// Compliance screening decision for a transaction.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionScreeningDecision {
    /// ISO 8601 timestamp when the screening ran.
    pub screening_date: String,
    /// Name of the compliance rule that triggered, if any.
    pub rule_name: Option<String>,
    /// Actions recommended by the screening engine.
    pub actions: Option<Vec<RiskAction>>,
    /// Individual risk signals that contributed to the decision.
    pub reasons: Option<Vec<RiskSignal>>,
}

// ── Transaction ───────────────────────────────────────────────────────────────

/// A user-controlled wallet transaction.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    /// Circle-assigned transaction ID.
    pub id: String,
    /// Current on-chain state.
    pub state: TransactionState,
    /// Blockchain this transaction is on.
    pub blockchain: Blockchain,
    /// Direction of the transaction.
    pub transaction_type: TransactionType,
    /// ISO 8601 creation timestamp.
    pub create_date: String,
    /// ISO 8601 last-updated timestamp.
    pub update_date: String,

    /// ABI function signature for contract calls.
    pub abi_function_signature: Option<String>,
    /// ABI parameters for contract calls.
    pub abi_parameters: Option<Vec<serde_json::Value>>,
    /// Token amounts transferred (decimal strings).
    pub amounts: Option<Vec<String>>,
    /// USD equivalent of the transferred amount.
    pub amount_in_usd: Option<String>,
    /// Block hash for the confirmed transaction.
    pub block_hash: Option<String>,
    /// Block height for the confirmed transaction.
    pub block_height: Option<i64>,
    /// Target contract address.
    pub contract_address: Option<String>,
    /// Custody type of the source wallet.
    pub custody_type: Option<CustodyType>,
    /// Recipient address.
    pub destination_address: Option<String>,
    /// Short reason string for failures.
    pub error_reason: Option<String>,
    /// Extended error information.
    pub error_details: Option<String>,
    /// Estimated fee at submission time.
    pub estimated_fee: Option<TransactionFee>,
    /// Requested fee level.
    pub fee_level: Option<FeeLevel>,
    /// ISO 8601 timestamp of the first on-chain confirmation.
    pub first_confirm_date: Option<String>,
    /// Actual network fee paid (decimal string).
    pub network_fee: Option<String>,
    /// Network fee expressed in USD.
    pub network_fee_in_usd: Option<String>,
    /// NFTs transferred in this transaction.
    pub nfts: Option<Vec<Nft>>,
    /// High-level operation type.
    pub operation: Option<Operation>,
    /// Application-defined reference identifier.
    pub ref_id: Option<String>,
    /// Source address.
    pub source_address: Option<String>,
    /// Circle token ID transferred.
    pub token_id: Option<String>,
    /// On-chain transaction hash.
    pub tx_hash: Option<String>,
    /// ID of the end-user who initiated this transaction.
    pub user_id: Option<String>,
    /// ID of the source wallet.
    pub wallet_id: Option<String>,
    /// Compliance screening evaluation.
    pub transaction_screening_evaluation: Option<TransactionScreeningDecision>,
}

// ── Response wrappers ─────────────────────────────────────────────────────────

/// `data` payload wrapping a list of transactions.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionsData {
    /// List of transactions.
    pub transactions: Vec<Transaction>,
}

/// Response envelope for list-transactions.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Transactions {
    /// Paginated transactions.
    pub data: TransactionsData,
}

/// `data` payload wrapping a single transaction.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionData {
    /// The transaction record.
    pub transaction: Transaction,
}

/// Response envelope for a single-transaction lookup.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionResponse {
    /// Transaction data.
    pub data: TransactionData,
}

/// Fee information for the lowest-nonce transaction query.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LowestNonceTransactionFeeInfo {
    /// Suggested high-end fee for the replacement transaction.
    pub new_high_estimated_fee: TransactionFee,
    /// Difference in fee between old and new.
    pub fee_difference_amount: String,
}

/// `data` payload for the lowest-nonce transaction endpoint.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LowestNonceTransactionData {
    /// The stuck transaction.
    pub transaction: Transaction,
    /// Fee information for replacing it.
    pub fee_info: LowestNonceTransactionFeeInfo,
}

/// Response envelope for `getLowestNonceTransaction`.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLowestNonceTransactionResponse {
    /// Lowest-nonce transaction data.
    pub data: LowestNonceTransactionData,
}

/// Fee estimation breakdown for a transaction.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EstimateFeeData {
    /// High-priority fee estimate.
    pub high: Option<TransactionFee>,
    /// Low-priority fee estimate.
    pub low: Option<TransactionFee>,
    /// Medium-priority fee estimate.
    pub medium: Option<TransactionFee>,
    /// ERC-4337 call gas limit.
    pub call_gas_limit: Option<String>,
    /// ERC-4337 verification gas limit.
    pub verification_gas_limit: Option<String>,
    /// ERC-4337 pre-verification gas.
    pub pre_verification_gas: Option<String>,
}

/// Response envelope for fee estimation endpoints.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EstimateTransactionFee {
    /// Fee estimation data.
    pub data: EstimateFeeData,
}

/// `data` payload for address validation.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateAddressData {
    /// `true` if the address is valid for the given blockchain.
    pub is_valid: bool,
}

/// Response envelope for `validateAddress`.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateAddressResponse {
    /// Validation result.
    pub data: ValidateAddressData,
}

// ── Request bodies ────────────────────────────────────────────────────────────

/// Request body for `createTransferTransaction`.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransferTxRequest {
    /// Client-generated idempotency key (UUID).
    pub idempotency_key: String,
    /// Source wallet ID.
    pub wallet_id: String,
    /// Destination address.
    pub destination_address: String,
    /// Amounts to transfer (decimal strings).
    pub amounts: Option<Vec<String>>,
    /// Gas fee level preference.
    pub fee_level: Option<FeeLevel>,
    /// Custom gas limit override.
    pub gas_limit: Option<String>,
    /// Custom gas price override (legacy).
    pub gas_price: Option<String>,
    /// EIP-1559 max fee per gas override.
    pub max_fee: Option<String>,
    /// EIP-1559 priority fee override.
    pub priority_fee: Option<String>,
    /// NFT token IDs to transfer.
    pub nft_token_ids: Option<Vec<String>>,
    /// Application-defined reference identifier.
    pub ref_id: Option<String>,
    /// Circle token ID to transfer.
    pub token_id: Option<String>,
    /// On-chain token contract address (alternative to token_id).
    pub token_address: Option<String>,
    /// Blockchain to use (for cross-chain transfers).
    pub blockchain: Option<Blockchain>,
}

/// Request body for `accelerateTransaction`.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccelerateTxRequest {
    /// Client-generated idempotency key (UUID).
    pub idempotency_key: String,
}

/// Request body for `cancelTransaction`.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelTxRequest {
    /// Client-generated idempotency key (UUID).
    pub idempotency_key: String,
}

/// Request body for `createContractExecutionTransaction`.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateContractExecutionTxRequest {
    /// Client-generated idempotency key (UUID).
    pub idempotency_key: String,
    /// Source wallet ID.
    pub wallet_id: String,
    /// Target contract address.
    pub contract_address: String,
    /// ABI function signature to call.
    pub abi_function_signature: Option<String>,
    /// ABI parameters for the function call.
    pub abi_parameters: Option<Vec<serde_json::Value>>,
    /// Raw ABI-encoded call data (alternative to abi_function_signature).
    pub call_data: Option<String>,
    /// Amount of native coin to send with the call.
    pub amount: Option<String>,
    /// Gas fee level preference.
    pub fee_level: Option<FeeLevel>,
    /// Custom gas limit override.
    pub gas_limit: Option<String>,
    /// Custom gas price override.
    pub gas_price: Option<String>,
    /// EIP-1559 max fee per gas override.
    pub max_fee: Option<String>,
    /// EIP-1559 priority fee override.
    pub priority_fee: Option<String>,
    /// Application-defined reference identifier.
    pub ref_id: Option<String>,
}

/// Request body for `createWalletUpgradeTransaction`.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWalletUpgradeTxRequest {
    /// Client-generated idempotency key (UUID).
    pub idempotency_key: String,
    /// Wallet to upgrade.
    pub wallet_id: String,
    /// Target SCA core version (e.g. `"circle_6900_singleowner_v2"`).
    pub new_sca_core: String,
    /// Gas fee level preference.
    pub fee_level: Option<FeeLevel>,
    /// Custom gas limit override.
    pub gas_limit: Option<String>,
    /// Custom gas price override.
    pub gas_price: Option<String>,
    /// EIP-1559 max fee per gas override.
    pub max_fee: Option<String>,
    /// EIP-1559 priority fee override.
    pub priority_fee: Option<String>,
    /// Application-defined reference identifier.
    pub ref_id: Option<String>,
}

/// Request body for `estimateTransferFee`.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EstimateTransferFeeRequest {
    /// Amounts to transfer (decimal strings).
    pub amounts: Vec<String>,
    /// Destination address.
    pub destination_address: String,
    /// NFT token IDs to transfer.
    pub nft_token_ids: Option<Vec<String>>,
    /// Source address (used when wallet_id is not provided).
    pub source_address: Option<String>,
    /// Circle token ID.
    pub token_id: Option<String>,
    /// On-chain token contract address.
    pub token_address: Option<String>,
    /// Blockchain for the transfer.
    pub blockchain: Option<Blockchain>,
    /// Source wallet ID.
    pub wallet_id: Option<String>,
}

/// Request body for `estimateContractExecutionFee`.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EstimateContractExecFeeRequest {
    /// Target contract address.
    pub contract_address: String,
    /// ABI function signature.
    pub abi_function_signature: Option<String>,
    /// ABI parameters.
    pub abi_parameters: Option<Vec<serde_json::Value>>,
    /// Raw ABI-encoded call data.
    pub call_data: Option<String>,
    /// Amount of native coin to send.
    pub amount: Option<String>,
    /// Blockchain for the call.
    pub blockchain: Option<Blockchain>,
    /// Source address.
    pub source_address: Option<String>,
    /// Source wallet ID.
    pub wallet_id: Option<String>,
}

/// Request body for `validateAddress`.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateAddressRequest {
    /// Address to validate.
    pub address: String,
    /// Blockchain on which the address should be valid.
    pub blockchain: Blockchain,
}

/// Query parameters for `listTransactions`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTransactionsParams {
    /// Filter by blockchain.
    pub blockchain: Option<Blockchain>,
    /// Filter by destination address.
    pub destination_address: Option<String>,
    /// If `true`, include all transactions for the wallet set.
    pub include_all: Option<bool>,
    /// Filter by operation type.
    pub operation: Option<Operation>,
    /// Filter by transaction state.
    pub state: Option<TransactionState>,
    /// Filter by on-chain transaction hash.
    pub tx_hash: Option<String>,
    /// Filter by transaction type (inbound/outbound).
    pub tx_type: Option<TransactionType>,
    /// Filter by user ID.
    pub user_id: Option<String>,
    /// Comma-separated list of wallet IDs to filter by.
    pub wallet_ids: Option<String>,
    /// Start of date range (ISO 8601).
    pub from: Option<String>,
    /// End of date range (ISO 8601).
    pub to: Option<String>,
    /// Pagination cursors.
    #[serde(flatten)]
    pub page: PageParams,
}

/// Query parameters for `getLowestNonceTransaction`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLowestNonceTxParams {
    /// Blockchain to search on.
    pub blockchain: Option<Blockchain>,
    /// On-chain address to search for.
    pub address: Option<String>,
    /// Wallet ID to search for.
    pub wallet_id: Option<String>,
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transaction_state_screaming() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(serde_json::to_string(&TransactionState::Complete)?, "\"COMPLETE\"");
        assert_eq!(serde_json::to_string(&TransactionState::Confirmed)?, "\"CONFIRMED\"");
        assert_eq!(serde_json::to_string(&TransactionState::Queued)?, "\"QUEUED\"");
        Ok(())
    }

    #[test]
    fn risk_action_screaming() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(serde_json::to_string(&RiskAction::FreezeWallet)?, "\"FREEZE_WALLET\"");
        Ok(())
    }

    #[test]
    fn validate_address_request_camel_case() -> Result<(), Box<dyn std::error::Error>> {
        let req =
            ValidateAddressRequest { address: "0xabc".to_string(), blockchain: Blockchain::Eth };
        let s = serde_json::to_string(&req)?;
        assert!(s.contains("\"address\""), "address key in {s}");
        assert!(s.contains("\"blockchain\""), "blockchain key in {s}");
        Ok(())
    }

    #[test]
    fn transfer_tx_request_camel_case() -> Result<(), Box<dyn std::error::Error>> {
        let req = CreateTransferTxRequest {
            idempotency_key: "key".to_string(),
            wallet_id: "w1".to_string(),
            destination_address: "0xdest".to_string(),
            amounts: Some(vec!["1.0".to_string()]),
            fee_level: Some(FeeLevel::High),
            gas_limit: None,
            gas_price: None,
            max_fee: None,
            priority_fee: None,
            nft_token_ids: None,
            ref_id: None,
            token_id: None,
            token_address: None,
            blockchain: None,
        };
        let s = serde_json::to_string(&req)?;
        assert!(s.contains("idempotencyKey"), "{s}");
        assert!(s.contains("walletId"), "{s}");
        assert!(s.contains("destinationAddress"), "{s}");
        Ok(())
    }
}
