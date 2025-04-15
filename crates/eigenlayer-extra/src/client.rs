use crate::generic_task_aggregation::{SignedTaskResponse, TaskResponse};
use reqwest::{Client, Url};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::time::Duration;
use thiserror::Error;
use tokio::time::sleep;
use tracing::{debug, error, info};

/// Error type for the Aggregator Client
#[derive(Error, Debug)]
pub enum ClientError {
    /// URL parsing error
    #[error("URL parsing error: {0}")]
    UrlParseError(#[from] url::ParseError),

    /// JSON error
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// HTTP error
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    /// RPC error
    #[error("RPC error: {0}")]
    RpcError(String),

    /// Retry limit exceeded
    #[error("Retry limit exceeded after {0} attempts")]
    RetryLimitExceeded(u32),
}

/// Result type for the client
pub type Result<T> = std::result::Result<T, ClientError>;

/// Configuration for the Aggregator Client
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Maximum number of retry attempts for RPC requests
    pub max_retries: u32,

    /// Initial retry delay in seconds
    pub initial_retry_delay: Duration,

    /// Whether to use exponential backoff for retries
    pub use_exponential_backoff: bool,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            max_retries: 5,
            initial_retry_delay: Duration::from_secs(1),
            use_exponential_backoff: true,
        }
    }
}

/// Generic client for interacting with the Task Aggregator RPC server
#[derive(Debug, Clone)]
pub struct AggregatorClient<E, R, C>
where
    E: Serialize + Clone + Send + Sync + 'static,
    R: TaskResponse + DeserializeOwned + Debug + Send,
    C: Fn(E) -> SignedTaskResponse<R> + Clone + Send + Sync + 'static,
{
    /// The underlying HTTP client
    client: Client,

    /// The server URL
    server_url: Url,

    /// The conversion function to transform external types to generic task responses
    converter: C,

    /// Client configuration
    config: ClientConfig,

    /// Phantom data to indicate that E and R are used
    _phantom: PhantomData<(E, R)>,
}

impl<E, R, C> AggregatorClient<E, R, C>
where
    E: Serialize + Clone + Send + Sync + 'static,
    R: TaskResponse + DeserializeOwned + Debug + Send,
    C: Fn(E) -> SignedTaskResponse<R> + Clone + Send + Sync + 'static,
{
    /// Create a new Aggregator Client
    ///
    /// # Errors
    /// - `ClientError::UrlParse`: If the URL parsing fails
    pub fn new(aggregator_address: &str, converter: C) -> Result<Self> {
        let url = Url::parse(&format!("http://{}", aggregator_address))?;
        let client = Client::new();

        Ok(Self {
            client,
            server_url: url,
            converter,
            config: ClientConfig::default(),
            _phantom: PhantomData,
        })
    }

    /// Create a new Aggregator Client with custom configuration
    ///
    /// # Errors
    /// - `ClientError::UrlParse`: If the URL parsing fails
    pub fn with_config(
        aggregator_address: &str,
        converter: C,
        config: ClientConfig,
    ) -> Result<Self> {
        let url = Url::parse(&format!("http://{}", aggregator_address))?;
        let client = Client::new();

        Ok(Self {
            client,
            server_url: url,
            converter,
            config,
            _phantom: PhantomData,
        })
    }

    /// Send a JSON-RPC request to the server
    async fn json_rpc_request<T, U>(&self, method: &str, params: T) -> Result<U>
    where
        T: Serialize,
        U: DeserializeOwned,
    {
        let request = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": 1
        });

        let response = self
            .client
            .post(self.server_url.clone())
            .json(&request)
            .send()
            .await?
            .json::<Value>()
            .await?;

        // Check for JSON-RPC error response
        if let Some(error) = response.get("error") {
            return Err(ClientError::RpcError(error.to_string()));
        }

        // Extract the result
        let result = response
            .get("result")
            .ok_or_else(|| ClientError::RpcError("Missing 'result' in response".to_string()))?;

        // Deserialize the result to the expected type
        let result = serde_json::from_value(result.clone())?;

        Ok(result)
    }

    /// Send a signed task response to the aggregator with retries
    ///
    /// # Errors
    /// - `ClientError::RpcError`: If the JSON-RPC request fails
    /// - `ClientError::RetryLimitExceeded`: If the maximum number of retries is exceeded
    pub async fn send_signed_task_response(&self, external_response: E) -> Result<()> {
        // Convert to the generic format for debugging/logs
        let generic_response = (self.converter)(external_response.clone());

        for attempt in 1..=self.config.max_retries {
            match self
                .json_rpc_request::<E, bool>(
                    "process_signed_task_response",
                    external_response.clone(),
                )
                .await
            {
                Ok(true) => {
                    info!(
                        "Task response accepted by aggregator for task {}",
                        generic_response.response.reference_task_index()
                    );
                    return Ok(());
                }
                Ok(false) => {
                    debug!("Task response not accepted, retrying...");
                }
                Err(e) => {
                    error!("Error sending task response: {}", e);
                }
            }

            if attempt < self.config.max_retries {
                let delay = if self.config.use_exponential_backoff {
                    self.config.initial_retry_delay * 2u32.pow(attempt - 1)
                } else {
                    self.config.initial_retry_delay
                };

                info!("Retrying in {} seconds...", delay.as_secs());
                sleep(delay).await;
            }
        }

        debug!(
            "Failed to send signed task response after {} attempts",
            self.config.max_retries
        );

        Err(ClientError::RetryLimitExceeded(self.config.max_retries))
    }
}
