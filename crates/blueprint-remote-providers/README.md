# Blueprint Remote Providers

Production-ready multi-cloud infrastructure provisioning for Blueprint services.

## Supported Providers

**Virtual Machines**: AWS EC2, GCP Compute Engine, Azure VMs, DigitalOcean Droplets, Vultr instances
**Kubernetes**: GKE, EKS, AKS, DOKS, VKE, generic clusters
**Deployment**: SSH-based binary deployment with real Blueprint execution

## Architecture & Entrypoints

### 1. Cloud Provider Adapters (High-Level)

```rust
use blueprint_remote_providers::{
    CloudProviderAdapter, DeploymentTarget, ResourceSpec,
    AwsAdapter, GcpAdapter, AzureAdapter, DigitalOceanAdapter, VultrAdapter,
};

// Provider-specific adapters
let aws = AwsAdapter::new().await?;
let gcp = GcpAdapter::new().await?;
let azure = AzureAdapter::new().await?;
let digitalocean = DigitalOceanAdapter::new().await?;
let vultr = VultrAdapter::new().await?;

// Deploy to VM via SSH
let result = aws.deploy_blueprint_with_target(
    &DeploymentTarget::VirtualMachine { 
        runtime: ContainerRuntime::Docker 
    },
    "blueprint-image:latest",
    &ResourceSpec::default(),
    env_vars,
).await?;

// Deploy to managed Kubernetes
let result = gcp.deploy_blueprint_with_target(
    &DeploymentTarget::ManagedKubernetes {
        cluster_id: "my-gke-cluster".to_string(),
        namespace: "blueprints".to_string(),
    },
    "blueprint-image:latest",
    &ResourceSpec::default(),
    env_vars,
).await?;
```

### 2. Cloud Provisioner (Unified Access)

```rust
use blueprint_remote_providers::{CloudProvisioner, CloudProvider};

// Single provisioner for all providers
let provisioner = CloudProvisioner::new().await?;

// Get adapter for specific provider
let adapter = provisioner.get_adapter(&CloudProvider::AWS)?;

// Deploy using the adapter
let result = adapter.deploy_blueprint_with_target(
    &DeploymentTarget::VirtualMachine { 
        runtime: ContainerRuntime::Docker 
    },
    "blueprint-image:latest",
    &ResourceSpec::default(),
    env_vars,
).await?;
```

### 3. SSH Deployment Client (Direct SSH Access)

```rust
use blueprint_remote_providers::deployment::ssh::{
    SshDeploymentClient, SshConnection, DeploymentConfig, ContainerRuntime
};
use std::collections::HashMap;

// Create SSH connection
let connection = SshConnection {
    host: "192.168.1.100".to_string(),
    port: 22,
    username: "ubuntu".to_string(),
    key_path: Some("/path/to/key.pem".to_string()),
};

// Configure deployment
let config = DeploymentConfig {
    name: "my-blueprint".to_string(),
    namespace: "production".to_string(),
    runtime: ContainerRuntime::Docker,
};

// Create SSH client
let ssh_client = SshDeploymentClient::new(connection, config);

// Deploy container with resource limits
let mut env_vars = HashMap::new();
env_vars.insert("ENV".to_string(), "production".to_string());

let resource_spec = ResourceSpec {
    cpu: 2.0,
    memory_gb: 4.0,
    storage_gb: 50.0,
    gpu_count: None,
    allow_spot: false,
    qos: Default::default(),
};

let container_id = ssh_client
    .deploy_container_with_resources(
        "my-blueprint:latest",
        "my-container",
        env_vars,
        Some(&resource_spec),
    )
    .await?;

// Health check
let is_healthy = ssh_client.health_check_container(&container_id).await?;

// Cleanup
ssh_client.remove_container(&container_id).await?;
```

### 4. Deployment Tracker (Lifecycle Management)

```rust
use blueprint_remote_providers::{
    DeploymentTracker, DeploymentRecord, DeploymentType
};

// Create tracker
let tracker_path = std::path::PathBuf::from(".tangle/deployments");
let tracker = DeploymentTracker::new(&tracker_path).await?;

// Track new deployment
let mut record = DeploymentRecord::new(
    "blueprint-id".to_string(),
    DeploymentType::AwsEc2,
    ResourceSpec::default(),
    Some(3600), // 1 hour TTL
);
record.set_cloud_info(CloudProvider::AWS, "us-east-1".to_string());
record.add_resource("instance_id".to_string(), "i-1234567890".to_string());

tracker.track(record.clone()).await?;

// List all deployments
let deployments = tracker.list_all().await?;

// Get specific deployment
let deployment = tracker.get(&record.id).await?;

// Remove deployment
tracker.remove(&record.id).await?;
```

### 5. Update Manager (Zero-Downtime Updates)

```rust
use blueprint_remote_providers::deployment::{
    UpdateManager, UpdateStrategy
};
use std::time::Duration;

// Create update manager with strategy
let strategy = UpdateStrategy::BlueGreen {
    switch_timeout: Duration::from_secs(300),
    health_check_duration: Duration::from_secs(60),
};
let mut update_manager = UpdateManager::new(strategy);

// Update blueprint
let new_deployment = update_manager
    .update_blueprint(
        adapter.as_ref(),
        "blueprint:v2.0",
        &resource_spec,
        env_vars,
        &current_deployment,
    )
    .await?;

// Rollback if needed
let rollback_deployment = update_manager
    .rollback(
        adapter.as_ref(),
        "v1.0",
        &current_deployment,
    )
    .await?;

// View version history
let versions = update_manager.list_versions();
let history = update_manager.get_history(10);

// Cleanup old versions
update_manager
    .cleanup_old_versions(adapter.as_ref(), 5)
    .await?;
```

### 6. QoS Tunnel (Metrics & Monitoring)

```rust
use blueprint_remote_providers::deployment::qos_tunnel::{
    QosTunnel, QosTunnelManager
};

// Create QoS tunnel to remote deployment
let tunnel = QosTunnel::new(
    "blueprint-id".to_string(),
    "192.168.1.100".to_string(),
    9615, // QoS metrics port
).await?;

// Get metrics endpoint
let metrics_url = tunnel.local_metrics_url();

// Manage multiple tunnels
let mut tunnel_manager = QosTunnelManager::new();
tunnel_manager.create_tunnel(
    "blueprint-1",
    "192.168.1.100",
    9615,
).await?;

// Get tunnel
if let Some(tunnel) = tunnel_manager.get_tunnel("blueprint-1") {
    println!("Metrics at: {}", tunnel.local_metrics_url());
}

// Close tunnel
tunnel_manager.close_tunnel("blueprint-1").await?;
```

### 7. Kubernetes Deployment (Optional Feature)

```rust
#[cfg(feature = "kubernetes")]
use blueprint_remote_providers::deployment::kubernetes::{
    KubernetesDeploymentClient, KubeConfig
};

// Create Kubernetes client
let kube_config = KubeConfig {
    context: Some("my-cluster".to_string()),
    namespace: "blueprints".to_string(),
};
let k8s_client = KubernetesDeploymentClient::new(kube_config).await?;

// Deploy to Kubernetes
let deployment_result = k8s_client
    .deploy_blueprint(
        "blueprint-image:latest",
        "my-blueprint",
        &resource_spec,
        env_vars,
    )
    .await?;
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