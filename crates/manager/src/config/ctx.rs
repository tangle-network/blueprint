use crate::config::BlueprintManagerConfig;
use crate::error::Result;
#[cfg(feature = "vm-sandbox")]
use crate::rt::hypervisor::net::NetworkManager;
use blueprint_auth::db::RocksDb;
use std::ops::{Deref, DerefMut};
use tokio::sync::Mutex;

#[cfg(feature = "containers")]
pub struct ContainerContext {
    pub kube_client: kube::Client,
    pub kube_service_port: Mutex<crate::sdk::utils::PortLock>,
    pub local_ip: std::net::IpAddr,
}

#[cfg(feature = "vm-sandbox")]
pub struct VmContext {
    pub network_manager: NetworkManager,
    pub network_interface: String,
}

pub struct BlueprintManagerContext {
    #[cfg(feature = "containers")]
    pub containers: ContainerContext,
    #[cfg(feature = "vm-sandbox")]
    pub vm: VmContext,
    pub(crate) db: Mutex<Option<RocksDb>>,
    config: BlueprintManagerConfig,
    #[cfg(feature = "remote-providers")]
    cloud_config: Option<crate::executor::remote_provider_integration::CloudConfig>,
}

impl BlueprintManagerContext {
    /// Create a new `BlueprintManagerContext`
    ///
    /// # Errors
    ///
    /// * Unable to create the necessary directories
    /// * With the `vm-sandbox` enabled
    ///   * Unable to determine the default network interface if not specified in `config`
    /// * With the `containers` feature enabled
    ///   * Unable to connect to the Kubernetes cluster
    ///   * Unable to bind to the [`BlueprintManagerConfig::kube_service_port()`]
    ///   * Unable to determine the system's local IPv4 address
    #[allow(clippy::unused_async)]
    pub async fn new(mut config: BlueprintManagerConfig) -> Result<Self> {
        config.paths.data_dir = std::path::absolute(config.data_dir())?;

        config.verify_directories_exist()?;
        #[cfg(feature = "vm-sandbox")]
        let (network_manager, network_interface) = {
            use tracing::info;

            let interface = config.verify_network_interface()?;

            crate::rt::hypervisor::net::nftables::check_net_admin_capability()?;

            let network_candidates = config
                .vm_sandbox_options
                .default_address_pool
                .hosts()
                .filter(|ip| ip.octets()[3] != 0 && ip.octets()[3] != 255)
                .collect();

            if config.vm_sandbox_options.no_vm {
                info!("Skipping VM image check, running in no-vm mode");
            } else {
                info!("Checking for VM images");
                crate::rt::hypervisor::images::download_image_if_needed(config.cache_dir()).await?;
            }

            (NetworkManager::new(network_candidates).await?, interface)
        };

        Ok(Self {
            #[cfg(feature = "containers")]
            containers: ContainerContext {
                kube_client: kube::Client::try_default().await?,
                kube_service_port: Mutex::new(crate::sdk::utils::PortLock::lock(
                    config.kube_service_port(),
                )?),
                local_ip: local_ip_address::local_ip()?,
            },
            #[cfg(feature = "vm-sandbox")]
            vm: VmContext {
                network_manager,
                network_interface,
            },
            // Set in `run_blueprint_manager_with_keystore`
            db: Mutex::new(None),
            #[cfg(feature = "remote-providers")]
            cloud_config: Self::load_cloud_config(&config),
            config,
        })
    }

    pub async fn db(&self) -> Option<RocksDb> {
        self.db.lock().await.clone()
    }

    pub async fn set_db(&self, db: RocksDb) {
        self.db.lock().await.replace(db);
    }

    #[cfg(feature = "containers")]
    pub async fn kube_service_port(&self) -> u16 {
        let mut guard = self.containers.kube_service_port.lock().await;
        guard.unlock()
    }

    #[cfg(feature = "remote-providers")]
    pub fn cloud_config(&self) -> &Option<crate::executor::remote_provider_integration::CloudConfig> {
        &self.cloud_config
    }

    #[cfg(feature = "remote-providers")]
    fn load_cloud_config(config: &BlueprintManagerConfig) -> Option<crate::executor::remote_provider_integration::CloudConfig> {
        use std::env;
        use crate::executor::remote_provider_integration::{CloudConfig, AwsConfig, DigitalOceanConfig, VultrConfig};
        
        // Check for cloud provider environment variables
        let mut cloud_config = CloudConfig::default();
        let mut any_enabled = false;

        // AWS configuration
        if let (Ok(key), Ok(secret)) = (env::var("AWS_ACCESS_KEY_ID"), env::var("AWS_SECRET_ACCESS_KEY")) {
            cloud_config.aws = Some(AwsConfig {
                enabled: true,
                region: env::var("AWS_DEFAULT_REGION").unwrap_or_else(|_| "us-east-1".to_string()),
                access_key: key,
                secret_key: secret,
                priority: Some(10),
            });
            any_enabled = true;
        }

        // DigitalOcean configuration
        if let Ok(token) = env::var("DO_API_TOKEN") {
            cloud_config.digital_ocean = Some(DigitalOceanConfig {
                enabled: true,
                region: env::var("DO_DEFAULT_REGION").unwrap_or_else(|_| "nyc3".to_string()),
                api_token: token,
                priority: Some(5),
            });
            any_enabled = true;
        }

        // Vultr configuration
        if let Ok(key) = env::var("VULTR_API_KEY") {
            cloud_config.vultr = Some(VultrConfig {
                enabled: true,
                region: env::var("VULTR_DEFAULT_REGION").unwrap_or_else(|_| "ewr".to_string()),
                api_key: key,
                priority: Some(3),
            });
            any_enabled = true;
        }

        if any_enabled {
            cloud_config.enabled = true;
            Some(cloud_config)
        } else {
            None
        }
    }
}

impl Deref for BlueprintManagerContext {
    type Target = BlueprintManagerConfig;

    fn deref(&self) -> &Self::Target {
        &self.config
    }
}

impl DerefMut for BlueprintManagerContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.config
    }
}
