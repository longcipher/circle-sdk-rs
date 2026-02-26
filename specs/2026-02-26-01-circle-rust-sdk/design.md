# Design Document: Circle Rust SDK

| Metadata      | Details                                                    |
| :------------ | :--------------------------------------------------------- |
| **Author**    | pb-plan agent                                              |
| **Status**    | Draft                                                      |
| **Created**   | 2026-02-26                                                 |
| **Reviewers** | TBD                                                        |
| **Related Issues** | N/A                                                   |

## 1. Executive Summary

**Problem:** No idiomatic Rust SDK exists for Circle's Web3 Services (W3S) API family, forcing Rust developers to write raw HTTP calls or use JSON directly, without type safety, ergonomic error handling, or proper test coverage.

**Solution:** Implement four library crates (one per OpenAPI spec), a CLI binary, and a full test harness — all in the existing Cargo workspace. Each library crate is typed end-to-end from request to response, uses `hpx` with `rustls` as the HTTP transport, and is verified against Stoplight Prism mock servers driven by the OpenAPI YAML files already in `docs/wallets/`.

---

## 2. Requirements & Goals

### 2.1 Problem Statement

Circle's W3S platform exposes four REST API families (Modular Buidl Wallets, Compliance Engine, Developer-Controlled Wallets, User-Controlled Wallets) each documented by a separate OpenAPI 3.0 specification already present in `docs/wallets/`. Rust developers targeting Circle's platform need a safe, async, well-documented client that:

- Models every request/response type as Rust structs derived from the OpenAPI schemas.
- Provides a typed, future-based API method for every operation.
- Integrates naturally into `tokio` async runtimes.
- Ships with unit tests and contract-level integration tests against Prism mocks.
- Includes a one-binary CLI (`circle-cli`) to exercise all four API groups from the command line.
- Meets `crates.io` publishing standards (package metadata, `#![deny(missing_docs)]`, MSRV annotation, `README`, license).

### 2.2 Functional Goals

1. **circle-buidl-wallets crate:** Full typed client for `docs/wallets/buidl-wallets.yaml` — covering Transfers (list, get), UserOps (list, get, create), and Wallets (list, get, create) endpoints.
2. **circle-compliance crate:** Full typed client for `docs/wallets/compliance.yaml` — covering Address Screening (screen address) endpoints.
3. **circle-developer-controlled-wallets crate:** Full typed client for `docs/wallets/developer-controlled-wallets.yaml` — covering Wallet Sets (get all, get one, create, update), Wallets (create, list, get, update, list with balances), Transactions (send, list, get, estimate fee), Signing (sign message, sign typed data), and Token Lookup endpoints.
4. **circle-user-controlled-wallets crate:** Full typed client for `docs/wallets/user-controlled-wallets.yaml` — covering Users (create, list, get, get by token, challenges), PIN Authentication (setup challenge, user token), Social/Email Authentication, Wallets, Transactions, Token Lookup, and Signing endpoints.
5. **circle-cli binary:** A `clap`-based CLI with four subcommands (`buidl`, `compliance`, `developer`, `user`) each exposing sub-subcommands for every API operation, with human-readable and JSON output modes.
6. **Unit tests:** In-crate `#[cfg(test)]` modules using mocked HTTP responses covering happy paths and the main error paths for every endpoint.
7. **Integration tests:** `tests/` directory per crate; `just integration-test` launches Prism mock servers from the OpenAPI specs and runs `cargo test --test integration` against them.
8. **Justfile commands:** `just test` (unit), `just integration-test` (Prism-based end-to-end), `just prism-buidl / prism-compliance / prism-developer / prism-user` (individual servers).
9. **crates.io readiness:** `Cargo.toml` includes `description`, `license`, `repository`, `homepage`, `keywords`, `categories`, `documentation`, and `readme`; all public items carry doc-comments.

### 2.3 Non-Functional Goals

