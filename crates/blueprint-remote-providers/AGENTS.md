# blueprint-remote-providers

## Purpose
Crate `blueprint-remote-providers`: Multi-cloud infrastructure provisioning for the Blueprint Manager. Provides a unified `CloudProvider` abstraction over AWS, GCP, Azure, DigitalOcean, Vultr, Kubernetes, and Docker backends. Handles instance provisioning, SSH-based deployment, health monitoring, pricing/cost estimation, TLS secure bridging, and observability. Enables the Blueprint Manager to automatically deploy and manage blueprint services across cloud providers.

## Contents (one hop)
### Subdirectories
- [x] `src/` - Core architecture modules: `core/` (provider trait, error types, resource specs, remote operations), `config.rs` (per-cloud configuration structs), `providers/` (AWS, GCP, Azure, DigitalOcean, Vultr, Kubernetes implementations), `infra/` (provisioning and auto-deployment), `deployment/` (SSH client, deployment tracker with cleanup), `monitoring/` (health checks), `pricing/` (cost estimation with public pricing data), `security/` (TLS, encryption), `shared/` (common utilities), `auth_integration.rs`, `observability.rs`, `secure_bridge.rs`.
- [x] `tests/` - Extensive test suite: integration tests, provider-specific tests, deployment decision tests, property tests, networking tests, security tests, Kubernetes E2E, real blueprint deployment, SDK provisioning, update/rollback.

### Files
- `Cargo.toml` - Crate manifest (`blueprint-remote-providers`). Key deps: `blueprint-core`, `blueprint-keystore`, `blueprint-pricing-engine`, `bollard` (Docker), `kube`/`k8s-openapi` (Kubernetes), `aws-sdk-ec2`/`aws-sdk-eks`/`aws-sdk-autoscaling`, `reqwest`, `tokio-rustls`, `chacha20poly1305`. Features: `aws` (default), `aws-eks`, `gcp`, `azure`, `digitalocean`, `vultr`, `kubernetes`, `docker`, `testing`, `tee-attestation` (default; JWT path), `tee-attestation-nitro` (AWS Nitro COSE verifier), `tee-attestation-seam` (blueprint-tee deep-quote interop).
- `README.md` - Crate documentation.

## Key APIs (no snippets)
- `CloudProvider` trait -- core abstraction for provisioning, managing, and destroying cloud instances.
- `CloudProvisioner` -- orchestrates instance lifecycle across providers.
- `ProvisionedInstance` / `InstanceStatus` -- instance metadata and state tracking.
- `ResourceSpec` -- CPU, memory, and storage requirements for provisioning.
- `CloudConfig` enum with `AwsConfig`, `GcpConfig`, `AzureConfig`, `DigitalOceanConfig`, `VultrConfig` variants.
- `DeploymentTracker` -- tracks deployment state and handles cleanup.
- `SshDeploymentClient` -- deploys blueprint binaries to remote instances via SSH.
- `HealthMonitor` / `HealthCheckResult` / `HealthStatus` -- instance health monitoring.
- `PricingService` / `CostReport` -- cost estimation across providers.
- `AwsProvisioner` / `AwsInstanceMapper` -- AWS-specific provisioning (feature-gated).
- `create_provider_client()` / `create_metadata_client()` -- HTTP client factories.

## TEE attestation (`require_tee`) — operator notes

The `attestation` module gates a `require_tee` deployment: a VM is only reported
TEE-trusted after its provider attestation is fetched **and** cryptographically
verified. It fails closed — any fetch/verify failure errors the provision instead
of blessing an unattested VM (`gate_provisioned` is the chokepoint).

**End-to-end is NOT turnkey from this crate alone.** The verify halves are real and
tested, but the *fetch* of live evidence depends on a component that must run inside
the workload, which this crate does not deploy:

- **GCP Confidential Space / Azure MAA (JWT):** the provisioner fetches the OIDC /
  MAA token over HTTPS from a path the workload re-exposes
  (`/.well-known/attestation-token` for GCP, `/.well-known/maa-token` for Azure).
  **The blueprint workload image must ship an attestation-token-forwarding agent**
  that reads the in-VM launcher/MAA token and serves it on that path, challenged
  with our nonce. Until that agent is present and reachable, the gate fails closed
  (connection-refused → `AttestationError::Fetch`). There is no token-serving sidecar
  in this crate.
- **AWS Nitro (COSE):** the NSM attestation document is producible **only inside the
  enclave** and AWS exposes no API to pull it from the parent host, so
  `AwsNitroGate::fetch` always returns `Unsatisfiable`. A Nitro `require_tee` from the
  out-of-enclave provisioner can only fail closed. The COSE verifier
  (`verify_document`) is real and tested; the supported happy path is an in-enclave
  agent that POSTs its COSE document to `verify_document` over a channel the enclave
  application exposes. `tee-attestation-nitro` must be enabled or the AWS gate refuses
  outright.

**Workload binding.** Without `tee_expected_audience` and/or `tee_expected_image_digest`
pinned, the gate proves only "a genuine, fresh, non-debug confidential VM answered our
nonce" — not "running our workload for our relying party". Such a result is stamped
`tee_attested=hardware-only` (with `tee_workload_bound=false`), distinct from the
workload-bound `tee_attested=true`; downstream consumers must treat only `true` as
workload-bound. `gate_provisioned` also emits a loud warn in this case.

**Policy floor.** Freshness, signature, non-debug, and nonce are always on and cannot
be downgraded via `custom_config`: `tee_max_age_secs` is clamped to
`MAX_AGE_CEILING_SECS` (3600s) and `tee_allow_debug=true` is refused outside a
`testing` build.

## Relationships
- Depends on `blueprint-core` for tracing and core types.
- Depends on `blueprint-keystore` for credential management.
- Depends on `blueprint-pricing-engine` for cost calculation.
- Used by `blueprint-manager` for automated cloud deployment of blueprint services.
- Legacy compatibility modules (`auto_deployment`, `infrastructure`, `remote`, `resources`) re-export for manager integration.
