//! Multi-cloud infrastructure provisioner for Blueprint deployments
//!
//! Provides a single interface for provisioning across every provider in `providers`

use crate::core::error::{Error, Result};
use crate::core::remote::CloudProvider;
use crate::core::resources::ResourceSpec;
use crate::infra::mapper::InstanceTypeMapper;
use crate::infra::traits::CloudProviderAdapter;
use crate::infra::types::{InstanceStatus, ProvisionedInstance, RetryPolicy};
use crate::providers::akash::AkashAdapter;
#[cfg(feature = "aws")]
use crate::providers::aws::AwsAdapter;
use crate::providers::azure::adapter::AzureAdapter;
use crate::providers::bittensor_lium::BittensorLiumAdapter;
use crate::providers::coreweave::CoreWeaveAdapter;
use crate::providers::crusoe::CrusoeAdapter;
use crate::providers::digitalocean::adapter::DigitalOceanAdapter;
use crate::providers::fluidstack::FluidstackAdapter;
use crate::providers::gcp::GcpAdapter;
use crate::providers::hetzner::HetznerAdapter;
use crate::providers::io_net::IoNetAdapter;
use crate::providers::lambda_labs::LambdaLabsAdapter;
use crate::providers::ovh::OvhAdapter;
use crate::providers::paperspace::PaperspaceAdapter;
use crate::providers::prime_intellect::PrimeIntellectAdapter;
use crate::providers::render::RenderAdapter;
use crate::providers::runpod::RunPodAdapter;
use crate::providers::tensordock::TensorDockAdapter;
use crate::providers::vast_ai::VastAiAdapter;
use crate::providers::vultr::adapter::VultrAdapter;
use blueprint_core::{error, info, warn};
use blueprint_std::collections::HashMap;

/// Multi-cloud provisioner that handles deployments across all supported providers
pub struct CloudProvisioner {
    pub(crate) providers: HashMap<CloudProvider, Box<dyn CloudProviderAdapter>>,
    pub(crate) regions: HashMap<CloudProvider, String>,
    /// Why each provider the environment names has no adapter.
    init_errors: HashMap<CloudProvider, String>,
    retry_policy: RetryPolicy,
}

impl CloudProvisioner {
    /// Build an adapter for every provider the environment names.
    ///
    /// A provider that cannot start fails only its own work: [`Self::get_adapter`] reports why
    /// when that provider is used, so it never stops another provider's deployments.
    pub async fn new() -> Result<Self> {
        let mut provisioner = Self {
            providers: HashMap::new(),
            regions: HashMap::new(),
            init_errors: HashMap::new(),
            retry_policy: RetryPolicy::default(),
        };
        let present = |name: &str| std::env::var_os(name).is_some();

        #[cfg(feature = "aws")]
        if aws_credentials_available() {
            provisioner.add(CloudProvider::AWS, AwsAdapter::new().await);
        }
        if present("GCP_PROJECT_ID") {
            provisioner.add(CloudProvider::GCP, GcpAdapter::new().await);
        }
        if present("AZURE_SUBSCRIPTION_ID") {
            provisioner.add(CloudProvider::Azure, AzureAdapter::new().await);
        }
        if present("DIGITALOCEAN_TOKEN") {
            provisioner.add(
                CloudProvider::DigitalOcean,
                DigitalOceanAdapter::new().await,
            );
        }
        if present("VULTR_API_KEY") {
            provisioner.add(CloudProvider::Vultr, VultrAdapter::new().await);
        }
        if present("LAMBDA_LABS_API_KEY") {
            provisioner.add(CloudProvider::LambdaLabs, LambdaLabsAdapter::new().await);
        }
        if present("RUNPOD_API_KEY") {
            provisioner.add(CloudProvider::RunPod, RunPodAdapter::new().await);
        }
        if present("VAST_AI_API_KEY") {
            provisioner.add(CloudProvider::VastAi, VastAiAdapter::new().await);
        }
        if present("COREWEAVE_TOKEN") {
            provisioner.add(CloudProvider::CoreWeave, CoreWeaveAdapter::new().await);
        }
        if present("PAPERSPACE_API_KEY") {
            provisioner.add(CloudProvider::Paperspace, PaperspaceAdapter::new().await);
        }
        if present("FLUIDSTACK_API_KEY") {
            provisioner.add(CloudProvider::Fluidstack, FluidstackAdapter::new().await);
        }
        if present("TENSORDOCK_API_KEY") {
            provisioner.add(CloudProvider::TensorDock, TensorDockAdapter::new().await);
        }
        if present("AKASH_RPC_URL") {
            provisioner.add(CloudProvider::Akash, AkashAdapter::new().await);
        }
        if present("IO_NET_API_KEY") {
            provisioner.add(CloudProvider::IoNet, IoNetAdapter::new().await);
        }
        if present("PRIME_INTELLECT_API_KEY") {
            provisioner.add(
                CloudProvider::PrimeIntellect,
                PrimeIntellectAdapter::new().await,
            );
        }
        if present("RENDER_API_KEY") {
            provisioner.add(CloudProvider::Render, RenderAdapter::new().await);
        }
        if present("LIUM_API_KEY") {
            provisioner.add(
                CloudProvider::BittensorLium,
                BittensorLiumAdapter::new().await,
            );
        }
        if present("HETZNER_API_TOKEN") {
            provisioner.add(CloudProvider::Hetzner, HetznerAdapter::new().await);
        }
        if present("OVH_APPLICATION_KEY") {
            provisioner.add(CloudProvider::Ovh, OvhAdapter::new().await);
        }
        if present("CRUSOE_API_KEY") && present("CRUSOE_API_SECRET") {
            provisioner.add(CloudProvider::Crusoe, CrusoeAdapter::new().await);
        }

        Ok(provisioner)
    }

