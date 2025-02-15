use thiserror::Error;

#[derive(Error, Debug)]
pub enum RunnerError {
    #[error("Protocol error: {0}")]
    InvalidProtocol(String),

    #[error("Keystore error: {0}")]
    Keystore(String),

    #[error("Signature error: {0}")]
    SignatureError(String),

    #[error("Transaction error: {0}")]
    TransactionError(String),

    #[error("Not an active operator")]
    NotActiveOperator,

    #[error("Receive error: {0}")]
    Recv(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Tangle error: {0}")]
    Tangle(String),

    #[error("Blueprint runner configured without a router")]
    NoRouter,

    #[error("A background service failed: {0}")]
    BackgroundService(String),

    #[error("A job call failed: {0}")]
    JobCall(String),

    #[error("Generic error: {0}")]
    Other(String),
}

// Convenience Result type
pub type Result<T> = std::result::Result<T, RunnerError>;
