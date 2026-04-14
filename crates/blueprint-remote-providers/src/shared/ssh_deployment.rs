//! Shared SSH deployment logic across all cloud providers
//!
//! This module consolidates the near-identical SSH deployment patterns
//! used by all cloud provider adapters to eliminate code duplication.

use crate::core::error::{Error, Result};
use crate::core::resources::ResourceSpec;
use crate::deployment::ssh::{
    ContainerRuntime, DeploymentConfig, SshConnection, SshDeploymentClient,
};
use crate::infra::traits::BlueprintDeploymentResult;
use crate::infra::types::ProvisionedInstance;
use blueprint_core::{info, warn};
use blueprint_std::collections::HashMap;

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
        let public_ip = instance
            .public_ip
            .as_ref()
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

        let ssh_client =
            SshDeploymentClient::new(connection, ContainerRuntime::Docker, deployment_config)
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
            warn!(
                "QoS metrics port 9615 not exposed in {} deployment",
                ssh_config.provider_name
            );
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

    /// Create Lambda Labs SSH configuration
    pub fn lambda_labs() -> Self {
        Self {
            username: "ubuntu".to_string(),
            key_path: std::env::var("LAMBDA_LABS_SSH_KEY_PATH").ok(),
            namespace: "blueprint-lambda".to_string(),
            provider_name: "lambda-labs".to_string(),
            additional_metadata: HashMap::new(),
        }
    }

    /// Create RunPod SSH configuration
    pub fn runpod() -> Self {
        Self {
            username: "root".to_string(),
            key_path: std::env::var("RUNPOD_SSH_KEY_PATH").ok(),
            namespace: "blueprint-runpod".to_string(),
            provider_name: "runpod".to_string(),
            additional_metadata: HashMap::new(),
        }
    }

    /// Create Vast.ai SSH configuration
    pub fn vast_ai() -> Self {
        Self {
            username: "root".to_string(),
            key_path: std::env::var("VAST_AI_SSH_KEY_PATH").ok(),
            namespace: "blueprint-vastai".to_string(),
            provider_name: "vast-ai".to_string(),
            additional_metadata: HashMap::new(),
        }
    }

    /// Create Hetzner Cloud SSH configuration
    pub fn hetzner() -> Self {
        Self {
            username: "root".to_string(),
            key_path: std::env::var("HETZNER_SSH_KEY_PATH").ok(),
            namespace: "blueprint-hetzner".to_string(),
            provider_name: "hetzner".to_string(),
            additional_metadata: HashMap::new(),
        }
    }

    /// Create Crusoe Cloud SSH configuration
    pub fn crusoe() -> Self {
        Self {
            username: "crusoe".to_string(),
            key_path: std::env::var("CRUSOE_SSH_KEY_PATH").ok(),
            namespace: "blueprint-crusoe".to_string(),
            provider_name: "crusoe".to_string(),
            additional_metadata: HashMap::new(),
        }
    }

    /// Create CoreWeave SSH configuration
    pub fn coreweave() -> Self {
        Self {
            username: "coreweave".to_string(),
            key_path: std::env::var("COREWEAVE_SSH_KEY_PATH").ok(),
            namespace: "blueprint-coreweave".to_string(),
            provider_name: "coreweave".to_string(),
            additional_metadata: HashMap::new(),
        }
    }

    /// Create Paperspace SSH configuration
    pub fn paperspace() -> Self {
        Self {
            username: "paperspace".to_string(),
            key_path: std::env::var("PAPERSPACE_SSH_KEY_PATH").ok(),
            namespace: "blueprint-paperspace".to_string(),
            provider_name: "paperspace".to_string(),
            additional_metadata: HashMap::new(),
        }
    }

    /// Create Fluidstack SSH configuration
    pub fn fluidstack() -> Self {
        Self {
            username: "ubuntu".to_string(),
            key_path: std::env::var("FLUIDSTACK_SSH_KEY_PATH").ok(),
            namespace: "blueprint-fluidstack".to_string(),
            provider_name: "fluidstack".to_string(),
            additional_metadata: HashMap::new(),
        }
    }

    /// Create TensorDock SSH configuration
    pub fn tensordock() -> Self {
        Self {
            username: "user".to_string(),
            key_path: std::env::var("TENSORDOCK_SSH_KEY_PATH").ok(),
            namespace: "blueprint-tensordock".to_string(),
            provider_name: "tensordock".to_string(),
            additional_metadata: HashMap::new(),
        }
    }

    /// Create Akash SSH configuration
    pub fn akash() -> Self {
        Self {
            username: "root".to_string(),
            key_path: std::env::var("AKASH_SSH_KEY_PATH").ok(),
            namespace: "blueprint-akash".to_string(),
            provider_name: "akash".to_string(),
            additional_metadata: HashMap::new(),
        }
    }

    /// Create io.net SSH configuration
    pub fn io_net() -> Self {
        Self {
            username: "root".to_string(),
            key_path: std::env::var("IO_NET_SSH_KEY_PATH").ok(),
            namespace: "blueprint-ionet".to_string(),
            provider_name: "io-net".to_string(),
            additional_metadata: HashMap::new(),
        }
    }

    /// Create Prime Intellect SSH configuration
    pub fn prime_intellect() -> Self {
        Self {
            username: "ubuntu".to_string(),
            key_path: std::env::var("PRIME_INTELLECT_SSH_KEY_PATH").ok(),
            namespace: "blueprint-prime".to_string(),
            provider_name: "prime-intellect".to_string(),
            additional_metadata: HashMap::new(),
        }
    }

    /// Create Render SSH configuration
    pub fn render() -> Self {
        Self {
            username: "render".to_string(),
            key_path: std::env::var("RENDER_SSH_KEY_PATH").ok(),
            namespace: "blueprint-render".to_string(),
            provider_name: "render".to_string(),
            additional_metadata: HashMap::new(),
        }
    }

    /// Create Bittensor/Lium SSH configuration
    pub fn bittensor_lium() -> Self {
        Self {
            username: "root".to_string(),
            key_path: std::env::var("LIUM_SSH_KEY_PATH").ok(),
            namespace: "blueprint-lium".to_string(),
            provider_name: "bittensor-lium".to_string(),
            additional_metadata: HashMap::new(),
        }
    }
}
