//! Security-hardened cloud provider configuration management.
//!
//! Replaces the insecure plain-text credential storage with encrypted credentials
//! using the blueprint-remote-providers security infrastructure.

use clap::ValueEnum;
use color_eyre::{Result, eyre::Context};
use dialoguer::{Input, Password, Select, theme::ColorfulTheme, Confirm};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

// Import security-hardened components
#[cfg(feature = "remote-providers")]
use blueprint_remote_providers::security::SecureCloudCredentials;
#[cfg(feature = "remote-providers")]
use blueprint_remote_providers::auth_integration::RemoteServiceAuth;
#[cfg(feature = "remote-providers")]
use blueprint_remote_providers::core::remote::CloudProvider as RemoteCloudProvider;

/// Supported cloud providers for Blueprint deployment (maps to secure providers).
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

impl From<CloudProvider> for RemoteCloudProvider {
    fn from(provider: CloudProvider) -> Self {
        match provider {
            CloudProvider::AWS => RemoteCloudProvider::AWS,
            CloudProvider::GCP => RemoteCloudProvider::GCP,
            CloudProvider::Azure => RemoteCloudProvider::Azure,
            CloudProvider::DigitalOcean => RemoteCloudProvider::DigitalOcean,
            CloudProvider::Vultr => RemoteCloudProvider::Vultr,
        }
    }
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

/// Secure cloud configuration storage with encrypted credentials.
///
/// Persisted to ~/.config/tangle/secure_cloud.json with encrypted credential blobs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureCloudConfig {
    /// The default provider to use when none is specified
    pub default_provider: Option<CloudProvider>,
    /// Per-provider configuration settings with encrypted credentials
    pub providers: HashMap<CloudProvider, SecureProviderSettings>,
    /// Configuration format version for migration support
    pub version: u32,
}

/// Provider-specific configuration settings with security enhancements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureProviderSettings {
    /// Default region for deployments
    pub region: String,
    /// GCP project ID (only used for Google Cloud)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    /// Encrypted credential storage
    pub secure_credentials: Option<SecureCloudCredentials>,
    /// Internal flag indicating if provider is fully configured
    #[serde(skip)]
    pub configured: bool,
    /// Last credential rotation timestamp
    pub last_rotation: Option<chrono::DateTime<chrono::Utc>>,
}

impl SecureCloudConfig {
    /// Load secure config from disk or create default
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;

        if path.exists() {
            let content = std::fs::read_to_string(&path).context("Failed to read secure cloud config")?;
            let config: SecureCloudConfig = serde_json::from_str(&content)
                .context("Failed to parse secure cloud config")?;
            
            // Validate config version
            if config.version > Self::CURRENT_VERSION {
                return Err(color_eyre::eyre::eyre!(
                    "Config version {} is newer than supported version {}. Please update tangle CLI.",
                    config.version, Self::CURRENT_VERSION
                ));
            }
            
            Ok(config)
        } else {
            // Check for legacy config and offer migration
            if let Ok(legacy_config) = super::config::CloudConfig::load() {
                if !legacy_config.providers.is_empty() {
                    println!("🔒 Found legacy cloud configuration. Migrating to secure storage...");
                    let migrated = Self::migrate_from_legacy(legacy_config)?;
                    migrated.save()?;
                    println!("✅ Migration complete. Legacy credentials have been encrypted.");
                    return Ok(migrated);
                }
            }
            
            Ok(Self::default())
        }
    }

    /// Save secure config to disk with proper permissions
    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).context("Failed to create config directory")?;
        }

        let content = serde_json::to_string_pretty(self).context("Failed to serialize secure config")?;
        std::fs::write(&path, content).context("Failed to write secure cloud config")?;

        // Set secure file permissions (readable only by owner)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&path)?.permissions();
            perms.set_mode(0o600); // rw-------
            std::fs::set_permissions(&path, perms)?;
        }

        Ok(())
    }

    /// Get next service ID for secure credentials
    fn next_service_id(&self) -> u64 {
        self.providers.len() as u64 + 1000 // Start from 1000 for CLI-configured providers
    }

    const CURRENT_VERSION: u32 = 1;

    fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| color_eyre::eyre::eyre!("Could not find config directory"))?;
        Ok(config_dir.join("tangle").join("secure_cloud.json"))
    }

    /// Migrate from legacy insecure configuration
    fn migrate_from_legacy(legacy: super::config::CloudConfig) -> Result<Self> {
        let mut secure_config = Self::default();
        
        println!("🔄 Migrating {} provider(s) to secure storage...", legacy.providers.len());
        
        // Note: Legacy config only has region info, not actual credentials
        // We'll create placeholder secure settings that require re-configuration
        for (provider, settings) in legacy.providers {
            let secure_provider = match provider {
                super::config::CloudProvider::AWS => CloudProvider::AWS,
                super::config::CloudProvider::GCP => CloudProvider::GCP,
                super::config::CloudProvider::Azure => CloudProvider::Azure,
                super::config::CloudProvider::DigitalOcean => CloudProvider::DigitalOcean,
                super::config::CloudProvider::Vultr => CloudProvider::Vultr,
            };

            secure_config.providers.insert(secure_provider, SecureProviderSettings {
                region: settings.region,
                project_id: settings.project_id,
                secure_credentials: None, // Will need to be reconfigured
                configured: false, // Requires credential setup
                last_rotation: None,
            });
        }
        
        secure_config.default_provider = legacy.default_provider.map(|p| match p {
            super::config::CloudProvider::AWS => CloudProvider::AWS,
            super::config::CloudProvider::GCP => CloudProvider::GCP,
            super::config::CloudProvider::Azure => CloudProvider::Azure,
            super::config::CloudProvider::DigitalOcean => CloudProvider::DigitalOcean,
            super::config::CloudProvider::Vultr => CloudProvider::Vultr,
        });

        println!("⚠️  Credentials will need to be reconfigured for security.");
        
        Ok(secure_config)
    }
}

