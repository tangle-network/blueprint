# sdk

## Purpose
Umbrella crate (`blueprint-sdk`) that re-exports the entire Blueprint runtime surface so most projects can depend on a single crate. Provides jobs, router, runner, protocol integrations (Tangle, EigenLayer, EVM), optional gateways (x402, webhooks), networking, TEE, testing utilities, and build helpers -- all behind feature flags.

## Contents (one hop)
### Subdirectories
- [x] `src/` - Crate root with re-exports, `error.rs` for SDK-level error type, `registration.rs` for registration helpers

### Files
- `Cargo.toml` - Crate manifest (`blueprint-sdk`); extensive feature flags organized into Core, Protocol Support, Networking, Utilities, Payments, and TEE groups
- `CHANGELOG.md` - Release history
- `README.md` - Usage overview with minimal runner wiring example

## Key APIs (no snippets)
- Re-exports `blueprint_core` as `core` (and glob re-exports all core items at crate root)
- `runner` - Re-export of `blueprint-runner` (BlueprintRunner, BlueprintConfig, BackgroundService)
- `Router` - Re-export of `blueprint-router::Router`
- `crypto` - Re-export of `blueprint-crypto`
- `keystore` - Re-export of `blueprint-keystore`
- `contexts` - Re-export of `blueprint-contexts`
- `clients` - Re-export of `blueprint-clients`
- `auth` / `AuthContext` - Re-export of `blueprint-auth`
- `extract` module - Core extractors plus `FromRef` derive macro (feature `macros`)
- `producers::CronJob` (feature `cronjob`)
- Feature-gated modules: `evm`, `tangle`, `eigenlayer`, `networking`, `testing`, `build`, `x402`, `webhooks`, `tee`, `remote`, `stores`

## Relationships
- Depends on nearly every workspace crate as an optional dependency
- This is the primary dependency for blueprint authors -- most external consumers only need `blueprint-sdk`
- Features cascade to child crates (e.g., `tangle` enables `blueprint-runner/tangle`, `blueprint-clients/tangle`, `blueprint-contexts/tangle`, etc.)

## Notes
- Default features: `std` + `tracing`
- `macros` feature must be explicitly enabled for derive macros (`blueprint-macros`, `blueprint-context-derive`)
- `testing` feature pulls in `blueprint-testing-utils`, `blueprint-chain-setup`, and `tempfile`
- Docs.rs metadata enables a broad set of features for documentation generation
