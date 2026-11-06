//! Live provider estimates for the selected native VM SKU.
use super::CloudProvider;
use clap::Args;
use color_eyre::{
    Result,
    eyre::{ensure, eyre},
};

#[derive(Debug, Args)]
pub struct EstimateOptions {
    /// Compare all providers
    #[arg(short = 'c', long)]
    pub compare: bool,

    /// Specific provider to estimate
    #[arg(short, long)]
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

pub async fn estimate(opts: EstimateOptions) -> Result<()> {
    let hours = parse_duration(&opts.duration)?;
    ensure!(
        opts.cpu.is_finite() && opts.cpu > 0.0 && opts.memory.is_finite() && opts.memory > 0.0,
        "Resource requirements must be finite and positive"
    );
    ensure!(
        !opts.spot,
        "No live spot quote is available for this estimate"
    );
    #[cfg(feature = "remote-providers")]
    {
        estimate_live(opts, hours).await
    }
    #[cfg(not(feature = "remote-providers"))]
    {
        let _ = (opts, hours);
        Err(eyre!("Enable remote-providers for live VM pricing"))
    }
}

#[cfg(feature = "remote-providers")]
async fn estimate_live(opts: EstimateOptions, hours: f64) -> Result<()> {
    use super::config::CloudConfig;
    use blueprint_remote_providers::{
        ResourceSpec,
        infra::mapper::InstanceTypeMapper,
        pricing::{PricingError, PricingFetcher},
    };
    let config = CloudConfig::load()?;
    let requested = opts
        .provider
        .or(config.default_provider)
        .unwrap_or(CloudProvider::AWS);
    let providers: Vec<_> = if opts.compare {
        config.providers.keys().cloned().collect()
    } else {
        vec![requested]
    };
    let pricing = PricingFetcher::from_env().await?;
    let spec = ResourceSpec {
        cpu: opts.cpu,
        memory_gb: opts.memory,
        gpu_count: opts.gpu,
        ..ResourceSpec::recommended()
    };
    spec.validate().map_err(|error| eyre!(error))?;
    let mut estimates = Vec::new();
    for provider in providers {
        let settings = config
            .providers
            .get(&provider)
            .ok_or_else(|| eyre!("Configure a provider region before requesting its live price"))?;
        let native = provider.clone();
        let selection = match InstanceTypeMapper::map_to_instance_type(&spec, &native) {
            Ok(selection) => selection,
            Err(error) if !opts.compare => return Err(error.into()),
            Err(error) => {
                eprintln!("{provider}: {error}");
                continue;
            }
        };
        match pricing
            .quote_instance(native, &settings.region, &selection.instance_type)
            .await
        {
            Ok(Some(price)) => estimates.push((provider, selection.instance_type, price)),
            Err(PricingError::AuthRejected) => {
                return Err(eyre!("Provider rejected its pricing credential"));
            }
            Err(error) if !opts.compare => return Err(error.into()),
            Ok(None) if !opts.compare => {
                return Err(eyre!("No live price is available for this SKU and region"));
            }
            Ok(None) => {}
            Err(error) => eprintln!("{provider}: {error}"),
        }
    }
    ensure!(
        !estimates.is_empty(),
        "No configured provider has a known live price"
    );
    estimates.sort_by(|left, right| left.2.total_cmp(&right.2));
    if opts.compare {
        println!(
            "{:<20} {:<25} {:<12} Total",
            "Provider", "Instance Type", "USD/hour"
        );
        for (provider, sku, hourly) in estimates {
            println!(
                "{provider:<20} {sku:<25} {hourly:<12.4} ${:.4}",
                hourly * hours
            );
        }
    } else {
        let (provider, sku, hourly) = estimates.remove(0);
        println!("Provider: {provider}\nInstance Type: {sku}\nHourly: ${hourly:.4}");
        println!("Total for {}: ${:.4}", opts.duration, hourly * hours);
    }
    Ok(())
}

fn parse_duration(value: &str) -> Result<f64> {
    let value = value.to_ascii_lowercase();
    let (number, multiplier) = match value.chars().last() {
        Some('h') => (&value[..value.len() - 1], 1.0),
        Some('d') => (&value[..value.len() - 1], 24.0),
        Some('w') => (&value[..value.len() - 1], 168.0),
        Some('m') => (&value[..value.len() - 1], 720.0),
        _ => (value.as_str(), 1.0),
    };
    let hours = number
        .parse::<f64>()
        .map_err(|_| eyre!("Invalid duration"))?
        * multiplier;
    ensure!(
        hours.is_finite() && hours > 0.0,
        "Duration must be finite and positive"
    );
    Ok(hours)
}
