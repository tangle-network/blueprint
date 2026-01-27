//! Cloud provider configuration types

use serde::{Deserialize, Serialize};

/// Cloud provider configuration for all supported providers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CloudConfig {
    pub enabled: bool,
    pub aws: Option<AwsConfig>,
    pub gcp: Option<GcpConfig>,
    pub azure: Option<AzureConfig>,
    pub digital_ocean: Option<DigitalOceanConfig>,
    pub vultr: Option<VultrConfig>,
}

/// AWS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsConfig {
    pub enabled: bool,
    pub region: String,
    pub access_key: String,
    pub secret_key: String,
    pub priority: Option<u8>,
}

/// GCP configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcpConfig {
    pub enabled: bool,
    pub region: String,
    pub project_id: String,
    pub service_account_path: String,
    pub priority: Option<u8>,
}

/// Azure configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureConfig {
    pub enabled: bool,
    pub region: String,
    pub client_id: String,
    pub client_secret: String,
    pub tenant_id: String,
    pub priority: Option<u8>,
}

/// DigitalOcean configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalOceanConfig {
    pub enabled: bool,
    pub region: String,
    pub api_token: String,
    pub priority: Option<u8>,
}

/// Vultr configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VultrConfig {
    pub enabled: bool,
    pub region: String,
    pub api_key: String,
    pub priority: Option<u8>,
}

impl CloudConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Option<Self> {
        use std::env;

        let mut cloud_config = CloudConfig::default();
        let mut any_enabled = false;

        // AWS configuration
        if let (Ok(key), Ok(secret)) = (
            env::var("AWS_ACCESS_KEY_ID"),
            env::var("AWS_SECRET_ACCESS_KEY"),
        ) {
            cloud_config.aws = Some(AwsConfig {
                enabled: true,
                region: env::var("AWS_DEFAULT_REGION").unwrap_or_else(|_| "us-east-1".to_string()),
                access_key: key,
                secret_key: secret,
                priority: Some(10),
            });
            any_enabled = true;
        }

        // GCP configuration
        if let Ok(project_id) = env::var("GCP_PROJECT_ID") {
            let service_account_path = env::var("GOOGLE_APPLICATION_CREDENTIALS")
                .unwrap_or_else(|_| "/etc/gcp/service-account.json".to_string());
            cloud_config.gcp = Some(GcpConfig {
                enabled: true,
                region: env::var("GCP_DEFAULT_REGION")
                    .unwrap_or_else(|_| "us-central1".to_string()),
                project_id,
                service_account_path,
                priority: Some(8),
            });
            any_enabled = true;
        }

        // Azure configuration
        if let (Ok(client_id), Ok(client_secret), Ok(tenant_id)) = (
            env::var("AZURE_CLIENT_ID"),
            env::var("AZURE_CLIENT_SECRET"),
            env::var("AZURE_TENANT_ID"),
        ) {
            cloud_config.azure = Some(AzureConfig {
                enabled: true,
                region: env::var("AZURE_DEFAULT_REGION").unwrap_or_else(|_| "East US".to_string()),
                client_id,
                client_secret,
                tenant_id,
                priority: Some(7),
            });
            any_enabled = true;
        }

        // DigitalOcean configuration
        if let Ok(token) = env::var("DO_API_TOKEN") {
            cloud_config.digital_ocean = Some(DigitalOceanConfig {
                enabled: true,
                region: env::var("DO_DEFAULT_REGION").unwrap_or_else(|_| "nyc3".to_string()),
                api_token: token,
                priority: Some(5),
            });
            any_enabled = true;
        }

        // Vultr configuration
        if let Ok(key) = env::var("VULTR_API_KEY") {
            cloud_config.vultr = Some(VultrConfig {
                enabled: true,
                region: env::var("VULTR_DEFAULT_REGION").unwrap_or_else(|_| "ewr".to_string()),
                api_key: key,
                priority: Some(3),
            });
            any_enabled = true;
        }

        if any_enabled {
            cloud_config.enabled = true;
            Some(cloud_config)
        } else {
            None
        }
    }
}
