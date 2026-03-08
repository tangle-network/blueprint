# ssh

## Purpose
SSH deployment module providing secure bare-metal deployment for Blueprint services across Docker, Podman, and Containerd container runtimes. Handles single-host and parallel fleet deployments over SSH with security hardening.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Public API facade re-exporting types from submodules; includes integration tests
- `types.rs` - Data structures: `SshConnection`, `ContainerRuntime`, `DeploymentConfig`, `ResourceLimits`, `RemoteDeployment`, `RestartPolicy`
- `client.rs` - Core SSH deployment client (~799 lines); `SshDeploymentClient` handles container lifecycle with security hardening via `secure_ssh` and `secure_commands`
- `fleet.rs` - `BareMetalFleet` for parallel deployment across multiple hosts using `join_all`

## Key APIs (no snippets)
- `SshDeploymentClient::new(connection, runtime, config)` - establish SSH session with a target host
- `SshDeploymentClient::deploy_blueprint(image, spec, env_vars)` - deploy a container image with resource limits
- `BareMetalFleet::deploy_all(connections, image, spec, env_vars)` - parallel deployment across a fleet of hosts
- `SshConnection` - host, user, key_path, port, password, jump_host
- `ContainerRuntime` - enum: Docker, Podman, Containerd
- `DeploymentConfig` - name, namespace, restart_policy, health_check
- `RemoteDeployment` - result with container_id, host, ports

## Relationships
- **Imports from**: `deployment/secure_ssh.rs` (SSH session hardening), `deployment/secure_commands.rs` (command sanitization), `core/resources.rs` (`ResourceSpec`), `monitoring/health.rs` (health checks)
- **Used by**: `deployment/mod.rs` (re-exports), `deployment/tracker/cleanup/ssh.rs` (cleanup of SSH-deployed containers), all cloud provider adapters for VM-based deployment
