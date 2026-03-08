# crypto

## Purpose
Crate `blueprint-crypto`: Meta-crate that unifies all cryptographic scheme implementations behind feature flags. Re-exports sub-crates for k256 (secp256k1 ECDSA), Sr25519 (Schnorrkel), Ed25519 (Zebra), BLS (BLS12-377/381), BN254 (alt-bn128), and hashing/KDF primitives. Provides a unified `CryptoCoreError` enum and `IntoCryptoError` trait for error conversion across schemes.

## Contents (one hop)
### Subdirectories
- [x] `bls/` - Sub-crate `blueprint-crypto-bls`: BLS12-377 and BLS12-381 signature schemes via W3F `tnt-bls`. Key types: `W3fBls377`, `W3fBls381` with aggregation support.
- [x] `bn254/` - Sub-crate `blueprint-crypto-bn254`: BN254 BLS signatures using arkworks. Key types: `ArkBlsBn254Public/Secret/Signature`. Includes `sign()`, `verify()`, `hash_to_curve()`. EigenLayer-compatible.
- [x] `core/` - Sub-crate `blueprint-crypto-core`: Core crypto trait definitions (`KeyType`, `KeyTypeId`, `BytesEncoding`). Feature flags gate which key type IDs are available. Depends on `blueprint-std`, `serde`, optional `clap`.
- [x] `ed25519/` - Sub-crate `blueprint-crypto-ed25519`: Ed25519 signatures via `ed25519-zebra`. Implements `KeyType` from core with Zebra backend.
- [x] `hashing/` - Sub-crate `blueprint-crypto-hashing`: Hashing and KDF primitives. Feature-gated support for SHA-2, SHA-3, BLAKE3 hashers and HKDF, Argon2 key derivation.
- [x] `k256/` - Sub-crate `blueprint-crypto-k256`: secp256k1 ECDSA via the `k256` crate. Includes Alloy integration (`alloy-signer-local`, `alloy-primitives`) for EVM-compatible signing.
- [x] `sr25519/` - Sub-crate `blueprint-crypto-sr25519`: Schnorrkel Sr25519 signatures. Substrate-compatible signing scheme.
- [x] `src/` - Crate root `lib.rs`: re-exports all sub-crates, defines `CryptoCoreError` enum and `IntoCryptoError` trait.

### Files
- `CHANGELOG.md` - Version history.
- `Cargo.toml` - Crate manifest (`blueprint-crypto`). Deps: all sub-crates as optional, `thiserror`. Default features enable all schemes: `k256`, `sr25519-schnorrkel`, `ed25519`, `bls`, `bn254`, `hashing`. Additional feature: `aggregation` (BLS + BN254 signature aggregation).
- `README.md` - Crate documentation.

## Key APIs (no snippets)
- Re-exports from `blueprint-crypto-core`: `KeyType` trait, `KeyTypeId` enum, `BytesEncoding` trait, and all core crypto abstractions.
- `k256` module -- secp256k1 ECDSA key types and signing (feature-gated).
- `sr25519` module -- Schnorrkel Sr25519 key types (feature-gated).
- `ed25519` module -- Zebra Ed25519 key types (feature-gated).
- `bls` module -- BLS12-377/381 key types with aggregation (feature-gated).
- `bn254` module -- BN254 BLS key types, sign/verify, hash-to-curve (feature-gated).
- `hashing` module -- SHA-2, SHA-3, BLAKE3 hashers and HKDF/Argon2 KDFs (feature-gated).
- `CryptoCoreError` enum -- unified error type with variants for each crypto scheme.
- `IntoCryptoError` trait -- converts scheme-specific errors into `CryptoCoreError`.

## Relationships
- Depends on all crypto sub-crates (`blueprint-crypto-core`, `-k256`, `-sr25519`, `-ed25519`, `-bls`, `-bn254`, `-hashing`).
- Used by `blueprint-auth` for challenge-response verification (BN254, ECDSA, Sr25519).
- Used by `blueprint-keystore` for key storage and retrieval across all supported schemes.
- Used by `blueprint-client-tangle` for on-chain signing (k256).
- `no_std` compatible across all sub-crates.
