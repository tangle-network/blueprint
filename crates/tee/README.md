# blueprint-tee

First-class Trusted Execution Environment (TEE) support for the Blueprint SDK.

## Overview

`blueprint-tee` provides runtime TEE capabilities for blueprint services running in
confidential compute environments. It supports multiple TEE providers and deployment
modes, with attestation verification, sealed-secret key exchange, and Tower middleware
integration.

## Quick Start

```rust
use blueprint_runner::BlueprintRunner;
use blueprint_tee::{TeeConfig, TeeMode, TeeRequirement};

let tee = TeeConfig::builder()
    .requirement(TeeRequirement::Required)
    .mode(TeeMode::Direct)
    .build()?;

BlueprintRunner::builder(config, env)
    .tee(tee)
    .router(router)
    .run()
    .await?;
```

## Feature Flags

| Feature            | Description                                      |
|--------------------|--------------------------------------------------|
| `std` (default)    | Standard library support                         |
| `aws-nitro`        | AWS Nitro Enclave attestation verification       |
| `azure-snp`        | Azure SEV-SNP / SKR attestation verification     |
| `gcp-confidential` | GCP Confidential Space attestation verification  |
| `tdx`              | Intel TDX attestation verification               |
| `sev-snp`          | AMD SEV-SNP attestation verification             |
| `all-providers`    | Enables all provider backends                    |

## Architecture

```
blueprint-tee
|
|-- config          TeeConfig, TeeMode, policies, builder
|
|-- attestation
|   |-- report      AttestationReport, Measurement, PublicKeyBinding
|   |-- claims      Typed attestation claims
|   |-- verifier    AttestationVerifier trait, VerifiedAttestation wrapper
|   +-- providers   Feature-gated verifiers per TEE platform
|       |-- native         Unified TDX/SEV-SNP via ioctl
|       |-- aws_nitro      COSE document verification
|       |-- azure_snp      MAA token verification
|       +-- gcp_confidential  Confidential Space token verification
|
|-- exchange
|   |-- protocol    KeyExchangeSession (ephemeral, zeroized on drop)
|   +-- service     TeeAuthService (session management, TTL, cleanup)
|
|-- middleware
|   |-- tee_layer   Tower Layer injecting attestation into JobResult metadata
|   +-- tee_context TeeContext extractor for job handlers
|
+-- runtime
    |-- backend     TeeRuntimeBackend trait (deploy/attest/stop/destroy)
    |-- direct      Local TEE host with device passthrough
    +-- registry    Type-erased multi-backend dispatch
```

### Deployment Modes

- **Direct** -- Runner executes inside a TEE with device passthrough and hardened
  container defaults (read-only rootfs, dropped capabilities, no new privileges).
- **Remote** -- Runner provisions workloads in cloud TEE instances (AWS Nitro,
  Azure CVM, GCP Confidential Space).
- **Hybrid** -- Selected jobs run in TEE runtimes while others run normally.
  Routing is contract-driven by default (`teeRequired` flag on-chain).

### Security Model

- Sealed secrets are the only secret injection path for TEE deployments (enforced
  at the config level; container recreation is forbidden).
- Ephemeral key exchange sessions are one-time use with TTL enforcement and
  private key material is zeroed on drop via `write_volatile`.
- Attestation verification supports dual-path checking: local evidence validation
  plus optional on-chain hash comparison.

## License

Licensed under the same terms as the parent workspace.