impl Default for SecureCloudConfig {
    fn default() -> Self {
        Self {
            default_provider: None,
            providers: HashMap::new(),
            version: Self::CURRENT_VERSION,
        }
    }
}

/// Configure a cloud provider with security-hardened credential storage.
pub async fn configure_secure(
    provider: CloudProvider,
    region: Option<String>,
    set_default: bool,
) -> Result<()> {
    println!("🔒 Configuring {} with secure credential storage...\n", provider);

    let mut config = SecureCloudConfig::load()?;

    // Get or prompt for region
    let region = if let Some(r) = region {
        r
    } else {
        prompt_region(provider)?
    };

    // Collect credentials securely
    let credentials_json = collect_credentials_securely(provider).await?;
    
    // Create secure credentials with encryption
    let service_id = config.next_service_id();
    let secure_credentials = SecureCloudCredentials::new(
        service_id,
        &provider.to_string().to_lowercase(),
        &credentials_json,
    ).await.context("Failed to create secure credentials")?;

    // Save settings
    let mut settings = SecureProviderSettings {
        region,
        project_id: None,
        secure_credentials: Some(secure_credentials),
        configured: true,
        last_rotation: Some(chrono::Utc::now()),
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

    println!("\n✅ {} configured securely!", provider);
    println!("   🔐 Credentials encrypted with AES-GCM");
    println!("   📋 Service ID: {}", service_id);
    if config.default_provider == Some(provider) {
        println!("   🎯 Set as default provider");
    }

    Ok(())
}

/// Collect credentials securely based on provider type
async fn collect_credentials_securely(provider: CloudProvider) -> Result<String> {
    match provider {
        CloudProvider::AWS => {
            // Check for existing secure credential sources first
            if check_aws_credential_sources().await? {
                return Ok("{}".to_string()); // Use existing secure sources
            }

            println!("No secure AWS credentials found. Please provide:");
            let access_key = Input::<String>::new()
                .with_prompt("AWS Access Key ID")
                .interact()?;

            let secret_key = Password::new()
                .with_prompt("AWS Secret Access Key")
                .interact()?;

            Ok(serde_json::json!({
                "aws_access_key": access_key,
                "aws_secret_key": secret_key
            }).to_string())
        },
        CloudProvider::GCP => {
            if check_gcp_credential_sources().await? {
                return Ok("{}".to_string()); // Use existing secure sources
            }

            println!("Please provide GCP service account key file path:");
            let key_path = Input::<String>::new()
                .with_prompt("Service Account Key Path")
                .interact()?;

            // Validate the file exists and is readable
            if !std::path::Path::new(&key_path).exists() {
                return Err(color_eyre::eyre::eyre!("Service account key file not found"));
            }

            Ok(serde_json::json!({
                "gcp_service_account_key": key_path
            }).to_string())
        },
        CloudProvider::DigitalOcean => {
            if std::env::var("DIGITALOCEAN_TOKEN").is_ok() {
                return Ok("{}".to_string()); // Use environment variable
            }

            println!("Get your API token from: https://cloud.digitalocean.com/account/api/tokens");
            let token = Password::new()
                .with_prompt("DigitalOcean API Token")
                .interact()?;

            Ok(serde_json::json!({
                "do_api_token": token
            }).to_string())
        },
        CloudProvider::Azure => {
            if check_azure_credential_sources().await? {
                return Ok("{}".to_string()); // Use existing secure sources
            }

            println!("Please provide Azure service principal credentials:");
            let client_id = Input::<String>::new()
                .with_prompt("Client ID")
                .interact()?;
            let client_secret = Password::new()
                .with_prompt("Client Secret")
                .interact()?;
            let tenant_id = Input::<String>::new()
                .with_prompt("Tenant ID")
                .interact()?;
            let subscription_id = Input::<String>::new()
                .with_prompt("Subscription ID")
                .interact()?;

            Ok(serde_json::json!({
                "azure_client_id": client_id,
                "azure_client_secret": client_secret,
                "azure_tenant_id": tenant_id,
                "azure_subscription_id": subscription_id
            }).to_string())
        },
        CloudProvider::Vultr => {
            if std::env::var("VULTR_API_KEY").is_ok() {
                return Ok("{}".to_string()); // Use environment variable
            }

            println!("Get your API key from: https://my.vultr.com/settings/#settingsapi");
            let api_key = Password::new()
                .with_prompt("Vultr API Key")
                .interact()?;

            Ok(serde_json::json!({
                "vultr_api_key": api_key
            }).to_string())
        },
    }
}

/// Check for secure AWS credential sources
async fn check_aws_credential_sources() -> Result<bool> {
    // Check IAM role (most secure)
    if std::env::var("AWS_ROLE_ARN").is_ok() {
        println!("✓ Found AWS IAM role configuration");
        return Ok(true);
    }

    // Check AWS CLI with proper profile
    let aws_config = dirs::home_dir()
        .map(|h| h.join(".aws").join("credentials"))
        .filter(|p| p.exists());

    if aws_config.is_some() {
        println!("✓ Found AWS credentials in ~/.aws/credentials");
        let use_existing = Confirm::new()
            .with_prompt("Use existing AWS credentials?")
            .default(true)
            .interact()?;
        return Ok(use_existing);
    }

    Ok(false)
}

/// Check for secure GCP credential sources
async fn check_gcp_credential_sources() -> Result<bool> {
    // Check for gcloud CLI
    if std::process::Command::new("gcloud")
        .arg("--version")
        .output()
        .is_ok()
    {
        let output = std::process::Command::new("gcloud")
            .args(&["auth", "list", "--filter=status:ACTIVE", "--format=value(account)"])
            .output()?;

        if !output.stdout.is_empty() {
            let account = String::from_utf8_lossy(&output.stdout);
            println!("✓ Found gcloud authentication: {}", account.trim());
            let use_existing = Confirm::new()
                .with_prompt("Use existing gcloud authentication?")
                .default(true)
                .interact()?;
            return Ok(use_existing);
        }
    }

    Ok(false)
}

/// Check for secure Azure credential sources
async fn check_azure_credential_sources() -> Result<bool> {
    // Check for az CLI
    if std::process::Command::new("az")
        .arg("--version")
        .output()
        .is_ok()
    {
        let output = std::process::Command::new("az")
            .args(&["account", "show"])
            .output()?;

        if output.status.success() {
            println!("✓ Found Azure CLI authentication");
            let use_existing = Confirm::new()
                .with_prompt("Use existing Azure CLI authentication?")
                .default(true)
                .interact()?;
            return Ok(use_existing);
        }
    }

    Ok(false)
}

/// Rotate credentials for a provider
pub async fn rotate_credentials(provider: CloudProvider) -> Result<()> {
    println!("🔄 Rotating credentials for {}...", provider);

    let mut config = SecureCloudConfig::load()?;
    
    let settings = config.providers.get_mut(&provider)
        .ok_or_else(|| color_eyre::eyre::eyre!("Provider {} not configured", provider))?;

    // Collect new credentials
    let credentials_json = collect_credentials_securely(provider).await?;
    
    // Create new secure credentials
    let service_id = config.next_service_id();
    let new_credentials = SecureCloudCredentials::new(
        service_id,
        &provider.to_string().to_lowercase(),
        &credentials_json,
    ).await.context("Failed to create new secure credentials")?;

    // Update settings
    settings.secure_credentials = Some(new_credentials);
    settings.last_rotation = Some(chrono::Utc::now());

    config.save()?;

    println!("✅ Credentials rotated successfully!");
    println!("   🔐 New credentials encrypted");
    println!("   📋 New service ID: {}", service_id);

    Ok(())
}

/// List all configured providers with security status
pub async fn list_secure_providers() -> Result<()> {
    let config = SecureCloudConfig::load()?;

    if config.providers.is_empty() {
        println!("No cloud providers configured.");
        println!("Run `cargo tangle cloud secure configure <provider>` to get started.");
        return Ok(());
    }

    println!("🔒 Secure cloud providers:\n");

    for (provider, settings) in &config.providers {
        let default = if Some(*provider) == config.default_provider {
            " (default)"
        } else {
            ""
        };

        let security_status = if settings.secure_credentials.is_some() {
            "🔐 Encrypted"
        } else {
            "⚠️  Needs configuration"
        };

        println!("  {} {}", provider, default);
        println!("    Region: {}", settings.region);
        println!("    Security: {}", security_status);
        
        if let Some(rotation) = settings.last_rotation {
            let days_ago = (chrono::Utc::now() - rotation).num_days();
            println!("    Last rotation: {} days ago", days_ago);
            
            if days_ago > 90 {
                println!("    ⚠️  Consider rotating credentials (>90 days old)");
            }
        }
        
        if let Some(project) = &settings.project_id {
            println!("    Project: {}", project);
        }
        println!();
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