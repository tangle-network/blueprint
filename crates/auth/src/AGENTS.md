# src

## Purpose
Source directory for the `blueprint-auth` crate, implementing a three-tier token authentication system for the Blueprint SDK. Provides API key management, short-lived Paseto access tokens, challenge-response verification (ECDSA, Sr25519, BN254 BLS), mTLS certificate management, OAuth 2.0, and an authenticated reverse proxy built on Axum.

## Contents (one hop)
### Subdirectories
- [x] `oauth/` - OAuth 2.0 JWT assertion verifier and per-service policy enforcement. Contains `mod.rs` and `token.rs`.
- [x] `tests/` - Integration tests covering advanced auth flows, API key lifecycle, gRPC proxy, mTLS flows, multi-tenancy, OAuth, security isolation, and token exchange.

### Files
- `api_keys.rs` - Long-lived API key generation, storage, and lookup
- `api_tokens.rs` - API token generation for the authentication flow
- `auth_token.rs` - Unified authentication token enum handling API keys, Paseto access tokens, and legacy token formats
- `certificate_authority.rs` - Certificate Authority utilities for mTLS certificate generation
- `db.rs` - RocksDB-backed persistent storage for authentication data
- `lib.rs` - Crate root; defines the `Error` enum, `generate_challenge`, and `verify_challenge` dispatching to ECDSA/Sr25519/BN254 BLS verification
- `main.rs` - Standalone binary entry point that starts the authenticated proxy server
- `models.rs` - Database models for services, owners, and TLS profiles
- `paseto_tokens.rs` - Paseto v4.local token generation and validation
- `proxy.rs` - `AuthenticatedProxy` reverse proxy server built on Axum with per-service routing
- `request_auth.rs` - Request-level auth context parsing and extractors for incoming HTTP requests
- `request_extensions.rs` - Request extension plumbing for client certificate identity propagation
- `test_client.rs` - Test-only HTTP client for integration tests
- `tls_assets.rs` - TLS certificate and key asset management (loading, storing, rotating)
- `tls_client.rs` - TLS client configuration for outbound mTLS connections
- `tls_envelope.rs` - TLS envelope encryption for certificate material at rest
- `tls_listener.rs` - Dual-socket TCP listener supporting both HTTP and HTTPS
- `types.rs` - Core types including `KeyType`, `ServiceId`, `VerifyChallengeRequest`, and related structs
- `validation.rs` - HTTP header validation utilities to prevent injection attacks

## Key APIs
- `generate_challenge(rng)` -- produces a 32-byte random challenge for client signing
- `verify_challenge(challenge, signature, pub_key, key_type)` -- verifies a signed challenge across ECDSA, Sr25519, and BN254 BLS key types
- `AuthenticatedProxy::new(db_path)` -- creates a new proxy with persistent RocksDB storage
- `Error` enum -- unified error type covering crypto, DB, TLS, and IO failures

## Relationships
- Depends on `blueprint-crypto` for BN254 BLS verification, `blueprint-std` for RNG
- Used by `blueprint-manager` auth proxy integration and remote provider auth flows
- The `main.rs` binary provides a standalone auth proxy server for development and production use
