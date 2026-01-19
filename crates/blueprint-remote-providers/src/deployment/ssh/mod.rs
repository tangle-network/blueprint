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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::resources::ResourceSpec;

    #[test]
    fn test_ssh_connection_default() {
        let conn = SshConnection::default();
        assert_eq!(conn.host, "localhost");
        assert_eq!(conn.port, 22);
        assert_eq!(conn.user, "root");
    }

    #[test]
    fn test_deployment_config_default() {
        let config = DeploymentConfig::default();
        assert_eq!(config.name, "blueprint-deployment");
        assert_eq!(config.namespace, "default");
    }

    #[test]
    fn test_resource_limits_from_spec() {
        use crate::core::resources::QosParameters;

        let spec = ResourceSpec {
            cpu: 2.0,
            memory_gb: 4.0,
            storage_gb: 100.0,
            allow_spot: false,
            gpu_count: None,
            qos: QosParameters::default(),
        };

        let limits = ResourceLimits::from_spec(&spec);
        assert_eq!(limits.cpu_cores, Some(2.0));
        assert_eq!(limits.memory_mb, Some(4096));
        assert_eq!(limits.disk_gb, Some(100.0));
    }

    #[tokio::test]
    async fn test_bare_metal_fleet_creation() {
        let hosts = vec![
            SshConnection {
                host: "host1.example.com".to_string(),
                ..Default::default()
            },
            SshConnection {
                host: "host2.example.com".to_string(),
                ..Default::default()
            },
        ];

        let fleet = BareMetalFleet::new(hosts.clone());
        assert_eq!(fleet.get_fleet_status().len(), 0); // No deployments yet
    }
}
