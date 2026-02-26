//! User-Controlled Wallets API subcommands.

use eyre::Result;

use crate::output::OutputFormat;

/// Parse a blockchain string for the user crate via serde JSON.
fn parse_blockchain(s: &str) -> Result<circle_user_controlled_wallets::models::common::Blockchain> {
    serde_json::from_str::<circle_user_controlled_wallets::models::common::Blockchain>(&format!(
        "\"{}\"",
        s
    ))
    .map_err(|e| eyre::eyre!("Unrecognised blockchain '{}': {e}", s))
}

/// User-Controlled Wallets subcommands.
#[derive(Debug, clap::Subcommand)]
pub enum UserCommand {
    /// Create a new end-user.
    CreateUser {
        /// Application-defined user identifier.
        #[arg(long)]
        user_id: String,
    },
    /// Get an end-user by ID.
    GetUser {
        /// User UUID.
        id: String,
    },
    /// List end-users.
    ListUsers {
        /// Maximum number of items per page (1–50).
        #[arg(long)]
        page_size: Option<u32>,
    },
    /// Get a short-lived user token for a user.
    GetUserToken {
        /// Application-defined user identifier.
        #[arg(long)]
        user_id: String,
    },
    /// List wallets for an authenticated user.
    ListWallets {
        /// Short-lived user token (or set CIRCLE_USER_TOKEN env var).
        #[arg(long, env = "CIRCLE_USER_TOKEN")]
        user_token: String,
        /// Filter by blockchain (e.g. ETH, MATIC-AMOY).
        #[arg(long)]
        blockchain: Option<String>,
        /// Maximum number of items per page (1–50).
        #[arg(long)]
        page_size: Option<u32>,
    },
    /// Get a wallet by ID for an authenticated user.
    GetWallet {
        /// Short-lived user token (or set CIRCLE_USER_TOKEN env var).
        #[arg(long, env = "CIRCLE_USER_TOKEN")]
        user_token: String,
        /// Wallet UUID.
        id: String,
    },
    /// List transactions for an authenticated user.
    ListTransactions {
        /// Short-lived user token (or set CIRCLE_USER_TOKEN env var).
        #[arg(long, env = "CIRCLE_USER_TOKEN")]
        user_token: String,
        /// Maximum number of items per page (1–50).
        #[arg(long)]
        page_size: Option<u32>,
    },
    /// Get a transaction by ID for an authenticated user.
    GetTransaction {
        /// Short-lived user token (or set CIRCLE_USER_TOKEN env var).
        #[arg(long, env = "CIRCLE_USER_TOKEN")]
        user_token: String,
        /// Transaction UUID.
        id: String,
    },
    /// Validate a blockchain address.
    ValidateAddress {
        /// Address to validate.
        #[arg(long)]
        address: String,
        /// Blockchain identifier (e.g. ETH, SOL).
        #[arg(long)]
        blockchain: String,
    },
}

/// Run a User-Controlled Wallets subcommand.
pub(crate) async fn run(
    cmd: UserCommand,
    api_key: &str,
    base_url: &str,
    output: OutputFormat,
) -> Result<()> {
    let client =
        circle_user_controlled_wallets::UserWalletsClient::with_base_url(api_key, base_url);
    match cmd {
        UserCommand::CreateUser { user_id } => {
            use circle_user_controlled_wallets::models::user::CreateUserRequest;
            let req = CreateUserRequest { user_id };
            let result = client.create_user(&req).await.map_err(|e| eyre::eyre!("{e}"))?;
            crate::output::print_result(&result, output);
            Ok(())
        }
        UserCommand::GetUser { id } => {
            let result = client.get_user(&id).await.map_err(|e| eyre::eyre!("{e}"))?;
            crate::output::print_result(&result, output);
            Ok(())
        }
        UserCommand::ListUsers { page_size } => {
            use circle_user_controlled_wallets::models::{
                common::PageParams, user::ListUsersParams,
            };
            let params = ListUsersParams {
                pin_status: None,
                page: PageParams { page_size, ..Default::default() },
            };
            let result = client.list_users(&params).await.map_err(|e| eyre::eyre!("{e}"))?;
            crate::output::print_result(&result, output);
            Ok(())
        }
        UserCommand::GetUserToken { user_id } => {
            use circle_user_controlled_wallets::models::user::GetUserTokenRequest;
            let req = GetUserTokenRequest { user_id };
            let result = client.get_user_token(&req).await.map_err(|e| eyre::eyre!("{e}"))?;
            crate::output::print_result(&result, output);
            Ok(())
        }
        UserCommand::ListWallets { user_token, blockchain, page_size } => {
            use circle_user_controlled_wallets::models::{
                common::PageParams, wallet::ListWalletsParams,
            };
            let blockchain = blockchain.map(|s| parse_blockchain(&s)).transpose()?;
            let params = ListWalletsParams {
                blockchain,
                page: PageParams { page_size, ..Default::default() },
                ..Default::default()
            };
            let result =
                client.list_wallets(&user_token, &params).await.map_err(|e| eyre::eyre!("{e}"))?;
            crate::output::print_result(&result, output);
            Ok(())
        }
        UserCommand::GetWallet { user_token, id } => {
            let result =
                client.get_wallet(&user_token, &id).await.map_err(|e| eyre::eyre!("{e}"))?;
            crate::output::print_result(&result, output);
            Ok(())
        }
        UserCommand::ListTransactions { user_token, page_size } => {
            use circle_user_controlled_wallets::models::{
                common::PageParams, transaction::ListTransactionsParams,
            };
            let params = ListTransactionsParams {
                page: PageParams { page_size, ..Default::default() },
                ..Default::default()
            };
            let result = client
                .list_transactions(&user_token, &params)
                .await
                .map_err(|e| eyre::eyre!("{e}"))?;
            crate::output::print_result(&result, output);
            Ok(())
        }
        UserCommand::GetTransaction { user_token, id } => {
            let result =
                client.get_transaction(&user_token, &id).await.map_err(|e| eyre::eyre!("{e}"))?;
            crate::output::print_result(&result, output);
            Ok(())
        }
        UserCommand::ValidateAddress { address, blockchain } => {
            use circle_user_controlled_wallets::models::transaction::ValidateAddressRequest;
            let req =
                ValidateAddressRequest { address, blockchain: parse_blockchain(&blockchain)? };
            let result = client.validate_address(&req).await.map_err(|e| eyre::eyre!("{e}"))?;
            crate::output::print_result(&result, output);
            Ok(())
        }
    }
}
