//! Remote deployment service integration.

use super::provider_selector::{
    CloudProvider, DeploymentTarget, ProviderPreferences, ProviderSelector, ResourceSpec,
};
use crate::config::BlueprintManagerContext;
use crate::error::{Error, Result};
use crate::rt::ResourceLimits;
use crate::rt::service::Service;
use crate::sources::{BlueprintArgs, BlueprintEnvVars};

#[cfg(feature = "remote-deployer")]
use blueprint_remote_providers::{
    deployment_tracker::DeploymentTracker,
    infrastructure_unified::UnifiedProvisioner,
    ssh_deployment::SshDeploymentService,
};

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

/// Remote deployment policy loaded from CLI configuration.
#[derive(Debug, Clone)]
pub struct RemoteDeploymentPolicy {
    pub provider_preferences: ProviderPreferences,
    pub max_hourly_cost: Option<f32>,
    pub prefer_spot: bool,
    pub auto_terminate_hours: Option<u32>,
}

impl Default for RemoteDeploymentPolicy {
    fn default() -> Self {
        Self {
            provider_preferences: ProviderPreferences::default(),
            max_hourly_cost: Some(5.0),
            prefer_spot: true,
            auto_terminate_hours: Some(24),
        }
    }
}

/// Remote deployment service for Blueprint Manager.
pub struct RemoteDeploymentService {
    /// Provider selection logic
    selector: ProviderSelector,
    /// Remote deployment registry
    deployments: Arc<RwLock<HashMap<String, RemoteDeploymentInfo>>>,
    /// Deployment policy
    policy: RemoteDeploymentPolicy,
}

/// Information about a remote deployment (simplified for Phase 2).
#[derive(Debug, Clone)]
pub struct RemoteDeploymentInfo {
    pub instance_id: String,
    pub provider: CloudProvider,
    pub service_name: String,
    pub blueprint_id: Option<u64>,
    pub deployed_at: chrono::DateTime<chrono::Utc>,
    pub ttl_expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub public_ip: Option<String>,
}

impl RemoteDeploymentService {
    /// Create new remote deployment service.
    pub async fn new(policy: RemoteDeploymentPolicy) -> Result<Self> {
        let selector = ProviderSelector::new(policy.provider_preferences.clone());

        Ok(Self {
            selector,
            deployments: Arc::new(RwLock::new(HashMap::new())),
            policy,
        })
    }

    /// Create remote deployment service with default policy.
    pub async fn with_default_policy() -> Result<Self> {
        Self::new(RemoteDeploymentPolicy::default()).await
    }

    /// Deploy a service remotely based on resource requirements.
    pub async fn deploy_service(
        &self,
        ctx: &BlueprintManagerContext,
        service_name: &str,
        binary_path: &Path,
        env_vars: BlueprintEnvVars,
        arguments: BlueprintArgs,
        limits: ResourceLimits,
        blueprint_id: Option<u64>,
    ) -> Result<Service> {
        info!("Starting remote deployment for service: {}", service_name);

        // 1. Convert Blueprint Manager ResourceLimits to ResourceSpec
        let resource_spec = self.convert_limits_to_spec(&limits)?;

        // 2. Select deployment target
        let target = self
            .selector
            .select_target(&resource_spec)
            .map_err(|e| Error::Other(format!("Provider selection failed: {}", e)))?;

        // 3. Deploy based on target type
        match target {
            DeploymentTarget::CloudInstance(provider) => {
                self.deploy_to_cloud(
                    ctx,
                    provider,
                    service_name,
                    binary_path,
                    env_vars,
                    arguments,
                    resource_spec,
                    blueprint_id,
                )
                .await
            }
            DeploymentTarget::Kubernetes { context, namespace } => {
                self.deploy_to_kubernetes(
                    ctx,
                    &context,
                    &namespace,
                    service_name,
                    binary_path,
                    env_vars,
                    arguments,
                    resource_spec,
                )
                .await
            }
            DeploymentTarget::Hybrid {
                primary,
                fallback_k8s,
            } => {
                // Try primary provider first, fallback to K8s if it fails
                match self
                    .deploy_to_cloud(
                        ctx,
                        primary,
                        service_name,
                        binary_path,
                        env_vars.clone(),
                        arguments.clone(),
                        resource_spec.clone(),
                        blueprint_id,
                    )
                    .await
                {
                    Ok(service) => Ok(service),
                    Err(e) => {
                        warn!(
                            "Primary provider {} failed: {}, trying K8s fallback",
                            primary, e
                        );
                        self.deploy_to_kubernetes(
                            ctx,
                            &fallback_k8s,
                            "default",
                            service_name,
                            binary_path,
                            env_vars,
                            arguments,
                            resource_spec,
                        )
                        .await
                    }
                }
            }
        }
    }

