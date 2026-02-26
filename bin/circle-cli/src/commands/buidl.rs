//! Buidl Wallets API subcommands.

use eyre::Result;

use crate::output::OutputFormat;

/// Buidl Wallets subcommands.
#[derive(Debug, clap::Subcommand)]
pub enum BuidlCommand {
    /// List transfers for a wallet address.
    ListTransfers {
        /// Filter by wallet address (comma-separated list).
        #[arg(long)]
        wallet_id: Option<String>,
        /// Start of date range (ISO-8601).
        #[arg(long)]
        from: Option<String>,
        /// End of date range (ISO-8601).
        #[arg(long)]
        to: Option<String>,
        /// Maximum number of items per page (1–50).
        #[arg(long)]
        page_size: Option<u32>,
    },
    /// Get a single transfer by ID.
    GetTransfer {
        /// Transfer UUID.
        id: String,
    },
    /// List user operations (ERC-4337).
    ListUserOps {
        /// Filter by sender addresses (comma-separated).
        #[arg(long)]
        wallet_id: Option<String>,
        /// Maximum number of items per page (1–50).
        #[arg(long)]
        page_size: Option<u32>,
    },
    /// Get a single user operation by ID.
    GetUserOp {
        /// User operation UUID.
        id: String,
    },
    /// List token balances for a wallet.
    ListWalletBalances {
        /// Wallet UUID.
        #[arg(long)]
        wallet_id: String,
        /// Maximum number of items per page (1–50).
        #[arg(long)]
        page_size: Option<u32>,
    },
    /// List NFTs held by a wallet.
    ListWalletNfts {
        /// Wallet UUID.
        #[arg(long)]
        wallet_id: String,
        /// Maximum number of items per page (1–50).
        #[arg(long)]
        page_size: Option<u32>,
    },
}

/// Run a Buidl Wallets subcommand.
pub(crate) async fn run(
    cmd: BuidlCommand,
    api_key: &str,
    base_url: &str,
    output: OutputFormat,
) -> Result<()> {
    let client = circle_buidl_wallets::BuidlWalletsClient::with_base_url(api_key, base_url);
    match cmd {
        BuidlCommand::ListTransfers { wallet_id, from, to, page_size } => {
            use circle_buidl_wallets::models::{common::PageParams, transfer::ListTransfersParams};
            let params = ListTransfersParams {
                wallet_addresses: wallet_id.unwrap_or_default(),
                page: PageParams { from, to, page_size, ..Default::default() },
                ..Default::default()
            };
            let result = client.list_transfers(&params).await.map_err(|e| eyre::eyre!("{e}"))?;
            crate::output::print_result(&result, output);
            Ok(())
        }
        BuidlCommand::GetTransfer { id } => {
            let result = client.get_transfer(&id).await.map_err(|e| eyre::eyre!("{e}"))?;
            crate::output::print_result(&result, output);
            Ok(())
        }
        BuidlCommand::ListUserOps { wallet_id, page_size } => {
            use circle_buidl_wallets::models::{common::PageParams, user_op::ListUserOpsParams};
            let params = ListUserOpsParams {
                senders: wallet_id,
                page: PageParams { page_size, ..Default::default() },
                ..Default::default()
            };
            let result = client.list_user_ops(&params).await.map_err(|e| eyre::eyre!("{e}"))?;
            crate::output::print_result(&result, output);
            Ok(())
        }
        BuidlCommand::GetUserOp { id } => {
            let result = client.get_user_op(&id).await.map_err(|e| eyre::eyre!("{e}"))?;
            crate::output::print_result(&result, output);
            Ok(())
        }
        BuidlCommand::ListWalletBalances { wallet_id, page_size } => {
            use circle_buidl_wallets::models::{
                common::PageParams, wallet::ListWalletBalancesParams,
            };
            let params = ListWalletBalancesParams {
                page: PageParams { page_size, ..Default::default() },
                ..Default::default()
            };
            let result = client
                .list_wallet_balances_by_id(&wallet_id, &params)
                .await
                .map_err(|e| eyre::eyre!("{e}"))?;
            crate::output::print_result(&result, output);
            Ok(())
        }
        BuidlCommand::ListWalletNfts { wallet_id, page_size } => {
            use circle_buidl_wallets::models::{common::PageParams, wallet::ListWalletNftsParams};
            let params = ListWalletNftsParams {
                page: PageParams { page_size, ..Default::default() },
                ..Default::default()
            };
            let result = client
                .list_wallet_nfts_by_id(&wallet_id, &params)
                .await
                .map_err(|e| eyre::eyre!("{e}"))?;
            crate::output::print_result(&result, output);
            Ok(())
        }
    }
}
