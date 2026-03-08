# anvil

## Purpose
Crate (`blueprint-chain-setup-anvil`) providing Anvil-specific chain setup utilities for testing. Manages Anvil (Foundry's local EVM testnet) instances via testcontainers, handles pre-deployed contract state snapshots, key management, and state initialization for EVM integration tests.

## Contents (one hop)
### Subdirectories
- [x] `data/` - Static chain data; contains `state.json` with pre-configured Anvil chain state.
- [x] `snapshots/` - Pre-built Anvil snapshots including `localtestnet-broadcast.json` (deployment transaction records) and `localtestnet-state.json` (full chain state snapshot with deployed contracts).
- [x] `src/` - Crate source code with modules for Anvil container management (`anvil.rs`), error types (`error.rs`), test key utilities (`keys.rs`), snapshot loading (`snapshot.rs`), and state management (`state.rs`).

### Files
- `CHANGELOG.md` - Version history for the crate
- `Cargo.toml` - Crate manifest; depends on `blueprint-std`, `blueprint-keystore`, `blueprint-core-testing-utils`, alloy crates, `testcontainers`, and `tempfile`
- `README.md` - Crate documentation

## Key APIs
- `AnvilState` / `get_default_state()` / `get_default_state_json()` -- load pre-configured chain state
- `snapshot_available()` -- check if a snapshot is available for fast test initialization
- Anvil container management types (re-exported from `src/anvil.rs`)

## Relationships
- Used as a dev-dependency by `blueprint-client-evm`, `blueprint-client-eigenlayer`, `blueprint-client-tangle`, and EVM testing utilities
- Provides the local EVM testnet infrastructure for integration tests across the workspace
- Depends on `blueprint-keystore` for EVM/EigenLayer key management in test scenarios
