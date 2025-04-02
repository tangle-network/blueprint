use crate::contracts::BN254::{G1Point, G2Point};
use crate::contracts::IBLSSignatureCheckerTypes::NonSignerStakesAndSignature;
use crate::contracts::SquaringTask as IncredibleSquaringTaskManager;
use crate::contracts::TaskManager::{Task, TaskResponse};
use crate::error::TaskError;
use alloy_primitives::{FixedBytes, address, keccak256};
use alloy_sol_types::SolType;
use blueprint_sdk::eigenlayer::generic_task_aggregation::{
    EigenTask, ResponseSender, Result as AggResult, TaskResponse as GenericTaskResponse,
};
use blueprint_sdk::evm::util::get_provider_from_signer;
use eigensdk::crypto_bls::{BlsG1Point, BlsG2Point, convert_to_g1_point, convert_to_g2_point};
use eigensdk::services_blsaggregation::bls_aggregation_service_response::BlsAggregationServiceResponse;
use eigensdk::types::avs::{TaskIndex, TaskResponseDigest};
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicU32, Ordering};

// Wrapper for Task that includes the task index
#[derive(Clone)]
pub struct IndexedTask {
    pub task: Task,
    pub task_index: TaskIndex,
}

impl IndexedTask {
    pub fn new(task: Task, task_index: TaskIndex) -> Self {
        Self { task, task_index }
    }
}

// Implement EigenTask for the IndexedTask type
impl EigenTask for IndexedTask {
    fn task_index(&self) -> TaskIndex {
        self.task_index
    }

    fn created_block(&self) -> u32 {
        self.task.taskCreatedBlock
    }

    fn quorum_numbers(&self) -> Vec<u8> {
        self.task.quorumNumbers.to_vec()
    }

    fn quorum_threshold_percentage(&self) -> u8 {
        self.task.quorumThresholdPercentage as u8
    }

    fn encode(&self) -> Vec<u8> {
        <Task as SolType>::abi_encode(&self.task).to_vec()
    }
}

// Implement TaskResponse for the existing TaskResponse type
impl GenericTaskResponse for TaskResponse {
    fn reference_task_index(&self) -> TaskIndex {
        self.referenceTaskIndex
    }

    fn encode(&self) -> Vec<u8> {
        <TaskResponse as SolType>::abi_encode(self).to_vec()
    }
}

// Implement ResponseSender for sending aggregated responses to the contract
#[derive(Clone)]
pub struct SquaringTaskResponseSender {
    pub task_manager_address: alloy_primitives::Address,
    pub http_rpc_url: String,
}

impl ResponseSender<IndexedTask, TaskResponse> for SquaringTaskResponseSender {
    type Future = Pin<Box<dyn Future<Output = AggResult<()>> + Send + 'static>>;

    fn send_aggregated_response(
        &self,
        indexed_task: &IndexedTask,
        response: &TaskResponse,
        aggregation_result: BlsAggregationServiceResponse,
    ) -> Self::Future {
        let task_clone = indexed_task.task.clone();
        let response_clone = response.clone();
        let task_manager_address = self.task_manager_address;
        let http_rpc_url = self.http_rpc_url.clone();

        Box::pin(async move {
            let key = "0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6";
            let provider = get_provider_from_signer(key, &http_rpc_url);

            let contract =
                IncredibleSquaringTaskManager::new(task_manager_address, provider.clone());

            // Convert the aggregation result to the NonSignerStakesAndSignature format
            let non_signer_stakes_and_signature = NonSignerStakesAndSignature {
                nonSignerPubkeys: aggregation_result
                    .non_signers_pub_keys_g1
                    .into_iter()
                    .map(to_g1_point)
                    .collect(),
                nonSignerQuorumBitmapIndices: aggregation_result.non_signer_quorum_bitmap_indices,
                quorumApks: aggregation_result
                    .quorum_apks_g1
                    .into_iter()
                    .map(to_g1_point)
                    .collect(),
                apkG2: to_g2_point(aggregation_result.signers_apk_g2),
                sigma: to_g1_point(aggregation_result.signers_agg_sig_g1.g1_point()),
                quorumApkIndices: aggregation_result.quorum_apk_indices,
                totalStakeIndices: aggregation_result.total_stake_indices,
                nonSignerStakeIndices: aggregation_result.non_signer_stake_indices,
            };

            // Send the response to the contract
            contract
                .respondToSquaringTask(task_clone, response_clone, non_signer_stakes_and_signature)
                .from(address!("a0Ee7A142d267C1f36714E4a8F75612F20a79720"))
                .send()
                .await
                .map_err(|e| blueprint_sdk::eigenlayer::generic_task_aggregation::AggregationError::ContractError(e.to_string()))?
                .get_receipt()
                .await
                .map_err(|e| blueprint_sdk::eigenlayer::generic_task_aggregation::AggregationError::ContractError(e.to_string()))?;

            Ok(())
        })
    }
}

fn to_g1_point(pk: BlsG1Point) -> G1Point {
    let pt = convert_to_g1_point(pk.g1()).expect("Invalid G1 point");
    G1Point { X: pt.X, Y: pt.Y }
}

fn to_g2_point(pk: BlsG2Point) -> G2Point {
    let pt = convert_to_g2_point(pk.g2()).expect("Invalid G2 point");
    G2Point { X: pt.X, Y: pt.Y }
}
