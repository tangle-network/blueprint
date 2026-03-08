# src

## Purpose
Source directory for the blueprint keystore crate. Provides a unified interface for cryptographic key management across multiple backends (filesystem, in-memory) and remote signing services (AWS KMS, GCP KMS, Ledger hardware wallets), supporting all key types defined in `blueprint-crypto`.

## Contents (one hop)
### Subdirectories
- [x] `keystore/` - Core `Keystore` implementation with backend abstraction and configuration (`KeystoreConfig`); contains `backends/` subdirectory with concrete backend implementations
- [x] `remote/` - Remote signing integrations: AWS KMS (`aws.rs`), GCP KMS (`gcp.rs`), and Ledger hardware wallet (`ledger.rs`); feature-gated behind `aws-signer`, `gcp-signer`, `ledger-node`/`ledger-browser`
- [x] `storage/` - Storage backend trait and implementations: filesystem-based (`fs.rs`) and in-memory (`in_memory.rs`) key persistence

### Files
- `error.rs` - Comprehensive `Error` enum covering core keystore errors, crypto errors, and feature-gated remote signer errors (AWS, GCP, Ledger, Alloy); includes `impl_from_for_boxed_error!` macro
- `lib.rs` - Crate root; defines `cfg_remote!` macro for conditional compilation, declares modules, re-exports `keystore::*` and `blueprint_crypto as crypto`

## Key APIs (no snippets)
- `Keystore` (from `keystore/mod.rs`) - Main keystore interface re-exported at crate root
- `Error` enum - Unified error type with variants for I/O, key not found, crypto errors, and remote signer failures
- `cfg_remote!` macro - Conditionally compiles code when any remote signing feature is enabled
- `storage` module - `StorageBackend` trait with filesystem and in-memory implementations
- `remote` module - Feature-gated remote signing (AWS KMS, GCP KMS, Ledger)

## Relationships
- Depends on `blueprint-crypto` for key type definitions and operations
- Feature-gated dependencies: `alloy-signer-aws`, `alloy-signer-gcp`, `alloy-signer-ledger`, `gcloud-sdk`
- Used by `blueprint-runner`, `blueprint-manager`, and CLI for key management
- Central to all signing operations across Tangle, EigenLayer, and EVM networks

## Notes
- Supports `no_std` via feature gating (remote module requires std)
- Error type is `#[non_exhaustive]` to allow future extension
- Remote signing features are mutually independent
