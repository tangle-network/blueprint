# Blueprint Remote Providers

Cloud infrastructure provisioning and deployment for Blueprint services.

## Features

- AWS EC2, GCP Compute Engine, Azure VMs, DigitalOcean Droplets, Vultr instances
- Kubernetes cluster deployment support
- SSH-based binary deployment
- Cost estimation and tracking
- Provider selection based on resource requirements

## Usage

```rust
use blueprint_remote_providers::{CloudProvisioner, ResourceSpec};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provisioner = CloudProvisioner::new().await?;
    
    let spec = ResourceSpec {
        cpu: 4.0,
        memory_gb: 16.0,
        storage_gb: 100.0,
        gpu_count: None,
        allow_spot: true,
    };
    
    let instance = provisioner.provision(
        CloudProvider::AWS,
        &spec,
        "us-west-2",
    ).await?;
    
    println!("Instance provisioned: {}", instance.instance_id);
    Ok(())
}
```

## Configuration

Configure providers via environment variables or config files:

```toml
[aws]
region = "us-west-2"
availability_zone = "us-west-2a"

[digitalocean]
region = "nyc3"
ssh_key_id = "12345678"
```

## Testing

```bash
cargo test -p blueprint-remote-providers
```

## License

MIT OR Apache-2.0