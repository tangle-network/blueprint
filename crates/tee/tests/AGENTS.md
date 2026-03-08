# tests

## Purpose
Integration and unit test suite for the Blueprint TEE crate covering attestation verification, configuration management, key exchange protocol, middleware integration, and runtime backend lifecycle. ~240 tests across 5 files, 2,748 lines total.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `attestation_tests.rs` - Attestation reports, verifiers (TDX/Nitro/SEV-SNP/Azure/GCP), measurement validation (~55 tests, 627 lines).
  - **Key items**: `AttestationReport`, `Measurement`, `AttestationClaims`, provider verifiers, freshness validation, debug mode enforcement
- `config_tests.rs` - Config builder, policies, enum serde, deserialization invariants, BackendRegistry (~60 tests, 687 lines).
  - **Key items**: `TeeConfig` builder, `TeeMode` (Disabled/Direct/Remote/Hybrid), `TeeProvider` (5 variants), `SecretInjectionPolicy`, `RuntimeLifecyclePolicy`, `BackendRegistry::from_env()`
- `exchange_tests.rs` - Key exchange sessions, X25519 ECDH, ChaCha20-Poly1305 AEAD, service lifecycle (~45 tests, 488 lines).
  - **Key items**: `KeyExchangeSession`, `TeeAuthService`, `SealedSecretPayload`, session TTL, max_sessions limits, seal/open round-trips
- `middleware_tests.rs` - TeeLayer middleware, TeeContext, metadata injection, router integration (~20 tests, 338 lines).
  - **Key items**: `TeeLayer`, `TeeContext`, metadata key injection into job results, lock contention graceful degradation
- `runtime_tests.rs` - Backend lifecycle, deployment states, registry management, public key derivation (~60 tests, 608 lines).
  - **Key items**: `DirectBackend`, `TeeDeployRequest`, `TeeDeploymentHandle`, `TeeDeploymentStatus` (Provisioning/Running/Stopped/Destroyed), `TeePublicKey`, deterministic key derivation

## Key APIs (no snippets)
- **Attestation**: `AttestationReport`, `Measurement`, verifiers per provider, freshness validation
- **Config**: `TeeConfig` builder, `TeeMode`, `TeeProvider`, policies
- **Exchange**: `KeyExchangeSession` (X25519), `TeeAuthService`, `SealedSecretPayload` (ChaCha20-Poly1305)
- **Middleware**: `TeeLayer` (Tower service), `TeeContext`
- **Runtime**: `DirectBackend`, `TeeDeploymentHandle`, `TeeDeploymentStatus`, `BackendRegistry`

## Relationships
- **Depends on**: `blueprint-tee` (parent crate), `blueprint-core`, `blueprint-router`, `tokio`, `sha2`, `x25519-dalek`, `chacha20poly1305`
- **Used by**: CI pipeline for TEE subsystem validation

## Notes
- Cryptography: X25519 (32-byte keys), ChaCha20-Poly1305 (16-byte AEAD tag), SHA-256, HMAC-SHA256
- Feature-gated: provider verifiers behind cargo feature flags (#[cfg(feature = "tdx")], etc.)
- State machines: Deployment (Provisioning->Running->Stopped->Destroyed), Session (active->expired)
- TeeLayer uses `try_lock` for graceful degradation on contention
- 85+ async tokio tests; service methods use Arc<Mutex<>> for thread safety
