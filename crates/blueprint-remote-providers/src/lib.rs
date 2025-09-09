//! Remote deployment extensions for Blueprint Manager
//!
//! Features:
//! - Multi-cloud context management
//! - Remote cluster discovery and configuration
//! - Cost tracking and estimation
//! - Cloud-specific networking configurations

#![cfg_attr(docsrs, feature(doc_cfg))]

// Core modules
pub mod error;
pub mod networking;
pub mod remote;
pub mod resources;

// Organized feature modules
pub mod providers;
pub mod deployment;
pub mod pricing;
pub mod monitoring;

// Legacy modules (keeping for now)
pub mod provisioning;
pub mod cloud_provisioner;
pub mod infrastructure;

#[cfg(test)]
pub mod test_utils;

#[cfg(feature = "testing")]
pub mod testing;

// Primary API exports
pub use cloud_provisioner::{CloudProvisioner, InstanceStatus, ProvisionedInstance};
pub use deployment::{DeploymentTracker, RemoteDeploymentExtensions, SshDeploymentClient};
pub use error::{Error, Result};
pub use monitoring::{HealthCheckResult, HealthMonitor, HealthStatus};
pub use pricing::{CostReport, PricingService};
#[cfg(feature = "pricing")]
pub use pricing::{CloudCostReport, PricingAdapter};
pub use providers::{ProvisionedInfrastructure, ProvisioningConfig};
#[cfg(feature = "aws")]
pub use providers::{AwsProvisioner, AwsInstanceMapper};
pub use remote::{CloudProvider, RemoteClusterManager};
pub use resources::ResourceSpec;

// Legacy compatibility exports
pub use provisioning::InstanceTypeMapper;

#[cfg(any(feature = "aws", feature = "api-clients"))]
pub use infrastructure::InfrastructureProvisioner;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");