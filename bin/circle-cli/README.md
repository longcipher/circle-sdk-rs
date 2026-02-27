# circle-cli

`circle-cli` is a command-line interface for the Circle Web3 Services SDK. It provides subcommands for interacting with the Buidl Wallets, Compliance Engine, Developer-Controlled Wallets, and User-Controlled Wallets APIs.

## Build

```bash
cargo build -p circle-cli
# binary: target/debug/circle-cli
```

Or install globally:

```bash
cargo install --path bin/circle-cli
```

## Configuration

Set your Circle API key via the environment variable (recommended) or the `--api-key` flag:

```bash
export CIRCLE_API_KEY="<YOUR_API_KEY>"
```

> **Note:** For testnet experiments use a `TEST_API_KEY`. Testnet keys cannot access mainnet blockchains (e.g. `ETH`) — use testnet variants such as `ETH-SEPOLIA` or `MATIC-AMOY`.

## Global Flags

| Flag | Env var | Default | Description |
|------|---------|---------|-------------|
| `--api-key` | `CIRCLE_API_KEY` | — | Circle API key |
| `--base-url` | `CIRCLE_BASE_URL` | `https://api.circle.com` | Override base URL (e.g. for a Prism mock) |
| `--output` | — | `json` | Output format: `json` or `text` |

## Quick Reference

```text
circle-cli
├── buidl
│   ├── list-transfers        List transfers (requires --wallet-id)
│   ├── get-transfer <id>     Get a transfer by ID
│   ├── list-user-ops         List ERC-4337 user operations
│   ├── get-user-op <id>      Get a user operation by ID
│   ├── list-wallet-balances  List token balances for a wallet
│   └── list-wallet-nfts      List NFTs held by a wallet
├── compliance
│   └── screen-address        Screen a blockchain address for compliance risk
├── developer
│   ├── list-wallet-sets      List all developer wallet sets
│   ├── get-wallet-set <id>   Get a wallet set by ID
│   ├── list-wallets          List all developer wallets
│   ├── get-wallet <id>       Get a wallet by ID
│   ├── list-transactions     List transactions
│   ├── get-transaction <id>  Get a transaction by ID
│   ├── get-token <id>        Get a token definition by ID
│   └── validate-address      Validate a blockchain address
└── user
    ├── create-user           Create a new end-user
    ├── get-user <id>         Get an end-user by ID
    ├── list-users            List all end-users
    ├── get-user-token        Get a short-lived user token
    ├── list-wallets          List wallets for an authenticated user
    ├── get-wallet <id>       Get a user wallet by ID
    ├── list-transactions     List user transactions
    ├── get-transaction <id>  Get a user transaction by ID
    └── validate-address      Validate a blockchain address
```

---

## Verified Working Commands (Testnet)

The commands below have been verified against the Circle testnet API. Run them in order for a complete end-to-end walkthrough.

### Setup

```bash
export CIRCLE_API_KEY="<YOUR_TEST_API_KEY>"
BINARY="./target/debug/circle-cli"
```

---

### Developer-Controlled Wallets

#### List wallet sets

```bash
$BINARY developer list-wallet-sets
# {"data":{"walletSets":[]}}
```

#### List wallets

```bash
$BINARY developer list-wallets
# {"data":{"wallets":[]}}
```

Filter by blockchain (testnet only with a TEST key):

```bash
$BINARY developer list-wallets --blockchain ETH-SEPOLIA
```

#### List transactions

```bash
$BINARY developer list-transactions
# {"data":{"transactions":[]}}
```

Filter by blockchain and state:

```bash
$BINARY developer list-transactions --blockchain ETH-SEPOLIA --state COMPLETE --page-size 10
```

#### Validate a blockchain address

> Use testnet blockchain identifiers with a TEST key: `ETH-SEPOLIA`, `MATIC-AMOY`, `SOL-DEVNET`, etc.

```bash
$BINARY developer validate-address \
  --blockchain ETH-SEPOLIA \
  --address 0xab5801a7d398351b8be11c439e05c5b3259aec9b
# {"data":{"isValid":true}}
```

