# Circle Rust SDK — Implementation Tasks

| Metadata        | Details                                                                |
| :-------------- | :--------------------------------------------------------------------- |
| **Design Doc**  | [specs/2026-02-26-01-circle-rust-sdk/design.md](design.md)            |
| **Owner**       | TBD                                                                    |
| **Start Date**  | 2026-02-26                                                             |
| **Target Date** | TBD                                                                    |
| **Status**      | Planning                                                               |

## Summary & Phasing

Greenfield implementation of four Rust library crates and one CLI binary in the existing workspace, verified via Stoplight Prism mock servers from the four OpenAPI specs in `docs/wallets/`.

- **Phase 1: Foundation & Scaffolding** — Workspace dependency additions, Prism Justfile recipes, crate directory scaffold, shared `ApiKey` / `PaginationParams` patterns, `Error` skeleton per crate.
- **Phase 2: Core Logic** — Full model types + client methods + unit tests for all four library crates.
- **Phase 3: Integration & CLI** — Prism-driven integration tests per crate; `circle-cli` binary with all subcommands wired to library crates.
- **Phase 4: Polish, QA & Docs** — Doc-comments, crates.io `Cargo.toml` metadata, per-crate `README.md`, final lint and test green pass.

---

## Phase 1: Foundation & Scaffolding

### Task 1.1: Extend Workspace Dependencies

> **Context:** The workspace `Cargo.toml` at the root already declares shared dependencies for `serde`, `tracing`, `thiserror`, `eyre`, and `clap` but is missing `tokio` and `hpx`. All four library crates and the CLI require `tokio` (async runtime + test macro) and `hpx` (HTTP client with `rustls`). Adding them to `[workspace.dependencies]` once avoids version drift and enables `workspace = true` in sub-crates.
> **Verification:** `cargo check --all-targets` passes; `cargo tree | grep -E 'tokio|hpx'` shows a single version of each.

- **Priority:** P0
- **Scope:** Workspace dependency management
- **Status:** � DONE

- [x] **Step 1:** Run `cargo add tokio --workspace` — adds tokio to `[workspace.dependencies]`.
- [x] **Step 2:** Run `cargo add hpx --workspace` — adds hpx to `[workspace.dependencies]`; confirm version 2.3.1.
- [x] **Step 3:** Run `cargo add serde_json --workspace` — needed for response deserialization across all crates.
- [x] **Step 4:** Run `cargo add uuid --workspace` — needed for idempotency key and `X-Request-Id` generation.
- [x] **Step 5:** Run `cargo add serde_qs --workspace` — needed for serializing query-string parameters from structs.
- [x] **Verification:** `cargo check --workspace` exits 0; `grep -E 'tokio|hpx|serde_json|uuid|serde_qs' Cargo.toml` shows all five entries.

---

### Task 1.2: Add Prism Justfile Recipes

> **Context:** Stoplight Prism (`npx @stoplight/prism-cli mock`) is used to spin up mock API servers from the four OpenAPI specs. The `Justfile` already has `test` and `lint` recipes. Extending it with `prism-*` and `integration-test` recipes makes the integration test workflow repeatable via a single command.
> **Verification:** `just prism-buidl` starts Prism on port 4010 and `curl -s http://localhost:4010/v1/w3s/buidl/transfers?walletAddresses=0x1` returns a non-empty JSON body.

- **Priority:** P0
- **Scope:** Justfile / CI tooling
- **Status:** � DONE

- [x] **Step 1:** Add `prism-buidl` recipe: `npx --yes @stoplight/prism-cli mock docs/wallets/buidl-wallets.yaml --port 4010 &`.
- [x] **Step 2:** Add `prism-compliance` recipe for port 4011.
- [x] **Step 3:** Add `prism-developer` recipe for port 4012.
- [x] **Step 4:** Add `prism-user` recipe for port 4013.
- [x] **Step 5:** Add `prism-all` recipe that starts all four servers in parallel (background processes).
- [x] **Step 6:** Add `integration-test` recipe that calls `prism-all`, waits for readiness (sleep or poll loop), runs `cargo test --test integration --all-features --workspace`, then kills background Prism processes.
- [x] **Verification:** `just prism-buidl & sleep 3 && curl -sf http://localhost:4010/v1/w3s/buidl/transfers\?walletAddresses\=0x1` returns HTTP 200 or 422 (Prism responding).

