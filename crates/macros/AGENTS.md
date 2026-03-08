# macros

## Purpose
Procedural macros for the Blueprint SDK (`blueprint-macros`). Provides `#[debug_job]` for improved job function error messages, `#[derive(FromRef)]` for context field extraction, and `load_abi!` for EVM JSON ABI loading. Also contains the `blueprint-context-derive` sub-crate for deriving context extension traits.

## Contents (one hop)
### Subdirectories
- [x] `context-derive/` - Separate proc-macro crate (`blueprint-context-derive`) for deriving Context Extension traits; has its own `src/` with tangle-specific codegen and `tests/` with trybuild UI tests
- [x] `src/` - Main proc-macro source: `debug_job.rs` (job validation diagnostics), `from_ref.rs` (FromRef derive), `attr_parsing.rs` (attribute helpers), `with_position.rs` (iterator utility), `evm/` (ABI loading macro, gated on `evm` feature)
- [x] `tests/` - Trybuild compile-pass/compile-fail tests organized by macro: `debug_job/` and `from_ref/` each with `pass/` and `fail/` subdirectories

### Files
- `CHANGELOG.md` - Release history
- `Cargo.toml` - Crate manifest (`blueprint-macros`, `proc-macro = true`); depends on `proc-macro2`, `quote`, `syn` (full features); optional `serde_json` for EVM feature; dev-deps include `blueprint-sdk`, `trybuild`, `rustversion`
- `README.md` - Brief overview of job/debug macros, context/ref macros, EVM support

## Key APIs (no snippets)
- `#[debug_job]` -- attribute macro that validates job function signatures and produces clear compile errors (async required, context type inference); no-op in release builds
- `#[derive(FromRef)]` -- derive macro generating `FromRef` implementations for each struct field, enabling context extraction; supports `#[from_ref(skip)]`
- `load_abi!` (feature `evm`) -- proc macro that loads Solidity JSON ABI from a file path at compile time
- `context-derive` sub-crate -- `#[derive(TangleClientContext)]` and related derives for protocol-specific context extensions

## Relationships
- Used by `blueprint-sdk` (re-exported as `blueprint_sdk::macros`)
- `context-derive` sub-crate is a separate proc-macro used by `blueprint-contexts`
- Dev-tested against `blueprint-sdk` to ensure macro output compiles correctly
- Feature `__private` enables `syn/visit-mut` for internal use
