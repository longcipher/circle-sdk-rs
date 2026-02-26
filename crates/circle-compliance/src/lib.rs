//! Rust client for the Circle Web3 Services Compliance Engine API.
//!
//! This crate provides a typed async client for the Circle W3S Compliance Engine API,
//! covering address and transaction screening endpoints.
//!
//! # Quick Start
//!
//! ```no_run
//! use circle_compliance::ComplianceClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), circle_compliance::Error> {
//!     let client = ComplianceClient::new("your_api_key");
//!     let _ = client;
//!     Ok(())
//! }
//! ```

#![deny(missing_docs)]

pub mod client;
pub mod error;
pub mod models;

pub use client::ComplianceClient;
pub use error::Error;
