# blueprint-auth

Authentication and authorization toolkit for Blueprint services.

This crate provides request auth primitives and an auth proxy surface (API keys, short-lived access tokens, and policy-aware request validation) for HTTP/WS service edges.

## What it includes

- API key + token flows
- Request auth context extraction
- Proxy/server auth modules
- TLS and certificate helpers for secure service edges

## Related links

- Source: https://github.com/tangle-network/blueprint/tree/main/crates/auth
- Auth proxy docs: https://docs.tangle.tools/developers/blueprint-auth

## Resource owner sessions

`owner::ScopedOwnerAuth` supplies EIP-191 `personal_sign` recovery, one-use
challenges, PASETO issuance, and live authorization for one `ResourceScope`.
`new` uses registered ECDSA service owners; `with_owner` accepts an async
`OwnerPredicate` that returns the authoritative expiry and allowed scopes.
Existing proxy participant endpoints and their other key types are unchanged.

The server chooses `OwnerDomain`: use a stable audience and include the chain,
operator, and vault/contract context when applicable. Both the signed challenge
and token bind this domain, the service, resource, and signer. Pass the canonical
resource from the protected route to `authorize`; require the operation's scope.
Do not trust a caller-provided domain or treat token validation alone as access.
The predicate runs again at issuance and access, so revoked ownership, shortened
expiry, or removed capabilities fail closed. Tokens never outlive the predicate's
authorization or the token manager's configured lifetime.

The caller supplies the `PasetoTokenManager` and persists its key through the
existing secure key path when restart continuity is required. Pending challenges
are bounded and process-local: restarting invalidates them. Replicas must route
challenge verification to the issuing instance. This API supplies authentication;
the application remains responsible for its authoritative ownership source.
