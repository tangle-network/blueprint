# src

## Purpose
Implementation of the `blueprint-tee` crate, providing first-class TEE (Trusted Execution Environment) support for blueprint services. Covers attestation verification, key exchange, Tower middleware integration, and runtime backends for multiple cloud TEE providers (AWS Nitro, Azure SNP, GCP Confidential Space, Intel TDX, AMD SEV-SNP).

## Contents (one hop)
### Subdirectories
- [x] `attestation/` - Attestation framework: `AttestationReport` generation, `AttestationVerifier` trait, `AttestationClaims` with measurements and public key bindings, `VerificationLevel` enum, and provider-specific implementations in `providers/` subdirectory
- [x] `exchange/` - TEE key exchange protocol: `TeeAuthService` for attestation-authenticated key negotiation between TEE instances, protocol message types, and service implementation
- [x] `middleware/` - Tower middleware for TEE context injection: `TeeLayer` that wraps job handlers, `TeeContext` providing attestation state and TEE metadata to job functions
- [x] `runtime/` - TEE runtime backends: `TeeRuntimeBackend` trait, provider implementations (`aws_nitro.rs`, `azure_skr.rs`, `gcp_confidential.rs`, `direct.rs`), `BackendRegistry` for dynamic backend selection, deployment handle lifecycle management

### Files
- `lib.rs` - Crate root; declares modules, re-exports configuration types (`TeeConfig`, `TeeMode`, `TeeProvider`, `TeeRequirement`, etc.), attestation types, middleware types, and runtime types
- `config.rs` - Core configuration: `TeeMode` enum (Disabled/Direct/Remote/Hybrid), `TeeRequirement` (Preferred/Required), `TeeProvider` enum, `TeeConfig` builder with policies for attestation freshness, secret injection, public key handling, and runtime lifecycle
- `errors.rs` - `TeeError` enum for TEE-specific failures

## Key APIs (no snippets)
- `TeeConfig` / `TeeConfigBuilder` - Builder-pattern configuration for TEE integration
- `TeeMode` - Operational modes: Disabled, Direct (in-TEE), Remote (cloud TEE), Hybrid
- `TeeProvider` - Supported TEE hardware: AWS Nitro, Azure SNP, GCP Confidential, Intel TDX, AMD SEV-SNP
- `AttestationReport` / `AttestationVerifier` - Attestation generation and verification
- `TeeAuthService` - Key exchange service authenticated by attestation
- `TeeLayer` / `TeeContext` - Tower middleware injecting TEE context into job handlers
- `TeeRuntimeBackend` / `BackendRegistry` - Pluggable runtime backends for TEE deployment
- `TeeDeployRequest` / `TeeDeploymentHandle` - Deployment lifecycle management

## Relationships
- Integrated into `blueprint-runner` via `BlueprintRunnerBuilder::tee()` method
- `TeeLayer` composes with `blueprint-router` as a Tower middleware layer
- Used by `crates/manager/src/rt/` for TEE-aware service spawning
- Configuration types referenced by `blueprint-runner::config::BlueprintEnvironment`
