# k256

## Purpose
ECDSA signature scheme implementation over the secp256k1 (k256) curve for Tangle Blueprints. Provides Ethereum-compatible key operations including Alloy wallet integration for EVM signing.

## Contents (one hop)
### Subdirectories
- [x] `src/` - K256 ECDSA key type, wrapper types with serde, Alloy integration helpers, error definitions, and tests

### Files
- `CHANGELOG.md` - Version history
- `Cargo.toml` - Crate manifest; depends on `blueprint-crypto-core` (with `k256` feature), `k256` (ecdsa), `alloy-signer-local`, `alloy-primitives`
- `README.md` - Crate documentation

## Key APIs (no snippets)
- `K256Ecdsa` - Key type marker struct implementing `KeyType`
- `K256SigningKey` - Secret key wrapper with `alloy_key()` and `alloy_address()` methods for EVM compatibility
- `K256VerifyingKey` - Public key wrapper (implements `Copy`)
- `K256Signature` - Signature wrapper
- `K256SigningKey::alloy_key()` - Returns `LocalSigner<SigningKey>` for Alloy/EVM transaction signing
- `K256SigningKey::alloy_address()` - Returns the Ethereum `Address` derived from the key

## Relationships
- Implements traits from `blueprint-crypto-core` (`KeyType`, `BytesEncoding`, `KeyTypeId::Ecdsa`)
- Bridges to Alloy ecosystem via `alloy-signer-local` for EVM transaction signing
- Consumed by `blueprint-keystore` for key management
- Re-exported through `blueprint-crypto` meta-crate

## Notes
- Supports `no_std` via feature gating
- Seed must not exceed 32 bytes
- Uses SEC1 encoding for public key serialization
- Supports pre-hashed signing with recoverable signatures
