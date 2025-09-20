use crate::blueprint_manager_bridge_server::{
    BlueprintManagerBridge, BlueprintManagerBridgeServer,
};
use crate::client::VSOCK_PORT;
use crate::{
    AddOwnerToServiceRequest, Error, PortRequest, PortResponse,
    RegisterBlueprintServiceProxyRequest, RemoveOwnerFromServiceRequest,
    UnregisterBlueprintServiceProxyRequest,
};
use blueprint_auth::{
    db::RocksDb,
    models::{ServiceModel, ServiceOwnerModel},
    types::ServiceId,
};
use blueprint_core::{error, info};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::UnixListener;
use tokio::sync::{Mutex, oneshot};
use tokio::task::JoinHandle;
use tonic::codegen::tokio_stream::wrappers::UnixListenerStream;
use tonic::{Request, Response, transport::Server};

/// Handle to a running bridge
///
/// Dropping this handle will shut down the bridge.
pub struct BridgeHandle {
    sock_path: PathBuf,
    handle: JoinHandle<Result<(), Error>>,
}

impl BridgeHandle {
    pub fn shutdown(self) {}
}

impl Drop for BridgeHandle {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.sock_path);
        self.handle.abort();
    }
}

/// Manager <-> Service bridge
pub struct Bridge {
    runtime_dir: PathBuf,
    service_name: String,
    db: RocksDb,
    no_vm: bool,
}

impl Bridge {
    #[must_use]
    pub fn new(runtime_dir: PathBuf, service_name: String, db: RocksDb, no_vm: bool) -> Self {
        Self {
            runtime_dir,
            service_name,
            db,
            no_vm,
        }
    }

    /// The *base* bridge socket
    ///
    /// For native services, this is the only socket that exists.
    ///
    /// For sandboxed services, this is the base path. When a guest connects to the bridge, it will
    /// be through the socket at `<base>_VSOCK_PORT`. See [`Self::guest_socket_path()`].
    #[must_use]
    pub fn base_socket_path(&self) -> PathBuf {
        let sock_name = format!("{}.sock", self.service_name);
        self.runtime_dir.join(sock_name)
    }

    /// The socket path on the *host* for guest connections
    ///
    /// This socket is only created for sandboxed services. It will **not** exist for native services.
    #[must_use]
    pub fn guest_socket_path(&self) -> PathBuf {
        let sock_name = format!("{}.sock_{VSOCK_PORT}", self.service_name);
        self.runtime_dir.join(sock_name)
    }
}

impl Bridge {
    /// Spawn the bridge instance
    ///
    /// # Errors
    ///
    /// * Unable to bind to the socket, possibly an issue with the [`HypervisorInstance`] startup.
    ///
    /// [`HypervisorInstance`]: https://docs.rs/blueprint-manager/latest/blueprint_manager/rt/struct.HypervisorInstance.html
    pub fn spawn(self) -> Result<(BridgeHandle, oneshot::Receiver<()>), Error> {
        let (sock_path, listener) = if self.no_vm {
            self.spawn_for_native()?
        } else {
            self.spawn_for_vm()?
        };

        let (tx, rx) = oneshot::channel();

        let handle = tokio::task::spawn(async move {
            Server::builder()
                .add_service(BlueprintManagerBridgeServer::new(BridgeService::new(
                    tx, self.db,
                )))
                .serve_with_incoming(UnixListenerStream::new(listener))
                .await
                .map_err(Error::from)
        });

        Ok((BridgeHandle { sock_path, handle }, rx))
    }

    fn spawn_for_vm(&self) -> Result<(PathBuf, UnixListener), Error> {
        let sock_path = self.guest_socket_path();
        let _ = std::fs::remove_file(&sock_path);
        let listener = UnixListener::bind(&sock_path).map_err(|e| {
            error!(
                "Failed to bind bridge socket at {}: {e}",
                sock_path.display()
            );
            e
        })?;

        info!(
            "Connected to bridge for service `{}`, listening on VSOCK port {VSOCK_PORT}",
            self.service_name
        );

        Ok((sock_path, listener))
    }

