# src

## Purpose
Schnorrkel/Ristretto SR25519 signature scheme implementation using the `schnorrkel` crate. Provides key generation, signing, and verification conforming to the `blueprint-crypto-core` `KeyType` trait. SR25519 is the primary signature scheme used by Substrate-based chains including Tangle Network.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `lib.rs` - `SchnorrkelSr25519` key type implementing `KeyType`. `impl_schnorrkel_serde!` macro generating newtype wrappers (`SchnorrkelPublic`, `SchnorrkelSecret`, `SchnorrkelSignature`) with `BytesEncoding`, serde, `Eq`/`Ord`/`Hash`. Key generation supports deterministic seed (max 64 bytes, zero-padded) and random modes (via `MiniSecretKey`). Signing uses `signing_context(b"tangle")` as the domain separator.
- `error.rs` - `Sr25519Error` enum with variants: `InvalidSeed`, `SignatureError` (wraps `schnorrkel::SignatureError`), `HexError`. Type alias `Result<T>`.
- `tests.rs` - Tests for deterministic key generation, pair serialization round-trips, signature verification edge cases (empty/large messages), cross-key verification, concurrent key usage across threads.

## Key APIs
- `SchnorrkelSr25519` struct -- `KeyType` implementation for SR25519
- `SchnorrkelPublic` -- newtype around `schnorrkel::PublicKey`
- `SchnorrkelSecret` -- newtype around `schnorrkel::SecretKey`
- `SchnorrkelSignature` -- newtype around `schnorrkel::Signature`

## Relationships
- Depends on `blueprint-crypto-core` for `KeyType`, `KeyTypeId`, `BytesEncoding` traits
- Depends on `schnorrkel` for the Ristretto-based Schnorr signature implementation
- Used by Tangle Network integrations for Substrate-compatible signing
- Sibling to `blueprint-crypto-bls`, `blueprint-crypto-bn254`, `blueprint-crypto-ed25519`, `blueprint-crypto-k256`
- Re-exported through `blueprint-crypto` meta-crate
