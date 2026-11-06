//! Cloud provider configuration management.
//!
//! This module handles the configuration and authentication setup for various cloud providers.
//! It provides interactive setup flows, credential management, and persistent configuration storage.

use blueprint_remote_providers::security::auth;
use clap::ValueEnum;
use color_eyre::{Result, eyre::Context};
use dialoguer::{Select, theme::ColorfulTheme};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Supported cloud providers for Blueprint deployment.
#[derive(Debug, Clone, Copy, ValueEnum, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum CloudProvider {
    #[value(name = "aws")]
    AWS,
    #[value(name = "gcp")]
    GCP,
    #[value(name = "azure")]
    Azure,
    #[value(name = "digitalocean", alias = "do")]
    DigitalOcean,
    #[value(name = "vultr")]
    Vultr,
    #[value(name = "hetzner")]
    Hetzner,
    #[value(name = "runpod")]
    RunPod,
    #[value(name = "lambda", alias = "lambda-labs")]
    LambdaLabs,
    #[value(name = "prime-intellect", alias = "pi")]
    PrimeIntellect,
    #[value(name = "vast", alias = "vast-ai")]
    VastAi,
    #[value(name = "crusoe")]
    Crusoe,
}

impl std::fmt::Display for CloudProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AWS => write!(f, "AWS"),
            Self::GCP => write!(f, "Google Cloud"),
            Self::Azure => write!(f, "Azure"),
            Self::DigitalOcean => write!(f, "DigitalOcean"),
            Self::Vultr => write!(f, "Vultr"),
            Self::Hetzner => write!(f, "Hetzner"),
            Self::RunPod => write!(f, "RunPod"),
            Self::LambdaLabs => write!(f, "Lambda Labs"),
            Self::PrimeIntellect => write!(f, "Prime Intellect"),
            Self::VastAi => write!(f, "Vast.ai"),
            Self::Crusoe => write!(f, "Crusoe"),
        }
    }
}

/// Cloud configuration storage.
///
/// Persisted to ~/.config/tangle/cloud.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudConfig {
    /// The default provider to use when none is specified
    pub default_provider: Option<CloudProvider>,
    /// Per-provider configuration settings
    pub providers: HashMap<CloudProvider, ProviderSettings>,
}

/// Provider-specific configuration settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderSettings {
    /// Default region for deployments
    pub region: String,
    /// GCP project ID (only used for Google Cloud)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}

impl CloudConfig {
    /// Load config from disk or create default
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;

        if path.exists() {
            let content = std::fs::read_to_string(&path).context("Failed to read cloud config")?;
            // Try to parse as JSON first, fall back to TOML for backwards compatibility
            serde_json::from_str(&content)
                .or_else(|_| toml::from_str(&content))
                .context("Failed to parse cloud config")
        } else {
            Ok(Self::default())
        }
    }

    /// Save config to disk
    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).context("Failed to create config directory")?;
        }

        // For now, serialize to JSON (since toml serialization is not straightforward in v0.9)
        let content = serde_json::to_string_pretty(self).context("Failed to serialize config")?;

        std::fs::write(&path, content).context("Failed to write cloud config")?;

        Ok(())
    }

    fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| color_eyre::eyre::eyre!("Could not find config directory"))?;
        Ok(config_dir.join("tangle").join("cloud.json"))
    }
}

impl Default for CloudConfig {
    fn default() -> Self {
        Self {
            default_provider: None,
            providers: HashMap::new(),
        }
    }
}

/// Record a provider's region and default status, and report its credentials.
///
/// Provider credentials come from the environment, and for AWS also its shared credentials
/// file. This command names what is missing and never stores a credential.
///
/// # Arguments
///
/// * `provider` - The cloud provider to configure
/// * `region` - Optional region override (otherwise prompts user)
/// * `set_default` - Whether to set this as the default provider
///
/// # Errors
///
/// Returns an error if:
/// * Configuration directory cannot be created
/// * A required prompt has no terminal to read from
///
/// # Examples
///
/// ```no_run
/// # use cargo_tangle::command::cloud::{configure, CloudProvider};
/// # async fn example() -> color_eyre::Result<()> {
/// // Configure AWS as default provider
/// configure(CloudProvider::AWS, Some("us-east-1".to_string()), true).await?;
/// # Ok(())
/// # }
/// ```
pub async fn configure(
    provider: CloudProvider,
    region: Option<String>,
    set_default: bool,
) -> Result<()> {
    println!("🔧 Configuring {}...\n", provider);

    let mut config = CloudConfig::load()?;

    // Get or prompt for region
    let region = if let Some(r) = region {
        r
    } else {
        prompt_region(provider)?
    };

    if credentials_present(provider).await {
        println!("✓ Found {provider} credentials");
    } else {
        println!(
            "⚠️  {provider} credentials not found. Export {} before running cloud commands; \
             cargo-tangle reads them from the environment and does not store them.",
            credential_variables(provider).join(", ")
        );
    }

    let settings = ProviderSettings {
        region,
        project_id: std::env::var("GCP_PROJECT_ID")
            .ok()
            .filter(|_| provider == CloudProvider::GCP),
    };
    config.providers.insert(provider, settings);

    if set_default || config.default_provider.is_none() {
        config.default_provider = Some(provider);
    }

    config.save()?;

    println!("\n✅ {} region saved", provider);
    if config.default_provider == Some(provider) {
        println!("   Set as default provider");
    }

    Ok(())
}

