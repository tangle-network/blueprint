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
pub mod pricing_integration;

#[cfg(any(feature = "aws", feature = "api-clients"))]
pub mod infrastructure;

#[cfg(feature = "gcp")]
pub mod infrastructure_gcp;

#[cfg(feature = "azure")]
pub mod infrastructure_azure;

#[cfg(feature = "testing")]
pub mod testing;

pub use error::{Error, Result};
pub use remote::{RemoteClusterManager, RemoteDeploymentConfig, CloudProvider};
pub use cost::{CostEstimator, CostReport};
pub use networking::{TunnelManager, NetworkingMode};
pub use provisioning::{ResourceRequirements, InstanceTypeMapper, AutoScalingConfig};
pub use resources::{UnifiedResourceSpec, ComputeResources, StorageResources, NetworkResources, AcceleratorResources};
pub use pricing_integration::{PricingCalculator, DetailedCostReport, ResourceUsageMetrics};

#[cfg(any(feature = "aws", feature = "api-clients"))]
pub use infrastructure::{InfrastructureProvisioner, ProvisionedInfrastructure, ProvisioningConfig};

#[cfg(feature = "gcp")]
pub use infrastructure_gcp::{GcpInfrastructureProvisioner, GceInstance, GkeCluster};

#[cfg(feature = "azure")]
pub use infrastructure_azure::{AzureInfrastructureProvisioner, AzureVm, AksCluster};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");