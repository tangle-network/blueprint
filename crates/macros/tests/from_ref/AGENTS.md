# from_ref

## Purpose
Compile-time test suite for the `#[derive(FromRef)]` proc-macro. Organizes test cases into `fail/` (expected compile errors) and `pass/` (expected successful compilation) subdirectories, used by a `trybuild` test harness.

## Contents (one hop)
### Subdirectories
- [x] `fail/` - Test cases that must produce compile errors. Contains `generics.rs` testing that generic structs are correctly rejected, paired with `generics.stderr`. See `fail/AGENTS.md`.
- [x] `pass/` - Test cases that must compile successfully. Contains `basic.rs` (simple field extraction with Router integration), `reference-types.rs` (reference type fields), and `skip.rs` (skipping fields). See `pass/AGENTS.md`.

### Files
- (none)

## Key APIs
- (none -- test-only directory)

## Relationships
- Tests the `#[derive(FromRef)]` derive macro defined in the parent `blueprint-macros` crate
- `FromRef` enables extracting shared state fields from a context struct via `Context<T>` extractor
- Referenced by a `trybuild::TestCases` harness in the parent `tests/` directory
