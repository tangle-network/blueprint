# Implementation Roadmap

## Phase 1: CLI Cloud Commands (Current)

### Week 1: Core Infrastructure
- Add `cloud` subcommand to CLI
- Provider configuration management (credentials, regions, defaults)
- Resource specification parser for Blueprint.toml
- Integration layer with blueprint-remote-providers crate

### Week 2: Deployment Commands  
- `deploy --remote` with provider selection
- Cost estimation before deployment
- Interactive resource configuration
- Deployment state persistence

## Phase 2: Build Service MVP (Weeks 3-5)
- Docker build job implementation
- Layer caching with S3/GCS
- Multi-platform build support
- Registry push integration

## Phase 3: Production Hardening (Week 6)
- Health monitoring integration
- Auto-recovery on failure
- Metrics and observability
- Log streaming from remote instances

## Phase 4: Market Launch (Weeks 7-8)
- Public documentation
- Migration tools from depot.dev
- Performance benchmarks
- Pricing calculator UI

---

# Phase 1 Implementation Details

## 1. CLI Architecture Changes

### File Structure
```
cli/src/command/
├── cloud/
│   ├── mod.rs           # Cloud subcommand root
│   ├── config.rs        # Provider configuration
│   ├── deploy.rs        # Remote deployment logic
│   ├── estimate.rs      # Cost estimation
│   ├── status.rs        # Deployment monitoring
│   └── terminate.rs     # Cleanup commands
```

## 2. Cloud Configuration Management

### `cli/src/command/cloud/config.rs`
```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudConfig {
    pub default_provider: Option<String>,
    pub providers: HashMap<String, ProviderConfig>,
    pub defaults: DeploymentDefaults,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub provider_type: CloudProvider,
    pub credentials: CredentialSource,
    pub regions: Vec<String>,
    pub default_region: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CredentialSource {
    File { path: PathBuf },
    Environment { prefix: String },
    Command { cmd: String },
}

impl CloudConfig {
    pub fn load() -> Result<Self> {
        let config_path = dirs::config_dir()
            .ok_or_else(|| anyhow!("No config directory"))?
            .join("tangle")
            .join("cloud.toml");
        
        if !config_path.exists() {
            return Ok(Self::default());
        }
        
        let content = std::fs::read_to_string(config_path)?;
        toml::from_str(&content).map_err(Into::into)
    }
    
    pub fn save(&self) -> Result<()> {
        let config_path = dirs::config_dir()
            .ok_or_else(|| anyhow!("No config directory"))?
            .join("tangle")
            .join("cloud.toml");
        
        std::fs::create_dir_all(config_path.parent().unwrap())?;
        let content = toml::to_string_pretty(self)?;
        std::fs::write(config_path, content)?;
        Ok(())
    }
}
```

### CLI Command Implementation
```rust
// cli/src/command/cloud/mod.rs
#[derive(Subcommand, Debug)]
pub enum CloudCommands {
    /// Configure cloud providers
    Configure {
        #[arg(long)]
        provider: CloudProvider,
        #[arg(long)]
        region: Option<String>,
        #[arg(long)]
        credentials_file: Option<PathBuf>,
        #[arg(long)]
        set_default: bool,
    },
    
    /// Deploy blueprint to cloud
    Deploy {
        #[arg(long)]
        provider: Option<CloudProvider>,
        #[arg(long)]
        region: Option<String>,
        #[arg(long)]
        cpu: Option<f32>,
        #[arg(long)]
        memory: Option<f32>,
        #[arg(long)]
        gpu: Option<u32>,
        #[arg(long)]
        ttl: Option<String>,
        #[arg(long)]
        spot: bool,
        #[arg(long)]
        auto_select: bool,
    },
    
    /// Estimate deployment costs
    Estimate {
        #[arg(long)]
        provider: Option<CloudProvider>,
        #[arg(long)]
        cpu: f32,
        #[arg(long)]
        memory: f32,
        #[arg(long)]
        duration: String,
        #[arg(long)]
        compare: bool,
    },
    
    /// Show deployment status
    Status {
        #[arg(long)]
        deployment_id: Option<String>,
        #[arg(long)]
        watch: bool,
        #[arg(long)]
        format: OutputFormat,
    },
    
    /// Terminate deployments
    Terminate {
        #[arg(long)]
        deployment_id: Option<String>,
        #[arg(long)]
        blueprint_id: Option<u64>,
        #[arg(long)]
        all: bool,
        #[arg(long)]
        expired: bool,
    },
}
```

## 3. Resource Specification in Blueprint.toml

