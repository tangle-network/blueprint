# src

## Purpose
Implements a simple JSON-file-backed key-value store with mutex-guarded concurrent access and atomic writes (write-to-temp then rename). Provides CRUD operations, predicate search, atomic update, and bulk replace for any `Serialize + DeserializeOwned + Clone` value type.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `lib.rs` - `LocalDatabase<T>` struct: `open` (creates or loads JSON file), `set`, `get`, `remove`, `values`, `entries`, `find`, `update`, `replace`, `contains_key`, `len`, `is_empty`; all writes atomically flush via temp-file rename
- `error.rs` - `Error` enum with variants: `Io` (from `std::io::Error`), `Serialization` (from `serde_json::Error`), `Poisoned` (mutex poisoned)

## Key APIs
- `LocalDatabase::<T>::open(path)`: create or load a JSON-backed database, auto-creating parent directories
- `set(key, value)` / `get(key)` / `remove(key)`: basic CRUD with automatic persistence
- `update(key, closure)`: atomic read-modify-write with flush
- `replace(new_data)`: bulk replace all entries
- `find(predicate)`: linear scan for first matching value
- `entries()` / `values()`: clone all data out

## Relationships
- Depends on `blueprint-std` (with `std` feature) for filesystem and sync primitives
- Uses `serde` / `serde_json` for serialization
- Part of the `blueprint-stores` family; provides the simplest storage backend
- Used by the manager and other crates needing lightweight persistent state
