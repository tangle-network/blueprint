use crate::contracts::task_manager::IBLSSignatureChecker::NonSignerStakesAndSignature;
use crate::contracts::task_manager::BN254::{G1Point, G2Point};
use crate::contracts::task_manager::{SquaringTask, TaskManager};
use blueprint_eigenlayer_extra::contract_conversions::{
    convert_aggregation_response, ContractG1Point, ContractG2Point,
    NonSignerStakesAndSignature as NSSTrait,
};
use blueprint_eigenlayer_extra::generic_task_aggregation::{
    AggregationError, EigenTask, ResponseSender, Result, TaskResponse,
};
use blueprint_sdk::alloy::hex;
use blueprint_sdk::alloy::primitives::{Address, U256};
use blueprint_sdk::alloy::sol_types::SolType;
use blueprint_sdk::eigensdk::crypto_bls::{self, BlsG1Point, BlsG2Point};
use blueprint_sdk::eigensdk::services_blsaggregation::bls_aggregation_service_response::BlsAggregationServiceResponse;
use blueprint_sdk::evm::util::get_provider_http;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;

/// Implementation of EigenTask for the Incredible Squaring task
#[derive(Clone, Debug)]
pub struct IncredibleSquaringTask {
    pub task_index: u32,
    pub contract_task: TaskManager::Task,
}

impl EigenTask for IncredibleSquaringTask {
    fn task_index(&self) -> u32 {
        self.task_index
    }

    fn created_block(&self) -> u32 {
        self.contract_task.taskCreatedBlock
    }

    fn quorum_numbers(&self) -> Vec<u8> {
        self.contract_task.quorumNumbers.to_vec()
    }

    fn quorum_threshold_percentage(&self) -> u8 {
        self.contract_task.quorumThresholdPercentage as u8
    }

    fn encode(&self) -> Vec<u8> {
        TaskManager::Task::abi_encode(&self.contract_task)
    }
}

/// Implementation of TaskResponse for the Incredible Squaring response
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IncredibleSquaringTaskResponse {
    pub contract_response: TaskManager::TaskResponse,
}

impl TaskResponse for IncredibleSquaringTaskResponse {
    fn reference_task_index(&self) -> u32 {
        self.contract_response.referenceTaskIndex
    }

    fn encode(&self) -> Vec<u8> {
        TaskManager::TaskResponse::abi_encode(&self.contract_response)
    }
}

// Implement traits for G1Point to work with our conversion utilities
impl ContractG1Point for G1Point {
    type X = U256;
    type Y = U256;

    fn new(x: Self::X, y: Self::Y) -> Self {
        Self { X: x, Y: y }
    }
}

// Implement traits for G2Point to work with our conversion utilities
impl ContractG2Point for G2Point {
    type X = [U256; 2];
    type Y = [U256; 2];

    fn new(x: Self::X, y: Self::Y) -> Self {
        Self { X: x, Y: y }
    }
}

// Implement trait for NonSignerStakesAndSignature to work with our conversion utilities
impl NSSTrait<G1Point, G2Point> for NonSignerStakesAndSignature {
    fn new(
        non_signer_pubkeys: Vec<G1Point>,
        non_signer_quorum_bitmap_indices: Vec<u32>,
        quorum_apks: Vec<G1Point>,
        apk_g2: G2Point,
        sigma: G1Point,
        quorum_apk_indices: Vec<u32>,
        total_stake_indices: Vec<u32>,
        non_signer_stake_indices: Vec<Vec<u32>>,
    ) -> Self {
        Self {
            nonSignerPubkeys: non_signer_pubkeys,
            nonSignerQuorumBitmapIndices: non_signer_quorum_bitmap_indices,
            quorumApks: quorum_apks,
            apkG2: apk_g2,
            sigma,
            quorumApkIndices: quorum_apk_indices,
            totalStakeIndices: total_stake_indices,
            nonSignerStakeIndices: non_signer_stake_indices,
        }
    }
}

/// Service for sending response to the Incredible Squaring TaskManager contract
#[derive(Clone)]
pub struct IncredibleSquaringResponseSender {
    task_manager_address: Address,
    http_endpoint: String,
}

impl IncredibleSquaringResponseSender {
    pub fn new(task_manager_address: Address, http_endpoint: String) -> Self {
        Self {
            task_manager_address,
            http_endpoint,
        }
    }
}

impl ResponseSender<IncredibleSquaringTask, IncredibleSquaringTaskResponse>
    for IncredibleSquaringResponseSender
{
    type Future = Pin<Box<dyn Future<Output = Result<()>> + Send + 'static>>;

    fn send_aggregated_response(
        &self,
        task: &IncredibleSquaringTask,
        response: &IncredibleSquaringTaskResponse,
        aggregation_result: BlsAggregationServiceResponse,
    ) -> Self::Future {
        // Clone the values we need to move into the async block
        let task_manager_address = self.task_manager_address.clone();
        let provider = get_provider_http(&self.http_endpoint);
        let task_manager = SquaringTask::SquaringTaskInstance::new(task_manager_address, provider);
        let contract_task = task.contract_task.clone();
        let contract_response = response.contract_response.clone();

        Box::pin(async move {
            // Convert coordinates from the BLS library format to the contract format
            let convert_g1 = |point: &BlsG1Point| -> (U256, U256) {
                let pt = crypto_bls::convert_to_g1_point(point.g1())
                    .expect("Failed to convert G1 point");
                (pt.X, pt.Y)
            };

            let convert_g2 = |point: &BlsG2Point| -> ([U256; 2], [U256; 2]) {
                let pt = crypto_bls::convert_to_g2_point(point.g2())
                    .expect("Failed to convert G2 point");
                (pt.X, pt.Y)
            };

            // Use the generalized conversion utility
            let non_signer_stakes_and_signature =
                convert_aggregation_response::<G1Point, G2Point, NonSignerStakesAndSignature>(
                    &aggregation_result,
                    convert_g1,
                    convert_g2,
                );

            blueprint_sdk::info!(
                "Sending response to task {} with result {}",
                contract_response.referenceTaskIndex,
                hex::encode(&contract_response.message),
            );

            // Call the respondToSquaringTask function
            let tx = task_manager
                .respondToSquaringTask(
                    contract_task,
                    contract_response.clone(),
                    non_signer_stakes_and_signature,
                )
                .send()
                .await
                .map_err(|e| AggregationError::ContractError(e.to_string()))?;

            // Wait for receipt to confirm transaction succeeded
            let receipt = tx
                .get_receipt()
                .await
                .map_err(|e| AggregationError::ContractError(e.to_string()))?;

            if !receipt.status() {
                return Err(AggregationError::ContractError(
                    "Transaction failed".to_string(),
                ));
            }

            blueprint_sdk::info!(
                "Successfully sent response to task {} in tx {:?}",
                contract_response.referenceTaskIndex,
                receipt.transaction_hash,
            );

            Ok(())
        })
    }
}
