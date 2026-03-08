# storage

## Purpose
Defines the `RawStorage` trait for low-level key-value storage of cryptographic key material, along with a type-safe `TypedStorage` wrapper and two concrete implementations: in-memory and filesystem-backed.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - `RawStorage` trait with methods `store_raw`, `load_secret_raw`, `remove_raw`, `contains_raw`, `list_raw` operating on raw byte vectors keyed by `KeyTypeId`. `TypedStorage<S>` wrapper that adds type-safe `store`, `load`, `remove`, `contains`, `list` methods using `KeyType` generics and `BytesEncoding` for serialization.
- `fs.rs` - `FileStorage` implementing `RawStorage` with filesystem persistence. Keys are stored as JSON-encoded `(public_bytes, secret_bytes)` tuples in files named by blake3 hash of the public key, organized in subdirectories per `KeyTypeId`. Creates directories on demand. Requires `std` feature.
- `in_memory.rs` - `InMemoryStorage` implementing `RawStorage` with `parking_lot::RwLock<BTreeMap<KeyTypeId, BTreeMap<Vec<u8>, Vec<u8>>>>`. Thread-safe, ephemeral storage suitable for testing and short-lived processes.

## Key APIs
- `RawStorage` trait - object-safe interface for key material storage
- `TypedStorage<S>` - type-safe wrapper adding `KeyType`-aware operations
- `FileStorage::new(path)` - creates filesystem storage at given path
- `InMemoryStorage::new()` - creates ephemeral in-memory storage

## Relationships
- `RawStorage` implementors are used by `crate::keystore::Keystore` as `Box<dyn RawStorage>` in `LocalStorageEntry`
- `InMemoryStorage` is the default backend when no other storage is configured
- `FileStorage` is registered when `KeystoreConfig::fs_root()` is set
- Used by `crate::keystore::backends::BackendConfig::Local`
