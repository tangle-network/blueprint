//! Deployment tracking and lifecycle management
//!
//! Maps Blueprint service instances to their actual deployed infrastructure
//! and handles cleanup when services are terminated or TTL expires.

use crate::error::{Error, Result};
use crate::infrastructure::ProvisionedInfrastructure;
use crate::remote::CloudProvider;
use crate::ssh_deployment::RemoteDeployment;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

/// Global deployment tracker for mapping Blueprint instances to infrastructure
pub struct DeploymentTracker {
    /// Active deployments indexed by Blueprint instance ID
    deployments: Arc<RwLock<HashMap<String, DeploymentRecord>>>,
    /// Persistent state file path
    state_file: PathBuf,
    /// Cleanup handlers for different deployment types
    cleanup_handlers: Arc<RwLock<HashMap<DeploymentType, Box<dyn CleanupHandler>>>>,
}

impl DeploymentTracker {
    /// Create a new deployment tracker
    pub async fn new(state_dir: &Path) -> Result<Self> {
        let state_file = state_dir.join("deployment_state.json");

        // Load existing state if available
        let deployments = if state_file.exists() {
            Self::load_state(&state_file).await?
        } else {
            HashMap::new()
        };

        let mut tracker = Self {
            deployments: Arc::new(RwLock::new(deployments)),
            state_file,
            cleanup_handlers: Arc::new(RwLock::new(HashMap::new())),
        };

        // Register default cleanup handlers
        tracker.register_default_handlers().await;

        Ok(tracker)
    }

    /// Register a new deployment
    pub async fn register_deployment(
        &self,
        blueprint_id: String,
        deployment: DeploymentRecord,
    ) -> Result<()> {
        info!(
            "Registering deployment for Blueprint instance: {}",
            blueprint_id
        );

        let mut deployments = self.deployments.write().await;
        deployments.insert(blueprint_id.clone(), deployment.clone());
        drop(deployments);

        // Persist state
        self.save_state().await?;

        // Schedule TTL check if applicable
        if let Some(ttl) = deployment.ttl_seconds {
            self.schedule_ttl_cleanup(blueprint_id, ttl).await;
        }

        Ok(())
    }

    /// Handle Blueprint termination event
    pub async fn handle_termination(&self, blueprint_id: &str) -> Result<()> {
        info!(
            "Handling termination for Blueprint instance: {}",
            blueprint_id
        );

        let deployments = self.deployments.read().await;
        let deployment = deployments
            .get(blueprint_id)
            .ok_or_else(|| {
                Error::ConfigurationError(format!("No deployment found for {}", blueprint_id))
            })?
            .clone();
        drop(deployments);

        // Perform cleanup
        self.cleanup_deployment(&blueprint_id, &deployment).await?;

        // Remove from tracking
        let mut deployments = self.deployments.write().await;
        deployments.remove(blueprint_id);
        drop(deployments);

        // Update persistent state
        self.save_state().await?;

        Ok(())
    }

    /// Handle TTL expiry
    pub async fn handle_ttl_expiry(&self, blueprint_id: &str) -> Result<()> {
        info!(
            "Handling TTL expiry for Blueprint instance: {}",
            blueprint_id
        );

        let deployments = self.deployments.read().await;
        if let Some(deployment) = deployments.get(blueprint_id) {
            let now = Utc::now();
            if let Some(expiry) = deployment.expires_at {
                if now >= expiry {
                    info!("TTL expired for {}, initiating cleanup", blueprint_id);
                    drop(deployments);
                    return self.handle_termination(blueprint_id).await;
                } else {
                    debug!(
                        "TTL not yet expired for {} (expires at {})",
                        blueprint_id, expiry
                    );
                }
            }
        }

        Ok(())
    }

