# src

## Purpose
Source directory for the `blueprint-macros` proc-macro crate. Provides the `#[debug_job]` attribute macro for improved job function compile errors, the `#[derive(FromRef)]` macro for context field extraction, and the `load_abi!` macro for loading Solidity JSON ABI files at compile time.

## Contents (one hop)
### Subdirectories
- [x] `evm/` - `load_abi` proc-macro implementation: parses a Solidity JSON ABI file path and emits a const string of the ABI

### Files
- `attr_parsing.rs` - Shared attribute parsing utilities for proc-macro argument handling
- `debug_job.rs` - Core implementation of `#[debug_job]`: validates job functions are async, checks context types, generates compile-time check functions; no-op in release builds
- `from_ref.rs` - `#[derive(FromRef)]` implementation: generates `FromRef<ParentStruct>` impls for each field (supports `#[from_ref(skip)]`)
- `lib.rs` - Crate root; declares and exports `debug_job`, `FromRef`, `load_abi` proc-macros; contains `infer_context_types` helper and `run_ui_tests` test runner
- `with_position.rs` - Iterator adapter for tracking first/middle/last position in sequences

## Key APIs (no snippets)
- `#[debug_job]` - Attribute macro that generates better compile errors for job functions; validates async, correct parameter types, and context extraction; no-op in release mode
- `#[derive(FromRef)]` - Derives `FromRef` trait for each non-skipped struct field, enabling automatic context extraction via `Context<T>`
- `load_abi!(IDENT, "path/to/abi.json")` - Compiles a Solidity ABI JSON file into a const string (feature-gated on `evm`)
- `infer_context_types` - Internal helper that extracts the type parameter from `Context<T>` arguments

## Relationships
- Re-exported through `blueprint-sdk::macros`
- `debug_job` references `blueprint_sdk::Job` trait and `blueprint_sdk::Context` extractor
- `FromRef` references `blueprint_sdk::extract::FromRef`
- `load_abi` is gated behind the `evm` feature
- Companion to `blueprint-context-derive` which provides the `*Context` derive macros

## Notes
- Debug output available via `BLUEPRINT_MACROS_DEBUG` env var
- UI tests run only on nightly Rust (`#[rustversion::nightly]`)
- `BLUEPRINT_TEST_ONLY` env var can filter individual test files
