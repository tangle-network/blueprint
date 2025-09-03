# Blueprint Remote Providers

A pluggable provider system for deploying Blueprint instances to remote cloud infrastructure, supporting Kubernetes, Docker, and bare-metal deployments.

## Features

- **Multi-Cloud Support**: Deploy to AWS EKS, GCP GKE, Azure AKS, DigitalOcean, and more
- **Provider Abstraction**: Unified interface for different infrastructure providers
- **Cost Estimation**: Built-in cost estimation for cloud deployments
- **Resource Management**: Track and manage resources across providers
- **Tunnel Support**: Secure WireGuard tunnels for private networking
- **100% Backwards Compatible**: Extends existing Blueprint Manager without breaking changes

## Architecture

The crate follows a provider-based architecture:

```rust
pub trait RemoteInfrastructureProvider {
    async fn deploy_instance(&self, spec: DeploymentSpec) -> Result<RemoteInstance>;
    async fn get_instance_status(&self, id: &InstanceId) -> Result<InstanceStatus>;
    async fn terminate_instance(&self, id: &InstanceId) -> Result<()>;
    // ... more methods
}
```

## Supported Providers

### Kubernetes (Primary)
- Any Kubernetes cluster with kubeconfig access
- Managed services: EKS, GKE, AKS, DigitalOcean K8s, etc.
- Automatic namespace management
- Service endpoint discovery

### Docker (Secondary)
- Local Docker daemon
- Remote Docker via TLS
- Container lifecycle management
- Port mapping support

### SSH (Future)
- Bare-metal server deployments
- Custom VM provisioning

## Usage

### Basic Deployment

```rust
use blueprint_remote_providers::{
    kubernetes::{KubernetesProvider, KubernetesConfig},
    provider::RemoteInfrastructureProvider,
    types::DeploymentSpec,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure provider
    let config = KubernetesConfig {
        namespace: "my-blueprints".to_string(),
        ..Default::default()
    };
    
    // Create provider
    let provider = KubernetesProvider::new("k8s", config).await?;
    
    // Define deployment
    let spec = DeploymentSpec {
        name: "my-blueprint".to_string(),
        replicas: 3,
        ..Default::default()
    };
    
    // Deploy
    let instance = provider.deploy_instance(spec).await?;
    println!("Deployed: {}", instance.id);
    
    Ok(())
}
```

### Provider Registry

```rust
use blueprint_remote_providers::provider::ProviderRegistry;
use std::sync::Arc;

let registry = ProviderRegistry::new();

// Register multiple providers
registry.register("aws", aws_provider).await;
registry.register("gcp", gcp_provider).await;

// Deploy to specific provider
let provider = registry.get("aws").await.unwrap();
let instance = provider.deploy_instance(spec).await?;
```

### Cost Estimation

```rust
let cost = provider.estimate_cost(&spec).await?;
println!("Estimated cost: ${:.2}/month", cost.estimated_monthly);
```

## Integration with Blueprint Manager

The crate integrates seamlessly with the existing Blueprint Manager:

```rust
#[cfg(feature = "blueprint-manager")]
use blueprint_remote_providers::manager_integration::RemoteDeploymentExt;

// Extend existing manager with remote capabilities
manager.deploy_remote(provider, spec).await?;
```

## Configuration

### Kubernetes Provider

```toml
[remote.kubernetes]
kubeconfig = "~/.kube/config"
context = "production"
namespace = "blueprints"
service_type = "LoadBalancer"
```

### Docker Provider

```toml
[remote.docker]
endpoint = "tcp://docker.example.com:2376"
tls_cert = "~/.docker/cert.pem"
tls_key = "~/.docker/key.pem"
network = "bridge"
```

## Testing

The crate includes comprehensive tests:

```bash
# Run unit tests
cargo test -p blueprint-remote-providers

# Run integration tests (requires Docker/K8s)
cargo test -p blueprint-remote-providers --all-features -- --ignored

# Run specific provider tests
cargo test -p blueprint-remote-providers --features kubernetes
```

## Security

- All cloud credentials use existing keystore
- Network traffic through encrypted tunnels
- Provider-specific IAM integration
- Resource isolation per deployment

## Future Enhancements

- [ ] OpenStack support (if needed)
- [ ] Nomad orchestration
- [ ] Terraform integration
- [ ] GitOps workflows
- [ ] Multi-region deployments
- [ ] Auto-scaling policies

## License

MIT OR Apache-2.0