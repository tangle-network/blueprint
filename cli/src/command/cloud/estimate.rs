//! Cost estimation for cloud deployments.
//!
//! This module provides cost estimation and comparison functionality across different
//! cloud providers, helping users make informed decisions about deployment costs.

use clap::Args;
use color_eyre::Result;

use super::CloudProvider;

#[derive(Debug, Args)]
pub struct EstimateOptions {
    /// Compare all providers
    #[arg(short = 'c', long)]
    pub compare: bool,

    /// Specific provider to estimate
    #[arg(short, long, value_enum)]
    pub provider: Option<CloudProvider>,

    /// CPU cores
    #[arg(long, default_value = "4")]
    pub cpu: f32,

    /// Memory in GB
    #[arg(long, default_value = "16")]
    pub memory: f32,

    /// Number of GPUs
    #[arg(long)]
    pub gpu: Option<u32>,

    /// Duration (e.g., 1h, 24h, 30d)
    #[arg(short = 'd', long, default_value = "24h")]
    pub duration: String,

    /// Include spot pricing
    #[arg(short, long)]
    pub spot: bool,
}

#[derive(Debug)]
struct CostEstimate {
    provider: String,
    instance_type: String,
    hourly_cost: String,
    daily_cost: String,
    monthly_cost: String,
    total_cost: String,
}

/// Estimate deployment costs across cloud providers.
///
/// Provides detailed cost breakdowns including hourly, daily, and monthly rates.
/// Can compare costs across all providers or estimate for a specific provider.
///
/// # Arguments
///
/// * `opts` - Estimation options including resources, duration, and provider selection
///
/// # Errors
///
/// Returns an error if:
/// * Invalid duration format is provided
/// * Resource specifications are invalid
///
/// # Examples
///
/// ```bash
/// # Compare all providers
/// cargo tangle cloud estimate --compare --cpu 4 --memory 16
///
/// # Estimate with spot pricing
/// cargo tangle cloud estimate --provider aws --spot --duration 30d
/// ```
pub async fn estimate(opts: EstimateOptions) -> Result<()> {
    println!("💰 Cost Estimation\n");

    // Parse duration
    let hours = parse_duration(&opts.duration)?;

    // Show configuration
    println!("Configuration:");
    println!("  CPU: {} cores", opts.cpu);
    println!("  Memory: {} GB", opts.memory);
    if let Some(gpu) = opts.gpu {
        println!("  GPU: {} units", gpu);
    }
    println!("  Duration: {} ({:.1} hours)", opts.duration, hours);
    if opts.spot {
        println!("  Instance Type: Spot/Preemptible");
    }
    println!();

    if opts.compare {
        // Compare all providers
        let providers = vec![
            CloudProvider::AWS,
            CloudProvider::GCP,
            CloudProvider::Azure,
            CloudProvider::DigitalOcean,
            CloudProvider::Vultr,
        ];

        let mut estimates = Vec::new();

        for provider in providers {
            let instance_type = get_instance_type(provider, opts.cpu, opts.memory, opts.gpu);
            let (hourly, daily, monthly, total) =
                calculate_costs(provider, opts.cpu, opts.memory, opts.gpu, opts.spot, hours);

            estimates.push(CostEstimate {
                provider: provider.to_string(),
                instance_type,
                hourly_cost: format!("${:.2}", hourly),
                daily_cost: format!("${:.2}", daily),
                monthly_cost: format!("${:.2}", monthly),
                total_cost: format!("${:.2}", total),
            });
        }

        // Sort by total cost
        estimates.sort_by(|a, b| {
            let a_val: f32 = a.total_cost.trim_start_matches('$').parse().unwrap_or(0.0);
            let b_val: f32 = b.total_cost.trim_start_matches('$').parse().unwrap_or(0.0);
            a_val.partial_cmp(&b_val).unwrap()
        });

        // Display results in formatted output
        println!(
            "{:<20} {:<20} {:<10} {:<10} {:<12} {:<12}",
            "Provider", "Instance Type", "$/hour", "$/day", "$/month", "Total"
        );
        println!("{}", "-".repeat(84));

        for est in &estimates {
            println!(
                "{:<20} {:<20} {:<10} {:<10} {:<12} {:<12}",
                est.provider,
                est.instance_type,
                est.hourly_cost,
                est.daily_cost,
                est.monthly_cost,
                est.total_cost
            );
        }

        // Highlight cheapest
        if let Some(cheapest) = estimates.first() {
            println!(
                "\n✨ Cheapest: {} at {}",
                cheapest.provider, cheapest.total_cost
            );
        }
    } else {
        // Estimate for single provider
        let provider = opts.provider.unwrap_or(CloudProvider::AWS);
        let instance_type = get_instance_type(provider, opts.cpu, opts.memory, opts.gpu);
        let (hourly, daily, monthly, total) =
            calculate_costs(provider, opts.cpu, opts.memory, opts.gpu, opts.spot, hours);

        println!("Provider: {}", provider);
        println!("Instance Type: {}", instance_type);
        println!("\nCost Breakdown:");
        println!("  Hourly:  ${:.2}", hourly);
        println!("  Daily:   ${:.2}", daily);
        println!("  Monthly: ${:.2}", monthly);
        println!("\nTotal for {}: ${:.2}", opts.duration, total);

        if opts.spot {
            let regular_total = total / 0.7;
            println!("Spot Savings: ${:.2} (30% off)", regular_total - total);
        }
    }

    // Show tips
    println!("\n💡 Cost Optimization Tips:");
    if !opts.spot {
        println!("  • Use spot instances for 30% savings (add --spot)");
    }
    println!("  • Consider lower resource tiers if workload allows");
    println!("  • Set TTL to auto-terminate unused instances");
    println!("  • Use Vultr or DigitalOcean for lower costs");

    Ok(())
}

