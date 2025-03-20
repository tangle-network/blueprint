//! Incredible Squaring TaskManager Monitor
//!
//! Monitors TaskManager events for task creation and completion.

use ::std::sync::Arc;

use crate::contexts::client::{IncredibleSquaringAggregatorClient, SignedTaskResponse};
use crate::contexts::task::IncredibleSquaringTaskResponse;
use crate::contracts::task_manager::SquaringTask::NewTaskCreated;
use crate::contracts::task_manager::TaskManager::TaskResponse;
use blueprint_eigenlayer_extra::util::operator_id_from_ark_bls_bn254;
use blueprint_sdk::crypto::bn254::ArkBlsBn254;
use blueprint_sdk::evm::extract::{BlockNumber, ContractAddress, Events};
use blueprint_sdk::extract::Context;
use blueprint_sdk::job_result::Void;
use blueprint_sdk::keystore::backends::{bn254::Bn254Backend, Backend};
use blueprint_sdk::macros::context::{EigenlayerContext, KeystoreContext};
use blueprint_sdk::runner::config::BlueprintEnvironment;
use blueprint_sdk::{
    alloy::{
        hex,
        primitives::{keccak256, U256},
        sol_types::{SolType, SolValue},
    },
    core::{error, info},
    eigensdk::crypto_bls,
};

use super::TaskError;

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
    Events(evs): Events<NewTaskCreated>,
) -> Result<Void, TaskError> {
    for ev in evs {
        // Extract details from the event
        let task_index = ev.taskIndex;
        let number_to_be_squared =
            U256::from_be_bytes::<32>(ev.task.message.0.to_vec().try_into().unwrap());

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
        let secret_key = keystore.expose_bls_bn254_secret(&public_key)?;
        let signature = keystore.sign_with_local::<ArkBlsBn254>(&public_key, &msg_hash.to_vec())?;

        let operator_id = if let Some(secret_key) = secret_key {
            operator_id_from_ark_bls_bn254(&secret_key)?
        } else {
            error!("No secret key found");
            return Err(TaskError::KeystoreError(gadget_keystore::Error::Other(
                "No secret key found".to_string(),
            )));
        };

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
    }

    Ok(Void)
}
