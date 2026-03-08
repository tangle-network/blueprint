# providers

## Purpose
Central hub for cloud provider adapters implementing the `CloudProviderAdapter` trait. Enables Blueprint Manager to provision VMs and deploy services across AWS, Azure, GCP, DigitalOcean, Vultr, and Kubernetes.

## Contents (one hop)
### Subdirectories
- [x] `aws/` - AWS adapter using `aws_sdk_ec2`/`aws_sdk_eks`: provisioner, instance type mapper, EC2/EKS lifecycle (846 lines, feature-gated)
- [x] `azure/` - Azure adapter using ARM REST API: provisioner, VM sizing (B/D/E/F/NC series) (1,071 lines)
- [x] `common/` - Foundation: `CloudProvisioner` trait, `ProvisioningConfig`, `ProvisionedInfrastructure`, `InstanceSelection` (124 lines)
- [x] `digitalocean/` - DigitalOcean adapter: Droplet lifecycle, cloud-init, `SecureHttpClient` (904 lines)
- [x] `gcp/` - GCP adapter using REST API: zone tracking, firewall rules with CIDR validation (1,070 lines)
- [x] `kubernetes/` - Stub/placeholder (7 lines); actual K8s logic in `shared/kubernetes_deployment.rs`
- [x] `vultr/` - Vultr adapter: plan selection, Ubuntu 22.04 LTS provisioning (745 lines)

### Files
- `mod.rs` - Module declarations and re-exports from all provider submodules. AWS is feature-gated; others always enabled.
  - **Key items**: `pub mod common`, `#[cfg(feature = "aws")] pub mod aws`, `pub mod azure`, `pub mod digitalocean`, `pub mod gcp`, `pub mod vultr`

## Key APIs (no snippets)
- **Traits**: `CloudProvisioner` (`provision_instance()`, `terminate_instance()`), `CloudProviderAdapter` (`provision_instance()`, `terminate_instance()`, `get_instance_status()`, `deploy_blueprint_with_target()`, `health_check_blueprint()`, `cleanup_blueprint()`)
- **Types**: `ProvisionedInfrastructure` (IPs, region, metadata), `ProvisioningConfig` (name, region, ssh_key, custom_config HashMap), `InstanceSelection` (type, spot, cost)
- **Providers**: `AwsAdapter`/`AwsProvisioner`, `AzureAdapter`/`AzureProvisioner`, `GcpAdapter`/`GcpProvisioner`, `DigitalOceanAdapter`/`DigitalOceanProvisioner`, `VultrAdapter`/`VultrProvisioner`

## Relationships
- **Depends on**: `crate::core` (error, resources, remote), `crate::deployment::ssh` (SSH deployment), `crate::shared::kubernetes_deployment`, cloud SDKs (aws_sdk_ec2, reqwest for REST APIs)
- **Used by**: `blueprint-manager` (automated cloud deployment), `blueprint-runner`, `cargo-tangle` CLI
- **Data/control flow**:
  - Two-tier: low-level Provisioner (API calls) + high-level Adapter (orchestration + deployment)
  - All adapters implement `CloudProviderAdapter` with standard interface
  - Deployment routing: VM (Docker), ManagedK8s (EKS/AKS/GKE/DOKS/VKE), GenericK8s

## Notes
- ~4,800 lines of provider code total across all adapters
- TEE support: AWS (Enclave), Azure/GCP (Confidential VM), DO/Vultr (via env var)
- Feature gates: `aws` (default), `aws-eks`, `gcp`, `azure`, `digitalocean`, `vultr`, `kubernetes`
- Environment variables required: `AZURE_SUBSCRIPTION_ID`, `GCP_PROJECT_ID`, `DIGITALOCEAN_TOKEN`, `VULTR_API_KEY`
