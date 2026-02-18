use crate::VSOCK_PORT;
use crate::blueprint_manager_bridge_server::{
    BlueprintManagerBridge, BlueprintManagerBridgeServer,
};
use crate::{
    AddOwnerToServiceRequest, Error, PortRequest, PortResponse,
    RegisterBlueprintServiceProxyRequest, RemoveOwnerFromServiceRequest,
    UnregisterBlueprintServiceProxyRequest, UpdateBlueprintServiceTlsProfileRequest,
};
use blueprint_auth::{
    db::RocksDb,
    models::{ServiceModel, ServiceOwnerModel},
    types::ServiceId,
};
use blueprint_core::{error, info, warn};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::UnixListener;
use tokio::sync::{Mutex, oneshot};
use tokio::task::JoinHandle;
use tonic::codegen::tokio_stream::wrappers::UnixListenerStream;
use tonic::{Request, Response, transport::Server};

/// Handle to a running bridge
///
/// Dropping this handle will shut down the bridge and clean up any registered service.
pub struct BridgeHandle {
    sock_path: PathBuf,
    handle: JoinHandle<Result<(), Error>>,
    db: RocksDb,
    registered_service_id: Arc<std::sync::Mutex<Option<u64>>>,
}

impl BridgeHandle {
    pub fn shutdown(self) {}
}

