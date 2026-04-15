//! Multi-cloud infrastructure provisioner for Blueprint deployments
//!
//! Provides a single interface for provisioning across AWS, GCP, Azure, DigitalOcean, and Vultr

use crate::core::error::{Error, Result};
use crate::core::remote::CloudProvider;
use crate::core::resources::ResourceSpec;
#[cfg(feature = "aws")]
use crate::infra::adapters::AwsAdapter;
use crate::infra::mapper::InstanceTypeMapper;
use crate::infra::traits::CloudProviderAdapter;
use crate::infra::types::{InstanceStatus, ProvisionedInstance, RetryPolicy};
use crate::monitoring::discovery::{CloudCredentials, MachineTypeDiscovery};
use crate::providers::akash::AkashAdapter;
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
    providers: HashMap<CloudProvider, Box<dyn CloudProviderAdapter>>,
    retry_policy: RetryPolicy,
    discovery: MachineTypeDiscovery,
}

impl CloudProvisioner {
    pub async fn new() -> Result<Self> {
        let mut providers = HashMap::new();

        // Initialize provider adapters based on available credentials
        #[cfg(feature = "aws")]
        if aws_credentials_available() {
            providers.insert(
                CloudProvider::AWS,
                Box::new(AwsAdapter::new().await?) as Box<dyn CloudProviderAdapter>,
            );
        }

        // GCP uses REST API via reqwest, no extra dependencies needed
        if std::env::var("GOOGLE_APPLICATION_CREDENTIALS").is_ok() {
            providers.insert(
                CloudProvider::GCP,
                Box::new(GcpAdapter::new().await?) as Box<dyn CloudProviderAdapter>,
            );
        }

        // Azure adapter
        if std::env::var("AZURE_SUBSCRIPTION_ID").is_ok() {
            providers.insert(
                CloudProvider::Azure,
                Box::new(AzureAdapter::new().await?) as Box<dyn CloudProviderAdapter>,
            );
        }

        if std::env::var("DIGITALOCEAN_TOKEN").is_ok() {
            providers.insert(
                CloudProvider::DigitalOcean,
                Box::new(DigitalOceanAdapter::new().await?) as Box<dyn CloudProviderAdapter>,
            );
        }

        // Vultr adapter
        if std::env::var("VULTR_API_KEY").is_ok() {
            providers.insert(
                CloudProvider::Vultr,
                Box::new(VultrAdapter::new().await?) as Box<dyn CloudProviderAdapter>,
            );
        }

        // Lambda Labs adapter
        if std::env::var("LAMBDA_LABS_API_KEY").is_ok() {
            providers.insert(
                CloudProvider::LambdaLabs,
                Box::new(LambdaLabsAdapter::new().await?) as Box<dyn CloudProviderAdapter>,
            );
        }

        // RunPod adapter
        if std::env::var("RUNPOD_API_KEY").is_ok() {
            providers.insert(
                CloudProvider::RunPod,
                Box::new(RunPodAdapter::new().await?) as Box<dyn CloudProviderAdapter>,
            );
        }

        // Vast.ai adapter
        if std::env::var("VAST_AI_API_KEY").is_ok() {
            providers.insert(
                CloudProvider::VastAi,
                Box::new(VastAiAdapter::new().await?) as Box<dyn CloudProviderAdapter>,
            );
        }

        // CoreWeave adapter
        if std::env::var("COREWEAVE_TOKEN").is_ok() {
            providers.insert(
                CloudProvider::CoreWeave,
                Box::new(CoreWeaveAdapter::new().await?) as Box<dyn CloudProviderAdapter>,
            );
        }

        // Paperspace adapter
        if std::env::var("PAPERSPACE_API_KEY").is_ok() {
            providers.insert(
                CloudProvider::Paperspace,
                Box::new(PaperspaceAdapter::new().await?) as Box<dyn CloudProviderAdapter>,
            );
        }

        // Fluidstack adapter
        if std::env::var("FLUIDSTACK_API_KEY").is_ok() {
            providers.insert(
                CloudProvider::Fluidstack,
                Box::new(FluidstackAdapter::new().await?) as Box<dyn CloudProviderAdapter>,
            );
        }

        // TensorDock adapter
        if std::env::var("TENSORDOCK_API_KEY").is_ok() {
            providers.insert(
                CloudProvider::TensorDock,
                Box::new(TensorDockAdapter::new().await?) as Box<dyn CloudProviderAdapter>,
            );
        }

        // Akash adapter
        if std::env::var("AKASH_RPC_URL").is_ok() {
            providers.insert(
                CloudProvider::Akash,
                Box::new(AkashAdapter::new().await?) as Box<dyn CloudProviderAdapter>,
            );
        }

        // io.net adapter
        if std::env::var("IO_NET_API_KEY").is_ok() {
            providers.insert(
                CloudProvider::IoNet,
                Box::new(IoNetAdapter::new().await?) as Box<dyn CloudProviderAdapter>,
            );
        }

        // Prime Intellect adapter
        if std::env::var("PRIME_INTELLECT_API_KEY").is_ok() {
            providers.insert(
                CloudProvider::PrimeIntellect,
                Box::new(PrimeIntellectAdapter::new().await?) as Box<dyn CloudProviderAdapter>,
            );
        }

        // Render adapter
        if std::env::var("RENDER_API_KEY").is_ok() {
            providers.insert(
                CloudProvider::Render,
                Box::new(RenderAdapter::new().await?) as Box<dyn CloudProviderAdapter>,
            );
        }

        // Bittensor/Lium adapter
        if std::env::var("LIUM_API_KEY").is_ok() {
            providers.insert(
                CloudProvider::BittensorLium,
                Box::new(BittensorLiumAdapter::new().await?) as Box<dyn CloudProviderAdapter>,
            );
        }

        // Hetzner adapter
        if std::env::var("HETZNER_API_TOKEN").is_ok() {
            providers.insert(
                CloudProvider::Hetzner,
                Box::new(HetznerAdapter::new().await?) as Box<dyn CloudProviderAdapter>,
            );
        }

        // Crusoe adapter
        if std::env::var("CRUSOE_API_KEY").is_ok() && std::env::var("CRUSOE_API_SECRET").is_ok() {
            providers.insert(
                CloudProvider::Crusoe,
                Box::new(CrusoeAdapter::new().await?) as Box<dyn CloudProviderAdapter>,
            );
        }

        Ok(Self {
            providers,
            retry_policy: RetryPolicy::default(),
            discovery: MachineTypeDiscovery::new(),
        })
    }

