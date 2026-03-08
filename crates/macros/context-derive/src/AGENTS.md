# src

## Purpose
Proc-macro crate that provides derive macros for generating context extension trait implementations (`KeystoreContext`, `EVMProviderContext`, `EigenlayerContext`, `TangleClientContext`). Each derive macro finds a `#[config]`-annotated field holding a `BlueprintEnvironment` and generates the corresponding trait impl that delegates to that field.

## Contents (one hop)
### Subdirectories
- [x] `tangle/` - Tangle-specific context derive implementation. See `tangle/AGENTS.md`.

### Files
- `lib.rs` - Entry point defining four `#[proc_macro_derive]` functions: `KeystoreContext`, `EVMProviderContext` (feature-gated on `std` + `evm`), `EigenlayerContext`, and `TangleClientContext` (feature-gated on `tangle`). Each parses input, finds the config field via `cfg::find_config_field`, and delegates to the corresponding module's `generate_context_impl`.
- `cfg.rs` - `find_config_field` utility that searches struct fields for a `#[config]` attribute. Returns `FieldInfo::Named(Ident)` or `FieldInfo::Unnamed(Index)`. Produces compile errors for enums, unit structs, and missing config attributes.
- `keystore.rs` - Generates `KeystoreContext` impl that delegates `keystore()` to the config field via `BlueprintEnvironment`'s own `KeystoreContext` impl.
- `evm.rs` - Generates `EvmInstrumentedClientContext` impl with type aliases for Ethereum network/provider, creating an `InstrumentedClient` from the config's HTTP RPC endpoint.
- `eigenlayer.rs` - Generates `EigenlayerContext` impl with `OnceLock`-cached client creation, delegating to the config field's `EigenlayerContext` impl.

## Key APIs
- `#[derive(KeystoreContext)]` - generates `KeystoreContext` trait impl
- `#[derive(EVMProviderContext)]` - generates `EvmInstrumentedClientContext` trait impl
- `#[derive(EigenlayerContext)]` - generates `EigenlayerContext` trait impl
- `#[derive(TangleClientContext)]` - generates `TangleClientContext` trait impl
- All require a `#[config]` attribute on a `BlueprintEnvironment` field

## Relationships
- Generated code references `blueprint_sdk::contexts::*` trait paths
- Generated code references `blueprint_sdk::runner::config::BlueprintEnvironment`
- Tests in sibling `tests/` directory validate compile-pass and compile-fail scenarios via `trybuild`