- **Performance:** Zero-copy deserialization where possible; keep allocations proportional to response size; `hpx` connection pooling reused across requests.
- **Reliability:** All `unwrap` / `expect` / `panic` / `todo` / `unimplemented` are forbidden at compile-time via existing workspace Clippy rules. Every fallible operation returns `Result`.
- **Security:** TLS via `rustls` (enforced through `hpx` feature selection); API keys never logged; Bearer token held in `SecretString`-wrapped type or handled as `String` with explicit zeroing policy (assumption: no `secrecy` crate — see §2.5).
- **Observability:** `tracing` spans on every HTTP request and response; request ID propagated as `X-Request-Id` header.
- **Correctness:** Contract-level correctness guaranteed by integration tests against Prism mock servers which validate request/response shapes against the exact OpenAPI specs.

### 2.4 Out of Scope

- WebSocket streaming APIs (no WebSocket paths were found in the four OpenAPI specs; the `hpx`/`hpx-transport` WebSocket capability is available but not required here).
- SDKs for Circle's non-wallet APIs (Payments, Payouts, NFTs).
- A frontend/WASM build of the SDK.
- Full CLI interactive mode or TUI.
- Publishing to `crates.io` (this spec covers the code and metadata; publishing is a separate manual step).
- Automatic code generation from the OpenAPI YAML (the implementation is hand-written and validated against the specs via Prism).

### 2.5 Assumptions

1. **`hpx` version 2.3.1** is available on `crates.io` and its public API is compatible with `tokio`-based async code and provides a `rustls`-enabled HTTP/1.1 + HTTP/2 client. If the crate key differs (e.g., `hpx-transport`), the workspace `Cargo.toml` dependency name will be adjusted.
2. **`@stoplight/prism-cli`** can be installed via `npm` / `npx` and is available as `prism` in `PATH` for the Justfile integration-test recipe.
3. **No `secrecy` crate** is available in the workspace; API key security relies on encapsulation with no debug/display implementations exposing the value.
4. **`tokio` 1.x** is the async runtime; the test suite uses `#[tokio::test]`.
5. The four OpenAPI YAML files at `docs/wallets/` are the authoritative contract and will not change during implementation.
6. Crate names mirror the API families: `circle-buidl-wallets`, `circle-compliance`, `circle-developer-controlled-wallets`, `circle-user-controlled-wallets`. These are the crates.io identifiers.
7. `MSRV` is Rust 1.85 (edition 2024 is in use per workspace manifest).

---

## 3. Architecture Overview

### 3.1 System Context

```text
┌─────────────────────────────────────────────────────┐
│  circle-cli (bin/circle-cli)                        │
│  clap-based CLI — 4 subcommands                     │
└──────┬───────┬──────────────┬──────────────────┬────┘
       │       │              │                  │
       ▼       ▼              ▼                  ▼
circle-buidl  circle-        circle-developer-  circle-user-
-wallets      compliance     controlled-wallets controlled-wallets
  (crate)       (crate)         (crate)            (crate)
       │       │              │                  │
       └───────┴──────────────┴──────────────────┘
                              │
                         hpx (HTTP client, rustls)
                              │
                    https://api.circle.com  (prod)
                    http://localhost:4010   (Prism mock, buidl)
                    http://localhost:4011   (Prism mock, compliance)
                    http://localhost:4012   (Prism mock, developer)
                    http://localhost:4013   (Prism mock, user)
```

Each library crate is self-contained with its own error type, client struct, request/response models, and tests. The `circle-cli` binary depends on all four library crates and wires them to `clap` sub-commands.

### 3.2 Key Design Principles

1. **Typed client, typed models:** Every endpoint maps to a method on a `Client` struct. Parameters become typed builder structs or method arguments. Responses deserialize into typed structs.
2. **Builder pattern for optional params:** Requests with optional query/body parameters use a builder struct implementing `Default`, e.g., `ListTransfersParams::builder().blockchain(Blockchain::Eth).build()`.
3. **Thin client, no hidden state:** The `Client` holds only the base URL and the `hpx` HTTP client. Auth token is passed at construction time.
4. **Error types via `thiserror`:** Each crate defines `pub enum Error { Http(hpx::Error), Api(ApiError), Deserialize(serde_json::Error), ... }`.
5. **Pagination as async streams (future work):** Standard list methods return the first page; a `list_all` helper using `futures::Stream` is planned but is out of scope for Phase 1 (see §2.4).
6. **Prism-driven contract tests:** Integration tests point their client `base_url` at a locally running Prism mock server, enabling every request/response shape to be validated against the spec without real API keys.

