# cleanup

## Purpose
Platform-specific cleanup handlers for different deployment types. Each handler implements the `CleanupHandler` trait to terminate, deprovision, and release resources (VMs, containers, Kubernetes clusters, SSH sessions) across cloud providers and local runtimes.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Module coordinator re-exporting all cleanup handler types for use by the tracker core.
  - **Key items**: `AwsCleanup`, `AzureCleanup`, `DigitalOceanCleanup`, `GcpCleanup`, `VultrCleanup`, `AksCleanup`, `EksCleanup`, `GkeCleanup`, `LocalDockerCleanup`, `LocalHypervisorCleanup`, `LocalKubernetesCleanup`, `SshCleanup`
- `cloud_vms.rs` - Cloud provider VM termination handlers for AWS EC2, GCP, Azure, DigitalOcean, and Vultr.
  - **Key items**: `AwsCleanup`, `GcpCleanup`, `AzureCleanup`, `DigitalOceanCleanup`, `VultrCleanup`
  - **Interactions**: AWS uses `aws_sdk_ec2` with 30s wait before EBS cleanup; GCP delegates to `GcpProvisioner`; others use `CloudProvisioner`
- `kubernetes.rs` - Managed Kubernetes cluster termination handlers (EKS, GKE, AKS).
  - **Key items**: `EksCleanup`, `GkeCleanup`, `AksCleanup`
  - **Interactions**: EKS deletes nodegroups then cluster with 60s wait; GKE/AKS are unimplemented stubs
- `local.rs` - Local runtime cleanup for Docker containers, Kubernetes pods, and Cloud Hypervisor VMs.
  - **Key items**: `LocalDockerCleanup`, `LocalKubernetesCleanup`, `LocalHypervisorCleanup`, `safe_terminate_process()`
  - **Interactions**: Docker uses `docker rm -f`; K8s uses `kubectl delete`; Hypervisor sends SIGTERM then SIGKILL via `libc::kill`
- `ssh.rs` - Remote SSH deployment cleanup via `SshDeploymentClient`.
  - **Key items**: `SshCleanup`
  - **Interactions**: Reconstructs `SshConnection` from deployment metadata and calls `cleanup_deployment()`

## Key APIs (no snippets)
- **Types / Traits**: `CleanupHandler` (async trait with `cleanup` method), `DeploymentRecord`, `DeploymentType`
- **Functions**: `safe_terminate_process()` - async wrapper around `libc::kill` for Cloud Hypervisor process termination

## Relationships
- **Depends on**: `super::super::types` (CleanupHandler, DeploymentRecord), `crate::core::error`, platform SDKs (`aws_sdk_ec2`, `aws_sdk_eks`), `crate::deployment::ssh`
- **Used by**: tracker core (`core.rs`) registers default handlers and invokes `handler.cleanup()` with 3-attempt retry during termination
- **Data/control flow**:
  - Tracker registers handlers via `register_default_handlers()` mapping each `DeploymentType` to a handler
  - On termination, retrieves handler by type and calls `cleanup()` with up to 3 retries (5s backoff)
  - Success removes deployment from tracking; failure preserves record in Failed state

## Notes