    fn spawn_for_native(&self) -> Result<(PathBuf, UnixListener), Error> {
        let sock_path = self.base_socket_path();
        let _ = std::fs::remove_file(&sock_path);
        let listener = UnixListener::bind(&sock_path).map_err(|e| {
            error!(
                "Failed to bind bridge socket at {}: {e}",
                sock_path.display()
            );
            e
        })?;

        info!(
            "Connected to bridge for service `{}`, listening on socket `{}`",
            self.service_name,
            sock_path.display()
        );

        Ok((sock_path, listener))
    }
}

struct BridgeService {
    ready_tx: Arc<Mutex<Option<oneshot::Sender<()>>>>,
    db: RocksDb,
}

impl BridgeService {
    fn new(tx: oneshot::Sender<()>, db: RocksDb) -> Self {
        Self {
            ready_tx: Arc::new(Mutex::new(Some(tx))),
            db,
        }
    }

    async fn signal_ready(&self) {
        if let Some(tx) = self.ready_tx.lock().await.take() {
            let _ = tx.send(());
        }
    }
}

#[tonic::async_trait]
impl BlueprintManagerBridge for BridgeService {
    async fn ping(&self, _req: Request<()>) -> Result<Response<()>, tonic::Status> {
        self.signal_ready().await;
        Ok(Response::new(()))
    }

    #[allow(clippy::cast_possible_truncation)]
    async fn request_port(
        &self,
        req: Request<PortRequest>,
    ) -> Result<Response<PortResponse>, tonic::Status> {
        let PortRequest { preferred_port, .. } = req.into_inner();

        let port = allocate_host_port(preferred_port as u16).await?;

        Ok(Response::new(PortResponse {
            port: u32::from(port),
        }))
    }

    async fn register_blueprint_service_proxy(
        &self,
        req: Request<RegisterBlueprintServiceProxyRequest>,
    ) -> Result<Response<()>, tonic::Status> {
        let RegisterBlueprintServiceProxyRequest {
            service_id,
            api_key_prefix,
            upstream_url,
            owners,
        } = req.into_inner();

        let db = &self.db;

        // Convert protobuf owners to ServiceOwnerModel
        let service_owners: Vec<ServiceOwnerModel> = owners
            .into_iter()
            .map(|owner| ServiceOwnerModel {
                key_type: owner.key_type,
                key_bytes: owner.key_bytes,
            })
            .collect();
        let service = ServiceModel {
            api_key_prefix,
            owners: service_owners,
            upstream_url,
            tls_profile: None,
        };

        // Save to database
        let service_id = ServiceId(service_id, 0);
        service.save(service_id, db).map_err(|e| {
            error!("Failed to save service to database: {e}");
            tonic::Status::internal(format!("Database error: {e}"))
        })?;

        info!("Registered service proxy with ID: {}", service_id);
        Ok(Response::new(()))
    }

    async fn unregister_blueprint_service_proxy(
        &self,
        req: Request<UnregisterBlueprintServiceProxyRequest>,
    ) -> Result<Response<()>, tonic::Status> {
        let UnregisterBlueprintServiceProxyRequest { service_id } = req.into_inner();

        let db = &self.db;

        let service_id = ServiceId(service_id, 0);

        // Delete from database
        ServiceModel::delete(service_id, db).map_err(|e| {
            error!("Failed to delete service {} from database: {e}", service_id);
            tonic::Status::internal(format!("Database error: {e}"))
        })?;

        info!("Unregistered service proxy with ID: {}", service_id);
        Ok(Response::new(()))
    }

