//! Deployment orchestration and tracking

pub mod error_recovery;
pub mod manager_integration;
pub mod qos_tunnel;
pub mod secure_commands;
pub mod secure_ssh;
pub mod ssh;
pub mod tracker;
pub mod update_manager;

#[cfg(feature = "kubernetes")]
pub mod kubernetes;

pub use error_recovery::{
    CircuitBreaker, DeploymentTransaction, ErrorRecovery, RecoveryStrategy, SshConnectionRecovery,
};
pub use manager_integration::{RemoteDeploymentConfig, RemoteDeploymentExtensions};
pub use qos_tunnel::{QosTunnel, QosTunnelManager};
pub use ssh::SshDeploymentClient;
pub use tracker::{DeploymentRecord, DeploymentTracker, DeploymentType};
pub use update_manager::{DeploymentVersion, UpdateManager, UpdateStrategy};

#[cfg(feature = "kubernetes")]
pub use kubernetes::KubernetesDeploymentClient;
