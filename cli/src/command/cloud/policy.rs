//! Remote deployment policy configuration.
//!
//! This module handles configuration of Blueprint Manager's remote deployment policies,
//! allowing users to specify provider preferences, cost limits, and deployment strategies.

use super::CloudProvider;
use clap::{Args, ValueEnum};
use color_eyre::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Remote deployment policy configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteDeploymentPolicy {
    /// Provider preferences by workload type
    pub providers: ProviderPreferences,
    /// Cost constraints and optimization settings
    pub cost_limits: CostPolicy,
    /// Geographic deployment preferences
    pub regions: RegionPolicy,
    /// Failover and retry configuration
    pub failover: FailoverPolicy,
}

/// Provider preferences for different workload types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderPreferences {
    /// Providers to prefer for GPU workloads (ordered by preference)
    pub gpu_providers: Vec<CloudProvider>,
    /// Providers for CPU-intensive workloads
    pub cpu_intensive: Vec<CloudProvider>,
    /// Providers for memory-intensive workloads  
    pub memory_intensive: Vec<CloudProvider>,
    /// Providers for cost-optimized workloads
    pub cost_optimized: Vec<CloudProvider>,
}

/// Cost policy and limits.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostPolicy {
    /// Maximum hourly cost per deployment (USD)
    pub max_hourly_cost: Option<f32>,
    /// Prefer spot/preemptible instances when possible
    pub prefer_spot: bool,
    /// Auto-terminate deployments after this duration
    pub auto_terminate_after_hours: Option<u32>,
    /// Cost optimization strategy
    pub optimization_strategy: CostOptimization,
}

/// Cost optimization strategies.
#[derive(Debug, Clone, Serialize, Deserialize, ValueEnum)]
pub enum CostOptimization {
    /// Minimize cost above all else
    Cheapest,
    /// Balance cost and performance
    Balanced,
    /// Prioritize performance over cost
    Performance,
}

/// Regional deployment preferences.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionPolicy {
    /// Preferred regions (ordered by preference)
    pub preferred_regions: Vec<String>,
    /// Allow deployments outside preferred regions if needed
    pub allow_fallback_regions: bool,
    /// Latency requirements (milliseconds)
    pub max_latency_ms: Option<u32>,
}

/// Failover and retry configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailoverPolicy {
    /// Maximum number of provider retry attempts
    pub max_retries: u32,
    /// Retry delay between attempts (seconds)
    pub retry_delay_seconds: u32,
    /// Whether to automatically retry on different regions
    pub retry_different_regions: bool,
    /// Whether to automatically retry on different providers
    pub retry_different_providers: bool,
}

impl Default for RemoteDeploymentPolicy {
    fn default() -> Self {
        Self {
            providers: ProviderPreferences {
                gpu_providers: vec![CloudProvider::GCP, CloudProvider::AWS],
                cpu_intensive: vec![
                    CloudProvider::Vultr,
                    CloudProvider::DigitalOcean,
                    CloudProvider::AWS,
                ],
                memory_intensive: vec![CloudProvider::AWS, CloudProvider::GCP],
                cost_optimized: vec![CloudProvider::Vultr, CloudProvider::DigitalOcean],
            },
            cost_limits: CostPolicy {
                max_hourly_cost: Some(5.0),
                prefer_spot: true,
                auto_terminate_after_hours: Some(24),
                optimization_strategy: CostOptimization::Balanced,
            },
            regions: RegionPolicy {
                preferred_regions: vec!["us-east-1".to_string(), "us-west-2".to_string()],
                allow_fallback_regions: true,
                max_latency_ms: Some(100),
            },
            failover: FailoverPolicy {
                max_retries: 3,
                retry_delay_seconds: 30,
                retry_different_regions: true,
                retry_different_providers: true,
            },
        }
    }
}

impl RemoteDeploymentPolicy {
    /// Load policy from disk or create default.
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;

        if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            serde_json::from_str(&content)
                .or_else(|_| toml::from_str(&content))
                .map_err(|e| color_eyre::eyre::eyre!("Failed to parse deployment policy: {}", e))
        } else {
            Ok(Self::default())
        }
    }

    /// Save policy to disk.
    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&path, content)?;

        Ok(())
    }

    fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| color_eyre::eyre::eyre!("Could not find config directory"))?;
        Ok(config_dir.join("tangle").join("deployment-policy.json"))
    }
}

#[derive(Debug, Args)]
pub struct PolicyConfigureArgs {
    /// GPU providers (comma-separated, ordered by preference)
    #[arg(long)]
    pub gpu_providers: Option<String>,

    /// CPU-intensive workload providers
    #[arg(long)]
    pub cpu_providers: Option<String>,

    /// Memory-intensive workload providers
    #[arg(long)]
    pub memory_providers: Option<String>,

    /// Cost-optimized providers
    #[arg(long)]
    pub cost_providers: Option<String>,

    /// Maximum hourly cost limit (USD)
    #[arg(long)]
    pub max_cost: Option<f32>,

    /// Prefer spot instances
    #[arg(long)]
    pub prefer_spot: Option<bool>,

    /// Auto-terminate after hours
    #[arg(long)]
    pub auto_terminate: Option<u32>,

    /// Preferred regions (comma-separated)
    #[arg(long)]
    pub regions: Option<String>,

