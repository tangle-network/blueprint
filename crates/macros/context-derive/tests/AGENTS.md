# tests

## Purpose
Integration and compile-time tests for the context-derive proc-macros. Uses `trybuild` to verify that derive macros produce correct code for valid inputs and emit appropriate compile errors for invalid inputs.

## Contents (one hop)
### Subdirectories
- [x] `ui/` - Individual test case files for `trybuild` compile-pass and compile-fail scenarios. See `ui/AGENTS.md`.

### Files
- `tests.rs` - Main test harness that creates a `trybuild::TestCases` instance and registers pass cases (`basic.rs`, `unnamed_fields.rs`, `generic_struct.rs`) and fail cases (`missing_config_attr.rs`, `not_a_struct.rs`, `unit_struct.rs`).

## Key APIs
- (none -- test-only directory)

## Relationships
- Tests the derive macros defined in `../src/lib.rs` (`KeystoreContext`, `EVMProviderContext`, `TangleClientContext`)
- `.stderr` files in `ui/` contain expected compiler error messages for compile-fail tests
