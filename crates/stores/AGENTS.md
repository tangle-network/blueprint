# stores

## Purpose
Meta-crate for Blueprint storage backends. Provides a single dependency entry point for key-value storage, currently wrapping the local database implementation.

## Contents (one hop)
### Subdirectories
- [x] `local-database/` - Sub-crate (`blueprint-store-local-database`) implementing the local KV store backend
- [x] `src/` - Crate root: `lib.rs` (re-exports) and `error.rs` (error types)

### Files
- `Cargo.toml` - Crate manifest (`blueprint-stores`); single feature `local` (default) that enables the local database backend
- `CHANGELOG.md` - Release history
- `README.md` - Brief description of re-exports

## Key APIs (no snippets)
- `local_database` module - Re-export of `blueprint-store-local-database` (behind `local` feature)
- `error::Error` - Storage error type

## Relationships
- Depends on `blueprint-store-local-database` (optional, behind `local` feature)
- Re-exported by `blueprint-sdk` as `stores` (behind `local-store` feature)

## Notes
- Default feature: `local` (the only backend currently available)
- Designed to be extended with additional storage backends (e.g., remote, distributed) as new sub-crates
