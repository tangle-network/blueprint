# debug_job

## Purpose
Compile-time test suite for the `#[debug_job]` proc-macro attribute. Organizes test cases into `fail/` (expected compile errors) and `pass/` (expected successful compilation) subdirectories, used by a `trybuild` test harness.

## Contents (one hop)
### Subdirectories
- [x] `fail/` - Test cases that must produce compile errors, covering invalid `#[debug_job]` usage: non-async functions, non-function items, generic functions, invalid arguments, self receivers, wrong return types, duplicate args, non-Send futures, and non-extractor arguments. Each `.rs` file is paired with a `.stderr` file containing expected error output. See `fail/AGENTS.md`.
- [x] `pass/` - Test cases that must compile successfully, covering valid `#[debug_job]` usage: associated functions, self receivers, `impl Future` returns, `impl IntoJobResult` returns, `deny(unreachable_code)` compatibility, context inference, `ready()` returns, and explicit context setting. See `pass/AGENTS.md`.

### Files
- (none)

## Key APIs
- (none -- test-only directory)

## Relationships
- Tests the `#[debug_job]` attribute macro defined in the parent `blueprint-macros` crate
- Referenced by a `trybuild::TestCases` harness in the parent `tests/` directory