    /// Cost optimization strategy
    #[arg(long, value_enum)]
    pub cost_strategy: Option<CostOptimization>,
}

/// Configure remote deployment policy.
pub async fn configure_policy(args: PolicyConfigureArgs) -> Result<()> {
    println!("🔧 Configuring Remote Deployment Policy\n");

    let mut policy = RemoteDeploymentPolicy::load()?;
    let mut changed = false;

    // Update provider preferences
    if let Some(providers) = args.gpu_providers {
        policy.providers.gpu_providers = parse_providers(&providers)?;
        println!("✓ GPU providers: {:?}", policy.providers.gpu_providers);
        changed = true;
    }

    if let Some(providers) = args.cpu_providers {
        policy.providers.cpu_intensive = parse_providers(&providers)?;
        println!("✓ CPU providers: {:?}", policy.providers.cpu_intensive);
        changed = true;
    }

    if let Some(providers) = args.memory_providers {
        policy.providers.memory_intensive = parse_providers(&providers)?;
        println!(
            "✓ Memory providers: {:?}",
            policy.providers.memory_intensive
        );
        changed = true;
    }

    if let Some(providers) = args.cost_providers {
        policy.providers.cost_optimized = parse_providers(&providers)?;
        println!("✓ Cost providers: {:?}", policy.providers.cost_optimized);
        changed = true;
    }

    // Update cost limits
    if let Some(max_cost) = args.max_cost {
        policy.cost_limits.max_hourly_cost = Some(max_cost);
        println!("✓ Max hourly cost: ${:.2}", max_cost);
        changed = true;
    }

    if let Some(prefer_spot) = args.prefer_spot {
        policy.cost_limits.prefer_spot = prefer_spot;
        println!("✓ Prefer spot instances: {}", prefer_spot);
        changed = true;
    }

    if let Some(auto_terminate) = args.auto_terminate {
        policy.cost_limits.auto_terminate_after_hours = Some(auto_terminate);
        println!("✓ Auto-terminate after: {}h", auto_terminate);
        changed = true;
    }

    // Update regions
    if let Some(regions) = args.regions {
        policy.regions.preferred_regions =
            regions.split(',').map(|s| s.trim().to_string()).collect();
        println!(
            "✓ Preferred regions: {:?}",
            policy.regions.preferred_regions
        );
        changed = true;
    }

    // Update cost strategy
    if let Some(strategy) = args.cost_strategy {
        println!("✓ Cost strategy: {:?}", strategy);
        policy.cost_limits.optimization_strategy = strategy;
        changed = true;
    }

    if changed {
        policy.save()?;
        println!("\n✅ Deployment policy updated!");
        println!("   Blueprint Manager will use these settings for remote deployments.");
    } else {
        println!("No changes specified. Current policy:");
        show_current_policy(&policy).await?;
    }

    Ok(())
}

/// Show current deployment policy.
pub async fn show_policy() -> Result<()> {
    println!("📋 Current Remote Deployment Policy\n");

    let policy = RemoteDeploymentPolicy::load()?;
    show_current_policy(&policy).await
}

async fn show_current_policy(policy: &RemoteDeploymentPolicy) -> Result<()> {
    println!("Provider Preferences:");
    println!("  GPU workloads:      {:?}", policy.providers.gpu_providers);
    println!("  CPU intensive:      {:?}", policy.providers.cpu_intensive);
    println!(
        "  Memory intensive:   {:?}",
        policy.providers.memory_intensive
    );
    println!(
        "  Cost optimized:     {:?}",
        policy.providers.cost_optimized
    );

    println!("\nCost Limits:");
    if let Some(max_cost) = policy.cost_limits.max_hourly_cost {
        println!("  Max hourly cost:    ${:.2}", max_cost);
    } else {
        println!("  Max hourly cost:    No limit");
    }
    println!("  Prefer spot:        {}", policy.cost_limits.prefer_spot);
    if let Some(ttl) = policy.cost_limits.auto_terminate_after_hours {
        println!("  Auto-terminate:     {}h", ttl);
    }
    println!(
        "  Strategy:           {:?}",
        policy.cost_limits.optimization_strategy
    );

    println!("\nRegional Preferences:");
    println!(
        "  Preferred regions:  {:?}",
        policy.regions.preferred_regions
    );
    println!(
        "  Allow fallback:     {}",
        policy.regions.allow_fallback_regions
    );

    println!("\nFailover Settings:");
    println!("  Max retries:        {}", policy.failover.max_retries);
    println!(
        "  Retry delay:        {}s",
        policy.failover.retry_delay_seconds
    );

    Ok(())
}

fn parse_providers(input: &str) -> Result<Vec<CloudProvider>> {
    input
        .split(',')
        .map(|s| {
            let trimmed = s.trim().to_lowercase();
            match trimmed.as_str() {
                "aws" => Ok(CloudProvider::AWS),
                "gcp" | "google" => Ok(CloudProvider::GCP),
                "azure" => Ok(CloudProvider::Azure),
                "digitalocean" | "do" => Ok(CloudProvider::DigitalOcean),
                "vultr" => Ok(CloudProvider::Vultr),
                _ => Err(color_eyre::eyre::eyre!("Unknown provider: {}", trimmed)),
            }
        })
        .collect()
}