---

### Task 1.3: Scaffold circle-buidl-wallets Crate ✅

> **Context:** Creates the directory structure, `Cargo.toml`, `src/lib.rs`, `src/error.rs`, and empty module files for the Buidl Wallets library crate. This task establishes the pattern that the other three crates follow (Tasks 1.4–1.6).
> Uses `workspace = true` for all shared dependencies per project conventions.
> **Verification:** `cargo check -p circle-buidl-wallets` exits 0.

- **Priority:** P0
- **Scope:** Crate scaffolding
- **Status:** � DONE

- [x] **Step 1:** Create `crates/circle-buidl-wallets/Cargo.toml` with `workspace = true` for `version`, `edition`, `license`; inherit `serde`, `tracing`, `thiserror`, `hpx`, `tokio`, `serde_json`; add crates.io metadata fields (`description`, `repository`, `homepage`, `documentation`, `readme`, `keywords`, `categories`, `rust-version`).
- [x] **Step 2:** Create `crates/circle-buidl-wallets/src/lib.rs` with crate-level doc comment, `#![deny(missing_docs)]`, and module re-exports (`pub mod client; pub mod models; pub mod error; pub use client::BuidlWalletsClient; pub use error::Error;`).
- [x] **Step 3:** Create `crates/circle-buidl-wallets/src/error.rs` with `pub enum Error` skeleton (variants: `Http`, `Api`, `Deserialize`, `InvalidParam`).
- [x] **Step 4:** Create `crates/circle-buidl-wallets/src/models/mod.rs`, `common.rs`, `transfer.rs`, `user_op.rs`, `wallet.rs` — empty `pub mod` stubs.
- [x] **Step 5:** Create `crates/circle-buidl-wallets/src/client.rs` — empty `pub struct BuidlWalletsClient` stub.
- [x] **Step 6:** Create `crates/circle-buidl-wallets/tests/integration.rs` — empty file with `// Integration tests — requires Prism on port 4010`.
- [x] **Step 7:** Create `crates/circle-buidl-wallets/README.md` with one-paragraph description.
- [x] **Verification:** `cargo check -p circle-buidl-wallets` exits 0.

---

### Task 1.4: Scaffold circle-compliance Crate ✅

> **Context:** Same pattern as Task 1.3 but for the Compliance Engine API. Port 4011 for Prism.
> **Verification:** `cargo check -p circle-compliance` exits 0.

- **Priority:** P0
- **Scope:** Crate scaffolding
- **Status:** 🟢 DONE

- [x] **Step 1:** Create `crates/circle-compliance/Cargo.toml`
- [x] **Step 2:** Create `src/lib.rs`, `src/error.rs`, `src/client.rs` (`ComplianceClient`), `src/models/mod.rs`, `src/models/screening.rs`, `src/models/common.rs`.
- [x] **Step 3:** Create `tests/integration.rs` stub.
- [x] **Step 4:** Create `README.md`.
- [x] **Verification:** `cargo check -p circle-compliance` exits 0.

---

### Task 1.5: Scaffold circle-developer-controlled-wallets Crate ✅

> **Context:** Same pattern as Task 1.3 but for the Developer-Controlled Wallets API (the largest of the four specs, 3500+ lines). Port 4012 for Prism.
> **Verification:** `cargo check -p circle-developer-controlled-wallets` exits 0.

- **Priority:** P0
- **Scope:** Crate scaffolding
- **Status:** 🟢 DONE

- [x] **Step 1:** Create `crates/circle-developer-controlled-wallets/Cargo.toml`
- [x] **Step 2:** Create `src/lib.rs`, `src/error.rs`, `src/client.rs` (`DeveloperWalletsClient`), `src/models/` with `mod.rs`, `wallet_set.rs`, `wallet.rs`, `transaction.rs`, `signing.rs`, `token.rs`, `common.rs`.
- [x] **Step 3:** Create `tests/integration.rs` stub.
- [x] **Step 4:** Create `README.md`.
- [x] **Verification:** `cargo check -p circle-developer-controlled-wallets` exits 0.

---

### Task 1.6: Scaffold circle-user-controlled-wallets Crate ✅

