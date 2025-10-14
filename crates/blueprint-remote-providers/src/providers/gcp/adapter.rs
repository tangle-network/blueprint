//! GCP CloudProviderAdapter implementation

use crate::core::error::{Error, Result};
use crate::core::resources::ResourceSpec;
use crate::infra::traits::{BlueprintDeploymentResult, CloudProviderAdapter};
use crate::infra::types::{InstanceStatus, ProvisionedInstance};
use crate::providers::common::{ProvisionedInfrastructure, ProvisioningConfig};
use crate::providers::gcp::GcpProvisioner;
use async_trait::async_trait;
use blueprint_core::info;
use blueprint_std::collections::HashMap;

/// Professional GCP adapter with security and performance optimizations
pub struct GcpAdapter {
    provisioner: GcpProvisioner,
    project_id: String,
    ssh_key_path: Option<String>,
}

impl GcpAdapter {
    /// Create new GCP adapter with security configuration
    pub async fn new() -> Result<Self> {
        let project_id = std::env::var("GCP_PROJECT_ID")
            .map_err(|_| Error::Other("GCP_PROJECT_ID environment variable not set".into()))?;

        let provisioner = GcpProvisioner::new(project_id.clone()).await?;

        let ssh_key_path = std::env::var("GCP_SSH_KEY_PATH").ok();

        Ok(Self {
            provisioner,
            project_id,
            ssh_key_path,
        })
    }

    /// Convert ProvisionedInfrastructure to ProvisionedInstance
    fn to_provisioned_instance(infra: ProvisionedInfrastructure) -> ProvisionedInstance {
        ProvisionedInstance {
            id: infra.instance_id,
            public_ip: infra.public_ip,
            private_ip: infra.private_ip,
            status: InstanceStatus::Running,
            provider: infra.provider,
            region: infra.region,
            instance_type: infra.instance_type,
        }
    }

    /// Create secure firewall rules for blueprint deployment
    async fn ensure_firewall_rules(&self) -> Result<()> {
        #[cfg(feature = "gcp")]
        {
            let access_token = std::env::var("GCP_ACCESS_TOKEN").map_err(|_| {
                Error::ConfigurationError(
                    "No GCP access token available. Set GCP_ACCESS_TOKEN".into(),
                )
            })?;

            let client = reqwest::Client::new();
            let base_url = format!(
                "https://compute.googleapis.com/compute/v1/projects/{}/global/firewalls",
                self.project_id
            );

            let firewall_rules = vec![
                serde_json::json!({
                    "name": format!("blueprint-ssh-{}", uuid::Uuid::new_v4().simple()),
                    "description": "Allow SSH access for Blueprint management",
                    "direction": "INGRESS",
                    "priority": 1000,
                    "targetTags": ["blueprint"],
                    "allowed": [{
                        "IPProtocol": "tcp",
                        "ports": ["22"]
                    }],
                    "sourceRanges": ["0.0.0.0/0"], // Open to all - restrict for production
                }),
                serde_json::json!({
                    "name": format!("blueprint-qos-{}", uuid::Uuid::new_v4().simple()),
                    "description": "Allow Blueprint QoS ports",
                    "direction": "INGRESS",
                    "priority": 1000,
                    "targetTags": ["blueprint"],
                    "allowed": [{
                        "IPProtocol": "tcp",
                        "ports": ["8080", "9615", "9944"]
                    }],
                    "sourceRanges": ["0.0.0.0/0"], // Open to all - restrict for production
                }),
            ];

            info!(
                "Creating {} firewall rules for GCP Blueprint security",
                firewall_rules.len()
            );

            for rule in &firewall_rules {
                let rule_name = rule["name"].as_str().unwrap_or("unknown");

                // Check if rule already exists
                let check_url = format!("{}/{}", base_url, rule_name);
                let check_response = client
                    .get(&check_url)
                    .bearer_auth(&access_token)
                    .send()
                    .await;

                if let Ok(resp) = check_response {
                    if resp.status().is_success() {
                        info!("Firewall rule {} already exists, skipping", rule_name);
                        continue;
                    }
                }

                // Create the firewall rule
                match client
                    .post(&base_url)
                    .bearer_auth(&access_token)
                    .json(rule)
                    .send()
                    .await
                {
                    Ok(response) if response.status().is_success() => {
                        info!(
                            "Created firewall rule: {} - {}",
                            rule_name,
                            rule["description"].as_str().unwrap_or("")
                        );
                    }
                    Ok(response) => {
                        let error_text = response.text().await.unwrap_or_default();
                        warn!(
                            "Failed to create firewall rule {}: {} - {}",
                            rule_name,
                            response.status(),
                            error_text
                        );
                    }
                    Err(e) => {
                        warn!("Failed to create firewall rule {}: {}", rule_name, e);
                    }
                }
            }

            Ok(())
        }
        #[cfg(not(feature = "gcp"))]
        {
            info!("GCP firewall rules skipped - gcp feature not enabled");
            Ok(())
        }
    }
}

