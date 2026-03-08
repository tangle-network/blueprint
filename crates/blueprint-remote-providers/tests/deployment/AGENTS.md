# deployment

## Purpose
Tests for deployment infrastructure across SSH, Kubernetes, and Docker targets. Covers full provisioning lifecycles (provision, deploy, monitor, cleanup), Kubernetes cluster simulation for EKS/GKE/AKS, QoS-enabled Docker and Kubernetes deployments, and SSH-based container deployment with resource limits.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Module declarations for all deployment test submodules.
- `deployment_integration.rs` - Full deployment lifecycle tests with mocked AWS/GCP/DigitalOcean APIs (using `mockito`), TTL-based auto-cleanup, cost-optimized deployment, concurrent multi-provider deployment, and health monitoring with auto-restart.
- `kubernetes_deployment.rs` - Real Kubernetes tests using Kind clusters: deploy, multi-namespace, service exposure (LoadBalancer/ClusterIP/NodePort), resource limits, rolling updates, and namespace isolation.
- `kubernetes_simulation.rs` - Simulated Kubernetes deployment configs for local/EKS/GKE/AKS clusters, K8s-to-VM fallback, multi-cluster management, and resource mapping validation without requiring actual clusters.
- `qos_docker_tests.rs` - Docker QoS integration tests building and running the incredible-squaring blueprint with QoS gRPC endpoints, port exposure logic, resource limits, and multi-container scenarios.
- `qos_kubernetes_tests.rs` - Kubernetes QoS tests deploying blueprints with QoS metrics/RPC ports via Kind/k3d, verifying service discovery, metrics collection from pods, and autoscaling behavior.
- `ssh_deployment.rs` - SSH deployment tests using Docker containers as SSH targets (linuxserver/openssh-server), testing connection, container deployment, resource limit enforcement, and container lifecycle.

## Key APIs
- `CloudProvisioner` / `ProvisionedInstance` / `InstanceStatus` - cloud instance provisioning
- `SshDeploymentClient` / `SshConnection` / `DeploymentConfig` - SSH-based deployment
- `DeploymentTracker` / `DeploymentRecord` - deployment state tracking with TTL
- `RemoteDeploymentConfig` / `KubernetesCluster` - Kubernetes deployment configuration
- `PricingFetcher` - cloud pricing queries for cost optimization

## Relationships
- Depends on `blueprint_remote_providers` core types (cloud provisioner, SSH deployment, pricing, monitoring)
- Uses `mockito` for HTTP API mocking, `kube`/`k8s-openapi` for Kubernetes interactions
- Uses `blueprint_qos` proto definitions for gRPC endpoint testing
- Docker and Kind/k3d are optional runtime dependencies; tests skip gracefully when unavailable
