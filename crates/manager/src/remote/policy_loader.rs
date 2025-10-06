//! Policy loader for remote deployment configuration.
//!
//! Loads deployment policy from CLI config file with sensible defaults.

use super::serverless::{FaasProviderConfig, ServerlessConfig};
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Minimal policy structure matching CLI config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentPolicy {
    #[serde(default)]
    pub serverless: ServerlessSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerlessSettings {
    #[serde(default)]
    pub enable: bool,
    #[serde(default)]
    pub provider: FaasProviderDef,
    #[serde(default = "default_memory")]
    pub default_memory_mb: u32,
    #[serde(default = "default_timeout")]
    pub default_timeout_secs: u32,
    #[serde(default = "default_fallback")]
    pub fallback_to_vm: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum FaasProviderDef {
    AwsLambda { region: String },
    GcpFunctions { project_id: String },
    AzureFunctions { subscription_id: String },
    Custom { endpoint: String },
}

fn default_memory() -> u32 {
    512
}
fn default_timeout() -> u32 {
    300
}
fn default_fallback() -> bool {
    true
}

impl Default for ServerlessSettings {
    fn default() -> Self {
        Self {
            enable: false,
            provider: FaasProviderDef::AwsLambda {
                region: "us-east-1".to_string(),
            },
            default_memory_mb: default_memory(),
            default_timeout_secs: default_timeout(),
            fallback_to_vm: default_fallback(),
        }
    }
}

impl Default for DeploymentPolicy {
    fn default() -> Self {
        Self {
            serverless: ServerlessSettings::default(),
        }
    }
}

impl From<FaasProviderDef> for FaasProviderConfig {
    fn from(def: FaasProviderDef) -> Self {
        match def {
            FaasProviderDef::AwsLambda { region } => FaasProviderConfig::AwsLambda { region },
            FaasProviderDef::GcpFunctions { project_id } => {
                FaasProviderConfig::GcpFunctions { project_id }
            }
            FaasProviderDef::AzureFunctions { subscription_id } => {
                FaasProviderConfig::AzureFunctions { subscription_id }
            }
            FaasProviderDef::Custom { endpoint } => FaasProviderConfig::Custom { endpoint },
        }
    }
}

impl From<ServerlessSettings> for ServerlessConfig {
    fn from(settings: ServerlessSettings) -> Self {
        Self {
            provider: settings.provider.into(),
            default_memory_mb: settings.default_memory_mb,
            default_timeout_secs: settings.default_timeout_secs,
            fallback_to_vm: settings.fallback_to_vm,
        }
    }
}

/// Load deployment policy from CLI config or return default.
pub fn load_policy() -> DeploymentPolicy {
    match try_load_policy() {
        Ok(policy) => policy,
        Err(e) => {
            tracing::debug!("Failed to load policy, using defaults: {}", e);
            DeploymentPolicy::default()
        }
    }
}

fn try_load_policy() -> Result<DeploymentPolicy> {
    let path = policy_path()?;
    if !path.exists() {
        return Ok(DeploymentPolicy::default());
    }

    let content = std::fs::read_to_string(&path)
        .map_err(|e| Error::Other(format!("Failed to read policy file: {}", e)))?;

    serde_json::from_str(&content)
        .map_err(|e| Error::Other(format!("Failed to parse policy: {}", e)))
}

fn policy_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| Error::Other("Could not find config directory".to_string()))?;
    Ok(config_dir.join("tangle").join("deployment-policy.json"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_policy() {
        let policy = DeploymentPolicy::default();
        assert!(!policy.serverless.enable);
        assert_eq!(policy.serverless.default_memory_mb, 512);
        assert_eq!(policy.serverless.default_timeout_secs, 300);
    }

    #[test]
    fn test_deserialize_policy() {
        let json = r#"{
            "serverless": {
                "enable": true,
                "provider": {
                    "type": "aws-lambda",
                    "region": "us-west-2"
                },
                "default_memory_mb": 1024,
                "default_timeout_secs": 600,
                "fallback_to_vm": false
            }
        }"#;

        let policy: DeploymentPolicy = serde_json::from_str(json).unwrap();
        assert!(policy.serverless.enable);
        assert_eq!(policy.serverless.default_memory_mb, 1024);
        assert!(!policy.serverless.fallback_to_vm);
    }
}
