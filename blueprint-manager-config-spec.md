# Blueprint Manager Remote Configuration Specification

## Current Issue

The CLI cloud commands are doing orchestration instead of configuration. They should configure the Blueprint Manager to intelligently select deployment targets based on resource requirements.

## Correct Architecture

### 1. CLI Configures Deployment Policies

```bash
# Configure provider preferences
cargo tangle cloud configure-policy \
  --gpu-providers "gcp,aws" \
  --cpu-intensive "vultr,digitalocean" \
  --memory-intensive "aws,gcp" \
  --cost-optimize "vultr,digitalocean" \
  --regions "us-east-1,us-west-2,eu-west-1"

# Configure cost limits
cargo tangle cloud configure-costs \
  --max-hourly-cost 5.00 \
  --prefer-spot true \
  --auto-terminate-after 24h

# Configure failover policies  
cargo tangle cloud configure-failover \
  --primary-provider aws \
  --secondary-provider gcp \
  --fallback-regions "us-central1,eu-west-1"
```

### 2. Blueprint Manager Uses Configuration

```rust
// Blueprint Manager reads deployment policy
let deployment_policy = RemoteDeploymentPolicy::load()?;

// When deploying a blueprint, Manager decides based on requirements
match service.resource_requirements() {
    ResourceRequirements { gpu_count: Some(_), .. } => {
        // Use GPU providers (GCP, AWS)
        let provider = policy.select_gpu_provider(&requirements)?;
        deploy_to_provider(provider, service).await?;
    }
    ResourceRequirements { cpu > 8.0, memory < 16.0, .. } => {
        // CPU-intensive, use cost-optimized providers
        let provider = policy.select_cpu_provider(&requirements)?;
        deploy_to_provider(provider, service).await?;
    }
    _ => {
        // Standard workload, use cheapest provider
        let provider = policy.select_cheapest_provider(&requirements)?;
        deploy_to_provider(provider, service).await?;
    }
}
```

### 3. Existing Deploy Command Works

```bash
# This should trigger intelligent provider selection
cargo tangle blueprint deploy tangle --remote

# Override for specific needs
cargo tangle blueprint deploy tangle --remote --provider aws --region us-west-2
```

## Implementation Plan

### Phase 1: Configuration Schema
```rust
#[derive(Serialize, Deserialize)]
pub struct RemoteDeploymentPolicy {
    /// Preferred providers by resource type
    pub providers: ProviderPreferences,
    /// Cost constraints
    pub cost_limits: CostPolicy,
    /// Geographic preferences
    pub regions: RegionPolicy,
    /// Failover configuration
    pub failover: FailoverPolicy,
}

#[derive(Serialize, Deserialize)]
pub struct ProviderPreferences {
    /// Providers to use for GPU workloads (ordered by preference)
    pub gpu_providers: Vec<CloudProvider>,
    /// Providers for CPU-intensive workloads
    pub cpu_intensive: Vec<CloudProvider>,
    /// Providers for memory-intensive workloads  
    pub memory_intensive: Vec<CloudProvider>,
    /// Providers for cost-optimized workloads
    pub cost_optimized: Vec<CloudProvider>,
}
```

### Phase 2: Smart Provider Selection
```rust
impl RemoteDeploymentPolicy {
    pub fn select_provider(&self, requirements: &ResourceSpec) -> Result<CloudProvider> {
        // If GPU required, use GPU providers
        if requirements.gpu_count.is_some() {
            return self.select_from_list(&self.providers.gpu_providers, requirements);
        }
        
        // If high CPU/memory ratio, use CPU providers
        if requirements.cpu / requirements.memory_gb > 0.5 {
            return self.select_from_list(&self.providers.cpu_intensive, requirements);
        }
        
        // Otherwise use cost-optimized
        self.select_from_list(&self.providers.cost_optimized, requirements)
    }
}
```

### Phase 3: CLI Refactor
- Remove direct deployment orchestration from CLI
- Add policy configuration commands
- Keep cost estimation and status monitoring
- Make Blueprint Manager deployment respect policies

## Benefits of Correct Architecture

1. **Blueprint Manager Orchestrates**: Single point of control
2. **Intelligent Selection**: GPU -> GCP/AWS, CPU -> Vultr/DO, etc.
3. **Policy-Driven**: Configure once, works for all deployments
4. **Cost Optimization**: Automatic provider selection based on requirements
5. **Failover Support**: Automatic retry on other providers
6. **Backward Compatible**: Existing deployments unaffected