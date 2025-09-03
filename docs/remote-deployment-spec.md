# Remote Cloud Deployment Specification for Blueprint Manager

## Executive Summary

This specification defines the architecture and implementation for extending the Blueprint Manager to support remote cloud deployments while maintaining 100% backwards compatibility with existing local deployment mechanisms. The solution provides a unified abstraction layer for deploying Blueprint instances to any cloud provider, edge PaaS platform, or bare-metal infrastructure.

## Design Principles

1. **Zero Breaking Changes**: All existing local deployments must continue to work unchanged
2. **Provider Agnostic**: Support any cloud through pluggable provider system
3. **Minimal Surface Area**: Leverage existing components maximally
4. **Test-Driven Development**: Every component must have comprehensive tests
5. **Future-Proof Architecture**: Extensible for new providers without core changes
6. **Security First**: All remote communication encrypted, credentials properly managed

## Architecture Overview

### Core Abstraction: Remote Provider Trait

The system introduces a new crate `blueprint-remote-providers` that defines a universal provider interface following the existing patterns from `blueprint-keystore` remote backends:

```rust
#[auto_impl::auto_impl(&, Arc)]
pub trait RemoteInfrastructureProvider: Send + Sync + 'static {
    type InstanceId: Send + Sync + Clone + 'static;
    type NetworkConfig: Send + Sync + 'static;
    type Error: core::error::Error + Send + Sync + 'static;
    
    /// Deploy a blueprint instance to the remote infrastructure
    async fn deploy_instance(
        &self,
        spec: DeploymentSpec,
    ) -> Result<RemoteInstance<Self::InstanceId>, Self::Error>;
    
    /// Establish secure tunnel for networking
    async fn establish_tunnel(
        &self,
        hub: &TunnelHub,
    ) -> Result<TunnelHandle, Self::Error>;
    
    /// Get instance status
    async fn get_instance_status(
        &self,
        id: &Self::InstanceId,
    ) -> Result<InstanceStatus, Self::Error>;
    
    /// Terminate instance
    async fn terminate_instance(
        &self,
        id: &Self::InstanceId,
    ) -> Result<(), Self::Error>;
    
    /// Resource management
    async fn get_available_resources(&self) -> Result<Resources, Self::Error>;
    async fn estimate_cost(&self, spec: &DeploymentSpec) -> Result<Cost, Self::Error>;
}
```

### Provider Implementations

#### 1. Kubernetes Remote Provider (Primary)
- Leverages existing `ContainerRuntime` from `blueprint-manager`
- Supports all managed Kubernetes services (EKS, GKE, AKS, etc.)
- Uses kubeconfig contexts for multi-cluster management
- 90% code reuse from existing container runtime

#### 2. Docker Remote Provider (Secondary)
- For edge/PaaS platforms (Fly.io, Railway, Render)
- Docker API over TLS
- Minimal overhead for simple deployments

#### 3. SSH Provider (Tertiary)
- Bare-metal and VM deployments
- SSH-based orchestration
- Optional, only when needed

### Networking Architecture

#### Hub-and-Spoke Model
```
┌─────────────────────┐
│   Central Hub       │
│  (Auth Proxy)       │
│  (Service Registry) │
└──────┬──────────────┘
       │
    WireGuard
    Tunnels
       │
┌──────┴───────┐  ┌──────────────┐  ┌──────────────┐
│  AWS EKS     │  │  GCP GKE     │  │  Home Lab    │
│  Region 1    │  │  Region 2    │  │  Bare Metal  │
└──────────────┘  └──────────────┘  └──────────────┘
```

Benefits:
- Single tunnel per cloud/region (scalable)
- Centralized auth proxy (existing security model)
- Service discovery through existing bridge

### Integration with Existing Components

#### 1. Bridge Extension
The existing `blueprint-manager-bridge` extends naturally:
```rust
pub enum TransportType {
    UnixSocket(PathBuf),    // Existing local
    Vsock(u32, u32),       // Existing VM
    Tunnel(TunnelHandle),  // New: remote via WireGuard
}
```

#### 2. Service Runtime Enhancement
Minimal changes to existing `RuntimeService`:
```rust
impl RuntimeService {
    pub async fn deploy_remote(
        &self,
        provider: Arc<dyn RemoteInfrastructureProvider>,
        spec: DeploymentSpec,
    ) -> Result<ServiceId> {
        // Reuse existing service management
    }
}
```

#### 3. Configuration Integration
Feature-gated configuration following existing patterns:
```rust
#[cfg(feature = "remote-providers")]
#[derive(Debug, Args)]
pub struct RemoteProviderOptions {
    /// Cloud provider configuration
    #[arg(long)]
    pub provider_config: Option<PathBuf>,
    
    /// Remote deployment region
    #[arg(long)]
    pub region: Option<String>,
}
```

## Implementation Plan

### Phase 1: Core Infrastructure (Week 1)
- [ ] Create `blueprint-remote-providers` crate
- [ ] Define `RemoteInfrastructureProvider` trait
- [ ] Implement provider registry pattern
- [ ] Add comprehensive trait tests

### Phase 2: Kubernetes Provider (Week 2)
- [ ] Implement `KubernetesRemoteProvider`
- [ ] Integrate with existing `ContainerRuntime`
- [ ] Add multi-context support
- [ ] Test with Kind/Minikube

### Phase 3: Networking (Week 3)
- [ ] Implement WireGuard tunnel management
- [ ] Extend Bridge for tunnel transport
- [ ] Add service discovery through tunnels
- [ ] Test cross-region communication