### 3.3 Existing Components to Reuse

| Component              | Location                              | How to Reuse                                              |
| :--------------------- | :------------------------------------ | :-------------------------------------------------------- |
| Workspace `Cargo.toml` | `Cargo.toml`                          | Add all new crates as workspace members; inherit `version`, `edition`, `license` via `workspace.package` |
| `clap` dep             | `[workspace.dependencies]`            | Inherit in `bin/circle-cli/Cargo.toml`                   |
| `serde` dep            | `[workspace.dependencies]`            | Inherit in all four library crates for JSON models       |
| `tracing` dep          | `[workspace.dependencies]`            | Inherit for per-request spans                            |
| `thiserror` dep        | `[workspace.dependencies]`            | Inherit for crate-level `Error` enums                    |
| `eyre` dep             | `[workspace.dependencies]`            | Inherit in `bin/circle-cli` for top-level error propagation |
| `tokio` dep            | Must be added to `[workspace.dependencies]` | Runtime for all async tests and CLI               |
| `hpx` dep              | Must be added to `[workspace.dependencies]` | HTTP transport for all clients (feature `rustls`) |
| Justfile               | `Justfile`                            | `just test` recipe already exists; extend with `integration-test` and Prism helper recipes |
| Clippy/lint config     | `Cargo.toml` `[workspace.lints]`      | Automatically inherited; no per-crate override needed    |

> `tokio` and `hpx` are not yet in `[workspace.dependencies]` — they must be added via `cargo add`.

---

## 4. Detailed Design

### 4.1 Module Structure

```text
crates/
  circle-buidl-wallets/
    Cargo.toml
    README.md
    src/
      lib.rs              # re-exports, crate docs, #![deny(missing_docs)]
      client.rs           # BuidlWalletsClient
      models/
        mod.rs
        transfer.rs       # Transfer, Transfers, TransferState, ...
        user_op.rs        # UserOp, UserOps, UserOpState, ...
        wallet.rs         # Wallet, Wallets, ...
        common.rs         # Blockchain, Pagination, ApiError, ...
      error.rs            # pub enum Error
    tests/
      integration.rs      # requires Prism on localhost:4010

  circle-compliance/
    Cargo.toml
    README.md
    src/
      lib.rs
      client.rs           # ComplianceClient
      models/
        mod.rs
        screening.rs      # Address, BlockchainAddressScreeningResponse, ...
        common.rs         # Chain, RiskAction, RiskScore, ...
      error.rs
    tests/
      integration.rs      # requires Prism on localhost:4011

  circle-developer-controlled-wallets/
    Cargo.toml
    README.md
    src/
      lib.rs
      client.rs           # DeveloperWalletsClient
      models/
        mod.rs
        wallet_set.rs     # WalletSet, WalletSets, ...
        wallet.rs         # Wallet, Wallets, WalletBalance, ...
        transaction.rs    # Transaction, Transactions, EstimateFee, ...
        signing.rs        # SignMessageRequest, SignedMessage, ...
        token.rs          # Token, TokenLookup, ...
        common.rs         # Blockchain, Pagination, ApiError, ...
      error.rs
    tests/
      integration.rs      # requires Prism on localhost:4012

  circle-user-controlled-wallets/
    Cargo.toml
    README.md
    src/
      lib.rs
      client.rs           # UserWalletsClient
      models/
        mod.rs
        user.rs           # User, Users, UserResponse, ...
        challenge.rs      # Challenge, Challenges, Pin, ...
        token.rs          # UserTokenResponse, ...
        wallet.rs         # Wallet, Wallets, ...
        transaction.rs    # Transaction, Transactions, ...
        signing.rs        # SignMessageRequest, ...
        common.rs         # Blockchain, Pagination, ApiError, ...
      error.rs
    tests/
      integration.rs      # requires Prism on localhost:4013

bin/
  circle-cli/
    Cargo.toml
    README.md
    src/
      main.rs             # clap App, subcommand dispatch, eyre
      commands/
        mod.rs
        buidl.rs          # buidl subcommand + sub-subcommands
        compliance.rs     # compliance subcommand
        developer.rs      # developer subcommand
        user.rs           # user subcommand
      output.rs           # human-readable vs JSON output formatter
```

