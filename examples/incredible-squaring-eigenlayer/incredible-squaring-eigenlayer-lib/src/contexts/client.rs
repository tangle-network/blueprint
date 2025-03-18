use super::task::IncredibleSquaringTaskResponse;
use blueprint_eigenlayer_extra::client::{AggregatorClient, ClientConfig, ClientError};
use blueprint_eigenlayer_extra::generic_task_aggregation::SignedTaskResponse as GenericSignedTaskResponse;
use blueprint_sdk::eigensdk::crypto_bls::{OperatorId, Signature};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Default retry configuration
const DEFAULT_MAX_RETRIES: u32 = 5;
const DEFAULT_INITIAL_RETRY_DELAY: Duration = Duration::from_secs(1);

/// Signed task response sent by operators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedTaskResponse {
    pub task_response: IncredibleSquaringTaskResponse,
    pub signature: Signature,
    pub operator_id: OperatorId,
}

/// Type alias for the specialized AggregatorClient
pub type IncredibleSquaringAggregatorClient = AggregatorClient<
    SignedTaskResponse,
    IncredibleSquaringTaskResponse,
    fn(SignedTaskResponse) -> GenericSignedTaskResponse<IncredibleSquaringTaskResponse>,
>;

/// Creates a new AggregatorClient for the Incredible Squaring service
pub fn create_client(
    aggregator_address: &str,
) -> Result<IncredibleSquaringAggregatorClient, ClientError> {
    // Create a client with default configuration
    AggregatorClient::new(aggregator_address, |response: SignedTaskResponse| {
        GenericSignedTaskResponse::new(
            response.task_response.clone(),
            response.signature.clone(),
            response.operator_id,
        )
    })
}

/// Creates a new AggregatorClient with custom configuration
pub fn create_client_with_config(
    aggregator_address: &str,
    max_retries: u32,
    initial_retry_delay: Duration,
) -> Result<IncredibleSquaringAggregatorClient, ClientError> {
    let config = ClientConfig {
        max_retries,
        initial_retry_delay,
        use_exponential_backoff: true,
    };

    AggregatorClient::with_config(
        aggregator_address,
        |response: SignedTaskResponse| {
            GenericSignedTaskResponse::new(
                response.task_response.clone(),
                response.signature.clone(),
                response.operator_id,
            )
        },
        config,
    )
}
