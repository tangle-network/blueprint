//! Remote provider integration for Blueprint Manager
//!
//! Handles automatic cloud deployment when services are initiated

use crate::config::BlueprintManagerContext;
use crate::error::{Error, Result};
use blueprint_core::{info, warn};
use blueprint_remote_providers::deployment::DeploymentType;
use blueprint_remote_providers::deployment::manager_integration::{
    RemoteDeploymentConfig, RemoteDeploymentRegistry,
};
use blueprint_remote_providers::managed::{Lease, ManagedLeaseIdentity, RetireReport};
use blueprint_remote_providers::{
    CloudProvider, CloudProvisioner, DeploymentTracker, ResourceSpec,
};
use blueprint_std::collections::HashMap;
use blueprint_std::sync::Arc;
use chrono::Utc;
use std::time::Duration;

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

fn supports_tee(provider: &CloudProvider) -> bool {
    matches!(
        provider,
        CloudProvider::AWS | CloudProvider::GCP | CloudProvider::Azure
    )
}

/// What to deploy onto a provisioned VM.
///
/// Container images go through `docker pull` + `docker run`.
/// GitHub-release binaries go through `wget` + sha256 verify + systemd.
#[derive(Debug, Clone, Copy)]
pub enum DeployArtifact<'a> {
    /// A container image reference like `ghcr.io/tangle-network/llm-operator:v0.1.0`.
    ContainerImage(&'a str),
    /// A platform-specific binary archive from a GitHub release (or any URL).
    ///
    /// The archive is downloaded on the VM, its sha256 is verified, and the
    /// named binary is extracted + started as a systemd service.
    GithubBinary {
        archive_url: &'a str,
        sha256_hex: &'a str,
        binary_name: &'a str,
    },
}

/// Remote provider manager that handles cloud deployments
pub struct RemoteProviderManager {
    provisioner: Arc<CloudProvisioner>,
    registry: Arc<RemoteDeploymentRegistry>,
    namespace: String,
    max_hourly_cost: f64,
    retirement: tokio::task::JoinHandle<()>,
    provider_regions: HashMap<CloudProvider, String>,
}

impl RemoteProviderManager {
    /// Initialize from Blueprint Manager config
    pub async fn new(ctx: &BlueprintManagerContext) -> Result<Option<Self>> {
        // Check if remote providers are configured
        if !ctx
            .cloud_config()
            .as_ref()
            .is_some_and(|config| config.enabled)
        {
            info!("Remote cloud providers not configured");
            return Ok(None);
        }

        Self::with_provisioner(
            CloudProvisioner::new().await?,
            ctx.data_dir(),
            configured_provider_regions(ctx),
            ctx.remote_deployment_opts.max_hourly_cost,
        )
        .await
        .map(Some)
    }

