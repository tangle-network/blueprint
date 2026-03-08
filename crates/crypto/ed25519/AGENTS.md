# ed25519

## Purpose
Ed25519 signature scheme implementation for Tangle Blueprints using the `ed25519-zebra` library. Provides key generation, signing, and verification with the Edwards Curve 25519.

## Contents (one hop)
### Subdirectories
- [x] `src/` - Ed25519 key type, wrapper types with serde, error definitions, and tests

### Files
- `CHANGELOG.md` - Version history
- `Cargo.toml` - Crate manifest; depends on `blueprint-crypto-core` (with `zebra` feature), `ed25519-zebra`
- `README.md` - Crate documentation

## Key APIs (no snippets)
- `Ed25519Zebra` - Key type marker struct implementing `KeyType`
- `Ed25519SigningKey` (wraps `ed25519_zebra::SigningKey`) - Secret key with `BytesEncoding` and serde
- `Ed25519VerificationKey` (wraps `ed25519_zebra::VerificationKey`) - Public key with `BytesEncoding` and serde
- `Ed25519Signature` (wraps `ed25519_zebra::Signature`) - Signature with `BytesEncoding` and serde
- `impl_zebra_serde!` macro - Generates serde, comparison, hashing, and `BytesEncoding` impls for wrapper types

## Relationships
- Implements traits from `blueprint-crypto-core` (`KeyType`, `BytesEncoding`, `KeyTypeId::Ed25519`)
- Consumed by `blueprint-keystore` for key storage and management
- Re-exported through `blueprint-crypto` meta-crate

## Notes
- Supports `no_std` via feature gating
- Seed handling pads to 32 bytes if shorter; uses RNG if no seed provided
