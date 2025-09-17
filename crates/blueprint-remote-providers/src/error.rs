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

    #[cfg(feature = "aws")]
    #[error("AWS EC2 error: {0}")]
    AwsEc2(#[from] aws_sdk_ec2::Error),

    #[cfg(feature = "aws-eks")]
    #[error("AWS EKS error: {0}")]
    AwsEks(#[from] aws_sdk_eks::Error),

    #[error("IO error: {0}")]
    Io(#[from] blueprint_std::io::Error),

    #[error("Provider {0:?} not configured")]
    ProviderNotConfigured(crate::remote::CloudProvider),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("HTTP error: {0}")]
    HttpError(String),

    #[error("Other error: {0}")]
    Other(String),
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::SerializationError(err.to_string())
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Self {
        Error::SerializationError(err.to_string())
    }
}

#[cfg(feature = "aws")]
impl<E> From<aws_sdk_ec2::error::SdkError<E>> for Error
where
    E: blueprint_std::error::Error + Send + Sync + 'static,
{
    fn from(err: aws_sdk_ec2::error::SdkError<E>) -> Self {
        Error::Other(err.to_string())
    }
}

#[cfg(feature = "kubernetes")]
impl From<kube::config::InferConfigError> for Error {
    fn from(err: kube::config::InferConfigError) -> Self {
        Error::Other(err.to_string())
    }
}

#[cfg(feature = "kubernetes")]
impl From<kube::config::KubeconfigError> for Error {
    fn from(err: kube::config::KubeconfigError) -> Self {
        Error::Other(err.to_string())
    }
}

pub type Result<T> = blueprint_std::result::Result<T, Error>;
