use blueprint_manager_bridge::blueprint_manager_bridge_server::{
    BlueprintManagerBridge, BlueprintManagerBridgeServer,
};
use blueprint_manager_bridge::{Error, PortRequest, PortResponse};
use blueprint_manager_bridge::VSOCK_PORT;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::UnixListener;
use tokio::sync::{oneshot, Mutex};
use tokio::task::JoinHandle;
use tokio_stream::wrappers::UnixListenerStream;
use tonic::{Request, Response, transport::Server};
use tracing::{error, info};

pub struct BridgeHandle {
    sock_path: PathBuf,
    handle: JoinHandle<Result<(), Error>>,
}

impl BridgeHandle {
    pub fn shutdown(self) {
        let _ = std::fs::remove_file(self.sock_path);
        self.handle.abort();
    }
}

/// Manager <-> Service bridge
pub struct Bridge {
    runtime_dir: PathBuf,
    service_name: String,
}

impl Bridge {
    pub fn new(runtime_dir: PathBuf, service_name: String) -> Self {
        Self {
            runtime_dir,
            service_name,
        }
    }

    pub fn base_socket_path(&self) -> PathBuf {
        let sock_name = format!("{}.sock", self.service_name);
        self.runtime_dir.join(sock_name)
    }

    fn guest_socket_path(&self) -> PathBuf {
        let sock_name = format!("{}.sock_{VSOCK_PORT}", self.service_name);
        self.runtime_dir.join(sock_name)
    }
}

impl Bridge {
    pub fn spawn(self) -> Result<(BridgeHandle, oneshot::Receiver<()>), Error> {
        let sock_path = self.guest_socket_path();
        let _ = std::fs::remove_file(&sock_path);
        let listener = UnixListener::bind(&sock_path).map_err(|e| {
            error!(
                "Failed to bind bridge socket at {}: {e}",
                sock_path.display()
            );
            e
        })?;

        info!("Connected to bridge for service `{}`, listening on VSOCK port {VSOCK_PORT}", self.service_name);

        let (tx, rx) = oneshot::channel();

        let handle = tokio::task::spawn(async move {
            Server::builder()
                .add_service(BlueprintManagerBridgeServer::new(BridgeService::new(tx)))
                .serve_with_incoming(UnixListenerStream::new(listener))
                .await
                .map_err(Error::from)
        });

        Ok((BridgeHandle { sock_path, handle }, rx))
    }
}

struct BridgeService {
    ready_tx: Arc<Mutex<Option<oneshot::Sender<()>>>>,
}

impl BridgeService {
    fn new(tx: oneshot::Sender<()>) -> Self {
        Self { ready_tx: Arc::new(Mutex::new(Some(tx))) }
    }

    async fn signal_ready(&self) {
        if let Some(tx) = self.ready_tx.lock().await.take() {
            let _ = tx.send(());
        }
    }
}

#[tonic::async_trait]
impl BlueprintManagerBridge for BridgeService {
    async fn ping(
        &self,
        _req: Request<()>,
    ) -> Result<Response<()>, tonic::Status> {
        self.signal_ready().await;
        Ok(Response::new(()))
    }

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
