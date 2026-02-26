//! Rust client for the Circle Web3 Services User-Controlled Wallets API.
//!
//! This crate provides a typed async client for the Circle W3S User-Controlled
//! Wallets API, covering user management, PIN challenges, wallets, transactions,
//! signing, and token management endpoints.
//!
//! # Quick Start
//!
//! ```no_run
//! use circle_user_controlled_wallets::UserWalletsClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), circle_user_controlled_wallets::Error> {
//!     let client = UserWalletsClient::new("your_api_key");
//!     let _ = client;
//!     Ok(())
//! }
//! ```

#![deny(missing_docs)]

pub mod client;
pub mod error;
pub mod models;

pub use client::UserWalletsClient;
pub use error::Error;
