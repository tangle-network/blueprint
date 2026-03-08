# fail

## Purpose
Compile-fail UI tests for the `#[debug_job]` macro. Each `.rs` file is expected to fail compilation, and the corresponding `.stderr` file captures the expected error output.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `argument_not_extractor.rs` + `.stderr` - Rejects function arguments that don't implement the extractor trait.
- `duplicate_args.rs` + `.stderr` - Rejects duplicate parameter names.
- `extract_self_mut.rs` + `.stderr` - Rejects `&mut self` receivers (jobs must not take mutable self).
- `extract_self_ref.rs` + `.stderr` - Rejects `&self` receivers.
- `generics.rs` + `.stderr` - Rejects generic type parameters on job functions.
- `invalid_attrs.rs` + `.stderr` - Rejects unrecognized attributes on the macro.
- `not_a_function.rs` + `.stderr` - Rejects non-function items (e.g., structs).
- `not_async.rs` + `.stderr` - Rejects synchronous functions (jobs must be async).
- `not_send.rs` + `.stderr` - Rejects futures that are not `Send`.
- `single_wrong_return_tuple.rs` + `.stderr` - Rejects single-element tuples with wrong inner type.
- `wrong_return_type.rs` + `.stderr` - Rejects return types that don't implement `IntoJobResult`.

## Key APIs
- (none -- test-only module)

## Relationships
- Tests the `#[debug_job]` procedural macro from `blueprint-macros`
- Uses `trybuild` for compile-time error assertions
