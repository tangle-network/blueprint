use super::bridge::{Bridge, BridgeHandle};
use super::hypervisor::net::NetworkManager;
use super::hypervisor::{HypervisorInstance, ServiceVmConfig};
use crate::error::{Error, Result};
use crate::sources::{BlueprintArgs, BlueprintEnvVars};
use blueprint_auth::db::RocksDb;
use blueprint_core::error;
use std::path::Path;
use std::time::Duration;
use tokio::sync::oneshot;
use tokio::time;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Status {
    Running,
    Finished,
    Error,
}

/// A service instance
pub struct Service {
    hypervisor: HypervisorInstance,
    bridge: BridgeHandle,
    alive_rx: Option<oneshot::Receiver<()>>,
}

impl Service {
    /// Create a new `Service` instance
    ///
    /// This will:
    /// * Spawn a [`Bridge`]
    /// * Configure and create a VM to be started with [`Self::start()`].
    ///
    /// # Errors
    ///
    /// See:
    /// * [`Bridge::spawn()`]
    /// * [`HypervisorInstance::new()`]
    /// * [`HypervisorInstance::prepare()`]
    #[allow(clippy::too_many_arguments)]
    pub async fn new(
        vm_config: ServiceVmConfig,
        network_manager: NetworkManager,
        db: RocksDb,
        data_dir: impl AsRef<Path>,
        keystore: impl AsRef<Path>,
        cache_dir: impl AsRef<Path>,
        runtime_dir: impl AsRef<Path>,
        service_name: &str,
        binary_path: impl AsRef<Path>,
        env_vars: BlueprintEnvVars,
        arguments: BlueprintArgs,
    ) -> Result<Service> {
        let bridge = Bridge::new(
            runtime_dir.as_ref().to_path_buf(),
            service_name.to_string(),
            db,
        );
        let bridge_base_socket = bridge.base_socket_path();

        let (bridge_handle, alive_rx) = bridge.spawn().map_err(|e| {
            error!("Failed to spawn manager <-> service bridge: {e}");
            e
        })?;

        let mut hypervisor = HypervisorInstance::new(
            vm_config,
            cache_dir.as_ref(),
            runtime_dir.as_ref(),
            service_name,
        )?;

        hypervisor
            .prepare(
                network_manager,
                keystore,
                data_dir,
                cache_dir,
                bridge_base_socket,
                binary_path.as_ref(),
                env_vars,
                arguments,
            )
            .await?;

        Ok(Self {
            hypervisor,
            bridge: bridge_handle,
            alive_rx: Some(alive_rx),
        })
    }

    /// Check the status of the running service
    ///
    /// If this returns an error, the service may be dead.
    ///
    /// # Errors
    ///
    /// * See [`HypervisorInstance::client()`]
    pub fn status(&self) -> Result<Status> {
        // TODO: A way to actually check the status
        Ok(Status::Running)
    }

    /// Starts the service and returns a health check future
    ///
    /// If the service is already started, it will return `None`.
    ///
    /// The future **should** be polled to completion to determine whether the VM can be connected to
    /// via the bridge.
    ///
    /// # Errors
    ///
    /// * See [`HypervisorInstance::start()`]
    pub async fn start(&mut self) -> Result<Option<impl Future<Output = Result<()>> + use<>>> {
        let Some(alive_rx) = self.alive_rx.take() else {
            error!("Service already started!");
            return Ok(None);
        };

        self.hypervisor.start().await.map_err(|e| {
            error!("Failed to start hypervisor: {e}");
            e
        })?;

        Ok(Some(async move {
            if time::timeout(Duration::from_secs(480), alive_rx)
                .await
                .is_err()
            {
                error!("Service never connected to bridge (network error?)");
                return Err(Error::Other("Bridge connection timeout".into()));
            }

            Ok(())
        }))
    }

    /// Gracefully shutdown the service (VM+bridge)
    ///
    /// # Errors
    ///
    /// See:
    ///
    /// * [`HypervisorInstance::shutdown()`]
    /// * [`BridgeHandle::shutdown()`]
    pub async fn shutdown(self) -> Result<()> {
        self.hypervisor.shutdown().await.map_err(|e| {
            error!("Failed to shut down hypervisor: {e}");
            e
        })?;
        self.bridge.shutdown();

        Ok(())
    }

    #[must_use]
    pub fn hypervisor(&self) -> &HypervisorInstance {
        &self.hypervisor
    }
}
