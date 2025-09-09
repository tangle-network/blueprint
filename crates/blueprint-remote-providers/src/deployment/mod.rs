//! Deployment orchestration and tracking

pub mod tracker;
pub mod ssh;
pub mod manager_integration;

pub use tracker::{DeploymentRecord, DeploymentTracker, DeploymentType};
pub use ssh::SshDeploymentClient;
pub use manager_integration::{RemoteDeploymentExtensions, RemoteDeploymentConfig};