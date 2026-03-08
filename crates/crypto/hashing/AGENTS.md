# hashing

## Purpose
Feature-gated hashing and key derivation function (KDF) primitives for Tangle Blueprints. Provides standalone hash functions and KDF utilities without pulling in full crypto scheme dependencies.

## Contents (one hop)
### Subdirectories
- [x] `src/` - Hash function implementations and KDF module

### Files
- `CHANGELOG.md` - Version history
- `Cargo.toml` - Crate manifest; all hash/KDF backends are optional: `sha2`, `sha3`, `blake3`, `hkdf`, `argon2`
- `README.md` - Crate documentation

## Key APIs (no snippets)
- `sha2_256(data)` / `sha2_512(data)` - SHA-2 hash functions (feature: `sha2-hasher`)
- `keccak_256(data)` - Keccak-256 hash (feature: `sha3-hasher`)
- `blake3_256(data)` - BLAKE3-256 hash (feature: `blake3-hasher`)
- `kdf::hkdf_sha256` - HKDF key derivation using SHA-256 (feature: `kdf-hkdf`)
- `kdf::argon2id_derive` / `kdf::argon2id_derive_with` - Argon2id password-based key derivation (feature: `kdf-argon2`)

## Relationships
- Depends only on `blueprint-std` (no dependency on `blueprint-crypto-core`)
- Used as a dev-dependency by `blueprint-crypto-bls` and `blueprint-crypto-bn254` for test hashing
- Independent utility crate consumed by any crate needing lightweight hashing

## Notes
- All features enabled by default (sha2, sha3, blake3, hkdf, argon2)
- Pure function interfaces; no state or trait implementations
- Supports `no_std` via feature gating
