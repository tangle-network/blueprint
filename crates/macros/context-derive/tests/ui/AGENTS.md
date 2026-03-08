# ui

## Purpose
UI tests for context derive macros using trybuild. Validates that derive macros produce correct compile errors for invalid inputs and compile successfully for valid inputs.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Declares test modules: `basic`, `generic_struct`, `unnamed_fields`.
- `basic.rs` - Pass case: derives `KeystoreContext`, `EVMProviderContext`, `TangleClientContext` on a basic named-field struct.
- `generic_struct.rs` - Pass case: derives context traits on a generic struct with type parameters.
- `unnamed_fields.rs` - Pass case: derives context traits on a tuple struct with unnamed fields.
- `missing_config_attr.rs` + `.stderr` - Fail case: struct missing the required `#[config]` attribute on a field.
- `not_a_struct.rs` + `.stderr` - Fail case: derive applied to an enum instead of a struct.
- `unit_struct.rs` + `.stderr` - Fail case: derive applied to a unit struct with no fields.

## Key APIs
- (none -- test-only module)

## Relationships
- Tests `KeystoreContext`, `EVMProviderContext`, `TangleClientContext` derive macros from `blueprint-macros-context-derive`
- Uses `trybuild` for compile-time test assertions