    /// Cleanup a deployment
    async fn cleanup_deployment(
        &self,
        blueprint_id: &str,
        deployment: &DeploymentRecord,
    ) -> Result<()> {
        info!(
            "Cleaning up deployment: {} (type: {:?})",
            blueprint_id, deployment.deployment_type
        );

        let handlers = self.cleanup_handlers.read().await;
        let handler = handlers.get(&deployment.deployment_type).ok_or_else(|| {
            Error::ConfigurationError(format!("No handler for {:?}", deployment.deployment_type))
        })?;

        // Perform cleanup with retries
        let mut attempts = 0;
        let max_attempts = 3;

        while attempts < max_attempts {
            match handler.cleanup(deployment).await {
                Ok(_) => {
                    info!("Successfully cleaned up deployment: {}", blueprint_id);

                    // Send notification if configured
                    if let Some(webhook) = &deployment.cleanup_webhook {
                        self.send_cleanup_notification(webhook, blueprint_id, "success")
                            .await;
                    }

                    return Ok(());
                }
                Err(e) => {
                    attempts += 1;
                    error!(
                        "Cleanup attempt {} failed for {}: {}",
                        attempts, blueprint_id, e
                    );

                    if attempts >= max_attempts {
                        // Send failure notification
                        if let Some(webhook) = &deployment.cleanup_webhook {
                            self.send_cleanup_notification(webhook, blueprint_id, "failed")
                                .await;
                        }
                        return Err(e);
                    }

                    // Wait before retry
                    tokio::time::sleep(tokio::time::Duration::from_secs(5 * attempts)).await;
                }
            }
        }

        Ok(())
    }

    /// Schedule TTL-based cleanup
    async fn schedule_ttl_cleanup(&self, blueprint_id: String, ttl_seconds: u64) {
        let tracker = self.clone();

        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(ttl_seconds)).await;

