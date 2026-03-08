# fail

## Purpose
Compile-failure test cases for the `#[derive(FromRef)]` proc-macro. Holds Rust source files expected to fail compilation, paired with `.stderr` files documenting expected compiler error output. Run by the `trybuild` harness in the parent `blueprint-macros` crate.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `generics.rs` - Test input that applies `#[derive(FromRef)]` to a generic struct `AppContext<T>`, which the macro must reject.
  - **Key items**: `AppContext<T>`, `#[derive(Clone, FromRef)]`, `blueprint_sdk::extract::FromRef`
- `generics.stderr` - Expected compiler diagnostic for `generics.rs`; verifies error message and span location.
  - **Key items**: error message `#[derive(FromRef)] doesn't support generics`, line/column pointer to `<T>`

## Key APIs (no snippets)
- No runtime APIs. Exercises compile-time validation in `from_ref::expand()` (`crates/macros/src/from_ref.rs:12-16`).

## Relationships
- **Depends on**: `blueprint_sdk::extract::FromRef` trait; `#[derive(FromRef)]` proc-macro in `crates/macros/src/from_ref.rs`
- **Used by**: `trybuild` harness invoked from `crates/macros/src/from_ref.rs:103-106` via `run_ui_tests("from_ref")`
- **Sibling**: `../pass/` contains successful compilation test cases

## Files (detailed)

### `generics.rs`
- **Role**: Minimal compile-fail input applying `#[derive(FromRef)]` to a struct with a generic type parameter.
- **Key items**: `AppContext<T>`, `FromRef` derive, `fn main() {}`
- **Interactions**: Compiled by trybuild; output compared against `generics.stderr`
- **Knobs / invariants**: Must remain a syntactically valid Rust file with proper imports from `blueprint_sdk`

### `generics.stderr`
- **Role**: Expected compiler error output for trybuild line-by-line comparison.
- **Key items**: Error text, source span at `generics.rs:4:18`
- **Interactions**: Must match actual rustc output exactly (including line/column numbers)
- **Knobs / invariants**: Line numbers must stay in sync with `generics.rs`; only runs on nightly Rust

## End-to-end flow
1. `cargo test -p blueprint-macros` triggers `#[test] fn ui()` in `from_ref.rs`
2. `run_ui_tests("from_ref")` creates `trybuild::TestCases` and calls `.compile_fail("tests/from_ref/fail/*.rs")`
3. Trybuild compiles `generics.rs`; macro detects generic params and emits `syn::Error`
4. Trybuild captures stderr, compares line-by-line with `generics.stderr`
5. Test passes if output matches exactly

## Notes
- Only runs on Rust nightly (`#[rustversion::nightly]` guard)
- Currently contains a single test case; more can be added as the macro evolves
