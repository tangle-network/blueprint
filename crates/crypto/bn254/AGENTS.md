# bn254

## Purpose
BLS signature scheme over the BN254 (alt-bn128) curve using arkworks libraries. Provides key generation, signing, verification, and hash-to-curve functionality. Used for EigenLayer-compatible BLS operations.

## Contents (one hop)
### Subdirectories
- [x] `src/` - BN254 key type implementation, aggregation, error types, and tests

### Files
- `CHANGELOG.md` - Version history
- `Cargo.toml` - Crate manifest; depends on `blueprint-crypto-core` (with `bn254` feature), `ark-bn254`, `ark-ec`, `ark-ff`, `ark-serialize`, `sha2`, `num-bigint`
- `README.md` - Crate documentation

## Key APIs (no snippets)
- `ArkBlsBn254` - Key type marker struct implementing `KeyType`
- `ArkBlsBn254Public` (wraps `G2Affine`), `ArkBlsBn254Secret` (wraps `Fr`), `ArkBlsBn254Signature` (wraps `G1Affine`) - Wrapper types with serde via `impl_ark_serde!` macro
- `sign(sk, message)` - Signs a message with a secret key, hashing to G1
- `verify(public_key, message, signature)` - Verifies a BN254 BLS signature using pairing checks
- `hash_to_curve(digest)` - Maps a digest to a point on the G1 curve
- `aggregation` module - BN254 signature aggregation

## Relationships
- Implements traits from `blueprint-crypto-core` (`KeyType`, `BytesEncoding`, `KeyTypeId::Bn254`)
- Dev-depends on `blueprint-crypto-hashing` (sha3 feature) for tests
- Used by EigenLayer integration paths that require BN254 BLS signatures

## Notes
- Supports `no_std` via feature gating
- Hash-to-curve uses incremental x-coordinate search for valid curve points
- Pairing-based verification: checks `e(H(m), pk) == e(sig, G2_generator)`
