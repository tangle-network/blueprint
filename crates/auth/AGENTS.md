# auth

## Purpose
Crate `blueprint-auth`: HTTP/WebSocket authentication system for Blueprint services. Provides a three-tier token model (API keys, PASETO access tokens, legacy tokens), challenge-response verification for ECDSA/Sr25519/BN254 key types, an authenticated reverse proxy built on Axum, mTLS certificate authority, TLS envelope encryption, and OAuth 2.0 JWT assertion support. Includes a standalone `auth-server` binary (behind the `standalone` feature). Persistent storage via RocksDB with Protobuf-encoded models.

## Contents (one hop)
### Subdirectories
- [x] `fixtures/` - TypeScript test fixtures using `@polkadot/keyring` to generate signed challenge-response payloads for ECDSA and Sr25519 integration tests.
- [x] `proto/` - Protocol Buffer definitions for a test echo gRPC service (`EchoService`) used to verify authenticated gRPC proxy functionality.
- [x] `src/` - All crate source: modules for API keys, PASETO tokens, auth token unification, certificate authority, RocksDB storage, models, OAuth, proxy server, request auth extractors, TLS assets/client/envelope/listener, types, and header validation. Contains `main.rs` for the standalone binary.

### Files
- `AUTH_WORKFLOW.md` - Documentation of the authentication workflow.
- `CHANGELOG.md` - Version history.
- `Cargo.toml` - Crate manifest (`blueprint-auth`). Key deps: `axum`, `tower`/`tower-http`, `k256`, `schnorrkel`, `blueprint-crypto` (bn254), `pasetors`, `jsonwebtoken`, `rocksdb`, `prost`, `rcgen`, `chacha20poly1305`, `tokio-rustls`. Features: `std`, `standalone`, `tracing`.
- `README.md` - Crate documentation.
- `build.rs` - Runs `tonic-build` to compile `.proto` files for gRPC proxy tests.

## Key APIs (no snippets)
- `generate_challenge(rng)` -- produces a 32-byte random challenge for client signing.
- `verify_challenge(challenge, signature, pub_key, key_type)` -- dispatches verification to ECDSA, Sr25519, or BN254 BLS.
- `proxy::AuthenticatedProxy` -- Axum-based reverse proxy with built-in auth middleware, key/token management, and TLS termination.
- `api_keys` module -- long-lived API key generation and validation (`ak_xxxxx.yyyyy` format).
- `paseto_tokens` module -- short-lived PASETO v4 local token creation and validation.
- `auth_token::AuthToken` -- unified enum over API key, access token, and legacy token formats.
- `certificate_authority` module -- mTLS CA for generating and managing service certificates.
- `tls_envelope` module -- envelope encryption for TLS certificate material using ChaCha20Poly1305.
- `oauth` module -- OAuth 2.0 JWT assertion verifier with per-service policy.
- `db` module -- RocksDB-backed persistent storage for keys and service models.

## Relationships
- Depends on `blueprint-core` for tracing macros and core types.
- Depends on `blueprint-crypto` (bn254 feature) for BN254 BLS signature verification.
- Depends on `blueprint-std` for RNG and standard library abstractions.
- Used by the Blueprint Manager and remote provider infrastructure for service authentication.
- Dev-depends on `blueprint-sdk` for integration tests.
