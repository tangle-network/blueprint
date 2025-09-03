use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Provider not found: {0}")]
    ProviderNotFound(String),
    
    #[error("Deployment failed: {0}")]
    DeploymentFailed(String),
    
    #[error("Instance not found: {0}")]
    InstanceNotFound(String),
    
    #[error("Network tunnel error: {0}")]
    TunnelError(String),
    
    #[error("Resource limit exceeded: {0}")]
    ResourceLimitExceeded(String),
    
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    
    #[cfg(feature = "kubernetes")]
    #[error("Kubernetes error: {0}")]
    Kube(#[from] kube::Error),
    
    #[cfg(feature = "docker")]
    #[error("Docker error: {0}")]
    Docker(#[from] bollard::errors::Error),
    
    #[cfg(feature = "ssh")]
    #[error("SSH error: {0}")]
    Ssh(#[from] openssh::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
    
    #[error("Auth error: {0}")]
    Auth(#[from] blueprint_auth::Error),
    
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, Error>;