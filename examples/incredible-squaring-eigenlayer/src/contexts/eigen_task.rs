use crate::BN254::{G1Point, G2Point};
use crate::IBLSSignatureCheckerTypes::NonSignerStakesAndSignature;
use crate::SquaringTask as IncredibleSquaringTaskManager;
use crate::TaskManager::{Task, TaskResponse};
use alloy_primitives::address;
use alloy_sol_types::SolType;
use blueprint_sdk::eigenlayer::generic_task_aggregation::{
    EigenTask, ResponseSender, Result as AggResult, TaskResponse as GenericTaskResponse,
};
use blueprint_sdk::evm::util::get_provider_from_signer;
use eigensdk::crypto_bls::{BlsG1Point, BlsG2Point, convert_to_g1_point, convert_to_g2_point};
use eigensdk::services_blsaggregation::bls_aggregation_service_response::BlsAggregationServiceResponse;
use eigensdk::types::avs::TaskIndex;
use reqwest::Url;
use std::future::Future;
use ark_ec::AffineRepr;
use std::pin::Pin;

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
    pub http_rpc_url: Url,
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

        blueprint_sdk::info!("aggregation_result: {:#?}", aggregation_result);

        // Validate aggregation result before processing
        blueprint_sdk::info!("Validating aggregation result...");
        blueprint_sdk::info!("non_signers_pub_keys_g1 count: {}", aggregation_result.non_signers_pub_keys_g1.len());
        blueprint_sdk::info!("quorum_apks_g1 count: {}", aggregation_result.quorum_apks_g1.len());
        blueprint_sdk::info!("signers_apk_g2: {:?}", aggregation_result.signers_apk_g2);
        blueprint_sdk::info!("signers_agg_sig_g1: {:?}", aggregation_result.signers_agg_sig_g1);

        Box::pin(async move {
            let key = "0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6"; // Private key from our Aggregator Anvil account
            let provider = get_provider_from_signer(key, http_rpc_url);

            let contract =
                IncredibleSquaringTaskManager::new(task_manager_address, provider.clone());

            // Convert the aggregation result to the NonSignerStakesAndSignature format
            blueprint_sdk::info!("Starting conversion of aggregation result...");
            
            // Convert non-signer pubkeys
            blueprint_sdk::info!("Converting non_signers_pub_keys_g1 (count: {})", aggregation_result.non_signers_pub_keys_g1.len());
            let non_signer_pubkeys = aggregation_result
                .non_signers_pub_keys_g1
                .into_iter()
                .enumerate()
                .map(|(i, pk)| {
                    blueprint_sdk::info!("Converting non-signer pubkey {}: {:?}", i, pk);
                    to_g1_point(pk)
                })
                .collect::<Vec<_>>();
            blueprint_sdk::info!("Successfully converted {} non-signer pubkeys", non_signer_pubkeys.len());

            // Convert quorum APKs
            blueprint_sdk::info!("Converting quorum_apks_g1 (count: {})", aggregation_result.quorum_apks_g1.len());
            
            // Debug each quorum APK in detail
            for (i, pk) in aggregation_result.quorum_apks_g1.iter().enumerate() {
                blueprint_sdk::info!("Quorum APK {} details:", i);
                blueprint_sdk::info!("  - BlsG1Point: {:?}", pk);
                blueprint_sdk::info!("  - g1() data: {:?}", pk.g1());

                let g1 = pk.g1();
                blueprint_sdk::info!("Is x point exits: {}", g1.x().is_some());
                blueprint_sdk::info!("Is y point exits: {}", g1.y().is_some());
                
                // Try to convert and catch the error
                match convert_to_g1_point(pk.g1()) {
                    Ok(pt) => {
                        blueprint_sdk::info!("  - Conversion successful: X={:?}, Y={:?}", pt.X, pt.Y);
                    }
                    Err(e) => {
                        blueprint_sdk::error!("  - Conversion failed: {:?}", e);
                        blueprint_sdk::error!("  - Raw g1 data: {:?}", pk.g1());
                        blueprint_sdk::error!("  - BlsG1Point structure: {:?}", pk);
                    }
                }
            }
            
            let quorum_apks = aggregation_result
                .quorum_apks_g1
                .into_iter()
                .enumerate()
                .map(|(i, pk)| {
                    blueprint_sdk::info!("Converting quorum APK {}: {:?}", i, pk);
                    to_g1_point(pk)
                })
                .collect::<Vec<_>>();
            blueprint_sdk::info!("Successfully converted {} quorum APKs", quorum_apks.len());

            // Convert APK G2
            blueprint_sdk::info!("Converting signers_apk_g2: {:?}", aggregation_result.signers_apk_g2);
            let apk_g2 = to_g2_point(aggregation_result.signers_apk_g2);
            blueprint_sdk::info!("Successfully converted APK G2");

            // Convert signature G1
            blueprint_sdk::info!("Converting signers_agg_sig_g1: {:?}", aggregation_result.signers_agg_sig_g1);
            let sigma = to_g1_point(aggregation_result.signers_agg_sig_g1.g1_point());
            blueprint_sdk::info!("Successfully converted signature G1");

            let non_signer_stakes_and_signature = NonSignerStakesAndSignature {
                nonSignerPubkeys: non_signer_pubkeys,
                nonSignerQuorumBitmapIndices: aggregation_result.non_signer_quorum_bitmap_indices,
                quorumApks: quorum_apks,
                apkG2: apk_g2,
                sigma: sigma,
                quorumApkIndices: aggregation_result.quorum_apk_indices,
                totalStakeIndices: aggregation_result.total_stake_indices,
                nonSignerStakeIndices: aggregation_result.non_signer_stake_indices,
            };

            // Send the response to the contract
            contract
                .respondToSquaringTask(task_clone, response_clone, non_signer_stakes_and_signature)
                .from(address!("a0Ee7A142d267C1f36714E4a8F75612F20a79720")) // Aggregator Anvil account address
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
