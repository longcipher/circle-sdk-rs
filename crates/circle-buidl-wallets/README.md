# circle-buidl-wallets

[![crates.io](https://img.shields.io/crates/v/circle-buidl-wallets.svg)](https://crates.io/crates/circle-buidl-wallets)
[![docs.rs](https://docs.rs/circle-buidl-wallets/badge.svg)](https://docs.rs/circle-buidl-wallets)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](https://github.com/longcipher/circle-sdk-rs/blob/master/LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org)

Async Rust client for the [Circle Web3 Services Modular Wallets (Buidl) API](https://developers.circle.com/api-reference/w3s/buidl-wallets).

Buidl Wallets are Account-Abstraction-powered (ERC-4337) wallets that let users pay gas in ERC-20 tokens and batch arbitrary on-chain operations.

## Installation

```toml
[dependencies]
circle-buidl-wallets = "0.1"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

## Quick Start

```rust,no_run
use circle_buidl_wallets::{BuidlWalletsClient, models::transfer::ListTransfersParams};

#[tokio::main]
async fn main() -> Result<(), circle_buidl_wallets::Error> {
    let client = BuidlWalletsClient::new("your_api_key");

    // List recent transfers for a wallet
    let params = ListTransfersParams {
        wallet_addresses: Some("0xYourWalletAddress".to_string()),
        ..Default::default()
    };
    let resp = client.list_transfers(&params).await?;
    for transfer in &resp.data.transfers {
        println!("Transfer {:?}: {:?}", transfer.id, transfer.state);
    }
    Ok(())
}
```

## API Coverage

| Area | Endpoints |
|------|-----------|
| Transfers | List transfers, Get transfer |
| User Operations (ERC-4337) | List user ops, Get user op |
| Wallets | List wallet balances, List wallet NFTs |

## Authentication

Obtain an API key from the [Circle Developer Console](https://console.circle.com) and pass it to the client constructor, or set the `CIRCLE_API_KEY` environment variable.

## License

Licensed under the [Apache-2.0 License](https://github.com/longcipher/circle-sdk-rs/blob/master/LICENSE).
