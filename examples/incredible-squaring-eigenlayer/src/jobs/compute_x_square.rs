#![allow(dead_code)]
use crate::contexts::client::SignedTaskResponse;
use crate::contexts::combined::CombinedContext;
use crate::contracts::SquaringTask::NewTaskCreated;
use crate::contracts::TaskManager::TaskResponse;
use crate::error::TaskError;
use alloy_primitives::{U256, keccak256};
use alloy_sol_types::{SolEvent, SolType, SolValue};
use blueprint_sdk::contexts::keystore::KeystoreContext;
use blueprint_sdk::crypto::bn254::ArkBlsBn254;
use blueprint_sdk::evm::extract::BlockEvents;
use blueprint_sdk::extract::Context;
use blueprint_sdk::keystore::backends::Backend;
use blueprint_sdk::keystore::backends::bn254::Bn254Backend;
use blueprint_sdk::{error, info};
use eigensdk::crypto_bls::BlsKeyPair;
use eigensdk::types::operator::operator_id_from_g1_pub_key;

pub const XSQUARE_JOB_ID: u32 = 0;

/// Sends a signed task response to the BLS Aggregator.
///
/// This job is triggered by the `NewTaskCreated` event emitted by the `IncredibleSquaringTaskManager`.
/// The job calculates the square of the number to be squared and sends the signed task response to the BLS Aggregator.
#[blueprint_sdk::macros::debug_job]
pub async fn xsquare_eigen(
    Context(ctx): Context<CombinedContext>,
    BlockEvents(events): BlockEvents,
) -> Result<(), TaskError> {
    let client = ctx.eigen_context.client.clone();

    let task_created_events = events.iter().filter_map(|log| {
        NewTaskCreated::decode_log(&log.inner, true)
            .map(|event| event.data)
            .ok()
    });

    for task_created in task_created_events {
        let task = task_created.task;
        let task_index = task_created.taskIndex;

        let message_bytes = task.message;
        let number_to_be_squared = U256::from_be_slice(&message_bytes.0);
        info!("Number to be squared: {}", number_to_be_squared);

        // Calculate the square
        let squared_result = number_to_be_squared.saturating_pow(U256::from(2u32));
        info!("Squared result: {}", squared_result);

        // Properly encode the result as a uint256 instead of a string
        let message = SolValue::abi_encode(&squared_result);

        // Calculate our response to job
        let task_response = TaskResponse {
            referenceTaskIndex: task_index,
            message: message.into(),
        };

        let bn254_public = ctx.keystore().first_local::<ArkBlsBn254>().unwrap();
        let bn254_secret = match ctx.keystore().expose_bls_bn254_secret(&bn254_public) {
            Ok(s) => match s {
                Some(s) => s,
                None => {
                    return Err(TaskError::Task(
                        "Failed to send signed task response".to_string(),
                    ));
                }
            },
            Err(e) => {
                return Err(TaskError::Task(format!(
                    "Failed to send signed task response: {:?}",
                    e
                )));
            }
        };
        let bls_key_pair = match BlsKeyPair::new(bn254_secret.0.to_string()) {
            Ok(pair) => pair,
            Err(e) => {
                return Err(TaskError::Task(format!(
                    "Failed to send signed task response: {:?}",
                    e
                )));
            }
        };
        let operator_id = operator_id_from_g1_pub_key(bls_key_pair.public_key())?;

        // Sign the Hashed Message and send it to the BLS Aggregator
        let msg_hash = keccak256(<TaskResponse as SolType>::abi_encode(&task_response));
        let signed_response = SignedTaskResponse {
            task_response,
            signature: bls_key_pair.sign_message(msg_hash.as_ref()),
            operator_id,
        };

        info!(
            "Sending signed task response to BLS Aggregator: {:#?}",
            signed_response
        );
        if let Err(e) = client.send_signed_task_response(signed_response).await {
            error!("Failed to send signed task response: {:?}", e);
            return Err(TaskError::Task(format!(
                "Failed to send signed task response: {:?}",
                e
            )));
        }
    }

    Ok(())
}
