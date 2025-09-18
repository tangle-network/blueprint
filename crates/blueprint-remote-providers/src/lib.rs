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

pub mod deployment;

// Primary exports
pub use config::{CloudConfig, AwsConfig, GcpConfig, AzureConfig, DigitalOceanConfig, VultrConfig};
pub use core::{Error, Result, CloudProvider, ResourceSpec};
pub use infra::{CloudProvisioner, InstanceStatus, ProvisionedInstance};
pub use deployment::{DeploymentTracker, SshDeploymentClient};
pub use monitoring::{HealthMonitor, HealthStatus, HealthCheckResult};
pub use pricing::{PricingService, ServiceCostReport as CostReport};
pub use providers::{ProvisioningConfig, ProvisionedInfrastructure};

#[cfg(feature = "aws")]
pub use providers::{AwsProvisioner, AwsInstanceMapper};

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