    fn add(
        &mut self,
        provider: CloudProvider,
        adapter: Result<impl CloudProviderAdapter + 'static>,
    ) {
        match adapter {
            Ok(adapter) => {
                self.providers.insert(provider, Box::new(adapter));
            }
            Err(error) => {
                self.init_errors.insert(provider, error.to_string());
            }
        }
    }

    pub fn with_providers(
        providers: HashMap<CloudProvider, Box<dyn CloudProviderAdapter>>,
    ) -> Self {
        Self {
            providers,
            regions: HashMap::new(),
            init_errors: HashMap::new(),
            retry_policy: RetryPolicy::default(),
        }
    }

    pub fn with_regions(mut self, regions: HashMap<CloudProvider, String>) -> Self {
        self.regions = regions;
        self
    }

    /// Get the adapter for a specific provider
    pub fn get_adapter(
        &self,
        provider: &CloudProvider,
    ) -> Result<&(dyn CloudProviderAdapter + '_)> {
        if let Some(reason) = self.init_errors.get(provider) {
            return Err(Error::ConfigurationError(format!(
                "{provider:?} is unavailable: {reason}"
            )));
        }
        self.providers
            .get(provider)
            .map(Box::as_ref)
            .ok_or_else(|| Error::ProviderNotConfigured(provider.clone()))
    }

    /// Provision infrastructure on specified provider with retry logic
    pub async fn provision(
        &self,
        provider: CloudProvider,
        resource_spec: &ResourceSpec,
        region: &str,
    ) -> Result<ProvisionedInstance> {
        self.provision_with_requirements(provider, resource_spec, region, false)
            .await
    }

    /// Provision infrastructure with explicit deployment requirements.
    pub async fn provision_with_requirements(
        &self,
        provider: CloudProvider,
        resource_spec: &ResourceSpec,
        region: &str,
        require_tee: bool,
    ) -> Result<ProvisionedInstance> {
        resource_spec
            .validate()
            .map_err(|_| Error::Provider(crate::managed::ProviderErrorCode::InvalidRequest))?;
        let require_tee = require_tee || resource_spec.tee_required;
        let adapter = self.get_adapter(&provider)?;

        if require_tee
            && !matches!(
                provider,
                CloudProvider::AWS | CloudProvider::GCP | CloudProvider::Azure
            )
        {
            return Err(Error::ConfigurationError(format!(
                "Provider {provider:?} does not support confidential-compute provisioning"
            )));
        }

        if adapter.managed().is_some()
            && (resource_spec.gpu_count.is_some() || !resource_spec.arms.is_empty())
        {
            use crate::managed::ProviderErrorCode;
            let mut spec = resource_spec.clone();
            if spec
                .provider
                .as_ref()
                .is_some_and(|value| value != &provider)
                || spec
                    .region
                    .as_deref()
                    .is_some_and(|value| !region.is_empty() && value != region)
                || spec.ownership.is_none()
                || spec.lease_duration_secs.is_none_or(|seconds| seconds == 0)
            {
                return Err(Error::Provider(ProviderErrorCode::InvalidRequest));
            }
            spec.provider = Some(provider.clone());
            if spec.region.is_none() && !region.is_empty() {
                spec.region = Some(region.into());
            }
            spec.tee_required |= require_tee;
            let offers = self.quotes(&spec).await?;
            let quote = offers.quotes.first().ok_or_else(|| {
                Error::Provider(
                    offers
                        .errors
                        .first()
                        .map_or(ProviderErrorCode::NoStock, |error| error.code),
                )
            })?;
            return self
                .create_managed(&spec, &quote.candidate)
                .await
                .map(|resource| resource.instance);
        }

        // Map resources to appropriate instance type
        let instance_selection = InstanceTypeMapper::map_to_instance_type_with_requirements(
            resource_spec,
            &provider,
            require_tee,
        )?;
        info!(
            "Provisioning {} instance type {} (tee_required={})",
            provider, instance_selection.instance_type, require_tee
        );

        // Retry with exponential backoff
        let mut attempt = 0;
        loop {
            match adapter
                .provision_instance(&instance_selection.instance_type, region, require_tee)
                .await
            {
                Ok(instance) => {
                    info!(
                        "Successfully provisioned {} instance: {}",
                        provider, instance.id
                    );
                    return Ok(instance);
                }
                Err(e) if attempt < self.retry_policy.max_retries => {
                    attempt += 1;
                    let delay = self.retry_policy.delay_for_attempt(attempt);
                    warn!(
                        "Provision attempt {} failed: {}, retrying in {:?}",
                        attempt, e, delay
                    );
                    tokio::time::sleep(delay).await;
                }
                Err(e) => {
                    error!("Failed to provision after {} attempts: {}", attempt + 1, e);
                    return Err(e);
                }
            }
        }
    }

