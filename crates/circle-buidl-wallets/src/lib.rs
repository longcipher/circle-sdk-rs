//! Rust client for the Circle Web3 Services Modular Wallets (Buidl) API.
//!
//! This crate provides a typed async client for the Circle W3S Buidl Wallets API,
//! covering Transfers, UserOps (ERC-4337 user operations), and Wallets endpoints.
//!
//! # Quick Start
//!
//! ```no_run
//! use circle_buidl_wallets::{BuidlWalletsClient, models::transfer::ListTransfersParams};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), circle_buidl_wallets::Error> {
//!     let client = BuidlWalletsClient::new("your_api_key");
//!     let params = ListTransfersParams {
//!         wallet_addresses: "0xYourAddress".to_string(),
//!         ..Default::default()
//!     };
//!     let transfers = client.list_transfers(&params).await?;
//!     println!("Found {} transfers", transfers.data.transfers.len());
//!     Ok(())
//! }
//! ```

#![deny(missing_docs)]

pub mod client;
pub mod error;
pub mod models;

pub use client::BuidlWalletsClient;
pub use error::Error;
