use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    
    #[error("Cluster not found: {0}")]
    ClusterNotFound(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[cfg(feature = "kubernetes")]
    #[error("Kubernetes error: {0}")]
    Kube(#[from] kube::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, Error>;