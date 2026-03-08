# chain-setup

## Purpose
Crate `blueprint-chain-setup`: Meta-crate that re-exports chain setup utilities for use with the Blueprint SDK. Currently provides Anvil (local EVM testnet) setup through the `anvil` feature flag. Acts as a single entry point for chain environment configuration across different testing and deployment scenarios.

## Contents (one hop)
### Subdirectories
- [x] `anvil/` - Sub-crate `blueprint-chain-setup-anvil`: Anvil-specific chain setup utilities including EVM contract deployment, keystore initialization, and test container management. Depends on `alloy-contract`, `alloy-provider`, `testcontainers`, `blueprint-keystore` (eigenlayer + evm features), and `blueprint-core-testing-utils`. Contains `data/` and `snapshots/` subdirectories for chain state fixtures.
- [x] `src/` - Minimal `lib.rs` that conditionally re-exports `blueprint-chain-setup-anvil` as the `anvil` module.

### Files
- `CHANGELOG.md` - Version history.
- `Cargo.toml` - Crate manifest (`blueprint-chain-setup`). Optional dep: `blueprint-chain-setup-anvil`. Features: `std` (default), `anvil`.
- `README.md` - Crate documentation.

## Key APIs (no snippets)
- `anvil` module (feature-gated) -- re-exports all of `blueprint-chain-setup-anvil` for Anvil testnet provisioning, contract deployment, and keystore setup.

## Relationships
- Re-exports `blueprint-chain-setup-anvil` which depends on `blueprint-keystore`, `blueprint-core-testing-utils`, `alloy-*` crates, and `testcontainers`.
- Used by EVM-related test crates (`blueprint-client-evm`, `blueprint-client-eigenlayer`, `blueprint-anvil-testing-utils`) for local chain setup.
- Part of the testing infrastructure layer of the workspace.
