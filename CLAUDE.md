# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Tangle Network Blueprint SDK - a Rust workspace for building blockchain services ("blueprints") on Tangle Network, EigenLayer, and EVM networks. The project is in alpha stage with 40+ crates organized in a modular architecture.

## Essential Commands

### Build & Test
```bash
# Build entire workspace
cargo build

# Run all tests
cargo test

# Run tests with nextest (preferred in CI)
cargo nextest run --profile ci

# Format code
cargo fmt

# Lint code
cargo clippy -- -D warnings
cargo clippy --tests --examples -- -D warnings
```

### CLI Tool
```bash
# Install the CLI
cargo install cargo-tangle --git https://github.com/tangle-network/blueprint --force

# Create new blueprint
cargo tangle blueprint create --name <blueprint_name>

# Deploy to devnet (auto-starts local testnet)
cargo tangle blueprint deploy tangle --devnet --package <package_name>

# Generate keys
cargo tangle blueprint generate-keys -k <KEY_TYPE> -p <PATH> -s <SURI/SEED>
```

## Architecture

### Core Abstraction: Job System
The blueprint SDK centers around a job system that provides:
- Multi-network execution contexts (Tangle, EigenLayer, EVM)
- Dynamic routing and scheduling via `blueprint-router`
- Protocol-specific execution via `blueprint-runner`
- Network management via `blueprint-manager`

### Workspace Structure
```
cli/                    # cargo-tangle CLI tool
crates/
├── blueprint-sdk/      # Main SDK re-export crate
├── blueprint-core/     # Job system primitives
├── blueprint-runner/   # Job execution engine
├── blueprint-router/   # Dynamic job scheduling
├── blueprint-manager/  # Network connection layer
├── blueprint-clients/  # Network-specific clients (Tangle, EigenLayer, EVM)
├── blueprint-crypto/   # Multi-scheme cryptography (BLS, BN254, Ed25519, K256, Sr25519)
├── blueprint-networking/ # P2P with MPC protocol extensions
├── blueprint-testing-utils/ # Test utilities for different networks
└── blueprint-stores/   # Local database implementations
examples/               # Example blueprints (incredible-squaring)
```

### Key Architectural Patterns

1. **Meta-crates Pattern**: Major functionality is exposed through meta-crates that re-export component crates
2. **Multi-Network Abstraction**: Common interfaces with network-specific implementations in separate crates
3. **Hardware Wallet Support**: Built-in support for Ledger and remote signing (AWS KMS, GCP)
4. **Test Isolation**: Certain crates require serial test execution due to resource conflicts

## Development Constraints

### Rust Configuration
- Version: 1.86 (2024 edition)
- Workspace resolver: v3
- Nightly features used for formatting

### Linting Rules
- Clippy pedantic enabled with specific allowances
- Deny: rust_2018_idioms, trivial_casts, unused_import_braces
- Custom doc-valid-idents including "EigenLayer"

### Serial Test Crates
These require `--test-threads=1` or nextest serial execution:
- `blueprint-tangle-testing-utils`
- `blueprint-client-evm`
- `blueprint-tangle-extra`
- `blueprint-networking`
- `blueprint-qos`
- `cargo-tangle`

### External Dependencies
- **Substrate**: sp-core, sp-runtime (v34-39.x)
- **Alloy**: EVM interaction (v0.12)
- **libp2p**: P2P networking (v0.55)
- **Eigensdk**: EigenLayer integration (v0.5)
- **Foundry**: Smart contract compilation