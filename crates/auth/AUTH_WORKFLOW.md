# Authentication Overview

This document describes how Blueprint proxies authenticate clients and authorize requests to blueprint services. It is implementation‑focused: flows, headers, endpoints, enforcement, storage, and per‑service configuration.

## Mechanisms

-   API Keys (long‑lived)

    -   Format: ak\_<key_id>.<secret>
    -   Stored hashed in RocksDB
    -   Must be exchanged for a short‑lived access token before use

-   PASETO Access Tokens (short‑lived)

    -   Format: v4.local.<encrypted_payload>
    -   Encrypted and validated only by the proxy
    -   Default TTL 15 minutes (per‑service configurable)

-   OAuth 2.0 JWT Bearer Assertion
    -   Client presents a short‑lived signed JWT assertion (RS256/ES256)
    -   Proxy verifies it and issues a short‑lived PASETO access token

Legacy id|token remains for backwards compatibility only.

## Core Flows

### 1) Challenge → API Key → Access Token

1. Request challenge
    - POST /v1/auth/challenge
    - Headers: X-Service-Id: <service_id>
    - Body: { pub_key, key_type }
2. Verify challenge
    - POST /v1/auth/verify
    - Headers: X-Service-Id: <service_id>
    - Body: { challenge, signature, ... }
    - Result: API key (long‑lived)
3. Exchange API key for access token
    - POST /v1/auth/exchange
    - Headers: Authorization: Bearer <ak\_...>, Content-Type: application/json
    - Body (optional): { additional_headers, ttl_seconds }
    - Result: short‑lived PASETO access token

### 2) OAuth JWT Assertion → Access Token

1. Present assertion
    - POST /v1/oauth/token
    - Headers: X-Service-Id: <service_id>, Content-Type: application/x-www-form-urlencoded
    - Body (form):
        - grant_type=urn:ietf:params:oauth:grant-type:jwt-bearer
        - assertion=<compact_jwt>
    - Result: short‑lived PASETO access token

### 3) Authorized Request (PASETO)

-   Call the proxy with Authorization: Bearer <v4.local...>
-   Proxy validates token, injects authorized headers (e.g., tenant), forwards to upstream

## Endpoints

-   POST /v1/auth/challenge — start signature challenge
-   POST /v1/auth/verify — verify signature and issue API key
-   POST /v1/auth/exchange — exchange API key for access token
-   POST /v1/oauth/token — exchange JWT assertion for access token
-   All other paths — reverse proxy after successful authorization

## Required Headers

-   X-Service-Id: <service_id> — required for challenge/verify and OAuth token issuance
-   Authorization: Bearer <token>
    -   API key (ak\_...) only for /v1/auth/exchange
    -   PASETO (v4.local...) for authorized requests

## Proxy Enforcement

-   Assertions (OAuth):

    -   Signature algorithms: RS256 or ES256
    -   Claims:
        -   iss ∈ per‑service allowed_issuers
        -   aud ∈ per‑service required_audiences (if configured)
        -   sub, iat, exp required; small clock skew tolerated (~60s)
        -   Assertion TTL (exp − iat) ≤ per‑service max_assertion_ttl_secs
    -   Replay prevention: jti stored until exp; re‑use rejected
    -   Tenant derivation: x-tenant-id = hash(sub) injected after validation
    -   Scopes: requested scopes (space‑delimited) intersected with per‑service allowed_scopes

-   Access tokens:

    -   PASETO v4.local; TTL capped per service
    -   Validated only by the proxy; upstream services trust injected headers

-   Header hygiene:
    -   Client‑supplied Authorization, x-tenant-\*, x-scope are stripped before injection
    -   Only validated, canonical headers are forwarded

## Storage (RocksDB CFs)

-   svs_usr_keys — services and owners
-   api_keys, api_keys_by_id — API key hashes and lookups
-   services_oauth_policy — per‑service OAuth policy
-   oauth_jti — JWT assertion replay cache (jti → exp)
-   oauth_rl — best‑effort rate limit buckets for /v1/oauth/token
-   seq — internal sequence counters

## Per‑Service Policy

Stored per ServiceId and loaded on issuance/verification:

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

Notes:

-   public_keys_pem enables offline verification without external fetches
-   For dynamic rotation, configure JWKS (future enhancement)

## Rate Limiting

-   /v1/oauth/token guarded by a simple per‑service/IP windowed counter (best‑effort) in oauth_rl
-   For strict multi‑replica enforcement, back with a shared KV (e.g., Redis)

## Operational Notes

-   Keep API keys server‑side; clients should only handle short‑lived access tokens
-   The proxy is the single trust boundary; upstream services rely on injected headers and do not validate tokens