    /// Terminate instance with cleanup verification
    pub async fn terminate(&self, provider: CloudProvider, instance_id: &str) -> Result<()> {
        self.terminate_in(provider, instance_id, None).await
    }

    /// Terminate an instance in the region its deployment record names, with cleanup verification
    pub async fn terminate_in(
        &self,
        provider: CloudProvider,
        instance_id: &str,
        region: Option<&str>,
    ) -> Result<()> {
        let adapter = self.get_adapter(&provider)?;

        adapter.terminate_instance_in(instance_id, region).await?;

        // Verify termination
        let mut retries = 0;
        while retries < 10 {
            match adapter.get_instance_status_in(instance_id, region).await {
                Ok(InstanceStatus::Terminated) => {
                    info!("Instance {} successfully terminated", instance_id);
                    return Ok(());
                }
                Ok(status) => {
                    warn!(
                        "Instance {} still in status {:?}, waiting...",
                        instance_id, status
                    );
                    tokio::time::sleep(blueprint_std::time::Duration::from_secs(5)).await;
                    retries += 1;
                }
                Err(e) => {
                    if is_not_found_error(&e) {
                        info!(
                            "Instance {} no longer found after termination request",
                            instance_id
                        );
                        return Ok(());
                    }
                    warn!(
                        "Failed to verify termination status for {}: {}",
                        instance_id, e
                    );
                    tokio::time::sleep(blueprint_std::time::Duration::from_secs(5)).await;
                    retries += 1;
                }
            }
        }

        Err(Error::Other(
            "Instance termination verification timeout".into(),
        ))
    }

    /// Get current status of an instance
    pub async fn get_status(
        &self,
        provider: CloudProvider,
        instance_id: &str,
    ) -> Result<InstanceStatus> {
        self.get_status_in(provider, instance_id, None).await
    }

    /// Get an instance's status in the region its deployment record names
    pub async fn get_status_in(
        &self,
        provider: CloudProvider,
        instance_id: &str,
        region: Option<&str>,
    ) -> Result<InstanceStatus> {
        self.get_adapter(&provider)?
            .get_instance_status_in(instance_id, region)
            .await
    }

    /// Deploy a Blueprint to a provisioned instance using the appropriate adapter
    pub async fn deploy_blueprint_to_instance(
        &self,
        provider: &CloudProvider,
        instance: &ProvisionedInstance,
        blueprint_image: &str,
        resource_spec: &ResourceSpec,
        env_vars: std::collections::HashMap<String, String>,
    ) -> Result<crate::infra::traits::BlueprintDeploymentResult> {
        let adapter = self.get_adapter(provider)?;

        adapter
            .deploy_blueprint(instance, blueprint_image, resource_spec, env_vars)
            .await
    }

