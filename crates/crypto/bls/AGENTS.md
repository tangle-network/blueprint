# bls

## Purpose
BLS (Boneh-Lynn-Shacham) signature scheme implementation for Tangle Blueprints, providing BLS12-377 and BLS12-381 key types via the W3F `tnt-bls` library. Implements the `KeyType` trait from `blueprint-crypto-core` for both curve variants.

## Contents (one hop)
### Subdirectories
- [x] `src/` - BLS key type definitions (Bls377/Bls381), serialization macros, signature aggregation, error types, and tests

### Files
- `CHANGELOG.md` - Version history
- `Cargo.toml` - Crate manifest; depends on `blueprint-crypto-core` (with `bls` feature), `tnt-bls`, `ark-serialize`; optional `sha2` for aggregation
- `README.md` - Crate documentation

## Key APIs (no snippets)
- `bls377::W3fBls377` / `bls381::W3fBls381` - Key type marker structs implementing `KeyType`
- `W3fBls377Public`, `W3fBls377Secret`, `W3fBls377Signature` (and Bls381 equivalents) - Wrapper types with serde and `BytesEncoding` via `impl_w3f_serde!` macro
- `to_bytes` / `from_bytes` - Canonical serialization/deserialization helpers
- `aggregation` module (feature-gated on `aggregation`) - BLS signature aggregation
- `define_bls_key!` macro - Generates full `KeyType` implementations for both BLS curves

## Relationships
- Implements traits from `blueprint-crypto-core` (`KeyType`, `BytesEncoding`, `KeyTypeId`)
- Uses `tnt-bls` for underlying BLS cryptographic operations
- Dev-depends on `blueprint-crypto-hashing` for test hashing
- Consumed by `blueprint-keystore` and higher-level SDK crates

## Notes
- Supports `no_std` via feature gating
- Default features: `std` and `aggregation`
- Signing context is hardcoded to `b"tangle"`