/// Environment variables that authenticate each provider.
fn credential_variables(provider: CloudProvider) -> &'static [&'static str] {
    match provider {
        CloudProvider::AWS => &["AWS_ACCESS_KEY_ID", "AWS_SECRET_ACCESS_KEY"],
        CloudProvider::GCP => &["GCP_PROJECT_ID", "GCP_ACCESS_TOKEN"],
        CloudProvider::Azure => &["AZURE_SUBSCRIPTION_ID", "AZURE_ACCESS_TOKEN"],
        CloudProvider::DigitalOcean => &["DIGITALOCEAN_TOKEN"],
        CloudProvider::Vultr => &["VULTR_API_KEY"],
        CloudProvider::Hetzner => &["HETZNER_API_TOKEN"],
        CloudProvider::RunPod => &["RUNPOD_API_KEY"],
        CloudProvider::LambdaLabs => &["LAMBDA_LABS_API_KEY"],
        CloudProvider::PrimeIntellect => &["PRIME_INTELLECT_API_KEY"],
        CloudProvider::VastAi => &["VAST_AI_API_KEY"],
        CloudProvider::Crusoe => &["CRUSOE_API_KEY", "CRUSOE_API_SECRET", "CRUSOE_PROJECT_ID"],
    }
}

/// Whether a provider's credentials are available to this process.
///
/// GCP and Azure ask the token lookup their adapters use, because a metadata server or
/// `az login` can stand in for an exported token.
pub(super) async fn credentials_present(provider: CloudProvider) -> bool {
    let exported = |names: &[&str]| names.iter().all(|name| std::env::var_os(name).is_some());
    match provider {
        CloudProvider::AWS => {
            exported(credential_variables(provider))
                || dirs::home_dir().is_some_and(|home| home.join(".aws/credentials").exists())
        }
        CloudProvider::GCP => {
            exported(&["GCP_PROJECT_ID"]) && auth::gcp_access_token().await.is_ok()
        }
        CloudProvider::Azure => {
            exported(&["AZURE_SUBSCRIPTION_ID"]) && auth::azure_access_token().await.is_ok()
        }
        _ => exported(credential_variables(provider)),
    }
}

/// Prompt for region selection
fn prompt_region(provider: CloudProvider) -> Result<String> {
    let regions = match provider {
        CloudProvider::AWS => vec![
            ("us-east-1", "US East (N. Virginia)"),
            ("us-west-2", "US West (Oregon)"),
            ("eu-west-1", "Europe (Ireland)"),
            ("ap-northeast-1", "Asia Pacific (Tokyo)"),
        ],
        CloudProvider::GCP => vec![
            ("us-central1", "US Central (Iowa)"),
            ("us-west1", "US West (Oregon)"),
            ("europe-west1", "Europe (Belgium)"),
            ("asia-northeast1", "Asia (Tokyo)"),
        ],
        CloudProvider::Azure => vec![
            ("eastus", "East US"),
            ("westus2", "West US 2"),
            ("northeurope", "North Europe"),
            ("japaneast", "Japan East"),
        ],
        CloudProvider::DigitalOcean => vec![
            ("nyc3", "New York 3"),
            ("sfo3", "San Francisco 3"),
            ("ams3", "Amsterdam 3"),
            ("sgp1", "Singapore 1"),
        ],
        CloudProvider::Vultr => vec![
            ("ewr", "New Jersey"),
            ("lax", "Los Angeles"),
            ("ams", "Amsterdam"),
            ("nrt", "Tokyo"),
        ],
        CloudProvider::Hetzner => vec![
            ("fsn1", "Falkenstein (DE)"),
            ("nbg1", "Nuremberg (DE)"),
            ("hel1", "Helsinki (FI)"),
            ("ash", "Ashburn (US)"),
            ("hil", "Hillsboro (US)"),
        ],
        CloudProvider::RunPod => vec![("US", "United States"), ("EU", "Europe"), ("CA", "Canada")],
        CloudProvider::LambdaLabs => vec![
            ("us-west-1", "US West"),
            ("us-east-1", "US East"),
            ("us-south-1", "US South"),
            ("europe-central-1", "Europe Central"),
        ],
        CloudProvider::PrimeIntellect => vec![
            ("us-east", "US East"),
            ("us-west", "US West"),
            ("eu-west", "EU West"),
        ],
        CloudProvider::VastAi => vec![
            ("any", "Any (cheapest)"),
            ("US", "United States"),
            ("EU", "Europe"),
        ],
        CloudProvider::Crusoe => vec![
            ("us-east1", "US East"),
            ("us-central1", "US Central"),
            ("us-northwest1", "US Northwest"),
        ],
    };

    let display_regions: Vec<String> = regions
        .iter()
        .map(|(code, name)| format!("{} ({})", name, code))
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select region")
        .items(&display_regions)
        .default(0)
        .interact()?;

    Ok(regions[selection].0.to_string())
}

/// List all configured cloud providers.
///
/// Displays a formatted list of all configured providers with their settings,
/// including region, default status, and project IDs where applicable.
///
/// # Errors
///
/// Returns an error if the configuration file cannot be read.
///
/// # Examples
///
/// ```bash
/// cargo tangle cloud list
/// ```
pub async fn list_providers() -> Result<()> {
    let config = CloudConfig::load()?;

    if config.providers.is_empty() {
        println!("No cloud providers configured.");
        println!("Run `cargo tangle cloud configure <provider>` to get started.");
        return Ok(());
    }

    println!("Configured providers:\n");

    for (provider, settings) in &config.providers {
        let default = if Some(*provider) == config.default_provider {
            " (default)"
        } else {
            ""
        };

        println!("  {} {}", provider, default);
        println!("    Region: {}", settings.region);
        if let Some(project) = &settings.project_id {
            println!("    Project: {}", project);
        }
        println!();
    }

    Ok(())
}