impl Drop for BridgeHandle {
    fn drop(&mut self) {
        // Clean up any registered service from the database
        if let Some(service_id) = self
            .registered_service_id
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .take()
        {
            let sid = ServiceId(service_id, 0);
            if let Err(e) = ServiceModel::delete(sid, &self.db) {
                warn!("Failed to clean up service {sid} on bridge drop: {e}");
            } else {
                info!("Cleaned up service {sid} registration on bridge shutdown");
            }
        }
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
    expected_service_id: Option<u64>,
}

impl Bridge {
    #[must_use]
    pub fn new(
        runtime_dir: PathBuf,
        service_name: String,
        db: RocksDb,
        no_vm: bool,
        expected_service_id: Option<u64>,
    ) -> Self {
        Self {
            runtime_dir,
            service_name,
            db,
            no_vm,
            expected_service_id,
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

    /// Spawn the bridge instance
    ///
    /// # Errors
    ///
    /// * Unable to bind to the socket, possibly an issue with the [`HypervisorInstance`] startup.
    ///
    /// [`HypervisorInstance`]: https://docs.rs/blueprint-manager/latest/blueprint_manager/rt/struct.HypervisorInstance.html
    pub fn spawn(self) -> Result<(BridgeHandle, oneshot::Receiver<()>), Error> {
        let sock_path = if self.no_vm {
            self.base_socket_path()
        } else {
            self.guest_socket_path()
        };

        let _ = std::fs::remove_file(&sock_path);
        let listener = UnixListener::bind(&sock_path).map_err(|e| {
            error!(
                "Failed to bind bridge socket at {}: {e}",
                sock_path.display()
            );
            e
        })?;

        info!(
            "Bridge for service `{}` listening on {}",
            self.service_name,
            sock_path.display()
        );

        let (tx, rx) = oneshot::channel();
        let registered_service_id = Arc::new(std::sync::Mutex::new(None));
        let registered_service_id_clone = Arc::clone(&registered_service_id);
        let db_clone = self.db.clone();

        let handle = tokio::task::spawn(async move {
            Server::builder()
                .add_service(BlueprintManagerBridgeServer::new(
                    BridgeService::with_service_pinning(
                        tx,
                        self.db,
                        self.expected_service_id,
                        registered_service_id_clone,
                    ),
                ))
                .serve_with_incoming(UnixListenerStream::new(listener))
                .await
                .map_err(Error::from)
        });

        Ok((
            BridgeHandle {
                sock_path,
                handle,
                db: db_clone,
                registered_service_id,
            },
            rx,
        ))
    }
}

struct BridgeService {
    ready_tx: Arc<Mutex<Option<oneshot::Sender<()>>>>,
    db: RocksDb,
    expected_service_id: Option<u64>,
    registered_service_id: Arc<std::sync::Mutex<Option<u64>>>,
}

impl BridgeService {
    fn new(tx: oneshot::Sender<()>, db: RocksDb) -> Self {
        Self {
            ready_tx: Arc::new(Mutex::new(Some(tx))),
            db,
            expected_service_id: None,
            registered_service_id: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    fn with_service_pinning(
        tx: oneshot::Sender<()>,
        db: RocksDb,
        expected_service_id: Option<u64>,
        registered_service_id: Arc<std::sync::Mutex<Option<u64>>>,
    ) -> Self {
        Self {
            ready_tx: Arc::new(Mutex::new(Some(tx))),
            db,
            expected_service_id,
            registered_service_id,
        }
    }

    async fn signal_ready(&self) {
        if let Some(tx) = self.ready_tx.lock().await.take() {
            let _ = tx.send(());
        }
    }

    /// Verify that the given service_id is allowed for this bridge.
    ///
    /// Checks the BPM-provided expected_service_id constraint and the self-pinning constraint.
    fn verify_service_id(&self, service_id: u64) -> Result<(), tonic::Status> {
        // Check BPM-provided constraint
        if let Some(expected) = self.expected_service_id {
            if service_id != expected {
                return Err(tonic::Status::permission_denied(format!(
                    "Bridge is configured for service_id {expected}, got {service_id}"
                )));
            }
        }

        // Check self-pinning constraint
        let pinned = self
            .registered_service_id
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        if let Some(pinned_id) = *pinned {
            if service_id != pinned_id {
                return Err(tonic::Status::permission_denied(format!(
                    "Bridge is pinned to service_id {pinned_id}, cannot operate on {service_id}"
                )));
            }
        }

        Ok(())
    }

    /// Verify that this bridge has previously registered a service and the given service_id matches.
    fn verify_registered_service_id(&self, service_id: u64) -> Result<(), tonic::Status> {
        self.verify_service_id(service_id)?;

        let pinned = self
            .registered_service_id
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        if pinned.is_none() {
            return Err(tonic::Status::failed_precondition(
                "No service registered on this bridge",
            ));
        }

        Ok(())
    }
}

fn validate_tls_profile(profile: &blueprint_auth::models::TlsProfile) -> Result<(), tonic::Status> {
    if !profile.tls_enabled {
        return Ok(());
    }

    if profile.encrypted_server_cert.is_empty() || profile.encrypted_server_key.is_empty() {
        return Err(tonic::Status::invalid_argument(
            "Server certificate and key are required when TLS is enabled",
        ));
    }

    if profile.require_client_mtls && profile.encrypted_client_ca_bundle.is_empty() {
        return Err(tonic::Status::invalid_argument(
            "Client CA bundle is required when mutual TLS authentication is enabled",
        ));
    }

    Ok(())
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
            tls_profile,
        } = req.into_inner();

        // Verify this bridge is allowed to operate on this service_id
        self.verify_service_id(service_id)?;

        let db = &self.db;

        // Convert protobuf owners to ServiceOwnerModel
        let service_owners: Vec<ServiceOwnerModel> = owners
            .into_iter()
            .map(|owner| ServiceOwnerModel {
                key_type: owner.key_type,
                key_bytes: owner.key_bytes,
            })
            .collect();

        let tls_profile: Option<blueprint_auth::models::TlsProfile> = tls_profile.map(Into::into);
        if let Some(ref profile) = tls_profile {
            validate_tls_profile(profile)?;
        }

        let service = ServiceModel {
            api_key_prefix,
            owners: service_owners,
            upstream_url,
            tls_profile,
        };

        let service_id = ServiceId(service_id, 0);

        // First-time registration uses an atomic check-then-write to prevent
        // cross-bridge hijacking races. Updates (bridge already pinned) use plain save.
        let is_update = self
            .registered_service_id
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .is_some();

        if is_update {
            service.save(service_id, db).map_err(|e| {
                error!("Failed to save service to database: {e}");
                tonic::Status::internal(format!("Database error: {e}"))
            })?;
        } else {
            let saved = service.save_if_absent(service_id, db).map_err(|e| {
                error!("Failed to save service to database: {e}");
                tonic::Status::internal(format!("Database error: {e}"))
            })?;
            if !saved {
                return Err(tonic::Status::already_exists(format!(
                    "Service {service_id} is already registered by another bridge"
                )));
            }
        }

        // Pin this bridge to the registered service
        {
            let mut pinned = self
                .registered_service_id
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            *pinned = Some(service_id.0);
        }

        info!("Registered service proxy with ID: {}", service_id);
        Ok(Response::new(()))
    }

    async fn unregister_blueprint_service_proxy(
        &self,
        req: Request<UnregisterBlueprintServiceProxyRequest>,
    ) -> Result<Response<()>, tonic::Status> {
        let UnregisterBlueprintServiceProxyRequest { service_id } = req.into_inner();

        // Verify this bridge owns this service
        self.verify_registered_service_id(service_id)?;

        let db = &self.db;

        let service_id = ServiceId(service_id, 0);

        // Delete from database
        ServiceModel::delete(service_id, db).map_err(|e| {
            error!("Failed to delete service {} from database: {e}", service_id);
            tonic::Status::internal(format!("Database error: {e}"))
        })?;

        // Clear the pin
        {
            let mut pinned = self
                .registered_service_id
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            *pinned = None;
        }

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

        // Verify this bridge owns this service
        self.verify_registered_service_id(service_id)?;

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

        // Verify this bridge owns this service
        self.verify_registered_service_id(service_id)?;

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

    async fn update_blueprint_service_tls_profile(
        &self,
        req: Request<UpdateBlueprintServiceTlsProfileRequest>,
    ) -> Result<Response<()>, tonic::Status> {
        let UpdateBlueprintServiceTlsProfileRequest {
            service_id,
            tls_profile,
        } = req.into_inner();

        // Verify this bridge owns this service
        self.verify_registered_service_id(service_id)?;

        let db = &self.db;
        let service_id = ServiceId(service_id, 0);

        // Load existing service
        let mut service = ServiceModel::find_by_id(service_id, db)
            .map_err(|e| {
                error!("Failed to load service {} from database: {e}", service_id);
                tonic::Status::internal(format!("Database error: {e}"))
            })?
            .ok_or_else(|| {
                error!(
                    "Service {} not found for update_blueprint_service_tls_profile",
                    service_id
                );
                tonic::Status::not_found(format!("Service {} not found", service_id))
            })?;

        let new_tls_profile: Option<blueprint_auth::models::TlsProfile> =
            tls_profile.map(Into::into);
        if let Some(ref profile) = new_tls_profile {
            validate_tls_profile(profile)?;
        }

        service.tls_profile = new_tls_profile;

        // Save updated service
        service.save(service_id, db).map_err(|e| {
            error!(
                "Failed to save updated service {} TLS profile to database: {e}",
                service_id
            );
            tonic::Status::internal(format!("Database error: {e}"))
        })?;

        info!("Updated TLS profile for service ID: {}", service_id);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TlsProfileConfig;
    use blueprint_auth::db::{RocksDb, RocksDbConfig};
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_tls_profile_validation_server_tls_required() {
        let tmp_dir = tempdir().unwrap();
        let db = RocksDb::open(tmp_dir.path(), &RocksDbConfig::default()).unwrap();
        let service = BridgeService::new(tokio::sync::oneshot::channel().0, db);

        // Test valid server TLS configuration
        let valid_request = tonic::Request::new(RegisterBlueprintServiceProxyRequest {
            service_id: 1,
            api_key_prefix: "test".to_string(),
            upstream_url: "http://localhost:8080".to_string(),
            owners: vec![],
            tls_profile: Some(TlsProfileConfig {
                tls_enabled: true,
                require_client_mtls: false,
                encrypted_server_cert: b"cert".to_vec(),
                encrypted_server_key: b"key".to_vec(),
                encrypted_client_ca_bundle: b"ca".to_vec(),
                encrypted_upstream_ca_bundle: Vec::new(),
                encrypted_upstream_client_cert: Vec::new(),
                encrypted_upstream_client_key: Vec::new(),
                client_cert_ttl_hours: 24,
                sni: None,
                subject_alt_name_template: None,
                allowed_dns_names: Vec::new(),
            }),
        });

        let result = service
            .register_blueprint_service_proxy(valid_request)
            .await;
        assert!(result.is_ok());

        // Test that client CA bundle is optional when client mTLS is disabled
        let optional_ca_request = tonic::Request::new(RegisterBlueprintServiceProxyRequest {
            service_id: 1,
            api_key_prefix: "test".to_string(),
            upstream_url: "http://localhost:8080".to_string(),
            owners: vec![],
            tls_profile: Some(TlsProfileConfig {
                tls_enabled: true,
                require_client_mtls: false,
                encrypted_server_cert: b"cert".to_vec(),
                encrypted_server_key: b"key".to_vec(),
                encrypted_client_ca_bundle: Vec::new(),
                encrypted_upstream_ca_bundle: Vec::new(),
                encrypted_upstream_client_cert: Vec::new(),
                encrypted_upstream_client_key: Vec::new(),
                client_cert_ttl_hours: 24,
                sni: None,
                subject_alt_name_template: None,
                allowed_dns_names: Vec::new(),
            }),
        });

        let result = service
            .register_blueprint_service_proxy(optional_ca_request)
            .await;
        assert!(result.is_ok());

        // Test invalid server TLS configuration (missing cert)
        let invalid_request = tonic::Request::new(RegisterBlueprintServiceProxyRequest {
            service_id: 1,
            api_key_prefix: "test".to_string(),
            upstream_url: "http://localhost:8080".to_string(),
            owners: vec![],
            tls_profile: Some(TlsProfileConfig {
                tls_enabled: true,
                require_client_mtls: false,
                encrypted_server_cert: Vec::new(), // Missing
                encrypted_server_key: b"key".to_vec(),
                encrypted_client_ca_bundle: b"ca".to_vec(),
                encrypted_upstream_ca_bundle: Vec::new(),
                encrypted_upstream_client_cert: Vec::new(),
                encrypted_upstream_client_key: Vec::new(),
                client_cert_ttl_hours: 24,
                sni: None,
                subject_alt_name_template: None,
                allowed_dns_names: Vec::new(),
            }),
        });

        let result = service
            .register_blueprint_service_proxy(invalid_request)
            .await;
        assert!(result.is_err());
        assert_eq!(
            result.as_ref().unwrap_err().code(),
            tonic::Code::InvalidArgument
        );
        assert_eq!(
            result.as_ref().unwrap_err().message(),
            "Server certificate and key are required when TLS is enabled"
        );

        // Test invalid server TLS configuration (missing key)
        let invalid_request = tonic::Request::new(RegisterBlueprintServiceProxyRequest {
            service_id: 1,
            api_key_prefix: "test".to_string(),
            upstream_url: "http://localhost:8080".to_string(),
            owners: vec![],
            tls_profile: Some(TlsProfileConfig {
                tls_enabled: true,
                require_client_mtls: false,
                encrypted_server_cert: b"cert".to_vec(),
                encrypted_server_key: Vec::new(), // Missing
                encrypted_client_ca_bundle: b"ca".to_vec(),
                encrypted_upstream_ca_bundle: Vec::new(),
                encrypted_upstream_client_cert: Vec::new(),
                encrypted_upstream_client_key: Vec::new(),
                client_cert_ttl_hours: 24,
                sni: None,
                subject_alt_name_template: None,
                allowed_dns_names: Vec::new(),
            }),
        });

        let result = service
            .register_blueprint_service_proxy(invalid_request)
            .await;
        assert!(result.is_err());
        assert_eq!(
            result.as_ref().unwrap_err().code(),
            tonic::Code::InvalidArgument
        );
        assert_eq!(
            result.as_ref().unwrap_err().message(),
            "Server certificate and key are required when TLS is enabled"
        );
    }

    #[tokio::test]
    async fn test_tls_profile_validation_client_mtls_required() {
        let tmp_dir = tempdir().unwrap();
        let db = RocksDb::open(tmp_dir.path(), &RocksDbConfig::default()).unwrap();
        let service = BridgeService::new(tokio::sync::oneshot::channel().0, db);

        // Test valid client mTLS configuration
        let valid_request = tonic::Request::new(RegisterBlueprintServiceProxyRequest {
            service_id: 1,
            api_key_prefix: "test".to_string(),
            upstream_url: "http://localhost:8080".to_string(),
            owners: vec![],
            tls_profile: Some(TlsProfileConfig {
                tls_enabled: true,
                require_client_mtls: true,
                encrypted_server_cert: b"cert".to_vec(),
                encrypted_server_key: b"key".to_vec(),
                encrypted_client_ca_bundle: b"client_ca".to_vec(),
                encrypted_upstream_ca_bundle: Vec::new(),
                encrypted_upstream_client_cert: Vec::new(),
                encrypted_upstream_client_key: Vec::new(),
                client_cert_ttl_hours: 24,
                sni: None,
                subject_alt_name_template: None,
                allowed_dns_names: Vec::new(),
            }),
        });

        let result = service
            .register_blueprint_service_proxy(valid_request)
            .await;
        assert!(result.is_ok());

        // Test invalid client mTLS configuration (missing client CA)
        let invalid_request = tonic::Request::new(RegisterBlueprintServiceProxyRequest {
            service_id: 1,
            api_key_prefix: "test".to_string(),
            upstream_url: "http://localhost:8080".to_string(),
            owners: vec![],
            tls_profile: Some(TlsProfileConfig {
                tls_enabled: true,
                require_client_mtls: true,
                encrypted_server_cert: b"cert".to_vec(),
                encrypted_server_key: b"key".to_vec(),
                encrypted_client_ca_bundle: Vec::new(), // Missing
                encrypted_upstream_ca_bundle: Vec::new(),
                encrypted_upstream_client_cert: Vec::new(),
                encrypted_upstream_client_key: Vec::new(),
                client_cert_ttl_hours: 24,
                sni: None,
                subject_alt_name_template: None,
                allowed_dns_names: Vec::new(),
            }),
        });

        let result = service
            .register_blueprint_service_proxy(invalid_request)
            .await;
        assert!(result.is_err());
        assert_eq!(
            result.as_ref().unwrap_err().code(),
            tonic::Code::InvalidArgument
        );
        assert_eq!(
            result.as_ref().unwrap_err().message(),
            "Client CA bundle is required when mutual TLS authentication is enabled"
        );
    }

    #[tokio::test]
    async fn test_tls_profile_update_validation() {
        let tmp_dir = tempdir().unwrap();
        let db = RocksDb::open(tmp_dir.path(), &RocksDbConfig::default()).unwrap();
        let service = BridgeService::new(tokio::sync::oneshot::channel().0, db.clone());

        // First register a service without TLS
        let register_request = tonic::Request::new(RegisterBlueprintServiceProxyRequest {
            service_id: 1,
            api_key_prefix: "test".to_string(),
            upstream_url: "http://localhost:8080".to_string(),
            owners: vec![],
            tls_profile: None,
        });

        let result = service
            .register_blueprint_service_proxy(register_request)
            .await;
        assert!(result.is_ok());

        // Test updating to valid TLS configuration without client CA when mTLS is disabled
        let valid_update_request = tonic::Request::new(UpdateBlueprintServiceTlsProfileRequest {
            service_id: 1,
            tls_profile: Some(TlsProfileConfig {
                tls_enabled: true,
                require_client_mtls: false,
                encrypted_server_cert: b"cert".to_vec(),
                encrypted_server_key: b"key".to_vec(),
                encrypted_client_ca_bundle: Vec::new(),
                encrypted_upstream_ca_bundle: Vec::new(),
                encrypted_upstream_client_cert: Vec::new(),
                encrypted_upstream_client_key: Vec::new(),
                client_cert_ttl_hours: 24,
                sni: Some("example.com".to_string()),
                subject_alt_name_template: None,
                allowed_dns_names: Vec::new(),
            }),
        });

        let result = service
            .update_blueprint_service_tls_profile(valid_update_request)
            .await;
        assert!(result.is_ok());

        // Test updating to invalid TLS configuration
        let invalid_update_request = tonic::Request::new(UpdateBlueprintServiceTlsProfileRequest {
            service_id: 1,
            tls_profile: Some(TlsProfileConfig {
                tls_enabled: true,
                require_client_mtls: true,
                encrypted_server_cert: b"cert".to_vec(),
                encrypted_server_key: b"key".to_vec(),
                encrypted_client_ca_bundle: Vec::new(), // Missing for client mTLS
                encrypted_upstream_ca_bundle: Vec::new(),
                encrypted_upstream_client_cert: Vec::new(),
                encrypted_upstream_client_key: Vec::new(),
                client_cert_ttl_hours: 24,
                sni: None,
                subject_alt_name_template: None,
                allowed_dns_names: Vec::new(),
            }),
        });

        let result = service
            .update_blueprint_service_tls_profile(invalid_update_request)
            .await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), tonic::Code::InvalidArgument);
    }

    #[tokio::test]
    async fn test_tls_profile_disable_tls() {
        let tmp_dir = tempdir().unwrap();
        let db = RocksDb::open(tmp_dir.path(), &RocksDbConfig::default()).unwrap();
        let service = BridgeService::new(tokio::sync::oneshot::channel().0, db.clone());

        // First register a service with TLS
        let register_request = tonic::Request::new(RegisterBlueprintServiceProxyRequest {
            service_id: 1,
            api_key_prefix: "test".to_string(),
            upstream_url: "http://localhost:8080".to_string(),
            owners: vec![],
            tls_profile: Some(TlsProfileConfig {
                tls_enabled: true,
                require_client_mtls: false,
                encrypted_server_cert: b"cert".to_vec(),
                encrypted_server_key: b"key".to_vec(),
                encrypted_client_ca_bundle: b"ca".to_vec(),
                encrypted_upstream_ca_bundle: Vec::new(),
                encrypted_upstream_client_cert: Vec::new(),
                encrypted_upstream_client_key: Vec::new(),
                client_cert_ttl_hours: 24,
                sni: None,
                subject_alt_name_template: None,
                allowed_dns_names: Vec::new(),
            }),
        });

        let result = service
            .register_blueprint_service_proxy(register_request)
            .await;
        assert!(result.is_ok());

        // Test disabling TLS by setting TLS profile to None
        let disable_request = tonic::Request::new(UpdateBlueprintServiceTlsProfileRequest {
            service_id: 1,
            tls_profile: None,
        });

        let result = service
            .update_blueprint_service_tls_profile(disable_request)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_service_id_pinning() {
        let tmp_dir = tempdir().unwrap();
        let db = RocksDb::open(tmp_dir.path(), &RocksDbConfig::default()).unwrap();
        let service = BridgeService::new(tokio::sync::oneshot::channel().0, db);

        // Register service_id=1
        let request = tonic::Request::new(RegisterBlueprintServiceProxyRequest {
            service_id: 1,
            api_key_prefix: "test".to_string(),
            upstream_url: "http://localhost:8080".to_string(),
            owners: vec![],
            tls_profile: None,
        });
        assert!(
            service
                .register_blueprint_service_proxy(request)
                .await
                .is_ok()
        );

        // Try to register service_id=2 on the same bridge - should be rejected (pinned to 1)
        let request = tonic::Request::new(RegisterBlueprintServiceProxyRequest {
            service_id: 2,
            api_key_prefix: "test".to_string(),
            upstream_url: "http://localhost:9090".to_string(),
            owners: vec![],
            tls_profile: None,
        });
        let result = service.register_blueprint_service_proxy(request).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), tonic::Code::PermissionDenied);
    }

    #[tokio::test]
    async fn test_cross_bridge_hijack_protection() {
        let tmp_dir = tempdir().unwrap();
        let db = RocksDb::open(tmp_dir.path(), &RocksDbConfig::default()).unwrap();

        // Bridge A registers service_id=1
        let bridge_a = BridgeService::new(tokio::sync::oneshot::channel().0, db.clone());
        let request = tonic::Request::new(RegisterBlueprintServiceProxyRequest {
            service_id: 1,
            api_key_prefix: "test".to_string(),
            upstream_url: "http://localhost:8080".to_string(),
            owners: vec![],
            tls_profile: None,
        });
        assert!(
            bridge_a
                .register_blueprint_service_proxy(request)
                .await
                .is_ok()
        );

        // Bridge B (different instance, same DB) tries to register service_id=1 - should be rejected
        let bridge_b = BridgeService::new(tokio::sync::oneshot::channel().0, db);
        let request = tonic::Request::new(RegisterBlueprintServiceProxyRequest {
            service_id: 1,
            api_key_prefix: "evil".to_string(),
            upstream_url: "http://attacker:9090".to_string(),
            owners: vec![],
            tls_profile: None,
        });
        let result = bridge_b.register_blueprint_service_proxy(request).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), tonic::Code::AlreadyExists);
    }

    #[tokio::test]
    async fn test_unregister_requires_registration() {
        let tmp_dir = tempdir().unwrap();
        let db = RocksDb::open(tmp_dir.path(), &RocksDbConfig::default()).unwrap();
        let service = BridgeService::new(tokio::sync::oneshot::channel().0, db);

        // Try to unregister without having registered - should fail
        let request = tonic::Request::new(UnregisterBlueprintServiceProxyRequest { service_id: 1 });
        let result = service.unregister_blueprint_service_proxy(request).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), tonic::Code::FailedPrecondition);
    }
}
