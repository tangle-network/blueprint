//! Error types for the webhook gateway.

/// Errors that can occur in the webhook gateway.
#[derive(Debug, thiserror::Error)]
pub enum WebhookError {
    /// Configuration error.
    #[error("webhook config error: {0}")]
    Config(String),

    /// Authentication failed.
    #[error("webhook auth failed: {0}")]
    AuthFailed(String),

    /// Server failed to start or encountered a runtime error.
    #[error("server error: {0}")]
    Server(String),

    /// Producer channel closed.
    #[error("producer channel closed")]
    ProducerChannelClosed,

    /// I/O error.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// TOML parsing error.
    #[error(transparent)]
    TomlParse(#[from] toml::de::Error),
}

impl From<WebhookError> for blueprint_runner::error::RunnerError {
    fn from(err: WebhookError) -> Self {
        blueprint_runner::error::RunnerError::Other(Box::new(err))
    }
}
