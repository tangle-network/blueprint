# clients

## Purpose
Crate `blueprint-clients`: Meta-crate that unifies network-specific client implementations behind feature flags. Provides the `BlueprintServicesClient` trait (from `blueprint-client-core`) and conditionally re-exports Tangle, EigenLayer, and EVM clients. Serves as the single dependency for crates needing multi-network client access.

## Contents (one hop)
### Subdirectories
- [x] `core/` - Sub-crate `blueprint-client-core`: Defines the `BlueprintServicesClient` trait with associated types for operator identity, account identity, and blueprint IDs. Provides `get_operators()`, `operator_id()`, `blueprint_id()`, and derived methods for operator indexing. Depends only on `blueprint-std`, `auto_impl`, `thiserror`.
- [x] `eigenlayer/` - Sub-crate `blueprint-client-eigenlayer`: EigenLayer client using `eigensdk` for AVS registry, BLS aggregation, and operator management. Depends on `blueprint-runner` (eigenlayer feature), `alloy-*` crates, `eigensdk`, `blueprint-evm-extra`.
- [x] `evm/` - Sub-crate `blueprint-client-evm`: Generic EVM client with instrumented RPC calls, event watching, and metrics. Built on Alloy (`alloy-provider`, `alloy-primitives`, `alloy-rpc-types`). Includes `blueprint-metrics-rpc-calls` for RPC call tracking.
- [x] `src/` - Crate root: `lib.rs` re-exports core types and conditionally exposes `eigenlayer`, `evm`, `tangle` modules. `error.rs` defines the unified error type.
- [x] `tangle/` - Sub-crate `blueprint-client-tangle`: Tangle network client connecting to Tangle EVM contracts via Alloy. Handles blueprint metadata, operator registration, service instances, and job result submission. Depends on `tnt-core-bindings`, `blueprint-keystore`, `blueprint-crypto` (k256).

### Files
- `CHANGELOG.md` - Version history.
- `Cargo.toml` - Crate manifest (`blueprint-clients`). Deps: `blueprint-client-core` (always), optional `blueprint-client-eigenlayer`, `blueprint-client-evm`, `blueprint-client-tangle`. Features: `eigenlayer`, `evm`, `tangle`.
- `README.md` - Crate documentation.

## Key APIs (no snippets)
- `BlueprintServicesClient` trait (from core) -- async interface for operator set queries, operator identity, and blueprint ID retrieval.
- `OperatorSet<K, V>` type alias -- `BTreeMap` mapping account identities to application identities.
- `eigenlayer` module -- EigenLayer AVS client (feature-gated).
- `evm` module -- generic EVM client with instrumented provider (feature-gated).
- `tangle` module -- Tangle network client for on-chain blueprint operations (feature-gated).

## Relationships
- Core dependency for `blueprint-contexts` which provides context traits on top of these clients.
- Core dependency for `blueprint-runner` which uses clients for chain interaction during job execution.
- Each sub-crate can be used independently or through this meta-crate.
- `blueprint-client-tangle` is the most feature-rich, handling metadata, registration, and job lifecycle.
