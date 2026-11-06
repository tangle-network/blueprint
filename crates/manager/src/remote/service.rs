//! Remote deployment service integration.

mod cloud;
mod kubernetes;

#[cfg(feature = "remote-providers")]
use kubernetes::best_effort_kubernetes_cleanup;

use super::provider_selector::{
    CloudProvider, DeploymentTarget, ProviderPreferences, ProviderSelector, ResourceSpec,
};
use crate::config::BlueprintManagerContext;
use crate::error::{Error, Result};
use crate::rt::ResourceLimits;
#[cfg(feature = "remote-providers")]
use crate::rt::remote::RemoteServiceInstance;
use crate::rt::service::Service;
use crate::sources::{BlueprintArgs, BlueprintEnvVars};

#[cfg(feature = "remote-providers")]
use blueprint_remote_providers::deployment::{
    DeploymentRecord, DeploymentType, RemoteDeploymentConfig,
};
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
    pub tee_required: bool,
    pub tee_backend: Option<String>,
}

impl Default for RemoteDeploymentPolicy {
    fn default() -> Self {
        Self {
            provider_preferences: ProviderPreferences::default(),
            max_hourly_cost: Some(5.0),
            prefer_spot: true,
            auto_terminate_hours: Some(24),
            tee_required: env_bool("BLUEPRINT_REMOTE_TEE_REQUIRED"),
            tee_backend: std::env::var("TEE_BACKEND")
                .ok()
                .and_then(|value| parse_tee_backend(&value)),
        }
    }
}

fn env_bool(name: &str) -> bool {
    std::env::var(name).is_ok_and(|value| {
        matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "1" | "true" | "yes"
        )
    })
}

