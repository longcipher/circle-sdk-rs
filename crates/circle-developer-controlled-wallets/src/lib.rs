//! Rust client for the Circle Web3 Services Developer-Controlled Wallets API.
//!
//! This crate provides a typed async client for the Circle W3S Developer-Controlled
//! Wallets API, covering wallet sets, wallets, transactions, signing, and token
//! management endpoints.
//!
//! # Quick Start
//!
//! ```no_run
//! use circle_developer_controlled_wallets::DeveloperWalletsClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), circle_developer_controlled_wallets::Error> {
//!     let client = DeveloperWalletsClient::new("your_api_key");
//!     let _ = client;
//!     Ok(())
//! }
//! ```

#![deny(missing_docs)]

pub mod client;
pub mod error;
pub mod models;

pub use client::DeveloperWalletsClient;
pub use error::Error;
