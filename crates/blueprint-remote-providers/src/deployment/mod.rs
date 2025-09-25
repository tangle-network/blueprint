//! Deployment orchestration and tracking

pub mod manager_integration;
pub mod secure_commands;
pub mod secure_ssh;
pub mod ssh;
pub mod tracker;

#[cfg(feature = "kubernetes")]
pub mod kubernetes;

pub use manager_integration::{RemoteDeploymentConfig, RemoteDeploymentExtensions};
pub use ssh::SshDeploymentClient;
pub use tracker::{DeploymentRecord, DeploymentTracker, DeploymentType};

#[cfg(feature = "kubernetes")]
pub use kubernetes::KubernetesDeploymentClient;
