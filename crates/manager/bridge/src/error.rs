#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Transport(#[from] tonic::transport::Error),

    #[error(transparent)]
    Status(#[from] tonic::Status),
}
