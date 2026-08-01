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

## Provenance — keep in step with `tnt-core-bindings`
These fixtures deploy real tnt-core contracts, so their ABI must match the
`tnt-core-bindings` version the workspace resolves. When the two drift, calls
still succeed and decode into the wrong fields — e.g. tnt-core 0.19 reordered
`Types.JobCall` (`payment` moved behind `completed`/`isRFQ`), so an 0.18 fixture
read under 0.19.1 bindings reports every job as never completed. Regenerate the
fixtures in the same change as any `tnt-core-bindings` bump.

- Current fixture: tnt-core `f173dce`, matching `tnt-core-bindings` 0.19.1
  (whose `TNT_CORE_VERSION` is `7cdda757…`, the ABI `f173dce` still ships).
- The broadcast's top-level `commit` field records the tnt-core commit it was
  generated from — check it first when a Tangle e2e test decodes nonsense.
- Regenerate by running `script/sh/update-localtestnet-fixtures.sh` in a tnt-core
  checkout and copying both files here. As of `f173dce` that script needs
  `FOUNDRY_CODE_SIZE_LIMIT` raised to ~1000000: `LocalTestnetSetup`'s initcode is
  ~595 KB and forge caps initcode at twice the code-size limit.
- Deployment addresses are CREATE addresses derived from the deployer's nonce
  order, so they move whenever the deploy sequence changes. After regenerating,
  re-read them from the broadcast and update `TANGLE_ADDRESS` / `RESTAKING_ADDRESS`
  / `STATUS_REGISTRY_ADDRESS` in `crates/testing-utils/anvil/src/tangle.rs` plus
  the matching constants in `cli/src/command/dev/up.rs`.
- The snapshot is an anvil 1.7.x state dump; it must stay in step with `ANVIL_TAG`
  in `../src/anvil.rs` because dump formats are not forward compatible.
