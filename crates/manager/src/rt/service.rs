#[cfg(feature = "vm-sandbox")]
use super::hypervisor::{HypervisorInstance, ServiceVmConfig, net::NetworkManager};
use super::native::ProcessHandle;
use crate::config::BlueprintManagerContext;
use crate::error::{Error, Result};
#[cfg(feature = "containers")]
use crate::rt::container::ContainerInstance;
use crate::sources::{BlueprintArgs, BlueprintEnvVars};
use blueprint_core::error;
use blueprint_core::{info, warn};
use blueprint_manager_bridge::server::{Bridge, BridgeHandle};
use blueprint_runner::config::BlueprintEnvironment;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::sync::oneshot;
use tokio::time;
use crate::rt::ResourceLimits;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Status {
    NotStarted,
    Pending,
    Running,
    Finished,
    Error,
    Unknown,
}

struct NativeProcessInfo {
    limits: ResourceLimits,
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
    #[cfg(feature = "vm-sandbox")]
    Hypervisor(HypervisorInstance),
    #[cfg(feature = "containers")]
    Container(ContainerInstance),
    Native(NativeProcess),
}

/// A service instance
pub struct Service {
    runtime: Runtime,
    bridge: BridgeHandle,
    alive_rx: Option<oneshot::Receiver<()>>,
}

