# src

## Purpose
Meta-crate source that re-exports chain setup modules for local development environments. Currently provides Anvil (Foundry's local Ethereum node) setup utilities behind a feature flag.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `lib.rs` - Single re-export: conditionally publishes `blueprint_chain_setup_anvil` as `anvil` when the `anvil` feature is enabled.

## Key APIs (no snippets)
- `anvil` module (feature-gated) -- re-exports from `blueprint-chain-setup-anvil` for spawning and configuring local Anvil instances in tests and development.

## Relationships
- Depends on `blueprint-chain-setup-anvil` (behind the `anvil` feature).
- Used by testing utilities and integration tests that need a local EVM chain.
- Follows the workspace meta-crate pattern of feature-gated re-exports.
