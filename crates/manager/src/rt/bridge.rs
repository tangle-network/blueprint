use blueprint_manager_bridge::blueprint_manager_bridge_server::{
    BlueprintManagerBridge, BlueprintManagerBridgeServer,
};
use blueprint_manager_bridge::{Error, PortRequest, PortResponse};
use futures::Stream;
use std::path::PathBuf;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio::task::JoinHandle;
use tokio_vsock::{VsockAddr, VsockStream};
use tonic::transport::server::Connected;
use tonic::{Request, Response, transport::Server};
use tracing::{error, info};

struct VsockAdapter {
    stream: Option<VsockStream>,
}

impl VsockAdapter {
    const SERVICE_PORT: u32 = 8000;

    async fn bind(cid: u32) -> std::io::Result<Self> {
        let stream = VsockStream::connect(VsockAddr::new(cid, Self::SERVICE_PORT)).await?;
        Ok(VsockAdapter {
            stream: Some(stream),
        })
    }
}

impl Stream for VsockAdapter {
    type Item = std::io::Result<VsockStreamAdapter>;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Ready(
            self.stream
                .take()
                .map(|s| VsockStreamAdapter { stream: s })
                .map(Ok),
        )
    }
}

pin_project_lite::pin_project! {
    struct VsockStreamAdapter {
        #[pin]
        stream: VsockStream
    }
}

impl Connected for VsockStreamAdapter {
    type ConnectInfo = ();

    fn connect_info(&self) -> Self::ConnectInfo {
        ()
    }
}

impl AsyncRead for VsockStreamAdapter {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let s = self.project();
        s.stream.poll_read(cx, buf)
    }
}

impl AsyncWrite for VsockStreamAdapter {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        let s = self.project();
        s.stream.poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), std::io::Error>> {
        let s = self.project();
        s.stream.poll_flush(cx)
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        let s = self.project();
        s.stream.poll_shutdown(cx)
    }
}

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

    pub fn socket_path(&self) -> PathBuf {
        let sock_name = format!("{}.sock", self.service_name);
        self.runtime_dir.join(sock_name)
    }
}

impl Bridge {
    pub async fn spawn(self, cid: u32) -> Result<BridgeHandle, Error> {
        let sock_path = self.socket_path();

        let listener = VsockAdapter::bind(cid).await.map_err(|e| {
            error!(
                "Failed to connect to bridge socket at {} (cid={cid}): {e}",
                sock_path.display()
            );
            e
        })?;

        info!("Connected to bridge for service `{}`", self.service_name);

        let handle = tokio::task::spawn(async move {
            Server::builder()
                .add_service(BlueprintManagerBridgeServer::new(BridgeService))
                .serve_with_incoming(listener)
                .await
                .map_err(Error::from)
        });

        Ok(BridgeHandle { sock_path, handle })
    }
}

struct BridgeService;

#[tonic::async_trait]
impl BlueprintManagerBridge for BridgeService {
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
