# exchange

## Purpose
TEE key exchange and sealed secret handoff. Implements a two-phase X25519 Diffie-Hellman key exchange where the TEE generates an ephemeral keypair attested to the platform, and clients encrypt secrets to that key using ChaCha20Poly1305.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Re-exports `TeeAuthService` from the `service` submodule
- `protocol.rs` - `KeyExchangeSession` (ephemeral X25519 keypair with TTL, one-time-use decryption via `open()`); `KeyExchangeRequest`/`KeyExchangeResponse` (attestation-bound public key exchange); `SealedSecretPayload` (client-side `seal()` and TEE-side decryption); `SealedSecretResult` (audit record of successful injection)
- `service.rs` - `TeeAuthService` managing a session map with capacity limits, TTL enforcement, background cleanup loop, and atomic one-time-use session consumption

## Key APIs
- `KeyExchangeSession::new(ttl_secs)` - generates random X25519 keypair and session ID; private key is `Zeroize`-on-drop
- `KeyExchangeSession::open(payload)` - reconstructs shared secret via DH, derives ChaCha20Poly1305 key via SHA-256, decrypts sealed secret
- `SealedSecretPayload::seal(session_id, plaintext, tee_public_key)` - client-side encryption: generates ephemeral X25519, DH with TEE key, encrypts with ChaCha20Poly1305
- `TeeAuthService::create_session()` - creates a new session (evicts expired, enforces max capacity)
- `TeeAuthService::consume_session(id)` - atomically removes and returns a valid session (one-time use)
- `TeeAuthService::start_cleanup_loop()` - spawns background tokio task that periodically evicts expired sessions

## Relationships
- Depends on `crate::attestation::report::AttestationReport` for binding attestation to key exchange responses
- Depends on `crate::config::TeeKeyExchangeConfig` for session TTL and capacity limits
- Designed to be wrapped as a `BackgroundService` by the runner integration
- Consumed by runner TEE key-exchange HTTP endpoints
