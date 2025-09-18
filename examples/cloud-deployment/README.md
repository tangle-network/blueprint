# Cloud Deployment Example

This example demonstrates how to deploy a Blueprint service to cloud providers using the Tangle CLI.

## Architecture

The Tangle CLI follows a **configuration-based approach** for cloud deployments:

1. **CLI configures deployment policies** - Set provider preferences, cost limits, regions
2. **Blueprint Manager orchestrates deployments** - Uses policies to make intelligent provider selection
3. **Remote providers crate handles provisioning** - Actual cloud infrastructure management

This ensures Blueprint Manager remains the single point of orchestration while enabling flexible cloud deployment strategies.

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

### 2. Configure Deployment Policy

Set up intelligent provider selection based on workload requirements:

```bash
# Configure provider preferences by workload type
cargo tangle cloud policy \
  --gpu-providers "gcp,aws" \
  --cpu-providers "vultr,digitalocean" \
  --cost-providers "vultr,digitalocean" \
  --max-cost 5.00 \
  --prefer-spot true

# Show current policy
cargo tangle cloud show-policy
```

### 3. Estimate Costs

Before deploying, estimate costs:

```bash
# Estimate for specific configuration
cargo tangle cloud estimate --cpu 4 --memory 16 --duration 24h

# Compare all providers
cargo tangle cloud estimate --compare --cpu 8 --memory 32 --gpu 1
```

### 4. Deploy Blueprint

Deploy your blueprint using the configured policy:

```bash
# Deploy using configured policy (Blueprint Manager selects best provider)
cargo tangle blueprint deploy tangle --remote

# Override provider/region if needed
cargo tangle blueprint deploy tangle --remote --provider aws --region us-west-2
```

### 5. Monitor Deployment

Check deployment status:

```bash
# List all deployments
cargo tangle cloud status

# Check specific deployment
cargo tangle cloud status --deployment-id dep-abc123

# Watch deployment in real-time
cargo tangle cloud status --watch
```

### 6. Terminate Deployment

Clean up when done:

```bash
# Terminate specific deployment
cargo tangle cloud terminate --deployment-id dep-abc123

# Terminate all deployments
cargo tangle cloud terminate --all
```

## Cargo.toml Configuration (Optional)

You can optionally add cloud resource specifications to your existing `Cargo.toml`:

```toml
# This section is OPTIONAL - system works without it
[package.metadata.blueprint.resources]
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

**Note**: If you don't add this section, the system will use sensible defaults and you can still override resources via CLI arguments.

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