### 4.2 Data Structures & Types

**Shared across crates (each crate defines its own; no shared common crate in v1):**

```rust
/// Circle API error payload.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct ApiError {
    pub code: i32,
    pub message: String,
}

/// Pagination cursor parameters shared by list endpoints.
#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct PaginationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
}

/// Bearer auth token (no Debug impl to prevent leaking in logs).
pub struct ApiKey(String);
```

**Example — Buidl Wallets:**

```rust
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Transfer {
    pub id: String,
    pub blockchain: Blockchain,
    pub state: TransferState,
    pub tx_hash: Option<String>,
    pub user_op_hash: Option<String>,
    // ...
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Blockchain {
    Eth,
    EthSepolia,
    Matic,
    MaticAmoy,
    // ... all values from enum in spec
}
```

### 4.3 Interface Design

**Client construction (same pattern in every crate):**

```rust
impl BuidlWalletsClient {
    /// Create a new client using the given API key and the default Circle
    /// production base URL (`https://api.circle.com`).
    pub fn new(api_key: impl Into<String>) -> Self { ... }

    /// Create a client pointing to a custom base URL (useful for Prism mocks).
    pub fn with_base_url(api_key: impl Into<String>, base_url: impl Into<String>) -> Self { ... }
}
```

**Endpoint methods (every spec operation becomes a method):**

```rust
// GET /v1/w3s/buidl/transfers
pub async fn list_transfers(&self, params: &ListTransfersParams) -> Result<Transfers, Error>;

// GET /v1/w3s/buidl/transfers/{id}
pub async fn get_transfer(&self, id: &str) -> Result<TransferId, Error>;

// GET /v1/w3s/buidl/userOps
pub async fn list_user_ops(&self, params: &ListUserOpsParams) -> Result<UserOps, Error>;

// GET /v1/w3s/buidl/userOps/{id}
pub async fn get_user_op(&self, id: &str) -> Result<UserOp, Error>;

