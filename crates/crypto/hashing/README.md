# blueprint-crypto-hashing

Hashing and key-derivation primitives for Blueprint services.

## Feature-gated capabilities

- Hashing: SHA2, SHA3/Keccak, BLAKE3.
- KDF: HKDF-SHA256 and Argon2id (`kdf` module).

## When to use

Use this crate for deterministic hashing/KDF logic shared by signing, auth, and protocol flows.

## Related links

- Source: https://github.com/tangle-network/blueprint/tree/main/crates/crypto/hashing
