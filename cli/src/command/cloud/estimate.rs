//! Deployment estimates from the same configured SDK providers used for allocation.

use blueprint_remote_providers::{CloudProvisioner, ResourceSpec};
use clap::Args;
use color_eyre::{Result, eyre::eyre};

use super::{CloudProvider, config::CloudConfig};

#[derive(Debug, Args)]
pub struct EstimateOptions {
    /// Compare configured providers
    #[arg(short = 'c', long)]
    pub compare: bool,
    #[arg(short, long)]
    pub provider: Option<CloudProvider>,
    #[arg(long, default_value = "4")]
    pub cpu: f32,
    #[arg(long, default_value = "16")]
    pub memory: f32,
    #[arg(long)]
    pub gpu: Option<u32>,
    /// Canonical GPU class from `cloud catalog --json`
    #[arg(long)]
    pub gpu_class: Option<String>,
    /// Minimum VRAM in GB for each GPU
    #[arg(long)]
    pub gpu_vram: Option<u32>,
    #[arg(short = 'd', long, default_value = "24h")]
    pub duration: String,
    #[arg(short, long)]
    pub spot: bool,
}

pub async fn estimate(opts: EstimateOptions) -> Result<()> {
    let hours = parse_duration(&opts.duration)?;
    let config = CloudConfig::load()?;
    let spec = ResourceSpec {
        provider: opts
            .provider
            .or_else(|| (!opts.compare).then_some(config.default_provider).flatten()),
        cpu: opts.cpu,
        memory_gb: opts.memory,
        gpu_count: opts.gpu,
        gpu_class: opts.gpu_class,
        gpu_vram_gb: opts.gpu_vram,
        allow_spot: opts.spot,
        ..ResourceSpec::default()
    };
    let mut provisioner = CloudProvisioner::new().await?;
    for (provider, settings) in config.providers {
        provisioner = provisioner.with_region(provider, settings.region);
    }
    let quotes = provisioner.quotes(&spec).await?;
    if quotes.is_empty() {
        return Err(eyre!(
            "No configured provider has a known price for these resources"
        ));
    }
    println!(
        "{:<18} {:<32} {:>12} {:>14}",
        "Provider", "SKU", "USD/hour", "Total USD"
    );
    for quote in quotes {
        let price = quote
            .selection
            .estimated_hourly_cost
            .expect("SDK quotes require known prices");
        println!(
            "{:<18} {:<32} {:>12.6} {:>14.6}",
            quote.provider,
            quote.selection.instance_type,
            price,
            price * hours
        );
    }
    Ok(())
}

fn parse_duration(value: &str) -> Result<f64> {
    let value = value.to_ascii_lowercase();
    let (number, multiplier) = match value.chars().last() {
        Some('h') => (&value[..value.len() - 1], 1.0),
        Some('d') => (&value[..value.len() - 1], 24.0),
        Some('w') => (&value[..value.len() - 1], 168.0),
        Some('m') => (&value[..value.len() - 1], 730.0),
        _ => (value.as_str(), 1.0),
    };
    let hours = number.parse::<f64>()? * multiplier;
    if !hours.is_finite() || hours <= 0.0 {
        return Err(eyre!("Duration must be finite and positive"));
    }
    Ok(hours)
}
