# src

## Purpose
Meta-crate source that provides a unified entry point for all network-specific client implementations. Re-exports EigenLayer, EVM, and Tangle clients behind feature flags, along with core client types and a unified error type.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `error.rs` - Defines a unified `Error` enum that wraps errors from `blueprint-client-core`, `blueprint-client-eigenlayer`, and `blueprint-client-evm` (each behind their respective feature flags). Provides a convenience `Error::msg()` constructor.
- `lib.rs` - Feature-gated re-exports: `eigenlayer` (from `blueprint-client-eigenlayer`), `evm` (from `blueprint-client-evm`), `tangle` (from `blueprint-client-tangle`). Unconditionally re-exports everything from `blueprint-client-core`. Supports `no_std`.

## Key APIs (no snippets)
- `eigenlayer` module (feature-gated) -- re-export of `blueprint-client-eigenlayer`.
- `evm` module (feature-gated) -- re-export of `blueprint-client-evm`.
- `tangle` module (feature-gated) -- re-export of `blueprint-client-tangle`.
- `Error` enum -- unified error type across all client backends.
- All `blueprint-client-core` types are re-exported at the crate root.

## Relationships
- Depends on `blueprint-client-core`, `blueprint-client-eigenlayer`, `blueprint-client-evm`, and `blueprint-client-tangle`.
- Consumed by `crates/contexts/` to provide protocol-specific context traits.
- Consumed by `blueprint-sdk` as the primary client access point for blueprint authors.
