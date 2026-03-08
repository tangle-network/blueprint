# tests

## Purpose
Compile-time UI test suites for the `blueprint-macros` proc-macro crate. Uses `trybuild` to verify that the `#[debug_job]` and `#[derive(FromRef)]` macros produce correct compile errors on invalid usage and compile successfully on valid usage.

## Contents (one hop)
### Subdirectories
- [x] `debug_job/` - UI test cases for `#[debug_job]` macro: `pass/` contains valid usages that should compile, `fail/` contains invalid usages that should produce specific compile errors
- [x] `from_ref/` - UI test cases for `#[derive(FromRef)]` macro: `pass/` contains valid struct patterns, `fail/` contains invalid patterns with expected error messages

### Files
- (none)

## Key APIs (no snippets)
- Test cases are `.rs` files in `pass/` and `fail/` subdirectories; no library APIs

## Relationships
- Tests the macros defined in `crates/macros/src/` (`debug_job` and `from_ref` modules)
- Test runner is `run_ui_tests()` in `crates/macros/src/lib.rs`
- Uses `trybuild` crate for compile-fail/compile-pass testing

## Notes
- UI tests only run on nightly Rust
- `fail/` directories contain `.rs` files paired with `.stderr` files for expected error output
- Can filter tests via `BLUEPRINT_TEST_ONLY` env var
