# core

## Purpose
Core cryptographic trait definitions and type identifiers for the Tangle Blueprint crypto subsystem. Defines the `KeyType` trait that all crypto scheme crates implement, the `KeyTypeId` enum for runtime key type discrimination, and the `BytesEncoding` trait for uniform serialization.

## Contents (one hop)
### Subdirectories
- [x] `src/` - Trait definitions (`KeyType`, `BytesEncoding`, `AggregatableSignature`, `WeightedAggregatableSignature`), `KeyTypeId` enum, `impl_crypto_tests!` macro, and optional `clap::ValueEnum` impl

### Files
- `CHANGELOG.md` - Version history
- `Cargo.toml` - Crate manifest; minimal dependencies (`blueprint-std`, `serde`, optional `clap`); feature flags gate `KeyTypeId` variants (`bn254`, `k256`, `sr25519-schnorrkel`, `bls`, `zebra`)
- `README.md` - Crate documentation

## Key APIs (no snippets)
- `KeyType` trait - Unified interface for all crypto schemes: key generation (from seed/string), public key derivation, signing, pre-hashed signing, verification
- `KeyTypeId` enum - Feature-gated variants: `Bn254`, `Ecdsa`, `Sr25519`, `Bls381`, `Bls377`, `Ed25519`
- `BytesEncoding` trait - `to_bytes()` / `from_bytes()` for uniform byte-level key/signature serialization
- `AggregatableSignature` trait - Defines `aggregate()` and `verify_aggregate()` for multi-signature schemes
- `WeightedAggregatableSignature` trait - Extends aggregation with threshold-weighted verification
- `impl_crypto_tests!` macro - Generates standard test suite (key generation, sign/verify, serialization, comparison) for any `KeyType` implementor

## Relationships
- Foundation crate for all `crates/crypto/*` scheme crates (bls, bn254, ed25519, k256, sr25519)
- Depended on by `blueprint-keystore` for key management
- Re-exported through `blueprint-crypto` meta-crate

## Notes
- Supports `no_std` via feature gating
- `KeyTypeId` variants are conditionally compiled based on enabled features
- Provides `get_rng()` (crypto-secure in std, deterministic in no_std) and `get_test_rng()` (always deterministic)
