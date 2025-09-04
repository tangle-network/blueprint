//! Remote deployment service integration.

use crate::config::BlueprintManagerContext;
use crate::error::{Error, Result};
use crate::rt::ResourceLimits;
use crate::rt::service::Service;
use crate::sources::{BlueprintArgs, BlueprintEnvVars};
use super::provider_selector::{ProviderSelector, DeploymentTarget, ProviderPreferences, CloudProvider, ResourceSpec};

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error};

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
        let target = self.selector.select_target(&resource_spec)
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
                ).await
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
                ).await
            }
            DeploymentTarget::Hybrid { primary, fallback_k8s } => {
                // Try primary provider first, fallback to K8s if it fails
                match self.deploy_to_cloud(
                    ctx, primary, service_name, binary_path, env_vars.clone(),
                    arguments.clone(), resource_spec.clone(), blueprint_id,
                ).await {
                    Ok(service) => Ok(service),
                    Err(e) => {
                        warn!("Primary provider {} failed: {}, trying K8s fallback", primary, e);
                        self.deploy_to_kubernetes(
                            ctx, &fallback_k8s, "default", service_name,
                            binary_path, env_vars, arguments, resource_spec,
                        ).await
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
        info!("🚀 Phase 2: Simulating deployment to cloud provider: {:?}", provider);
        info!("   Service: {}", service_name);
        info!("   Resources: {:.1} CPU, {:.0} GB RAM", resource_spec.cpu, resource_spec.memory_gb);
        
        // 1. Simulate instance provisioning
        let instance_id = format!("sim-{}-{}", 
            provider.to_string().to_lowercase().replace(' ', "-"),
            uuid::Uuid::new_v4().to_string()[0..8].to_string()
        );
        let mock_ip = format!("10.{}.{}.{}", 
            rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>());
        
        info!("✓ Simulated instance: {} at {}", instance_id, mock_ip);
        
        // 2. Register simulated deployment
        let deployment_info = RemoteDeploymentInfo {
            instance_id: instance_id.clone(),
            provider,
            service_name: service_name.to_string(),
            blueprint_id,
            deployed_at: chrono::Utc::now(),
            ttl_expires_at: self.policy.auto_terminate_hours.map(|hours| {
                chrono::Utc::now() + chrono::Duration::hours(hours as i64)
            }),
            public_ip: Some(mock_ip),
        };
        
        {
            let mut deployments = self.deployments.write().await;
            deployments.insert(instance_id.clone(), deployment_info);
        }
        
        info!("✓ Deployment registered with TTL tracking");
        info!("⚠️  Phase 2: Creating local service (remote bridge not yet implemented)");
        
        // Create local service for now (Phase 2 limitation)
        let runtime_dir = ctx.data_dir().join("runtime").join(service_name);
        Service::new_native(
            ctx,
            ResourceLimits::default(),
            runtime_dir,
            service_name,
            binary_path,
            env_vars,
            arguments,
        ).await
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
        Err(Error::Other("Kubernetes deployment not implemented yet".into()))
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
            info!("🚀 Phase 2: Simulating termination of instance: {}", deployment_info.instance_id);
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
                error!("Failed to cleanup expired deployment {}: {}", instance_id, e);
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
            
            remote_service.deploy_service(
                ctx,
                service_name,
                binary_path.as_ref(),
                env_vars,
                arguments,
                limits,
                None, // TODO: Extract blueprint_id from context
            ).await
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
            ).await
        }
    }
}