    /// Deploy a native GitHub-release binary to a provisioned instance via SSH.
    ///
    /// No Docker required — the binary is downloaded on the VM, verified, and
    /// started as a systemd service. This is the path used when the blueprint
    /// ships a `BlueprintSource::Github` or `BlueprintSource::Remote` source
    /// with platform binaries (linux/amd64, linux/arm64) instead of a container
    /// image.
    ///
    /// Caller is responsible for picking the right archive URL for the VM's
    /// platform (the manager does this before the call).
    pub async fn deploy_github_binary_to_instance(
        &self,
        provider: &CloudProvider,
        instance: &ProvisionedInstance,
        archive_url: &str,
        sha256_hex: &str,
        binary_name: &str,
        resource_spec: &ResourceSpec,
        env_vars: std::collections::HashMap<String, String>,
    ) -> Result<crate::infra::traits::BlueprintDeploymentResult> {
        use crate::shared::ssh_deployment::{SharedSshDeployment, SshDeploymentConfig};

        // Build provider-specific SSH config (picks the right username + key).
        let ssh_config = match provider {
            CloudProvider::AWS => SshDeploymentConfig::aws(),
            CloudProvider::GCP => SshDeploymentConfig::gcp("blueprint"),
            CloudProvider::Azure => SshDeploymentConfig::azure(),
            CloudProvider::DigitalOcean => SshDeploymentConfig::digitalocean(),
            CloudProvider::Hetzner => SshDeploymentConfig::hetzner(),
            CloudProvider::Ovh => SshDeploymentConfig::ovh(),
            CloudProvider::RunPod => SshDeploymentConfig::runpod(),
            CloudProvider::LambdaLabs => SshDeploymentConfig::lambda_labs(),
            CloudProvider::VastAi => SshDeploymentConfig::vast_ai(),
            CloudProvider::Vultr => SshDeploymentConfig::vultr(),
            other => {
                return Err(Error::Other(format!(
                    "native binary deploy not yet supported for provider {other:?}"
                )));
            }
        };

        SharedSshDeployment::deploy_github_binary_to_instance(
            instance,
            archive_url,
            sha256_hex,
            binary_name,
            resource_spec,
            env_vars,
            ssh_config,
        )
        .await
    }

    /// Deploy a Blueprint with specific deployment target
    pub async fn deploy_with_target(
        &self,
        target: &crate::core::deployment_target::DeploymentTarget,
        blueprint_image: &str,
        resource_spec: &ResourceSpec,
        env_vars: std::collections::HashMap<String, String>,
    ) -> Result<crate::infra::traits::BlueprintDeploymentResult> {
        let provider = self.resolve_single_provider()?;
        self.deploy_with_target_for_provider(
            provider,
            target,
            blueprint_image,
            resource_spec,
            env_vars,
        )
        .await
    }

    /// Deploy a Blueprint to a specific provider with a specific target.
    pub async fn deploy_with_target_for_provider(
        &self,
        provider: &CloudProvider,
        target: &crate::core::deployment_target::DeploymentTarget,
        blueprint_image: &str,
        resource_spec: &ResourceSpec,
        env_vars: std::collections::HashMap<String, String>,
    ) -> Result<crate::infra::traits::BlueprintDeploymentResult> {
        let adapter = self.get_adapter(provider)?;

        adapter
            .deploy_blueprint_with_target(target, blueprint_image, resource_spec, env_vars)
            .await
    }

    fn resolve_single_provider(&self) -> Result<&CloudProvider> {
        let mut providers = self.providers.keys();
        let provider = providers
            .next()
            .ok_or_else(|| Error::Other("No providers configured".into()))?;
        if providers.next().is_some() {
            return Err(Error::ConfigurationError(
                "Multiple providers configured; select one explicitly".into(),
            ));
        }
        Ok(provider)
    }

    /// Get the status of an instance using the appropriate adapter (alias for compatibility)
    pub async fn get_instance_status(
        &self,
        provider: &CloudProvider,
        instance_id: &str,
    ) -> Result<crate::infra::types::InstanceStatus> {
        self.get_status(provider.clone(), instance_id).await
    }

    /// Get full instance details including public IP
    pub async fn get_instance_details(
        &self,
        provider: &CloudProvider,
        instance_id: &str,
    ) -> Result<ProvisionedInstance> {
        let adapter = self.get_adapter(provider)?;

        adapter.get_instance_details(instance_id).await
    }
}

fn is_not_found_error(error: &Error) -> bool {
    let message = match error {
        Error::ConfigurationError(message)
        | Error::NetworkError(message)
        | Error::SerializationError(message)
        | Error::HttpError(message)
        | Error::Other(message) => message.as_str(),
        #[cfg(feature = "kubernetes")]
        Error::Kube(err) => return err.to_string().contains("NotFound"),
        _ => return false,
    };

    message.contains("404")
        || message.to_ascii_lowercase().contains("not found")
        || message.to_ascii_lowercase().contains("does not exist")
}

#[cfg(feature = "aws")]
fn aws_credentials_available() -> bool {
    let env_credentials = std::env::var("AWS_ACCESS_KEY_ID").is_ok()
        && std::env::var("AWS_SECRET_ACCESS_KEY").is_ok();
    if env_credentials {
        return true;
    }

    if let Ok(shared_credentials_file) = std::env::var("AWS_SHARED_CREDENTIALS_FILE") {
        if !shared_credentials_file.trim().is_empty()
            && std::path::Path::new(&shared_credentials_file).exists()
        {
            return true;
        }
    }

    std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(std::path::PathBuf::from)
        .map(|home| home.join(".aws").join("credentials").exists())
        .unwrap_or(false)
}