### Parser Implementation
```rust
// cli/src/command/cloud/resources.rs
use blueprint_remote_providers::ResourceSpec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BlueprintManifest {
    pub package: PackageSection,
    pub blueprint: BlueprintSection,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlueprintSection {
    pub protocol: String,
    #[serde(default)]
    pub resources: ResourceRequirements,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ResourceRequirements {
    pub min_cpu: Option<f32>,
    pub min_memory_gb: Option<f32>,
    pub min_storage_gb: Option<f32>,
    pub recommended_cpu: Option<f32>,
    pub recommended_memory_gb: Option<f32>,
    pub recommended_storage_gb: Option<f32>,
    pub gpu_count: Option<u32>,
    pub allow_spot: Option<bool>,
}

impl ResourceRequirements {
    pub fn to_resource_spec(&self, use_recommended: bool) -> ResourceSpec {
        if use_recommended && self.recommended_cpu.is_some() {
            ResourceSpec {
                cpu: self.recommended_cpu.unwrap_or(4.0),
                memory_gb: self.recommended_memory_gb.unwrap_or(16.0),
                storage_gb: self.recommended_storage_gb.unwrap_or(100.0),
                gpu_count: self.gpu_count,
                allow_spot: self.allow_spot.unwrap_or(false),
            }
        } else {
            ResourceSpec {
                cpu: self.min_cpu.unwrap_or(2.0),
                memory_gb: self.min_memory_gb.unwrap_or(4.0),
                storage_gb: self.min_storage_gb.unwrap_or(20.0),
                gpu_count: self.gpu_count,
                allow_spot: self.allow_spot.unwrap_or(true),
            }
        }
    }
}

pub fn read_blueprint_manifest(path: &Path) -> Result<BlueprintManifest> {
    let manifest_path = path.join("Blueprint.toml");
    if !manifest_path.exists() {
        // Fall back to Cargo.toml for compatibility
        return read_cargo_manifest(path);
    }
    
    let content = std::fs::read_to_string(manifest_path)?;
    toml::from_str(&content).map_err(Into::into)
}
```

## 4. Deploy Command Implementation

### `cli/src/command/cloud/deploy.rs`
```rust
use blueprint_remote_providers::{
    UnifiedInfrastructureProvisioner,
    RemoteDeploymentExtensions,
    ResourceSpec,
    CloudProvider,
    PricingService,
};

pub async fn deploy_remote(
    opts: DeployOptions,
    manifest_path: PathBuf,
) -> Result<DeploymentResult> {
    // 1. Load blueprint manifest
    let manifest = read_blueprint_manifest(&manifest_path)?;
    let resources = manifest.blueprint.resources;
    
    // 2. Determine resource spec
    let spec = if let (Some(cpu), Some(mem)) = (opts.cpu, opts.memory) {
        ResourceSpec {
            cpu,
            memory_gb: mem,
            storage_gb: opts.storage.unwrap_or(20.0),
            gpu_count: opts.gpu,
            allow_spot: opts.spot,
        }
    } else {
        resources.to_resource_spec(!opts.spot)
    };
    
    // 3. Select provider
    let provider = if opts.auto_select {
        select_cheapest_provider(&spec).await?
    } else if let Some(p) = opts.provider {
        p
    } else {
        prompt_provider_selection()?
    };
    
    // 4. Estimate costs
    let pricing = PricingService::new();
    let ttl_hours = parse_duration(&opts.ttl.unwrap_or_else(|| "24h".to_string()))?;
    let cost = pricing.calculate_cost(&spec, provider, ttl_hours);
    
    println!("Estimated cost: ${:.2}/hr (${:.2} total)", 
        cost.final_hourly_cost, cost.total_cost);
    
    if !opts.yes && !confirm_deployment()? {
        return Ok(DeploymentResult::Cancelled);
    }
    
    // 5. Provision infrastructure
    let provisioner = UnifiedInfrastructureProvisioner::new().await?;
    let region = opts.region.unwrap_or_else(|| default_region(&provider));
    
    let instance = provisioner.provision(provider, &spec, &region).await?;
    
    // 6. Deploy blueprint
    deploy_blueprint_to_instance(&instance, &manifest).await?;
    
    // 7. Register with tracker
    let tracker = get_deployment_tracker().await?;
    let record = DeploymentRecord::new(
        manifest.blueprint.id,
        instance.id.clone(),
        spec,
        Some(ttl_hours as u64 * 3600),
    );
    
    tracker.register_deployment(instance.id.clone(), record).await?;
    
    Ok(DeploymentResult::Success {
        deployment_id: instance.id,
        public_ip: instance.public_ip,
        dashboard_url: format!("https://tangle.tools/deployments/{}", instance.id),
    })
}

async fn select_cheapest_provider(spec: &ResourceSpec) -> Result<CloudProvider> {
    let pricing = PricingService::new();
    let (provider, _report) = pricing.find_cheapest_provider(spec, 1.0);
    Ok(provider)
}

fn prompt_provider_selection() -> Result<CloudProvider> {
    use dialoguer::Select;
    
    let providers = vec![
        "AWS EC2",
        "GCP Compute Engine", 
        "Azure VM",
        "DigitalOcean Droplet",
        "Vultr Instance",
    ];
    
    let selection = Select::new()
        .with_prompt("Select deployment target")
        .items(&providers)
        .default(0)
        .interact()?;
    
    Ok(match selection {
        0 => CloudProvider::AWS,
        1 => CloudProvider::GCP,
        2 => CloudProvider::Azure,
        3 => CloudProvider::DigitalOcean,
        4 => CloudProvider::Vultr,
        _ => unreachable!(),
    })
}
```

