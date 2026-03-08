# src

## Purpose
Ed25519 signature scheme implementation using the `ed25519-zebra` library (Zcash's Ed25519 implementation with ZIP-215 verification rules). Provides key generation, signing, and verification conforming to the `blueprint-crypto-core` `KeyType` trait.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `lib.rs` - `Ed25519Zebra` key type implementing `KeyType`. `impl_zebra_serde!` macro generating newtype wrappers (`Ed25519SigningKey`, `Ed25519VerificationKey`, `Ed25519Signature`) with `BytesEncoding`, serde, `Eq`/`Ord`/`Hash`. Key generation supports seed-based (deterministic, zero-padded to 32 bytes) and random modes. Signing uses `ed25519_zebra::SigningKey::sign`, verification uses `VerificationKey::verify`.
- `error.rs` - `Ed25519Error` enum with variants: `InvalidSeed`, `InvalidVerifyingKey`, `InvalidSigner`, `InvalidSignature`, `ZebraError`, `HexError`. Type alias `Result<T>`.
- `tests.rs` - Tests for deterministic key generation, signature verification edge cases (empty/large messages, modified messages), key reuse, signature malleability resistance, cross-key verification, boundary conditions (various message sizes), key serialization round-trips, invalid deserialization, concurrent key usage across threads.

## Key APIs
- `Ed25519Zebra` struct -- `KeyType` implementation
- `Ed25519SigningKey` -- newtype around `ed25519_zebra::SigningKey`
- `Ed25519VerificationKey` -- newtype around `ed25519_zebra::VerificationKey`
- `Ed25519Signature` -- newtype around `ed25519_zebra::Signature`

## Relationships
- Depends on `blueprint-crypto-core` for `KeyType`, `KeyTypeId`, `BytesEncoding` traits
- Depends on `ed25519-zebra` for the underlying Ed25519 implementation
- Sibling to `blueprint-crypto-bls`, `blueprint-crypto-bn254`, `blueprint-crypto-k256`, `blueprint-crypto-sr25519`
- Re-exported through `blueprint-crypto` meta-crate