fn parse_tee_backend(raw: &str) -> Option<String> {
    raw.split(',').find_map(|entry| {
        let trimmed = entry.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TeeAttestationPolicy {
    Structural,
    Cryptographic,
}

impl TeeAttestationPolicy {
    fn from_env() -> Result<Self> {
        let raw = std::env::var("BLUEPRINT_REMOTE_TEE_ATTESTATION_POLICY")
            .unwrap_or_else(|_| "cryptographic".to_string());
        parse_tee_attestation_policy(&raw)
    }
}

fn parse_tee_attestation_policy(raw: &str) -> Result<TeeAttestationPolicy> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "cryptographic" => Ok(TeeAttestationPolicy::Cryptographic),
        "structural" => Ok(TeeAttestationPolicy::Structural),
        other => Err(Error::TeeRuntimeUnavailable {
            reason: format!(
                "Unsupported BLUEPRINT_REMOTE_TEE_ATTESTATION_POLICY value: {other}. Use 'cryptographic' or 'structural'."
            ),
        }),
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
    pub tee_attestation_policy: Option<String>,
    pub tee_attestation_verified_at: Option<chrono::DateTime<chrono::Utc>>,
    pub tee_attestation_proof: Option<String>,
    pub kubernetes_context: Option<String>,
    pub kubernetes_namespace: Option<String>,
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

        let resource_spec = self.convert_limits_to_spec(&limits)?;
        // CPU-only FaaS cannot satisfy mandatory GPU or confidential placement.
        if !matches!(limits.gpu_policy, crate::rt::GpuSchedulingPolicy::Required)
            && !resource_spec.tee_required
            && let Some(serverless_strategy) = self.should_use_serverless(blueprint_id).await?
        {
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

        let target = self
            .selector
            .select_target(&resource_spec)
            .map_err(|e| Error::Other(format!("Provider selection failed: {}", e)))?;

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
                        primary.clone(),
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

    /// Convert Blueprint Manager `ResourceLimits` to `ResourceSpec`.
    fn convert_limits_to_spec(&self, limits: &ResourceLimits) -> Result<ResourceSpec> {
        let mut spec = ResourceSpec::from(limits);
        spec.allow_spot = self.policy.prefer_spot;
        spec.tee_required = self.policy.tee_required;
        spec.validate().map_err(Error::Other)?;
        Ok(spec)
    }

    /// Get all active remote deployments.
    pub async fn list_deployments(&self) -> HashMap<String, RemoteDeploymentInfo> {
        let deployments = self.deployments.read().await;
        deployments.clone()
    }

    /// Terminate a remote deployment.
    pub async fn terminate_deployment(&self, instance_id: &str) -> Result<()> {
        info!("Terminating remote deployment: {}", instance_id);

        let deployment = {
            let deployments = self.deployments.read().await;
            deployments.get(instance_id).cloned()
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

                if deployment_info.provider != CloudProvider::Generic {
                    // Terminate the instance with the correct provider
                    provisioner
                        .terminate(
                            deployment_info.provider.clone(),
                            &deployment_info.instance_id,
                        )
                        .await
                        .map_err(|e| {
                            Error::Other(format!("Failed to terminate instance: {}", e))
                        })?;

                    info!(
                        "✅ Instance {} terminated successfully",
                        deployment_info.instance_id
                    );
                } else {
                    match (
                        deployment_info.kubernetes_context.as_deref(),
                        deployment_info.kubernetes_namespace.as_deref(),
                    ) {
                        (Some(context), Some(namespace)) => {
                            best_effort_kubernetes_cleanup(
                                context,
                                namespace,
                                &deployment_info.instance_id,
                            )
                            .await
                            .map_err(|e| {
                                Error::Other(format!(
                                    "Failed to cleanup Kubernetes deployment {}: {}",
                                    deployment_info.instance_id, e
                                ))
                            })?;
                            info!(
                                "✅ Kubernetes deployment {} cleaned up",
                                deployment_info.instance_id
                            );
                        }
                        _ => {
                            warn!(
                                "Generic deployment {} has no Kubernetes context/namespace metadata; cleanup is best-effort only",
                                deployment_info.instance_id
                            );
                        }
                    }
                }
            }

            #[cfg(not(feature = "remote-providers"))]
            {
                warn!(
                    "Remote deployment termination requires 'remote-providers' feature. Instance {} not terminated.",
                    deployment_info.instance_id
                );
            }

            // Remove from tracking only after successful termination attempt
            let mut deployments = self.deployments.write().await;
            deployments.remove(instance_id);
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
                Err(Error::Other(
                    "Health monitor unavailable for remote deployments".to_string(),
                ))
            }
        }

        #[cfg(not(feature = "remote-providers"))]
        {
            Err(Error::Other(
                "Health monitoring requires the 'remote-providers' feature".to_string(),
            ))
        }
    }

    /// Get health status of all deployments.
    pub async fn get_all_health_status(&self) -> Result<HashMap<String, bool>> {
        let mut health_status = HashMap::new();

        let deployments = self.deployments.read().await;
        for (instance_id, _) in deployments.iter() {
            let is_healthy = self.check_deployment_health(instance_id).await?;
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

impl std::fmt::Display for TeeAttestationPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TeeAttestationPolicy::Structural => write!(f, "structural"),
            TeeAttestationPolicy::Cryptographic => write!(f, "cryptographic"),
        }
    }
}

#[cfg(feature = "remote-providers")]
fn ttl_seconds_from_hours(hours: Option<u32>) -> Option<u64> {
    hours.map(|value| u64::from(value).saturating_mul(3600))
}

#[cfg(feature = "remote-providers")]
async fn ensure_deployment_tracker(
    ctx: &BlueprintManagerContext,
    tracker: Option<Arc<DeploymentTracker>>,
) -> Result<Arc<DeploymentTracker>> {
    if let Some(tracker) = tracker {
        return Ok(tracker);
    }

    let tracker_path = ctx.data_dir().join("remote_deployments");
    DeploymentTracker::new(&tracker_path)
        .await
        .map(Arc::new)
        .map_err(|e| Error::Other(format!("Failed to initialize deployment tracker: {e}")))
}