fn parse_duration(duration_str: &str) -> Result<f32> {
    let duration = duration_str.to_lowercase();

    if let Some(hours) = duration.strip_suffix('h') {
        hours
            .parse::<f32>()
            .map_err(|_| color_eyre::eyre::eyre!("Invalid hours value"))
    } else if let Some(days) = duration.strip_suffix('d') {
        Ok(days
            .parse::<f32>()
            .map_err(|_| color_eyre::eyre::eyre!("Invalid days value"))?
            * 24.0)
    } else if let Some(weeks) = duration.strip_suffix('w') {
        Ok(weeks
            .parse::<f32>()
            .map_err(|_| color_eyre::eyre::eyre!("Invalid weeks value"))?
            * 168.0)
    } else if let Some(months) = duration.strip_suffix('m') {
        Ok(months
            .parse::<f32>()
            .map_err(|_| color_eyre::eyre::eyre!("Invalid months value"))?
            * 730.0)
    } else {
        duration
            .parse::<f32>()
            .map_err(|_| color_eyre::eyre::eyre!("Invalid duration value"))
    }
}

fn get_instance_type(provider: CloudProvider, cpu: f32, memory: f32, gpu: Option<u32>) -> String {
    if gpu.is_some() {
        match provider {
            CloudProvider::AWS => "p3.2xlarge",
            CloudProvider::GCP => "n1-standard-8-nvidia-t4",
            CloudProvider::Azure => "NC6s_v3",
            _ => "GPU Instance",
        }
        .to_string()
    } else {
        match provider {
            CloudProvider::AWS => if cpu <= 2.0 && memory <= 8.0 {
                "t3.medium"
            } else if cpu <= 4.0 && memory <= 16.0 {
                "t3.xlarge"
            } else if cpu <= 8.0 && memory <= 32.0 {
                "t3.2xlarge"
            } else {
                "c5.4xlarge"
            }
            .to_string(),
            CloudProvider::GCP => if cpu <= 2.0 && memory <= 8.0 {
                "n2-standard-2"
            } else if cpu <= 4.0 && memory <= 16.0 {
                "n2-standard-4"
            } else if cpu <= 8.0 && memory <= 32.0 {
                "n2-standard-8"
            } else {
                "n2-standard-16"
            }
            .to_string(),
            CloudProvider::Azure => if cpu <= 2.0 && memory <= 8.0 {
                "Standard_D2s_v3"
            } else if cpu <= 4.0 && memory <= 16.0 {
                "Standard_D4s_v3"
            } else if cpu <= 8.0 && memory <= 32.0 {
                "Standard_D8s_v3"
            } else {
                "Standard_D16s_v3"
            }
            .to_string(),
            CloudProvider::DigitalOcean => if cpu <= 2.0 && memory <= 4.0 {
                "s-2vcpu-4gb"
            } else if cpu <= 4.0 && memory <= 8.0 {
                "s-4vcpu-8gb"
            } else if cpu <= 8.0 && memory <= 16.0 {
                "s-8vcpu-16gb"
            } else {
                "s-16vcpu-32gb"
            }
            .to_string(),
            CloudProvider::Vultr => if cpu <= 2.0 && memory <= 4.0 {
                "vc2-2c-4gb"
            } else if cpu <= 4.0 && memory <= 8.0 {
                "vc2-4c-8gb"
            } else if cpu <= 6.0 && memory <= 16.0 {
                "vc2-6c-16gb"
            } else {
                "vc2-8c-32gb"
            }
            .to_string(),
        }
    }
}

fn calculate_costs(
    provider: CloudProvider,
    cpu: f32,
    memory: f32,
    gpu: Option<u32>,
    spot: bool,
    hours: f32,
) -> (f32, f32, f32, f32) {
    // Base costs per provider (simplified)
    let base_hourly = match provider {
        CloudProvider::AWS => 0.10 * cpu + 0.008 * memory,
        CloudProvider::GCP => 0.09 * cpu + 0.007 * memory,
        CloudProvider::Azure => 0.11 * cpu + 0.009 * memory,
        CloudProvider::DigitalOcean => 0.08 * cpu + 0.006 * memory,
        CloudProvider::Vultr => 0.07 * cpu + 0.005 * memory,
    };

    // Add GPU costs
    let gpu_hourly = if let Some(gpu_count) = gpu {
        match provider {
            CloudProvider::AWS => 3.06 * gpu_count as f32,
            CloudProvider::GCP => 2.48 * gpu_count as f32,
            CloudProvider::Azure => 2.88 * gpu_count as f32,
            _ => 2.50 * gpu_count as f32,
        }
    } else {
        0.0
    };

    let hourly = base_hourly + gpu_hourly;
    let final_hourly = if spot { hourly * 0.7 } else { hourly };

    let daily = final_hourly * 24.0;
    let monthly = final_hourly * 730.0;
    let total = final_hourly * hours;

    (final_hourly, daily, monthly, total)
}
