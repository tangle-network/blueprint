# Remote Cloud Deployment Guide for Blueprint Operators

## Overview

This guide explains how to deploy and manage Blueprint instances on remote cloud infrastructure, allowing operators to run the Blueprint Manager locally in lightweight mode while leveraging cloud resources for actual service execution.

## Architecture

```
┌─────────────────┐
│  Local Machine  │
│                 │
│ Blueprint       │
│ Manager (lite)  ├──────┐
│                 │      │
│ - Auth Proxy    │      │ WireGuard/Direct
│ - Bridge        │      │ Connection
│ - Registry      │      │
└─────────────────┘      │
                         │
        ┌────────────────┼────────────────┐
        │                │                │
        ▼                ▼                ▼
┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│   AWS EKS    │ │   GCP GKE    │ │  Bare Metal  │
│              │ │              │ │              │
│  Blueprint   │ │  Blueprint   │ │  Blueprint   │
│  Instances   │ │  Instances   │ │  Instances   │
└──────────────┘ └──────────────┘ └──────────────┘
```

## Quick Start

### 1. Install Dependencies

```bash
# Install the Blueprint CLI with remote providers support
cargo install cargo-tangle --features remote-providers

# Verify installation
cargo tangle --version
```

### 2. Configure Cloud Providers

Create a configuration file at `~/.blueprint/remote-config.toml`:

```toml
# AWS EKS Configuration
[providers.aws]
type = "kubernetes"
name = "aws-eks-provider"
kubeconfig = "~/.kube/eks-config"
context = "arn:aws:eks:us-west-2:123456789:cluster/my-cluster"
namespace = "blueprint-production"
service_type = "LoadBalancer"

# GCP GKE Configuration
[providers.gcp]
type = "kubernetes"
name = "gcp-gke-provider"
kubeconfig = "~/.kube/gke-config"
context = "gke_my-project_us-central1_my-cluster"
namespace = "blueprint-production"
service_type = "ClusterIP"  # Use with Ingress

# Docker (for development/edge)
[providers.docker]
type = "docker"
name = "docker-local"
endpoint = "unix:///var/run/docker.sock"

# SSH Bare Metal
[providers.bare-metal]
type = "ssh"
name = "homelab"
hosts = ["192.168.1.10", "192.168.1.11", "192.168.1.12"]
user = "blueprint"
key_path = "~/.ssh/blueprint_rsa"
runtime = "docker"

# Networking Configuration
[network]
tunnel_enabled = true
hub_endpoint = "manager.blueprint.network"
hub_port = 51820
private_key = "${WIREGUARD_PRIVATE_KEY}"
```

### 3. Set Up Cloud Credentials

#### AWS EKS
```bash
# Configure AWS CLI
aws configure

# Update kubeconfig for EKS
aws eks update-kubeconfig --name my-cluster --region us-west-2
```

#### GCP GKE
```bash
# Authenticate with GCP
gcloud auth login

# Get GKE credentials
gcloud container clusters get-credentials my-cluster \
    --zone us-central1-a \
    --project my-project
```

#### DigitalOcean Kubernetes
```bash
# Install doctl
brew install doctl  # or your package manager

# Authenticate
doctl auth init

# Get cluster credentials
doctl kubernetes cluster kubeconfig save my-cluster
```

### 4. Deploy Your First Blueprint

```bash
# Deploy to AWS
cargo tangle blueprint deploy \
    --remote aws \
    --package my-blueprint \
    --replicas 3

# Deploy to GCP
cargo tangle blueprint deploy \
    --remote gcp \
    --package my-blueprint \
    --replicas 2

# Deploy to bare metal
cargo tangle blueprint deploy \
    --remote bare-metal \
    --package my-blueprint
```

## Advanced Configuration

### Lightweight Manager Mode

Run the Blueprint Manager in lightweight mode, delegating heavy workloads to remote clouds:

```rust
use blueprint_sdk::remote::{
    ProviderRegistry,
    kubernetes::{KubernetesProvider, KubernetesConfig},
    RemoteBridgeManager,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize provider registry
    let registry = ProviderRegistry::new();
    
    // Configure remote provider
    let k8s_config = KubernetesConfig {
        kubeconfig_path: Some("~/.kube/config".into()),
        context: Some("production".into()),
        namespace: "blueprints".into(),
        ..Default::default()
    };
    
    // Register provider
    let provider = Arc::new(
        KubernetesProvider::new("aws", k8s_config).await?
    );
    registry.register("aws", provider.clone()).await;
    
    // Initialize bridge manager for remote connections
    let bridge_manager = RemoteBridgeManager::new();
    
    // Deploy blueprint
    let spec = DeploymentSpec {
        name: "my-service".into(),
        replicas: 3,
        resources: ResourceLimits {
            cpu: Some("2".into()),
            memory: Some("4Gi".into()),
            storage: Some("10Gi".into()),
        },
        ..Default::default()
    };
    
    let instance = provider.deploy_instance(spec).await?;
    
    // Establish bridge connection
    let connection = bridge_manager
        .connect_to_instance(provider, &instance.id)
        .await?;
    
    println!("Connected to remote instance: {:?}", connection);
    
    Ok(())
}
```

### Multi-Region Deployment