            if let Err(e) = tracker.handle_ttl_expiry(&blueprint_id).await {
                error!("Failed to handle TTL expiry for {}: {}", blueprint_id, e);
            }
        });
    }

    /// Register default cleanup handlers
    async fn register_default_handlers(&mut self) {
        let mut handlers = self.cleanup_handlers.write().await;

        // Local deployment handlers
        handlers.insert(DeploymentType::LocalDocker, Box::new(LocalDockerCleanup));
        handlers.insert(
            DeploymentType::LocalKubernetes,
            Box::new(LocalKubernetesCleanup),
        );
        handlers.insert(
            DeploymentType::LocalHypervisor,
            Box::new(LocalHypervisorCleanup),
        );

        // Cloud deployment handlers
        handlers.insert(DeploymentType::AwsEc2, Box::new(AwsCleanup));
        handlers.insert(DeploymentType::GcpGce, Box::new(GcpCleanup));
        handlers.insert(DeploymentType::AzureVm, Box::new(AzureCleanup));
        handlers.insert(
            DeploymentType::DigitalOceanDroplet,
            Box::new(DigitalOceanCleanup),
        );
        handlers.insert(DeploymentType::VultrInstance, Box::new(VultrCleanup));

        // Kubernetes cluster handlers
        handlers.insert(DeploymentType::AwsEks, Box::new(EksCleanup));
        handlers.insert(DeploymentType::GcpGke, Box::new(GkeCleanup));
        handlers.insert(DeploymentType::AzureAks, Box::new(AksCleanup));

        // SSH/Bare metal handler
        handlers.insert(DeploymentType::SshRemote, Box::new(SshCleanup));
    }

    /// Send cleanup notification webhook
    async fn send_cleanup_notification(&self, webhook_url: &str, blueprint_id: &str, status: &str) {
        #[cfg(feature = "api-clients")]
        {
            let client = reqwest::Client::new();
            let body = serde_json::json!({
                "blueprint_id": blueprint_id,
                "event": "cleanup",
                "status": status,
                "timestamp": Utc::now().to_rfc3339(),
            });

            if let Err(e) = client.post(webhook_url).json(&body).send().await {
                warn!("Failed to send cleanup notification: {}", e);
            }
        }
    }

    /// Load state from disk
    async fn load_state(path: &Path) -> Result<HashMap<String, DeploymentRecord>> {
        let content = tokio::fs::read_to_string(path)
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to read state: {}", e)))?;

        serde_json::from_str(&content)
            .map_err(|e| Error::ConfigurationError(format!("Failed to parse state: {}", e)))
    }

    /// Save state to disk
    async fn save_state(&self) -> Result<()> {
        let deployments = self.deployments.read().await;
        let json = serde_json::to_string_pretty(&*deployments)
            .map_err(|e| Error::ConfigurationError(format!("Failed to serialize state: {}", e)))?;

        tokio::fs::write(&self.state_file, json)
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to write state: {}", e)))?;

        Ok(())
    }

    /// Check all deployments for expired TTLs
    pub async fn check_all_ttls(&self) -> Result<()> {
        let deployments = self.deployments.read().await;
        let now = Utc::now();

        let expired: Vec<String> = deployments
            .iter()
            .filter_map(|(id, record)| {
                record
                    .expires_at
                    .filter(|expiry| now >= *expiry)
                    .map(|_| id.clone())
            })
            .collect();

        drop(deployments);

        for blueprint_id in expired {
            if let Err(e) = self.handle_ttl_expiry(&blueprint_id).await {
                error!("Failed to handle TTL expiry for {}: {}", blueprint_id, e);
            }
        }

        Ok(())
    }

    /// Get deployment status
    pub async fn get_deployment_status(&self, blueprint_id: &str) -> Option<DeploymentStatus> {
        let deployments = self.deployments.read().await;
        deployments.get(blueprint_id).map(|d| d.status.clone())
    }

    /// List all active deployments
    pub async fn list_deployments(&self) -> Vec<(String, DeploymentRecord)> {
        let deployments = self.deployments.read().await;
        deployments
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    /// List only active deployments
    pub async fn list_active(&self) -> Result<Vec<DeploymentRecord>> {
        let deployments = self.deployments.read().await;
        Ok(deployments
            .values()
            .filter(|d| d.status == DeploymentStatus::Active)
            .cloned()
            .collect())
    }

    /// Get a specific deployment
    pub async fn get(&self, deployment_id: &str) -> Result<Option<DeploymentRecord>> {
        let deployments = self.deployments.read().await;
        Ok(deployments.get(deployment_id).cloned())
    }

    /// Update instance ID for a deployment (used during recovery)
    pub async fn update_instance_id(&self, old_id: &str, new_id: &str) -> Result<()> {
        let mut deployments = self.deployments.write().await;
        if let Some(mut deployment) = deployments.remove(old_id) {
            deployment
                .resource_ids
                .insert("instance_id".to_string(), new_id.to_string());
            deployments.insert(new_id.to_string(), deployment);
            drop(deployments);
            self.save_state().await?;
        }
        Ok(())
    }
}

impl Clone for DeploymentTracker {
    fn clone(&self) -> Self {
        Self {
            deployments: self.deployments.clone(),
            state_file: self.state_file.clone(),
            cleanup_handlers: self.cleanup_handlers.clone(),
        }
    }
}

/// Deployment record tracking all necessary cleanup information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentRecord {
    /// Unique deployment ID
    pub id: String,
    /// Blueprint instance ID
    pub blueprint_id: String,
    /// Type of deployment
    pub deployment_type: DeploymentType,
    /// Cloud provider (if applicable)
    pub provider: Option<CloudProvider>,
    /// Region/zone
    pub region: Option<String>,
    /// Resource specification
    pub resource_spec: crate::resources::ResourceSpec,
    /// Resource identifiers (instance IDs, container IDs, etc.)
    pub resource_ids: HashMap<String, String>,
    /// Deployment timestamp
    pub deployed_at: DateTime<Utc>,
    /// TTL in seconds (if applicable)
    pub ttl_seconds: Option<u64>,
    /// Expiry timestamp
    pub expires_at: Option<DateTime<Utc>>,
    /// Current status
    pub status: DeploymentStatus,
    /// Cleanup webhook URL (optional)
    pub cleanup_webhook: Option<String>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl DeploymentRecord {
    /// Create a new deployment record
    pub fn new(
        blueprint_id: String,
        deployment_type: DeploymentType,
        resource_spec: crate::resources::ResourceSpec,
        ttl_seconds: Option<u64>,
    ) -> Self {
        let expires_at = ttl_seconds.map(|ttl| Utc::now() + Duration::seconds(ttl as i64));
        let id = format!("dep-{}", uuid::Uuid::new_v4());

        Self {
            id,
            blueprint_id,
            deployment_type,
            provider: None,
            region: None,
            resource_spec,
            resource_ids: HashMap::new(),
            deployed_at: Utc::now(),
            ttl_seconds,
            expires_at,
            status: DeploymentStatus::Active,
            cleanup_webhook: None,
            metadata: HashMap::new(),
        }
    }

    /// Add a resource ID
    pub fn add_resource(&mut self, resource_type: String, resource_id: String) {
        self.resource_ids.insert(resource_type, resource_id);
    }

    /// Set cloud provider information
    pub fn set_cloud_info(&mut self, provider: CloudProvider, region: String) {
        self.provider = Some(provider);
        self.region = Some(region);
    }
}

