# pass

## Purpose
Compile-pass UI tests for the `#[debug_job]` macro. Each file demonstrates a valid usage pattern that must compile successfully.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `associated_fn_without_self.rs` - Associated function on an impl block without self receiver.
- `deny_unreachable_code.rs` - Verifies macro works with `#[deny(unreachable_code)]` lint enabled.
- `impl_future.rs` - Returns `impl Future` instead of using async fn syntax.
- `impl_into_job_result.rs` - Returns a custom type implementing `IntoJobResult`.
- `infer_context.rs` - Context type is inferred from the function signature.
- `ready.rs` - Uses `std::future::ready` for an immediately-resolved future.
- `self_receiver.rs` - Uses a by-value `self` receiver (move semantics).
- `set_ctx.rs` - Explicitly sets the context type via macro attribute.

## Key APIs
- (none -- test-only module)

## Relationships
- Tests the `#[debug_job]` procedural macro from `blueprint-macros`
- Uses `trybuild` for compile-pass assertions
