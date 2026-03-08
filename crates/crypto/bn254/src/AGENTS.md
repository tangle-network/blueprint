# src

## Purpose
BLS signature scheme implementation on the BN254 (alt-bn128) curve using `ark-bn254`. Provides key generation, hash-to-curve signing, pairing-based verification, and signature aggregation. This is the curve used by EigenLayer for on-chain BLS signature verification.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `lib.rs` - Crate root. `hash_to_curve()` function mapping arbitrary bytes to a G1 point via SHA-256 + try-and-increment. `sign()` function multiplying the hashed point by the secret key scalar. `verify()` function checking `e(H(m), pk) == e(sig, G2)` via BN254 pairings. `ArkBlsBn254` key type implementing `KeyType` with `G2Affine` public keys, `Fr` secret keys, `G1Affine` signatures. `impl_ark_serde!` macro for newtype wrappers with `BytesEncoding` and serde support.
- `aggregation.rs` - `AggregatableSignature` impl for `ArkBlsBn254`. Aggregates by summing G1 signature points and G2 public key points. `verify_aggregate` delegates to `KeyType::verify`.
- `error.rs` - `Bn254Error` enum with variants: `InvalidSeed`, `SignatureFailed`, `SignatureNotInSubgroup`, `InvalidInput`. Type alias `Result<T>`.
- `tests.rs` - Comprehensive tests: key generation, signing/verification, hash-to-curve determinism and edge cases, serialization round-trips, aggregation success/failure/mismatched keys, corrupted signature handling.

## Key APIs
- `ArkBlsBn254` struct -- `KeyType` implementation for BN254 BLS
- `ArkBlsBn254Public` (G2Affine), `ArkBlsBn254Secret` (Fr), `ArkBlsBn254Signature` (G1Affine) -- newtype wrappers
- `sign(sk, message)` -- low-level signing function
- `verify(public_key, message, signature)` -- low-level pairing-based verification
- `hash_to_curve(digest)` -- maps bytes to G1Affine via try-and-increment
- `AggregatableSignature` impl -- `aggregate()`, `verify_aggregate()`

## Relationships
- Depends on `blueprint-crypto-core` for `KeyType`, `KeyTypeId`, `BytesEncoding`, `AggregatableSignature` traits
- Depends on `ark-bn254`, `ark-ec`, `ark-ff`, `ark-serialize` for curve arithmetic
- Depends on `blueprint-crypto-hashing` (in tests) for `keccak_256`
- Used by EigenLayer integrations for on-chain BLS verification
- Sibling to `blueprint-crypto-bls`, `blueprint-crypto-ed25519`, `blueprint-crypto-k256`, `blueprint-crypto-sr25519`
- Re-exported through `blueprint-crypto` meta-crate
