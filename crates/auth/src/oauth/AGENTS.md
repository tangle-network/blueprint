# oauth

## Purpose
Implements OAuth 2.0 JWT Bearer Assertion grant (RFC 7523) for service-to-service authentication. Verifies incoming JWT assertions with RS256/ES256, enforces per-service policies (issuers, audiences, scopes, TTL), provides JTI replay protection and rate limiting, then issues PASETO access tokens.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Assertion verification engine. `ServiceOAuthPolicy` (per-service config: issuers, audiences, public keys, scopes, DPoP, TTL), `AssertionClaims`, `VerifiedAssertion`, `AssertionVerifier` (JWT verification with RS256/ES256, JTI replay via `DashSet`, configurable window), `VerificationError` enum, `rate_limit_check()`.
- `token.rs` - OAuth token endpoint handler. `oauth_token()` axum handler implementing the `urn:ietf:params:oauth:grant-type:jwt-bearer` flow: loads per-service policy, rate-limits, verifies assertion, intersects requested scopes with allowed scopes, generates PASETO access token.

## Key APIs
- `AssertionVerifier::verify(&self, assertion: &str) -> Result<VerifiedAssertion, VerificationError>` - validates JWT signature, claims, and replay
- `ServiceOAuthPolicy` - per-service configuration struct controlling allowed issuers, audiences, public keys, scope sets, DPoP requirements, and TTL
- `oauth_token()` - axum handler at the token endpoint; returns JSON `{ access_token, token_type, expires_in, scope }`
- `rate_limit_check(service_id, policy) -> Result<()>` - per-service rate limiting guard

## Relationships
- Consumed by the auth crate's router as the `/oauth/token` endpoint
- Uses PASETO token generation from the parent `crate::token` module
- `ServiceOAuthPolicy` is loaded per-service, enabling multi-tenant OAuth configurations
