//! Azure Resource Manager provisioning
//!
//! Provisions Azure Virtual Machines using Azure Resource Manager APIs

use crate::core::error::{Error, Result};
use crate::core::resources::ResourceSpec;
use crate::providers::common::{ProvisionedInfrastructure, ProvisioningConfig};

/// Azure Resource Manager provisioner
pub struct AzureProvisioner {
    subscription_id: String,
    resource_group: String,
    client: reqwest::Client,
}

impl AzureProvisioner {
    /// Create new Azure provisioner
    pub async fn new() -> Result<Self> {
        let subscription_id = std::env::var("AZURE_SUBSCRIPTION_ID")
            .map_err(|_| Error::ConfigurationError("AZURE_SUBSCRIPTION_ID not set".into()))?;
        
        let resource_group = std::env::var("AZURE_RESOURCE_GROUP")
            .unwrap_or_else(|_| "blueprint-resources".to_string());

        let client = reqwest::Client::new();

        Ok(Self {
            subscription_id,
            resource_group,
            client,
        })
    }

    /// Provision an Azure VM
    pub async fn provision_instance(
        &self,
        _spec: &ResourceSpec,
        _config: &ProvisioningConfig,
    ) -> Result<ProvisionedInfrastructure> {
        Err(Error::ConfigurationError(
            "Azure provisioning not yet implemented - use Azure CLI or ARM templates".into()
        ))
    }

    /// Terminate an Azure VM
    pub async fn terminate_instance(&self, _instance_id: &str) -> Result<()> {
        Err(Error::ConfigurationError(
            "Azure termination not yet implemented".into()
        ))
    }
}