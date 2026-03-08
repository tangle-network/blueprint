# src

## Purpose
Meta-crate entry point that re-exports testing utilities from feature-gated sub-crates, giving blueprint authors a single dependency for test harnesses across different network backends.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `lib.rs` - Re-exports `blueprint_core_testing_utils::*` unconditionally; conditionally re-exports `blueprint_anvil_testing_utils` (feature `anvil`) and `blueprint_eigenlayer_testing_utils` (feature `eigenlayer`)

## Key APIs
- All public items from `blueprint_core_testing_utils` (always available)
- `anvil` module: Anvil/EVM test helpers (behind `anvil` feature)
- `eigenlayer` module: EigenLayer test helpers (behind `eigenlayer` feature)

## Relationships
- Depends on `blueprint-core-testing-utils`, `blueprint-anvil-testing-utils`, `blueprint-eigenlayer-testing-utils`
- Consumed by blueprint integration tests and example crates
- Re-exported through `blueprint-testing-utils` in the top-level SDK
