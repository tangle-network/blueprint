use thiserror::Error;

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
}

pub type Result<T> = std::result::Result<T, Error>;
