# testing-utils

## Purpose
Meta-crate for Blueprint testing helper crates. Provides a single dependency entry point that re-exports core testing utilities unconditionally and feature-gates Anvil (EVM) and EigenLayer test helpers.

## Contents (one hop)
### Subdirectories
- [x] `anvil/` - Sub-crate (`blueprint-anvil-testing-utils`) for Anvil/EVM test infrastructure
- [x] `core/` - Sub-crate (`blueprint-core-testing-utils`) for core testing utilities (always re-exported)
- [x] `eigenlayer/` - Sub-crate (`blueprint-eigenlayer-testing-utils`) for EigenLayer test infrastructure
- [x] `src/` - Crate root: `lib.rs` re-exports `blueprint_core_testing_utils::*` unconditionally; conditionally re-exports `anvil` and `eigenlayer` modules

### Files
- `Cargo.toml` - Crate manifest (`blueprint-testing-utils`); features: `anvil`, `eigenlayer`, `faas` (forwarded to core)
- `CHANGELOG.md` - Release history
- `README.md` - Brief list of re-exported sub-crates

## Key APIs (no snippets)
- All public items from `blueprint-core-testing-utils` (always available at crate root)
- `anvil` module (feature `anvil`) - Anvil/EVM test helpers from `blueprint-anvil-testing-utils`
- `eigenlayer` module (feature `eigenlayer`) - EigenLayer test helpers from `blueprint-eigenlayer-testing-utils`

## Relationships
- Depends on `blueprint-core-testing-utils` (always), `blueprint-anvil-testing-utils` (feature `anvil`), `blueprint-eigenlayer-testing-utils` (feature `eigenlayer`)
- Re-exported by `blueprint-sdk` under `testing::utils` (behind `testing` feature)
- Consumed by blueprint integration tests and example crates

## Notes
- No default features -- consumers must opt in to `anvil` and/or `eigenlayer`
- `faas` feature is forwarded to `blueprint-core-testing-utils/faas` for FaaS testing support
