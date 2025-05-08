use blueprint_manager_bridge::blueprint_manager_bridge_server::{
    BlueprintManagerBridge, BlueprintManagerBridgeServer,
};
use blueprint_manager_bridge::{Error, PortRequest, PortResponse};
use std::path::{Path, PathBuf};
use tokio::net::UnixListener;
use tokio::task::JoinHandle;
use tokio_stream::wrappers::UnixListenerStream;
use tonic::{Request, Response, transport::Server};

pub struct BridgeHandle {
    sock_path: PathBuf,
    handle: JoinHandle<Result<(), Error>>,
}

/// Manager <-> Service bridge
pub struct Bridge(());

impl Bridge {
    pub fn spawn(runtime_dir: &Path, service_name: &str) -> Result<BridgeHandle, Error> {
        let sock_name = format!("{service_name}.sock");
        let sock_path = runtime_dir.join(&sock_name);
        let _ = std::fs::remove_file(&sock_path);
        let listener = UnixListener::bind(&sock_path)?;

        let handle = tokio::task::spawn(async move {
            Server::builder()
                .add_service(BlueprintManagerBridgeServer::new(Bridge(())))
                .serve_with_incoming(UnixListenerStream::new(listener))
                .await
                .map_err(Error::from)
        });

        Ok(BridgeHandle { sock_path, handle })
    }
}

#[tonic::async_trait]
impl BlueprintManagerBridge for Bridge {
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
}

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
