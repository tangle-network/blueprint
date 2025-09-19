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
    cloud_config: Option<blueprint_remote_providers::CloudConfig>,
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
    pub fn cloud_config(&self) -> &Option<blueprint_remote_providers::CloudConfig> {
        &self.cloud_config
    }

    #[cfg(feature = "remote-providers")]
    fn load_cloud_config(_config: &BlueprintManagerConfig) -> Option<blueprint_remote_providers::CloudConfig> {
        // Use the centralized config loading from remote providers crate
        blueprint_remote_providers::CloudConfig::from_env()
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
