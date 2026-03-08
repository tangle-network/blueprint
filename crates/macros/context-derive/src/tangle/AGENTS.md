# tangle

## Purpose
Procedural macro implementation for the `TangleClientContext` derive macro. Generates a trait impl that delegates `tangle_client()` calls to a config field on the deriving struct.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Re-exports `client` module.
- `client.rs` - `generate_context_impl()` function. Takes `DeriveInput` and `FieldInfo` (named or unnamed), produces a `TokenStream` implementing `TangleClientContext` by delegating to the annotated config field's own `TangleClientContext` impl. Returns a `Future<Output = Result<TangleClient, Error>>`.

## Key APIs
- `generate_context_impl(DeriveInput, FieldInfo) -> TokenStream` -- the derive macro's code generation entry point

## Relationships
- Uses `crate::cfg::FieldInfo` for field access resolution
- Uses `quote`/`syn` for code generation
- Generated code references `blueprint_sdk::contexts::tangle::{TangleClientContext, TangleClient, Error}`
- Feature-gated: `tangle`