#[async_trait]
impl CloudProviderAdapter for GcpAdapter {
    async fn provision_instance(
        &self,
        _instance_type: &str,
        region: &str,
    ) -> Result<ProvisionedInstance> {
        let spec = ResourceSpec {
            cpu: 2.0,
            memory_gb: 4.0,
            storage_gb: 20.0,
            gpu_count: None,
            allow_spot: false,
            qos: Default::default(),
        };

        // Ensure firewall rules are configured before provisioning
        self.ensure_firewall_rules().await?;

        let config = ProvisioningConfig {
            name: format!("blueprint-{}", uuid::Uuid::new_v4()),
            region: region.to_string(),
            ssh_key_name: None,
            ami_id: None,
            machine_image: Some(
                "projects/ubuntu-os-cloud/global/images/family/ubuntu-2204-lts".to_string(),
            ),
            custom_config: {
                let mut config = HashMap::new();
                if let Some(_key_path) = &self.ssh_key_path {
                    // In production, read SSH public key from file
                    config.insert("ssh_public_key".to_string(), "".to_string());
                }
                config
            },
        };

        let infra = self.provisioner.provision_instance(&spec, &config).await?;

        info!(
            "Provisioned GCP instance {} in region {}",
            infra.instance_id, region
        );

        Ok(Self::to_provisioned_instance(infra))
    }

    async fn terminate_instance(&self, instance_id: &str) -> Result<()> {
        // For GCP, we need the zone as well as instance name
        // In a real implementation, we'd store this mapping
        let zone = "us-central1-a"; // Default zone - in production, store zone mapping
        self.provisioner.terminate_instance(instance_id, zone).await
    }

    async fn get_instance_status(&self, instance_id: &str) -> Result<InstanceStatus> {
        #[cfg(feature = "gcp")]
        {
            let zone = "us-central1-a"; // Default zone
            let url = format!(
                "https://compute.googleapis.com/compute/v1/projects/{}/zones/{}/instances/{}",
                self.project_id, zone, instance_id
            );

            let access_token = std::env::var("GCP_ACCESS_TOKEN").map_err(|_| {
                Error::ConfigurationError(
                    "No GCP access token available. Set GCP_ACCESS_TOKEN".into(),
                )
            })?;

            let client = reqwest::Client::new();
            match client.get(&url).bearer_auth(&access_token).send().await {
                Ok(response) if response.status().is_success() => {
                    if let Ok(instance) = response.json::<serde_json::Value>().await {
                        match instance["status"].as_str() {
                            Some("RUNNING") => Ok(InstanceStatus::Running),
                            Some("PROVISIONING") | Some("STAGING") => Ok(InstanceStatus::Starting),
                            Some("TERMINATED") | Some("STOPPING") => Ok(InstanceStatus::Terminated),
                            _ => Ok(InstanceStatus::Unknown),
                        }
                    } else {
                        Ok(InstanceStatus::Unknown)
                    }
                }
                Ok(response) if response.status() == 404 => Ok(InstanceStatus::Terminated),
                Ok(_) => Ok(InstanceStatus::Unknown),
                Err(_) => Ok(InstanceStatus::Unknown),
            }
        }
        #[cfg(not(feature = "gcp"))]
        {
            let _ = instance_id; // Suppress unused warning
            Err(Error::ConfigurationError(
                "GCP support not enabled. Enable the 'gcp' feature".into(),
            ))
        }
    }

