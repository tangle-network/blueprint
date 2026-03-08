# runtime

## Purpose
TEE runtime backend abstraction layer. Defines the lifecycle contract for TEE deployments (deploy, attest, stop, destroy) and provides a type-erased registry for dispatching across multiple cloud/local backends.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Re-exports core types; conditionally compiles cloud backends behind feature flags (`aws-nitro`, `azure-snp`, `gcp-confidential`)
- `backend.rs` - `TeeRuntimeBackend` trait (RPITIT, not dyn-compatible) with `deploy`, `get_attestation`, `cached_attestation`, `derive_public_key`, `status`, `stop`, `destroy` methods; `TeeDeployRequest`, `TeeDeploymentHandle`, `TeeDeploymentStatus`, `TeePublicKey` types
- `registry.rs` - `BackendRegistry` providing type-erased dispatch via internal `ErasedBackend` trait with boxed futures; `register()`, `has_provider()`, `providers()`, and all lifecycle methods delegating to the correct backend by `TeeProvider`; `from_env()` factory reading `TEE_BACKEND` env var
- `direct.rs` - `DirectBackend` implementing `TeeRuntimeBackend` for local TEE hosts (TDX/SEV-SNP); SHA-256 software measurement of running binary; HMAC-SHA256 key derivation; hardened container defaults (read-only rootfs, dropped capabilities)
- `aws_nitro.rs` - `NitroBackend` (feature-gated `aws-nitro`); provisions EC2 instances with Nitro Enclave support via AWS SDK; configured from `AWS_NITRO_*` env vars
- `azure_skr.rs` - `AzureSkrBackend` (feature-gated `azure-snp`); provisions Azure Confidential VMs (DCasv5/ECasv5) via ARM REST API with MAA token retrieval and Secure Key Release
- `gcp_confidential.rs` - `GcpConfidentialBackend` (feature-gated `gcp-confidential`); provisions Confidential Space VMs on GCE with AMD SEV-SNP or Intel TDX; OIDC attestation tokens from local teeserver

## Key APIs
- `TeeRuntimeBackend` trait - core SPI for backend providers; lifecycle: `deploy() -> get_attestation() -> ... -> stop() -> destroy()`
- `TeeDeployRequest::new(image)` - builder for deployment requests with env vars, provider preference, and extra ports
- `TeeDeploymentHandle` - carries deployment ID, provider, metadata, cached attestation, port mapping, and lifecycle policy (`CloudManaged`)
- `BackendRegistry::from_env()` - creates a registry from `TEE_BACKEND` comma-separated env var (values: `direct`, `direct-tdx`, `direct-sev-snp`, `aws-nitro`, `gcp-confidential`, `azure-snp`)
- `DirectBackend::tdx()` / `DirectBackend::sev_snp()` - convenience constructors for local TEE hosts

## Relationships
- Depends on `crate::attestation` for `AttestationReport` and related types
- Depends on `crate::config` for `TeeProvider`, `RuntimeLifecyclePolicy`
- `BackendRegistry` is used by the manager/runner to dispatch TEE lifecycle operations
- Cloud backends depend on respective SDKs (`aws-sdk-ec2`, `reqwest` for Azure/GCP APIs)
