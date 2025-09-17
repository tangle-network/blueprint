//! Deployment orchestration and tracking

pub mod manager_integration;
pub mod ssh;
pub mod tracker;

pub use manager_integration::{RemoteDeploymentConfig, RemoteDeploymentExtensions};
pub use ssh::SshDeploymentClient;
pub use tracker::{DeploymentRecord, DeploymentTracker, DeploymentType};
