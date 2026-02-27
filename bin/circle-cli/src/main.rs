//! Circle CLI — command-line interface for the Circle Web3 Services SDK.

use clap::Parser;
use eyre::Result;

mod commands;
mod output;

use commands::{buidl, compliance, developer, user};
use output::OutputFormat;

/// Circle Web3 Services command-line interface.
#[derive(Debug, Parser)]
#[command(name = "circle-cli", about = "Circle Web3 Services SDK CLI", version)]
pub struct Cli {
    /// Circle API key (or set CIRCLE_API_KEY env var).
    #[arg(long, env = "CIRCLE_API_KEY", global = true)]
    pub api_key: Option<String>,

    /// Override the Circle API base URL (useful for Prism mock servers).
    #[arg(long, env = "CIRCLE_BASE_URL", global = true, default_value = "https://api.circle.com")]
    pub base_url: String,

    /// Output format: text or json.
    #[arg(long, global = true, default_value = "json")]
    pub output: OutputFormat,

    /// Subcommand to run.
    #[command(subcommand)]
    pub command: Command,
}

/// Top-level subcommands.
#[derive(Debug, clap::Subcommand)]
pub enum Command {
    /// Buidl Wallets API operations.
    Buidl {
        /// Buidl Wallets subcommand.
        #[command(subcommand)]
        cmd: buidl::BuidlCommand,
    },
    /// Compliance Engine API operations.
    Compliance {
        /// Compliance subcommand.
        #[command(subcommand)]
        cmd: compliance::ComplianceCommand,
    },
    /// Developer-Controlled Wallets API operations.
    Developer {
        /// Developer Wallets subcommand.
        #[command(subcommand)]
        cmd: developer::DeveloperCommand,
    },
    /// User-Controlled Wallets API operations.
    User {
        /// User Wallets subcommand.
        #[command(subcommand)]
        cmd: user::UserCommand,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Install the default rustls CryptoProvider (aws-lc-rs) before any TLS connections.
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    let api_key = cli.api_key.unwrap_or_default();
    let base_url = cli.base_url;
    let output = cli.output;

    match cli.command {
        Command::Buidl { cmd } => buidl::run(cmd, &api_key, &base_url, output).await,
        Command::Compliance { cmd } => compliance::run(cmd, &api_key, &base_url, output).await,
        Command::Developer { cmd } => developer::run(cmd, &api_key, &base_url, output).await,
        Command::User { cmd } => user::run(cmd, &api_key, &base_url, output).await,
    }
}
