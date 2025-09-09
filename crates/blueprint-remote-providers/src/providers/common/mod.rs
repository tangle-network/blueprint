//! Common types and traits for all cloud providers

use crate::remote::CloudProvider;
use blueprint_std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Result of instance type selection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceSelection {
    pub instance_type: String,
    pub spot_capable: bool,
    pub estimated_hourly_cost: Option<f64>,
}

/// Configuration for infrastructure provisioning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvisioningConfig {
    /// Deployment name/identifier
    pub name: String,
    /// Target region
    pub region: String,
    /// SSH key name (provider-specific)
    pub ssh_key_name: Option<String>,
    /// AMI ID for AWS (optional)
    pub ami_id: Option<String>,
    /// Machine image for GCP (optional)
    pub machine_image: Option<String>,
    /// Additional provider-specific configuration
    pub custom_config: HashMap<String, String>,
}

impl Default for ProvisioningConfig {
    fn default() -> Self {
        Self {
            name: "blueprint-deployment".to_string(),
            region: "us-west-2".to_string(),
            ssh_key_name: None,
            ami_id: None,
            machine_image: None,
            custom_config: HashMap::new(),
        }
    }
}

/// Provisioned infrastructure details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvisionedInfrastructure {
    pub provider: CloudProvider,
    pub instance_id: String,
    pub public_ip: Option<String>,
    pub private_ip: Option<String>,
    pub region: String,
    pub instance_type: String,
    pub metadata: HashMap<String, String>,
}

impl ProvisionedInfrastructure {
    /// Check if the infrastructure is ready for deployment
    pub async fn is_ready(&self) -> bool {
        // TODO: Add instance health checks
        self.public_ip.is_some() || self.private_ip.is_some()
    }

    /// Get connection endpoint for this infrastructure
    pub fn get_endpoint(&self) -> Option<String> {
        self.public_ip.clone().or_else(|| self.private_ip.clone())
    }
}

/// Trait for cloud provider provisioners
pub trait CloudProvisioner {
    type Config: Clone + Send + Sync;
    type Instance: Clone + Send + Sync;
    
    async fn new(config: Self::Config) -> crate::error::Result<Self>
    where
        Self: Sized;
    
    async fn provision_instance(
        &self,
        spec: &crate::resources::ResourceSpec,
        config: &ProvisioningConfig,
    ) -> crate::error::Result<ProvisionedInfrastructure>;
    
    async fn terminate_instance(&self, instance_id: &str) -> crate::error::Result<()>;
}