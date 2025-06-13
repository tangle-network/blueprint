use super::bridge::{Bridge, BridgeHandle};
use super::hypervisor::net::NetworkManager;
use super::hypervisor::{HypervisorInstance, ServiceVmConfig};
use super::native::ProcessHandle;
use crate::error::{Error, Result};
use crate::sources::{BlueprintArgs, BlueprintEnvVars};
use blueprint_auth::db::RocksDb;
use blueprint_core::error;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::sync::oneshot;
use tokio::time;
use tracing::{info, warn};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Status {
    NotStarted,
    Running,
    Finished,
    Error,
}

struct NativeProcessInfo {
    binary_path: PathBuf,
    service_name: String,
    env_vars: BlueprintEnvVars,
    arguments: BlueprintArgs,
}

enum NativeProcess {
    NotStarted(NativeProcessInfo),
    Started(ProcessHandle),
}

enum Runtime {
    Hypervisor(HypervisorInstance),
    Native(NativeProcess),
}

/// A service instance
pub struct Service {
    runtime: Runtime,
    bridge: BridgeHandle,
    alive_rx: Option<oneshot::Receiver<()>>,
}

fn create_bridge(
    runtime_dir: impl AsRef<Path>,
    service_name: &str,
    db: RocksDb,
    no_vm: bool,
) -> Result<(PathBuf, BridgeHandle, oneshot::Receiver<()>)> {
    let bridge = Bridge::new(
        runtime_dir.as_ref().to_path_buf(),
        service_name.to_string(),
        db,
        no_vm,
    );
    let bridge_base_socket = bridge.base_socket_path();

    let (handle, alive_rx) = bridge.spawn().map_err(|e| {
        error!("Failed to spawn manager <-> service bridge: {e}");
        e
    })?;

    Ok((bridge_base_socket, handle, alive_rx))
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
        mut env_vars: BlueprintEnvVars,
        arguments: BlueprintArgs,
    ) -> Result<Service> {
        // Not used by sandboxed services
        env_vars.bridge_socket_path = None;

        let (bridge_base_socket, bridge_handle, alive_rx) =
            create_bridge(runtime_dir.as_ref(), service_name, db, false)?;

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
            runtime: Runtime::Hypervisor(hypervisor),
            bridge: bridge_handle,
            alive_rx: Some(alive_rx),
        })
    }

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
    pub fn new_native(
        db: RocksDb,
        runtime_dir: impl AsRef<Path>,
        service_name: &str,
        binary_path: impl AsRef<Path>,
        mut env_vars: BlueprintEnvVars,
        arguments: BlueprintArgs,
    ) -> Result<Service> {
        let (bridge_base_socket, bridge_handle, alive_rx) =
            create_bridge(runtime_dir.as_ref(), service_name, db, true)?;

        env_vars.bridge_socket_path = Some(bridge_base_socket);

        Ok(Self {
            runtime: Runtime::Native(NativeProcess::NotStarted(NativeProcessInfo {
                binary_path: binary_path.as_ref().to_path_buf(),
                service_name: service_name.to_string(),
                env_vars,
                arguments,
            })),
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
    /// * See [`HypervisorInstance::statuc()`]
    /// * See [`ProcessHandle::status()`]
    pub async fn status(&mut self) -> Result<Status> {
        match &mut self.runtime {
            Runtime::Hypervisor(hypervisor) => hypervisor.status().await,
            Runtime::Native(NativeProcess::Started(instance)) => Ok(instance.status()),
            Runtime::Native(NativeProcess::NotStarted(_)) => Ok(Status::NotStarted),
        }
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

        match &mut self.runtime {
            Runtime::Hypervisor(hypervisor) => {
                hypervisor.start().await.map_err(|e| {
                    error!("Failed to start hypervisor: {e}");
                    e
                })?;
            }
            Runtime::Native(instance) => match instance {
                NativeProcess::NotStarted(info) => {
                    let process_handle = tokio::process::Command::new(&info.binary_path)
                        .kill_on_drop(true)
                        .stdin(std::process::Stdio::null())
                        .current_dir(&std::env::current_dir()?)
                        .envs(info.env_vars.encode())
                        .args(info.arguments.encode())
                        .spawn()?;

                    let handle =
                        generate_running_process_status_handle(process_handle, &info.service_name);
                    *instance = NativeProcess::Started(handle);
                }
                NativeProcess::Started(_) => {
                    error!("Service already started!");
                    return Ok(None);
                }
            },
        }

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
        match self.runtime {
            Runtime::Hypervisor(hypervisor) => {
                hypervisor.shutdown().await.map_err(|e| {
                    error!("Failed to shut down hypervisor: {e}");
                    e
                })?;
            }
            Runtime::Native(NativeProcess::Started(instance)) => {
                if !instance.abort() {
                    error!("Failed to abort service");
                }
            }
            _ => warn!("No process attached"),
        }

        self.bridge.shutdown();

        Ok(())
    }

    #[must_use]
    pub fn hypervisor(&self) -> Option<&HypervisorInstance> {
        match &self.runtime {
            Runtime::Hypervisor(hypervisor) => Some(hypervisor),
            _ => None,
        }
    }
}

#[must_use]
fn generate_running_process_status_handle(
    process: tokio::process::Child,
    service_name: &str,
) -> ProcessHandle {
    let (abort_tx, abort_rx) = tokio::sync::oneshot::channel::<()>();
    let (status_tx, status_rx) = tokio::sync::mpsc::unbounded_channel::<Status>();
    let service_name = service_name.to_string();

    let task = async move {
        info!("Starting process execution for {service_name}");
        let _ = status_tx.send(Status::Running);
        let output = process.wait_with_output().await;
        if output.is_ok() {
            let _ = status_tx.send(Status::Finished).ok();
        } else {
            let _ = status_tx.send(Status::Error).ok();
        }

        warn!("Process for {service_name} exited: {output:?}");
    };

    let task = async move {
        tokio::select! {
            _ = abort_rx => {},
            () = task => {},
        }
    };

    tokio::spawn(task);

    ProcessHandle::new(status_rx, abort_tx)
}
