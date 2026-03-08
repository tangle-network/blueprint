# azure

## Purpose
Azure cloud provider implementation using Azure Resource Manager REST APIs for provisioning VMs and deploying Blueprint containers. Supports VM, AKS, and generic Kubernetes deployment targets with Confidential VM (TEE) support.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Module declarations; re-exports `AzureAdapter` and `AzureProvisioner`
- `adapter.rs` - `AzureAdapter` implementing `CloudProviderAdapter`; routes to VM/AKS/generic-K8s targets; SSH-based VM deployment using `azureuser`; instance status via ARM instanceView API
- `provisioner.rs` - `AzureProvisioner` managing full VM lifecycle via ARM REST API; creates VNet, NIC, public IP, and VM; authenticates via managed identity or Azure CLI fallback; `select_vm_size()` maps `ResourceSpec` to Azure VM sizes (B/D/E/F/NC series)

## Key APIs (no snippets)
- `AzureAdapter::new()` - create adapter (requires `AZURE_SUBSCRIPTION_ID`)
- `AzureAdapter::provision_instance(type, region, require_tee)` - provision Azure VM
- `AzureAdapter::deploy_to_aks(cluster, namespace, image, spec, env_vars)` - deploy to AKS (feature-gated)
- `AzureProvisioner::provision_instance(spec, config)` - create VM with networking resources
- `AzureProvisioner::get_access_token()` - managed identity or CLI-based auth
- `AzureProvisioner::select_vm_size(spec)` - map resources to VM size (GPU/memory/CPU tiers)

## Relationships
- **Imports from**: `providers/common` (`ProvisioningConfig`, `ProvisionedInfrastructure`), `deployment/ssh` (VM deployment), `shared` (K8s deployment helpers), `reqwest`, `serde_json`
- **Used by**: `providers/mod.rs` re-exports; selected at runtime by provider resolution logic
- **Feature gates**: `kubernetes` (AKS and generic K8s deployment)
- **Env vars**: `AZURE_SUBSCRIPTION_ID` (required), `AZURE_RESOURCE_GROUP` (default: `blueprint-resources`), `AZURE_SSH_KEY_NAME`, `AZURE_SSH_KEY_PATH`, `AZURE_SSH_PUBLIC_KEY`, `BLUEPRINT_REMOTE_TEE_REQUIRED`
