use crate::contracts::TaskManager::{Task, TaskResponse};
use crate::error::TaskError as Error;
use crate::{
    contexts::client::SignedTaskResponse, contracts::SquaringTask as IncredibleSquaringTaskManager,
    error::TaskError,
};
use alloy_network::Ethereum;
use alloy_primitives::{Address, FixedBytes, keccak256};
use alloy_sol_types::SolType;
use blueprint_sdk::evm::util::{get_provider_from_signer, get_provider_http};
use blueprint_sdk::macros::context::KeystoreContext;
use blueprint_sdk::runner::config::BlueprintEnvironment;
use color_eyre::Result;
use eigenlayer_extra::generic_task_aggregation::{
    AggregationError, EigenTask, ResponseSender, TaskAggregator,
    TaskResponse as GenericTaskResponse,
};
use eigensdk::crypto_bls::{
    BLSOperatorStateRetriever, BlsAggregationService, OperatorId, Signature, convert_to_g1_point,
    convert_to_g2_point,
};
use eigensdk::services_blsaggregation::bls_agg::{
    BlsAggregationServiceConfig, BlsAggregationServiceError, TaskMetadata, TaskSignature,
};
use eigensdk::services_blsaggregation::bls_aggregation_service_response::BlsAggregationServiceResponse;
use eigensdk::types::avs::{TaskIndex, TaskResponseDigest};
use eigensdk::types::operator::QuorumThresholdPercentage;
use jsonrpsee::RpcModule;
use jsonrpsee::core::async_trait;
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::server::{ServerBuilder, ServerHandle};
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, error, info, warn};

use crate::task_types::SquaringTaskResponseSender;

#[rpc(server)]
pub trait AggregatorRpc {
    #[method(name = "process_signed_task_response")]
    async fn process_signed_task_response(
        &self,
        params: SignedTaskResponse,
    ) -> Result<bool, ErrorObjectOwned>;
}

/// Context for the Aggregator server
#[derive(Clone, KeystoreContext)]
pub struct AggregatorContext {
    pub port_address: String,
    pub task_manager_address: Address,
    pub tasks: Arc<Mutex<HashMap<TaskIndex, Task>>>,
    pub tasks_responses: Arc<Mutex<HashMap<TaskIndex, HashMap<TaskResponseDigest, TaskResponse>>>>,
    pub service_handle: Option<Arc<Mutex<BlsAggregationService>>>,
    pub server_handle: Option<Arc<Mutex<ServerHandle>>>,
    pub response_cache: Arc<Mutex<VecDeque<SignedTaskResponse>>>,
    pub task_aggregator:
        Option<Arc<Mutex<TaskAggregator<Task, TaskResponse, SquaringTaskResponseSender>>>>,
    #[config]
    pub std_config: BlueprintEnvironment,
}

impl AggregatorContext {
    /// Creates a new AggregatorContext
    pub fn new(
        port_address: String,
        task_manager_address: Address,
        std_config: BlueprintEnvironment,
    ) -> Self {
        Self {
            port_address,
            task_manager_address,
            tasks: Arc::new(Mutex::new(HashMap::new())),
            tasks_responses: Arc::new(Mutex::new(HashMap::new())),
            service_handle: None,
            server_handle: None,
            response_cache: Arc::new(Mutex::new(VecDeque::new())),
            task_aggregator: None,
            std_config,
        }
    }

