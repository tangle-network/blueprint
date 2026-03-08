# infra

## Purpose
Cloud infrastructure provisioning and lifecycle management. Provides a unified abstraction over AWS, GCP, Azure, DigitalOcean, and Vultr for instance provisioning, blueprint deployment, auto-deployment with cost-based provider selection, and instance type mapping.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Re-exports from all submodules: adapters, auto, mapper, provisioner, traits, types.
- `traits.rs` - `CloudProviderAdapter` async trait with methods: `provision_instance`, `terminate_instance`, `get_instance_status`, `deploy_blueprint_to_target`, `health_check`, `cleanup`. `BlueprintDeploymentResult` struct with instance, blueprint ID, port mappings, and QoS endpoint metadata.
- `types.rs` - `ProvisionedInstance` (id, IPs, status, provider, region, instance type). `InstanceStatus` enum (Pending, Running, Stopped, Terminated, Error). `RetryPolicy` with exponential backoff parameters.
- `adapters.rs` - `AdapterFactory` for creating provider-specific `CloudProviderAdapter` implementations. Supports AWS, GCP, Azure, DigitalOcean, and Vultr.
- `auto.rs` - `AutoDeploymentManager` for cost-optimized provider selection. `DeploymentPreferences` and `EnabledProvider` config. Loads config from file, detects credentials, selects cheapest provider matching resource requirements, provisions and deploys in one call.
- `mapper.rs` - `InstanceTypeMapper` mapping `ResourceSpec` to provider-specific instance types (e.g., `t3.medium`, `e2-standard-2`). Includes TEE/confidential compute mapping (AWS Nitro, Azure DCsv3, GCP C2D). `InstanceSelection`, `AutoScalingConfig`. Per-provider cost estimation.
- `provisioner.rs` - `CloudProvisioner` with retry logic, multi-provider support, credential detection via env vars, instance lifecycle management (provision, status, terminate), blueprint deployment, and machine type discovery integration.

## Key APIs
- `CloudProviderAdapter` trait - unified cloud provider interface
- `AdapterFactory::create(provider) -> Box<dyn CloudProviderAdapter>` - factory method
- `AutoDeploymentManager::auto_deploy(resource_spec, image, env_vars)` - cost-optimized deployment
- `InstanceTypeMapper::map(provider, spec) -> InstanceSelection` - resource-to-instance mapping
- `CloudProvisioner::provision_with_retry(provider, spec) -> ProvisionedInstance` - provisioning with retries

## Relationships
- Depends on `core/` for `ResourceSpec`, `CloudProvider`, `DeploymentTarget`, and `Error`
- Uses `monitoring/discovery` for machine type lookups
- Used by `shared/` for SSH and Kubernetes deployment flows
- `AutoDeploymentManager` integrates pricing data for provider selection
