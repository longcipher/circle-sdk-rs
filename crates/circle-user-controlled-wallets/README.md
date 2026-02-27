# circle-user-controlled-wallets

[![crates.io](https://img.shields.io/crates/v/circle-user-controlled-wallets.svg)](https://crates.io/crates/circle-user-controlled-wallets)
[![docs.rs](https://docs.rs/circle-user-controlled-wallets/badge.svg)](https://docs.rs/circle-user-controlled-wallets)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](https://github.com/longcipher/circle-sdk-rs/blob/master/LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org)

Async Rust client for the [Circle Web3 Services User-Controlled Wallets API](https://developers.circle.com/api-reference/w3s/user-controlled-wallets).

With User-Controlled Wallets, end-users own their signing keys through a PIN-protected security model — Circle never has access to the private keys.

## Installation

```toml
[dependencies]
circle-user-controlled-wallets = "0.1"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

## Quick Start

```rust,no_run
use circle_user_controlled_wallets::
    {UserWalletsClient, models::user::CreateUserRequest};

#[tokio::main]
async fn main() -> Result<(), circle_user_controlled_wallets::Error> {
    let client = UserWalletsClient::new("your_api_key");

    let req = CreateUserRequest {
        user_id: "user-unique-id-123".to_string(),
    };
    let resp = client.create_user(&req).await?;
    println!("Created user: {:?}", resp.data);
    Ok(())
}
```

## API Coverage

| Area | Endpoints |
|------|-----------|
| Users | Create user, Get user |
| PIN Challenges | Initialize challenge, Get challenge |
| Wallets | List wallets, Get wallet, List balances, List NFTs |
| Transactions | Initiate transaction, List transactions, Get transaction, Cancel transaction |
| Signing | Sign message, Sign typed data |
| Tokens | Get token |

## Authentication

Obtain an API key from the [Circle Developer Console](https://console.circle.com) and pass it to the client constructor, or set the `CIRCLE_API_KEY` environment variable.

## License

Licensed under the [Apache-2.0 License](https://github.com/longcipher/circle-sdk-rs/blob/master/LICENSE).