```bash
$BINARY developer validate-address \
  --blockchain MATIC-AMOY \
  --address 0xab5801a7d398351b8be11c439e05c5b3259aec9b
# {"data":{"isValid":true}}
```

---

### User-Controlled Wallets

#### List end-users

```bash
$BINARY user list-users
# {"data":{"users":[]}}
```

#### Create an end-user

```bash
$BINARY user create-user --user-id my-test-user-001
# {"data":{"id":"my-test-user-001","pinStatus":"UNSET","status":"ENABLED",...}}
```

#### Get an end-user

```bash
$BINARY user get-user my-test-user-001
```

#### Get a short-lived user token

The token is required for user-scoped wallet / transaction operations and expires after ~1 hour.

```bash
$BINARY user get-user-token --user-id my-test-user-001
# {"data":{"userToken":"<JWT>","encryptionKey":"<base64>"}}
```

Save the token for subsequent calls:

```bash
export CIRCLE_USER_TOKEN=$($BINARY user get-user-token --user-id my-test-user-001 \
  | jq -r '.data.userToken')
```

#### List user wallets

```bash
$BINARY user list-wallets --user-token "$CIRCLE_USER_TOKEN"
# {"data":{"wallets":[]}}
```

Filter by blockchain:

```bash
$BINARY user list-wallets \
  --user-token "$CIRCLE_USER_TOKEN" \
  --blockchain ETH-SEPOLIA
```

#### List user transactions

```bash
$BINARY user list-transactions --user-token "$CIRCLE_USER_TOKEN"
# {"data":{"transactions":[]}}
```

#### Validate a blockchain address (user context)

```bash
$BINARY user validate-address \
  --blockchain ETH-SEPOLIA \
  --address 0xab5801a7d398351b8be11c439e05c5b3259aec9b
# {"data":{"isValid":true}}
```

---

### Buidl Wallets

#### List ERC-4337 user operations

```bash
$BINARY buidl list-user-ops
# {"data":{"userOperations":[]}}
```

#### List token balances for a wallet

Replace `<WALLET_UUID>` with an actual developer or user wallet UUID from the list commands above.

```bash
$BINARY buidl list-wallet-balances --wallet-id <WALLET_UUID>
```

#### List NFTs for a wallet

```bash
$BINARY buidl list-wallet-nfts --wallet-id <WALLET_UUID>
```

#### List transfers

`--wallet-id` is required by the Circle API. Pass one or more wallet addresses (comma-separated):

```bash
$BINARY buidl list-transfers --wallet-id 0xab5801a7d398351b8be11c439e05c5b3259aec9b
# {"data":{"transfers":[]}}
```

Multiple addresses:

```bash
$BINARY buidl list-transfers --wallet-id "<ADDR1>,<ADDR2>"
```

---

### Compliance

#### Screen a blockchain address

```bash
$BINARY compliance screen-address \
  --chain ETH-SEPOLIA \
  --address 0xab5801a7d398351b8be11c439e05c5b3259aec9b
# {
#   "result": "APPROVED",
#   "decision": { "screeningDate": "2026-02-27T05:58:36Z", ... },
#   "id": "<uuid>",
#   "address": "0xab5801a7d398351b8be11c439e05c5b3259aec9b",
#   "chain": "ETH-SEPOLIA",
#   "details": []
# }
```

Other supported chains: `ETH`, `MATIC`, `MATIC-AMOY`, `SOL`, `SOL-DEVNET`, `BTC`, `ARB`, `ARB-SEPOLIA`, `AVAX`, `AVAX-FUJI`, etc.

---

## Known Limitations

| Command | Status | Notes |
|---------|--------|-------|
| `developer validate-address --blockchain ETH` | ❌ Error 156006 | Mainnet blocked; use `ETH-SEPOLIA` with TEST key |
| `buidl list-transfers` without `--wallet-id` | ❌ (arg required) | `--wallet-id` is a required CLI argument |
| `buidl list-wallet-balances` / `list-wallet-nfts` with unknown UUID | ❌ Error 230003 | Wallet UUID must exist in the system |

---

## Reference

- [Circle API Reference — Programmable Wallets](https://developers.circle.com/api-reference/wallets/programmable-wallets/request-testnet-tokens)
- [Circle Developer Console](https://console.circle.com)
