# sr25519

## Purpose
Schnorrkel/Ristretto SR25519 signature scheme implementation for Tangle Blueprints. The standard Substrate-compatible signature scheme used for validator and operator key operations.

## Contents (one hop)
### Subdirectories
- [x] `src/` - SR25519 key type, wrapper types with serde, error definitions, and tests

### Files
- `CHANGELOG.md` - Version history
- `Cargo.toml` - Crate manifest; depends on `blueprint-crypto-core` (with `sr25519-schnorrkel` feature), `schnorrkel`
- `README.md` - Crate documentation

## Key APIs (no snippets)
- `SchnorrkelSr25519` - Key type marker struct implementing `KeyType`
- `SchnorrkelPublic` (wraps `schnorrkel::PublicKey`) - Public key with `BytesEncoding` and serde
- `SchnorrkelSecret` (wraps `schnorrkel::SecretKey`) - Secret key with `BytesEncoding` and serde
- `SchnorrkelSignature` (wraps `schnorrkel::Signature`) - Signature with `BytesEncoding` and serde
- `impl_schnorrkel_serde!` macro - Generates serde, comparison, hashing, and `BytesEncoding` impls

## Relationships
- Implements traits from `blueprint-crypto-core` (`KeyType`, `BytesEncoding`, `KeyTypeId::Sr25519`)
- Consumed by `blueprint-keystore` for Substrate-compatible key management
- Re-exported through `blueprint-crypto` meta-crate

## Notes
- Supports `no_std` via feature gating
- Signing context is `b"tangle"`
- Key generation from seed uses `MiniSecretKey` with `UNIFORM_MODE` expansion
- Seed must not exceed 64 bytes
