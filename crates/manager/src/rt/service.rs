use super::bridge::{Bridge, BridgeHandle};
use super::hypervisor::HypervisorInstance;
use super::hypervisor::net::NetworkManager;
use crate::error::{Error, Result};
use crate::sources::{BlueprintArgs, BlueprintEnvVars};
use std::path::Path;
use std::time::Duration;
use tokio::sync::oneshot;
use tokio::time;
use tracing::error;

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
    #[allow(clippy::too_many_arguments)]
    pub async fn new(
        id: u32,
        network_manager: NetworkManager,
        data_dir: impl AsRef<Path>,
        keystore: impl AsRef<Path>,
        cache_dir: impl AsRef<Path>,
        runtime_dir: impl AsRef<Path>,
        pty_slave_path: Option<&Path>,
        service_name: &str,
        binary_path: impl AsRef<Path>,
        env_vars: BlueprintEnvVars,
        arguments: BlueprintArgs,
    ) -> Result<Service> {
        let db_path = data_dir
            .as_ref()
            .join("private")
            .join("auth-proxy")
            .join("db");
        tokio::fs::create_dir_all(&db_path).await.map_err(|e| {
            error!(
                "Failed to create database directory at {}: {e}",
                db_path.display()
            );
            Error::Other(format!("Failed to create database directory: {e}"))
        })?;

        let bridge = Bridge::new(
            runtime_dir.as_ref().to_path_buf(),
            service_name.to_string(),
            db_path,
        );
        let bridge_base_socket = bridge.base_socket_path();

        let (bridge_handle, alive_rx) = bridge.spawn().map_err(|e| {
            error!("Failed to spawn manager <-> service bridge: {e}");
            e
        })?;

        let mut hypervisor = HypervisorInstance::new(
            data_dir,
            keystore,
            cache_dir.as_ref(),
            runtime_dir.as_ref(),
            service_name,
        )?;

        hypervisor
            .prepare(
                id,
                network_manager,
                bridge_base_socket,
                pty_slave_path,
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
            if time::timeout(Duration::from_secs(30), alive_rx)
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
    pub async fn shutdown(self) -> Result<()> {
        self.hypervisor.shutdown().await.map_err(|e| {
            error!("Failed to shut down hypervisor: {e}");
            e
        })?;
        self.bridge.shutdown();

        Ok(())
    }
}
