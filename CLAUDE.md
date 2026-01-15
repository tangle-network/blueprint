# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Tangle Network Blueprint SDK - a Rust workspace for building decentralized blockchain services ("blueprints") on Tangle Network (EVM), EigenLayer, and standard EVM chains. Blueprints turn complex on-chain and off-chain infrastructure into reproducible, deployable units of logic.

The v2 architecture is EVM-only, removing all Substrate dependencies in favor of `tnt-core` v2 contracts.

## Essential Commands

### Build & Test
```bash
# Build entire workspace
cargo build

# Run all tests
cargo test

# Run tests with nextest (preferred)
cargo nextest run --profile ci

# Run a single test
cargo test -p <crate_name> --test <test_name> -- --nocapture

# Run serial tests (for crates requiring single-threaded execution)
cargo nextest run --profile serial -p <crate_name>

# Format code (requires nightly)
cargo +nightly fmt

# Lint code
cargo clippy --tests --examples -- -D warnings
```

### Anvil-Based Integration Tests
Tests require Docker for `testcontainers`. Set `RUN_TNT_E2E=1` for longer integration suites.

```bash
# Client integration tests
cargo test -p blueprint-client-tangle-evm --test anvil

# Blueprint harness end-to-end
cargo test -p hello-tangle-blueprint --test anvil -- --nocapture

# Manager runner tests
cargo test -p blueprint-manager --test tangle_evm_runner

# Pricing engine tests
cargo test -p blueprint-pricing-engine --test evm_listener
```

### CLI Tool
```bash
# Install the CLI
cargo install cargo-tangle --git https://github.com/tangle-network/blueprint --force

# Create new blueprint
cargo tangle blueprint create --name <blueprint_name>

# Deploy to devnet (auto-starts local testnet)
cargo tangle blueprint deploy tangle --network devnet --package <package_name>

# Deploy to testnet/mainnet using a definition manifest
cargo tangle blueprint deploy tangle --network testnet --definition ./path/to/definition.json

# Generate keys
cargo tangle blueprint generate-keys -k <KEY_TYPE> -p <PATH> -s <SURI/SEED>

# Run blueprint
cargo tangle blueprint run --protocol tangle-evm --http-rpc-url <URL> --ws-rpc-url <URL> --keystore-path <PATH>
```

## Architecture

### Job System
The SDK centers around a job system:
- `blueprint-core`: Job primitives and definitions
- `blueprint-router`: Dynamic job scheduling and routing
- `blueprint-runner`: Protocol-specific job execution
- `blueprint-manager`: Network connection layer, spawns blueprint binaries

### Network Clients (v2 EVM-only)
- `blueprint-client-tangle-evm`: Tangle v2 EVM contracts (`Tangle.sol`, `MultiAssetDelegation.sol`, `OperatorStatusRegistry.sol`)
- `blueprint-client-eigenlayer`: EigenLayer AVS integration
- `blueprint-client-evm`: Generic EVM utilities

### Key Crate Groups
- **Crypto** (`crates/crypto/`): BLS, BN254, Ed25519, K256, Sr25519 implementations
- **Networking** (`crates/networking/`): P2P with libp2p, round-based MPC protocol extensions
- **Testing** (`crates/testing-utils/`): Anvil harness, core utilities, EigenLayer test utilities
- **Chain Setup** (`crates/chain-setup/anvil/`): Anvil testnet snapshots and deployment

### Meta-crates Pattern
Major functionality exposed through re-export crates:
- `blueprint-sdk`: Main entry point, re-exports all SDK components
- `blueprint-clients`: Network client collection
- `blueprint-crypto`: All cryptographic schemes
- `blueprint-testing-utils`: All test utilities

## Development Constraints

### Rust Configuration
- Edition: 2024
- Rust version: 1.88
- Workspace resolver: v3
- Nightly required for formatting only

### Linting
Clippy pedantic enabled. Key denials: `rust_2018_idioms`, `trivial_casts`, `unused_import_braces`.

### Serial Test Crates
These require `--test-threads=1` or nextest `--profile serial`:
- `blueprint-client-evm`
- `blueprint-networking`
- `blueprint-qos`
- `cargo-tangle`

### Test Fixtures
- Anvil snapshot: `crates/chain-setup/anvil/snapshots/localtestnet-state.json`
- Fallback broadcast: `crates/chain-setup/anvil/snapshots/localtestnet-broadcast.json`
- Refresh with: `scripts/fetch-localtestnet-fixtures.sh`

### External Dependencies
- **Alloy**: EVM interaction (v1.0.x)
- **libp2p**: P2P networking (v0.54)
- **Eigensdk**: EigenLayer integration (v2.0)
- **Foundry**: Smart contract compilation
- **tnt-core-bindings**: Tangle v2 contract bindings
