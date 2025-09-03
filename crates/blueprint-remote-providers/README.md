# Blueprint Remote Providers

Extensions for deploying Blueprint instances to remote Kubernetes clusters and cloud environments, built on top of the existing Blueprint Manager runtime.

## Overview

This crate **extends** the existing Blueprint Manager's `ContainerRuntime` to support remote deployments without duplicating functionality. It adds:

- **Multi-cluster management** - Deploy to multiple Kubernetes clusters
- **Cost tracking** - Monitor cloud spending without modifying deployments
- **Cloud-specific networking** - Tunneling and networking configurations
- **Provider metadata** - Cloud provider information for optimizations

## Key Design Principle: Extend, Don't Replace

This crate **reuses** the existing Blueprint Manager infrastructure:

- ✅ Uses existing `ContainerRuntime` for all deployments
- ✅ Works with existing bridge/proxy communication
- ✅ Compatible with existing Kata containers sandboxing
- ✅ Leverages existing namespace and service management

## Architecture

```
┌──────────────────────────────┐
│   Blueprint Manager          │
│                              │
│  ┌────────────────────┐      │
│  │ ContainerRuntime    │◄─────┼─── Extended by RemoteClusterManager
│  │ - Deploy pods       │      │    to use remote Kubernetes clients
│  │ - Manage services   │      │
│  │ - Kata containers   │      │
│  └────────────────────┘      │
│                              │
│  ┌────────────────────┐      │
│  │ Bridge/Proxy        │◄─────┼─── Extended by TunnelManager for
│  │ - Communication     │      │    cross-cloud networking
│  │ - Auth             │      │
│  └────────────────────┘      │
└──────────────────────────────┘
                ▲
                │
                │ Remote deployments via
                │ existing infrastructure
                │
    ┌───────────┼───────────┐
    ▼           ▼           ▼
┌────────┐ ┌────────┐ ┌────────┐
│AWS EKS │ │GCP GKE │ │  Any   │
│        │ │        │ │  K8s   │
└────────┘ └────────┘ └────────┘
```

## Usage

### Basic Remote Deployment

```rust
use blueprint_remote_providers::{
    RemoteClusterManager,
    RemoteDeploymentConfig,
    CloudProvider,
    CostEstimator,
};
use blueprint_manager::rt::container::ContainerRuntime;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create cluster manager
    let manager = RemoteClusterManager::new();
    
    // Add remote clusters
    let aws_config = RemoteDeploymentConfig {
        kubeconfig_path: Some("~/.kube/eks-config".into()),
        context: Some("aws-production".into()),
        namespace: "blueprints".into(),
        provider: CloudProvider::AWS,
        region: Some("us-west-2".into()),
    };
    
    manager.add_cluster("aws", aws_config).await?;
    
    // Get ContainerRuntime configured for remote cluster
    // This is the SAME ContainerRuntime from blueprint-manager!
    let runtime = manager.get_runtime_for_active_cluster().await?;
    
    // Deploy using existing ContainerRuntime methods
    // All existing deployment logic works unchanged!
    runtime.deploy_pod(...).await?;
    
    // Add cost tracking on top
    let estimator = CostEstimator::new();
    let cost = estimator.estimate(
        &CloudProvider::AWS,
        2.0,  // CPUs
        4.0,  // Memory GB
        10.0, // Storage GB
        3,    // Replicas
    );
    
    println!("Estimated cost: ${}/month", cost.estimated_monthly);
    
    Ok(())
}
```

### Multi-Cluster Management

```rust
// Add multiple clusters
manager.add_cluster("aws", aws_config).await?;
manager.add_cluster("gcp", gcp_config).await?;
manager.add_cluster("azure", azure_config).await?;

// Switch between clusters
manager.set_active_cluster("gcp").await?;
let runtime = manager.get_runtime_for_active_cluster().await?;

// List all clusters
for (name, provider) in manager.list_clusters().await {
    println!("Cluster: {} ({})", name, provider);
}
```

### Networking Extensions

```rust
use blueprint_remote_providers::{
    TunnelManager,
    NetworkingMode,
    WireGuardConfig,
};

// Setup tunneling for private clusters
let tunnel_manager = TunnelManager::new(
    NetworkingMode::WireGuard {
        config: WireGuardConfig { ... },
    }
);

// Establish tunnel if needed (based on provider)
let tunnel = tunnel_manager.establish_if_needed(
    "my-cluster",
    &CloudProvider::Generic,  // Requires tunnel
    "cluster.local",
).await?;

// Use with existing bridge
let bridge_ext = RemoteBridgeExtension::new(Arc::new(tunnel_manager));
let endpoint = bridge_ext.get_remote_endpoint(
    "my-cluster",
    "original-endpoint",
).await?;
```

## Features

- `kubernetes` - Kubernetes cluster support (default)
- `docker` - Docker remote host support
- `testing` - Test utilities

## Configuration

```toml
# ~/.blueprint/remote-clusters.toml

[clusters.production]
kubeconfig = "~/.kube/eks-config"
context = "arn:aws:eks:us-west-2:123456789:cluster/production"
namespace = "blueprint-prod"
provider = "AWS"
region = "us-west-2"

[clusters.staging]
kubeconfig = "~/.kube/gke-config"
context = "gke_my-project_us-central1_staging"
namespace = "blueprint-stage"
provider = "GCP"
region = "us-central1"

[networking]
mode = "WireGuard"
private_key = "${WIREGUARD_KEY}"
endpoint = "vpn.example.com:51820"

[cost]
monthly_limit = 1000.0
alert_email = "ops@example.com"
```

## How It Works

1. **RemoteClusterManager** creates Kubernetes clients for remote clusters
2. These clients are passed to the existing `ContainerRuntime`
3. `ContainerRuntime` deploys to remote clusters using its existing logic
4. **CostEstimator** tracks costs without modifying deployments
5. **TunnelManager** handles networking for private clusters
6. Everything else (Kata containers, services, bridge) works unchanged

## Comparison with Local Deployment

| Feature | Local (Existing) | Remote (This Crate) |
|---------|-----------------|---------------------|
| Runtime | ContainerRuntime | Same ContainerRuntime |
| Deployment | Local kubeconfig | Remote kubeconfig |
| Networking | Bridge/Proxy | Bridge/Proxy + Tunnels |
| Sandboxing | Kata containers | Same Kata containers |
| Cost | N/A | CostEstimator layer |
| Multi-cluster | No | Yes via RemoteClusterManager |

## Testing

```bash
# Run tests
cargo test -p blueprint-remote-providers

# Test with real cluster (requires kubeconfig)
KUBECONFIG=/path/to/config cargo test --features integration
```

## Why This Approach?

1. **Maximum Reuse** - Uses existing ContainerRuntime, bridge, Kata containers
2. **Minimal Changes** - No modifications to core Blueprint Manager
3. **Clean Extension** - Adds capabilities without duplicating code
4. **Future Proof** - Easy to add new providers without changing core
5. **Backwards Compatible** - All existing deployments continue to work

## License

MIT OR Apache-2.0