> **Context:** Same pattern as Task 1.3 but for the User-Controlled Wallets API (3700+ lines, includes PIN auth, social auth, challenges). Port 4013 for Prism.
> **Verification:** `cargo check -p circle-user-controlled-wallets` exits 0.

- **Priority:** P0
- **Scope:** Crate scaffolding
- **Status:** 🟢 DONE

- [x] **Step 1:** Create `crates/circle-user-controlled-wallets/Cargo.toml`
- [x] **Step 2:** Create `src/lib.rs`, `src/error.rs`, `src/client.rs` (`UserWalletsClient`), `src/models/` with `mod.rs`, `user.rs`, `challenge.rs`, `token.rs`, `wallet.rs`, `transaction.rs`, `signing.rs`, `common.rs`.
- [x] **Step 3:** Create `tests/integration.rs` stub.
- [x] **Step 4:** Create `README.md`.
- [x] **Verification:** `cargo check -p circle-user-controlled-wallets` exits 0.

---

### Task 1.7: Scaffold circle-cli Binary Crate ✅

> **Context:** `bin/circle-cli` is the CLI crate. It depends on all four library crates and uses `clap` for argument parsing and `eyre` for error propagation. The directory must be registered as a workspace member (already covered by `members = ["bin/*", "crates/*"]`).
> **Verification:** `cargo check -p circle-cli` exits 0; `cargo run -p circle-cli -- --help` shows the top-level help text.

- **Priority:** P0
- **Scope:** CLI binary scaffolding
- **Status:** � DONE

- [x] **Step 1:** Create `bin/circle-cli/Cargo.toml` with `[[bin]]` entry, `name = "circle-cli"`, inherits `version`/`edition`; depends on `clap` with `features = ["derive"]`, `eyre`, `tokio` with `features = ["rt-block-on", "macros"]`, `tracing`, `tracing-subscriber`, and all four local library crates.
- [x] **Step 2:** Create `bin/circle-cli/src/main.rs` with a `#[tokio::main]` entry point, `clap::Parser`-derived `Cli` struct with `--api-key`, `--base-url`, `--output` flags, and four subcommand variants (`Buidl`, `Compliance`, `Developer`, `User`).
- [x] **Step 3:** Create `bin/circle-cli/src/commands/mod.rs`, `buidl.rs`, `compliance.rs`, `developer.rs`, `user.rs` — empty stubs that print `"not yet implemented"`.
- [x] **Step 4:** Create `bin/circle-cli/src/output.rs` — `pub enum OutputFormat { Text, Json }` and a `print_result` helper.
- [x] **Step 5:** Create `bin/circle-cli/README.md`.
- [x] **Verification:** `cargo check -p circle-cli` exits 0; `cargo run -p circle-cli -- --help` shows usage.

---

## Phase 2: Core Logic

### Task 2.1: Implement circle-buidl-wallets Models and Client

> **Context:** Based on `docs/wallets/buidl-wallets.yaml`. The spec defines three tag groups: Transfers (2 ops), UserOps (3 ops), Wallets (3 ops). Implement all request/response model types, then implement `BuidlWalletsClient` with one async method per operation. Add unit tests for JSON round-trips and request construction.
> **Verification:** `cargo test -p circle-buidl-wallets` exits 0 with all unit tests passing.

- **Priority:** P0
- **Scope:** Full library crate implementation
- **Status:** 🔴 TODO

- [ ] **Step 1:** Implement `models/common.rs`: `Blockchain` enum (all values from spec), `PaginationParams`, `ApiError`, `PageInfo` (for pagination metadata), `ApiKey` (no Debug/Display).
- [ ] **Step 2:** Implement `models/transfer.rs`: `Transfer`, `Transfers`, `TransferState` enum, `TransferType` enum, `ListTransfersParams`, `TxHash`, `UserOpHash`, `Addresses`, `TransferId`.
- [ ] **Step 3:** Implement `models/user_op.rs`: `UserOp`, `UserOps`, `UserOpState` enum, `ListUserOpsParams`, `CreateUserOpRequest`, `ReferenceId`.
- [ ] **Step 4:** Implement `models/wallet.rs`: `Wallet`, `Wallets` (buidl-specific wallet types as defined in the spec schemas).
- [ ] **Step 5:** Implement `BuidlWalletsClient` in `client.rs`: `new`, `with_base_url`, and eight async methods — `list_transfers`, `get_transfer`, `list_user_ops`, `get_user_op`, `create_user_op`, and the buidl-wallets wallet operations. Each method adds `Authorization`, `X-Request-Id` headers, sends via `hpx`, dispatches on status code to `Ok`/`Err(Error::Api)`.
- [ ] **Step 6:** Add unit tests in each model file: at least one JSON fixture per struct/enum demonstrating deserialization. Add unit tests in `client.rs` for request URL construction using a mock HTTP server (or by inspecting the built URL directly).
- [ ] **Verification:** `cargo test -p circle-buidl-wallets --all-features` exits 0.

