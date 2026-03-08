# src

## Purpose
Provides Anvil (Foundry) EVM testnet container management for integration tests, including container lifecycle, key management, state loading, and snapshot support.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `lib.rs` - Crate root re-exporting `anvil::*`, `snapshot_available`, and state types (`AnvilState`, `get_default_state`, `get_default_state_json`).
- `anvil.rs` - Core container management: `AnvilTestnet` struct holding the Docker container and endpoints, `start_anvil_container()` with retry logic and state loading via `--load-state`, `mine_anvil_blocks()` via `cast rpc`, `start_default_anvil_testnet()`, `start_empty_anvil_testnet()`, `start_anvil_testnet_with_state()`, and `get_receipt()` helper for transaction submission.
- `error.rs` - Error enum covering container, mining, wait-response, contract, transaction, and keystore failures.
- `keys.rs` - `ANVIL_PRIVATE_KEYS` constant array (10 default Anvil dev keys) and `inject_anvil_key()` which generates ECDSA and BLS BN254 keys in a keystore from a seed string.
- `snapshot.rs` - Snapshot file utilities: `default_snapshot_path()` resolving to `snapshots/localtestnet-state.json`, `snapshot_state_json()` / `snapshot_state_json_from_path()` for reading snapshot files, and `snapshot_available()` existence check.
- `state.rs` - `AnvilState` and `AccountState` serde types for Anvil state JSON, plus `get_default_state_json()` (returns embedded `&'static str`) and `get_default_state()` (parses into `AnvilState`).

## Key APIs
- `start_anvil_container()` / `start_default_anvil_testnet()` / `start_empty_anvil_testnet()` - boot Anvil Docker containers with optional pre-loaded state
- `AnvilTestnet` - holds `ContainerAsync`, HTTP/WS endpoints, and temp directory
- `mine_anvil_blocks()` - advance the chain by N blocks
- `inject_anvil_key()` - populate a keystore with ECDSA + BLS keys for testing
- `ANVIL_PRIVATE_KEYS` - 10 well-known Anvil dev private keys
- `get_receipt()` - send a contract call and await its receipt

## Relationships
- Uses `testcontainers` to manage the `ghcr.io/foundry-rs/foundry` Docker image
- Depends on `blueprint_keystore` for key injection and `blueprint_core_testing_utils` for `TestRunnerError`
- Consumed by `blueprint_eigenlayer_testing_utils`, `blueprint_anvil_testing_utils`, `blueprint_client_eigenlayer/tests`, `blueprint_client_tangle/tests`, and other integration test harnesses
