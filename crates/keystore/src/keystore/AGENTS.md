# keystore

## Purpose
Central `Keystore` struct that manages cryptographic key storage across multiple prioritized backends. Supports generating, storing, signing, listing, and removing keys of various types (ECDSA, BLS, Ed25519, Sr25519). Dispatches operations to registered local storage backends sorted by priority.

## Contents (one hop)
### Subdirectories
- [x] `backends/` - Defines the `Backend` trait with core key operations, `BackendConfig` enum, and specialized backend traits (`Bn254Backend`, `EvmBackend`, `EigenlayerBackend`) plus remote backend integration. See `backends/AGENTS.md`.

### Files
- `config.rs` - `KeystoreConfig` builder with methods for `in_memory(bool)`, `fs_root(path)`, and `remote(RemoteConfig)` (feature-gated). Falls back to in-memory storage when no backends are explicitly configured.
- `mod.rs` - `Keystore` struct holding `BTreeMap<KeyTypeId, Vec<LocalStorageEntry>>` for local backends and optionally remote entries. Implements the `Backend` trait, delegating to all registered storage backends. `Keystore::new(config)` registers backends based on `KeystoreConfig` settings. Includes tests for generate, sign, list across multiple key types.

## Key APIs
- `Keystore::new(KeystoreConfig)` - construct a keystore with configured backends
- `KeystoreConfig` - builder for storage backend selection (in-memory, filesystem, remote)
- `Backend` trait (from `backends/`) - `generate`, `insert`, `sign_with_local`, `list_local`, `first_local`, `contains_local`, `remove`, `get_secret`

## Relationships
- Uses `crate::storage::{RawStorage, InMemoryStorage, FileStorage}` as backing stores
- Uses `crate::remote::RemoteConfig` for remote signer configuration (feature-gated)
- Depends on `blueprint_crypto` for `KeyType`, `KeyTypeId`, `BytesEncoding` traits
- Consumed by blueprint contexts via the `KeystoreContext` trait