---

### Task 2.2: Implement circle-compliance Models and Client

> **Context:** Based on `docs/wallets/compliance.yaml`. The spec defines one endpoint: `POST /v1/w3s/compliance/screening/addresses`. Models: `Address`, `Chain` enum, `BlockchainAddressScreeningResponse`, `BaseScreeningDecision`, `RiskAction` enum, `RiskScore` enum.
> **Verification:** `cargo test -p circle-compliance` exits 0.

- **Priority:** P0
- **Scope:** Full library crate implementation
- **Status:** 🔴 TODO

- [ ] **Step 1:** Implement `models/common.rs`: `Chain` enum (30+ values from spec: ETH, ETH-SEPOLIA, MATIC, etc.), `IdempotencyKey` type alias (`String`), `ApiError`.
- [ ] **Step 2:** Implement `models/screening.rs`: `Address` (request body: `idempotency_key`, `address`, `chain`), `RiskAction` enum (APPROVE, REVIEW, FREEZE_WALLET, DENY), `RiskScore` enum (UNKNOWN, LOW, MEDIUM, HIGH), `BaseScreeningDecision`, `BlockchainAddressScreeningResponse` (full response wrapper).
- [ ] **Step 3:** Implement `ComplianceClient` in `client.rs`: `new`, `with_base_url`, `screen_address(req: &Address) -> Result<BlockchainAddressScreeningResponse, Error>`.
- [ ] **Step 4:** Add unit tests: JSON round-trip for `Address` and `BlockchainAddressScreeningResponse`, enum serialization for `RiskAction` and `RiskScore`.
- [ ] **Verification:** `cargo test -p circle-compliance --all-features` exits 0.

---

### Task 2.3: Implement circle-developer-controlled-wallets Models and Client

> **Context:** Based on `docs/wallets/developer-controlled-wallets.yaml` (3504 lines). Operations span five tag groups: Wallet Sets (4 ops), Wallets (8+ ops including balance listing), Signing (2 ops), Transactions (4+ ops), Token Lookup (1+ ops). This is the most complex crate. Implement models first, then client methods, then unit tests.
> **Verification:** `cargo test -p circle-developer-controlled-wallets` exits 0.

- **Priority:** P0
- **Scope:** Full library crate implementation
- **Status:** 🔴 TODO

- [ ] **Step 1:** Implement `models/common.rs`: `Blockchain` enum, `PaginationParams`, `ApiError`, `ApiKey`, shared date/time types (`DateTime<Utc>` via `chrono` — add `chrono` to workspace deps).
- [ ] **Step 2:** Implement `models/wallet_set.rs`: `WalletSet`, `WalletSets`, `WalletSetResponse`, `CreateWalletSetRequest`, `UpdateWalletSetRequest`.
- [ ] **Step 3:** Implement `models/wallet.rs`: `Wallet`, `Wallets`, `WalletResponse`, `WalletBalance`, `WalletBalances`, `CreateWalletRequest`, `UpdateWalletRequest`, `ListWalletsParams`, `ListWalletsWithBalancesParams`.
- [ ] **Step 4:** Implement `models/transaction.rs`: `Transaction`, `Transactions`, `TransactionState` enum, `CreateTransactionRequest`, `EstimateFeeRequest`, `EstimateFeeResponse`, `ListTransactionsParams`.
- [ ] **Step 5:** Implement `models/signing.rs`: `SignMessageRequest`, `SignTypedDataRequest`, `SignedMessageResponse`.
- [ ] **Step 6:** Implement `models/token.rs`: `Token`, `TokenLookupResponse`.
- [ ] **Step 7:** Implement `DeveloperWalletsClient` with all async methods: Wallet Set operations (4), Wallet operations (8+), Transaction operations (4+), Signing operations (2), Token Lookup operations (1+). Each method follows the standard request lifecycle from design §4.4.
- [ ] **Step 8:** Add unit tests: JSON fixtures per model struct, error variant tests, pagination param serialization tests.
- [ ] **Verification:** `cargo test -p circle-developer-controlled-wallets --all-features` exits 0.

