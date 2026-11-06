//! SSH-based bare metal deployment
//!
//! This module provides secure SSH deployment to remote hosts with support for
//! multiple container runtimes (Docker, Podman, Containerd) and batch deployments.

mod client;
mod fleet;
mod types;

// Re-export public API
pub use client::SshDeploymentClient;
pub use fleet::BareMetalFleet;
pub use types::{
    ContainerRuntime, DeploymentConfig, HealthCheck, NativeDeployment, RemoteDeployment,
    ResourceLimits, RestartPolicy, SshAuth, SshConnection,
};
