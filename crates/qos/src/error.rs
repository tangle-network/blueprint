use blueprint_core::error as core_error;
use thiserror::Error;
use tokio::sync::oneshot;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Heartbeat error: {0}")]
    Heartbeat(String),

    #[error("Metrics error: {0}")]
    Metrics(String),

    #[error("gRPC error: {0}")]
    Grpc(#[from] tonic::transport::Error),

    #[error("System metrics error: {0}")]
    SystemMetrics(String),

    #[error("Consumer error: {0}")]
    Consumer(String),

    #[error("{0}")]
    Other(String),

    #[error("JSON serialization/deserialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Grafana API error: {0}")]
    GrafanaApi(String),

    #[error("Docker connection error: {0}")]
    DockerConnection(String),

    #[error("Docker operation error: {0}")]
    DockerOperation(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Generic(String),

    #[error("Core error: {0}")]
    Core(#[from] core_error::Error),

    #[error("Oneshot receive error: {0}")]
    Recv(#[from] oneshot::error::RecvError),
}

pub type Result<T> = std::result::Result<T, Error>;