---

### Task 2.4: Implement circle-user-controlled-wallets Models and Client

> **Context:** Based on `docs/wallets/user-controlled-wallets.yaml` (3777 lines). Operations span seven tag groups: Users (6+ ops), PIN Authentication (3+ ops), Social/Email Authentication (2+ ops), Wallets (4+ ops), Transactions (4+ ops), Token Lookup, Signing. User-token-based requests require an `X-User-Token` header in addition to the bearer key.
> **Verification:** `cargo test -p circle-user-controlled-wallets` exits 0.

- **Priority:** P0
- **Scope:** Full library crate implementation
- **Status:** 🔴 TODO

- [ ] **Step 1:** Implement `models/common.rs`: `Blockchain` enum, `PaginationParams`, `ApiError`, `ApiKey`, `UserToken` (no Debug).
- [ ] **Step 2:** Implement `models/user.rs`: `User`, `Users`, `UserResponse`, `GetUserByIDResponse`, `CreateUserRequest`, `ListUsersParams`, `PinStatus` enum, `SecurityQuestionStatus` enum.
- [ ] **Step 3:** Implement `models/challenge.rs`: `Challenge`, `Challenges`, `ChallengeResponse`, `Pin`, `ChallengeStatus` enum.
- [ ] **Step 4:** Implement `models/token.rs`: `UserTokenRequest`, `UserTokenResponse`.
- [ ] **Step 5:** Implement `models/wallet.rs`: `Wallet`, `Wallets`, user-wallet-specific create/update/list params.
- [ ] **Step 6:** Implement `models/transaction.rs`: `Transaction`, `Transactions`, user transaction request/response types.
- [ ] **Step 7:** Implement `models/signing.rs`: user signing request/response types.
- [ ] **Step 8:** Implement `UserWalletsClient` in `client.rs`: `new`, `with_base_url`. Methods accepting `user_token: &str` must add `X-User-Token` header. Implement all async methods across all seven tag groups.
- [ ] **Step 9:** Add unit tests: JSON fixtures, header construction assertions, enum serialization.
- [ ] **Verification:** `cargo test -p circle-user-controlled-wallets --all-features` exits 0.

---

## Phase 3: Integration & Features

### Task 3.1: Write Integration Tests for circle-buidl-wallets

> **Context:** `tests/integration.rs` in `circle-buidl-wallets` exercises every API method against Prism mock on `http://localhost:4010`. Uses `BuidlWalletsClient::with_base_url("http://localhost:4010", "test_key")`. Each test is `#[tokio::test]` and asserts the method returns `Ok(...)` for Prism's mock response.
> The integration test binary is only compiled/run via `just integration-test` to avoid failures when Prism is not running.
> **Verification:** `just integration-test` completes with 0 failures (Prism servers running).

- **Priority:** P0
- **Scope:** Integration test harness — buidl wallets
- **Status:** 🔴 TODO

- [ ] **Step 1:** Add `[dev-dependencies]` to `crates/circle-buidl-wallets/Cargo.toml`: `tokio` with `features = ["macros", "rt"]`.
- [ ] **Step 2:** In `tests/integration.rs`, write a `#[tokio::test]` for each client method: `test_list_transfers`, `test_get_transfer`, `test_list_user_ops`, `test_get_user_op`, `test_create_user_op`, and buidl wallet ops.
- [ ] **Step 3:** For list operations, assert `result.is_ok()` and that returned data type matches expected struct shape.
- [ ] **Step 4:** Skip tests when `PRISM_BUIDL_PORT` is unset or Prism not reachable (skip macro or env check at test start to keep `just test` green).
- [ ] **Verification:** `just integration-test` and check that `circle-buidl-wallets` integration tests report 0 failures.

