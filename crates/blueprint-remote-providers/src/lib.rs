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

pub mod error;
pub mod remote;
pub mod cost;
pub mod networking;
pub mod provisioning;
pub mod resources;
pub mod resources_simple;
pub mod pricing_unified;
pub mod health_monitor;
pub mod blueprint_requirements;
pub mod pricing_integration;
#[cfg(feature = "pricing")]
pub mod pricing_adapter;

pub mod infrastructure;
pub mod infrastructure_unified;
pub mod ssh_deployment;
pub mod deployment_tracker;
pub mod manager_integration;

#[cfg(feature = "testing")]
pub mod testing;

// Simplified public API
pub use error::{Error, Result};
pub use remote::{RemoteClusterManager, CloudProvider};
pub use resources_simple::ResourceSpec;
pub use deployment_tracker::DeploymentTracker;
pub use infrastructure_unified::{UnifiedInfrastructureProvisioner, ProvisionedInstance, InstanceStatus};
pub use manager_integration::RemoteDeploymentExtensions;
pub use pricing_unified::{PricingService, CostReport};
pub use health_monitor::{HealthMonitor, HealthStatus, HealthCheckResult};

// Legacy compatibility exports
pub use resources::{ComputeResources, StorageResources, NetworkResources};
pub use provisioning::InstanceTypeMapper;
pub use pricing_integration::{PricingCalculator, DetailedCostReport, ResourceUsageMetrics};
#[cfg(feature = "pricing")]
pub use pricing_adapter::{PricingAdapter, CloudCostReport};

#[cfg(any(feature = "aws", feature = "api-clients"))]
pub use infrastructure::{InfrastructureProvisioner, ProvisionedInfrastructure, ProvisioningConfig};

#[cfg(feature = "gcp")]
pub use infrastructure_gcp::{GcpInfrastructureProvisioner, GceInstance, GkeCluster};

#[cfg(feature = "azure")]
pub use infrastructure_azure::{AzureInfrastructureProvisioner, AzureVm, AksCluster};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");