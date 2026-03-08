# blueprint-remote-providers

## Purpose
Crate `blueprint-remote-providers`: Multi-cloud infrastructure provisioning for the Blueprint Manager. Provides a unified `CloudProvider` abstraction over AWS, GCP, Azure, DigitalOcean, Vultr, Kubernetes, and Docker backends. Handles instance provisioning, SSH-based deployment, health monitoring, pricing/cost estimation, TLS secure bridging, and observability. Enables the Blueprint Manager to automatically deploy and manage blueprint services across cloud providers.

## Contents (one hop)
### Subdirectories
- [x] `src/` - Core architecture modules: `core/` (provider trait, error types, resource specs, remote operations), `config.rs` (per-cloud configuration structs), `providers/` (AWS, GCP, Azure, DigitalOcean, Vultr, Kubernetes implementations), `infra/` (provisioning and auto-deployment), `deployment/` (SSH client, deployment tracker with cleanup), `monitoring/` (health checks), `pricing/` (cost estimation with public pricing data), `security/` (TLS, encryption), `shared/` (common utilities), `auth_integration.rs`, `observability.rs`, `secure_bridge.rs`.
- [x] `tests/` - Extensive test suite: integration tests, provider-specific tests, deployment decision tests, property tests, networking tests, security tests, Kubernetes E2E, real blueprint deployment, SDK provisioning, update/rollback.

### Files
- `Cargo.toml` - Crate manifest (`blueprint-remote-providers`). Key deps: `blueprint-core`, `blueprint-keystore`, `blueprint-pricing-engine`, `bollard` (Docker), `kube`/`k8s-openapi` (Kubernetes), `aws-sdk-ec2`/`aws-sdk-eks`/`aws-sdk-autoscaling`, `reqwest`, `tokio-rustls`, `chacha20poly1305`. Features: `aws` (default), `aws-eks`, `gcp`, `azure`, `digitalocean`, `vultr`, `kubernetes`, `docker`, `testing`.
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

## Relationships
- Depends on `blueprint-core` for tracing and core types.
- Depends on `blueprint-keystore` for credential management.
- Depends on `blueprint-pricing-engine` for cost calculation.
- Used by `blueprint-manager` for automated cloud deployment of blueprint services.
- Legacy compatibility modules (`auto_deployment`, `infrastructure`, `remote`, `resources`) re-export for manager integration.
