# CLI Remote Deployment Enhancement Specification

## Executive Summary

Enhance the `cargo-tangle` CLI to support remote cloud deployments, enabling developers to deploy Blueprint services to AWS, GCP, Azure, DigitalOcean, and Vultr with a single command. This will position the platform to compete with services like depot.dev and Docker Build Cloud.

## Current State Analysis

### Existing CLI Capabilities
1. **Blueprint Creation** (`cargo tangle blueprint create`)
   - Template-based project generation
   - Support for Tangle and Eigenlayer blueprints
   - Interactive and non-interactive modes

2. **Deployment** (`cargo tangle blueprint deploy`)
   - Local devnet deployment
   - Tangle Network deployment
   - Eigenlayer deployment
   - MBSM contract deployment

3. **Running** (`cargo tangle blueprint run`)
   - Local execution with Blueprint Manager
   - Keystore management
   - Protocol settings configuration

4. **Service Management**
   - Request/accept/reject service instances
   - Job submission
   - Blueprint registration

### Missing for Remote Cloud Deployment
1. No cloud provider configuration
2. No resource specification in CLI
3. No remote deployment target option
4. No cloud credentials management
5. No cost estimation before deployment
6. No remote instance monitoring

## Proposed CLI Enhancements

### 1. New Commands

#### `cargo tangle cloud configure`
Configure cloud provider credentials and settings.

```bash
# Interactive setup
cargo tangle cloud configure

# Direct configuration
cargo tangle cloud configure --provider aws --region us-east-1 --credentials-file ~/.aws/credentials

# List configured providers
cargo tangle cloud list-providers
```

#### `cargo tangle cloud estimate`
Estimate deployment costs before deployment.

```bash
# Estimate costs for a blueprint
cargo tangle cloud estimate --blueprint-id 123 --provider aws --duration 730h

# Compare costs across providers
cargo tangle cloud estimate --compare-all --cpu 4 --memory 16 --storage 100
```

#### `cargo tangle blueprint deploy --remote`
Deploy to remote cloud providers.

```bash
# Deploy to specific cloud
cargo tangle blueprint deploy tangle --remote aws --region us-west-2 --instance-type t3.medium

# Deploy with resource requirements
cargo tangle blueprint deploy tangle --remote gcp \
  --cpu 4 --memory 16 --storage 100 --gpu 1

# Deploy with TTL
cargo tangle blueprint deploy tangle --remote vultr --ttl 24h

# Deploy to cheapest provider
cargo tangle blueprint deploy tangle --remote auto --max-cost 0.50
```

#### `cargo tangle cloud status`
Monitor remote deployments.

```bash
# List all deployments
cargo tangle cloud status

# Check specific deployment
cargo tangle cloud status --deployment-id dep-xyz123

# Watch deployment health
cargo tangle cloud status --watch --deployment-id dep-xyz123
```

#### `cargo tangle cloud terminate`
Clean up remote deployments.

```bash
# Terminate specific deployment
cargo tangle cloud terminate --deployment-id dep-xyz123

# Terminate all deployments for a blueprint
cargo tangle cloud terminate --blueprint-id 123 --all

# Terminate expired deployments
cargo tangle cloud terminate --expired
```

### 2. Blueprint.toml Resource Specification

Add resource requirements to blueprint configuration:

```toml
[package]
name = "my-blueprint"
version = "0.1.0"

[blueprint]
protocol = "tangle"

[blueprint.resources]
# Minimum requirements
min_cpu = 2.0
min_memory_gb = 4.0
min_storage_gb = 20.0

# Recommended requirements (for production)
recommended_cpu = 4.0
recommended_memory_gb = 16.0
recommended_storage_gb = 100.0

# Optional GPU requirements
gpu_count = 0
gpu_type = "nvidia-a100"

# Network requirements
public_ip = true
bandwidth_tier = "standard"

# Allow spot instances
allow_spot = true
```

### 3. Settings File for Remote Deployment

`.blueprint-cloud.toml` configuration file:

```toml
[defaults]
provider = "aws"
region = "us-east-1"
max_hourly_cost = 1.0
auto_terminate_hours = 24

[aws]
credentials_file = "~/.aws/credentials"
profile = "default"
default_instance_type = "t3.medium"

[gcp]
project_id = "my-project"
credentials_file = "~/.gcp/credentials.json"
default_machine_type = "n2-standard-4"

[monitoring]
health_check_interval = 60
auto_recovery = true
alerts_webhook = "https://hooks.slack.com/..."
```

### 4. Interactive Deployment Flow

