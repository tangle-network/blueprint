# vultr

## Purpose
Vultr cloud provider implementation with VM provisioning via the Vultr REST API and Blueprint container deployment. Supports VM, VKE (Vultr Kubernetes Engine), and generic Kubernetes deployment targets.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Re-exports `VultrAdapter` and `VultrProvisioner`
- `provisioner.rs` - `VultrProvisioner` as Vultr API client; plan selection based on `ResourceSpec`; user-data generation for instance initialization; hardcoded OS ID 1743 (Ubuntu 22.04)
- `adapter.rs` - `VultrAdapter` implementing `CloudProviderAdapter`; routes deployments to VM, VKE, or generic K8s targets; SSH-based VM deployment

## Key APIs (no snippets)
- `VultrAdapter::new()` - create adapter (requires `VULTR_API_KEY`)
- `VultrAdapter::provision_instance(type, region, require_tee)` - provision a Vultr VM
- `VultrProvisioner` - VM lifecycle via Vultr API (create, destroy, status, list plans)
- `VultrInstance` - provisioned instance with id, ip, plan, region

## Relationships
- **Imports from**: `providers/common` (`ProvisioningConfig`, `ProvisionedInfrastructure`), `deployment/ssh` (VM deployment), `reqwest`
- **Used by**: `providers/mod.rs` re-exports; selected at runtime by provider resolution logic
- **Env vars**: `VULTR_API_KEY` (required), `VULTR_SSH_KEY_ID`, `VULTR_SSH_KEY_PATH`
