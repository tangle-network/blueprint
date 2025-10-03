# Blueprint Remote Providers

Production-ready multi-cloud infrastructure provisioning for Blueprint services.

## Supported Providers

**Virtual Machines**: AWS EC2, GCP Compute Engine, Azure VMs, DigitalOcean Droplets, Vultr instances
**Kubernetes**: GKE, EKS, AKS, DOKS, VKE, generic clusters
**Deployment**: SSH-based binary deployment with real Blueprint execution

## Architecture

```rust
use blueprint_remote_providers::{
    CloudProviderAdapter, DeploymentTarget, ResourceSpec
};

// Provider-specific adapters
let aws = AwsAdapter::new().await?;
let gcp = GcpAdapter::new().await?;
let azure = AzureAdapter::new().await?;
let digitalocean = DigitalOceanAdapter::new().await?;
let vultr = VultrAdapter::new().await?;

// Deploy to VM via SSH
let result = aws.deploy_blueprint_with_target(
    &DeploymentTarget::VirtualMachine { runtime: None },
    "blueprint-image:latest",
    &ResourceSpec::default(),
    env_vars,
).await?;

// Deploy to managed Kubernetes
let result = gcp.deploy_blueprint_with_target(
    &DeploymentTarget::ManagedKubernetes {
        cluster_id: "my-gke-cluster",
        namespace: "blueprints",
    },
    "blueprint-image:latest",
    &ResourceSpec::default(),
    env_vars,
).await?;
```

## Configuration

Set provider credentials via environment variables:

```bash
# AWS
export AWS_ACCESS_KEY_ID="..."
export AWS_SECRET_ACCESS_KEY="..."

# GCP
export GCP_PROJECT_ID="my-project"
export GCP_ACCESS_TOKEN="..."

# Azure
export AZURE_SUBSCRIPTION_ID="..."
export AZURE_ACCESS_TOKEN="..."

# DigitalOcean
export DO_API_TOKEN="..."

# Vultr
export VULTR_API_KEY="..."
```

## Features

**Production-Ready**: All critical issues resolved, comprehensive testing implemented
**Shared Components**: Unified SSH deployment and security group management
**Real Implementations**: No mocking in production code paths
**Cost-Controlled Testing**: E2E tests with $0.01-0.10 cloud resource limits
**Security**: Unified firewall/security group abstractions across providers

## Testing

```bash
# All tests (197 functions across 44 files)
cargo test -p blueprint-remote-providers

# Feature-specific tests
cargo test -p blueprint-remote-providers --features kubernetes
cargo test -p blueprint-remote-providers --features aws,gcp

# E2E tests (requires cloud credentials)
cargo test -p blueprint-remote-providers test_multi_provider_real_sdk_integration -- --nocapture
```

## Provider Support

| Feature | AWS | GCP | Azure | DigitalOcean | Vultr |
|---------|-----|-----|-------|--------------|-------|
| **VM Provisioning** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **SSH Deployment** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Managed K8s** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Security Groups** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Health Checks** | ✅ | ✅ | ✅ | ✅ | ✅ |

✅ **PRODUCTION READY** - All critical issues resolved, comprehensive testing implemented