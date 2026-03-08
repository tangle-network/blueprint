# context-derive

## Purpose
Procedural macro crate for deriving Context Extension trait implementations. Generates boilerplate code that connects user-defined context structs to the blueprint-sdk's keystore, EVM provider, EigenLayer, and Tangle client systems by finding a `#[config]`-annotated field of type `BlueprintEnvironment`.

## Contents (one hop)
### Subdirectories
- [x] `src/` - Proc-macro implementations: `KeystoreContext`, `EVMProviderContext` (feature-gated on `evm`), `EigenlayerContext`, `TangleClientContext` (feature-gated on `tangle`); config field finder and per-extension code generators
- [x] `tests/` - Compile-time UI tests via `trybuild`; `tests.rs` runner and `ui/` directory with pass/fail test cases

### Files
- `CHANGELOG.md` - Version history
- `Cargo.toml` - Proc-macro crate; depends on `syn` (full), `quote`, `proc-macro2`; dev-depends on `blueprint-sdk`, `trybuild`, `alloy-*`, `round-based`
- `README.md` - Crate documentation

## Key APIs (no snippets)
- `#[derive(KeystoreContext)]` - Generates `KeystoreContext` trait impl from a struct with a `#[config]` field
- `#[derive(EVMProviderContext)]` - Generates EVM provider access (requires `evm` + `std` features)
- `#[derive(EigenlayerContext)]` - Generates EigenLayer context access
- `#[derive(TangleClientContext)]` - Generates Tangle substrate client access (requires `tangle` feature)
- All derive macros require a `#[config]` attribute on a field of type `BlueprintEnvironment`

## Relationships
- Used by blueprint developers to connect context structs to SDK infrastructure
- Consumed through `blueprint-sdk` re-exports (the `macros` feature)
- Generates code that references `blueprint_runner::config::BlueprintEnvironment`
- Companion to the main `blueprint-macros` crate which provides `debug_job` and `FromRef`

## Notes
- Proc-macro crate (`lib` type = `proc-macro`)
- Feature-gated: `evm`, `tangle`, `networking` features control which derive macros are available
- Config field discovery uses `CONFIG_TAG_NAME = "config"` and `CONFIG_TAG_TYPE = "blueprint_runner::config::BlueprintEnvironment"`
