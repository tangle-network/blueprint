# digitalocean

## Purpose
DigitalOcean cloud provider implementation with Droplet provisioning, DOKS (DigitalOcean Kubernetes Service) support, and cloud-init based instance setup. Uses the DigitalOcean REST API for VM and cluster management.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - `DigitalOceanProvisioner` with API client, Droplet/DOKSCluster types, size selection logic, and cloud-init user-data generation
- `adapter.rs` - `DigitalOceanAdapter` implementing `CloudProviderAdapter`; routes to Droplet VM or DOKS Kubernetes targets

## Key APIs (no snippets)
- `DigitalOceanAdapter::new()` - create adapter (requires `DIGITALOCEAN_TOKEN`)
- `DigitalOceanAdapter::provision_instance(type, region, require_tee)` - provision a Droplet
- `DigitalOceanProvisioner` - Droplet lifecycle via DigitalOcean API (create, destroy, status)
- `Droplet` - instance representation with id, name, ip, region, size
- `DOKSCluster` - managed Kubernetes cluster representation

## Relationships
- **Imports from**: `providers/common` (`ProvisioningConfig`, `ProvisionedInfrastructure`), `deployment/ssh` (Droplet VM deployment), `reqwest`
- **Used by**: `providers/mod.rs` re-exports; selected at runtime by provider resolution logic
- **Env vars**: `DIGITALOCEAN_TOKEN` (required), `DO_REGION`, `DO_SSH_KEY_IDS`
