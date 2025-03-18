//! Incredible Squaring TaskManager Monitor
//!
//! Monitors TaskManager events for task creation and completion.

use ::std::sync::Arc;

use crate::config::Keystore;
use crate::contracts::TaskManager::TaskResponse;
use blueprint_eigenlayer_extra::util::{operator_id_from_ark_bls_bn254, operator_id_from_key};
use blueprint_sdk::alloy::hex;
use blueprint_sdk::alloy::primitives::Address;
use blueprint_sdk::alloy::primitives::{keccak256, U256};
use blueprint_sdk::alloy::providers::RootProvider;
use blueprint_sdk::alloy::sol_types::{SolEvent, SolType, SolValue};
use blueprint_sdk::crypto::bn254::ArkBlsBn254;
use blueprint_sdk::eigensdk::crypto_bls::{self, BlsKeyPair};
use blueprint_sdk::eigensdk::services_blsaggregation::bls_agg;
use blueprint_sdk::eigensdk::services_blsaggregation::bls_agg::{TaskMetadata, TaskSignature};
use blueprint_sdk::eigensdk::services_blsaggregation::bls_aggregation_service_error::BlsAggregationServiceError;
use blueprint_sdk::evm::extract::{BlockNumber, ContractAddress, Events, FirstEvent, Tx};
use blueprint_sdk::evm::filters::{contract::MatchesContract, event::MatchesEvent};
use blueprint_sdk::extract::Context;
use blueprint_sdk::job_result::Void;
use blueprint_sdk::keystore::backends::bn254::Bn254Backend;
use blueprint_sdk::keystore::backends::Backend;
use blueprint_sdk::macros::context::{EigenlayerContext, KeystoreContext};
use blueprint_sdk::runner::config::BlueprintEnvironment;
use blueprint_sdk::*;
use contexts::client::{IncredibleSquaringAggregatorClient, SignedTaskResponse};
use contexts::task::IncredibleSquaringTaskResponse;
use contracts::SquaringTask::{self, NewTaskCreated};
use contracts::TaskManager::TaskResponse;
use tokio::sync::Mutex;
use tower::filter::FilterLayer;

#[derive(Debug, Clone, thiserror::Error)]
pub enum TaskError {
    #[error(transparent)]
    SolType(#[from] blueprint_sdk::alloy::sol_types::Error),
    #[error(transparent)]
    BlsAggregationService(#[from] BlsAggregationServiceError),
    #[error("Aggregated response receiver closed")]
    AggregatedResponseReceiverClosed,
    #[error("Task aggregator not initialized")]
    TaskAggregatorNotInitialized,
    #[error("Keystore error: {0}")]
    KeystoreError(#[from] gadget_keystore::Error),
    #[error("BLS error: {0}")]
    BlsError(#[from] crypto_bls::error::BlsError),
}

/// Service context shared between jobs
#[derive(Clone, Debug, KeystoreContext, EigenlayerContext)]
pub struct IncredibleSquaringClientContext {
    #[config]
    env: BlueprintEnvironment,
    client: Arc<IncredibleSquaringAggregatorClient>,
}

/// Sends a signed task response to the BLS Aggregator.
///
/// This job is triggered by the `NewTaskCreated` event emitted by the `IncredibleSquaringTaskManager`.
/// The job calculates the square of the number to be squared and sends the signed task response to the BLS Aggregator.
/// The job returns 1 if the task response was sent successfully.
/// The job returns 0 if the task response failed to send or failed to get the BLS key.
#[blueprint_sdk::macros::debug_job]
pub async fn compute_x_square(
    Context(ctx): Context<IncredibleSquaringClientContext>,
    BlockNumber(_block_num): BlockNumber,
    ContractAddress(_addr): ContractAddress,
    Events(ev): Events<NewTaskCreated>,
) -> Result<Void, TaskError> {
    // Extract details - note: these would come from the actual event
    // For now, let's use placeholder values
    let task_index = 0u32; // This would be extracted from the event
    let number_to_be_squared = U256::from(42); // This would be extracted from the event

    let task_response = TaskResponse {
        referenceTaskIndex: task_index,
        message: number_to_be_squared
            .saturating_pow(U256::from(2u32))
            .abi_encode()
            .into(),
    };

    info!("Task response prepared: {:#?}", task_response);

    // Get BLS keystore
    let keystore = ctx.env.keystore();

    // Sign the task response
    let msg_hash = keccak256(<TaskResponse as SolType>::abi_encode(&task_response));
    let public_key = keystore.first_local::<ArkBlsBn254>()?;
    let secret_key = keystore
        .expose_bls_bn254_secret(&public_key)?
        .ok_or_else(|| TaskError::KeystoreError("No BLS secret key found".into()))?;
    let signature = keystore.sign_with_local::<ArkBlsBn254>(&public_key, &msg_hash.to_vec())?;
    let operator_id = operator_id_from_ark_bls_bn254(secret_key)?;

    info!(
        "Signed task response with operator_id: {}",
        hex::encode(operator_id)
    );

    let signed_response = SignedTaskResponse {
        task_response: IncredibleSquaringTaskResponse {
            contract_response: task_response,
        },
        signature: crypto_bls::Signature::new(signature.0),
        operator_id,
    };

    if let Err(e) = ctx.client.send_signed_task_response(signed_response).await {
        error!("Failed to send task response: {}", e);
    } else {
        info!("Successfully sent task response for task {}", task_index);
    }

    Ok(Void)
}
