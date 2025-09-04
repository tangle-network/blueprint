# Cloud Deployment Example

This example demonstrates how to deploy a Blueprint service to cloud providers using the Tangle CLI.

## Quick Start

### 1. Configure Cloud Provider

First, configure your preferred cloud provider:

```bash
# Configure AWS
cargo tangle cloud configure aws --region us-east-1 --set-default

# Configure GCP
cargo tangle cloud configure gcp --region us-central1

# Configure DigitalOcean
cargo tangle cloud configure digitalocean --region nyc3

# List configured providers
cargo tangle cloud list
```

### 2. Estimate Costs

Before deploying, estimate costs:

```bash
# Estimate for specific configuration
cargo tangle cloud estimate --cpu 4 --memory 16 --duration 24h

# Compare all providers
cargo tangle cloud estimate --compare --cpu 8 --memory 32 --gpu 1

# Include spot pricing
cargo tangle cloud estimate --spot --cpu 4 --memory 16
```

### 3. Deploy Blueprint

Deploy your blueprint to the cloud:

```bash
# Deploy to default provider
cargo tangle blueprint deploy tangle --remote

# Deploy to specific provider with custom resources
cargo tangle blueprint deploy tangle --remote aws \
  --cpu 8 --memory 32 --storage 500 \
  --spot --ttl 24h

# Deploy to cheapest provider automatically
cargo tangle blueprint deploy tangle --remote auto --max-cost 0.50
```

### 4. Monitor Deployment

Check deployment status:

```bash
# List all deployments
cargo tangle cloud status

# Check specific deployment
cargo tangle cloud status --deployment-id dep-abc123

# Watch deployment in real-time
cargo tangle cloud status --watch
```

### 5. Terminate Deployment

Clean up when done:

```bash
# Terminate specific deployment
cargo tangle cloud terminate --deployment-id dep-abc123

# Terminate all deployments
cargo tangle cloud terminate --all
```

## Blueprint.toml Configuration

The `Blueprint.toml` file defines resource requirements:

```toml
[blueprint.resources]
# Minimum requirements
min_cpu = 2.0
min_memory_gb = 4.0
min_storage_gb = 20.0

# Recommended for production
recommended_cpu = 8.0
recommended_memory_gb = 32.0
recommended_storage_gb = 500.0

# GPU support
gpu_count = 1
gpu_type = "nvidia-t4"

# Cost optimization
allow_spot = true  # 30% discount with spot instances
```

## Environment-Specific Settings

Deploy with different configurations per environment:

```bash
# Development (minimal resources, auto-terminate)
cargo tangle blueprint deploy tangle --remote --env development

# Staging (moderate resources)
cargo tangle blueprint deploy tangle --remote --env staging

# Production (full resources, no spot instances)
cargo tangle blueprint deploy tangle --remote --env production
```

## Cost Optimization Tips

1. **Use Spot Instances**: Add `--spot` for 30% savings
2. **Set TTL**: Use `--ttl 24h` to auto-terminate
3. **Compare Providers**: Use `cloud estimate --compare`
4. **Right-size Resources**: Start small, scale as needed
5. **Use Vultr/DigitalOcean**: Often cheaper for smaller workloads

## Supported Providers

- **AWS**: EC2 instances with full feature support
- **GCP**: Compute Engine with preemptible instances
- **Azure**: Virtual Machines with spot instances
- **DigitalOcean**: Droplets with predictable pricing
- **Vultr**: Cloud Compute with competitive rates

## Troubleshooting

### Provider Not Configured
```bash
cargo tangle cloud configure <provider>
```

### Authentication Issues
- AWS: Check `~/.aws/credentials` or set `AWS_ACCESS_KEY_ID`
- GCP: Run `gcloud auth application-default login`
- Azure: Run `az login`
- DigitalOcean/Vultr: Set API token in environment

### Deployment Failures
1. Check logs: `cargo tangle cloud status --deployment-id <id>`
2. Verify resources: Ensure sufficient quota in region
3. Check networking: Ensure ports are accessible

## Example Use Cases

### 1. CI/CD Build Service
Deploy a distributed Docker build service competing with depot.dev:
```bash
cargo tangle blueprint deploy tangle --remote aws \
  --cpu 16 --memory 64 --storage 1000 \
  --spot --ttl 2h
```

### 2. ML Training Job
Deploy GPU-enabled training infrastructure:
```bash
cargo tangle blueprint deploy tangle --remote gcp \
  --cpu 8 --memory 32 --gpu 1 \
  --region us-central1
```

### 3. Serverless Platform
Deploy lightweight function execution environment:
```bash
cargo tangle blueprint deploy tangle --remote digitalocean \
  --cpu 2 --memory 4 --storage 20 \
  --spot
```

## Next Steps

1. Customize `Blueprint.toml` for your service
2. Set up CI/CD integration for automated deployments
3. Configure monitoring and alerts
4. Implement auto-scaling based on load