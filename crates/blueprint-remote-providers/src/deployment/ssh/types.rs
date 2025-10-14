//! Type definitions for SSH deployment

use crate::core::resources::ResourceSpec;
use blueprint_std::{collections::HashMap, path::PathBuf};
use serde::{Deserialize, Serialize};

/// SSH authentication method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SshAuth {
    /// SSH key authentication
    Key(String),
    /// Password authentication
    Password(String),
}

/// SSH connection parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshConnection {
    /// Hostname or IP address
    pub host: String,
    /// SSH port (default: 22)
    pub port: u16,
    /// SSH username
    pub user: String,
    /// Path to SSH private key
    pub key_path: Option<PathBuf>,
    /// SSH password (not recommended)
    pub password: Option<String>,
    /// Jump host for bastion access
    pub jump_host: Option<String>,
}

impl Default for SshConnection {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 22,
            user: "root".to_string(),
            key_path: None,
            password: None,
            jump_host: None,
        }
    }
}

/// Container runtime type on remote host
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContainerRuntime {
    Docker,
    Podman,
    Containerd,
}

/// Deployment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentConfig {
    /// Deployment name
    pub name: String,
    /// Deployment namespace/project
    pub namespace: String,
    /// Auto-restart policy
    pub restart_policy: RestartPolicy,
    /// Health check configuration
    pub health_check: Option<HealthCheck>,
}

/// Container restart policy
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum RestartPolicy {
    Always,
    #[default]
    OnFailure,
    Never,
}

impl Default for DeploymentConfig {
    fn default() -> Self {
        Self {
            name: "blueprint-deployment".to_string(),
            namespace: "default".to_string(),
            restart_policy: RestartPolicy::default(),
            health_check: None,
        }
    }
}

/// Health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    pub command: String,
    pub interval: u32,
    pub timeout: u32,
    pub retries: u32,
}

/// Resource limits for container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub cpu_cores: Option<f64>,
    pub memory_mb: Option<u64>,
    pub disk_gb: Option<f64>,
    pub network_bandwidth_mbps: Option<u32>,
}

impl ResourceLimits {
    pub(super) fn from_spec(spec: &ResourceSpec) -> Self {
        Self {
            cpu_cores: Some(spec.cpu as f64),
            memory_mb: Some((spec.memory_gb * 1024.0) as u64),
            disk_gb: Some(spec.storage_gb as f64),
            network_bandwidth_mbps: Some(1000), // Default 1Gbps
        }
    }
}

/// Container details (internal use)
pub(super) struct ContainerDetails {
    pub(super) status: String,
    pub(super) ports: HashMap<String, String>,
}

/// Remote deployment information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteDeployment {
    pub host: String,
    pub container_id: String,
    pub runtime: ContainerRuntime,
    pub status: String,
    pub ports: HashMap<String, String>,
    pub resource_limits: ResourceLimits,
}

/// Native (non-containerized) deployment information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativeDeployment {
    pub host: String,
    pub service_name: String,
    pub config_path: String,
    pub status: String,
}
