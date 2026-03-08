# evm-extra

## Purpose
EVM job utilities (`blueprint-evm-extra`). Provides producers, consumers, extractors, filters, and helpers for EVM-based job processing in the Blueprint framework -- event extraction from logs, block processing, finality determination, and contract interaction.

## Contents (one hop)
### Subdirectories
- [x] `src/` - Crate source: `consumer/` (result handling), `extract/` (event/data extraction), `filters/` (event filtering), `producer/` (EVM event/job producers), plus `util.rs`

### Files
- `CHANGELOG.md` - Release history
- `Cargo.toml` - Crate manifest (`blueprint-evm-extra`); depends on `blueprint-core`, `blueprint-std`, Alloy EVM stack (`alloy-provider` with pubsub, `alloy-rpc-types`, `alloy-sol-types`, etc.), async/streaming utilities (`async-stream`, `futures`, `tokio`, `tower`)
- `README.md` - Brief overview: EVM event/job producers, consumers, extractors, filters

## Key APIs (no snippets)
- `consumer` module -- EVM result handling and output consumers
- `extract` module -- event and data extraction from EVM logs/transactions
- `filters` module -- event filtering predicates for EVM data streams
- `producer` module -- EVM event/job producers that feed the router
- `util` module -- shared EVM helper functions

## Relationships
- Depends on `blueprint-core`, `blueprint-std`
- Used by `blueprint-eigenlayer-extra` for EVM-layer integration
- Used alongside `blueprint-tangle-extra` when blueprints need EVM event triggers
- Feature flags: `std` (serde_json std), `tracing` (blueprint-core tracing support)
