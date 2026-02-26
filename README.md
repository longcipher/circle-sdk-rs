# Circle SDK for Rust

A Rust SDK for [Circle API](https://developers.circle.com/api-reference/).

## Features

- **User-Controlled Wallets** - Full API coverage for user-managed wallets
- **Developer-Controlled Wallets** - Full API coverage for developer-managed wallets
- **Compliance Engine** - Address and transaction screening for regulatory compliance
- **Buidl Wallets** - Modular wallet support (Account Abstraction)
- **CLI** - Command-line interface for all services

## Installation

Add the desired crates to your `Cargo.toml`:

```toml
[dependencies]
circle-user-controlled-wallets = { version = "0.1.0", package = "circle-user-controlled-wallets" }
circle-developer-controlled-wallets = { version = "0.1.0", package = "circle-developer-controlled-wallets" }
circle-compliance = { version = "0.1.0", package = "circle-compliance" }
circle-buidl-wallets = { version = "0.1.0", package = "circle-buidl-wallets" }
```

## Quick Start

```rust
use circle_user_controlled_wallets::Client;

// Create a client
let client = Client::new(
    "https://api.circle.com",
    "YOUR_API_KEY",
);

// List wallets
let wallets = client.list_wallets().await?;
```

## Crates

| Crate | Description |
|-------|-------------|
| `circle-user-controlled-wallets` | User-Controlled Wallets API |
| `circle-developer-controlled-wallets` | Developer-Controlled Wallets API |
| `circle-compliance` | Compliance Engine API |
| `circle-buidl-wallets` | Modular Wallets (Buidl) API |
| `circle-cli` | CLI for all services |

## CLI Usage

```bash
# Build the CLI
cargo build -p circle-cli

# Run commands
cargo run -p circle-cli -- --help
```

## License

Apache-2.0
