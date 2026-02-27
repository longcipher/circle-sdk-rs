# circle-developer-controlled-wallets

[![crates.io](https://img.shields.io/crates/v/circle-developer-controlled-wallets.svg)](https://crates.io/crates/circle-developer-controlled-wallets)
[![docs.rs](https://docs.rs/circle-developer-controlled-wallets/badge.svg)](https://docs.rs/circle-developer-controlled-wallets)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](https://github.com/longcipher/circle-sdk-rs/blob/master/LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org)

Async Rust client for the [Circle Web3 Services Developer-Controlled Wallets API](https://developers.circle.com/api-reference/w3s/developer-controlled-wallets).

With Developer-Controlled Wallets, your backend service manages signing keys on behalf of users, enabling a seamless UX without user-managed private keys.

## Installation

```toml
[dependencies]
circle-developer-controlled-wallets = "0.1"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

## Quick Start

```rust,no_run
use circle_developer_controlled_wallets::
    {DeveloperWalletsClient, models::wallet::ListWalletsParams};

#[tokio::main]
async fn main() -> Result<(), circle_developer_controlled_wallets::Error> {
    let client = DeveloperWalletsClient::new("your_api_key");

    let params = ListWalletsParams::default();
    let resp = client.list_wallets(&params).await?;
    for wallet in &resp.data.wallets {
        println!("Wallet {:?} on {:?}", wallet.id, wallet.blockchain);
    }
    Ok(())
}
```

## API Coverage

| Area | Endpoints |
|------|-----------|
| Wallet Sets | List wallet sets, Get wallet set |
| Wallets | List wallets, Get wallet, Create wallet, List balances, List NFTs |
| Transactions | Initiate transaction, List transactions, Get transaction, Cancel transaction |
| Signing | Sign message, Sign typed data |
| Tokens | Get token |
| Addresses | Validate address |

## Authentication

Obtain an API key from the [Circle Developer Console](https://console.circle.com) and pass it to the client constructor, or set the `CIRCLE_API_KEY` environment variable.

## License

Licensed under the [Apache-2.0 License](https://github.com/longcipher/circle-sdk-rs/blob/master/LICENSE).
