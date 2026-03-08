# local-database

## Purpose
JSON-file-backed key-value store crate providing the simplest persistent storage backend in the Blueprint SDK. Supports concurrent access via mutex, atomic writes via temp-file rename, and generic value types via serde.

## Contents (one hop)
### Subdirectories
- [x] `src/` - `LocalDatabase<T>` implementation with CRUD, atomic update, predicate search, bulk replace; `Error` enum for I/O, serialization, and mutex poisoning

### Files
- `Cargo.toml` - Crate manifest; depends on `blueprint-std` (std), `serde`, `serde_json`, `thiserror`; dev-depends on `tempfile`
- `CHANGELOG.md` - Version history
- `README.md` - Crate documentation

## Key APIs
- `LocalDatabase::<T>::open(path)`: create or load a JSON-backed database
- `set` / `get` / `remove` / `update` / `replace` / `find` / `entries` / `values`

## Relationships
- Depends on `blueprint-std` for filesystem and sync primitives
- Part of the `blueprint-stores` family alongside other storage backends
- Used by the manager and other crates needing lightweight persistent state