    async fn deploy_to_cloud(
        &self,
        ctx: &BlueprintManagerContext,
        provider: CloudProvider,
        service_name: &str,
        binary_path: &Path,
        env_vars: BlueprintEnvVars,
        arguments: BlueprintArgs,
        resource_spec: ResourceSpec,
        blueprint_id: Option<u64>,
    ) -> Result<Service> {
        info!(
            "🚀 Deploying to cloud provider: {:?}",
            provider
        );
        info!("   Service: {}", service_name);
        info!(
            "   Resources: {:.1} CPU, {:.0} GB RAM",
            resource_spec.cpu, resource_spec.memory_gb
        );

        #[cfg(feature = "remote-deployer")]
        {
            // Use real cloud provider SDK
            use blueprint_remote_providers::cloud_provisioner::CloudProvisioner;
            use blueprint_remote_providers::ssh_deployment::SshDeploymentService;
            
            let provisioner = CloudProvisioner::new().await
                .map_err(|e| Error::Other(format!("Failed to create provisioner: {}", e)))?;
            
            // Convert resource spec to provider requirements
            let requirements = convert_resource_spec(&resource_spec);
            
            // Provision the actual instance
            let instance = provisioner.provision(
                provider,
                &requirements,
                "us-west-2", // TODO: Get region from config
            ).await
                .map_err(|e| Error::Other(format!("Failed to provision instance: {}", e)))?;
            
            info!("✅ Instance provisioned: {} at {}", instance.instance_id, instance.public_ip);
            
            // Deploy the binary via SSH
            let ssh_service = SshDeploymentService::new();
            ssh_service.deploy(
                &instance.public_ip,
                binary_path,
                service_name,
                env_vars.clone(),
                arguments.clone(),
            ).await
                .map_err(|e| Error::Other(format!("Failed to deploy via SSH: {}", e)))?;
            
            info!("✅ Blueprint binary deployed to remote instance");
            
            // Register deployment
            let deployment_info = RemoteDeploymentInfo {
                instance_id: instance.instance_id.clone(),
                provider,
                service_name: service_name.to_string(),
                blueprint_id,
                deployed_at: chrono::Utc::now(),
                ttl_expires_at: self
                    .policy
                    .auto_terminate_hours
                    .map(|hours| chrono::Utc::now() + chrono::Duration::hours(hours as i64)),
                public_ip: Some(instance.public_ip.clone()),
            };

            {
                let mut deployments = self.deployments.write().await;
                deployments.insert(instance.instance_id.clone(), deployment_info.clone());
            }

            info!("✅ Deployment registered with TTL tracking");
            
            // For now, still create a local service handle
            // In future, this should return a RemoteService handle
            let runtime_dir = ctx.data_dir().join("runtime").join(service_name);
            Service::new_native(
                ctx,
                ResourceLimits::default(),
                runtime_dir,
                service_name,
                binary_path,
                env_vars,
                arguments,
            )
            .await
        }
        
        #[cfg(not(feature = "remote-deployer"))]
        {
            return Err(Error::Other(
                "Remote cloud deployment requires the 'remote-deployer' feature to be enabled".into()
            ));
        }
    }

    async fn deploy_to_kubernetes(
        &self,
        _ctx: &BlueprintManagerContext,
        _context: &str,
        _namespace: &str,
        _service_name: &str,
        _binary_path: &Path,
        _env_vars: BlueprintEnvVars,
        _arguments: BlueprintArgs,
        _resource_spec: ResourceSpec,
    ) -> Result<Service> {
        // TODO: Implement K8s deployment using existing RemoteClusterManager
        warn!("Kubernetes deployment not yet implemented in Phase 2");
        Err(Error::Other(
            "Kubernetes deployment not implemented yet".into(),
        ))
    }

    /// Convert Blueprint Manager ResourceLimits to ResourceSpec.
    fn convert_limits_to_spec(&self, limits: &ResourceLimits) -> Result<ResourceSpec> {
        Ok(ResourceSpec {
            cpu: 4.0, // TODO: Extract CPU from ResourceLimits when available
            memory_gb: (limits.memory_size as f32) / (1024.0 * 1024.0 * 1024.0), // Convert bytes to GB
            storage_gb: (limits.storage_space as f32) / (1024.0 * 1024.0 * 1024.0), // Convert bytes to GB
            gpu_count: None, // TODO: Extract GPU from ResourceLimits when available
            allow_spot: self.policy.prefer_spot,
        })
    }

