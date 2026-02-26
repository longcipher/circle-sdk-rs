//! Developer-Controlled Wallets API subcommands.

use eyre::Result;

use crate::output::OutputFormat;

/// Parse a blockchain string into the developer `Blockchain` enum via serde JSON.
fn parse_blockchain(
    s: &str,
) -> Result<circle_developer_controlled_wallets::models::common::Blockchain> {
    serde_json::from_str::<circle_developer_controlled_wallets::models::common::Blockchain>(
        &format!("\"{}\"", s),
    )
    .map_err(|e| eyre::eyre!("Unrecognised blockchain '{}': {e}", s))
}

/// Parse a transaction state string via serde JSON.
fn parse_tx_state(
    s: &str,
) -> Result<circle_developer_controlled_wallets::models::transaction::TransactionState> {
    serde_json::from_str::<
        circle_developer_controlled_wallets::models::transaction::TransactionState,
    >(&format!("\"{}\"", s))
    .map_err(|e| eyre::eyre!("Unrecognised transaction state '{}': {e}", s))
}

/// Developer-Controlled Wallets subcommands.
#[derive(Debug, clap::Subcommand)]
pub enum DeveloperCommand {
    /// List wallet sets.
    ListWalletSets {
        /// Maximum number of items per page (1–50).
        #[arg(long)]
        page_size: Option<u32>,
    },
    /// Get a wallet set by ID.
    GetWalletSet {
        /// Wallet set UUID.
        id: String,
    },
    /// List wallets.
    ListWallets {
        /// Filter by blockchain (e.g. ETH, MATIC-AMOY).
        #[arg(long)]
        blockchain: Option<String>,
        /// Maximum number of items per page (1–50).
        #[arg(long)]
        page_size: Option<u32>,
    },
    /// Get a wallet by ID.
    GetWallet {
        /// Wallet UUID.
        id: String,
    },
    /// List transactions.
    ListTransactions {
        /// Filter by blockchain (e.g. ETH, MATIC-AMOY).
        #[arg(long)]
        blockchain: Option<String>,
        /// Filter by transaction state (e.g. COMPLETE, FAILED).
        #[arg(long)]
        state: Option<String>,
        /// Maximum number of items per page (1–50).
        #[arg(long)]
        page_size: Option<u32>,
    },
    /// Get a transaction by ID.
    GetTransaction {
        /// Transaction UUID.
        id: String,
    },
    /// Get a token definition by ID.
    GetToken {
        /// Token UUID.
        id: String,
    },
    /// Validate a blockchain address.
    ValidateAddress {
        /// Blockchain identifier (e.g. ETH, SOL).
        #[arg(long)]
        blockchain: String,
        /// Address to validate.
        #[arg(long)]
        address: String,
    },
}

/// Run a Developer-Controlled Wallets subcommand.
pub(crate) async fn run(
    cmd: DeveloperCommand,
    api_key: &str,
    base_url: &str,
    output: OutputFormat,
) -> Result<()> {
    let client = circle_developer_controlled_wallets::DeveloperWalletsClient::with_base_url(
        api_key, base_url,
    );
    match cmd {
        DeveloperCommand::ListWalletSets { page_size } => {
            use circle_developer_controlled_wallets::models::{
                common::PageParams, wallet_set::ListWalletSetsParams,
            };
            let params =
                ListWalletSetsParams { page: PageParams { page_size, ..Default::default() } };
            let result = client.list_wallet_sets(&params).await.map_err(|e| eyre::eyre!("{e}"))?;
            crate::output::print_result(&result, output);
            Ok(())
        }
        DeveloperCommand::GetWalletSet { id } => {
            let result = client.get_wallet_set(&id).await.map_err(|e| eyre::eyre!("{e}"))?;
            crate::output::print_result(&result, output);
            Ok(())
        }
        DeveloperCommand::ListWallets { blockchain, page_size } => {
            use circle_developer_controlled_wallets::models::{
                common::PageParams, wallet::ListWalletsParams,
            };
            let blockchain = blockchain.map(|s| parse_blockchain(&s)).transpose()?;
            let params = ListWalletsParams {
                blockchain,
                page: PageParams { page_size, ..Default::default() },
                ..Default::default()
            };
            let result = client.list_wallets(&params).await.map_err(|e| eyre::eyre!("{e}"))?;
            crate::output::print_result(&result, output);
            Ok(())
        }
        DeveloperCommand::GetWallet { id } => {
            let result = client.get_wallet(&id).await.map_err(|e| eyre::eyre!("{e}"))?;
            crate::output::print_result(&result, output);
            Ok(())
        }
        DeveloperCommand::ListTransactions { blockchain, state, page_size } => {
            use circle_developer_controlled_wallets::models::transaction::ListTransactionsParams;
            let blockchain = blockchain.map(|s| parse_blockchain(&s)).transpose()?;
            let state = state.map(|s| parse_tx_state(&s)).transpose()?;
            let params =
                ListTransactionsParams { blockchain, state, page_size, ..Default::default() };
            let result = client.list_transactions(&params).await.map_err(|e| eyre::eyre!("{e}"))?;
            crate::output::print_result(&result, output);
            Ok(())
        }
        DeveloperCommand::GetTransaction { id } => {
            let result = client.get_transaction(&id).await.map_err(|e| eyre::eyre!("{e}"))?;
            crate::output::print_result(&result, output);
            Ok(())
        }
        DeveloperCommand::GetToken { id } => {
            let result = client.get_token(&id).await.map_err(|e| eyre::eyre!("{e}"))?;
            crate::output::print_result(&result, output);
            Ok(())
        }
        DeveloperCommand::ValidateAddress { blockchain, address } => {
            use circle_developer_controlled_wallets::models::transaction::ValidateAddressRequest;
            let req =
                ValidateAddressRequest { blockchain: parse_blockchain(&blockchain)?, address };
            let result = client.validate_address(&req).await.map_err(|e| eyre::eyre!("{e}"))?;
            crate::output::print_result(&result, output);
            Ok(())
        }
    }
}