```
$ cargo tangle blueprint deploy tangle --remote

? Select deployment target:
  > AWS EC2
    GCP Compute Engine
    Azure VM
    DigitalOcean Droplet
    Vultr Instance
    Auto (cheapest)

? Select region:
  > us-east-1 (N. Virginia)
    us-west-2 (Oregon)
    eu-west-1 (Ireland)

? Configure resources:
  CPU cores: 4
  Memory (GB): 16
  Storage (GB): 100
  GPU: None

? Deployment options:
  [x] Use spot instances (30% discount)
  [ ] Enable auto-recovery
  [x] Set TTL (24 hours)

Estimated cost: $0.42/hour ($306.60/month)
? Proceed with deployment? (Y/n)

Deploying blueprint...
✓ Provisioning instance i-1234567890
✓ Installing dependencies
✓ Deploying blueprint service
✓ Configuring networking
✓ Starting health monitoring

Deployment successful!
Instance ID: i-1234567890
Public IP: 54.123.45.67
Status: Running
Dashboard: https://tangle.tools/deployments/dep-xyz123
```

## Implementation Plan

### Phase 1: Core CLI Infrastructure (Week 1-2)
1. Add `cloud` subcommand structure
2. Implement provider configuration management
3. Add resource specification parsing
4. Integrate with blueprint-remote-providers crate

### Phase 2: Deployment Commands (Week 2-3)
1. Implement `deploy --remote` functionality
2. Add cost estimation command
3. Implement provider selection logic
4. Add interactive deployment flow

### Phase 3: Monitoring & Management (Week 3-4)
1. Implement status monitoring
2. Add termination commands
3. Create health check integration
4. Add deployment listing and filtering

### Phase 4: Developer Experience (Week 4-5)
1. Add Blueprint.toml resource specification
2. Create project templates with cloud config
3. Add debugging tools for remote deployments
4. Implement log streaming from remote instances

### Phase 5: Production Features (Week 5-6)
1. Add multi-region deployment support
2. Implement blue-green deployments
3. Add rollback capabilities
4. Create CI/CD integration examples

## Use Cases for Blueprint Services

### 1. Build Cloud Service (depot.dev competitor)
```rust
// Blueprint that provides distributed build services
#[job(id = 1, name = "docker-build")]
fn docker_build(dockerfile: String, context: Vec<u8>) -> Result<ImageId> {
    // Execute Docker build on remote infrastructure
    // Leverage multiple machines for layer caching
    // Return built image ID
}
```

### 2. Distributed Compute Service
```rust
// Blueprint for distributed ML training
#[job(id = 1, name = "train-model")]
fn train_model(dataset: Url, config: TrainingConfig) -> Result<ModelWeights> {
    // Distribute training across GPU nodes
    // Aggregate results
    // Return trained model
}
```

### 3. Data Processing Pipeline
```rust
// Blueprint for ETL operations
#[job(id = 1, name = "process-dataset")]
fn process_dataset(input: S3Location, transform: Script) -> Result<S3Location> {
    // Process large datasets in parallel
    // Store results back to S3
}
```

### 4. Serverless Function Platform
```rust
// Blueprint for running user functions
#[job(id = 1, name = "execute-function")]
fn execute_function(code: WasmBinary, input: Json) -> Result<Json> {
    // Run isolated WASM functions
    // Auto-scale based on demand
}
```

## Success Metrics

1. **Developer Adoption**
   - Time to first remote deployment < 5 minutes
   - 80% of blueprints include resource specifications
   - 50% of deployments use remote infrastructure

2. **Operational Efficiency**
   - 99.9% deployment success rate
   - < 2 minute provisioning time
   - Automatic recovery within 5 minutes

3. **Cost Optimization**
   - 30% cost savings with spot instances
   - Accurate cost estimation (±10%)
   - Auto-termination prevents runaway costs

## Security Considerations

1. **Credential Management**
   - Never store credentials in Blueprint.toml
   - Use system keychain integration
   - Support IAM roles for cloud providers

2. **Network Security**
   - Default deny-all firewall rules
   - Explicit port allowlisting
   - VPN/tunnel support for private deployments

3. **Compliance**
   - Audit logging for all deployments
   - Resource tagging for cost allocation
   - Region restrictions for data sovereignty

## Conclusion

These CLI enhancements will transform the Blueprint SDK into a comprehensive platform for deploying distributed services. By adding remote cloud deployment capabilities, we enable developers to build production-ready services that can compete with established platforms like depot.dev and Docker Build Cloud.

The key differentiator is the Blueprint protocol's built-in support for:
- Decentralized job coordination
- Cryptographic proof of execution
- Native multi-cloud support
- Cost-optimized resource allocation

This positions the Tangle Network as the premier platform for building the next generation of distributed cloud services.