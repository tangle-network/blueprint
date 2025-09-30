//! Shared implementations across cloud providers
//!
//! This module contains common patterns and utilities used by multiple
//! cloud provider implementations to reduce code duplication.

pub mod ssh_deployment;
pub mod security;

#[cfg(feature = "kubernetes")]
pub mod kubernetes_deployment;

pub use ssh_deployment::{SharedSshDeployment, SshDeploymentConfig};
pub use security::{BlueprintSecurityConfig, SecurityGroupManager, AzureNsgManager, DigitalOceanFirewallManager, VultrFirewallManager};

#[cfg(feature = "kubernetes")]
pub use kubernetes_deployment::{SharedKubernetesDeployment, ManagedK8sConfig};