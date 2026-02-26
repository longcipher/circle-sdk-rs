# Default recipe to display help
default:
  @just --list

# Format all code
format:
  rumdl fmt .
  taplo fmt
  cargo +nightly fmt --all

# Auto-fix linting issues
fix:
  rumdl check --fix .

# Run all lints
lint:
  typos
  rumdl check .
  taplo fmt --check
  cargo +nightly fmt --all -- --check
  cargo +nightly clippy --all -- -D warnings
  cargo machete

# Run tests
test:
  cargo test --all-features

# Run tests with coverage
test-coverage:
  cargo tarpaulin --all-features --workspace --timeout 300

# Build entire workspace
build:
  cargo build --workspace

# Check all targets compile
check:
  cargo check --all-targets --all-features

# Check for Chinese characters
check-cn:
  rg --line-number --column "\p{Han}"

# Full CI check
ci: lint test build

# ============================================================
# Maintenance & Tools
# ============================================================

# Clean build artifacts
clean:
  cargo clean

# Install all required development tools
setup:
  cargo install cargo-leptos
  cargo install cargo-machete
  cargo install taplo-cli
  cargo install typos-cli
  cargo install leptosfmt

# Generate documentation for the workspace
docs:
  cargo doc --no-deps --open

# ============================================================
# Circle SDK — Prism Mock Server Recipes
# ============================================================

# Start Prism mock server for Buidl Wallets API (port 4010, runs in foreground)
prism-buidl:
  npx --yes @stoplight/prism-cli mock docs/wallets/buidl-wallets.yaml --port 4010

# Start Prism mock server for Compliance Engine API (port 4011, runs in foreground)
prism-compliance:
  npx --yes @stoplight/prism-cli mock docs/wallets/compliance.yaml --port 4011

# Start Prism mock server for Developer-Controlled Wallets API (port 4012, runs in foreground)
prism-developer:
  npx --yes @stoplight/prism-cli mock docs/wallets/developer-controlled-wallets.yaml --port 4012

# Start Prism mock server for User-Controlled Wallets API (port 4013, runs in foreground)
prism-user:
  npx --yes @stoplight/prism-cli mock docs/wallets/user-controlled-wallets.yaml --port 4013

# Start all four Prism mock servers in the background (for manual dev use)
prism-all:
  #!/usr/bin/env bash
  set -euo pipefail
  npx --yes @stoplight/prism-cli mock docs/wallets/buidl-wallets.yaml --port 4010 &
  echo "Buidl Wallets Prism server started on port 4010 (PID $!)"
  npx --yes @stoplight/prism-cli mock docs/wallets/compliance.yaml --port 4011 &
  echo "Compliance Prism server started on port 4011 (PID $!)"
  npx --yes @stoplight/prism-cli mock docs/wallets/developer-controlled-wallets.yaml --port 4012 &
  echo "Developer Wallets Prism server started on port 4012 (PID $!)"
  npx --yes @stoplight/prism-cli mock docs/wallets/user-controlled-wallets.yaml --port 4013 &
  echo "User Wallets Prism server started on port 4013 (PID $!)"
  echo "All Prism servers started. Stop with: pkill -f prism-cli"

# Run integration tests against Prism mock servers
# Starts all four Prism servers, waits for readiness, runs cargo integration tests, then cleans up.
integration-test:
  #!/usr/bin/env bash
  set -euo pipefail

  echo "==> Starting Prism mock servers..."
  npx --yes @stoplight/prism-cli mock docs/wallets/buidl-wallets.yaml --port 4010 --errors 2>/dev/null &
  PRISM_PID1=$!
  npx --yes @stoplight/prism-cli mock docs/wallets/compliance.yaml --port 4011 --errors 2>/dev/null &
  PRISM_PID2=$!
  npx --yes @stoplight/prism-cli mock docs/wallets/developer-controlled-wallets.yaml --port 4012 --errors 2>/dev/null &
  PRISM_PID3=$!
  npx --yes @stoplight/prism-cli mock docs/wallets/user-controlled-wallets.yaml --port 4013 --errors 2>/dev/null &
  PRISM_PID4=$!

  cleanup() {
    echo "==> Stopping Prism mock servers..."
    kill "$PRISM_PID1" "$PRISM_PID2" "$PRISM_PID3" "$PRISM_PID4" 2>/dev/null || true
  }
  trap cleanup EXIT

  echo "==> Waiting 8s for Prism servers to be ready..."
  sleep 8

  echo "==> Running integration tests..."
  PRISM_BUIDL_PORT=4010 \
  PRISM_COMPLIANCE_PORT=4011 \
  PRISM_DEVELOPER_PORT=4012 \
  PRISM_USER_PORT=4013 \
  cargo test --test integration --all-features --workspace
