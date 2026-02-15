//! Error types for the x402 payment gateway.

/// Errors that can occur in the x402 payment gateway.
#[derive(Debug, thiserror::Error)]
pub enum X402Error {
    /// Configuration error.
    #[error("x402 config error: {0}")]
    Config(String),

    /// Quote not found or expired.
    #[error("quote not found or expired: {0}")]
    QuoteNotFound(String),

    /// Payment verification failed.
    #[error("payment verification failed: {0}")]
    PaymentVerification(String),

    /// Price conversion error.
    #[error("price conversion error: {0}")]
    PriceConversion(String),

    /// The requested job is not available via x402.
    #[error("job not available: service_id={service_id} job_index={job_index}")]
    JobNotAvailable { service_id: u64, job_index: u32 },

    /// Server failed to start or encountered a runtime error.
    #[error("server error: {0}")]
    Server(String),

    /// Failed to inject job into the producer channel.
    #[error("producer channel closed")]
    ProducerChannelClosed,

    /// I/O error.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// TOML parsing error.
    #[error(transparent)]
    TomlParse(#[from] toml::de::Error),
}

impl From<X402Error> for blueprint_runner::error::RunnerError {
    fn from(err: X402Error) -> Self {
        blueprint_runner::error::RunnerError::Other(Box::new(err))
    }
}
