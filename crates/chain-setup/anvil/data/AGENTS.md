# data

## Purpose
Stores the default Anvil EVM state JSON used to bootstrap local testnets with pre-deployed contracts and funded accounts.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `state.json` - Anvil chain state dump containing block metadata (number, coinbase, basefee, hardfork settings) and account states (nonce, balance, bytecode, storage) for pre-deployed contracts including the `MultiAssetDelegation` contract and other test infrastructure.

## Key APIs
- Consumed by `state.rs` via `include_str!("../data/state.json")` as `DEFAULT_STATE`
- Loaded into Anvil containers with the `--load-state` flag

## Relationships
- Read by `../src/state.rs` at compile time to embed as a constant
- Used by `start_anvil_container()` in `../src/anvil.rs` to seed testnet state
- Contains deployed contract bytecode and storage for integration tests across the workspace
