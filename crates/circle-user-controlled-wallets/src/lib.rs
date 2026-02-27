//! Rust client for the Circle Web3 Services User-Controlled Wallets API.
//!
//! This crate provides a typed, async HTTP client for the
//! [Circle W3S User-Controlled Wallets API](https://developers.circle.com/api-reference/w3s/user-controlled-wallets),
//! where end-users own their signing keys through a PIN-protected security model.
//!
//! ## Covered Endpoints
//!
//! | Module | Functionality |
//! |--------|---------------|
//! | [`models::user`] | Create and retrieve end-user accounts |
//! | [`models::challenge`] | Initiate and retrieve PIN / security-factor challenges |
//! | [`models::wallet`] | List wallets and query balances and NFTs |
//! | [`models::transaction`] | Initiate and track on-chain transactions |
//! | [`models::signing`] | Sign messages and typed data |
//! | [`models::token`] | Look up token metadata |
//!
//! ## Quick Start
//!
//! ```no_run
//! use circle_user_controlled_wallets::{UserWalletsClient, models::user::CreateUserRequest};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), circle_user_controlled_wallets::Error> {
//!     let client = UserWalletsClient::new("your_api_key");
//!     let req = CreateUserRequest { user_id: "user-123".to_string() };
//!     let user = client.create_user(&req).await?;
//!     println!("Created user: {:?}", user.data);
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

pub use client::UserWalletsClient;
pub use error::Error;
