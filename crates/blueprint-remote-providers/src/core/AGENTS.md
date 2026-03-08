# core

## Purpose
Foundational types and abstractions for the remote providers system: deployment targets, resource specifications, multi-cluster Kubernetes management, and error handling.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Re-exports: `DeploymentTarget`, `DeploymentConfig`, `ContainerRuntime`, `Error`, `Result`, `CloudProvider`, `RemoteClusterManager`, `ResourceSpec`.
- `deployment_target.rs` - `DeploymentTarget` enum (VirtualMachine, ManagedKubernetes, GenericKubernetes, Serverless). `ContainerRuntime` enum. `DeploymentConfig` combining cloud provider with target type.
- `error.rs` - `Error` enum with variants for configuration, cluster-not-found, network, Kube, AWS EC2/EKS, IO, provider-not-configured, serialization, HTTP, and generic errors. Feature-gated variants (e.g., AWS errors behind `aws` feature).
- `remote.rs` - `RemoteClusterManager` for multi-cluster Kubernetes management (add, switch, list clusters via kubeconfig/context). `KubernetesClusterConfig`. Re-exports `CloudProvider` from pricing engine. `CloudProviderExt` trait adding K8s service type and tunnel requirement queries.
- `resources.rs` - `ResourceSpec` with CPU, memory, storage, GPU count, spot preference, and QoS tier. Preset tiers: `minimal()`, `basic()`, `recommended()`, `performance()`. Conversions to K8s `ResourceRequirements` and Docker resource limits. Cost estimation via pricing units.
- `test_utils.rs` - `minimal_resource_spec()` and `mock_provisioned_instance()` test helpers.

## Key APIs
- `DeploymentTarget` / `DeploymentConfig` - deployment strategy selection
- `RemoteClusterManager::add_cluster()` / `switch_cluster()` / `list_clusters()` - multi-cluster management
- `ResourceSpec::basic()` / `recommended()` / `to_k8s_resources()` / `estimated_hourly_cost()` - resource specification and cost estimation
- `CloudProviderExt::service_type()` / `requires_tunnel()` - provider-specific K8s behavior

## Relationships
- Used by `infra/`, `monitoring/`, `shared/`, and `security/` sibling modules as the foundational type layer
- `CloudProvider` is re-exported from `blueprint-pricing-engine`
- `ResourceSpec` converts to both Kubernetes and Docker resource formats