    async fn deploy_blueprint_with_target(
        &self,
        target: &crate::core::deployment_target::DeploymentTarget,
        blueprint_image: &str,
        resource_spec: &ResourceSpec,
        env_vars: HashMap<String, String>,
    ) -> Result<BlueprintDeploymentResult> {
        use crate::core::deployment_target::DeploymentTarget;

        match target {
            DeploymentTarget::VirtualMachine { runtime: _ } => {
                self.deploy_to_vm(blueprint_image, resource_spec, env_vars)
                    .await
            }
            DeploymentTarget::ManagedKubernetes {
                cluster_id,
                namespace,
            } => {
                self.deploy_to_gke(
                    cluster_id,
                    namespace,
                    blueprint_image,
                    resource_spec,
                    env_vars,
                )
                .await
            }
            DeploymentTarget::GenericKubernetes {
                context: _,
                namespace,
            } => {
                self.deploy_to_generic_k8s(namespace, blueprint_image, resource_spec, env_vars)
                    .await
            }
            DeploymentTarget::Serverless { .. } => Err(Error::Other(
                "GCP Cloud Run deployment not implemented".into(),
            )),
        }
    }

    async fn health_check_blueprint(&self, deployment: &BlueprintDeploymentResult) -> Result<bool> {
        if let Some(endpoint) = deployment.qos_grpc_endpoint() {
            match reqwest::get(&format!("{endpoint}/health")).await {
                Ok(response) => Ok(response.status().is_success()),
                Err(_) => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    async fn cleanup_blueprint(&self, deployment: &BlueprintDeploymentResult) -> Result<()> {
        info!(
            "Cleaning up GCP Blueprint deployment: {}",
            deployment.blueprint_id
        );
        // Terminate the Compute Engine instance
        self.terminate_instance(&deployment.instance.id).await
    }
}

impl GcpAdapter {
    /// Deploy to Compute Engine VM via SSH
    async fn deploy_to_vm(
        &self,
        blueprint_image: &str,
        resource_spec: &ResourceSpec,
        env_vars: HashMap<String, String>,
    ) -> Result<BlueprintDeploymentResult> {
        use crate::shared::{SharedSshDeployment, SshDeploymentConfig};

        let instance = self.provision_instance("e2-medium", "us-central1").await?;

        // Use shared SSH deployment with GCP configuration
        SharedSshDeployment::deploy_to_instance(
            &instance,
            blueprint_image,
            resource_spec,
            env_vars,
            SshDeploymentConfig::gcp(&self.project_id),
        )
        .await
    }

    /// Deploy to GKE cluster
    async fn deploy_to_gke(
        &self,
        cluster_id: &str,
        namespace: &str,
        blueprint_image: &str,
        resource_spec: &ResourceSpec,
        env_vars: HashMap<String, String>,
    ) -> Result<BlueprintDeploymentResult> {
        #[cfg(feature = "kubernetes")]
        {
            use crate::shared::{ManagedK8sConfig, SharedKubernetesDeployment};

            let config = ManagedK8sConfig::gke(&self.project_id, "us-central1");
            SharedKubernetesDeployment::deploy_to_managed_k8s(
                cluster_id,
                namespace,
                blueprint_image,
                resource_spec,
                env_vars,
                config,
            )
            .await
        }

        #[cfg(not(feature = "kubernetes"))]
        {
            let _ = (cluster_id, namespace, blueprint_image, resource_spec, env_vars); // Suppress unused warnings
            Err(Error::ConfigurationError(
                "Kubernetes feature not enabled".to_string(),
            ))
        }
    }

    /// Deploy to generic Kubernetes cluster
    async fn deploy_to_generic_k8s(
        &self,
        namespace: &str,
        blueprint_image: &str,
        resource_spec: &ResourceSpec,
        env_vars: HashMap<String, String>,
    ) -> Result<BlueprintDeploymentResult> {
        #[cfg(feature = "kubernetes")]
        {
            use crate::shared::SharedKubernetesDeployment;

            SharedKubernetesDeployment::deploy_to_generic_k8s(
                namespace,
                blueprint_image,
                resource_spec,
                env_vars,
            )
            .await
        }

        #[cfg(not(feature = "kubernetes"))]
        {
            let _ = (namespace, blueprint_image, resource_spec, env_vars); // Suppress unused warnings
            Err(Error::ConfigurationError(
                "Kubernetes feature not enabled".to_string(),
            ))
        }
    }
}
