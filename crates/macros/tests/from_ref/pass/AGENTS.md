# pass

## Purpose
Compile-success test cases for the `#[derive(FromRef)]` proc-macro. Each `.rs` file must compile without errors, validating that the macro correctly generates `FromRef` trait implementations for struct fields across different usage patterns.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `basic.rs` - Tests canonical `FromRef` derivation with Router integration: struct with one `String` field, `Context<String>` extractor, and `Router::new().route().with_context()` wiring.
  - **Key items**: `AppContext`, `#[derive(Clone, FromRef)]`, `Context<String>`, `Router`
- `reference-types.rs` - Tests `FromRef` derivation on reference-type fields (`&'static str`), verifying no unnecessary `.clone()` is emitted.
  - **Key items**: `MyContext`, `inner: &'static str`, `#![deny(noop_method_call)]`
- `skip.rs` - Tests field skipping via `#[from_ref(skip)]` attribute: two `String` fields, one extracted, one skipped.
  - **Key items**: `MyContext`, `#[from_ref(skip)]`, `auth_token`, `also_string`

## Key APIs (no snippets)
- **`#[derive(FromRef)]`** - Proc-macro generating `impl FromRef<Struct> for FieldType`
- **`#[from_ref(skip)]`** - Field attribute to suppress `FromRef` impl generation
- **`FromRef` trait** - Defined in `crates/core/src/extract/from_ref.rs:11-14`

## Relationships
- **Depends on**: `blueprint_sdk::extract::{FromRef, Context}`, `blueprint_sdk::Router`; macro impl in `crates/macros/src/from_ref.rs`
- **Used by**: `trybuild` harness via `crates/macros/src/from_ref.rs:103-106`
- **Sibling**: `../fail/` contains compile-failure test cases

## Files (detailed)

### `basic.rs`
- **Role**: Canonical usage showing FromRef with Router context extraction.
- **Key items**: `AppContext { auth_token: String }`, `async fn job(_: Context<String>)`, `Router::new().route(0, job).with_context(ctx)`
- **Interactions**: Exercises full SDK integration path (derive + context + router)
- **Knobs / invariants**: Field type must implement `Clone` for non-reference FromRef

### `reference-types.rs`
- **Role**: Validates reference-type fields emit bare access (no `.clone()`).
- **Key items**: `MyContext { inner: &'static str }`, `#![deny(noop_method_call)]`
- **Interactions**: The `deny` lint catches if macro incorrectly emits `.clone()` on a reference
- **Knobs / invariants**: Reference detection in macro at `from_ref.rs:43-46`

### `skip.rs`
- **Role**: Validates `#[from_ref(skip)]` prevents impl generation for marked fields.
- **Key items**: `MyContext { auth_token: String, #[from_ref(skip)] also_string: String }`
- **Interactions**: Only `auth_token` gets a `FromRef` impl; `also_string` is excluded
- **Knobs / invariants**: Attribute parsing via `FieldAttrs` struct in `from_ref.rs:71-101`

## End-to-end flow
1. `cargo test -p blueprint-macros` triggers `#[test] fn ui()` in `from_ref.rs`
2. `run_ui_tests("from_ref")` creates `trybuild::TestCases` and calls `.pass("tests/from_ref/pass/*.rs")`
3. Each `.rs` file compiled in isolation as a test crate with macro expansion
4. Trybuild verifies successful compilation (no `.stderr` files needed)
5. Test passes if all files compile without errors

## Notes
- Only runs on Rust nightly (`#[rustversion::nightly]` guard)
- Clone is required for non-reference field types in FromRef derivations
- No `.stderr` files in this directory (unlike `fail/`)
