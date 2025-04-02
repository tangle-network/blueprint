use alloy_primitives::{FixedBytes, keccak256};
use alloy_sol_types::SolType;
use blueprint_sdk::evm::util::get_provider_http;
use eigensdk::types::avs::TaskIndex;
use serde::{Deserialize, Serialize};

use crate::contracts::TaskManager::{Task, TaskResponse};
use eigenlayer_extra::generic_task_aggregation::{EigenTask, TaskResponse as GenericTaskResponse};

/// Implementation of the EigenTask trait for the SquaringTask's Task structure
impl EigenTask for Task {
    fn task_index(&self) -> TaskIndex {
        self.taskCreatedBlock as TaskIndex
    }

    fn created_block(&self) -> u32 {
        self.taskCreatedBlock
    }

    fn quorum_numbers(&self) -> Vec<u8> {
        self.quorumNumbers.0.to_vec()
    }

    fn quorum_threshold_percentage(&self) -> u8 {
        self.quorumThresholdPercentage
    }

    fn encode(&self) -> Vec<u8> {
        <Task as SolType>::abi_encode(self)
    }

    fn digest(&self) -> FixedBytes<32> {
        keccak256(self.encode())
    }
}

/// Implementation of the TaskResponse trait for the SquaringTask's TaskResponse structure
impl GenericTaskResponse for TaskResponse {
    fn reference_task_index(&self) -> TaskIndex {
        self.referenceTaskIndex
    }

    fn encode(&self) -> Vec<u8> {
        <TaskResponse as SolType>::abi_encode(self)
    }

    fn digest(&self) -> FixedBytes<32> {
        keccak256(self.encode())
    }
}

/// Conversion from the client's SignedTaskResponse to the generic SignedTaskResponse
impl From<crate::contexts::client::SignedTaskResponse>
    for eigenlayer_extra::generic_task_aggregation::SignedTaskResponse<TaskResponse>
{
    fn from(resp: crate::contexts::client::SignedTaskResponse) -> Self {
        Self::new(resp.task_response, resp.signature, resp.operator_id)
    }
}

#[derive(Clone, Debug)]
pub struct SquaringTaskResponseSender {
    pub task_manager_address: alloy_primitives::Address,
}

impl eigenlayer_extra::generic_task_aggregation::ResponseSender<Task, TaskResponse>
    for SquaringTaskResponseSender
{
    type Future = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = eigenlayer_extra::generic_task_aggregation::Result<()>>
                + Send
                + 'static,
        >,
    >;

    fn send_aggregated_response(
        &self,
        _task: &Task,
        _response: &TaskResponse,
        aggregation_result: eigensdk::services_blsaggregation::bls_aggregation_service_response::BlsAggregationServiceResponse,
    ) -> Self::Future {
        use crate::contracts::BN254::{G1Point, G2Point};
        use crate::contracts::IBLSSignatureCheckerTypes::NonSignerStakesAndSignature;
        use crate::contracts::SquaringTask as IncredibleSquaringTaskManager;
        use alloy_network::Ethereum;
        use blueprint_sdk::evm::util::get_provider_from_signer;
        use eigensdk::crypto_bls::{convert_to_g1_point, convert_to_g2_point};

        let task_manager_address = self.task_manager_address;

        Box::pin(async move {
            // Get the provider
            let provider = get_provider_http(self.http_endpoint.clone());

            // Create the contract instance
            let task_manager =
                IncredibleSquaringTaskManager::new(task_manager_address, provider.clone());

            // Convert the aggregated signature to G1Point
            let aggregated_signature = convert_to_g1_point(&aggregation_result.agg_signature);
            let agg_sig = G1Point {
                X: aggregated_signature.0,
                Y: aggregated_signature.1,
            };

            // Convert the signing pub keys to G2Point
            let signing_pub_keys = aggregation_result
                .signing_pub_keys
                .iter()
                .map(|pk| {
                    let g2_point = convert_to_g2_point(pk);
                    G2Point {
                        X: [g2_point.0, g2_point.1],
                        Y: [g2_point.2, g2_point.3],
                    }
                })
                .collect::<Vec<_>>();

            // Create the non-signer stakes and signature
            let non_signer_stakes_and_sig = NonSignerStakesAndSignature {
                nonSignerPubkeys: signing_pub_keys,
                quorumApks: aggregation_result.quorum_apks,
                apkG1: aggregation_result.apk_g1,
                apkG2: aggregation_result.apk_g2,
                sigma: agg_sig,
            };

            // Send the response to the contract
            task_manager
                .respondToSquaringTask(
                    aggregation_result.task_index,
                    aggregation_result.task_response_digest,
                    non_signer_stakes_and_sig,
                )
                .send()
                .await
                .map_err(|e| {
                    eigenlayer_extra::generic_task_aggregation::AggregationError::ContractError(
                        e.to_string(),
                    )
                })?;

            Ok(())
        })
    }
}
