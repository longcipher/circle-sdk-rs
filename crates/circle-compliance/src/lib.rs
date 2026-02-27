//! Rust client for the Circle Web3 Services Compliance Engine API.
//!
//! This crate provides a typed, async HTTP client for the
//! [Circle W3S Compliance Engine API](https://developers.circle.com/api-reference/w3s/compliance),
//! which enables automated blockchain address and transaction screening for
//! regulatory compliance (OFAC, AML/KYC workflows).
//!
//! ## Covered Endpoints
//!
//! | Module | Functionality |
//! |--------|---------------|
//! | [`models::screening`] | Screen blockchain addresses for sanctions / risk |
//!
//! ## Quick Start
//!
//! ```no_run
//! use circle_compliance::{
//!     ComplianceClient,
//!     models::screening::{Chain, ScreenAddressRequest},
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<(), circle_compliance::Error> {
//!     let client = ComplianceClient::new("your_api_key");
//!     let request = ScreenAddressRequest {
//!         idempotency_key: "550e8400-e29b-41d4-a716-446655440000".to_string(),
//!         address: "0xTargetAddress".to_string(),
//!         chain: Chain::Eth,
//!     };
//!     let result = client.screen_address(&request).await?;
//!     println!("Screening result: {:?}", result);
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

pub use client::ComplianceClient;
pub use error::Error;
