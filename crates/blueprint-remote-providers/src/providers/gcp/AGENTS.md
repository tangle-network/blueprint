# gcp

## Purpose
Google Cloud Platform provider implementation with Compute Engine instance provisioning, GKE support, Confidential VM (TEE) support, and security-hardened firewall rule management. Uses GCP REST APIs via reqwest.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - `GcpProvisioner` for GCE instance lifecycle; instance type mapping (`map_instance`) across e2/n2/n1 families with cost estimation; startup script generation; auth via `GCP_ACCESS_TOKEN` env var or GCE metadata service; feature-gated behind `gcp`
- `adapter.rs` - `GcpAdapter` implementing `CloudProviderAdapter`; manages zone-to-instance tracking via `RwLock<HashMap>`; firewall rule creation/update with fail-closed CIDR validation; routes to VM/GKE/generic-K8s targets; uses `SharedSshDeployment` for VM deploys

## Key APIs (no snippets)
- `GcpProvisioner::new(project_id)` - create provisioner (feature-gated)
- `GcpProvisioner::provision_instance(spec, config)` - create GCE instance with startup script and optional Confidential VM
- `GcpProvisioner::map_instance(spec)` - map `ResourceSpec` to GCP instance type with cost estimate
- `GcpAdapter::new()` - create adapter (requires `GCP_PROJECT_ID`)
- `GcpAdapter::ensure_firewall_rules()` - create/update SSH and QoS firewall rules with CIDR validation
- `GcpAdapter::build_firewall_rules()` - construct rules from `BlueprintSecurityConfig`; rejects open ingress without explicit env var override

## Relationships
- **Imports from**: `providers/common` (`ProvisioningConfig`, `ProvisionedInfrastructure`, `InstanceSelection`), `deployment/ssh` (VM deployment via shared helpers), `security/auth` (GCP token retrieval), `shared/security` (`BlueprintSecurityConfig`)
- **Used by**: `providers/mod.rs` re-exports; selected at runtime by provider resolution logic
- **Feature gates**: `gcp` (provisioner core), `kubernetes` (GKE and generic K8s deployment)
- **Env vars**: `GCP_PROJECT_ID` (required), `GCP_ACCESS_TOKEN`, `GCP_SSH_KEY_PATH`, `GCP_DEFAULT_REGION` (default: `us-central1`), `BLUEPRINT_ALLOWED_SSH_CIDRS`, `BLUEPRINT_ALLOWED_QOS_CIDRS`, `BLUEPRINT_REMOTE_TEE_REQUIRED`
