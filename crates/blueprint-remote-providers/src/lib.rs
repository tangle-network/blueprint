//! Multi-cloud infrastructure provisioning for Blueprint Manager

#![cfg_attr(docsrs, feature(doc_cfg))]

// Core architecture
pub mod core;
pub mod infra;
pub mod deployment;
pub mod monitoring;
pub mod pricing;
pub mod providers;

// Primary exports
pub use core::{Error, Result, CloudProvider, ResourceSpec};
pub use infra::{CloudProvisioner, InstanceStatus, ProvisionedInstance};
pub use deployment::{DeploymentTracker, SshDeploymentClient};
pub use monitoring::{HealthMonitor, HealthStatus, HealthCheckResult};
pub use pricing::{PricingService, ServiceCostReport as CostReport};
pub use providers::{ProvisioningConfig, ProvisionedInfrastructure};

#[cfg(feature = "aws")]
pub use providers::{AwsProvisioner, AwsInstanceMapper};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
