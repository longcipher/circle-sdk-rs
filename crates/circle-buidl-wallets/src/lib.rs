//! Rust client for the Circle Web3 Services Modular Wallets (Buidl) API.
//!
//! This crate provides a typed, async HTTP client for the
//! [Circle W3S Buidl Wallets API](https://developers.circle.com/api-reference/w3s/buidl-wallets),
//! which enables Account-Abstraction-powered (ERC-4337) wallets.
//!
//! ## Covered Endpoints
//!
//! | Module | Functionality |
//! |--------|---------------|
//! | [`models::transfer`] | List and retrieve cross-chain transfers |
//! | [`models::user_op`] | List and retrieve ERC-4337 user operations |
//! | [`models::wallet`] | Query wallet balances and NFT holdings |
//!
//! ## Quick Start
//!
//! ```no_run
//! use circle_buidl_wallets::{BuidlWalletsClient, models::transfer::ListTransfersParams};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), circle_buidl_wallets::Error> {
//!     let client = BuidlWalletsClient::new("your_api_key");
//!     let params = ListTransfersParams {
//!         wallet_addresses: Some("0xYourWalletAddress".to_string()),
//!         ..Default::default()
//!     };
//!     let transfers = client.list_transfers(&params).await?;
//!     println!("Found {} transfers", transfers.data.transfers.len());
//!     Ok(())
//! }
//! ```
//!
//! ## Authentication
//!
//! All requests require a Circle API key, which can be created in the
//! [Circle Developer Console](https://console.circle.com).
//!
//! ## Error Handling
//!
//! Every fallible operation returns [`Error`], which captures both HTTP-level
//! transport failures and API-level error responses from Circle.
//!
//! ## Feature Flags
//!
//! This crate currently has no optional feature flags.

#![deny(missing_docs)]

pub mod client;
pub mod error;
pub mod models;

pub use client::BuidlWalletsClient;
pub use error::Error;
