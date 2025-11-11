# OAuth Module

This module implements the OAuth 2.0 JWT Bearer Assertion grant at the proxy to mint short-lived PASETO access tokens. It keeps the reverse proxy as the single enforcement point while supporting first/third-party issuers with clear per-service policy.

## Components

- mod.ts
  - ServiceOAuthPolicy: per-service policy stored in RocksDB (issuers, audiences, PEM public keys, scopes, DPoP flag, TTL caps)
  - AssertionVerifier: verifies signed JWT assertions (RS256/ES256), validates claims, enforces TTL, prevents replays, parses scopes
  - rate_limit_check: best-effort windowed rate limiting using RocksDB counters
- token.ts
  - POST /v1/oauth/token: form-encoded endpoint; enforces grant type; loads policy; verifies; derives tenant; issues PASETO; returns no-store responses

## Verification Rules

- Signature: RS256/ES256 using per-service public_keys_pem; at least one must validate
- Claims: iss in allowed_issuers; aud in required_audiences (if set); sub required
- Time: iat/exp validated with small skew; (exp - iat) ≤ max_assertion_ttl_secs
- Replay: jti stored in RocksDB until exp; replays are rejected
- Scopes: requested scopes (space-delimited) are intersected with allowed_scopes
- Tenant: x-tenant-id = hash(sub) injected via validated headers

## Configuration (per service)

```jsonc
{
  "allowed_issuers": ["https://issuer.example.com"],
  "required_audiences": ["https://proxy.example.com"],
  "public_keys_pem": ["-----BEGIN PUBLIC KEY-----..."],
  "allowed_scopes": ["data:read", "data:write"],
  "require_dpop": false,
  "max_access_token_ttl_secs": 900,
  "max_assertion_ttl_secs": 120
}
```

## Future Work (recommended)

- JWKS fetch/cache: configure issuer JWKS URLs; cache by kid; rotate safely; reduces PEM maintenance
- DPoP enforcement: when require_dpop = true, verify proofs and bind tokens to key thumbprints
- Admin tooling: CLI/RPC to manage ServiceOAuthPolicy
- Stronger rate limiting: shared KV (e.g., Redis) and per-issuer limits across replicas
- Observability: metrics for issuance/denials, replay hits, RL hits; structured logs and alerts
