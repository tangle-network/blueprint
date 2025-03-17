use alloy_primitives::{Address, FixedBytes, keccak256, U256};
use eigenlayer_extra::generic_task_aggregation::{
    AggregationError, EigenTask, Result, ResponseSender, TaskResponse,
};
use eigenlayer_extra::contract_conversions::{
    ContractG1Point, ContractG2Point, NonSignerStakesAndSignature as NSSTrait,
    convert_aggregation_response,
};
use eigensdk::services_blsaggregation::bls_aggregation_service_response::BlsAggregationServiceResponse;
use eigensdk::types::avs::TaskIndex;
use eigensdk::crypto_bls::{BlsG1Point, BlsG2Point};
use std::pin::Pin;
use std::future::Future;
use std::sync::Arc;
use blueprint_sdk::alloy::sol_types::SolType;
use blueprint_sdk::chain::{ContractReceipt, ContractWrite};
use std::fmt::Debug;

use crate::contracts::IIncredibleSquaringTaskManager::{self, Task as ContractTask, TaskResponse as ContractTaskResponse};
use crate::contracts::IBLSSignatureChecker::NonSignerStakesAndSignature;
use crate::contracts::BN254::{G1Point, G2Point};
use crate::IncredibleSquaringTaskManager;

/// Implementation of EigenTask for the Incredible Squaring task
#[derive(Clone, Debug)]
pub struct IncredibleTask {
    pub task_index: TaskIndex,
    pub contract_task: ContractTask,
}

impl EigenTask for IncredibleTask {
    fn task_index(&self) -> TaskIndex {
        self.task_index
    }

    fn created_block(&self) -> u32 {
        self.contract_task.taskCreatedBlock
    }

    fn quorum_numbers(&self) -> Vec<u8> {
        self.contract_task.quorumNumbers.to_vec()
    }

    fn quorum_threshold_percentage(&self) -> u32 {
        self.contract_task.quorumThresholdPercentage as u32
    }

    fn encode(&self) -> Vec<u8> {
        IIncredibleSquaringTaskManager::Task::abi_encode(&self.contract_task)
    }
}

/// Implementation of TaskResponse for the Incredible Squaring response
#[derive(Clone, Debug)]
pub struct IncredibleTaskResponse {
    pub contract_response: ContractTaskResponse,
}

impl TaskResponse for IncredibleTaskResponse {
    fn reference_task_index(&self) -> TaskIndex {
        self.contract_response.referenceTaskIndex
    }

    fn encode(&self) -> Vec<u8> {
        IIncredibleSquaringTaskManager::TaskResponse::abi_encode(&self.contract_response)
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
        non_signer_stake_indices: Vec<u32>,
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
pub struct IncredibleResponseSender {
    task_manager: Arc<IncredibleSquaringTaskManager>,
}

impl IncredibleResponseSender {
    pub fn new(task_manager: IncredibleSquaringTaskManager) -> Self {
        Self {
            task_manager: Arc::new(task_manager),
        }
    }
}

impl ResponseSender<IncredibleTask, IncredibleTaskResponse> for IncredibleResponseSender {
    type Future = Pin<Box<dyn Future<Output = Result<()>> + Send + 'static>>;

    fn send_aggregated_response(
        &self,
        task: &IncredibleTask,
        response: &IncredibleTaskResponse,
        aggregation_result: BlsAggregationServiceResponse,
    ) -> Self::Future {
        // Clone the values we need to move into the async block
        let task_manager = self.task_manager.clone();
        let contract_task = task.contract_task.clone();
        let contract_response = response.contract_response.clone();
        
        Box::pin(async move {
            // Convert coordinates from the BLS library format to the contract format
            let convert_g1 = |point: &BlsG1Point| -> (U256, U256) {
                let pt = eigensdk::crypto_bls::convert_to_g1_point(point.g1())
                    .expect("Failed to convert G1 point");
                (pt.X, pt.Y)
            };
            
            let convert_g2 = |point: &BlsG2Point| -> ([U256; 2], [U256; 2]) {
                let pt = eigensdk::crypto_bls::convert_to_g2_point(point.g2())
                    .expect("Failed to convert G2 point");
                (pt.X, pt.Y)
            };
            
            // Use the generalized conversion utility
            let non_signer_stakes_and_signature = convert_aggregation_response::<G1Point, G2Point, NonSignerStakesAndSignature>(
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
                    contract_response,
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