    /// Use caller-owned provider clients; useful for embedding the manager with explicit credentials.
    pub async fn with_provisioner(
        provisioner: CloudProvisioner,
        state_dir: &std::path::Path,
        provider_regions: HashMap<CloudProvider, String>,
        max_hourly_cost: f64,
    ) -> Result<Self> {
        use sha2::{Digest, Sha256};
        if !max_hourly_cost.is_finite() || max_hourly_cost <= 0.0 {
            return Err(Error::Other(
                "Manager GPU spending cap must be positive and finite".into(),
            ));
        }
        let enabled = provider_regions.keys().cloned().collect::<Vec<_>>();
        let provisioner = Arc::new(
            provisioner
                .only_providers(&enabled)
                .with_regions(provider_regions.clone()),
        );
        let tracker =
            Arc::new(DeploymentTracker::new(&state_dir.join("remote_deployments")).await?);
        let registry = Arc::new(RemoteDeploymentRegistry::new(tracker));
        // Resolve aliases before hashing: the same manager state keeps its ownership after restart.
        let state_dir = std::fs::canonicalize(state_dir)?;
        let namespace = format!(
            "bpm-{:x}",
            Sha256::digest(state_dir.as_os_str().as_encoded_bytes())
        );
        let retire_namespace = namespace.clone();
        let retire_provisioner = provisioner.clone();
        let retirement = tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            loop {
                interval.tick().await;
                let report = retire_provisioner.retire_expired(&retire_namespace).await;
                for failure in report.failures {
                    warn!(provider = %failure.provider, code = %failure.code, "GPU lease retirement will retry");
                }
            }
        });
        Ok(Self {
            provisioner,
            registry,
            namespace,
            max_hourly_cost,
            retirement,
            provider_regions,
        })
    }

    /// Handle service initiated event. The artifact controls what gets
    /// deployed onto the provisioned VM:
    ///
    /// - `None` → VM is provisioned idle (operator deploys separately)
    /// - `Some(ContainerImage{..})` → `docker pull` + `docker run` via SSH
    /// - `Some(GithubBinary{..})` → `wget` + sha256 verify + systemd service
    ///
    /// Both artifact forms are first-class. Blueprint authors choose which
    /// to ship based on runtime needs (CUDA-heavy workloads ship containers;
    /// lighter workloads can ship plain binaries).
    pub async fn on_service_initiated(
        &self,
        blueprint_id: u64,
        service_id: u64,
        resource_requirements: Option<ResourceSpec>,
        artifact: Option<DeployArtifact<'_>>,
        extra_env: HashMap<String, String>,
    ) -> Result<()> {
        let artifact_kind = match &artifact {
            Some(DeployArtifact::ContainerImage(_)) => "container",
            Some(DeployArtifact::GithubBinary { .. }) => "github_binary",
            None => "none",
        };
        info!(
            "Remote provider handling service initiation: blueprint={}, service={}, artifact={}",
            blueprint_id, service_id, artifact_kind
        );

        // Use provided resources or default
        let resource_spec = resource_requirements.unwrap_or_else(ResourceSpec::minimal);
        let tee_required = env_bool("BLUEPRINT_REMOTE_TEE_REQUIRED") || resource_spec.tee_required;
        if !tee_required
            && (resource_spec.gpu_count.is_some()
                || resource_spec.gpu_class.is_some()
                || !resource_spec.arms.is_empty())
        {
            return self
                .allocate_service(blueprint_id, service_id, resource_spec, artifact, extra_env)
                .await;
        }

        // CPU and confidential-compute requests retain their existing provisioning path.
        let preferred_provider = if tee_required {
            CloudProvider::AWS
        } else if resource_spec.cpu > 8.0 {
            CloudProvider::Vultr
        } else if resource_spec.memory_gb > 32.0 {
            CloudProvider::AWS
        } else {
            CloudProvider::DigitalOcean
        };
        let provider = self.select_configured_provider(preferred_provider, tee_required)?;

        // Use configured region when available.
        let region = self
            .provider_regions
            .get(&provider)
            .map(String::as_str)
            .unwrap_or_else(|| provider_default_region(&provider));

        if tee_required && !supports_tee(&provider) {
            return Err(Error::TeePrerequisiteMissing {
                prerequisite: format!("{provider} confidential-compute support"),
                hint: "Select AWS, GCP, or Azure when BLUEPRINT_REMOTE_TEE_REQUIRED=true"
                    .to_string(),
            });
        }

        let instance = self
            .provisioner
            .provision_with_requirements(provider.clone(), &resource_spec, region, tee_required)
            .await?;

        self.registry
            .register(
                blueprint_id,
                service_id,
                RemoteDeploymentConfig {
                    deployment_type: deployment_type_from_provider(&provider),
                    provider: Some(provider.clone()),
                    region: Some(region.to_string()),
                    instance_id: instance.id.clone(),
                    resource_spec: resource_spec.clone(),
                    ttl_seconds: Some(3600),
                    deployed_at: Utc::now(),
                },
            )
            .await;

        info!(
            "Remote VM provisioned on {}: instance={}",
            provider, instance.id
        );

        // Dispatch deploy based on artifact kind. Both container and native
        // binary paths are supported; blueprint authors choose at metadata time.
        match artifact {
            Some(DeployArtifact::ContainerImage(image)) => {
                let mut env_vars = extra_env;
                env_vars
                    .entry("BLUEPRINT_ID".to_string())
                    .or_insert_with(|| blueprint_id.to_string());
                env_vars
                    .entry("SERVICE_ID".to_string())
                    .or_insert_with(|| service_id.to_string());

                let deploy_result = self
                    .provisioner
                    .deploy_blueprint_to_instance(
                        &provider,
                        &instance,
                        image,
                        &resource_spec,
                        env_vars,
                    )
                    .await
                    .map_err(|e| Error::Other(format!("deploy_blueprint_to_instance: {e}")))?;
                info!(
                    blueprint_id,
                    service_id,
                    instance_id = %instance.id,
                    image,
                    deployed_id = %deploy_result.blueprint_id,
                    "Container deployed on remote VM (docker pull + run)"
                );
            }
            Some(DeployArtifact::GithubBinary {
                archive_url,
                sha256_hex,
                binary_name,
            }) => {
                let mut env_vars = extra_env;
                env_vars
                    .entry("BLUEPRINT_ID".to_string())
                    .or_insert_with(|| blueprint_id.to_string());
                env_vars
                    .entry("SERVICE_ID".to_string())
                    .or_insert_with(|| service_id.to_string());

                let deploy_result = self
                    .provisioner
                    .deploy_github_binary_to_instance(
                        &provider,
                        &instance,
                        archive_url,
                        sha256_hex,
                        binary_name,
                        &resource_spec,
                        env_vars,
                    )
                    .await
                    .map_err(|e| Error::Other(format!("deploy_github_binary_to_instance: {e}")))?;
                info!(
                    blueprint_id,
                    service_id,
                    instance_id = %instance.id,
                    archive_url,
                    binary_name,
                    deployed_id = %deploy_result.blueprint_id,
                    "Native binary deployed on remote VM (wget + sha256 + systemd)"
                );
            }
            None => {
                info!(
                    blueprint_id,
                    service_id,
                    instance_id = %instance.id,
                    "Remote VM provisioned idle (no deploy artifact supplied)"
                );
            }
        }

        Ok(())
    }

    fn lease_identity(&self, blueprint_id: u64, service_id: u64) -> ManagedLeaseIdentity {
        ManagedLeaseIdentity {
            namespace: self.namespace.clone(),
            lease_id: format!("{blueprint_id:x}-{service_id:x}"),
        }
    }

    /// Reconcile provider-recorded deadlines without an in-memory lease or tracker entry.
    pub async fn retire_expired(&self) -> RetireReport {
        self.provisioner.retire_expired(&self.namespace).await
    }

    async fn allocate_service(
        &self,
        blueprint_id: u64,
        service_id: u64,
        mut spec: ResourceSpec,
        artifact: Option<DeployArtifact<'_>>,
        mut env: HashMap<String, String>,
    ) -> Result<()> {
        spec.validate()
            .map_err(|_| Error::Other("Invalid GPU resource request".into()))?;
        let identity = self.lease_identity(blueprint_id, service_id);
        if spec
            .ownership
            .as_ref()
            .is_some_and(|owner| owner != &identity)
            || (artifact.is_some() && spec.ssh_public_key.is_some())
            || (artifact.is_none() && (!spec.env.is_empty() || !env.is_empty()))
        {
            return Err(Error::Other(
                "GPU request cannot be delivered by this manager".into(),
            ));
        }
        spec.ownership = Some(identity.clone());
        spec.max_hourly_cost_usd = Some(
            spec.max_hourly_cost_usd
                .map_or(self.max_hourly_cost, |cap| cap.min(self.max_hourly_cost)),
        );
        let ttl = Duration::from_secs(spec.lease_duration_secs.unwrap_or(3600));
        // Workload credentials are delivered over SSH, never in a provider launch request.
        env.extend(std::mem::take(&mut spec.env));
        env.entry("BLUEPRINT_ID".into())
            .or_insert_with(|| blueprint_id.to_string());
        env.entry("SERVICE_ID".into())
            .or_insert_with(|| service_id.to_string());
        let lease = self.provisioner.allocate(&spec, ttl).await?;
        let Some(artifact) = artifact else {
            return Ok(());
        };
        if let Err(error) = self.deploy_lease(&lease, artifact, &spec, env).await {
            let cleanup = self
                .provisioner
                .destroy_managed(
                    &lease.resource.instance.provider,
                    &identity,
                    Some(&lease.resource.instance.id),
                )
                .await;
            return match cleanup {
                Ok(()) => Err(error),
                Err(cleanup) => Err(Error::Other(format!(
                    "GPU deployment failed ({error}); cleanup remains pending ({cleanup})"
                ))),
            };
        }
        Ok(())
    }

    async fn deploy_lease(
        &self,
        lease: &Lease,
        artifact: DeployArtifact<'_>,
        spec: &ResourceSpec,
        env: HashMap<String, String>,
    ) -> Result<()> {
        use blueprint_remote_providers::deployment::ssh::{
            ContainerRuntime, DeploymentConfig, SshConnection, SshDeploymentClient,
        };
        use std::io::Write;
        let endpoint = lease
            .resource
            .ssh_endpoint
            .as_deref()
            .ok_or_else(|| Error::Other("GPU lease has no SSH endpoint".into()))?;
        let endpoint = url::Url::parse(endpoint)
            .map_err(|_| Error::Other("Invalid GPU SSH endpoint".into()))?;
        if endpoint.scheme() != "ssh"
            || endpoint.username().is_empty()
            || endpoint.password().is_some()
        {
            return Err(Error::Other("Invalid GPU SSH endpoint".into()));
        }
        let mut key = tempfile::NamedTempFile::new()?;
        key.write_all(
            lease
                .access
                .private_key()
                .ok_or_else(|| Error::Other("GPU lease has no upload key".into()))?
                .as_bytes(),
        )?;
        let client = SshDeploymentClient::new(
            SshConnection {
                host: endpoint
                    .host_str()
                    .ok_or_else(|| Error::Other("GPU SSH endpoint has no host".into()))?
                    .into(),
                port: endpoint.port().unwrap_or(22),
                user: endpoint.username().into(),
                key_path: Some(key.path().into()),
                password: None,
                jump_host: None,
            },
            ContainerRuntime::Docker,
            DeploymentConfig {
                name: lease.resource.lease_id.clone(),
                namespace: self.namespace.clone(),
                ..Default::default()
            },
        )
        .await?;
        match artifact {
            DeployArtifact::ContainerImage(image) => {
                client.deploy_blueprint(image, spec, env).await?;
            }
            DeployArtifact::GithubBinary {
                archive_url,
                sha256_hex,
                binary_name,
            } => {
                client
                    .deploy_github_binary(archive_url, sha256_hex, binary_name, spec, &env)
                    .await?;
            }
        }
        Ok(())
    }

    /// Handle service terminated event
    pub async fn on_service_terminated(&self, blueprint_id: u64, service_id: u64) -> Result<()> {
        info!(
            "Remote provider handling service termination: blueprint={}, service={}",
            blueprint_id, service_id
        );

        if self.registry.get(blueprint_id, service_id).await.is_some() {
            self.registry.cleanup(blueprint_id, service_id).await?;
            return Ok(());
        }
        let identity = self.lease_identity(blueprint_id, service_id);
        let mut failures = Vec::new();
        for provider in self.provider_regions.keys() {
            if self.provisioner.get_adapter(provider)?.managed().is_some()
                && let Err(error) = self
                    .provisioner
                    .destroy_managed(provider, &identity, None)
                    .await
            {
                failures.push(format!("{provider}: {error}"));
            }
        }
        if !failures.is_empty() {
            return Err(Error::Other(format!(
                "GPU lease cleanup remains pending: {}",
                failures.join("; ")
            )));
        }
        Ok(())
    }
}