/// Deployment type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeploymentType {
    // Local deployments
    LocalDocker,
    LocalKubernetes,
    LocalHypervisor,

    // Cloud VMs
    AwsEc2,
    GcpGce,
    AzureVm,
    DigitalOceanDroplet,
    VultrInstance,

    // Kubernetes clusters
    AwsEks,
    GcpGke,
    AzureAks,
    DigitalOceanDoks,
    VultrVke,

    // Other
    SshRemote,
    BareMetal,
}

/// Deployment status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeploymentStatus {
    Active,
    Terminating,
    Terminated,
    Failed,
    Unknown,
}

/// Cleanup handler trait
#[async_trait::async_trait]
pub trait CleanupHandler: Send + Sync {
    /// Perform cleanup for a deployment
    async fn cleanup(&self, deployment: &DeploymentRecord) -> Result<()>;
}

/// Local Docker cleanup
struct LocalDockerCleanup;

#[async_trait::async_trait]
impl CleanupHandler for LocalDockerCleanup {
    async fn cleanup(&self, deployment: &DeploymentRecord) -> Result<()> {
        if let Some(container_id) = deployment.resource_ids.get("container_id") {
            info!("Cleaning up Docker container: {}", container_id);

            let output = tokio::process::Command::new("docker")
                .args(&["rm", "-f", container_id])
                .output()
                .await
                .map_err(|e| Error::ConfigurationError(format!("Docker cleanup failed: {}", e)))?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                if !stderr.contains("No such container") {
                    return Err(Error::ConfigurationError(format!(
                        "Docker rm failed: {}",
                        stderr
                    )));
                }
            }
        }

        Ok(())
    }
}

/// Local Kubernetes cleanup
struct LocalKubernetesCleanup;

#[async_trait::async_trait]
impl CleanupHandler for LocalKubernetesCleanup {
    async fn cleanup(&self, deployment: &DeploymentRecord) -> Result<()> {
        let namespace = deployment
            .resource_ids
            .get("namespace")
            .map(|s| s.as_str())
            .unwrap_or("default");

        if let Some(pod_name) = deployment.resource_ids.get("pod") {
            info!("Cleaning up Kubernetes pod: {}/{}", namespace, pod_name);

            let output = tokio::process::Command::new("kubectl")
                .args(&[
                    "delete",
                    "pod",
                    pod_name,
                    "-n",
                    namespace,
                    "--grace-period=30",
                ])
                .output()
                .await
                .map_err(|e| Error::ConfigurationError(format!("kubectl cleanup failed: {}", e)))?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                if !stderr.contains("NotFound") {
                    return Err(Error::ConfigurationError(format!(
                        "kubectl delete failed: {}",
                        stderr
                    )));
                }
            }
        }

        // Also cleanup any services, configmaps, etc.
        for (resource_type, resource_name) in &deployment.resource_ids {
            if resource_type != "pod" && resource_type != "namespace" {
                let _ = tokio::process::Command::new("kubectl")
                    .args(&["delete", resource_type, resource_name, "-n", namespace])
                    .output()
                    .await;
            }
        }

