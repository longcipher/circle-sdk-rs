//! Transfer request and response types for the Buidl Wallets API.
//!
//! Covers [`ListTransfersParams`], [`Transfer`], [`Transfers`], and [`TransferId`].

use super::common::{Blockchain, PageParams};

/// State of a transfer.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransferState {
    /// Transfer is confirmed on-chain but not yet finalized.
    Confirmed,
    /// Transfer has reached finality.
    Complete,
    /// Transfer failed.
    Failed,
}

/// Direction of a transfer relative to the queried wallet.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransferType {
    /// Tokens received by the wallet.
    InboundTransfer,
    /// Tokens sent from the wallet.
    OutboundTransfer,
}

/// Error reason for a failed transfer.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransferErrorReason {
    /// Transfer failed due to a chain reorganization.
    FailedReorg,
}

/// NFT metadata bundled inside a transfer.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NftIdMetadata {
    /// IPFS or HTTP URI of the NFT metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    /// On-chain token ID of the NFT.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nft_token_id: Option<String>,
}

/// A single transfer event.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transfer {
    /// System-generated unique identifier (UUID).
    pub id: String,
    /// Unique identifier of the wallet related to this transfer.
    pub wallet_id: String,
    /// Amount of tokens transferred.
    pub amount: String,
    /// Date and time the block was mined (ISO-8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_date: Option<String>,
    /// Block hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<String>,
    /// Block height.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_height: Option<i64>,
    /// Blockchain network.
    pub blockchain: Blockchain,
    /// Error reason (only set when `state` is `Failed`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_reason: Option<TransferErrorReason>,
    /// Sender address.
    pub from: String,
    /// NFT metadata (only set for NFT transfers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nft: Option<NftIdMetadata>,
    /// Current state of the transfer.
    pub state: TransferState,
    /// Recipient address.
    pub to: String,
    /// Contract address of the transferred token (absent for native transfers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_address: Option<String>,
    /// Unique identifier of the token.
    pub token_id: String,
    /// Direction of the transfer.
    pub transfer_type: TransferType,
    /// Transaction hash.
    pub tx_hash: String,
    /// User operation hash (account-abstraction transfers only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_op_hash: Option<String>,
    /// Address of the wallet.
    pub wallet_address: String,
    /// Creation timestamp (ISO-8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    /// Last-update timestamp (ISO-8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<String>,
}

/// Inner data object of the `listTransfers` response.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TransfersData {
    /// Transfers matching the query filters.
    pub transfers: Vec<Transfer>,
}

/// Response wrapper for the `listTransfers` endpoint.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Transfers {
    /// Response data.
    pub data: TransfersData,
}

/// Inner data object of the `getTransfer` response.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TransferIdData {
    /// The retrieved transfer.
    pub transfer: Transfer,
}

/// Response wrapper for the `getTransfer` endpoint.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TransferId {
    /// Response data.
    pub data: TransferIdData,
}

/// Query parameters for [`crate::BuidlWalletsClient::list_transfers`].
#[derive(Debug, Default, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTransfersParams {
    /// Filter by wallet addresses (comma-separated list; required by the API).
    pub wallet_addresses: String,
    /// Filter by blockchain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockchain: Option<Blockchain>,
    /// Filter by transfer state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<TransferState>,
    /// Filter by transfer type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_type: Option<TransferType>,
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
    fn transfer_state_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
        let s = serde_json::to_string(&TransferState::Complete)?;
        assert_eq!(s, "\"COMPLETE\"");
        let parsed: TransferState = serde_json::from_str("\"FAILED\"")?;
        assert_eq!(parsed, TransferState::Failed);
        Ok(())
    }

    #[test]
    fn transfer_type_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
        let s = serde_json::to_string(&TransferType::InboundTransfer)?;
        assert_eq!(s, "\"INBOUND_TRANSFER\"");
        Ok(())
    }

    #[test]
    fn transfers_response_deserializes() -> Result<(), Box<dyn std::error::Error>> {
        let json = r#"{
            "data": {
                "transfers": [
                    {
                        "id": "c4d1da72-111e-4d52-bdbf-2e74a2d803d5",
                        "walletId": "aaa11111-111e-4d52-bdbf-2e74a2d803d5",
                        "amount": "1.0",
                        "blockchain": "MATIC-AMOY",
                        "from": "0x4b6c0b0078b63f881503e7fd3a9a1061065db242",
                        "state": "COMPLETE",
                        "to": "0x187785007d4a7d6756e834768597da8fa6fcfe8a",
                        "tokenId": "bbb22222-111e-4d52-bdbf-2e74a2d803d5",
                        "transferType": "OUTBOUND_TRANSFER",
                        "txHash": "0x4a25cc5e661d8504b59c5f38ba93f010e8518966f00e2ceda7955c4b8621357d",
                        "walletAddress": "0x4b6c0b0078b63f881503e7fd3a9a1061065db242"
                    }
                ]
            }
        }"#;
        let resp: Transfers = serde_json::from_str(json)?;
        assert_eq!(resp.data.transfers.len(), 1);
        assert_eq!(resp.data.transfers[0].id, "c4d1da72-111e-4d52-bdbf-2e74a2d803d5");
        assert_eq!(resp.data.transfers[0].state, TransferState::Complete);
        Ok(())
    }
}