// POST /v1/w3s/buidl/userOps
pub async fn create_user_op(&self, req: &CreateUserOpRequest) -> Result<UserOp, Error>;
```

**Error type (per-crate):**

```rust
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP transport error: {0}")]
    Http(#[from] hpx::Error),

    #[error("Circle API error {code}: {message}")]
    Api { code: i32, message: String },

    #[error("Failed to deserialize response: {0}")]
    Deserialize(#[from] serde_json::Error),

    #[error("Invalid parameter: {0}")]
    InvalidParam(String),
}
```

### 4.4 Logic Flow

**Standard GET list request lifecycle:**

```text
Client::list_transfers(params)
  │
  ├─ Build URL: base_url + "/v1/w3s/buidl/transfers"
  ├─ Serialize params as query string (serde_qs or manual)
  ├─ Add headers: Authorization: Bearer <key>, X-Request-Id: uuid
  ├─ hpx::Client::get(url).headers(...).send().await?
  │
  ├─ status 200 → deserialize body as Transfers → Ok(Transfers)
  ├─ status 4xx/5xx → deserialize body as ApiError → Err(Error::Api{...})
  └─ transport error → Err(Error::Http(...))
```

**Standard POST mutation request lifecycle:**

```text
Client::create_wallet_set(req)
  │
  ├─ Build URL: base_url + "/v1/w3s/developer/walletSets"
  ├─ Serialize req as JSON body
  ├─ Add headers: Authorization, Content-Type: application/json, X-Request-Id
  ├─ hpx::Client::post(url).json(&req).send().await?
  │
  ├─ status 200 / 201 → deserialize as WalletSetResponse → Ok(WalletSetResponse)
  ├─ status 4xx/5xx  → Err(Error::Api{...})
  └─ transport error → Err(Error::Http(...))
```

**Prism integration test flow:**

```text
just integration-test
  │
  ├─ Start Prism mock servers: ports 4010-4013 per spec
  ├─ Wait for each server to be ready (health poll)
  ├─ cargo test --test integration --all-features (all 4 crates)
  ├─ Each integration test creates Client::with_base_url("http://localhost:40XX")
  ├─ Executes every API method
  ├─ Asserts on mocked response structure
  └─ Stop Prism servers (cleanup via trap / background process kill)
```

### 4.5 Configuration

**Per-crate (no config file; config via constructor):**

| Config Item           | How Set                         | Default                       |
| :-------------------- | :------------------------------ | :---------------------------- |
| API key               | `Client::new(api_key)`         | None — required               |
| Base URL              | `Client::with_base_url(...)`   | `https://api.circle.com`      |
| Request timeout       | `ClientBuilder::timeout(dur)`  | 30 seconds (assumption)       |
| Connection pool size  | Managed by `hpx` defaults      | hpx default                   |

**CLI (env + flags):**

| Config Item   | CLI Flag            | Env Var              |
| :------------ | :------------------ | :------------------- |
| API key       | `--api-key`         | `CIRCLE_API_KEY`     |
| Base URL      | `--base-url`        | `CIRCLE_BASE_URL`    |
| Output format | `--output json\|text` | —                  |

**Justfile environment for integration tests:**

| Variable              | Default              |
| :-------------------- | :------------------- |
| `PRISM_BUIDL_PORT`    | `4010`               |
| `PRISM_COMPLIANCE_PORT` | `4011`             |
| `PRISM_DEVELOPER_PORT` | `4012`              |
| `PRISM_USER_PORT`     | `4013`               |

### 4.6 Error Handling

- Every public `async fn` returns `Result<T, Error>`.
- HTTP 4xx responses are deserialized from `{"code": N, "message": "..."}` into `Error::Api`.
- HTTP 5xx responses use the same deserialization; if the body is not a valid API error, the raw status is surfaced.
- Network failures (timeout, DNS, TLS) map to `Error::Http`.
- JSON deserialization failures map to `Error::Deserialize`.
- Invalid caller inputs (e.g., empty wallet address) map to `Error::InvalidParam`.
- `tracing::error!` is emitted on all error branches; request IDs are included in spans for correlation.

---

## 5. Verification & Testing Strategy

### 5.1 Unit Tests

Each crate has `#[cfg(test)] mod tests` blocks at the bottom of `client.rs` and every `models/*.rs` file. Coverage targets:

- **Model serialization/deserialization:** Round-trip JSON for every struct using inline `serde_json::from_str` / `serde_json::to_string` assertions.
- **Error mapping:** Verify that a 4xx JSON body produces `Error::Api` and that a transport error produces `Error::Http`.
- **Query string construction:** Verify that optional pagination params produce correct query strings.

Tooling: `cargo test --all-features` (the `just test` recipe).

### 5.2 Integration Testing

Each crate's `tests/integration.rs` uses `tokio` and `Client::with_base_url` to hit a Prism mock server. Prism is started by the Justfile recipe:

```text
just integration-test
```

Mock strategy:

- Prism runs in "mock" mode from the spec: `prism mock docs/wallets/<spec>.yaml --port <port>`
- Prism returns spec-compliant example responses for all defined operations.
- Tests assert: method does not return `Err`, response fields are non-empty, and status codes match spec.
- No real API keys are used; Prism accepts any `Authorization: Bearer ...` header unless validation is enabled.

### 5.3 Critical Path Verification (The "Harness")

| Verification Step | Command                                               | Success Criteria                                  |
| :---------------- | :---------------------------------------------------- | :------------------------------------------------ |
| **VP-01**         | `just test`                                           | All unit tests pass with 0 failures               |
| **VP-02**         | `just integration-test`                               | All integration tests pass (Prism mocks running)  |
| **VP-03**         | `just lint`                                           | 0 clippy warnings, 0 typos, 0 unused deps         |
| **VP-04**         | `cargo doc --all-features --no-deps`                  | 0 rustdoc warnings; all public items documented   |
| **VP-05**         | `circle-cli buidl list-transfers --wallet-addresses 0x123` | Exit code 0, JSON output printed            |
| **VP-06**         | `circle-cli compliance screen-address --address 0x123 --chain ETH --idempotency-key <uuid>` | Exit code 0 |
| **VP-07**         | `circle-cli developer list-wallet-sets`               | Exit code 0, JSON output printed                  |
| **VP-08**         | `circle-cli user list-users`                          | Exit code 0, JSON output printed                  |

### 5.4 Validation Rules

| Test Case ID | Action                                                   | Expected Outcome                        | Verification Method          |
| :----------- | :------------------------------------------------------- | :-------------------------------------- | :--------------------------- |
| **TC-01**    | Call `list_transfers` with valid params on Prism         | Returns `Ok(Transfers)` with ≥0 items   | Unit assertion in integration test |
| **TC-02**    | Call `get_transfer` with non-existent ID on Prism        | Returns `Err(Error::Api { code: 404 })` (if Prism returns 404) or `Ok(...)` if mock returns 200 | Assertion on result variant |
| **TC-03**    | Deserialize a valid Transfer JSON fixture                | All fields populated                    | `serde_json::from_str` test  |
| **TC-04**    | Call `screen_address` via Prism                          | Returns `Ok(BlockchainAddressScreeningResponse)` | Integration test assertion |
| **TC-05**    | Call `create_wallet_set` via Prism                       | Returns `Ok(WalletSetResponse)` with HTTP 201 | Integration test assertion |
| **TC-06**    | Call `create_user` via Prism                             | Returns `Ok(UserResponse)` with HTTP 201 | Integration test assertion |
| **TC-07**    | CLI `circle-cli developer list-wallet-sets` with Prism base URL | Exit 0, JSON on stdout          | Shell assertion in Justfile smoke recipe |
| **TC-08**    | All model enums serialize to expected SCREAMING_SNAKE_CASE | Round-trip to/from JSON      | Serde unit test per enum type |

---

## 6. Implementation Plan

- [ ] **Phase 1: Foundation** — Workspace prep, workspace dependencies (tokio, hpx), crate scaffolding, shared patterns (ApiKey, PaginationParams, Error skeleton), Prism Justfile recipes.
- [ ] **Phase 2: Core Logic** — Implement all four library crates: models, client methods, unit tests.
- [ ] **Phase 3: Integration** — Write integration test harness per crate; verify all endpoints against Prism mocks; add `circle-cli` binary with all subcommands.
- [ ] **Phase 4: Polish** — Doc-comments on all public items, crates.io metadata in all `Cargo.toml` files, `README.md` per crate, final `just lint` and `just test` green pass.

---

## 7. Cross-Functional Concerns

### Security

- `ApiKey` struct carries no `Debug` or `Display` impl; the inner `String` is never passed to `tracing` macros. The token is included in the `Authorization` header which `hpx` sends over TLS only.
- No secrets appear in test output; integration tests use a placeholder key (`TEST_API_KEY=test`).

### Backward Compatibility

- All four library crates start at version `0.1.0` (following workspace `package.version`). Semver guarantees apply from `0.1.x` onward. Breaking model changes bump minor version.
- The CLI is `0.1.0`; its CLI interface is not stabilized until `1.0.0`.

### Migration

- No migration needed — this is a greenfield implementation in an existing workspace with empty `crates/` and `bin/` directories.

### Monitoring / Observability

- `tracing::instrument` on every client method; span fields include `url`, `method`, `request_id`.
- `tracing::debug!` on successful responses; `tracing::error!` on errors.
- No metrics (OpenTelemetry OTLP is available via workspace but out of scope for v0.1).

### Publishing Checklist (crates.io)

Each library `Cargo.toml` must include:

```toml
[package]
name = "circle-buidl-wallets"
description = "Rust client for the Circle Web3 Services Modular Wallets (Buidl) API"
license = "Apache-2.0"
repository = "https://github.com/longcipher/circle-sdk-rs"
homepage = "https://github.com/longcipher/circle-sdk-rs"
documentation = "https://docs.rs/circle-buidl-wallets"
readme = "README.md"
keywords = ["circle", "web3", "wallet", "blockchain", "api"]
categories = ["api-bindings", "web-programming::http-client"]
rust-version = "1.85"
```
