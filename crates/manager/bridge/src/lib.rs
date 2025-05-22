pub mod error;
pub use error::Error;

mod api;
pub use api::*;

use crate::blueprint_manager_bridge_client::BlueprintManagerBridgeClient;
use tokio_vsock::{VsockAddr, VsockStream};
use tonic::transport::Channel;

pub const VSOCK_PORT: u32 = 8000;

#[derive(Debug)]
pub struct Bridge {
    client: BlueprintManagerBridgeClient<Channel>,
}

impl Bridge {
    pub async fn connect() -> Result<Self, Error> {
        let channel = Channel::from_static("http://[::]:50051")
            .connect_with_connector(tower::service_fn(move |_| {
                VsockStream::connect(VsockAddr::new(tokio_vsock::VMADDR_CID_HOST, VSOCK_PORT))
            }))
            .await?;

        Ok(Self {
            client: BlueprintManagerBridgeClient::new(channel),
        })
    }

    pub async fn ping(&self) -> Result<(), Error> {
        let reply = self.client.clone().ping(()).await?.into_inner();
        Ok(reply)
    }

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
}