## 5. Interactive Flow Enhancement

### `cli/src/command/cloud/interactive.rs`
```rust
use dialoguer::{Confirm, Input, Select};
use indicatif::{ProgressBar, ProgressStyle};

pub async fn interactive_deploy(manifest_path: PathBuf) -> Result<()> {
    println!("🚀 Blueprint Remote Deployment\n");
    
    // 1. Provider selection
    let provider = prompt_provider_selection()?;
    
    // 2. Region selection
    let regions = get_available_regions(provider).await?;
    let region = Select::new()
        .with_prompt("Select region")
        .items(&regions)
        .interact()?;
    
    // 3. Resource configuration
    let cpu: f32 = Input::new()
        .with_prompt("CPU cores")
        .default(4.0)
        .interact()?;
    
    let memory: f32 = Input::new()
        .with_prompt("Memory (GB)")
        .default(16.0)
        .interact()?;
    
    let gpu: u32 = Input::new()
        .with_prompt("GPU count (0 for none)")
        .default(0)
        .interact()?;
    
    // 4. Cost optimization
    let use_spot = Confirm::new()
        .with_prompt("Use spot instances? (30% discount)")
        .default(true)
        .interact()?;
    
    // 5. TTL configuration
    let ttl: String = Input::new()
        .with_prompt("Auto-terminate after")
        .default("24h".to_string())
        .interact()?;
    
    // 6. Show cost estimate
    let spec = ResourceSpec {
        cpu,
        memory_gb: memory,
        storage_gb: 100.0,
        gpu_count: if gpu > 0 { Some(gpu) } else { None },
        allow_spot: use_spot,
    };
    
    let pricing = PricingService::new();
    let cost = pricing.calculate_cost(&spec, provider, parse_duration(&ttl)?);
    
    println!("\n📊 Deployment Summary:");
    println!("  Provider: {:?}", provider);
    println!("  Region: {}", regions[region]);
    println!("  Resources: {} CPU, {} GB RAM", cpu, memory);
    if gpu > 0 {
        println!("  GPU: {} units", gpu);
    }
    println!("  Spot Instance: {}", if use_spot { "Yes" } else { "No" });
    println!("  TTL: {}", ttl);
    println!("\n💰 Estimated Cost:");
    println!("  ${:.2}/hour", cost.final_hourly_cost);
    println!("  ${:.2} total for {}", cost.total_cost, ttl);
    
    if !Confirm::new()
        .with_prompt("\nProceed with deployment?")
        .default(true)
        .interact()? {
        println!("Deployment cancelled.");
        return Ok(());
    }
    
    // 7. Deploy with progress
    let pb = ProgressBar::new(5);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{bar:40}] {pos}/{len} {msg}")
            .unwrap()
    );
    
    pb.set_message("Provisioning infrastructure...");
    let instance = provision_with_progress(provider, &spec, &regions[region], &pb).await?;
    
    pb.inc(1);
    pb.set_message("Installing dependencies...");
    tokio::time::sleep(Duration::from_secs(2)).await;
    
    pb.inc(1);
    pb.set_message("Deploying blueprint...");
    deploy_blueprint_to_instance(&instance, &manifest_path).await?;
    
    pb.inc(1);
    pb.set_message("Configuring networking...");
    tokio::time::sleep(Duration::from_secs(1)).await;
    
    pb.inc(1);
    pb.set_message("Starting health monitoring...");
    tokio::time::sleep(Duration::from_secs(1)).await;
    
    pb.finish_with_message("✅ Deployment complete!");
    
    println!("\n🎉 Deployment Successful!");
    println!("  Instance ID: {}", instance.id);
    if let Some(ip) = instance.public_ip {
        println!("  Public IP: {}", ip);
    }
    println!("  Dashboard: https://tangle.tools/deployments/{}", instance.id);
    
    Ok(())
}
```

## 6. Integration Points

### Main CLI Entry
```rust
// cli/src/main.rs
Commands::Cloud { command } => {
    match command {
        CloudCommands::Configure { .. } => cloud::configure(opts).await?,
        CloudCommands::Deploy { .. } => cloud::deploy(opts).await?,
        CloudCommands::Estimate { .. } => cloud::estimate(opts).await?,
        CloudCommands::Status { .. } => cloud::status(opts).await?,
        CloudCommands::Terminate { .. } => cloud::terminate(opts).await?,
    }
}
```

## Next Steps
1. Add cloud subcommand to Cargo.toml features
2. Implement provider-specific authentication
3. Add deployment state persistence
4. Create integration tests
5. Update documentation