// Cloud configuration types are now imported from blueprint_remote_providers

impl RemoteProviderManager {
    fn select_configured_provider(
        &self,
        preferred: CloudProvider,
        tee_required: bool,
    ) -> Result<CloudProvider> {
        let ordered_candidates = if tee_required {
            // TEE only on hyperscalers with confidential compute
            vec![
                preferred,
                CloudProvider::AWS,
                CloudProvider::GCP,
                CloudProvider::Azure,
            ]
        } else {
            // CPU workloads: cost-optimized first
            vec![
                preferred,
                CloudProvider::Hetzner,
                CloudProvider::Vultr,
                CloudProvider::DigitalOcean,
                CloudProvider::GCP,
                CloudProvider::AWS,
                CloudProvider::Azure,
            ]
        };

        for candidate in ordered_candidates {
            if self.provider_regions.contains_key(&candidate)
                && (!tee_required || supports_tee(&candidate))
            {
                return Ok(candidate);
            }
        }

        Err(Error::Other(
            "No configured cloud provider can satisfy deployment requirements".to_string(),
        ))
    }
}

fn deployment_type_from_provider(provider: &CloudProvider) -> DeploymentType {
    match provider {
        CloudProvider::AWS => DeploymentType::AwsEc2,
        CloudProvider::GCP => DeploymentType::GcpGce,
        CloudProvider::Azure => DeploymentType::AzureVm,
        CloudProvider::DigitalOcean => DeploymentType::DigitalOceanDroplet,
        CloudProvider::Vultr => DeploymentType::VultrInstance,
        CloudProvider::Hetzner => DeploymentType::HetznerServer,
        CloudProvider::Ovh => DeploymentType::OvhInstance,
        _ => DeploymentType::SshRemote,
    }
}

