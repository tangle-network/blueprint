pub mod error;
pub use error::Error;

mod api;
pub use api::*;

use crate::blueprint_manager_bridge_client::BlueprintManagerBridgeClient;
use blueprint_auth::models::ServiceOwnerModel;
use blueprint_core::debug;
use hyper_util::rt::TokioIo;
use std::path::Path;
use tokio::net::UnixStream;
use tokio_vsock::{VsockAddr, VsockStream};
use tonic::transport::Channel;

pub const VSOCK_PORT: u32 = 8000;

#[derive(Debug)]
pub struct Bridge {
    client: BlueprintManagerBridgeClient<Channel>,
}

impl Bridge {
    /// Connect to the blueprint manager bridge.
    ///
    /// # Errors
    /// - If the connection to the bridge fails.
    pub async fn connect(socket_path: Option<&Path>) -> Result<Self, Error> {
        let channel = match socket_path {
            Some(path) => {
                debug!("Connecting to UDS bridge at {}", path.display());

                let path = path.to_path_buf();
                Channel::from_static("http://[::]:50051")
                    .connect_with_connector(tower::service_fn(move |_| {
                        let path = path.clone();
                        async move {
                            Ok::<_, std::io::Error>(TokioIo::new(UnixStream::connect(&path).await?))
                        }
                    }))
                    .await?
            }
            None => {
                debug!("Connecting to VSOCK bridge at port {}", VSOCK_PORT);
                Channel::from_static("http://[::]:50051")
                    .connect_with_connector(tower::service_fn(|_| async {
                        Ok::<_, std::io::Error>(TokioIo::new(
                            VsockStream::connect(VsockAddr::new(
                                tokio_vsock::VMADDR_CID_HOST,
                                VSOCK_PORT,
                            ))
                            .await?,
                        ))
                    }))
                    .await?
            }
        };

        Ok(Self {
            client: BlueprintManagerBridgeClient::new(channel),
        })
    }

    /// Sends a Ping request to the blueprint manager bridge.
    ///
    /// This method is used to check the connectivity and responsiveness of the bridge.
    ///
    /// # Errors
    /// - Returns an error if the ping operation fails, for example, due to network issues or if the bridge service is not responding.
    pub async fn ping(&self) -> Result<(), Error> {
        self.client.clone().ping(()).await?;
        Ok(())
    }

    /// Requests a port from the blueprint manager bridge.
    ///
    /// The bridge will attempt to reserve the preferred port if provided,
    /// otherwise it will assign an available port.
    ///
    /// # Arguments
    /// * `preferred`: An optional `u16` specifying the preferred port number.
    ///
    /// # Returns
    /// A `Result` containing the `u16` port number assigned by the bridge,
    /// or an `Error` if the request fails.
    ///
    /// # Errors
    /// - Returns an error if the port request to the bridge fails, e.g., if the bridge cannot allocate a port or encounters an internal error.
    #[allow(clippy::cast_possible_truncation)]
    pub async fn request_port(&self, preferred: Option<u16>) -> Result<u16, Error> {
        let reply = self
            .client
            .clone()
            .request_port(PortRequest {
                preferred_port: u32::from(preferred.unwrap_or(0)),
            })
            .await?
            .into_inner();

        Ok(reply.port as u16)
    }

    /// Registers a blueprint service proxy with the blueprint manager bridge.
    ///
    /// This method allows a blueprint to make a service accessible via the proxy.
    ///
    /// # Arguments
    /// * `service_id`: A `u64` unique identifier for the service.
    /// * `api_key_prefix`: An optional string slice representing the prefix of the API key used for service authentication.
    /// * `upstream_url`: A string slice representing the URL of the upstream service.
    /// * `owners`: A slice of `ServiceOwnerModel` defining the owners authorized to use this service.
    ///
    /// # Errors
    /// - Returns an error if registering the service proxy fails, such as issues with the provided parameters or bridge internal errors.
    pub async fn register_blueprint_service_proxy(
        &self,
        service_id: u64,
        api_key_prefix: Option<&str>,
        upstream_url: &str,
        owners: &[ServiceOwnerModel],
    ) -> Result<(), Error> {
        let owners = owners
            .iter()
            .map(|owner| ServiceOwner {
                key_type: owner.key_type,
                key_bytes: owner.key_bytes.clone(),
            })
            .collect();

        let request = RegisterBlueprintServiceProxyRequest {
            service_id,
            api_key_prefix: api_key_prefix.unwrap_or_default().to_owned(),
            upstream_url: upstream_url.to_string(),
            owners,
        };

        self.client
            .clone()
            .register_blueprint_service_proxy(request)
            .await?;

        Ok(())
    }

    /// Unregisters a blueprint service proxy from the blueprint manager bridge.
    ///
    /// This method is called by a blueprint to remove a service from the proxy.
    ///
    /// # Arguments
    /// * `service_id`: A `u64` unique identifier for the service to be unregistered.
    ///
    /// # Errors
    /// - Returns an error if unregistering the service proxy fails, for instance, if the service ID is not found or the bridge encounters an issue.
    pub async fn unregister_blueprint_service_proxy(&self, service_id: u64) -> Result<(), Error> {
        let request = UnregisterBlueprintServiceProxyRequest { service_id };

        self.client
            .clone()
            .unregister_blueprint_service_proxy(request)
            .await?;

        Ok(())
    }

    /// Adds an owner to a registered blueprint service.
    ///
    /// This method allows a blueprint to grant an additional owner access to a service.
    ///
    /// # Arguments
    /// * `service_id`: A `u64` unique identifier for the service.
    /// * `owner_to_add`: A `ServiceOwnerModel` representing the owner to be added.
    ///
    /// # Errors
    /// - Returns an error if adding an owner to the service fails, e.g., if the service ID is invalid or the owner cannot be added.
    pub async fn add_owner_to_service(
        &self,
        service_id: u64,
        owner_to_add: ServiceOwnerModel,
    ) -> Result<(), Error> {
        let request = AddOwnerToServiceRequest {
            service_id,
            owner_to_add: Some(ServiceOwner {
                key_type: owner_to_add.key_type,
                key_bytes: owner_to_add.key_bytes,
            }),
        };

        self.client.clone().add_owner_to_service(request).await?;

        Ok(())
    }

    /// Removes an owner from a registered blueprint service.
    ///
    /// This method allows a blueprint to revoke an owner's access to a service.
    ///
    /// # Arguments
    /// * `service_id`: A `u64` unique identifier for the service.
    /// * `owner_to_remove`: A `ServiceOwnerModel` representing the owner to be removed.
    ///
    /// # Errors
    /// - Returns an error if removing an owner from the service fails, for example, if the service ID or owner is not found.
    pub async fn remove_owner_from_service(
        &self,
        service_id: u64,
        owner_to_remove: ServiceOwnerModel,
    ) -> Result<(), Error> {
        let request = RemoveOwnerFromServiceRequest {
            service_id,
            owner_to_remove: Some(ServiceOwner {
                key_type: owner_to_remove.key_type,
                key_bytes: owner_to_remove.key_bytes,
            }),
        };

        self.client
            .clone()
            .remove_owner_from_service(request)
            .await?;

        Ok(())
    }
}
