# fixtures

## Purpose
TypeScript test fixtures for generating cryptographic challenge-response payloads used by the auth crate's integration tests. Produces signed `VerifyChallengeRequest` JSON bodies for both ECDSA and Sr25519 key types using Polkadot/Substrate dev keys.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `bun.lock` - Bun package manager lockfile for the fixture project.
- `package.json` - Declares the sole dependency on `@polkadot/keyring` for key derivation and signing.
- `sign.ts` - Derives Alice's ECDSA and Sr25519 keypairs from the Substrate dev phrase, signs a 32-byte challenge with each, and prints JSON request bodies to stdout.

## Key APIs (no snippets)
- `VerifyChallengeRequest` interface -- the JSON shape consumed by the Rust auth server (`pub_key`, `key_type`, `challenge`, `signature`, `expires_at`).
- Uses `secp256k1Sign` for ECDSA and `sr25519Sign` for Sr25519 signature generation.

## Relationships
- Consumed by integration tests in `crates/auth/` to verify challenge-response authentication flows.
- Depends on `@polkadot/keyring` and `@polkadot/util-crypto` for Substrate-compatible key derivation.
