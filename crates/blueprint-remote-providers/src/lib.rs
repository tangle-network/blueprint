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

#[cfg(any(feature = "aws", feature = "api-clients"))]
pub mod infrastructure;

#[cfg(feature = "testing")]
pub mod testing;

pub use error::{Error, Result};
pub use remote::{RemoteClusterManager, RemoteDeploymentConfig, CloudProvider};
pub use cost::{CostEstimator, CostReport};
pub use networking::{TunnelManager, NetworkingMode};
pub use provisioning::{ResourceRequirements, InstanceTypeMapper, AutoScalingConfig};

#[cfg(any(feature = "aws", feature = "api-clients"))]
pub use infrastructure::{InfrastructureProvisioner, ProvisionedInfrastructure, ProvisioningConfig};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");