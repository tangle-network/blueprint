//! Remote deployment service integration.

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
    std::env::var(name)
        .ok()
        .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes"
            )
        })
        .unwrap_or(false)
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
            "   Resources: {:.1} CPU, {:.0} GB RAM (tee_required={})",
            resource_spec.cpu, resource_spec.memory_gb, resource_spec.tee_required
        );

        if resource_spec.tee_required && !provider.supports_tee() {
            return Err(Error::TeePrerequisiteMissing {
                prerequisite: format!("{provider} confidential-compute support"),
                hint: "Select AWS, GCP, or Azure when tee_required=true".to_string(),
            });
        }

        #[cfg(feature = "remote-providers")]
        {
            // Use real cloud provider SDK
            use blueprint_remote_providers::CloudProvisioner;

            let tracker = ensure_deployment_tracker(ctx, self.deployment_tracker.clone()).await?;
            let tee_attestation_policy = if resource_spec.tee_required {
                Some(TeeAttestationPolicy::from_env()?)
            } else {
                None
            };

            let provisioner = CloudProvisioner::new()
                .await
                .map_err(|e| Error::Other(format!("Failed to create provisioner: {}", e)))?;

            // Resolve region from cloud config when available; otherwise use provider defaults.
            let region = resolve_provider_region(ctx, provider);

            info!("   Region: {}", region);
            let remote_provider = convert_provider(provider)?;

            // Provision the actual instance using the remote providers resource spec directly
            let instance = provisioner
                .provision_with_requirements(
                    remote_provider.clone(),
                    &convert_resource_spec(&resource_spec),
                    &region,
                    resource_spec.tee_required,
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

            if resource_spec.tee_required {
                let backend = self
                    .policy
                    .tee_backend
                    .clone()
                    .or_else(|| provider_default_tee_backend(provider).map(str::to_string))
                    .ok_or_else(|| Error::TeePrerequisiteMissing {
                        prerequisite: "TEE_BACKEND".to_string(),
                        hint: format!(
                            "Set TEE_BACKEND or use a provider with a known default backend. provider={provider}"
                        ),
                    })?;
                env_map.insert("TEE_REQUIRED".to_string(), "true".to_string());
                env_map.entry("TEE_BACKEND".to_string()).or_insert(backend);
            }

            // Deploy to the provisioned instance. This must not provision a second VM.
            let deployment_result = match provisioner
                .deploy_blueprint_to_instance(
                    &remote_provider,
                    &instance,
                    &blueprint_image,
                    &convert_resource_spec(&resource_spec),
                    env_map,
                )
                .await
            {
                Ok(result) => result,
                Err(deployment_error) => {
                    warn!(
                        "Blueprint deployment failed for provider {} (instance={}); attempting best-effort cleanup before returning error: {}",
                        provider, instance.id, deployment_error
                    );
                    if let Err(cleanup_error) = provisioner
                        .terminate(remote_provider.clone(), &instance.id)
                        .await
                    {
                        warn!(
                            "Best-effort cleanup failed after deployment failure (instance={}): {}",
                            instance.id, cleanup_error
                        );
                        return Err(Error::Other(format!(
                            "Failed to deploy blueprint: {}. Cleanup after failed deployment also failed for instance {}: {}",
                            deployment_error, instance.id, cleanup_error
                        )));
                    }
                    info!(
                        "✅ Instance {} terminated after deployment failure",
                        instance.id
                    );
                    return Err(Error::Other(format!(
                        "Failed to deploy blueprint: {}",
                        deployment_error
                    )));
                }
            };

            let tee_attestation_proof = if let Some(policy) = tee_attestation_policy {
                match verify_tee_attestation(policy, provider, &deployment_result).await {
                    Ok(proof) => proof,
                    Err(verification_error) => {
                        warn!(
                            "TEE attestation verification failed for provider {} (instance={}); attempting best-effort cleanup before returning error: {}",
                            provider, instance.id, verification_error
                        );
                        if let Err(cleanup_error) = provisioner
                            .terminate(remote_provider.clone(), &instance.id)
                            .await
                        {
                            warn!(
                                "Best-effort cleanup failed after attestation verification failure (instance={}): {}",
                                instance.id, cleanup_error
                            );
                            return Err(Error::Other(format!(
                                "TEE attestation verification failed: {}. Cleanup after attestation failure also failed for instance {}: {}",
                                verification_error, instance.id, cleanup_error
                            )));
                        } else {
                            info!(
                                "✅ Instance {} terminated after attestation verification failure",
                                instance.id
                            );
                        }
                        return Err(verification_error);
                    }
                }
            } else {
                None
            };
            let tee_attestation_verified_at =
                tee_attestation_policy.and_then(|policy| match policy {
                    TeeAttestationPolicy::Cryptographic if tee_attestation_proof.is_some() => {
                        Some(chrono::Utc::now())
                    }
                    _ => None,
                });

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
                                    deployment_result.instance.id.clone(),
                                    host.to_string(),
                                    port,
                                    blueprint_id.unwrap_or(0),
                                    blueprint_id.unwrap_or(0),
                                )
                                .await;
                            info!("✅ QoS endpoint registered: {}:{}", host, port);
                            info!("   Instance: {}", deployment_result.instance.id);
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
                instance_id: deployment_result.instance.id.clone(),
                provider,
                service_name: service_name.to_string(),
                blueprint_id,
                deployed_at: chrono::Utc::now(),
                ttl_expires_at: self
                    .policy
                    .auto_terminate_hours
                    .map(|hours| chrono::Utc::now() + chrono::Duration::hours(hours as i64)),
                public_ip: deployment_result.instance.public_ip.clone(),
                tee_attestation_policy: tee_attestation_policy.map(|p| p.to_string()),
                tee_attestation_verified_at,
                tee_attestation_proof,
                kubernetes_context: None,
                kubernetes_namespace: None,
            };

            let ttl_seconds = ttl_seconds_from_hours(self.policy.auto_terminate_hours);
            let deployment_key = deployment_result.instance.id.clone();

            let mut deployment_record = DeploymentRecord::new(
                deployment_key.clone(),
                deployment_type_for_provider(&remote_provider),
                convert_resource_spec(&resource_spec),
                ttl_seconds,
            );
            deployment_record.id = deployment_result.instance.id.clone();
            deployment_record.provider = Some(remote_provider.clone());
            deployment_record.region = Some(tracker_region_for_provider(provider, &region));
            deployment_record.resource_ids.insert(
                "instance_id".to_string(),
                deployment_result.instance.id.clone(),
            );
            deployment_record.resource_ids.insert(
                "instance_name".to_string(),
                deployment_result.instance.id.clone(),
            );
            if let Some(public_ip) = &deployment_result.instance.public_ip {
                deployment_record
                    .resource_ids
                    .insert("public_ip".to_string(), public_ip.clone());
            }
            if provider == CloudProvider::GCP {
                if let Ok(project_id) = std::env::var("GCP_PROJECT_ID") {
                    deployment_record
                        .metadata
                        .insert("project_id".to_string(), project_id);
                }
            }
            if let Some(proof) = deployment_info.tee_attestation_proof.clone() {
                deployment_record
                    .metadata
                    .insert("tee_attestation_proof".to_string(), proof);
            }
            if let Err(track_error) = tracker
                .register_deployment(deployment_key.clone(), deployment_record)
                .await
            {
                warn!(
                    "Deployment tracking failed for provider {} (instance={}); attempting best-effort cleanup before returning error: {}",
                    provider, deployment_result.instance.id, track_error
                );
                if let Err(cleanup_error) = provisioner
                    .terminate(remote_provider.clone(), &deployment_result.instance.id)
                    .await
                {
                    return Err(Error::Other(format!(
                        "Failed to track deployment: {}. Cleanup after tracker failure also failed for instance {}: {}",
                        track_error, deployment_result.instance.id, cleanup_error
                    )));
                }
                return Err(Error::Other(format!(
                    "Failed to track deployment: {}",
                    track_error
                )));
            }

            {
                let mut deployments = self.deployments.write().await;
                deployments.insert(
                    deployment_result.instance.id.clone(),
                    deployment_info.clone(),
                );
            }
            info!("✅ Deployment registered with TTL tracking");

            let remote_config = RemoteDeploymentConfig {
                deployment_type: deployment_type_for_provider(&remote_provider),
                provider: Some(remote_provider),
                region: Some(region),
                instance_id: deployment_key,
                resource_spec: convert_resource_spec(&resource_spec),
                ttl_seconds,
                deployed_at: chrono::Utc::now(),
            };
            let remote_instance = RemoteServiceInstance::new(remote_config, tracker);
            let runtime_dir = ctx.data_dir().join("runtime").join(service_name);
            Service::new_remote(ctx, runtime_dir, service_name, remote_instance).await
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
        let _ = binary_path;

        #[cfg(feature = "remote-providers")]
        {
            use blueprint_remote_providers::{
                CloudProvisioner, core::deployment_target::DeploymentTarget,
            };

            let tracker = ensure_deployment_tracker(ctx, self.deployment_tracker.clone()).await?;

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

            // Deploy to Kubernetes using either an inferred provider or a single configured provider.
            let inferred_provider = infer_remote_provider_from_context(context);
            let deployment_result = if let Some(provider) = inferred_provider.clone() {
                provisioner
                    .deploy_with_target_for_provider(
                        &provider,
                        &target,
                        &blueprint_image,
                        &convert_resource_spec(&resource_spec),
                        env_map,
                    )
                    .await
                    .map_err(|e| Error::Other(format!("Failed to deploy to Kubernetes: {}", e)))?
            } else {
                provisioner
                    .deploy_with_target(
                        &target,
                        &blueprint_image,
                        &convert_resource_spec(&resource_spec),
                        env_map,
                    )
                    .await
                    .map_err(|e| Error::Other(format!("Failed to deploy to Kubernetes: {}", e)))?
            };
            let deployment_provider = inferred_provider
                .clone()
                .or_else(|| Some(deployment_result.instance.provider.clone()));

            info!(
                "✅ Blueprint deployed to Kubernetes: {}",
                deployment_result.blueprint_id
            );

            // Register deployment
            let deployment_info = RemoteDeploymentInfo {
                instance_id: deployment_result.blueprint_id.clone(),
                provider: CloudProvider::Generic,
                service_name: service_name.to_string(),
                blueprint_id: None,
                deployed_at: chrono::Utc::now(),
                ttl_expires_at: self
                    .policy
                    .auto_terminate_hours
                    .map(|hours| chrono::Utc::now() + chrono::Duration::hours(hours as i64)),
                public_ip: deployment_result.instance.public_ip.clone(),
                tee_attestation_policy: None,
                tee_attestation_verified_at: None,
                tee_attestation_proof: None,
                kubernetes_context: Some(context.to_string()),
                kubernetes_namespace: Some(namespace.to_string()),
            };

            info!("✅ Kubernetes deployment registered");
            let ttl_seconds = ttl_seconds_from_hours(self.policy.auto_terminate_hours);
            let deployment_key = deployment_result.blueprint_id.clone();

            let mut deployment_record = DeploymentRecord::new(
                deployment_key.clone(),
                DeploymentType::LocalKubernetes,
                convert_resource_spec(&resource_spec),
                ttl_seconds,
            );
            deployment_record.id = deployment_result.blueprint_id.clone();
            if let Some(provider) = deployment_provider.clone() {
                deployment_record.provider = Some(provider);
            }
            deployment_record
                .resource_ids
                .insert("namespace".to_string(), namespace.to_string());
            deployment_record.resource_ids.insert(
                "deployment".to_string(),
                deployment_result.blueprint_id.clone(),
            );
            deployment_record.resource_ids.insert(
                "service".to_string(),
                format!("{}-service", deployment_result.blueprint_id),
            );
            deployment_record
                .resource_ids
                .insert("context".to_string(), context.to_string());
            deployment_record
                .metadata
                .insert("k8s_context".to_string(), context.to_string());
            deployment_record
                .metadata
                .insert("k8s_namespace".to_string(), namespace.to_string());
            if let Err(track_error) = tracker
                .register_deployment(deployment_key.clone(), deployment_record)
                .await
            {
                warn!(
                    "Kubernetes deployment tracking failed (deployment={}); attempting best-effort cleanup before returning error: {}",
                    deployment_result.blueprint_id, track_error
                );
                if let Err(cleanup_error) = best_effort_kubernetes_cleanup(
                    context,
                    namespace,
                    &deployment_result.blueprint_id,
                )
                .await
                {
                    return Err(Error::Other(format!(
                        "Failed to track Kubernetes deployment: {}. Cleanup after tracker failure also failed for deployment {}: {}",
                        track_error, deployment_result.blueprint_id, cleanup_error
                    )));
                }
                return Err(Error::Other(format!(
                    "Failed to track Kubernetes deployment: {}",
                    track_error
                )));
            }

            {
                let mut deployments = self.deployments.write().await;
                deployments.insert(
                    deployment_result.blueprint_id.clone(),
                    deployment_info.clone(),
                );
            }

            let remote_config = RemoteDeploymentConfig {
                deployment_type: DeploymentType::LocalKubernetes,
                provider: deployment_provider,
                region: None,
                instance_id: deployment_key,
                resource_spec: convert_resource_spec(&resource_spec),
                ttl_seconds,
                deployed_at: chrono::Utc::now(),
            };
            let remote_instance = RemoteServiceInstance::new(remote_config, tracker);
            let runtime_dir = ctx.data_dir().join("runtime").join(service_name);
            Service::new_remote(ctx, runtime_dir, service_name, remote_instance).await
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
            tee_required: self.policy.tee_required,
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
                    let remote_provider = convert_provider(deployment_info.provider)?;

                    // Terminate the instance with the correct provider
                    provisioner
                        .terminate(remote_provider, &deployment_info.instance_id)
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

#[cfg(feature = "remote-providers")]
fn convert_resource_spec(
    spec: &ResourceSpec,
) -> blueprint_remote_providers::resources::ResourceSpec {
    // Remote provider spec does not carry TEE policy; TEE policy is passed separately.
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
) -> Result<blueprint_remote_providers::CloudProvider> {
    match provider {
        crate::remote::provider_selector::CloudProvider::AWS => {
            Ok(blueprint_remote_providers::CloudProvider::AWS)
        }
        crate::remote::provider_selector::CloudProvider::GCP => {
            Ok(blueprint_remote_providers::CloudProvider::GCP)
        }
        crate::remote::provider_selector::CloudProvider::Azure => {
            Ok(blueprint_remote_providers::CloudProvider::Azure)
        }
        crate::remote::provider_selector::CloudProvider::DigitalOcean => {
            Ok(blueprint_remote_providers::CloudProvider::DigitalOcean)
        }
        crate::remote::provider_selector::CloudProvider::Vultr => {
            Ok(blueprint_remote_providers::CloudProvider::Vultr)
        }
        crate::remote::provider_selector::CloudProvider::Generic => Err(Error::Other(
            "Generic provider cannot be converted to a provider-specific cloud API operation"
                .to_string(),
        )),
    }
}

fn provider_default_tee_backend(provider: CloudProvider) -> Option<&'static str> {
    match provider {
        CloudProvider::AWS => Some("aws-nitro"),
        CloudProvider::GCP => Some("gcp-confidential"),
        CloudProvider::Azure => Some("azure-skr"),
        _ => None,
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
fn infer_remote_provider_from_context(
    context: &str,
) -> Option<blueprint_remote_providers::CloudProvider> {
    let context = context.to_ascii_lowercase();
    if context.contains("aws") || context.contains("eks") {
        return Some(blueprint_remote_providers::CloudProvider::AWS);
    }
    if context.contains("gcp") || context.contains("gke") || context.contains("google") {
        return Some(blueprint_remote_providers::CloudProvider::GCP);
    }
    if context.contains("azure") || context.contains("aks") {
        return Some(blueprint_remote_providers::CloudProvider::Azure);
    }
    if context.contains("digitalocean") || context.contains("doks") {
        return Some(blueprint_remote_providers::CloudProvider::DigitalOcean);
    }
    if context.contains("vultr") || context.contains("vke") {
        return Some(blueprint_remote_providers::CloudProvider::Vultr);
    }
    None
}

#[cfg(feature = "remote-providers")]
fn resolve_provider_region(ctx: &BlueprintManagerContext, provider: CloudProvider) -> String {
    if let Some(config) = ctx.cloud_config() {
        let configured = match provider {
            CloudProvider::AWS => config.aws.as_ref().map(|c| c.region.clone()),
            CloudProvider::GCP => config.gcp.as_ref().map(|c| c.region.clone()),
            CloudProvider::Azure => config.azure.as_ref().map(|c| c.region.clone()),
            CloudProvider::DigitalOcean => config.digital_ocean.as_ref().map(|c| c.region.clone()),
            CloudProvider::Vultr => config.vultr.as_ref().map(|c| c.region.clone()),
            CloudProvider::Generic => None,
        };
        if let Some(region) = configured {
            return region;
        }
    }

    match provider {
        CloudProvider::AWS => "us-east-1".to_string(),
        CloudProvider::GCP => "us-central1".to_string(),
        CloudProvider::Azure => "eastus".to_string(),
        CloudProvider::DigitalOcean => "nyc3".to_string(),
        CloudProvider::Vultr => "ewr".to_string(),
        CloudProvider::Generic => "default".to_string(),
    }
}

#[cfg(feature = "remote-providers")]
fn deployment_type_for_provider(
    provider: &blueprint_remote_providers::CloudProvider,
) -> DeploymentType {
    match provider {
        blueprint_remote_providers::CloudProvider::AWS => DeploymentType::AwsEc2,
        blueprint_remote_providers::CloudProvider::GCP => DeploymentType::GcpGce,
        blueprint_remote_providers::CloudProvider::Azure => DeploymentType::AzureVm,
        blueprint_remote_providers::CloudProvider::DigitalOcean => {
            DeploymentType::DigitalOceanDroplet
        }
        blueprint_remote_providers::CloudProvider::Vultr => DeploymentType::VultrInstance,
        blueprint_remote_providers::CloudProvider::Linode
        | blueprint_remote_providers::CloudProvider::Generic
        | blueprint_remote_providers::CloudProvider::DockerLocal
        | blueprint_remote_providers::CloudProvider::DockerRemote(_)
        | blueprint_remote_providers::CloudProvider::BareMetal(_) => DeploymentType::SshRemote,
    }
}

#[cfg(feature = "remote-providers")]
fn tracker_region_for_provider(provider: CloudProvider, region: &str) -> String {
    match provider {
        CloudProvider::GCP => format!("{region}-a"),
        _ => region.to_string(),
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

#[cfg(feature = "remote-providers")]
async fn best_effort_kubernetes_cleanup(
    context: &str,
    namespace: &str,
    deployment_id: &str,
) -> std::result::Result<(), String> {
    run_kubectl_delete(context, namespace, "deployment", deployment_id).await?;

    let mut service_errors = Vec::new();
    let compat_service_name = format!("{deployment_id}-service");
    if let Err(error) =
        run_kubectl_delete(context, namespace, "service", &compat_service_name).await
    {
        service_errors.push(error);
    }
    if let Err(error) = run_kubectl_delete(context, namespace, "service", deployment_id).await {
        service_errors.push(error);
    }
    if service_errors.len() == 2 {
        return Err(format!(
            "kubectl cleanup failed for Kubernetes services associated with deployment {}: {}; {}",
            deployment_id, service_errors[0], service_errors[1]
        ));
    }

    Ok(())
}

#[cfg(feature = "remote-providers")]
async fn run_kubectl_delete(
    context: &str,
    namespace: &str,
    resource_kind: &str,
    resource_name: &str,
) -> std::result::Result<(), String> {
    let mut command = tokio::process::Command::new("kubectl");
    if !context.trim().is_empty() {
        command.args(["--context", context]);
    }
    let output = command
        .args([
            "delete",
            resource_kind,
            resource_name,
            "-n",
            namespace,
            "--ignore-not-found=true",
        ])
        .output()
        .await
        .map_err(|error| {
            format!(
                "failed to execute kubectl cleanup for {resource_kind}/{resource_name}: {error}"
            )
        })?;
    if output.status.success() {
        return Ok(());
    }

    Err(format!(
        "kubectl cleanup failed for {resource_kind}/{resource_name}: {}",
        String::from_utf8_lossy(&output.stderr).trim()
    ))
}

#[cfg(feature = "remote-providers")]
async fn verify_tee_attestation(
    policy: TeeAttestationPolicy,
    provider: CloudProvider,
    deployment_result: &blueprint_remote_providers::infra::traits::BlueprintDeploymentResult,
) -> Result<Option<String>> {
    match policy {
        TeeAttestationPolicy::Structural => {
            info!(
                "TEE structural attestation gate satisfied for provider {} (instance={})",
                provider, deployment_result.instance.id
            );
            Ok(None)
        }
        TeeAttestationPolicy::Cryptographic => {
            let command_spec = std::env::var("BLUEPRINT_REMOTE_TEE_ATTESTATION_VERIFY_CMD")
                .map_err(|_| Error::TeeRuntimeUnavailable {
                    reason: "Cryptographic attestation policy requires BLUEPRINT_REMOTE_TEE_ATTESTATION_VERIFY_CMD".to_string(),
                })?;
            let mut parts = command_spec.split_whitespace();
            let executable = parts.next().ok_or_else(|| Error::TeeRuntimeUnavailable {
                reason: "BLUEPRINT_REMOTE_TEE_ATTESTATION_VERIFY_CMD is empty".to_string(),
            })?;

            let metadata_json = serde_json::to_string(&deployment_result.metadata)?;
            let mut cmd = tokio::process::Command::new(executable);
            cmd.args(parts);
            cmd.env("BLUEPRINT_TEE_PROVIDER", provider.to_string());
            cmd.env(
                "BLUEPRINT_TEE_INSTANCE_ID",
                deployment_result.instance.id.clone(),
            );
            cmd.env(
                "BLUEPRINT_TEE_PUBLIC_IP",
                deployment_result
                    .instance
                    .public_ip
                    .clone()
                    .unwrap_or_default(),
            );
            cmd.env(
                "BLUEPRINT_TEE_DEPLOYMENT_ID",
                deployment_result.blueprint_id.clone(),
            );
            cmd.env("BLUEPRINT_TEE_METADATA_JSON", metadata_json);
            if let Some(backend) = provider_default_tee_backend(provider) {
                cmd.env("BLUEPRINT_TEE_BACKEND", backend);
            }

            let output = cmd
                .output()
                .await
                .map_err(|e| Error::TeeRuntimeUnavailable {
                    reason: format!("Failed to execute cryptographic attestation verifier: {e}"),
                })?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
                return Err(Error::TeeRuntimeUnavailable {
                    reason: format!(
                        "Cryptographic attestation verification failed with status {}. {}",
                        output.status,
                        if stderr.is_empty() {
                            "No verifier stderr output provided".to_string()
                        } else {
                            stderr
                        }
                    ),
                });
            }

            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            info!(
                "TEE cryptographic attestation verification succeeded for provider {} (instance={})",
                provider, deployment_result.instance.id
            );
            if stdout.is_empty() {
                Ok(None)
            } else {
                Ok(Some(stdout))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{TeeAttestationPolicy, parse_tee_attestation_policy};

    #[test]
    fn parses_attestation_policy_modes() {
        assert_eq!(
            parse_tee_attestation_policy("cryptographic").unwrap(),
            TeeAttestationPolicy::Cryptographic
        );
        assert_eq!(
            parse_tee_attestation_policy("STRUCTURAL").unwrap(),
            TeeAttestationPolicy::Structural
        );
        assert!(
            parse_tee_attestation_policy("unexpected").is_err(),
            "unknown policy should error"
        );
    }

    #[cfg(feature = "remote-providers")]
    #[test]
    fn generic_provider_conversion_is_rejected() {
        let err = super::convert_provider(super::CloudProvider::Generic).unwrap_err();
        assert!(err.to_string().contains("Generic provider"));
    }
}