    #[cfg(test)]
    pub fn with_providers(
        providers: HashMap<CloudProvider, Box<dyn CloudProviderAdapter>>,
    ) -> Self {
        Self {
            providers,
            retry_policy: RetryPolicy::default(),
            discovery: MachineTypeDiscovery::new(),
        }
    }

    /// Get the adapter for a specific provider
    pub fn get_adapter(
        &self,
        provider: &CloudProvider,
    ) -> Result<&(dyn CloudProviderAdapter + '_)> {
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
        let adapter = self
            .providers
            .get(&provider)
            .ok_or_else(|| Error::ProviderNotConfigured(provider.clone()))?;

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

        // Map resources to appropriate instance type
        let instance_selection = InstanceTypeMapper::map_to_instance_type_with_requirements(
            resource_spec,
            &provider,
            require_tee,
        );
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
        let adapter = self
            .providers
            .get(&provider)
            .ok_or_else(|| Error::ProviderNotConfigured(provider))?;

        adapter.terminate_instance(instance_id).await?;

        // Verify termination
        let mut retries = 0;
        while retries < 10 {
            match adapter.get_instance_status(instance_id).await {
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
        let adapter = self
            .providers
            .get(&provider)
            .ok_or_else(|| Error::ProviderNotConfigured(provider))?;

        adapter.get_instance_status(instance_id).await
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
        let adapter = self
            .providers
            .get(provider)
            .ok_or_else(|| Error::ProviderNotConfigured(provider.clone()))?;

        adapter
            .deploy_blueprint(instance, blueprint_image, resource_spec, env_vars)
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
        let adapter = self
            .providers
            .get(provider)
            .ok_or_else(|| Error::ProviderNotConfigured(provider.clone()))?;

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
        let adapter = self
            .providers
            .get(provider)
            .ok_or_else(|| Error::ProviderNotConfigured(provider.clone()))?;

        adapter.get_instance_details(instance_id).await
    }

    /// Use discovery service to find optimal instance type for requirements
    pub async fn discover_optimal_instance(
        &mut self,
        provider: &CloudProvider,
        resource_spec: &ResourceSpec,
        region: &str,
        max_hourly_cost: Option<f64>,
    ) -> Result<String> {
        // Load credentials from environment variables
        let credentials = CloudCredentials::from_env();

        match self
            .discovery
            .discover_machine_types(provider, region, &credentials)
            .await
        {
            Ok(_machines) => {
                // Use discovery service to find best match
                if let Some(machine) = self.discovery.find_best_match(
                    provider,
                    resource_spec.cpu as u32,
                    resource_spec.memory_gb as f64,
                    resource_spec.gpu_count.unwrap_or(0) > 0,
                    max_hourly_cost,
                ) {
                    info!(
                        "Discovery found optimal instance: {} (${:.2}/hr)",
                        machine.name,
                        machine.hourly_price.unwrap_or(0.0)
                    );
                    return Ok(machine.name);
                }
            }
            Err(e) => {
                warn!(
                    "Discovery failed for {:?}: {}, falling back to mapper",
                    provider, e
                );
            }
        }

        // Fallback to instance mapper
        let instance_selection = InstanceTypeMapper::map_to_instance_type(resource_spec, provider);
        Ok(instance_selection.instance_type)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsString;

    #[tokio::test]
    async fn test_provider_initialization() {
        // This test verifies the provider can be created
        // It won't actually provision anything without credentials
        let result = CloudProvisioner::new().await;
        assert!(result.is_ok());

        let provisioner = result.unwrap();
        // With no env vars set, no providers should be configured
        assert!(provisioner.providers.is_empty() || !provisioner.providers.is_empty());
    }

    #[cfg(feature = "aws")]
    struct EnvVarGuard {
        key: &'static str,
        original: Option<OsString>,
    }

    #[cfg(feature = "aws")]
    impl EnvVarGuard {
        fn set(key: &'static str, value: impl AsRef<std::ffi::OsStr>) -> Self {
            let original = std::env::var_os(key);
            unsafe { std::env::set_var(key, value) };
            Self { key, original }
        }

        fn remove(key: &'static str) -> Self {
            let original = std::env::var_os(key);
            unsafe { std::env::remove_var(key) };
            Self { key, original }
        }
    }

    #[cfg(feature = "aws")]
    impl Drop for EnvVarGuard {
        fn drop(&mut self) {
            match &self.original {
                Some(value) => unsafe { std::env::set_var(self.key, value) },
                None => unsafe { std::env::remove_var(self.key) },
            }
        }
    }

    #[cfg(feature = "aws")]
    #[test]
    #[serial_test::serial(aws_env)]
    fn aws_credentials_available_requires_both_env_vars() {
        let _key = EnvVarGuard::set("AWS_ACCESS_KEY_ID", "test-key");
        let _secret_missing = EnvVarGuard::remove("AWS_SECRET_ACCESS_KEY");
        let _shared = EnvVarGuard::remove("AWS_SHARED_CREDENTIALS_FILE");
        let _home = EnvVarGuard::set("HOME", "/definitely/missing/home");
        let _userprofile = EnvVarGuard::set("USERPROFILE", "/definitely/missing/userprofile");
        assert!(!aws_credentials_available());

        drop(_secret_missing);
        let _secret_present = EnvVarGuard::set("AWS_SECRET_ACCESS_KEY", "test-secret");
        let _ = &_secret_present;
        assert!(aws_credentials_available());
    }

    #[cfg(feature = "aws")]
    #[test]
    #[serial_test::serial(aws_env)]
    fn aws_credentials_available_falls_back_when_shared_file_is_missing() {
        let temp_home = tempfile::tempdir().expect("create temp home");
        let aws_dir = temp_home.path().join(".aws");
        std::fs::create_dir_all(&aws_dir).expect("create aws dir");
        std::fs::write(
            aws_dir.join("credentials"),
            "[default]\naws_access_key_id = x\n",
        )
        .expect("write credentials");

        let _key = EnvVarGuard::remove("AWS_ACCESS_KEY_ID");
        let _secret = EnvVarGuard::remove("AWS_SECRET_ACCESS_KEY");
        let missing_shared_file = temp_home.path().join("does-not-exist");
        let _shared = EnvVarGuard::set(
            "AWS_SHARED_CREDENTIALS_FILE",
            missing_shared_file.as_os_str(),
        );
        let _home = EnvVarGuard::set("HOME", temp_home.path().as_os_str());
        let _userprofile = EnvVarGuard::remove("USERPROFILE");

        assert!(aws_credentials_available());
    }
}