### Phase 4: Docker & SSH Providers (Week 4)
- [ ] Implement `DockerRemoteProvider`
- [ ] Implement `SshProvider` (optional)
- [ ] Add provider-specific tests
- [ ] End-to-end integration tests

### Phase 5: CLI & Documentation (Week 5)
- [ ] Extend `cargo-tangle` CLI
- [ ] Add remote deployment commands
- [ ] Comprehensive documentation
- [ ] Example configurations

## Testing Strategy

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;
    
    mock! {
        Provider {}
        impl RemoteInfrastructureProvider for Provider {
            type InstanceId = String;
            type NetworkConfig = ();
            type Error = anyhow::Error;
            
            async fn deploy_instance(&self, spec: DeploymentSpec) 
                -> Result<RemoteInstance<String>, anyhow::Error>;
        }
    }
    
    #[tokio::test]
    async fn test_deployment_lifecycle() {
        let mut mock = MockProvider::new();
        mock.expect_deploy_instance()
            .returning(|_| Ok(RemoteInstance::new("test-123")));
        
        let result = mock.deploy_instance(DeploymentSpec::default()).await;
        assert!(result.is_ok());
    }
}
```

### Integration Tests
- Local Kubernetes (Kind/Minikube)
- Docker-in-Docker for Docker provider
- Mock SSH server for SSH provider

### End-to-End Tests
- Deploy to actual cloud providers (gated by feature flags)
- Cost estimation validation
- Network tunnel establishment
- Cross-region communication

## Security Considerations

1. **Credential Management**
   - Leverage existing `blueprint-keystore` for cloud credentials
   - Support environment variables for CI/CD
   - Integration with cloud provider IAM

2. **Network Security**
   - All traffic through WireGuard encryption
   - No direct internet exposure of services
   - Auth proxy validates all requests

3. **Resource Isolation**
   - Provider-specific namespace/project isolation
   - Resource quotas enforced at provider level
   - Cost alerts and limits

## Configuration Examples

### Kubernetes Provider (AWS EKS)
```toml
[remote.aws]
type = "kubernetes"
kubeconfig = "~/.kube/eks-config"
context = "eks-us-west-2"
namespace = "blueprint-production"

[remote.aws.resources]
cpu_limit = "4"
memory_limit = "8Gi"
max_instances = 10

[remote.aws.tunnel]
endpoint = "hub.mycompany.com:51820"
private_key = "${WIREGUARD_KEY}"
```

### Docker Provider (Fly.io)
```toml
[remote.fly]
type = "docker"
endpoint = "https://api.fly.io"
auth_token = "${FLY_AUTH_TOKEN}"
organization = "my-org"

[remote.fly.regions]
primary = "sea"
replicas = ["lax", "ewr"]
```

### SSH Provider (Bare Metal)
```toml
[remote.homelab]
type = "ssh"
hosts = ["192.168.1.10", "192.168.1.11"]
user = "blueprint"
key = "~/.ssh/blueprint_deploy"

[remote.homelab.runtime]
type = "docker"  # or "native"
docker_socket = "unix:///var/run/docker.sock"
```

## CLI Usage

### Deploy to Remote
```bash
# Deploy to specific cloud
cargo tangle blueprint deploy --remote aws --package my-blueprint

# Deploy with custom config
cargo tangle blueprint deploy --remote-config ./production.toml --package my-blueprint

# List remote deployments
cargo tangle blueprint list --remote

# Get deployment status
cargo tangle blueprint status --remote --instance bp-abc123

# Terminate deployment
cargo tangle blueprint terminate --remote --instance bp-abc123
```

### Cost Management
```bash
# Estimate costs before deployment
cargo tangle blueprint estimate-cost --remote aws --package my-blueprint

# Get current month costs
cargo tangle blueprint costs --remote aws

# Set cost alerts
cargo tangle blueprint set-alert --remote aws --monthly-limit 100
```

## Backwards Compatibility

### Guaranteed Unchanged Behaviors
1. All existing local deployments work without modification
2. Default behavior remains local deployment
3. No changes to existing configuration files
4. All existing CLI commands work unchanged
5. No impact on existing Bridge/Proxy functionality

### Feature Detection
```rust
#[cfg(feature = "remote-providers")]
fn has_remote_support() -> bool { true }

#[cfg(not(feature = "remote-providers"))]
fn has_remote_support() -> bool { false }
```

## Future Extensions

### Planned Enhancements
1. **Multi-region deployments** - Automatic failover and geo-distribution
2. **Auto-scaling** - Provider-specific scaling policies
3. **Backup/Restore** - Cross-provider state migration
4. **Observability** - Unified metrics across all providers
5. **GitOps Integration** - Declarative deployment manifests

### Provider Roadmap
- **Phase 1**: Kubernetes, Docker
- **Phase 2**: OpenStack (if needed), Nomad
- **Phase 3**: Serverless (Lambda, Cloud Functions)
- **Phase 4**: Edge computing (Cloudflare Workers, Fastly)

## Success Metrics

1. **Zero Breaking Changes** - All existing tests pass
2. **Provider Coverage** - Support 5+ major cloud providers
3. **Performance** - <5s deployment initiation time
4. **Reliability** - 99.9% tunnel uptime
5. **Cost Efficiency** - <10% overhead vs direct deployment

## Conclusion

This specification provides a comprehensive, backwards-compatible solution for remote cloud deployments that:
- Maximally reuses existing Blueprint Manager components
- Provides a clean abstraction for any cloud provider
- Maintains security and networking models
- Enables progressive adoption without disruption
- Follows established patterns from the codebase

The implementation is designed to be completed in 5 weeks with extensive testing at each phase, ensuring production-ready quality throughout the development process.