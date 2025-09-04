//! Cloud provider configuration management.
//!
//! This module handles the configuration and authentication setup for various cloud providers.
//! It provides interactive setup flows, credential management, and persistent configuration storage.

use clap::ValueEnum;
use color_eyre::{Result, eyre::Context};
use dialoguer::{Input, Password, Select, theme::ColorfulTheme};
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
}

impl std::fmt::Display for CloudProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AWS => write!(f, "AWS"),
            Self::GCP => write!(f, "Google Cloud"),
            Self::Azure => write!(f, "Azure"),
            Self::DigitalOcean => write!(f, "DigitalOcean"),
            Self::Vultr => write!(f, "Vultr"),
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
    /// Internal flag indicating if provider is fully configured
    #[serde(skip)]
    pub configured: bool,
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

/// Configure a cloud provider with interactive setup.
///
/// This function guides the user through provider-specific authentication setup,
/// including credential configuration, region selection, and default settings.
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
/// * Credentials are invalid or cannot be saved
/// * Provider-specific CLI tools are not available
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

    // Provider-specific setup
    match provider {
        CloudProvider::AWS => configure_aws().await?,
        CloudProvider::GCP => configure_gcp().await?,
        CloudProvider::Azure => configure_azure().await?,
        CloudProvider::DigitalOcean => configure_digitalocean().await?,
        CloudProvider::Vultr => configure_vultr().await?,
    }

    // Save settings
    let mut settings = ProviderSettings {
        region,
        project_id: None,
        configured: true,
    };

    // GCP needs project ID
    if provider == CloudProvider::GCP {
        settings.project_id = Some(Input::new().with_prompt("GCP Project ID").interact()?);
    }

    config.providers.insert(provider, settings);

    if set_default || config.default_provider.is_none() {
        config.default_provider = Some(provider);
    }

    config.save()?;

    println!("\n✅ {} configured successfully!", provider);
    if config.default_provider == Some(provider) {
        println!("   Set as default provider");
    }

    Ok(())
}

/// Configure AWS credentials
async fn configure_aws() -> Result<()> {
    // Check for existing AWS CLI config
    let aws_config = dirs::home_dir()
        .map(|h| h.join(".aws").join("credentials"))
        .filter(|p| p.exists());

    if let Some(_) = aws_config {
        println!("✓ Found AWS credentials in ~/.aws/credentials");
        return Ok(());
    }

    // Check environment variables
    if std::env::var("AWS_ACCESS_KEY_ID").is_ok() {
        println!("✓ Found AWS credentials in environment");
        return Ok(());
    }

    // Prompt for credentials
    println!("No AWS credentials found. Please provide:");
    println!("(These will be stored in ~/.aws/credentials)");

    let access_key = Input::<String>::new()
        .with_prompt("AWS Access Key ID")
        .interact()?;

    let secret_key = Password::new()
        .with_prompt("AWS Secret Access Key")
        .interact()?;

    // Save to ~/.aws/credentials
    let aws_dir = dirs::home_dir()
        .ok_or_else(|| color_eyre::eyre::eyre!("Could not find home directory"))?
        .join(".aws");

    std::fs::create_dir_all(&aws_dir)?;

    let credentials = format!(
        "[default]\naws_access_key_id = {}\naws_secret_access_key = {}\n",
        access_key, secret_key
    );

    std::fs::write(aws_dir.join("credentials"), credentials)?;

    Ok(())
}

/// Configure GCP credentials  
async fn configure_gcp() -> Result<()> {
    // Check for gcloud CLI
    if std::process::Command::new("gcloud")
        .arg("--version")
        .output()
        .is_ok()
    {
        println!("✓ Found gcloud CLI");

        // Check if already authenticated
        let output = std::process::Command::new("gcloud")
            .args(&[
                "auth",
                "list",
                "--filter=status:ACTIVE",
                "--format=value(account)",
            ])
            .output()?;

        if !output.stdout.is_empty() {
            let account = String::from_utf8_lossy(&output.stdout);
            println!("✓ Authenticated as {}", account.trim());
            return Ok(());
        }

        // Run gcloud auth
        println!("Running gcloud auth login...");
        std::process::Command::new("gcloud")
            .args(&["auth", "application-default", "login"])
            .status()?;
    } else {
        println!("⚠️  gcloud CLI not found");
        println!("   Please install: https://cloud.google.com/sdk/docs/install");
        println!("   Or set GOOGLE_APPLICATION_CREDENTIALS to a service account key file");
    }

    Ok(())
}

/// Configure Azure credentials
async fn configure_azure() -> Result<()> {
    // Check for az CLI
    if std::process::Command::new("az")
        .arg("--version")
        .output()
        .is_ok()
    {
        println!("✓ Found Azure CLI");

        // Check if logged in
        let output = std::process::Command::new("az")
            .args(&["account", "show"])
            .output()?;

        if output.status.success() {
            println!("✓ Already logged in to Azure");
            return Ok(());
        }

        // Run az login
        println!("Running az login...");
        std::process::Command::new("az").arg("login").status()?;
    } else {
        println!("⚠️  Azure CLI not found");
        println!("   Please install: https://aka.ms/azure-cli");
    }

    Ok(())
}

/// Configure DigitalOcean credentials
async fn configure_digitalocean() -> Result<()> {
    if std::env::var("DIGITALOCEAN_TOKEN").is_ok() {
        println!("✓ Found DigitalOcean token in environment");
        return Ok(());
    }

    println!("Get your API token from: https://cloud.digitalocean.com/account/api/tokens");

    let token = Password::new()
        .with_prompt("DigitalOcean API Token")
        .interact()?;

    // Save to .env file
    let env_file = std::env::current_dir()?.join(".env");
    let mut content = if env_file.exists() {
        std::fs::read_to_string(&env_file)?
    } else {
        String::new()
    };

    if !content.contains("DIGITALOCEAN_TOKEN") {
        content.push_str(&format!("\nDIGITALOCEAN_TOKEN={}\n", token));
        std::fs::write(env_file, content)?;
        println!("✓ Saved to .env file");
    }

    Ok(())
}

/// Configure Vultr credentials
async fn configure_vultr() -> Result<()> {
    if std::env::var("VULTR_API_KEY").is_ok() {
        println!("✓ Found Vultr API key in environment");
        return Ok(());
    }

    println!("Get your API key from: https://my.vultr.com/settings/#settingsapi");

    let api_key = Password::new().with_prompt("Vultr API Key").interact()?;

    // Save to .env file
    let env_file = std::env::current_dir()?.join(".env");
    let mut content = if env_file.exists() {
        std::fs::read_to_string(&env_file)?
    } else {
        String::new()
    };

    if !content.contains("VULTR_API_KEY") {
        content.push_str(&format!("\nVULTR_API_KEY={}\n", api_key));
        std::fs::write(env_file, content)?;
        println!("✓ Saved to .env file");
    }

    Ok(())
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