---

### Task 3.2: Write Integration Tests for circle-compliance

> **Context:** `tests/integration.rs` in `circle-compliance` against Prism on port 4011. One meaningful test: `test_screen_address` calls `screen_address` with a valid `Address` body and asserts `Ok(BlockchainAddressScreeningResponse)`.
> **Verification:** `just integration-test` completes with 0 failures.

- **Priority:** P0
- **Scope:** Integration test harness — compliance
- **Status:** 🔴 TODO

- [ ] **Step 1:** Add `tokio` dev-dependency to `circle-compliance`.
- [ ] **Step 2:** Write `test_screen_address` test using a fixed UUID idempotency key, address `0x1bf9ad0cc2ad298c69a2995aa806ee832788218c`, chain `Chain::EthSepolia`.
- [ ] **Step 3:** Assert response is `Ok(...)` and `screening_date` is populated.
- [ ] **Step 4:** Add skip guard for when Prism is not Running.
- [ ] **Verification:** `just integration-test` passes for `circle-compliance`.

---

### Task 3.3: Write Integration Tests for circle-developer-controlled-wallets

> **Context:** `tests/integration.rs` in `circle-developer-controlled-wallets` against Prism on port 4012. Cover at least one test per tag group: wallet sets, wallets (create + list + get + update + balances), transactions, signing, token lookup.
> **Verification:** `just integration-test` completes with 0 failures.

- **Priority:** P0
- **Scope:** Integration test harness — developer wallets
- **Status:** 🔴 TODO

- [ ] **Step 1:** Add `tokio` dev-dependency, write `test_get_wallet_sets`, `test_create_wallet_set`, `test_update_wallet_set`.
- [ ] **Step 2:** Write `test_create_wallet`, `test_get_wallets`, `test_get_wallet`, `test_update_wallet`, `test_list_wallets_with_balances`.
- [ ] **Step 3:** Write `test_create_transaction`, `test_list_transactions`, `test_get_transaction`, `test_estimate_fee`.
- [ ] **Step 4:** Write `test_sign_message`, `test_sign_typed_data`, `test_token_lookup`.
- [ ] **Step 5:** Add skip guards and assert `is_ok()` on all results.
- [ ] **Verification:** `just integration-test` passes for `circle-developer-controlled-wallets`.

---

### Task 3.4: Write Integration Tests for circle-user-controlled-wallets

> **Context:** `tests/integration.rs` in `circle-user-controlled-wallets` against Prism on port 4013. Cover at least one test per tag group: users, challenges, PIN auth, user token, wallets, transactions, signing, token lookup. User-token tests use a placeholder token string.
> **Verification:** `just integration-test` completes with 0 failures.

- **Priority:** P0
- **Scope:** Integration test harness — user wallets
- **Status:** 🔴 TODO

- [ ] **Step 1:** Add `tokio` dev-dependency, write `test_create_user`, `test_list_users`, `test_get_user`, `test_get_user_by_token`.
- [ ] **Step 2:** Write `test_get_user_token`, `test_create_user_pin_challenge`, `test_create_user_with_pin_challenge`.
- [ ] **Step 3:** Write `test_list_challenges`, `test_get_challenge`.
- [ ] **Step 4:** Write `test_user_wallets`, `test_user_transactions`, `test_user_signing`, `test_user_token_lookup`.
- [ ] **Step 5:** Add skip guards, assert `is_ok()`.
- [ ] **Verification:** `just integration-test` passes for `circle-user-controlled-wallets`.

---

### Task 3.5: Implement circle-cli Subcommands

> **Context:** `bin/circle-cli/src/commands/*.rs` are currently stubs. Each subcommand module needs to define all the `clap`-annotated sub-subcommands corresponding to the library crate's API methods and run them by constructing the appropriate client and calling the method.
> **Verification:** `cargo run -p circle-cli -- buidl list-transfers --wallet-addresses 0x123 --base-url http://localhost:4010 --api-key test` exits 0 and prints JSON.

- **Priority:** P1
- **Scope:** CLI binary — full subcommand implementation
- **Status:** 🔴 TODO

