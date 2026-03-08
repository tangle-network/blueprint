# src

## Purpose
Core cryptographic abstractions shared by all signature scheme crates. Defines the `KeyType` trait (the universal interface for key generation, signing, and verification), the `KeyTypeId` enum (discriminant for each supported scheme), the `BytesEncoding` trait for serialization, and the `AggregatableSignature` / `WeightedAggregatableSignature` traits for multi-party signature schemes.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `lib.rs` - `KeyTypeId` enum with feature-gated variants (`Bn254`, `Ecdsa`, `Sr25519`, `Bls381`, `Bls377`, `Ed25519`). `BytesEncoding` trait (`to_bytes`, `from_bytes`). `KeyType` trait requiring associated types `Secret`, `Public`, `Signature`, `Error` with extensive trait bounds (Clone, Serialize, Ord, Hash, etc.), plus methods: `key_type_id()`, `generate_with_seed()`, `generate_with_string()`, `public_from_secret()`, `sign_with_secret()`, `sign_with_secret_pre_hashed()`, `verify()`. Default `get_rng()` / `get_test_rng()` methods. Optional `clap::ValueEnum` impl for CLI integration. `impl_crypto_tests!` macro generating a standard test suite for any `KeyType` implementation.
- `aggregation.rs` - `AggregatableSignature` trait extending `KeyType` with `aggregate()` and `verify_aggregate()` methods plus associated types `AggregatedSignature` / `AggregatedPublic`. `WeightedAggregatableSignature` trait adding `verify_weighted_aggregate()` with a threshold parameter.

## Key APIs
- `KeyType` trait -- universal interface for all cryptographic key schemes
- `KeyTypeId` enum -- discriminant identifying the signature scheme
- `BytesEncoding` trait -- `to_bytes()` / `from_bytes()` for key and signature serialization
- `AggregatableSignature` trait -- `aggregate()` and `verify_aggregate()` for multi-signer schemes
- `WeightedAggregatableSignature` trait -- threshold-weighted aggregate verification
- `impl_crypto_tests!` macro -- generates standard test suite for `KeyType` implementors

## Relationships
- Depended upon by all signature scheme crates: `blueprint-crypto-bls`, `blueprint-crypto-bn254`, `blueprint-crypto-ed25519`, `blueprint-crypto-k256`, `blueprint-crypto-sr25519`
- Used by `blueprint-keystore` for key storage abstraction
- Re-exported through `blueprint-crypto` meta-crate
