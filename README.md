# Circle SDK for Rust

[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org)
[![CI](https://github.com/longcipher/circle-sdk-rs/actions/workflows/release.yml/badge.svg)](https://github.com/longcipher/circle-sdk-rs/actions)

Async Rust SDK for the [Circle Web3 Services API](https://developers.circle.com/api-reference/).

## Crates

| Crate | crates.io | docs.rs | Description |
|-------|-----------|---------|-------------|
| [`circle-user-controlled-wallets`](crates/circle-user-controlled-wallets) | [![crates.io](https://img.shields.io/crates/v/circle-user-controlled-wallets.svg)](https://crates.io/crates/circle-user-controlled-wallets) | [![docs.rs](https://docs.rs/circle-user-controlled-wallets/badge.svg)](https://docs.rs/circle-user-controlled-wallets) | User-Controlled Wallets API |
| [`circle-developer-controlled-wallets`](crates/circle-developer-controlled-wallets) | [![crates.io](https://img.shields.io/crates/v/circle-developer-controlled-wallets.svg)](https://crates.io/crates/circle-developer-controlled-wallets) | [![docs.rs](https://docs.rs/circle-developer-controlled-wallets/badge.svg)](https://docs.rs/circle-developer-controlled-wallets) | Developer-Controlled Wallets API |
| [`circle-compliance`](crates/circle-compliance) | [![crates.io](https://img.shields.io/crates/v/circle-compliance.svg)](https://crates.io/crates/circle-compliance) | [![docs.rs](https://docs.rs/circle-compliance/badge.svg)](https://docs.rs/circle-compliance) | Compliance Engine API |
| [`circle-buidl-wallets`](crates/circle-buidl-wallets) | [![crates.io](https://img.shields.io/crates/v/circle-buidl-wallets.svg)](https://crates.io/crates/circle-buidl-wallets) | [![docs.rs](https://docs.rs/circle-buidl-wallets/badge.svg)](https://docs.rs/circle-buidl-wallets) | Modular Wallets (Buidl / ERC-4337) API |
| [`circle-cli`](bin/circle-cli) | [![crates.io](https://img.shields.io/crates/v/circle-cli.svg)](https://crates.io/crates/circle-cli) | [![docs.rs](https://docs.rs/circle-cli/badge.svg)](https://docs.rs/circle-cli) | CLI for all services |

## Features

- **User-Controlled Wallets** — End-users own their signing keys via PIN-protected security model
- **Developer-Controlled Wallets** — Your backend manages signing keys for a seamless UX
- **Compliance Engine** — Automated OFAC/AML blockchain address screening
- **Buidl Wallets** — Account-Abstraction (ERC-4337) wallets with gasless operations
- **CLI** — `circle-cli` command-line tool for all services

## Installation

Add the crates you need to `Cargo.toml`:

```toml
[dependencies]
circle-user-controlled-wallets = "0.1"
circle-developer-controlled-wallets = "0.1"
circle-compliance = "0.1"
circle-buidl-wallets = "0.1"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

Or install the CLI:

```bash
cargo install circle-cli
```

## Quick Start

```rust,no_run
use circle_developer_controlled_wallets::{
    DeveloperWalletsClient, models::wallet::ListWalletsParams,
};

#[tokio::main]
async fn main() -> Result<(), circle_developer_controlled_wallets::Error> {
    let client = DeveloperWalletsClient::new("your_api_key");
    let resp = client.list_wallets(&ListWalletsParams::default()).await?;
    println!("Wallets: {:?}", resp.data.wallets);
    Ok(())
}
```

See each crate's README for detailed examples.

## CLI Usage

```bash
export CIRCLE_API_KEY="<YOUR_API_KEY>"

# List developer wallets
circle-cli developer list-wallets

# Screen a blockchain address
circle-cli compliance screen-address --chain ETH --address 0xYourAddress

# Show all commands
circle-cli --help
```

See [bin/circle-cli/README.md](bin/circle-cli/README.md) for the full command reference with verified testnet examples.

## Authentication

Obtain an API key from the [Circle Developer Console](https://console.circle.com). Pass it via the `CIRCLE_API_KEY` environment variable or the `--api-key` flag (CLI).

## License

Apache-2.0
