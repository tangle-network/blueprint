//! Shared SSH deployment logic across all cloud providers
//!
//! This module consolidates the near-identical SSH deployment patterns
//! used by all cloud provider adapters to eliminate code duplication.

use crate::core::error::{Error, Result};
use crate::core::resources::ResourceSpec;
use crate::deployment::ssh::{ContainerRuntime, DeploymentConfig, SshConnection, SshDeploymentClient};
use crate::infra::traits::BlueprintDeploymentResult;
use crate::infra::types::ProvisionedInstance;
use std::collections::HashMap;
use tracing::{info, warn};

/// Shared SSH deployment implementation
pub struct SharedSshDeployment;

impl SharedSshDeployment {
    /// Deploy blueprint to any cloud provider instance via SSH
    pub async fn deploy_to_instance(
        instance: &ProvisionedInstance,
        blueprint_image: &str,
        resource_spec: &ResourceSpec,
        env_vars: HashMap<String, String>,
        ssh_config: SshDeploymentConfig,
    ) -> Result<BlueprintDeploymentResult> {
        let public_ip = instance.public_ip.as_ref()
            .ok_or_else(|| Error::Other("Instance has no public IP".into()))?;

        // SSH connection configuration
        let connection = SshConnection {
            host: public_ip.clone(),
            user: ssh_config.username,
            key_path: ssh_config.key_path.map(|p| p.into()),
            port: 22,
            password: None,
            jump_host: None,
        };

        let deployment_config = DeploymentConfig {
            name: format!("blueprint-{}", uuid::Uuid::new_v4()),
            namespace: ssh_config.namespace,
            restart_policy: crate::deployment::ssh::RestartPolicy::OnFailure,
            health_check: None,
        };

        let ssh_client = SshDeploymentClient::new(connection, ContainerRuntime::Docker, deployment_config)
            .await
            .map_err(|e| Error::Other(format!("Failed to establish SSH connection: {e}")))?;

        let deployment = ssh_client
            .deploy_blueprint(blueprint_image, resource_spec, env_vars)
            .await
            .map_err(|e| Error::Other(format!("Blueprint deployment failed: {e}")))?;

        // Extract and validate port mappings
        let mut port_mappings = HashMap::new();
        for (internal_port_str, external_port_str) in &deployment.ports {
            if let (Ok(internal), Ok(external)) = (
                internal_port_str.trim_end_matches("/tcp").parse::<u16>(),
                external_port_str.parse::<u16>(),
            ) {
                port_mappings.insert(internal, external);
            }
        }

        // Verify QoS ports are exposed
        if !port_mappings.contains_key(&9615) {
            warn!("QoS metrics port 9615 not exposed in {} deployment", ssh_config.provider_name);
        }

        let mut metadata = HashMap::new();
        metadata.insert("provider".to_string(), ssh_config.provider_name.clone());
        metadata.insert("container_id".to_string(), deployment.container_id.clone());
        metadata.insert("ssh_host".to_string(), deployment.host.clone());

        // Add provider-specific metadata
        for (key, value) in ssh_config.additional_metadata {
            metadata.insert(key, value);
        }

        info!(
            "Successfully deployed blueprint {} to {} instance {}",
            deployment.container_id, ssh_config.provider_name, instance.id
        );

        Ok(BlueprintDeploymentResult {
            instance: instance.clone(),
            blueprint_id: deployment.container_id,
            port_mappings,
            metadata,
        })
    }
}

/// Configuration for SSH deployment
pub struct SshDeploymentConfig {
    pub username: String,
    pub key_path: Option<String>,
    pub namespace: String,
    pub provider_name: String,
    pub additional_metadata: HashMap<String, String>,
}

impl SshDeploymentConfig {
    /// Create AWS SSH configuration
    pub fn aws() -> Self {
        Self {
            username: "ec2-user".to_string(),
            key_path: std::env::var("AWS_SSH_KEY_PATH").ok(),
            namespace: "blueprint-aws".to_string(),
            provider_name: "aws".to_string(),
            additional_metadata: {
                let mut metadata = HashMap::new();
                metadata.insert("security_hardened".to_string(), "true".to_string());
                metadata
            },
        }
    }

    /// Create GCP SSH configuration
    pub fn gcp(project_id: &str) -> Self {
        Self {
            username: "ubuntu".to_string(),
            key_path: std::env::var("GCP_SSH_KEY_PATH").ok(),
            namespace: "blueprint-gcp".to_string(),
            provider_name: "gcp".to_string(),
            additional_metadata: {
                let mut metadata = HashMap::new();
                metadata.insert("project_id".to_string(), project_id.to_string());
                metadata.insert("security_hardened".to_string(), "true".to_string());
                metadata
            },
        }
    }

    /// Create Azure SSH configuration
    pub fn azure() -> Self {
        Self {
            username: "azureuser".to_string(),
            key_path: std::env::var("AZURE_SSH_KEY_PATH").ok(),
            namespace: "blueprint-azure".to_string(),
            provider_name: "azure-vm".to_string(),
            additional_metadata: HashMap::new(),
        }
    }

    /// Create DigitalOcean SSH configuration
    pub fn digitalocean() -> Self {
        Self {
            username: "root".to_string(),
            key_path: std::env::var("DO_SSH_KEY_PATH").ok(),
            namespace: "blueprint-do".to_string(),
            provider_name: "digitalocean-droplet".to_string(),
            additional_metadata: HashMap::new(),
        }
    }

    /// Create Vultr SSH configuration
    pub fn vultr() -> Self {
        Self {
            username: "root".to_string(),
            key_path: std::env::var("VULTR_SSH_KEY_PATH").ok(),
            namespace: "blueprint-vultr".to_string(),
            provider_name: "vultr-instance".to_string(),
            additional_metadata: HashMap::new(),
        }
    }
}