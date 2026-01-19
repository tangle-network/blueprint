//! Multi-cloud infrastructure provisioning for Blueprint Manager

#![cfg_attr(docsrs, feature(doc_cfg))]

// Core architecture
pub mod auth_integration;
pub mod config;
pub mod core;
pub mod infra;
pub mod monitoring;
pub mod observability;
pub mod pricing;
pub mod providers;
pub mod secure_bridge;
pub mod security;
pub mod shared;

pub mod deployment;

// Primary exports
pub use config::{AwsConfig, AzureConfig, CloudConfig, DigitalOceanConfig, GcpConfig, VultrConfig};
pub use core::{CloudProvider, Error, ResourceSpec, Result};
pub use deployment::{DeploymentTracker, SshDeploymentClient};
pub use infra::{CloudProvisioner, InstanceStatus, ProvisionedInstance};
pub use monitoring::{HealthCheckResult, HealthMonitor, HealthStatus};
pub use pricing::{PricingService, ServiceCostReport as CostReport};
pub use providers::{ProvisionedInfrastructure, ProvisioningConfig};

#[cfg(feature = "aws")]
pub use providers::{AwsInstanceMapper, AwsProvisioner};

pub fn create_provider_client(timeout_secs: u64) -> Result<reqwest::Client> {
    use blueprint_std::time::Duration;
    reqwest::Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .build()
        .map_err(|e| Error::HttpError(e.to_string()))
}

pub fn create_default_provider_client() -> Result<reqwest::Client> {
    create_provider_client(10)
}

pub fn create_metadata_client(timeout_secs: u64) -> Result<reqwest::Client> {
    create_provider_client(timeout_secs)
}

// Legacy compatibility for manager integration
pub mod auto_deployment {
    pub use crate::infra::auto::*;
}
pub mod infrastructure {
    pub use crate::infra::*;
}
pub mod remote {
    pub use crate::core::remote::*;
}
pub mod resources {
    pub use crate::core::resources::*;
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
