# src

## Purpose
ECDSA signature scheme implementation on the secp256k1 (K-256) curve using the `k256` crate. Provides key generation, signing (including pre-hashed/recoverable), and verification conforming to the `blueprint-crypto-core` `KeyType` trait. Includes Alloy integration for EVM-compatible signing and address derivation.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `lib.rs` - `K256Ecdsa` key type implementing `KeyType`. `impl_serde_bytes!` macro generating newtype wrappers (`K256SigningKey`, `K256VerifyingKey`, `K256Signature`) with `BytesEncoding`, serde, `Eq`/`Ord`, `Display`. Key generation supports deterministic seed (max 32 bytes, zero-padded) and random modes. Pre-hashed signing uses `sign_prehash_recoverable`. `K256SigningKey` has additional methods: `verifying_key()`, `public()`, `alloy_key()` (returns `LocalSigner`), `alloy_address()` (returns Alloy `Address`).
- `error.rs` - `K256Error` enum with variants: `InvalidSeed`, `InvalidVerifyingKey`, `InvalidSigner`, `HexError`, `InvalidSignature`, `SignatureFailed`. Type alias `Result<T>`.
- `tests.rs` - Tests for deterministic key generation, signature verification edge cases, key reuse, signature malleability, cross-key verification, boundary conditions, key/signature serialization, invalid deserialization, concurrent usage, low-S normalization check.

## Key APIs
- `K256Ecdsa` struct -- `KeyType` implementation for secp256k1 ECDSA
- `K256SigningKey` -- newtype around `k256::ecdsa::SigningKey` with `alloy_key()` and `alloy_address()` helpers
- `K256VerifyingKey` -- newtype around `k256::ecdsa::VerifyingKey` (also `Copy`)
- `K256Signature` -- newtype around `k256::ecdsa::Signature`

## Relationships
- Depends on `blueprint-crypto-core` for `KeyType`, `KeyTypeId`, `BytesEncoding` traits
- Depends on `k256` for secp256k1 curve operations
- Depends on `alloy-signer-local` for EVM signing integration (`alloy_key()`, `alloy_address()`)
- Used by EigenLayer and EVM integrations for ECDSA signing
- Sibling to `blueprint-crypto-bls`, `blueprint-crypto-bn254`, `blueprint-crypto-ed25519`, `blueprint-crypto-sr25519`
- Re-exported through `blueprint-crypto` meta-crate