    /// Starts the aggregator server
    pub async fn start_server(&mut self) -> Result<(), Error> {
        // Create the BLS Aggregation Service
        let bls_config = BlsAggregationServiceConfig {
            avs_registry_coordinator_address: self.task_manager_address,
            operator_state_retriever: Arc::new(BLSOperatorStateRetriever::new(
                self.task_manager_address,
                None,
            )),
        };

        let bls_service = BlsAggregationService::new(bls_config)
            .map_err(|e| Error::Task(format!("Failed to create BLS Aggregation Service: {}", e)))?;

        self.service_handle = Some(Arc::new(Mutex::new(bls_service)));

        // Create the task aggregator
        let response_sender = SquaringTaskResponseSender {
            task_manager_address: self.task_manager_address,
        };

        let task_aggregator = TaskAggregator::new(
            self.service_handle.as_ref().unwrap().clone(),
            response_sender,
        );

        self.task_aggregator = Some(Arc::new(Mutex::new(task_aggregator)));

        // Start the server
        let server = ServerBuilder::default()
            .build(format!("0.0.0.0:{}", self.port_address))
            .await
            .map_err(|e| Error::Task(format!("Failed to build server: {}", e)))?;

        let mut module = RpcModule::new(());
        let ctx = self.clone();
        module
            .register_async_method("process_signed_task_response", move |params, _| {
                let ctx = ctx.clone();
                async move {
                    debug!("Received RPC request: process_signed_task_response");

                    // Extract the params from the request
                    let params_value = match params.one::<Value>() {
                        Ok(value) => value,
                        Err(e) => {
                            error!("Failed to parse params: {}", e);
                            return Err(ErrorObjectOwned::owned(
                                4001,
                                "Invalid params",
                                Some(format!("Failed to parse params: {}", e)),
                            ));
                        }
                    };

                    // Extract the inner params from the JSON-RPC request
                    let inner_params = match params_value.get("params") {
                        Some(inner_params) => inner_params,
                        None => {
                            error!("Missing 'params' field in request");
                            return Err(ErrorObjectOwned::owned(
                                4002,
                                "Invalid request",
                                Some("Missing 'params' field in request".to_string()),
                            ));
                        }
                    };

                    // Now parse the inner params as SignedTaskResponse
                    let signed_task_response: SignedTaskResponse =
                        match serde_json::from_value(inner_params.clone()) {
                            Ok(response) => response,
                            Err(e) => {
                                error!("Invalid SignedTaskResponse: {}", e);
                                return Err(ErrorObjectOwned::owned(
                                    4003,
                                    "Invalid SignedTaskResponse",
                                    Some(format!("Failed to parse SignedTaskResponse: {}", e)),
                                ));
                            }
                        };

                    // Process the signed task response
                    match ctx.process_signed_task_response(signed_task_response).await {
                        Ok(()) => Ok(true),
                        Err(e) => {
                            error!("Failed to process signed task response: {}", e);
                            Ok(false)
                        }
                    }
                }
            })
            .map_err(|e| Error::Task(format!("Failed to register RPC method: {}", e)))?;

        let server_handle = server.start(module);
        self.server_handle = Some(Arc::new(Mutex::new(server_handle)));

        // Process any cached responses
        let cached_responses = {
            let mut cache = self.response_cache.lock().await;
            let responses = cache.drain(..).collect::<Vec<_>>();
            responses
        };

        for resp in cached_responses {
            if let Err(e) = self.process_signed_task_response(resp).await {
                warn!("Failed to process cached response: {}", e);
            }
        }

        Ok(())
    }

    /// Processes a signed task response
    pub async fn process_signed_task_response(
        &self,
        resp: SignedTaskResponse,
    ) -> Result<(), Error> {
        // If we have a task aggregator, use it to process the response
        if let Some(task_aggregator) = &self.task_aggregator {
            let mut aggregator = task_aggregator.lock().await;

            // Convert the SignedTaskResponse to the generic version
            let generic_response =
                eigenlayer_extra::generic_task_aggregation::SignedTaskResponse::from(resp);

            // Process the response using the generic task aggregator
            match aggregator.process_response(generic_response).await {
                Ok(_) => {
                    info!("Successfully processed response with generic task aggregator");
                    return Ok(());
                }
                Err(AggregationError::TaskNotFound) => {
                    // If the task is not found, cache the response for later processing
                    let task_index = resp.task_response.referenceTaskIndex;
                    info!(
                        "Task {} not yet initialized, caching response for later processing",
                        task_index
                    );
                    let mut cache = self.response_cache.lock().await;
                    cache.push_back(resp);
                    return Ok(());
                }
                Err(e) => {
                    return Err(Error::Task(format!("Failed to process response: {}", e)));
                }
            }
        } else {
            // Fall back to the old implementation if the task aggregator is not available
            self.process_response(resp).await
        }
    }

