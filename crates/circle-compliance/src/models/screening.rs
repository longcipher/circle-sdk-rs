//! Screening resource models for the Circle Compliance Engine API.

/// Supported blockchain networks for address screening.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE")]
pub enum Chain {
    /// Ethereum mainnet.
    Eth,
    /// Ethereum Sepolia testnet.
    #[serde(rename = "ETH-SEPOLIA")]
    EthSepolia,
    /// Avalanche C-Chain mainnet.
    Avax,
    /// Avalanche Fuji testnet.
    #[serde(rename = "AVAX-FUJI")]
    AvaxFuji,
    /// Polygon PoS mainnet.
    Matic,
    /// Polygon Amoy testnet.
    #[serde(rename = "MATIC-AMOY")]
    MaticAmoy,
    /// Algorand mainnet.
    Algo,
    /// Cosmos Hub mainnet.
    Atom,
    /// Arbitrum One mainnet.
    Arb,
    /// Arbitrum Sepolia testnet.
    #[serde(rename = "ARB-SEPOLIA")]
    ArbSepolia,
    /// Hedera mainnet.
    Hbar,
    /// Solana mainnet.
    Sol,
    /// Solana devnet.
    #[serde(rename = "SOL-DEVNET")]
    SolDevnet,
    /// Unichain mainnet.
    Uni,
    /// Unichain Sepolia testnet.
    #[serde(rename = "UNI-SEPOLIA")]
    UniSepolia,
    /// TRON mainnet.
    Trx,
    /// Stellar mainnet.
    Xlm,
    /// Bitcoin Cash mainnet.
    Bch,
    /// Bitcoin mainnet.
    Btc,
    /// Bitcoin SV mainnet.
    Bsv,
    /// Ethereum Classic mainnet.
    Etc,
    /// Litecoin mainnet.
    Ltc,
    /// Monero mainnet.
    Xmr,
    /// XRP Ledger mainnet.
    Xrp,
    /// 0x / ZRX.
    Zrx,
    /// Optimism mainnet.
    Op,
    /// Polkadot mainnet.
    Dot,
}

/// Request body for the `screenAddress` endpoint.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenAddressRequest {
    /// UUID v4 idempotency key.
    pub idempotency_key: String,
    /// Blockchain address to screen.
    pub address: String,
    /// Blockchain network.
    pub chain: Chain,
}

/// Action to take based on a screening decision.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RiskAction {
    /// Address is safe to proceed.
    Approve,
    /// Address requires manual review.
    Review,
    /// The wallet associated with the address should be frozen.
    FreezeWallet,
    /// Transaction/interaction should be denied.
    Deny,
}

/// Risk severity score.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RiskScore {
    /// Risk cannot be determined.
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

/// Risk category of a signal.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RiskCategory {
    /// Government/international sanctions.
    Sanctions,
    /// Child sexual abuse material.
    Csam,
    /// General illicit behavior.
    IllicitBehavior,
    /// Gambling-related.
    Gambling,
    /// Terrorist financing.
    TerroristFinancing,
    /// Unsupported category.
    Unsupported,
    /// Frozen address.
    Frozen,
    /// Other risk.
    Other,
    /// Industry considered high-risk.
    HighRiskIndustry,
    /// Politically exposed person.
    Pep,
    /// Trusted entity.
    Trusted,
    /// Hacking-related.
    Hacking,
    /// Human trafficking.
    HumanTrafficking,
    /// Subject to special regulatory measures.
    SpecialMeasures,
}

/// Relationship type of a risk signal to the screened address.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RiskType {
    /// Direct ownership risk.
    Ownership,
    /// Risk from a counterparty.
    Counterparty,
    /// Indirect exposure.
    Indirect,
}

/// Risk signal source identifier and location.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalSource {
    /// UUID of the vendor response row.
    pub row_id: String,
    /// JSON path of the signal in the vendor response.
    pub pointer: String,
}

/// A risk signal associated with the screened address.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskSignal {
    /// Signal data source (`ADDRESS`, `BLOCKCHAIN`, or `ASSET`).
    pub source: String,
    /// Value of the source (e.g. a blockchain address).
    pub source_value: String,
    /// Risk severity.
    pub risk_score: RiskScore,
    /// Risk categories.
    pub risk_categories: Vec<RiskCategory>,
    /// Relationship type.
    #[serde(rename = "type")]
    pub risk_type: RiskType,
    /// Pointer back to the raw vendor response.
    pub signal_source: Option<SignalSource>,
}

/// Screening decision for a blockchain address.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressScreeningDecision {
    /// ISO-8601 date the screening was run.
    pub screening_date: String,
    /// Matched rule name (if any).
    pub rule_name: Option<String>,
    /// Actions to take.
    pub actions: Option<Vec<RiskAction>>,
    /// Risk signals driving the decision.
    pub reasons: Option<Vec<RiskSignal>>,
}

/// Raw vendor response detail.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreeningVendorDetail {
    /// UUID of this vendor response record.
    pub id: String,
    /// Vendor name.
    pub vendor: String,
    /// Free-form vendor response payload.
    pub response: serde_json::Value,
    /// Creation timestamp (ISO-8601).
    pub create_date: String,
}

/// Response from the `screenAddress` endpoint.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockchainAddressScreeningResponse {
    /// Summary result of the screening evaluation.
    pub result: ScreeningResult,
    /// Detailed screening decision.
    pub decision: AddressScreeningDecision,
    /// UUID matching the idempotency key from the request.
    pub id: String,
    /// Screened blockchain address.
    pub address: String,
    /// Blockchain network.
    pub chain: Chain,
    /// Raw vendor response details.
    pub details: Vec<ScreeningVendorDetail>,
    /// UUID of any generated compliance alert.
    pub alert_id: Option<String>,
}

/// Top-level outcome of a screening request.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ScreeningResult {
    /// Address is approved.
    Approved,
    /// Address is denied.
    Denied,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chain_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
        let s = serde_json::to_string(&Chain::EthSepolia)?;
        assert_eq!(s, "\"ETH-SEPOLIA\"");
        let parsed: Chain = serde_json::from_str("\"MATIC\"")?;
        assert_eq!(parsed, Chain::Matic);
        Ok(())
    }

    #[test]
    fn risk_action_deserializes() -> Result<(), Box<dyn std::error::Error>> {
        let a: RiskAction = serde_json::from_str("\"APPROVE\"")?;
        assert_eq!(a, RiskAction::Approve);
        Ok(())
    }
}
