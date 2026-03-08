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

# Format check in CI parity mode (required before push)
cargo +nightly-2026-02-24 fmt -- --check

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
cargo tangle blueprint deploy tangle --network devnet

# Deploy to testnet/mainnet using a definition manifest
cargo tangle blueprint deploy tangle --network testnet --definition ./path/to/definition.json

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
- Version: 1.88 (2024 edition)
- Workspace resolver: v3
- Rustfmt in CI is pinned to: `nightly-2026-02-24`

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

## PR Quality Gate

PRs targeting `main` are validated by `.github/workflows/pr-quality-gate.yml`. The PR body **must** contain these markdown sections (exactly as `## Section Name`):

1. `## Summary` — bullet-point description of changes
2. `## Change Class` — must include `- Selected class: Class X` and `- Why this class: ...`
3. `## Behavior Contract` — describe what changed behaviorally (new errors, changed defaults, etc.)
4. `## Risk And Scope` — blast radius, mitigation, risk assessment
5. `## Verification` — at least one fenced code block with test/build commands
6. `## Harness Evidence` — which tests cover the changes and their status
7. `## Checklist` — markdown checklist of quality items

### Change Class Rules (from `.github/pr-quality-gate.toml`)
- **Class D** (required) when changing files under:
  - `crates/manager/src/protocol/`, `crates/manager/src/rt/container/`, `crates/manager/src/sources/`
  - `crates/clients/tangle/src/`, `crates/tee/src/`, `cli/src/command/deploy/`
- **Class C** is auto-promoted for multi-crate changes or CLI+crate changes
- **Docs-only** PRs (only `*.md`, `docs/**`, `.github/**`) skip validation

### Pre-push Hook
The local `.git/hooks/pre-push` runs: (1) `cargo fmt -- --check`, (2) clippy on changed crates, (3) tests on changed crates, (4) optional security audit. All checks must pass before push succeeds.

### PR Body Source
The quality gate reads the PR body from `GITHUB_EVENT_PATH` (the event payload), **not** from the live API. This means:
- Editing the PR body after push does **not** update already-running checks
- To re-evaluate after a body edit, push a new commit (even empty) to trigger a fresh `synchronize` event
- `gh run rerun` replays the old event payload and will see the old body
