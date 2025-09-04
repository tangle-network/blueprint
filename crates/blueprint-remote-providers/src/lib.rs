//! Remote deployment extensions for Blueprint Manager
//!
//! This crate extends the existing Blueprint Manager runtime to support
//! remote deployments to arbitrary cloud Kubernetes clusters and Docker hosts
//! that are separated from the host Blueprint Manager machine.
//!
//! It reuses the existing ContainerRuntime and adds:
//! - Multi-cloud context management
//! - Remote cluster discovery and configuration
//! - Cost tracking and estimation
//! - Cloud-specific networking configurations

#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod blueprint_requirements;
pub mod cost;
pub mod error;
pub mod health_monitor;
pub mod networking;
#[cfg(feature = "pricing")]
pub mod pricing_adapter;
pub mod pricing_integration;
pub mod pricing_unified;
pub mod provisioning;
pub mod remote;
pub mod resources;
pub mod resources_simple;

pub mod deployment_tracker;
pub mod infrastructure;
pub mod infrastructure_unified;
pub mod manager_integration;
pub mod ssh_deployment;

#[cfg(feature = "testing")]
pub mod testing;

// Simplified public API
pub use deployment_tracker::DeploymentTracker;
pub use error::{Error, Result};
pub use health_monitor::{HealthCheckResult, HealthMonitor, HealthStatus};
pub use infrastructure_unified::{
    InstanceStatus, ProvisionedInstance, UnifiedInfrastructureProvisioner,
};
pub use manager_integration::RemoteDeploymentExtensions;
pub use pricing_unified::{CostReport, PricingService};
pub use remote::{CloudProvider, RemoteClusterManager};
pub use resources_simple::ResourceSpec;

// Legacy compatibility exports
#[cfg(feature = "pricing")]
pub use pricing_adapter::{CloudCostReport, PricingAdapter};
pub use pricing_integration::{DetailedCostReport, PricingCalculator, ResourceUsageMetrics};
pub use provisioning::InstanceTypeMapper;
pub use resources::{ComputeResources, NetworkResources, StorageResources};

#[cfg(any(feature = "aws", feature = "api-clients"))]
pub use infrastructure::{
    InfrastructureProvisioner, ProvisionedInfrastructure, ProvisioningConfig,
};

#[cfg(feature = "gcp")]
pub use infrastructure_gcp::{GceInstance, GcpInfrastructureProvisioner, GkeCluster};

#[cfg(feature = "azure")]
pub use infrastructure_azure::{AksCluster, AzureInfrastructureProvisioner, AzureVm};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