    async fn add_owner_to_service(
        &self,
        req: Request<AddOwnerToServiceRequest>,
    ) -> Result<Response<()>, tonic::Status> {
        let AddOwnerToServiceRequest {
            service_id,
            owner_to_add,
        } = req.into_inner();

        let db = &self.db;

        let service_id = ServiceId(service_id, 0);
        let new_owner_proto = owner_to_add.ok_or_else(|| {
            error!(
                "Owner is missing in AddOwnerToServiceRequest for service ID: {}",
                service_id
            );
            tonic::Status::invalid_argument("owner_to_add is required")
        })?;

        // Load existing service
        let mut service = ServiceModel::find_by_id(service_id, db)
            .map_err(|e| {
                error!("Failed to load service {} from database: {e}", service_id);
                tonic::Status::internal(format!("Database error: {e}"))
            })?
            .ok_or_else(|| {
                error!("Service {} not found for add_owner_to_service", service_id);
                tonic::Status::not_found(format!("Service {} not found", service_id))
            })?;

        // Convert protobuf owner to ServiceOwnerModel
        let new_owner = ServiceOwnerModel {
            key_type: new_owner_proto.key_type,
            key_bytes: new_owner_proto.key_bytes,
        };

        // Add owner if not already present
        if service.owners.contains(&new_owner) {
            info!("Owner already exists for service ID: {service_id}");
        } else {
            service.owners.push(new_owner);
            // Save updated service
            service.save(service_id, db).map_err(|e| {
                error!("Failed to save updated service {service_id} to database: {e}",);
                tonic::Status::internal(format!("Database error: {e}"))
            })?;
            info!("Added owner to service ID: {service_id}");
        }

        Ok(Response::new(()))
    }

    async fn remove_owner_from_service(
        &self,
        req: Request<RemoveOwnerFromServiceRequest>,
    ) -> Result<Response<()>, tonic::Status> {
        let RemoveOwnerFromServiceRequest {
            service_id,
            owner_to_remove,
        } = req.into_inner();

        let db = &self.db;

        let service_id = ServiceId(service_id, 0);
        let owner_to_remove_proto = owner_to_remove.ok_or_else(|| {
            error!(
                "Owner is missing in RemoveOwnerFromServiceRequest for service ID: {}",
                service_id
            );
            tonic::Status::invalid_argument("owner_to_remove is required")
        })?;

        // Load existing service
        let mut service = ServiceModel::find_by_id(service_id, db)
            .map_err(|e| {
                error!("Failed to load service {} from database: {e}", service_id);
                tonic::Status::internal(format!("Database error: {e}"))
            })?
            .ok_or_else(|| {
                error!(
                    "Service {} not found for remove_owner_from_service",
                    service_id
                );
                tonic::Status::not_found(format!("Service {} not found", service_id))
            })?;

        // Convert protobuf owner to ServiceOwnerModel
        let owner_to_remove = ServiceOwnerModel {
            key_type: owner_to_remove_proto.key_type,
            key_bytes: owner_to_remove_proto.key_bytes,
        };

        // Remove owner
        let initial_len = service.owners.len();
        service.owners.retain(|o| o != &owner_to_remove);

        if service.owners.len() < initial_len {
            // Save updated service
            service.save(service_id, db).map_err(|e| {
                error!(
                    "Failed to save updated service {} to database: {e}",
                    service_id
                );
                tonic::Status::internal(format!("Database error: {e}"))
            })?;
            info!("Removed owner from service ID: {}", service_id);
        } else {
            info!(
                "Owner not found for service ID: {}, nothing to remove",
                service_id
            );
        }

        Ok(Response::new(()))
    }
}

// TODO: Actually allocate a port to the VM
#[expect(clippy::unused_async, reason = "This isn't actually setup yet")]
async fn allocate_host_port(hint: u16) -> Result<u16, tonic::Status> {
    let listener = std::net::TcpListener::bind(format!("0.0.0.0:{hint}"))
        .map_err(|e| tonic::Status::unavailable(e.to_string()))?;
    let port = listener
        .local_addr()
        .expect("Should have a local address")
        .port();
    drop(listener);
    Ok(port)
}