        Ok(())
    }
}

/// Local Hypervisor cleanup (Cloud Hypervisor)
struct LocalHypervisorCleanup;

#[async_trait::async_trait]
impl CleanupHandler for LocalHypervisorCleanup {
    async fn cleanup(&self, deployment: &DeploymentRecord) -> Result<()> {
        if let Some(vm_id) = deployment.resource_ids.get("vm_id") {
            info!("Cleaning up Cloud Hypervisor VM: {}", vm_id);

            // Send shutdown signal to Cloud Hypervisor API
            if let Some(api_socket) = deployment.resource_ids.get("api_socket") {
                let client = reqwest::Client::new();
                let _ = client
                    .put(&format!("http://localhost/{}/shutdown", api_socket))
                    .send()
                    .await;
            }

            // Kill the process if still running
            if let Some(pid) = deployment.resource_ids.get("pid") {
                if let Ok(pid_num) = pid.parse::<i32>() {
                    unsafe {
                        libc::kill(pid_num, libc::SIGTERM);
                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                        libc::kill(pid_num, libc::SIGKILL);
                    }
                }
            }

            // Clean up disk images and sockets
            if let Some(disk_path) = deployment.resource_ids.get("disk_image") {
                let _ = tokio::fs::remove_file(disk_path).await;
            }
        }

        Ok(())
    }
}

/// AWS cleanup
struct AwsCleanup;

#[async_trait::async_trait]
impl CleanupHandler for AwsCleanup {
    async fn cleanup(&self, deployment: &DeploymentRecord) -> Result<()> {
        #[cfg(feature = "aws")]
        {
            let config = aws_config::load_from_env().await;
            let ec2 = aws_sdk_ec2::Client::new(&config);

            if let Some(instance_id) = deployment.resource_ids.get("instance_id") {
                info!("Terminating AWS EC2 instance: {}", instance_id);

                ec2.terminate_instances()
                    .instance_ids(instance_id)
                    .send()
                    .await
                    .map_err(|e| {
                        Error::ConfigurationError(format!("Failed to terminate EC2: {}", e))
                    })?;
            }

            // Also cleanup associated resources
            if let Some(eip_allocation) = deployment.resource_ids.get("elastic_ip") {
                let _ = ec2
                    .release_address()
                    .allocation_id(eip_allocation)
                    .send()
                    .await;
            }

            if let Some(volume_id) = deployment.resource_ids.get("ebs_volume") {
                // Wait a bit for instance termination
                tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;

                let _ = ec2.delete_volume().volume_id(volume_id).send().await;
            }
        }

        Ok(())
    }
}

/// GCP cleanup
struct GcpCleanup;

#[async_trait::async_trait]
impl CleanupHandler for GcpCleanup {
    async fn cleanup(&self, deployment: &DeploymentRecord) -> Result<()> {
        #[cfg(feature = "gcp")]
        {
            use crate::infrastructure_gcp::GcpInfrastructureProvisioner;

            if let (Some(project), Some(zone)) = (
                deployment.metadata.get("project_id"),
                deployment.region.as_ref(),
            ) {
                let provisioner = GcpInfrastructureProvisioner::new(
                    project.clone(),
                    zone.rsplit_once('-')
                        .map(|r| r.0)
                        .unwrap_or(zone)
                        .to_string(),
                    zone.clone(),
                )
                .await?;

                if let Some(instance_name) = deployment.resource_ids.get("instance_name") {
                    info!("Deleting GCP instance: {}", instance_name);
                    provisioner.delete_gce_instance(instance_name).await?;
                }
            }
        }

        Ok(())
    }
}

/// Azure cleanup
struct AzureCleanup;

