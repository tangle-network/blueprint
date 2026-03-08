# deployment

## Purpose
Orchestrate Blueprint service deployment and lifecycle management across bare-metal (SSH), cloud VMs, and Kubernetes. Handles container deployment, state tracking with TTL expiry, error recovery with retries, cleanup on termination, and update strategies (blue-green, canary, rolling).

## Contents (one hop)
### Subdirectories
- [x] `ssh/` - SSH deployment client, fleet management, types, and bare-metal provisioning (4 files)
- [x] `tracker/` - Deployment lifecycle tracking, state persistence, TTL management, cleanup handlers for 14 deployment types

### Files
- `mod.rs` - Module facade; re-exports `SshDeploymentClient`, `DeploymentTracker`, strategies, manager integration.
  - **Key items**: public API surface, submodule declarations
- `error_recovery.rs` - Retry, rollback, failfast, and fallback recovery strategies with circuit breaker.
  - **Key items**: `ErrorRecovery`, `RecoveryStrategy`, `DeploymentCheckpoint`, `CircuitBreaker`
- `manager_integration.rs` - Bridge between Blueprint Manager service-level tracking and deployment tracker.
  - **Key items**: `RemoteDeploymentConfig`, `RemoteDeploymentRegistry`
- `secure_ssh.rs` - Secure SSH connection with hostname/port/key validation and strict host checking.
  - **Key items**: `SecureSshConnection`, `SecureSshClient`
- `secure_commands.rs` - Container command builder with shell escaping and security hardening (non-root, cap-drop, read-only).
  - **Key items**: `SecureContainerCommands`, `build_create_command()`
- `qos_tunnel.rs` - SSH port-forwarding tunnels for remote metrics access.
  - **Key items**: `QosTunnel`, `QosTunnelManager`
- `update_manager.rs` - Deployment update strategies with version history.
  - **Key items**: `UpdateManager`, `UpdateStrategy` (RollingUpdate, BlueGreen, Canary, Recreate)
- `kubernetes.rs` - Kubernetes deployment client (feature-gated); creates Deployments + Services with QoS port.
  - **Key items**: `KubernetesDeploymentClient`, `deploy_blueprint()`
- `secure_installer.rs` - Secure installation utilities for remote hosts.
  - **Key items**: installer validation, binary deployment

## Key APIs (no snippets)
- **Types**: `DeploymentTracker` (lifecycle orchestrator), `SshDeploymentClient` (SSH-based deployment), `ErrorRecovery` (strategy dispatcher), `UpdateManager` (versioned updates), `SecureSshConnection` (validated SSH), `SecureContainerCommands` (hardened container creation)
- **Functions**: `deploy_blueprint()`, `handle_termination()`, `execute_with_recovery()`, `deploy_version()`

## Relationships
- **Depends on**: `crate::core` (error, resources, remote), `crate::monitoring` (health, logs), `crate::infra::traits`, cloud SDKs, `kube`, `shell_escape`
- **Used by**: `crate::lib.rs` (public re-exports), `crate::secure_bridge`, `blueprint-manager`
- **Data/control flow**:
  - Provisioner creates VM -> SSH client receives connection -> deploys container
  - RemoteDeploymentRegistry creates DeploymentRecord -> DeploymentTracker persists to JSON
  - Background TTL task -> handle_termination() -> cleanup handler by type -> retry up to 3x

## Notes
- State persistence via JSON file for restart resilience
- Security hardening: strict SSH host checking, shell escaping, Docker non-root + cap-drop
- Kubernetes support is feature-gated; infers kubeconfig, exposes QoS port 9615
- QoS tunneling for remote metrics without public port exposure
- 21 files total across root + ssh/ + tracker/ + cleanup/
