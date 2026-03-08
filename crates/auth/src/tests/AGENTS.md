# tests

## Purpose
Integration and unit test suite for the auth crate covering API key lifecycle, token exchange, OAuth flows, mTLS, gRPC proxy, multi-tenancy isolation, and security hardening.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Module declarations for all 8 test modules.
- `advanced_tests.rs` - PASETO key persistence across restarts, service deletion impact on keys, max header size validation, PII hashing for tenant isolation, already-hashed tenant ID preservation.
- `api_key_lifecycle_tests.rs` - API key rotation, renewal, revocation, expiration rejection, concurrent key operations.
- `grpc_proxy_tests.rs` - gRPC proxy integration with `TestEchoService`: unary and streaming round-trips, HTTP/1.1 downgrade rejection, forbidden header stripping, content-type validation.
- `mtls_flow_tests.rs` - mTLS workflow via `MtlsTestHarness`: TLS profile configuration, certificate issuance, TTL policy enforcement, plaintext rejection, mTLS gRPC success path.
- `multi_tenancy_tests.rs` - Tenant isolation enforcement, tenant impersonation prevention, tier-based rate limiting, tenant data isolation.
- `oauth_tests.rs` - OAuth RS256 success, HS256 rejection, expired/future IAT rejection, JTI replay rejection, scope forwarding and normalization, client scope stripping.
- `security_isolation_tests.rs` - Cross-user API key isolation, PASETO token isolation, concurrent multi-user auth, header injection security.
- `token_exchange_tests.rs` - Token exchange flow (API key to PASETO), invalid/missing API key handling, forbidden headers in exchange, reverse proxy with PASETO forwarding.

## Key APIs
- Test modules use `#[tokio::test]` throughout
- `MtlsTestHarness` - sets up TLS certificates and server for mTLS testing
- `TestEchoService` - mock gRPC service for proxy tests

## Relationships
- Tests the public API surface of `crate::oauth`, `crate::token`, `crate::mtls`, `crate::grpc_proxy`, and `crate::api_key` modules
- Depends on `axum::test` helpers and `tonic` for gRPC test infrastructure