#[async_trait::async_trait]
impl CleanupHandler for AzureCleanup {
    async fn cleanup(&self, deployment: &DeploymentRecord) -> Result<()> {
        #[cfg(feature = "azure")]
        {
            use crate::infrastructure_azure::AzureInfrastructureProvisioner;

            if let (Some(subscription), Some(rg)) = (
                deployment.metadata.get("subscription_id"),
                deployment.metadata.get("resource_group"),
            ) {
                let location = deployment.region.as_ref().unwrap_or(&"eastus".to_string());

                let provisioner = AzureInfrastructureProvisioner::new(
                    subscription.clone(),
                    rg.clone(),
                    location.clone(),
                )
                .await?;

                if let Some(vm_name) = deployment.resource_ids.get("vm_name") {
                    info!("Deleting Azure VM: {}", vm_name);
                    provisioner.delete_vm(vm_name).await?;
                }
            }
        }

        Ok(())
    }
}

/// DigitalOcean cleanup
struct DigitalOceanCleanup;

#[async_trait::async_trait]
impl CleanupHandler for DigitalOceanCleanup {
    async fn cleanup(&self, deployment: &DeploymentRecord) -> Result<()> {
        // TODO: Rewrite to use CloudProvisioner
        warn!("DigitalOcean cleanup not yet implemented with CloudProvisioner");
        Ok(())
    }
}

/// Vultr cleanup
struct VultrCleanup;

#[async_trait::async_trait]
impl CleanupHandler for VultrCleanup {
    async fn cleanup(&self, deployment: &DeploymentRecord) -> Result<()> {
        // TODO: Rewrite to use CloudProvisioner
        warn!("Vultr cleanup not yet implemented with CloudProvisioner");
        Ok(())
    }
}

/// EKS cleanup
struct EksCleanup;

#[async_trait::async_trait]
impl CleanupHandler for EksCleanup {
    async fn cleanup(&self, deployment: &DeploymentRecord) -> Result<()> {
        #[cfg(feature = "aws-eks")]
        {
            let config = aws_config::load_from_env().await;
            let eks = aws_sdk_eks::Client::new(&config);

            if let Some(cluster_name) = deployment.resource_ids.get("cluster_name") {
                info!("Deleting EKS cluster: {}", cluster_name);

                // Delete node groups first
                let nodegroups = eks
                    .list_nodegroups()
                    .cluster_name(cluster_name)
                    .send()
                    .await?;

                for ng in nodegroups.nodegroups().unwrap_or_default() {
                    let _ = eks
                        .delete_nodegroup()
                        .cluster_name(cluster_name)
                        .nodegroup_name(ng)
                        .send()
                        .await;
                }

                // Wait for nodegroups to be deleted
                tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;

                // Delete cluster
                eks.delete_cluster()
                    .name(cluster_name)
                    .send()
                    .await
                    .map_err(|e| {
                        Error::ConfigurationError(format!("Failed to delete EKS: {}", e))
                    })?;
            }
        }

        Ok(())
    }
}

/// GKE cleanup
struct GkeCleanup;

#[async_trait::async_trait]
impl CleanupHandler for GkeCleanup {
    async fn cleanup(&self, deployment: &DeploymentRecord) -> Result<()> {
        #[cfg(feature = "gcp")]
        {
            use crate::infrastructure_gcp::GcpInfrastructureProvisioner;

            if let (Some(project), Some(region)) = (
                deployment.metadata.get("project_id"),
                deployment.region.as_ref(),
            ) {
                let provisioner = GcpInfrastructureProvisioner::new(
                    project.clone(),
                    region.clone(),
                    format!("{}-a", region),
                )
                .await?;

                if let Some(cluster_name) = deployment.resource_ids.get("cluster_name") {
                    info!("Deleting GKE cluster: {}", cluster_name);
                    provisioner.delete_gke_cluster(cluster_name).await?;
                }
            }
        }

        Ok(())
    }
}

/// AKS cleanup
struct AksCleanup;

