pub mod error;
pub use error::Error;

mod api;
pub use api::*;

use crate::blueprint_manager_bridge_client::BlueprintManagerBridgeClient;
use std::sync::Arc;
use tokio::net::UnixStream;
use tonic::transport::Channel;

pub struct Bridge {
    client: BlueprintManagerBridgeClient<Channel>,
}

impl Bridge {
    pub async fn connect(socket: impl AsRef<std::path::Path>) -> Result<Self, Error> {
        let sock_path = Arc::new(socket.as_ref().to_owned());
        let channel = Channel::from_static("http://[::]:50051")
            .connect_with_connector(tower::service_fn(move |_| {
                let sock_path = sock_path.clone();
                async move { UnixStream::connect(&*sock_path).await }
            }))
            .await?;

        Ok(Self {
            client: BlueprintManagerBridgeClient::new(channel),
        })
    }

    #[allow(clippy::cast_possible_truncation)]
    pub async fn request_port(&mut self, preferred: Option<u16>) -> Result<u16, Error> {
        let reply = self
            .client
            .request_port(PortRequest {
                preferred_port: u32::from(preferred.unwrap_or(0)),
            })
            .await?
            .into_inner();

        Ok(reply.port as u16)
    }
}
