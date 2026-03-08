# src

## Purpose
Feature-gated cryptographic hash functions and key derivation functions (KDFs) for Tangle Blueprints. Provides thin wrappers around standard hash algorithms (SHA2, Keccak, BLAKE3) and KDFs (HKDF-SHA256, Argon2id) with a minimal dependency footprint controlled by cargo features.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `lib.rs` - Top-level hash functions: `sha2_256()` and `sha2_512()` (feature `sha2`), `keccak_256()` (feature `sha3`), `blake3_256()` (feature `blake3`). Conditionally declares the `kdf` module (features `kdf-hkdf` or `kdf-argon2`).
- `kdf.rs` - Key derivation functions. `KdfError` enum (`HkdfExpandFailed`, `Argon2Failed`). `hkdf_sha256<N>()` implementing RFC 5869 HKDF-SHA256 extract-and-expand (feature `kdf-hkdf`). `Argon2idConfig` struct with OWASP-recommended defaults (19 MiB, 2 iterations, 1 lane). `argon2id_derive<N>()` convenience function and `argon2id_derive_with<N>()` with custom config (feature `kdf-argon2`). Tests include RFC 5869 test vectors for HKDF and determinism/differentiation tests for Argon2id.

## Key APIs
- `sha2_256(data) -> [u8; 32]` -- SHA2-256 hash
- `sha2_512(data) -> [u8; 64]` -- SHA2-512 hash
- `keccak_256(data) -> [u8; 32]` -- Keccak-256 hash
- `blake3_256(data) -> [u8; 32]` -- BLAKE3-256 hash
- `kdf::hkdf_sha256<N>(ikm, salt, info) -> Result<[u8; N], KdfError>` -- HKDF-SHA256
- `kdf::argon2id_derive<N>(password, salt) -> Result<[u8; N], KdfError>` -- Argon2id with defaults
- `kdf::argon2id_derive_with<N>(password, salt, config) -> Result<[u8; N], KdfError>` -- Argon2id with custom params
- `kdf::Argon2idConfig` struct -- tuning parameters for Argon2id

## Relationships
- Used by `blueprint-crypto-bls` (tests use `sha2_256`) and `blueprint-crypto-bn254` (tests use `keccak_256`)
- Used across the workspace wherever hashing or key derivation is needed
- Re-exported through `blueprint-crypto` meta-crate
