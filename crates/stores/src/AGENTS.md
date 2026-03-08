# src

## Purpose
Meta-crate re-export gateway for Blueprint SDK storage backends. Conditionally re-exports `blueprint-store-local-database` behind the `local` feature flag with a unified error type.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `lib.rs` - Re-exports `blueprint_store_local_database` as `local_database` when `local` feature is enabled (default). Uses `document_features!()` macro.
  - **Key items**: `pub use blueprint_store_local_database as local_database`, `pub use error::Error`, `#[cfg(feature = "local")]`
- `error.rs` - Unified storage error enum with feature-gated variants.
  - **Key items**: `Error` enum with `LocalDatabase(blueprint_store_local_database::Error)` variant, `#[from]` derive for auto-conversion

## Key APIs (no snippets)
- **Modules**: `local_database` (feature-gated re-export of `blueprint-store-local-database`)
- **Types**: `Error` (unified error with `LocalDatabase` variant)

## Relationships
- **Depends on**: `blueprint-store-local-database` (optional, gated on `local` feature)
- **Used by**: `blueprint-sdk` re-exports as `stores` module (feature: `local-store`); user code accesses `stores::local_database::LocalDatabase::<T>::open(path)`

## Files (detailed)

### `lib.rs`
- **Role**: Minimal conditional re-export layer (14 lines).
- **Key items**: Single `#[cfg(feature = "local")] pub use` statement
- **Knobs / invariants**: `local` feature enabled by default; disabling prevents backend compilation

### `error.rs`
- **Role**: Error aggregation for future multi-backend support (10 lines).
- **Key items**: `Error` enum, `#[from]` for automatic conversion
- **Knobs / invariants**: Future backends (Redis, DynamoDB) added as new variants

## End-to-end flow
1. User enables `local-store` feature in SDK
2. SDK re-exports `blueprint-stores` as `stores` module
3. User accesses `stores::local_database::LocalDatabase::<T>::open(path)`
4. Errors propagate through `blueprint_stores::Error::LocalDatabase`

## Notes
- 24 total lines of code; pure re-export layer with no business logic
- Designed for extensibility: new backends as conditional re-exports
- Sibling `local-database/` contains 14.5KB `LocalDatabase<T>` generic struct with atomic JSON-based storage
