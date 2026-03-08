# core

## Purpose
Crate (`blueprint-client-core`) providing the core trait and primitives shared by all Blueprint client implementations (Tangle, EigenLayer, EVM). Defines the `BlueprintServicesClient` trait that abstracts operator set queries and identity management across different networks.

## Contents (one hop)
### Subdirectories
- [x] `src/` - Source code with `lib.rs` defining the `BlueprintServicesClient` trait and `error.rs` with shared error types.

### Files
- `CHANGELOG.md` - Version history
- `Cargo.toml` - Crate manifest; minimal dependencies: `blueprint-std`, `auto_impl`, `thiserror`
- `README.md` - Crate documentation

## Key APIs
- `BlueprintServicesClient` trait -- core interface with associated types for `PublicApplicationIdentity`, `PublicAccountIdentity`, `Id`, and `Error`; provides `get_operators()`, `operator_id()`, `blueprint_id()`, `get_operators_and_operator_id()`, and `get_operator_index()` methods
- `OperatorSet<K, V>` type alias -- `BTreeMap` of operator identities

## Relationships
- Implemented by `blueprint-client-eigenlayer` and `blueprint-client-tangle`
- Used by `blueprint-runner` and `blueprint-router` to interact with operators in a network-agnostic way
- Uses `auto_impl` for automatic trait implementations on `&T` and `Arc<T>`
