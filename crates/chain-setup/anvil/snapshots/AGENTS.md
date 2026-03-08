# snapshots

## Purpose
Stores Anvil chain snapshots for the local testnet, including the Forge broadcast output from contract deployment and the resulting chain state.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `localtestnet-broadcast.json` - Forge broadcast transaction log from running `LocalTestnet.s.sol`, containing all CREATE transactions for contract deployments (e.g., `MultiAssetDelegation`) with their addresses, calldata, and chain configuration.
- `localtestnet-state.json` - Full Anvil state dump after running the local testnet deployment script, containing all accounts, balances, contract bytecode, and storage.

## Key APIs
- `localtestnet-state.json` is loaded by `snapshot.rs` via `default_snapshot_path()` which resolves to this directory relative to `CARGO_MANIFEST_DIR`
- `snapshot_state_json()` reads the state file for use with `start_empty_anvil_testnet()`

## Relationships
- Read by `../src/snapshot.rs` to provide pre-seeded chain state for empty testnet starts
- The broadcast file documents which contracts were deployed and at which addresses
- Used by test harnesses across the workspace that need a fresh Anvil with deployed contracts