#[async_trait::async_trait]
impl CleanupHandler for AksCleanup {
    async fn cleanup(&self, deployment: &DeploymentRecord) -> Result<()> {
        #[cfg(feature = "azure")]
        {
            use crate::infrastructure_azure::AzureInfrastructureProvisioner;

            if let (Some(subscription), Some(rg)) = (
                deployment.metadata.get("subscription_id"),
                deployment.metadata.get("resource_group"),
            ) {
                let location = deployment.region.as_ref().unwrap_or(&"eastus".to_string());

                let provisioner = AzureInfrastructureProvisioner::new(
                    subscription.clone(),
                    rg.clone(),
                    location.clone(),
                )
                .await?;

                if let Some(cluster_name) = deployment.resource_ids.get("cluster_name") {
                    info!("Deleting AKS cluster: {}", cluster_name);
                    provisioner.delete_aks_cluster(cluster_name).await?;
                }
            }
        }

        Ok(())
    }
}

/// SSH remote cleanup
struct SshCleanup;

#[async_trait::async_trait]
impl CleanupHandler for SshCleanup {
    async fn cleanup(&self, deployment: &DeploymentRecord) -> Result<()> {
        use crate::ssh_deployment::{
            ContainerRuntime, DeploymentConfig, RestartPolicy, SshConnection, SshDeploymentClient,
        };

        if let (Some(host), Some(user)) = (
            deployment.metadata.get("ssh_host"),
            deployment.metadata.get("ssh_user"),
        ) {
            let connection = SshConnection {
                host: host.clone(),
                port: deployment
                    .metadata
                    .get("ssh_port")
                    .and_then(|p| p.parse().ok())
                    .unwrap_or(22),
                user: user.clone(),
                key_path: deployment.metadata.get("ssh_key_path").map(PathBuf::from),
                password: None,
                jump_host: deployment.metadata.get("jump_host").cloned(),
            };

            let runtime = match deployment.metadata.get("runtime").map(|s| s.as_str()) {
                Some("docker") => ContainerRuntime::Docker,
                Some("podman") => ContainerRuntime::Podman,
                _ => ContainerRuntime::Docker,
            };

            let client = SshDeploymentClient::new(
                connection,
                runtime,
                DeploymentConfig {
                    name: deployment.blueprint_id.clone(),
                    namespace: "default".to_string(),
                    restart_policy: RestartPolicy::Never,
                    health_check: None,
                },
            )
            .await?;

            if let Some(container_id) = deployment.resource_ids.get("container_id") {
                info!("Cleaning up remote container: {} on {}", container_id, host);
                client.cleanup_deployment(container_id).await?;
            }
        }

        Ok(())
    }
}

/// Periodic TTL checker task
pub async fn ttl_checker_task(tracker: DeploymentTracker) {
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60));

    loop {
        interval.tick().await;

        if let Err(e) = tracker.check_all_ttls().await {
            error!("TTL check failed: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_deployment_registration() {
        let temp_dir = TempDir::new().unwrap();
        let tracker = DeploymentTracker::new(temp_dir.path()).await.unwrap();

        let mut record = DeploymentRecord::new(
            "blueprint-123".to_string(),
            DeploymentType::LocalDocker,
            Some(3600),
        );
        record.add_resource("container_id".to_string(), "abc123".to_string());

        tracker
            .register_deployment("blueprint-123".to_string(), record)
            .await
            .unwrap();

        let status = tracker.get_deployment_status("blueprint-123").await;
        assert!(matches!(status, Some(DeploymentStatus::Active)));
    }

    #[tokio::test]
    async fn test_ttl_expiry() {
        let temp_dir = TempDir::new().unwrap();
        let tracker = DeploymentTracker::new(temp_dir.path()).await.unwrap();

        let mut record = DeploymentRecord::new(
            "blueprint-ttl".to_string(),
            DeploymentType::LocalDocker,
            Some(0), // Immediate expiry
        );
        record.expires_at = Some(Utc::now() - Duration::seconds(1));
        record.add_resource("container_id".to_string(), "expired123".to_string());

        tracker
            .register_deployment("blueprint-ttl".to_string(), record)
            .await
            .unwrap();

        // Check TTLs
        tracker.check_all_ttls().await.unwrap();

        // Should be cleaned up
        let status = tracker.get_deployment_status("blueprint-ttl").await;
        assert!(status.is_none());
    }
}
