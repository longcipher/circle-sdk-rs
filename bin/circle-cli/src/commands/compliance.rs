//! Compliance Engine API subcommands.

use eyre::Result;

use crate::output::OutputFormat;

/// Compliance Engine subcommands.
#[derive(Debug, clap::Subcommand)]
pub enum ComplianceCommand {
    /// Screen a blockchain address for compliance risk.
    ScreenAddress {
        /// Blockchain identifier (e.g. ETH, ETH-SEPOLIA, MATIC, SOL, BTC).
        #[arg(long)]
        chain: String,
        /// Blockchain address to screen.
        #[arg(long)]
        address: String,
    },
}

/// Parse a blockchain chain string into the `Chain` enum via serde JSON.
fn parse_chain(s: &str) -> Result<circle_compliance::models::screening::Chain> {
    serde_json::from_str::<circle_compliance::models::screening::Chain>(&format!("\"{}\"", s))
        .map_err(|e| eyre::eyre!("Unrecognised chain '{}': {e}", s))
}

/// Run a Compliance Engine subcommand.
pub(crate) async fn run(
    cmd: ComplianceCommand,
    api_key: &str,
    base_url: &str,
    output: OutputFormat,
) -> Result<()> {
    let client = circle_compliance::ComplianceClient::with_base_url(api_key, base_url);
    match cmd {
        ComplianceCommand::ScreenAddress { chain, address } => {
            use circle_compliance::models::screening::ScreenAddressRequest;
            let req = ScreenAddressRequest {
                idempotency_key: uuid::Uuid::new_v4().to_string(),
                address,
                chain: parse_chain(&chain)?,
            };
            let result = client.screen_address(&req).await.map_err(|e| eyre::eyre!("{e}"))?;
            crate::output::print_result(&result, output);
            Ok(())
        }
    }
}