fn configured_provider_regions(ctx: &BlueprintManagerContext) -> HashMap<CloudProvider, String> {
    let mut regions = HashMap::new();
    if let Some(config) = ctx.cloud_config() {
        if let Some(aws) = &config.aws {
            if aws.enabled {
                regions.insert(CloudProvider::AWS, aws.region.clone());
            }
        }
        if let Some(gcp) = &config.gcp {
            if gcp.enabled {
                regions.insert(CloudProvider::GCP, gcp.region.clone());
            }
        }
        if let Some(azure) = &config.azure {
            if azure.enabled {
                regions.insert(CloudProvider::Azure, azure.region.clone());
            }
        }
        if let Some(do_cfg) = &config.digital_ocean {
            if do_cfg.enabled {
                regions.insert(CloudProvider::DigitalOcean, do_cfg.region.clone());
            }
        }
        if let Some(vultr) = &config.vultr {
            if vultr.enabled {
                regions.insert(CloudProvider::Vultr, vultr.region.clone());
            }
        }
        if let Some(cfg) = &config.lambda_labs {
            if cfg.enabled {
                regions.insert(CloudProvider::LambdaLabs, cfg.region.clone());
            }
        }
        if let Some(cfg) = &config.runpod {
            if cfg.enabled {
                regions.insert(CloudProvider::RunPod, cfg.region.clone());
            }
        }
        if let Some(cfg) = &config.vast_ai {
            if cfg.enabled {
                regions.insert(CloudProvider::VastAi, "global".to_string());
            }
        }
        if let Some(cfg) = &config.coreweave {
            if cfg.enabled {
                regions.insert(CloudProvider::CoreWeave, cfg.region.clone());
            }
        }
        if let Some(cfg) = &config.paperspace {
            if cfg.enabled {
                regions.insert(CloudProvider::Paperspace, cfg.region.clone());
            }
        }
        if let Some(cfg) = &config.fluidstack {
            if cfg.enabled {
                regions.insert(CloudProvider::Fluidstack, cfg.region.clone());
            }
        }
        if let Some(cfg) = &config.tensordock {
            if cfg.enabled {
                regions.insert(CloudProvider::TensorDock, cfg.region.clone());
            }
        }
        if let Some(cfg) = &config.akash {
            if cfg.enabled {
                regions.insert(CloudProvider::Akash, "global".to_string());
            }
        }
        if let Some(cfg) = &config.io_net {
            if cfg.enabled {
                regions.insert(CloudProvider::IoNet, cfg.region.clone());
            }
        }
        if let Some(cfg) = &config.prime_intellect {
            if cfg.enabled {
                regions.insert(CloudProvider::PrimeIntellect, cfg.region.clone());
            }
        }
        if let Some(cfg) = &config.render {
            if cfg.enabled {
                regions.insert(CloudProvider::Render, cfg.region.clone());
            }
        }
        if let Some(cfg) = &config.bittensor_lium {
            if cfg.enabled {
                regions.insert(CloudProvider::BittensorLium, "global".to_string());
            }
        }
        if let Some(cfg) = &config.hetzner {
            if cfg.enabled {
                regions.insert(CloudProvider::Hetzner, cfg.region.clone());
            }
        }
        if let Some(cfg) = &config.crusoe {
            if cfg.enabled {
                regions.insert(CloudProvider::Crusoe, cfg.region.clone());
            }
        }
    }
    regions
}

fn provider_default_region(provider: &CloudProvider) -> &'static str {
    match provider {
        CloudProvider::AWS => "us-east-1",
        CloudProvider::GCP => "us-central1",
        CloudProvider::Azure => "eastus",
        CloudProvider::DigitalOcean => "nyc3",
        CloudProvider::Vultr => "ewr",
        CloudProvider::Hetzner => "fsn1",
        CloudProvider::Ovh => "GRA11",
        _ => "default",
    }
}

impl Drop for RemoteProviderManager {
    fn drop(&mut self) {
        self.retirement.abort();
    }
}
