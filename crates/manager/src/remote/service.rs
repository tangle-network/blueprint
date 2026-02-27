//! Remote deployment service integration.

use super::provider_selector::{
    CloudProvider, DeploymentTarget, ProviderPreferences, ProviderSelector, ResourceSpec,
};
use crate::config::BlueprintManagerContext;
use crate::error::{Error, Result};
use crate::rt::ResourceLimits;
use crate::rt::service::Service;
use crate::sources::{BlueprintArgs, BlueprintEnvVars};

#[cfg(feature = "remote-providers")]
use blueprint_remote_providers::{CloudProvisioner, DeploymentTracker, HealthMonitor};

use blueprint_std::collections::HashMap;
use blueprint_std::path::Path;
use blueprint_std::sync::Arc;
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
    /// Health monitor for deployment health checks
    #[cfg(feature = "remote-providers")]
    health_monitor: Option<HealthMonitor>,
    /// Deployment tracker for persistence
    #[cfg(feature = "remote-providers")]
    deployment_tracker: Option<Arc<DeploymentTracker>>,
    /// `QoS` remote metrics provider for collecting metrics from remote instances
    #[cfg(feature = "qos")]
    qos_provider: Option<Arc<blueprint_qos::remote::RemoteMetricsProvider>>,
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

        // Initialize QoS remote metrics provider if feature is enabled
        #[cfg(feature = "qos")]
        let qos_provider = {
            let provider = blueprint_qos::remote::RemoteMetricsProvider::new(100);
            Some(Arc::new(provider))
        };

        #[cfg(feature = "remote-providers")]
        let (health_monitor, deployment_tracker) = {
            // Initialize health monitor and deployment tracker if remote deployer is enabled
            use blueprint_std::env;
            use std::path::PathBuf;

            // Use config path or default
            let tracker_path = env::var("TANGLE_DEPLOYMENT_TRACKER_PATH")
                .map(PathBuf::from)
                .unwrap_or_else(|_| {
                    dirs::home_dir()
                        .unwrap_or_else(|| PathBuf::from("."))
                        .join(".tangle")
                        .join("remote_deployments")
                });

            match (
                CloudProvisioner::new().await,
                DeploymentTracker::new(&tracker_path).await,
            ) {
                (Ok(provisioner), Ok(tracker)) => {
                    let tracker_arc = Arc::new(tracker);
                    let health_monitor = Some(HealthMonitor::new(
                        Arc::new(provisioner),
                        tracker_arc.clone(),
                    ));
                    (health_monitor, Some(tracker_arc))
                }
                _ => (None, None),
            }
        };

        Ok(Self {
            selector,
            deployments: Arc::new(RwLock::new(HashMap::new())),
            policy,
            #[cfg(feature = "remote-providers")]
            health_monitor,
            #[cfg(feature = "remote-providers")]
            deployment_tracker,
            #[cfg(feature = "qos")]
            qos_provider,
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

        // 1. Check if serverless deployment is enabled and recommended
        if let Some(serverless_strategy) = self.should_use_serverless(blueprint_id).await? {
            match serverless_strategy {
                super::DeploymentStrategy::Serverless { job_ids } => {
                    info!("Deploying service '{}' in serverless mode", service_name);
                    return self
                        .deploy_serverless_service(
                            ctx,
                            service_name,
                            binary_path,
                            env_vars,
                            arguments,
                            job_ids,
                        )
                        .await;
                }
                super::DeploymentStrategy::Hybrid {
                    faas_jobs,
                    local_jobs,
                } => {
                    info!(
                        "Deploying service '{}' in hybrid mode ({} FaaS, {} local)",
                        service_name,
                        faas_jobs.len(),
                        local_jobs.len()
                    );
                    // TODO: Implement hybrid deployment
                    // For now, fall through to traditional deployment
                    warn!("Hybrid deployment not yet implemented, using traditional deployment");
                }
                _ => {}
            }
        }

        // 2. Convert Blueprint Manager ResourceLimits to ResourceSpec
        let resource_spec = self.convert_limits_to_spec(&limits)?;

        // 3. Select deployment target
        let target = self
            .selector
            .select_target(&resource_spec)
            .map_err(|e| Error::Other(format!("Provider selection failed: {}", e)))?;

        // 4. Deploy based on target type
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
        info!("🚀 Deploying to cloud provider: {:?}", provider);
        info!("   Service: {}", service_name);
        info!(
            "   Resources: {:.1} CPU, {:.0} GB RAM",
            resource_spec.cpu, resource_spec.memory_gb
        );

        #[cfg(feature = "remote-providers")]
        {
            // Use real cloud provider SDK
            use blueprint_remote_providers::CloudProvisioner;
            use blueprint_remote_providers::core::deployment_target::{
                ContainerRuntime, DeploymentTarget,
            };

            let provisioner = CloudProvisioner::new()
                .await
                .map_err(|e| Error::Other(format!("Failed to create provisioner: {}", e)))?;

            // Get region from policy or use default
            let region = match provider {
                CloudProvider::AWS => "us-east-1".to_string(),
                CloudProvider::GCP => "us-central1".to_string(),
                CloudProvider::Azure => "eastus".to_string(),
                CloudProvider::DigitalOcean => "nyc3".to_string(),
                CloudProvider::Vultr => "ewr".to_string(),
                _ => "default".to_string(),
            };

            info!("   Region: {}", region);

            // Create deployment target for VM with Docker
            let target = DeploymentTarget::VirtualMachine {
                runtime: ContainerRuntime::Docker,
            };

            // Provision the actual instance using the remote providers resource spec directly
            let instance = provisioner
                .provision(
                    convert_provider(provider),
                    &convert_resource_spec(&resource_spec),
                    &region,
                )
                .await
                .map_err(|e| Error::Other(format!("Failed to provision instance: {}", e)))?;

            info!(
                "✅ Instance provisioned: {} at {}",
                instance.id,
                instance.public_ip.as_deref().unwrap_or("unknown")
            );

            // Use binary path directly - container will be created by the adapter
            // The adapter will handle wrapping the binary in a container or systemd service
            let blueprint_image = binary_path.to_string_lossy().to_string();

            // Convert env_vars and arguments to HashMap
            let mut env_map = HashMap::new();
            for (key, value) in env_vars.encode() {
                env_map.insert(key, value);
            }
            let encoded_args = arguments.encode(false);
            for (i, arg) in encoded_args.iter().enumerate() {
                env_map.insert(format!("ARG_{}", i), arg.clone());
            }

            // Deploy blueprint using the adapter's deploy_blueprint_with_target
            // This will handle SSH deployment, Docker setup, and QoS port exposure
            let deployment_result = provisioner
                .deploy_with_target(
                    &target,
                    &blueprint_image,
                    &convert_resource_spec(&resource_spec),
                    env_map,
                )
                .await
                .map_err(|e| Error::Other(format!("Failed to deploy blueprint: {}", e)))?;

            info!("✅ Blueprint deployed with QoS monitoring enabled");

            // Register QoS endpoint for remote metrics collection
            if let Some(qos_endpoint) = deployment_result.qos_grpc_endpoint() {
                info!("📊 QoS endpoint available: {}", &qos_endpoint);

                // Parse host and port from endpoint (format: "http://host:port" or "host:port")
                let endpoint_str = qos_endpoint.replace("http://", "").replace("https://", "");
                if let Some((host, port_str)) = endpoint_str.rsplit_once(':') {
                    if let Ok(port) = port_str.parse::<u16>() {
                        // Register with the QoS remote metrics provider
                        // This allows the QoS system to collect metrics from the remote instance
                        #[cfg(feature = "qos")]
                        if let Some(ref qos_provider) = self.qos_provider {
                            qos_provider
                                .register_remote_instance(
                                    instance.id.clone(),
                                    host.to_string(),
                                    port,
                                )
                                .await;
                            info!("✅ QoS endpoint registered: {}:{}", host, port);
                            info!("   Instance: {}", instance.id);
                            info!("   Blueprint metrics will be collected from port {}", port);
                        }

                        #[cfg(not(feature = "qos"))]
                        {
                            info!(
                                "📊 QoS endpoint ready: {}:{} (QoS feature disabled)",
                                host, port
                            );
                        }
                    }
                }
            }

            // Register deployment
            let deployment_info = RemoteDeploymentInfo {
                instance_id: instance.id.clone(),
                provider,
                service_name: service_name.to_string(),
                blueprint_id,
                deployed_at: chrono::Utc::now(),
                ttl_expires_at: self
                    .policy
                    .auto_terminate_hours
                    .map(|hours| chrono::Utc::now() + chrono::Duration::hours(hours as i64)),
                public_ip: instance.public_ip.clone(),
            };

            {
                let mut deployments = self.deployments.write().await;
                deployments.insert(instance.id.clone(), deployment_info.clone());
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

        #[cfg(not(feature = "remote-providers"))]
        {
            Err(Error::Other(
                "Remote cloud deployment requires the 'remote-providers' feature to be enabled"
                    .into(),
            ))
        }
    }

    async fn deploy_to_kubernetes(
        &self,
        ctx: &BlueprintManagerContext,
        context: &str,
        namespace: &str,
        service_name: &str,
        binary_path: &Path,
        env_vars: BlueprintEnvVars,
        arguments: BlueprintArgs,
        resource_spec: ResourceSpec,
    ) -> Result<Service> {
        info!("🚀 Deploying to Kubernetes cluster");
        info!("   Context: {}", context);
        info!("   Namespace: {}", namespace);
        info!("   Service: {}", service_name);

        #[cfg(feature = "remote-providers")]
        {
            use blueprint_remote_providers::{
                CloudProvisioner, core::deployment_target::DeploymentTarget,
            };

            // Create provisioner
            let provisioner = CloudProvisioner::new()
                .await
                .map_err(|e| Error::Other(format!("Failed to create provisioner: {}", e)))?;

            // Create Kubernetes deployment target
            let target = DeploymentTarget::GenericKubernetes {
                context: Some(context.to_string()),
                namespace: namespace.to_string(),
            };

            // For Kubernetes, we need a container image
            let blueprint_image = format!("ghcr.io/tangle-network/blueprint/{service_name}:latest");

            // Convert env_vars to HashMap
            let mut env_map = HashMap::new();
            for (key, value) in env_vars.encode() {
                env_map.insert(key, value);
            }

            // Add arguments as environment variables
            let encoded_args = arguments.encode(false);
            for (i, arg) in encoded_args.iter().enumerate() {
                env_map.insert(format!("ARG_{}", i), arg.clone());
            }

            // Deploy to Kubernetes using generic adapter
            // Note: This will use kubectl to deploy to any K8s cluster
            let deployment_result = provisioner
                .deploy_with_target(
                    &target,
                    &blueprint_image,
                    &convert_resource_spec(&resource_spec),
                    env_map,
                )
                .await
                .map_err(|e| Error::Other(format!("Failed to deploy to Kubernetes: {}", e)))?;

            info!(
                "✅ Blueprint deployed to Kubernetes: {}",
                deployment_result.blueprint_id
            );

            // Register deployment
            let deployment_info = RemoteDeploymentInfo {
                instance_id: deployment_result.blueprint_id.clone(),
                provider: CloudProvider::Generic, // Generic K8s provider
                service_name: service_name.to_string(),
                blueprint_id: None,
                deployed_at: chrono::Utc::now(),
                ttl_expires_at: self
                    .policy
                    .auto_terminate_hours
                    .map(|hours| chrono::Utc::now() + chrono::Duration::hours(hours as i64)),
                public_ip: deployment_result.instance.public_ip.clone(),
            };

            {
                let mut deployments = self.deployments.write().await;
                deployments.insert(
                    deployment_result.blueprint_id.clone(),
                    deployment_info.clone(),
                );
            }

            info!("✅ Kubernetes deployment registered");

            // Return a service handle
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

        #[cfg(not(feature = "remote-providers"))]
        {
            Err(Error::Other(
                "Kubernetes deployment requires the 'remote-providers' feature to be enabled"
                    .into(),
            ))
        }
    }

    /// Convert Blueprint Manager `ResourceLimits` to `ResourceSpec`.
    fn convert_limits_to_spec(&self, limits: &ResourceLimits) -> Result<ResourceSpec> {
        Ok(ResourceSpec {
            cpu: limits.cpu_count.map_or(2.0, f32::from), // Use actual CPU count or default to 2
            memory_gb: (limits.memory_size as f32) / (1024.0 * 1024.0 * 1024.0), // Convert bytes to GB
            storage_gb: (limits.storage_space as f32) / (1024.0 * 1024.0 * 1024.0), // Convert bytes to GB
            gpu_count: limits.gpu_count.map(u32::from), // Use actual GPU count if specified
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
            #[cfg(feature = "remote-providers")]
            {
                // Use real cloud provider termination
                use blueprint_remote_providers::CloudProvisioner;

                info!(
                    "🚫 Terminating instance {} on provider {:?}",
                    deployment_info.instance_id, deployment_info.provider
                );

                let provisioner = CloudProvisioner::new()
                    .await
                    .map_err(|e| Error::Other(format!("Failed to create provisioner: {}", e)))?;

                // Terminate the instance with the correct provider
                provisioner
                    .terminate(
                        convert_provider(deployment_info.provider),
                        &deployment_info.instance_id,
                    )
                    .await
                    .map_err(|e| Error::Other(format!("Failed to terminate instance: {}", e)))?;

                info!(
                    "✅ Instance {} terminated successfully",
                    deployment_info.instance_id
                );
            }

            #[cfg(not(feature = "remote-providers"))]
            {
                warn!(
                    "Remote deployment termination requires 'remote-providers' feature. Instance {} not terminated.",
                    deployment_info.instance_id
                );
            }
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

    /// Check health of a specific deployment.
    pub async fn check_deployment_health(&self, instance_id: &str) -> Result<bool> {
        #[cfg(feature = "remote-providers")]
        {
            if let Some(ref health_monitor) = self.health_monitor {
                health_monitor
                    .is_healthy(instance_id)
                    .await
                    .map_err(|e| Error::Other(format!("Health check failed: {}", e)))
            } else {
                warn!("Health monitor not available, assuming healthy");
                Ok(true)
            }
        }

        #[cfg(not(feature = "remote-providers"))]
        {
            warn!("Health monitoring requires the 'remote-providers' feature");
            Ok(true)
        }
    }

    /// Get health status of all deployments.
    pub async fn get_all_health_status(&self) -> Result<HashMap<String, bool>> {
        let mut health_status = HashMap::new();

        let deployments = self.deployments.read().await;
        for (instance_id, _) in deployments.iter() {
            let is_healthy = self
                .check_deployment_health(instance_id)
                .await
                .unwrap_or(false);
            health_status.insert(instance_id.clone(), is_healthy);
        }

        Ok(health_status)
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

            // Extract blueprint_id from service name if it follows pattern: blueprint_{id}_service_{id}
            let blueprint_id = service_name
                .split('_')
                .nth(1)
                .and_then(|s| s.parse::<u64>().ok());

            remote_service
                .deploy_service(
                    ctx,
                    service_name,
                    binary_path.as_ref(),
                    env_vars,
                    arguments,
                    limits,
                    blueprint_id,
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

impl RemoteDeploymentService {
    /// Check if serverless deployment should be used for this blueprint.
    async fn should_use_serverless(
        &self,
        blueprint_id: Option<u64>,
    ) -> Result<Option<super::DeploymentStrategy>> {
        let policy = super::load_policy();

        if !policy.serverless.enable {
            return Ok(None);
        }

        let Some(blueprint_id) = blueprint_id else {
            return Ok(None);
        };

        let metadata = super::fetch_blueprint_metadata(blueprint_id, None, None).await?;
        let job_count = metadata.job_count;
        let job_profiles = &metadata.job_profiles;

        let limits = match &policy.serverless.provider {
            super::policy_loader::FaasProviderDef::AwsLambda { .. } => {
                super::FaasLimits::aws_lambda()
            }
            super::policy_loader::FaasProviderDef::GcpFunctions { .. } => {
                super::FaasLimits::gcp_functions()
            }
            super::policy_loader::FaasProviderDef::AzureFunctions { .. } => {
                super::FaasLimits::azure_functions()
            }
            super::policy_loader::FaasProviderDef::Custom { .. } => super::FaasLimits::custom(),
        };

        let analysis = super::analyze_blueprint(job_count, job_profiles, &limits, true);
        Ok(Some(analysis.recommended_strategy))
    }

    /// Deploy service in pure serverless mode.
    async fn deploy_serverless_service(
        &self,
        ctx: &BlueprintManagerContext,
        service_name: &str,
        binary_path: &Path,
        env_vars: BlueprintEnvVars,
        arguments: BlueprintArgs,
        job_ids: Vec<u32>,
    ) -> Result<Service> {
        info!("Deploying serverless service: {}", service_name);

        let policy = super::load_policy();
        let config: super::ServerlessConfig = policy.serverless.into();

        super::deploy_serverless(
            ctx,
            service_name,
            binary_path,
            env_vars,
            arguments,
            job_ids,
            &config,
        )
        .await
    }
}

#[cfg(feature = "remote-providers")]
fn convert_resource_spec(
    spec: &ResourceSpec,
) -> blueprint_remote_providers::resources::ResourceSpec {
    // Direct conversion since both structs now have the same flat structure
    blueprint_remote_providers::resources::ResourceSpec {
        cpu: spec.cpu,
        memory_gb: spec.memory_gb,
        storage_gb: spec.storage_gb,
        gpu_count: spec.gpu_count,
        allow_spot: spec.allow_spot,
        qos: blueprint_remote_providers::resources::QosParameters::default(),
    }
}

#[cfg(feature = "remote-providers")]
fn convert_provider(
    provider: crate::remote::provider_selector::CloudProvider,
) -> blueprint_remote_providers::CloudProvider {
    match provider {
        crate::remote::provider_selector::CloudProvider::AWS => {
            blueprint_remote_providers::CloudProvider::AWS
        }
        crate::remote::provider_selector::CloudProvider::GCP => {
            blueprint_remote_providers::CloudProvider::GCP
        }
        crate::remote::provider_selector::CloudProvider::Azure => {
            blueprint_remote_providers::CloudProvider::Azure
        }
        crate::remote::provider_selector::CloudProvider::DigitalOcean => {
            blueprint_remote_providers::CloudProvider::DigitalOcean
        }
        crate::remote::provider_selector::CloudProvider::Vultr => {
            blueprint_remote_providers::CloudProvider::Vultr
        }
        crate::remote::provider_selector::CloudProvider::Generic => {
            // Fallback for Generic to DigitalOcean or error?
            // Generic usually implies K8s which might not need a specific provider for some ops,
            // but provisioner.provision expects a provider.
            // However, for K8s we use deploy_with_target, not provision.
            // But if we ever need to convert Generic, let's map it to something safe or panic if invalid usage.
            // For now mapping to DigitalOcean as placeholder or we should handle it.
            blueprint_remote_providers::CloudProvider::DigitalOcean
        }
    }
}
