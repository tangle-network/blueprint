//! GCP CloudProviderAdapter implementation
//!
//! This adapter uses the GCP REST API via reqwest and is always available.

use crate::core::error::{Error, Result};
use crate::core::resources::ResourceSpec;
use crate::infra::traits::{BlueprintDeploymentResult, CloudProviderAdapter};
use crate::infra::types::{InstanceStatus, ProvisionedInstance};
use crate::providers::common::ProvisioningConfig;
use crate::providers::gcp::GcpProvisioner;
use crate::security::auth;
use crate::shared::security::BlueprintSecurityConfig;
use async_trait::async_trait;
use blueprint_core::{info, warn};
use blueprint_std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

/// Professional GCP adapter with security and performance optimizations
pub struct GcpAdapter {
    provisioner: Arc<Mutex<GcpProvisioner>>,
    project_id: String,
    ssh_key_path: Option<String>,
    /// Maps instance IDs to their zones for proper termination
    zone_map: Arc<RwLock<HashMap<String, String>>>,
    /// Default region used when zone lookup fails
    default_region: String,
}

impl GcpAdapter {
    /// Create new GCP adapter with security configuration
    pub async fn new() -> Result<Self> {
        let project_id = std::env::var("GCP_PROJECT_ID")
            .map_err(|_| Error::Other("GCP_PROJECT_ID environment variable not set".into()))?;

        let provisioner = GcpProvisioner::new(project_id.clone()).await?;

        let ssh_key_path = std::env::var("GCP_SSH_KEY_PATH").ok();
        let default_region =
            std::env::var("GCP_DEFAULT_REGION").unwrap_or_else(|_| "us-central1".to_string());

        Ok(Self {
            provisioner: Arc::new(Mutex::new(provisioner)),
            project_id,
            ssh_key_path,
            zone_map: Arc::new(RwLock::new(HashMap::new())),
            default_region,
        })
    }

    /// Create secure firewall rules for blueprint deployment
    async fn ensure_firewall_rules(&self) -> Result<()> {
        let access_token = auth::gcp_access_token().await?;

        let client = crate::create_provider_client(30)?;
        let base_url = format!(
            "https://compute.googleapis.com/compute/v1/projects/{}/global/firewalls",
            self.project_id
        );

        let firewall_rules = Self::build_firewall_rules();

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
                    let status = response.status();
                    let error_text = response.text().await.unwrap_or_default();
                    warn!(
                        "Failed to create firewall rule {}: {} - {}",
                        rule_name, status, error_text
                    );
                }
                Err(e) => {
                    warn!("Failed to create firewall rule {}: {}", rule_name, e);
                }
            }
        }

        Ok(())
    }

    fn build_firewall_rules() -> Vec<serde_json::Value> {
        let security_config = BlueprintSecurityConfig::default();
        let rules = security_config.standard_rules();
        vec![
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
                "sourceRanges": rules
                    .iter()
                    .find(|rule| rule.name == "blueprint-ssh")
                    .map(|rule| rule.source_cidrs.clone())
                    .unwrap_or_else(|| vec!["0.0.0.0/0".to_string()]),
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
                "sourceRanges": rules
                    .iter()
                    .find(|rule| rule.name == "blueprint-qos")
                    .map(|rule| rule.source_cidrs.clone())
                    .unwrap_or_else(|| vec!["0.0.0.0/0".to_string()]),
            }),
        ]
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

        let infra = self
            .provisioner
            .lock()
            .await
            .provision_instance(&spec, &config)
            .await?;

        // Store the zone for later termination/status lookups
        let zone = format!("{}-a", region);
        self.zone_map
            .write()
            .await
            .insert(infra.instance_id.clone(), zone.clone());

        info!(
            "Provisioned GCP instance {} in zone {} (region {})",
            infra.instance_id, zone, region
        );

        Ok(infra.into_provisioned_instance())
    }

    async fn terminate_instance(&self, instance_id: &str) -> Result<()> {
        // Retrieve the zone from our tracking map, falling back to default region
        let zone = self
            .zone_map
            .read()
            .await
            .get(instance_id)
            .cloned()
            .unwrap_or_else(|| format!("{}-a", self.default_region));

        self.provisioner
            .lock()
            .await
            .terminate_instance(instance_id, &zone)
            .await?;

        // Remove from zone map on successful termination
        self.zone_map.write().await.remove(instance_id);

        Ok(())
    }

    async fn get_instance_status(&self, instance_id: &str) -> Result<InstanceStatus> {
        // Retrieve the zone from our tracking map, falling back to default region
        let zone = self
            .zone_map
            .read()
            .await
            .get(instance_id)
            .cloned()
            .unwrap_or_else(|| format!("{}-a", self.default_region));

        let url = format!(
            "https://compute.googleapis.com/compute/v1/projects/{}/zones/{}/instances/{}",
            self.project_id, zone, instance_id
        );

        let access_token = auth::gcp_access_token().await?;

        let client = crate::create_provider_client(30)?;
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
        self.terminate_instance(&deployment.instance.id).await
    }
}

#[cfg(test)]
mod tests {
    use super::GcpAdapter;
    use std::sync::{Mutex, OnceLock};

    fn env_lock() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
    }

    #[test]
    fn firewall_rules_use_configured_cidrs() {
        let _guard = env_lock();
        unsafe {
            std::env::set_var("BLUEPRINT_ALLOWED_SSH_CIDRS", "10.0.0.0/8");
            std::env::set_var("BLUEPRINT_ALLOWED_QOS_CIDRS", "192.168.0.0/16");
        }

        let rules = GcpAdapter::build_firewall_rules();
        let ssh_rule = rules[0]["sourceRanges"].as_array().unwrap();
        let qos_rule = rules[1]["sourceRanges"].as_array().unwrap();
        assert_eq!(ssh_rule[0].as_str(), Some("10.0.0.0/8"));
        assert_eq!(qos_rule[0].as_str(), Some("192.168.0.0/16"));

        unsafe {
            std::env::remove_var("BLUEPRINT_ALLOWED_SSH_CIDRS");
            std::env::remove_var("BLUEPRINT_ALLOWED_QOS_CIDRS");
        }
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
            let _ = (
                cluster_id,
                namespace,
                blueprint_image,
                resource_spec,
                env_vars,
            ); // Suppress unused warnings
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