- [ ] **Step 1:** In `commands/buidl.rs`, define `BuidlSubcommand` enum with variants matching every `BuidlWalletsClient` method. Each variant holds the typed CLI arguments (e.g., `ListTransfers { wallet_addresses: Vec<String>, blockchain: Option<String>, ... }`).
- [ ] **Step 2:** Implement `run_buidl(cmd: BuidlSubcommand, client: BuidlWalletsClient, output: OutputFormat)` dispatching to the appropriate client method and printing via `output.rs`.
- [ ] **Step 3:** In `commands/compliance.rs`, define `ComplianceSubcommand` with `ScreenAddress { address, chain, idempotency_key }` variant and implement `run_compliance`.
- [ ] **Step 4:** In `commands/developer.rs`, define `DeveloperSubcommand` covering all `DeveloperWalletsClient` operations and implement `run_developer`.
- [ ] **Step 5:** In `commands/user.rs`, define `UserSubcommand` covering all `UserWalletsClient` operations (including `--user-token` flag for user-token-scoped operations) and implement `run_user`.
- [ ] **Step 6:** In `main.rs`, dispatch from the top-level `Cli.command` to the four `run_*` functions. Read `CIRCLE_API_KEY` env var as fallback for `--api-key`.
- [ ] **Step 7:** In `output.rs`, implement `print_result<T: serde::Serialize + std::fmt::Display>(val: T, format: OutputFormat)` — prints JSON via `serde_json::to_string_pretty` or human-readable via `Display`.
- [ ] **Verification:** `cargo run -p circle-cli -- --help` shows all four subcommands; `cargo run -p circle-cli -- compliance --help` shows `screen-address`; running each subcommand against a Prism base URL exits 0.

---

## Phase 4: Polish, QA & Docs

### Task 4.1: Add Doc-Comments to All Public Items

> **Context:** The workspace `[workspace.lints.rust]` includes `missing_docs = "warn"` and the workspace Clippy config denies `allow_attributes`, meaning `#![allow(missing_docs)]` is forbidden. Every public struct, enum, field, method, and module in all four library crates and the CLI must have a `///` doc-comment. The `cargo doc` command must produce zero warnings.
> **Verification:** `cargo doc --all-features --no-deps 2>&1 | grep -c "warning"` outputs `0`.

- **Priority:** P1
- **Scope:** Documentation coverage
- **Status:** 🔴 TODO

- [ ] **Step 1:** For `circle-buidl-wallets`: add `///` comments to all items in `lib.rs`, `client.rs`, `error.rs`, `models/*.rs`.
- [ ] **Step 2:** For `circle-compliance`: same.
- [ ] **Step 3:** For `circle-developer-controlled-wallets`: same.
- [ ] **Step 4:** For `circle-user-controlled-wallets`: same.
- [ ] **Step 5:** For `circle-cli`: add `///` / `//!` comments to all public commands, flags, and modules.
- [ ] **Verification:** `cargo doc --all-features --no-deps` exits 0 with no warnings.

---

### Task 4.2: Complete crates.io Metadata in All Cargo.toml Files

> **Context:** All five `Cargo.toml` files (four library crates + CLI binary) need the full set of crates.io publishing fields documented in design §7 Publishing Checklist. The root workspace `Cargo.toml` already has `license = "Apache-2.0"` and `version = "0.1.0"` but is missing `repository`. Each per-crate `Cargo.toml` must add the remaining fields not inheritable from the workspace (`description`, `keywords`, `categories`, `documentation`, `homepage`, `readme`, `rust-version`).
> **Verification:** `cargo package -p circle-buidl-wallets --no-verify --list` lists all expected files; `cargo publish --dry-run -p circle-compliance` exits 0 (optional, needs network).

- **Priority:** P1
- **Scope:** Publishing metadata
- **Status:** 🔴 TODO

