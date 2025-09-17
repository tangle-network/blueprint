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

/// Auto-deployment manager for Blueprint Manager integration
pub mod auto_deployment;

/// Service classification and deployment strategy
pub mod service_classifier;

/// Blueprint manifest extensions for deployment configuration  
pub mod blueprint_extensions;

/// Runtime interface for Blueprint Manager integration
pub mod runtime_interface;

/// Integration with existing on-chain heartbeat/QoS reporting system
#[cfg(feature = "qos-integration")]
pub mod heartbeat_integration;

/// Secure communication bridge for remote instances
pub mod secure_bridge;

/// Auth system integration for remote deployments
pub mod auth_integration;

/// Resilience patterns for remote communication
pub mod resilience;

/// Observability and metrics
pub mod observability;

/// Provider API integration tests
#[cfg(test)]
mod provider_api_tests;
pub mod resources;

// Organized feature modules
pub mod deployment;
pub mod monitoring;
pub mod pricing;
pub mod providers;

// Legacy modules (keeping for now)
pub mod cloud_provisioner;
pub mod infrastructure;
pub mod provisioning;

#[cfg(test)]
pub mod test_utils;

#[cfg(feature = "testing")]
pub mod testing;

// Primary API exports
pub use cloud_provisioner::{CloudProvisioner, InstanceStatus, ProvisionedInstance};
pub use deployment::{DeploymentTracker, RemoteDeploymentExtensions, SshDeploymentClient};
pub use error::{Error, Result};
pub use monitoring::{HealthCheckResult, HealthMonitor, HealthStatus};
#[cfg(feature = "pricing")]
pub use pricing::{CloudCostReport, PricingAdapter};
pub use pricing::{CostReport, PricingService};
#[cfg(feature = "aws")]
pub use providers::{AwsInstanceMapper, AwsProvisioner};
pub use providers::{ProvisionedInfrastructure, ProvisioningConfig};
pub use remote::{CloudProvider, RemoteClusterManager};
pub use resources::ResourceSpec;

// Legacy compatibility exports
pub use provisioning::InstanceTypeMapper;

#[cfg(any(feature = "aws", feature = "api-clients"))]
pub use infrastructure::InfrastructureProvisioner;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