    /// Get all active remote deployments.
    pub async fn list_deployments(&self) -> HashMap<String, RemoteDeploymentInfo> {
        let deployments = self.deployments.read().await;
        deployments.clone()
    }

    /// Terminate a remote deployment.
    pub async fn terminate_deployment(&self, instance_id: &str) -> Result<()> {
        info!("Terminating remote deployment: {}", instance_id);

        // Remove from our tracking
        let deployment = {
            let mut deployments = self.deployments.write().await;
            deployments.remove(instance_id)
        };

        if let Some(deployment_info) = deployment {
            info!(
                "🚀 Phase 2: Simulating termination of instance: {}",
                deployment_info.instance_id
            );
            info!("   Provider: {:?}", deployment_info.provider);
            info!("✓ Deployment terminated (simulated)");
        } else {
            warn!("Deployment {} not found in registry", instance_id);
        }

        Ok(())
    }

    /// Clean up expired deployments based on TTL.
    pub async fn cleanup_expired_deployments(&self) -> Result<()> {
        let now = chrono::Utc::now();
        let mut expired_instances = Vec::new();

        {
            let deployments = self.deployments.read().await;
            for (instance_id, info) in deployments.iter() {
                if let Some(expires_at) = info.ttl_expires_at {
                    if now > expires_at {
                        expired_instances.push(instance_id.clone());
                    }
                }
            }
        }


        for instance_id in expired_instances {
            info!("Cleaning up expired deployment: {}", instance_id);
            if let Err(e) = self.terminate_deployment(&instance_id).await {
                error!(
                    "Failed to cleanup expired deployment {}: {}",
                    instance_id, e
                );
            }
        }

        Ok(())
    }
}

/// Extension trait for Service to support remote deployment.
pub trait ServiceRemoteExt {
    /// Create a service with remote deployment capability.
    fn new_with_remote(
        ctx: &BlueprintManagerContext,
        limits: ResourceLimits,
        runtime_dir: impl AsRef<Path> + Send,
        service_name: &str,
        binary_path: impl AsRef<Path> + Send,
        env_vars: BlueprintEnvVars,
        arguments: BlueprintArgs,
        remote_policy: Option<RemoteDeploymentPolicy>,
    ) -> impl std::future::Future<Output = Result<Service>> + Send;
}

impl ServiceRemoteExt for Service {
    /// Create a service with optional remote deployment.
    async fn new_with_remote(
        ctx: &BlueprintManagerContext,
        limits: ResourceLimits,
        runtime_dir: impl AsRef<Path> + Send,
        service_name: &str,
        binary_path: impl AsRef<Path> + Send,
        env_vars: BlueprintEnvVars,
        arguments: BlueprintArgs,
        remote_policy: Option<RemoteDeploymentPolicy>,
    ) -> Result<Service> {
        if let Some(policy) = remote_policy {
            info!("Creating service with remote deployment policy");
            let remote_service = RemoteDeploymentService::new(policy).await?;

            remote_service
                .deploy_service(
                    ctx,
                    service_name,
                    binary_path.as_ref(),
                    env_vars,
                    arguments,
                    limits,
                    None, // TODO: Extract blueprint_id from context
                )
                .await
        } else {
            info!("Creating local service (no remote policy)");
            // Fall back to local deployment
            Service::new_native(
                ctx,
                limits,
                runtime_dir,
                service_name,
                binary_path,
                env_vars,
                arguments,
            )
            .await
        }
    }
}


#[cfg(feature = "remote-deployer")]
fn convert_resource_spec(spec: &ResourceSpec) -> blueprint_remote_providers::resources::ResourceSpec {
    blueprint_remote_providers::resources::ResourceSpec {
        compute: blueprint_remote_providers::resources::ComputeResources {
            cpu_count: spec.cpu as u32,
            memory_gb: spec.memory_gb as u32,
            gpu_type: spec.gpu_count.map(|_| "nvidia-t4".to_string()), // Default GPU type
            gpu_count: spec.gpu_count.unwrap_or(0),
        },
        storage: blueprint_remote_providers::resources::StorageResources {
            disk_size_gb: spec.storage_gb as u32,
            disk_type: "gp3".to_string(), // Default to gp3 for AWS
            iops: None,
            throughput_mbps: None,
        },
        network: blueprint_remote_providers::resources::NetworkResources {
            bandwidth_gbps: 1.0,
            public_ip_required: true,
            ipv6_required: false,
        },
    }
}
