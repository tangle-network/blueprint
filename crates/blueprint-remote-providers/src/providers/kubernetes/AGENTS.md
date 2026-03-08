# kubernetes

## Purpose
Kubernetes deployment provider for deploying Blueprints to existing Kubernetes clusters. Unlike cloud providers, this assumes a cluster already exists and does not provision infrastructure.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Module declaration for `adapter` submodule; re-exports `KubernetesAdapter`. Note: the `adapter` submodule is currently a stub/placeholder (file not yet implemented).

## Key APIs (no snippets)
- `KubernetesAdapter` - declared re-export for generic Kubernetes cluster deployment (implementation pending)

## Relationships
- **Imports from**: Expected to use `shared::SharedKubernetesDeployment` for K8s deployment logic when implemented
- **Used by**: `providers/mod.rs` re-exports; intended for deployments targeting pre-existing clusters without cloud provider coupling
- **Note**: This module is a skeleton. The actual Kubernetes deployment logic lives in `shared/` and is used by other provider adapters (AWS/Azure/GCP/etc.) via their `deploy_to_generic_k8s` methods.