Deploy the same blueprint across multiple regions for high availability:

```bash
# Deploy to multiple regions simultaneously
for region in us-west-2 eu-central-1 ap-southeast-1; do
    cargo tangle blueprint deploy \
        --remote aws-$region \
        --package my-blueprint \
        --replicas 2 &
done
wait
```

### Cost Optimization

Monitor and optimize cloud costs:

```bash
# Get cost estimates before deployment
cargo tangle blueprint estimate-cost \
    --remote aws \
    --package my-blueprint \
    --replicas 5

# Output:
# Estimated costs for aws:
#   Hourly:  $0.35
#   Monthly: $255.50
#   Breakdown:
#     - Compute: $200.00
#     - Storage: $30.00
#     - Network: $25.50

# List all running instances with costs
cargo tangle blueprint list --remote --show-costs

# Set cost alerts
cargo tangle blueprint set-cost-alert \
    --remote aws \
    --monthly-limit 500 \
    --email alerts@example.com
```

### Secure Networking

#### WireGuard Tunnel Setup

For private cloud resources, establish secure tunnels:

```bash
# Generate WireGuard keys
wg genkey | tee privatekey | wg pubkey > publickey

# Configure tunnel in remote-config.toml
[network.tunnel]
enabled = true
interface = "wg0"
private_key_path = "~/.wireguard/privatekey"
public_key_path = "~/.wireguard/publickey"
hub_endpoint = "hub.blueprint.network"
hub_port = 51820
allowed_ips = ["10.100.0.0/24"]
persistent_keepalive = 25
```

#### Direct Connection (Public Endpoints)

For services with public endpoints:

```toml
[providers.aws]
service_type = "LoadBalancer"  # Creates public endpoint
tunnel_required = false
```

### Monitoring and Observability

Monitor remote deployments:

```bash
# Get instance status
cargo tangle blueprint status --remote aws --instance bp-abc123

# Stream logs from remote instance
cargo tangle blueprint logs --remote aws --instance bp-abc123 --follow

# Health check all remote connections
cargo tangle blueprint health --remote --all
```

## Production Checklist

- [ ] **Cloud Credentials**: Securely configured and tested
- [ ] **Network Security**: Firewalls and security groups configured
- [ ] **Resource Limits**: CPU/memory limits set appropriately
- [ ] **Cost Alerts**: Monthly spending limits configured
- [ ] **Monitoring**: Logging and metrics collection enabled
- [ ] **Backup**: State and configuration backups automated
- [ ] **High Availability**: Multi-region deployment configured
- [ ] **Disaster Recovery**: Failover procedures documented

## Troubleshooting

### Connection Issues

```bash
# Test provider connectivity
cargo tangle blueprint test-connection --remote aws

# Debug bridge connections
RUST_LOG=blueprint_remote_providers=debug cargo tangle blueprint deploy ...

# Check tunnel status
sudo wg show
```

### Deployment Failures

```bash
# Check cloud provider logs
kubectl logs -n blueprint-production deployment/my-blueprint  # For K8s
docker logs my-blueprint  # For Docker

# Retry with verbose logging
RUST_LOG=debug cargo tangle blueprint deploy --remote aws --package my-blueprint
```

### Cost Overruns

```bash
# Terminate all instances in a provider
cargo tangle blueprint terminate-all --remote aws --confirm

# Scale down deployments
cargo tangle blueprint scale --remote aws --instance bp-abc123 --replicas 1
```

## Best Practices

1. **Start Small**: Test with a single provider before multi-cloud
2. **Use Namespaces**: Isolate environments (dev/staging/prod)
3. **Resource Tags**: Tag all resources for cost tracking
4. **Regular Cleanup**: Remove unused instances weekly
5. **Security First**: Always use encrypted connections
6. **Monitor Costs**: Set up daily cost reports
7. **Documentation**: Document your deployment architecture

## Migration from Local to Remote

Migrating existing local deployments to remote:

```bash
# Export current configuration
cargo tangle blueprint export-config > local-config.json

# Import to remote provider
cargo tangle blueprint import-config \
    --remote aws \
    --file local-config.json \
    --migrate

# Verify migration
cargo tangle blueprint verify-migration \
    --source local \
    --target aws
```

## Support

- Documentation: https://docs.tangle.tools/remote-providers
- Discord: https://discord.com/invite/tangle-network
- GitHub Issues: https://github.com/tangle-network/blueprint/issues

## Appendix: Provider-Specific Notes

### AWS EKS
- Requires IAM permissions for EKS, EC2, and VPC
- LoadBalancer services incur additional costs
- Consider using Fargate for serverless containers

### GCP GKE
- Enable required APIs: container.googleapis.com
- Autopilot clusters recommended for cost optimization
- Use Workload Identity for service authentication

### Azure AKS
- Requires Azure CLI and kubectl
- Use Azure CNI for advanced networking
- Enable cluster autoscaling for dynamic workloads

### DigitalOcean Kubernetes
- Simple setup with predictable pricing
- Limited to specific regions
- Good for small to medium deployments

### Bare Metal (SSH)
- Requires Docker or systemd on target hosts
- No automatic scaling
- Full control over resources
- Lowest cost for owned infrastructure