    /// Processes a signed task response (legacy implementation)
    async fn process_response(&mut self, resp: SignedTaskResponse) -> Result<(), Error> {
        let SignedTaskResponse {
            task_response,
            signature,
            operator_id,
        } = resp;

        let task_index = task_response.referenceTaskIndex;
        let task_response_digest = keccak256(TaskResponse::abi_encode(&task_response));

        // Check if the task exists
        let tasks = self.tasks.lock().await;
        if !tasks.contains_key(&task_index) {
            info!(
                "Task {} not yet initialized, caching response for later processing",
                task_index
            );
            let mut cache = self.response_cache.lock().await;
            cache.push_back(SignedTaskResponse {
                task_response,
                signature,
                operator_id,
            });
            return Ok(());
        }

        // Check if the response has already been processed
        let mut task_responses = self.tasks_responses.lock().await;
        let task_response_map = task_responses.entry(task_index).or_insert(HashMap::new());

        if task_response_map.contains_key(&task_response_digest) {
            info!(
                "Task response digest already processed for task index: {}",
                task_index
            );
            return Ok(());
        }

        // Add the response to the map
        task_response_map.insert(task_response_digest, task_response.clone());

        // Register the signature with the BLS Aggregation Service
        if let Some(service) = &self.service_handle {
            let task_signature =
                TaskSignature::new(task_index, task_response_digest, signature, operator_id);

            service
                .lock()
                .await
                .register_signature(task_signature)
                .await
                .map_err(|e| Error::Task(format!("Failed to register signature: {}", e)))?;
        }

        Ok(())
    }

    /// Stops the aggregator server
    pub async fn stop_server(&mut self) -> Result<(), Error> {
        if let Some(server_handle) = &self.server_handle {
            let mut handle = server_handle.lock().await;
            handle
                .stop()
                .map_err(|e| Error::Task(format!("Failed to stop server: {}", e)))?;
        }
        Ok(())
    }

    /// Checks if a task is ready to be aggregated and submitted
    pub async fn check_aggregation_readiness(
        &self,
        response: BlsAggregationServiceResponse,
    ) -> Result<(), Error> {
        let tasks = self.tasks.lock().await;
        let task = tasks.get(&response.task_index).expect("Task not found");

        let task_responses = self.tasks_responses.lock().await;
        let response_map = task_responses.get(&response.task_index).unwrap();
        let task_response = response_map
            .get(&response.task_response_digest)
            .expect("Task response not found");

        // Get the provider
        let provider = get_provider_http(self.http_endpoint.clone());

        // Create the contract instance
        let task_manager =
            IncredibleSquaringTaskManager::new(self.task_manager_address, provider.clone());

        // Convert the aggregated signature to G1Point
        let aggregated_signature = convert_to_g1_point(&response.agg_signature);
        let agg_sig = IncredibleSquaringTaskManager::G1Point {
            X: aggregated_signature.0,
            Y: aggregated_signature.1,
        };

        // Convert the signing pub keys to G2Point
        let signing_pub_keys = response
            .signing_pub_keys
            .iter()
            .map(|pk| {
                let g2_point = convert_to_g2_point(pk);
                IncredibleSquaringTaskManager::G2Point {
                    X: [g2_point.0, g2_point.1],
                    Y: [g2_point.2, g2_point.3],
                }
            })
            .collect::<Vec<_>>();

        // Create the non-signer stakes and signature
        let non_signer_stakes_and_sig =
            IncredibleSquaringTaskManager::IBLSSignatureCheckerTypes::NonSignerStakesAndSignature {
                nonSignerPubkeys: signing_pub_keys,
                quorumApks: response.quorum_apks,
                apkG1: response.apk_g1,
                apkG2: response.apk_g2,
                sigma: agg_sig,
            };

        // Send the response to the contract
        task_manager
            .respondToSquaringTask(
                response.task_index,
                response.task_response_digest,
                non_signer_stakes_and_sig,
            )
            .send()
            .await
            .map_err(|e| Error::Task(format!("Failed to send response to contract: {}", e)))?;

        Ok(())
    }
}
