//! Rust client for the Circle Web3 Services Developer-Controlled Wallets API.
//!
//! This crate provides a typed, async HTTP client for the
//! [Circle W3S Developer-Controlled Wallets API](https://developers.circle.com/api-reference/w3s/developer-controlled-wallets),
//! where your service holds the signing keys on behalf of users.
//!
//! ## Covered Endpoints
//!
//! | Module | Functionality |
//! |--------|---------------|
//! | [`models::wallet_set`] | Create and manage wallet sets |
//! | [`models::wallet`] | Create wallets, query balances and NFTs |
//! | [`models::transaction`] | Initiate and track on-chain transactions |
//! | [`models::signing`] | Sign messages and typed data |
//! | [`models::token`] | Look up token metadata |
//!
//! ## Quick Start
//!
//! ```no_run
//! use circle_developer_controlled_wallets::{
//!     DeveloperWalletsClient, models::wallet::ListWalletsParams,
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<(), circle_developer_controlled_wallets::Error> {
//!     let client = DeveloperWalletsClient::new("your_api_key");
//!     let params = ListWalletsParams::default();
//!     let wallets = client.list_wallets(&params).await?;
//!     println!("Found {} wallets", wallets.data.wallets.len());
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

#![deny(missing_docs)]

pub mod client;
pub mod error;
pub mod models;

pub use client::DeveloperWalletsClient;
pub use error::Error;