async fn create_bridge(
    ctx: &BlueprintManagerContext,
    runtime_dir: impl AsRef<Path>,
    service_name: &str,
    no_vm: bool,
) -> Result<(PathBuf, BridgeHandle, oneshot::Receiver<()>)> {
    let db = ctx
        .db()
        .await
        .expect("not possible to get to this point without a db set");

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
    /// Create a new `Service` instance for the given binary at `binary_path`
    ///
    /// This is the same as calling [`Service::new_vm()`] or [`Service::new_native()`], with the runtime
    /// being determined by the [`BlueprintManagerContext`] and enabled features.
    ///
    /// # Errors
    ///
    /// See:
    /// * [`Service::new_vm()`]
    /// * [`Service::new_native()`]
    pub async fn from_binary(
        ctx: &BlueprintManagerContext,
        limits: ResourceLimits,
        blueprint_config: &BlueprintEnvironment,
        id: u32,
        env: BlueprintEnvVars,
        args: BlueprintArgs,
        binary_path: &Path,
        sub_service_str: &str,
        cache_dir: &Path,
        runtime_dir: &Path,
    ) -> Result<Service> {
        #[cfg(feature = "vm-sandbox")]
        if !ctx.vm_sandbox_options.no_vm {
            return Service::new_vm(
                ctx,
                limits,
                // TODO: !!! Actually configure the VM with resource limits
                ServiceVmConfig {
                    id,
                    ..Default::default()
                },
                &blueprint_config.data_dir,
                &blueprint_config.keystore_uri,
                cache_dir,
                runtime_dir,
                sub_service_str,
                binary_path,
                env,
                args,
            )
            .await;
        }

        Service::new_native(ctx, limits, runtime_dir, sub_service_str, binary_path, env, args).await
    }

    /// Create a new `Service` instance, sandboxed via `cloud-hypervisor`
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
    #[cfg(feature = "vm-sandbox")]
    pub async fn new_vm(
        ctx: &BlueprintManagerContext,
        limits: ResourceLimits,
        vm_config: ServiceVmConfig,
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
            create_bridge(ctx, runtime_dir.as_ref(), service_name, false).await?;

        let mut hypervisor = HypervisorInstance::new(
            ctx,
            limits,
            vm_config,
            cache_dir.as_ref(),
            runtime_dir.as_ref(),
            service_name,
        )?;

        hypervisor
            .prepare(
                ctx,
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

    /// Create a new `Service` instance, sandboxed via `kata-containers`
    ///
    /// This will:
    /// * Spawn a [`Bridge`]
    /// * Configure and create a VM to be started with [`Self::start()`].
    ///
    /// # Errors
    ///
    /// See:
    /// * [`Bridge::spawn()`]
    /// * [`ContainerInstance::new()`]
    /// * [`ContainerInstance::prepare()`]
    #[allow(clippy::too_many_arguments)]
    #[cfg(feature = "containers")]
    pub async fn new_container(
        ctx: &BlueprintManagerContext,
        limits: ResourceLimits,
        runtime_dir: impl AsRef<Path>,
        service_name: &str,
        image: String,
        mut env_vars: BlueprintEnvVars,
        arguments: BlueprintArgs,
        debug: bool,
    ) -> Result<Service> {
        let (bridge_base_socket, bridge_handle, alive_rx) =
            create_bridge(ctx, runtime_dir.as_ref(), service_name, false).await?;

        env_vars.bridge_socket_path = Some(bridge_base_socket);

        let instance =
            ContainerInstance::new(ctx, limits, service_name, image, env_vars, arguments, debug)
                .await;

        Ok(Self {
            runtime: Runtime::Container(instance),
            bridge: bridge_handle,
            alive_rx: Some(alive_rx),
        })
    }

    /// Create a new `Service` instance **with no sandbox**
    ///
    /// NOTE: This should only be used for local testing.
    ///
    /// This will spawn a [`Bridge`] in preparation for the service to be started.
    ///
    /// # Errors
    ///
    /// See:
    /// * [`Bridge::spawn()`]
    #[allow(clippy::too_many_arguments)]
    pub async fn new_native(
        ctx: &BlueprintManagerContext,
        limits: ResourceLimits,
        runtime_dir: impl AsRef<Path>,
        service_name: &str,
        binary_path: impl AsRef<Path>,
        mut env_vars: BlueprintEnvVars,
        arguments: BlueprintArgs,
    ) -> Result<Service> {
        let (bridge_base_socket, bridge_handle, alive_rx) =
            create_bridge(ctx, runtime_dir.as_ref(), service_name, true).await?;

        env_vars.bridge_socket_path = Some(bridge_base_socket);

        Ok(Self {
            runtime: Runtime::Native(NativeProcess::NotStarted(NativeProcessInfo {
                limits,
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
    /// * See [`HypervisorInstance::status()`]
    /// * See [`ProcessHandle::status()`]
    pub async fn status(&mut self) -> Result<Status> {
        match &mut self.runtime {
            #[cfg(feature = "vm-sandbox")]
            Runtime::Hypervisor(hypervisor) => hypervisor.status().await,
            #[cfg(feature = "containers")]
            Runtime::Container(container) => container.status().await,
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
            #[cfg(feature = "vm-sandbox")]
            Runtime::Hypervisor(hypervisor) => {
                hypervisor.start().await.map_err(|e| {
                    error!("Failed to start hypervisor: {e}");
                    e
                })?;
            }
            #[cfg(feature = "containers")]
            Runtime::Container(container) => {
                container.start().await.map_err(|e| {
                    error!("Failed to start container: {e}");
                    e
                })?;
            }
            Runtime::Native(instance) => match instance {
                NativeProcess::NotStarted(info) => {
                    // TODO: Resource limits
                    let process_handle = tokio::process::Command::new(&info.binary_path)
                        .kill_on_drop(true)
                        .stdin(std::process::Stdio::null())
                        .current_dir(&std::env::current_dir()?)
                        .envs(info.env_vars.encode())
                        .args(info.arguments.encode(true))
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
            #[cfg(feature = "vm-sandbox")]
            Runtime::Hypervisor(hypervisor) => {
                hypervisor.shutdown().await.map_err(|e| {
                    error!("Failed to shut down hypervisor: {e}");
                    e
                })?;
            }
            #[cfg(feature = "containers")]
            Runtime::Container(container) => {
                container.shutdown().await.map_err(|e| {
                    error!("Failed to shut down container instance: {e}");
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
    #[cfg(feature = "vm-sandbox")]
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
