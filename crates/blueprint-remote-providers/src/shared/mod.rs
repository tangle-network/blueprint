//! Shared implementations across cloud providers
//!
//! This module contains common patterns and utilities used by multiple
//! cloud provider implementations to reduce code duplication.

pub mod security;
pub mod ssh_deployment;

#[cfg(feature = "kubernetes")]
pub mod kubernetes_deployment;

pub use security::{
    AzureNsgManager, BlueprintSecurityConfig, DigitalOceanFirewallManager, SecurityGroupManager,
    VultrFirewallManager,
};
pub use ssh_deployment::{SharedSshDeployment, SshDeploymentConfig};

#[cfg(feature = "kubernetes")]
pub use kubernetes_deployment::{ManagedK8sConfig, SharedKubernetesDeployment};