- [ ] **Step 1:** Update root `Cargo.toml` `[workspace.package]`: add `repository = "https://github.com/longcipher/circle-sdk-rs"`.
- [ ] **Step 2:** In `crates/circle-buidl-wallets/Cargo.toml`: add `description`, `keywords = ["circle", "web3", "wallet", "blockchain", "sdk"]`, `categories = ["api-bindings", "web-programming::http-client"]`, `documentation`, `homepage`, `readme = "README.md"`, `rust-version = "1.85"`.
- [ ] **Step 3:** Same for `circle-compliance` (adjust keywords/description).
- [ ] **Step 4:** Same for `circle-developer-controlled-wallets`.
- [ ] **Step 5:** Same for `circle-user-controlled-wallets`.
- [ ] **Step 6:** Same for `circle-cli` (binary crate; categories = `["command-line-utilities"]`).
- [ ] **Verification:** `cargo package -p circle-buidl-wallets --no-verify --list` includes `README.md`, `src/lib.rs`, and `Cargo.toml`.

---

### Task 4.3: Write per-Crate README.md Files

> **Context:** crates.io displays `README.md` as the crate landing page. Each README should include: a one-paragraph description, installation snippet, quick-start code example, link to API docs on docs.rs, and a link back to the workspace repository.
> **Verification:** `markdownlint crates/*/README.md bin/circle-cli/README.md` exits 0 (or manual review if markdownlint is not installed).

- **Priority:** P1
- **Scope:** End-user documentation
- **Status:** 🔴 TODO

- [ ] **Step 1:** Write `crates/circle-buidl-wallets/README.md` with crate description, `Cargo.toml` snippet, async usage example, and links.
- [ ] **Step 2:** Write `crates/circle-compliance/README.md` with similar structure, focus on address screening example.
- [ ] **Step 3:** Write `crates/circle-developer-controlled-wallets/README.md` with wallet set creation and wallet listing examples.
- [ ] **Step 4:** Write `crates/circle-user-controlled-wallets/README.md` with user creation and token example.
- [ ] **Step 5:** Update `bin/circle-cli/README.md` with installation instructions (`cargo install circle-cli`), environment variables table, and example commands for all four subcommands.
- [ ] **Update** the root `README.md` to list all four crates and the CLI with one-line descriptions and links.
- [ ] **Verification:** All README files render correctly; no broken links to local files.

---

### Task 4.4: Final Lint, Test, and Format Pass

> **Context:** Before considering the spec implemented, run the full quality checklist from the project instructions. This ensures `just format`, `just lint`, `just test`, and `just integration-test` all pass cleanly. This task captures any issues accumulated across Phases 1–3.
> **Verification:** All four commands exit 0 with no warnings or failures.

- **Priority:** P2
- **Scope:** Quality assurance
- **Status:** 🔴 TODO

- [ ] **Step 1:** Run `just format` — fix any formatting issues in Rust, TOML, and Markdown files.
- [ ] **Step 2:** Run `just lint` — address all Clippy warnings (particularly `missing_docs`, `unwrap_used`, `expect_used`, `print_stdout`). Fix without suppressing via `allow`.
- [ ] **Step 3:** Run `just test` — ensure all unit tests pass.
- [ ] **Step 4:** Run `just integration-test` — ensure all integration tests pass with Prism.
- [ ] **Step 5:** Run `cargo doc --all-features --no-deps` — fix any remaining doc warnings.
- [ ] **Step 6:** Run `cargo machete` — remove any unused dependencies.
- [ ] **Verification:** All six commands exit 0 with zero warnings/failures.

---

## Summary & Timeline

| Phase                  | Tasks | Target Date |
| :--------------------- | :---: | :---------- |
| **1. Foundation**      |   7   | TBD         |
| **2. Core Logic**      |   4   | TBD         |
| **3. Integration**     |   5   | TBD         |
| **4. Polish**          |   4   | TBD         |
| **Total**              | **20**|             |

## Definition of Done

1. [ ] **Linted:** `just lint` exits 0 — no clippy warnings, no typos, no unused deps.
2. [ ] **Tested:** `just test` exits 0 — all unit tests pass; coverage ≥ 80% on model and client code.
3. [ ] **Formatted:** `just format` exits 0 — all code, TOML, and Markdown formatted.
4. [ ] **Integration Verified:** `just integration-test` exits 0 — all integration tests pass against Prism mocks.
5. [ ] **Documented:** `cargo doc --all-features --no-deps` exits 0 with zero warnings.
6. [ ] **CLI Works:** `circle-cli --help` and all four subcommand `--help` texts print without error.
7. [ ] **Publishable:** `cargo package --no-verify` succeeds for all four library crates.
