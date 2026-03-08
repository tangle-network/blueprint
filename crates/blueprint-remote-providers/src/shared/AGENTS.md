# shared

## Purpose
Cross-provider shared implementations that eliminate code duplication across cloud provider adapters: unified SSH deployment, Kubernetes cluster authentication and deployment, and provider-agnostic firewall/security group management.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Re-exports: `BlueprintSecurityConfig`, `SecurityGroupManager`, `AzureNsgManager`, `DigitalOceanFirewallManager`, `VultrFirewallManager`, `SharedSshDeployment`, `SshDeploymentConfig`. Conditionally exports `SharedKubernetesDeployment` and `ManagedK8sConfig` behind `kubernetes` feature.
- `ssh_deployment.rs` - `SharedSshDeployment::deploy_to_instance()` deploys a blueprint container to any cloud instance via SSH. `SshDeploymentConfig` with provider presets: `aws()` (ec2-user), `gcp(project_id)` (ubuntu), `azure()` (azureuser), `digitalocean()` (root), `vultr()` (root). Handles SSH connection setup, container deployment, port mapping extraction, QoS port verification.
- `security.rs` - `BlueprintSecurityConfig` with toggleable SSH, QoS, and HTTPS outbound rules. `SecurityRule` with direction, protocol, ports, CIDRs, priority. CIDR sources configurable via `BLUEPRINT_ALLOWED_SSH_CIDRS` and `BLUEPRINT_ALLOWED_QOS_CIDRS` env vars (default: 0.0.0.0/0). `SecurityGroupManager` trait with `ensure_security_group()` and `delete_security_group()`. Implementations: `AzureNsgManager` (ARM API), `DigitalOceanFirewallManager` (DO API), `VultrFirewallManager` (Vultr API). Azure URL validation helper.
- `kubernetes_deployment.rs` - `SharedKubernetesDeployment` for managed K8s services. `deploy_to_managed_k8s()` handles cluster authentication (EKS via `aws eks`, GKE via `gcloud`, AKS via `az`, DOKS via `doctl`, VKE stub), health verification, and blueprint deployment. `deploy_to_generic_k8s()` for pre-configured clusters. `ManagedK8sConfig` with presets: `eks()`, `gke()`, `aks()`, `doks()`, `vke()`. Feature-gated behind `kubernetes`.

## Key APIs
- `SharedSshDeployment::deploy_to_instance(instance, image, spec, env, config)` - unified SSH deployment
- `SshDeploymentConfig::aws()` / `gcp()` / `azure()` / `digitalocean()` / `vultr()` - provider presets
- `SecurityGroupManager::ensure_security_group()` / `delete_security_group()` - firewall management trait
- `BlueprintSecurityConfig::standard_rules()` - generate provider-agnostic security rules
- `SharedKubernetesDeployment::deploy_to_managed_k8s()` / `deploy_to_generic_k8s()` - K8s deployment
- `ManagedK8sConfig::eks()` / `gke()` / `aks()` / `doks()` / `vke()` - managed K8s presets

## Relationships
- Uses `core/` for `ResourceSpec`, `CloudProvider`, and `Error`
- Uses `infra/traits` for `BlueprintDeploymentResult` and `infra/types` for `ProvisionedInstance`
- Uses `security/auth` for Azure access token acquisition in `AzureNsgManager`
- Uses `deployment/ssh` and `deployment/kubernetes` for actual deployment execution
- Called by provider-specific adapters in `infra/adapters` to avoid duplicating deployment logic
