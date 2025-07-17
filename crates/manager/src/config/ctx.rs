use crate::config::BlueprintManagerConfig;
use crate::error::Result;
use std::ops::{Deref, DerefMut};

#[cfg(feature = "tee")]
pub struct TeeContext {
    pub kube_client: kube::Client,
    pub kube_service_port: tokio::sync::Mutex<crate::sdk::utils::PortLock>,
    pub local_ip: std::net::IpAddr,
}

pub struct BlueprintManagerContext {
    #[cfg(feature = "tee")]
    pub tee: TeeContext,
    config: BlueprintManagerConfig,
}

impl BlueprintManagerContext {
    /// Create a new `BlueprintManagerContext`
    ///
    /// # Errors
    ///
    /// * Unable to create the necessary directories
    /// * With the `vm-sandbox` enabled
    ///   * Unable to determine the default network interface if not specified in `config`
    /// * With the `tee` feature enabled
    ///   * Unable to connect to the cluster
    ///   * Unable to bind to the [`BlueprintManagerConfig::kube_service_port()`]
    ///   * Unable to determine the system's local IPv4 address
    #[allow(clippy::unused_async)]
    pub async fn new(mut config: BlueprintManagerConfig) -> Result<Self> {
        config.paths.data_dir = std::path::absolute(config.data_dir())?;

        config.verify_directories_exist()?;
        #[cfg(feature = "vm-sandbox")]
        {
            config.verify_network_interface()?;
        }

        Ok(Self {
            #[cfg(feature = "tee")]
            tee: TeeContext {
                kube_client: kube::Client::try_default().await?,
                kube_service_port: tokio::sync::Mutex::new(crate::sdk::utils::PortLock::lock(
                    config.kube_service_port(),
                )?),
                local_ip: local_ip_address::local_ip()?,
            },
            config,
        })
    }

    #[cfg(feature = "tee")]
    pub async fn kube_service_port(&self) -> u16 {
        let mut guard = self.tee.kube_service_port.lock().await;
        guard.unlock()
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
