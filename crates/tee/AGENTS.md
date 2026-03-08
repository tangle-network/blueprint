# tee

## Purpose
First-class Trusted Execution Environment (TEE) support for the Blueprint SDK. Provides runtime TEE capabilities including attestation verification, sealed-secret key exchange, and Tower middleware integration for blueprint services running in confidential compute environments (AWS Nitro, Azure CVM, GCP Confidential Space, Intel TDX, AMD SEV-SNP).

## Contents (one hop)
### Subdirectories
- [x] `src/` - Core modules: `attestation/` (report parsing, claims, verifier trait, per-platform verifiers), `exchange/` (ephemeral key exchange protocol, session management), `middleware/` (Tower layer + TeeContext extractor), `runtime/` (backend trait, direct mode, registry), `config.rs`, `errors.rs`
- [x] `tests/` - Unit/integration tests: `attestation_tests.rs`, `config_tests.rs`, `exchange_tests.rs`, `middleware_tests.rs`, `runtime_tests.rs`

### Files
- `Cargo.toml` - Crate manifest (`blueprint-tee`); provider features: `aws-nitro`, `azure-snp`, `gcp-confidential`, `tdx`, `sev-snp`, `all-providers`; also `test-utils` (default)
- `README.md` - Architecture diagram, deployment modes, security model documentation

## Key APIs (no snippets)
- `TeeConfig` / `TeeConfigBuilder` - Builder-pattern configuration: requirement level, mode, provider selection, key exchange settings, policies
- `TeeMode` - `Direct` (runner in TEE), `Remote` (cloud provisioned), `Hybrid` (selective per-job)
- `TeeRequirement` - `Required`, `Optional`, `Disabled`
- `TeeProvider` / `TeeProviderSelector` - Platform selection (AwsNitro, AzureSnp, GcpConfidential, Tdx, SevSnp)
- `AttestationVerifier` trait - Platform-specific attestation document verification
- `AttestationReport`, `AttestationClaims`, `VerifiedAttestation`, `Measurement`, `PublicKeyBinding` - Attestation data model
- `TeeAuthService` - Session management for key exchange (ephemeral, zeroized on drop, TTL-enforced)
- `KeyExchangeSession` - Ephemeral X25519 + ChaCha20-Poly1305 key exchange
- `TeeLayer` / `TeeContext` - Tower middleware that injects attestation into JobResult metadata; context extractor for job handlers
- `TeeRuntimeBackend` trait - deploy/attest/stop/destroy lifecycle for TEE workloads
- `BackendRegistry` - Type-erased multi-backend dispatch

## Relationships
- Depends on `blueprint-core` (tracing, job primitives), `blueprint-std`, `tower` (middleware)
- Crypto deps: `x25519-dalek`, `chacha20poly1305`, `sha2`, `zeroize`
- Optional cloud provider deps: `aws-sdk-ec2`/`aws-config` (aws-nitro), `reqwest` (azure-snp, gcp), `gcp_auth` (gcp-confidential)
- Consumed by `blueprint-runner` (feature `tee`) and re-exported by `blueprint-sdk` (feature `tee`)

## Notes
- Security model: sealed secrets are the only secret injection path; ephemeral key exchange sessions are one-time use with TTL; private key material zeroed on drop via `write_volatile`
- Attestation supports dual-path: local evidence validation plus optional on-chain hash comparison
- `test-utils` feature (default) enables `VerifiedAttestation::new_for_test` for unit testing without real TEE hardware
- Hybrid mode routing is contract-driven by default (`teeRequired` flag on-chain)
