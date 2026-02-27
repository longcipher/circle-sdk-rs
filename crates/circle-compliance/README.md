# circle-compliance

[![crates.io](https://img.shields.io/crates/v/circle-compliance.svg)](https://crates.io/crates/circle-compliance)
[![docs.rs](https://docs.rs/circle-compliance/badge.svg)](https://docs.rs/circle-compliance)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](https://github.com/longcipher/circle-sdk-rs/blob/master/LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org)

Async Rust client for the [Circle Web3 Services Compliance Engine API](https://developers.circle.com/api-reference/w3s/compliance).

The Compliance Engine enables automated blockchain address and transaction screening for OFAC sanctions lists and AML/KYC risk assessment.

## Installation

```toml
[dependencies]
circle-compliance = "0.1"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

## Quick Start

```rust,no_run
use circle_compliance::{
    ComplianceClient,
    models::screening::{Chain, ScreenAddressRequest},
};

#[tokio::main]
async fn main() -> Result<(), circle_compliance::Error> {
    let client = ComplianceClient::new("your_api_key");
    let request = ScreenAddressRequest {
        idempotency_key: "550e8400-e29b-41d4-a716-446655440000".to_string(),
        address: "0xTargetAddress".to_string(),
        chain: Chain::Eth,
    };
    let result = client.screen_address(&request).await?;
    println!("Screening result: {:?}", result);
    Ok(())
}
```

## API Coverage

| Area | Endpoints |
|------|-----------|
| Address Screening | Screen a blockchain address for sanctions / risk |

## Authentication

Obtain an API key from the [Circle Developer Console](https://console.circle.com) and pass it to the client constructor, or set the `CIRCLE_API_KEY` environment variable.

## License

Licensed under the [Apache-2.0 License](https://github.com/longcipher/circle-sdk-rs/blob/master/LICENSE).
