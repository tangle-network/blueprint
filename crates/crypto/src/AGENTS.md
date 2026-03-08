# src

## Purpose
Meta-crate source that unifies all cryptographic scheme implementations behind feature flags and provides a top-level `CryptoCoreError` enum with `IntoCryptoError` conversion trait for seamless error propagation across schemes.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `lib.rs` - Feature-gated re-exports of five crypto backends (`k256`, `sr25519`, `ed25519`, `bls`, `bn254`) plus `hashing`. Defines `CryptoCoreError` enum wrapping each backend's error type and `IntoCryptoError` trait with implementations for all backends. Supports `no_std`.

## Key APIs (no snippets)
- `k256` module (feature `k256`) -- re-export of `blueprint-crypto-k256` (secp256k1/ECDSA).
- `sr25519` module (feature `sr25519-schnorrkel`) -- re-export of `blueprint-crypto-sr25519` (Schnorrkel).
- `ed25519` module (feature `ed25519`) -- re-export of `blueprint-crypto-ed25519`.
- `bls` module (feature `bls`) -- re-export of `blueprint-crypto-bls`.
- `bn254` module (feature `bn254`) -- re-export of `blueprint-crypto-bn254`.
- `hashing` module (feature `hashing`) -- re-export of `blueprint-crypto-hashing`.
- `CryptoCoreError` -- unified error enum with variants for each enabled crypto backend.
- `IntoCryptoError` trait -- converts backend-specific errors into `CryptoCoreError`.
- All `blueprint-crypto-core` types are re-exported at the crate root.

## Relationships
- Depends on `blueprint-crypto-core` (always), plus `blueprint-crypto-{k256,sr25519,ed25519,bls,bn254,hashing}` behind respective features.
- Consumed by `blueprint-sdk` as the unified cryptography interface.
- Used by auth, networking, and client crates that need multi-scheme cryptographic operations.
