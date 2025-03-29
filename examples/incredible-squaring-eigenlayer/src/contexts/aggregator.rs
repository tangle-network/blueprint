use crate::contracts::BN254::{G1Point, G2Point};
use crate::contracts::IBLSSignatureCheckerTypes::NonSignerStakesAndSignature;
use crate::contracts::TaskManager::{Task, TaskResponse};
use crate::error::TaskError as Error;
use crate::{
    contexts::client::SignedTaskResponse, contracts::SquaringTask as IncredibleSquaringTaskManager,
};
use alloy_network::{Ethereum, NetworkWallet};
use alloy_primitives::{Address, keccak256};
use alloy_sol_types::SolType;
use ark_ec::AffineRepr;
use blueprint_sdk::testing::chain_setup::anvil::get_receipt;
use eigensdk::client_avsregistry::writer::AvsRegistryChainWriter;
use jsonrpc_core::{IoHandler, Params, Value};
use jsonrpc_http_server::{AccessControlAllowOrigin, DomainsValidation, ServerBuilder};
use std::{collections::VecDeque, net::SocketAddr, sync::Arc, time::Duration};
use tokio::sync::{Mutex, Notify, oneshot};
use tokio::task::JoinHandle;
use tokio::time::interval;

use alloy_network::EthereumWallet;
use blueprint_sdk::contexts::eigenlayer::EigenlayerContext;
use blueprint_sdk::macros::context::{EigenlayerContext, KeystoreContext};
use blueprint_sdk::runner::config::ProtocolSettings;
use blueprint_sdk::runner::{BackgroundService, config::BlueprintEnvironment, error::RunnerError};
use blueprint_sdk::{debug, error, info, warn};
use eigensdk::client_avsregistry::reader::AvsRegistryChainReader;
use eigensdk::common::get_provider;
use eigensdk::crypto_bls::{BlsG1Point, BlsG2Point, convert_to_g1_point, convert_to_g2_point};
use eigensdk::services_avsregistry::chaincaller::AvsRegistryServiceChainCaller;
use eigensdk::services_blsaggregation::bls_agg::TaskSignature;
use eigensdk::services_blsaggregation::{
    bls_agg, bls_agg::BlsAggregatorService,
    bls_aggregation_service_response::BlsAggregationServiceResponse,
};
use eigensdk::services_operatorsinfo::operatorsinfo_inmemory::OperatorInfoServiceInMemory;
use eigensdk::types::avs::{TaskIndex, TaskResponseDigest};
use std::collections::HashMap;

pub type BlsAggServiceInMemory = BlsAggregatorService<
    AvsRegistryServiceChainCaller<AvsRegistryChainReader, OperatorInfoServiceInMemory>,
>;

#[derive(Clone, EigenlayerContext, KeystoreContext)]
pub struct AggregatorContext {
    pub port_address: String,
    pub task_manager_address: Address,
    pub tasks: Arc<Mutex<HashMap<TaskIndex, Task>>>,
    pub tasks_responses: Arc<Mutex<HashMap<TaskIndex, HashMap<TaskResponseDigest, TaskResponse>>>>,
    pub service_handle: Option<Arc<Mutex<bls_agg::ServiceHandle>>>,
    pub aggregate_receiver: Option<Arc<Mutex<bls_agg::AggregateReceiver>>>,
    pub http_rpc_url: String,
    pub wallet: EthereumWallet,
    pub response_cache: Arc<Mutex<VecDeque<SignedTaskResponse>>>,
    #[config]
    pub sdk_config: BlueprintEnvironment,
    shutdown: Arc<(Notify, Mutex<bool>)>,
}

impl AggregatorContext {
    pub async fn new(
        port_address: String,
        task_manager_address: Address,
        wallet: EthereumWallet,
        sdk_config: BlueprintEnvironment,
    ) -> Result<Self, Error> {
        let mut aggregator_context = AggregatorContext {
            port_address,
            task_manager_address,
            tasks: Arc::new(Mutex::new(HashMap::new())),
            tasks_responses: Arc::new(Mutex::new(HashMap::new())),
            service_handle: None,
            aggregate_receiver: None,
            http_rpc_url: sdk_config.http_rpc_endpoint.clone(),
            wallet,
            response_cache: Arc::new(Mutex::new(VecDeque::new())),
            sdk_config,
            shutdown: Arc::new((Notify::new(), Mutex::new(false))),
        };

        // Initialize the bls registry service
        let bls_service = aggregator_context
            .eigenlayer_client()
            .await
            .map_err(|e| Error::Context(e.to_string()))?
            .bls_aggregation_service_in_memory()
            .await
            .map_err(|e| Error::Context(e.to_string()))?;
        let (service_handle, aggregate_receiver) = bls_service.start();
        aggregator_context.aggregate_receiver = Some(Arc::new(Mutex::new(aggregate_receiver)));
        aggregator_context.service_handle = Some(Arc::new(Mutex::new(service_handle)));

        Ok(aggregator_context)
    }

    pub async fn start(self) -> JoinHandle<()> {
        let aggregator = Arc::new(Mutex::new(self));

        tokio::spawn(async move {
            info!("Starting aggregator RPC server");

            let server_handle = tokio::spawn(Self::start_server(Arc::clone(&aggregator)));

            let process_handle =
                tokio::spawn(Self::process_cached_responses(Arc::clone(&aggregator)));

            // Wait for both tasks to complete
            let (server_result, process_result) = tokio::join!(server_handle, process_handle);

            if let Err(e) = server_result {
                error!("Server task failed: {}", e);
            }
            if let Err(e) = process_result {
                error!("Process cached responses task failed: {}", e);
            }

            info!("Aggregator shutdown complete");
        })
    }

    pub async fn shutdown(&self) {
        info!("Initiating aggregator shutdown");

        // Set internal shutdown flag
        let (notify, is_shutdown) = &*self.shutdown;
        *is_shutdown.lock().await = true;
        notify.notify_waiters();
    }

    async fn start_server(aggregator: Arc<Mutex<Self>>) -> Result<(), Error> {
        let mut io = IoHandler::new();
        io.add_method("process_signed_task_response", {
            let aggregator = Arc::clone(&aggregator);
            move |params: Params| {
                let aggregator = Arc::clone(&aggregator);
                async move {
                    // Parse the outer structure first
                    let outer_params: Value = params.parse()?;

                    // Extract the inner "params" object
                    let inner_params = outer_params.get("params").ok_or_else(|| {
                        jsonrpc_core::Error::invalid_params("Missing 'params' field")
                    })?;

                    // Now parse the inner params as SignedTaskResponse
                    let signed_task_response: SignedTaskResponse =
                        serde_json::from_value(inner_params.clone()).map_err(|e| {
                            jsonrpc_core::Error::invalid_params(format!(
                                "Invalid SignedTaskResponse: {}",
                                e
                            ))
                        })?;

                    aggregator
                        .lock()
                        .await
                        .process_signed_task_response(signed_task_response)
                        .await
                        .map(|_| Value::Bool(true))
                        .map_err(|e| jsonrpc_core::Error::invalid_params(e.to_string()))
                }
            }
        });

        let socket: SocketAddr = aggregator
            .lock()
            .await
            .port_address
            .parse()
            .map_err(Error::Parse)?;
        let server = ServerBuilder::new(io)
            .cors(DomainsValidation::AllowOnly(vec![
                AccessControlAllowOrigin::Any,
            ]))
            .start_http(&socket)
            .map_err(|e| Error::Context(e.to_string()))?;

        info!("Server running at {}", socket);

        // Create a close handle before we move the server
        let close_handle = server.close_handle();

        // Get shutdown components
        let shutdown = {
            let agg = aggregator.lock().await;
            agg.shutdown.clone()
        };

        // Create a channel to coordinate shutdown
        let (server_tx, server_rx) = oneshot::channel();

        // Spawn the server in a blocking task
        let server_handle = tokio::task::spawn_blocking(move || {
            server.wait();
            let _ = server_tx.send(());
        });

        // Use tokio::select! to wait for either the server to finish or the shutdown signal
        tokio::select! {
            result = server_handle => {
                info!("Server has stopped naturally");
                result.map_err(|e| {
                    error!("Server task failed: {}", e);
                    Error::Runtime(e.to_string())
                })?;
            }
            _ = async {
                let (_, is_shutdown) = &*shutdown;
                loop {
                    if *is_shutdown.lock().await {
                        break;
                    }
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            } => {
                info!("Initiating server shutdown");
                // Spawn a blocking task to handle server shutdown
                tokio::task::spawn_blocking(move || {
                    close_handle.close();
                }).await.map_err(|e| Error::Runtime(e.to_string()))?;

                // Wait for server to complete
                let _ = server_rx.await;
                info!("Server has stopped after shutdown");
            }
        }

        info!("Server shutdown complete");
        Ok(())
    }

    async fn process_signed_task_response(
        &mut self,
        resp: SignedTaskResponse,
    ) -> Result<(), Error> {
        let task_index = resp.task_response.referenceTaskIndex;
        let task_response_digest = keccak256(TaskResponse::abi_encode(&resp.task_response));

        info!(
            "Caching signed task response for task index: {}, task response digest: {}",
            task_index, task_response_digest
        );

        self.response_cache.lock().await.push_back(resp);

        Ok(())
    }

    async fn process_cached_responses(aggregator: Arc<Mutex<Self>>) {
        let mut interval = interval(Duration::from_secs(6));

        // Get shutdown components
        let shutdown = {
            let agg = aggregator.lock().await;
            agg.shutdown.clone()
        };

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    // Check shutdown status first
                    if *shutdown.1.lock().await {
                        info!("Process cached responses received shutdown signal");
                        break;
                    }

                    // Get responses to process while holding the lock briefly
                    let responses_to_process = {
                        let guard = aggregator.lock().await;
                        let cache = guard.response_cache.lock().await;
                        cache.clone()
                    };

                    // Process each response without holding the main lock
                    for resp in responses_to_process {
                        let res = {
                            let mut guard = aggregator.lock().await;
                            guard.process_response(resp.clone()).await
                        };
                        match res {
                            Ok(_) => {
                                // Only remove from cache if processing succeeded
                                let guard = aggregator.lock().await;
                                let mut cache = guard.response_cache.lock().await;
                                cache.pop_front();
                            }
                            Err(e) => {
                                error!("Failed to process cached response: {:?}", e);
                                // Continue processing other responses without failing
                            }
                        }
                    }
                }
                _ = shutdown.0.notified() => {
                    if *shutdown.1.lock().await {
                        info!("Process cached responses received shutdown signal");
                        break;
                    }
                }
            }
        }
    }

    async fn process_response(&mut self, resp: SignedTaskResponse) -> Result<(), Error> {
        let SignedTaskResponse {
            task_response,
            signature,
            operator_id,
        } = resp.clone();
        let task_index = task_response.referenceTaskIndex;
        let task_response_digest = keccak256(TaskResponse::abi_encode(&task_response));

        // Check if we have the task initialized first
        if !self.tasks.lock().await.contains_key(&task_index) {
            info!(
                "Task {} not yet initialized, caching response for later processing",
                task_index
            );
            self.response_cache.lock().await.push_back(resp);
            return Ok(());
        }

        if self
            .tasks_responses
            .lock()
            .await
            .entry(task_index)
            .or_default()
            .contains_key(&task_response_digest)
        {
            info!(
                "Task response digest already processed for task index: {}",
                task_index
            );
            return Ok(());
        }

        info!(
            "Processing signed task response for task index: {}, task response digest: {}",
            task_index, task_response_digest
        );

        let task_signature =
            TaskSignature::new(task_index, task_response_digest, signature, operator_id);

        self.service_handle
            .as_ref()
            .ok_or_else(|| std::io::Error::other("BLS Aggregation Service not initialized"))
            .map_err(|e| Error::Context(e.to_string()))?
            .lock()
            .await
            .process_signature(task_signature)
            .await
            .map_err(|e| Error::Context(e.to_string()))?;

        if let Some(tasks_responses) = self.tasks_responses.lock().await.get_mut(&task_index) {
            tasks_responses.insert(task_response_digest, task_response.clone());
        }

        debug!(
            "Successfully processed new signature for task index: {}",
            task_index
        );

        let aggregated_response = self
            .aggregate_receiver
            .as_ref()
            .ok_or_else(|| std::io::Error::other("BLS Aggregation Service not initialized"))
            .map_err(|e| Error::Context(e.to_string()))?
            .lock()
            .await
            .receive_aggregated_response()
            .await;
        let response = aggregated_response.map_err(|e| Error::Context(e.to_string()))?;
        self.send_aggregated_response_to_contract(response).await?;
        Ok(())
    }

    async fn verify_bls_pubkeys_registered(
        &self,
        avs_registry_reader: &AvsRegistryChainReader,
    ) -> Result<bool, Error> {
        let port = self.http_rpc_url.split(':').last().unwrap();
        let ws_rpc_url = format!("ws://127.0.0.1:{}", port);
        warn!("ws_rpc_url: {}", ws_rpc_url);

        // Get the BLS public keys for the operators in the quorums
        let bls_pubkeys = avs_registry_reader
            .query_existing_registered_operator_pub_keys(260u64, 300u64, ws_rpc_url)
            .await
            .map_err(|e| {
                Error::Context(format!(
                    "Failed to get BLS public keys for operators: {}",
                    e
                ))
            })?;

        info!(
            "BLS public keys for operators in quorums: {:?}",
            bls_pubkeys
        );

        // Check if there are any BLS public keys registered
        if bls_pubkeys.1.is_empty() {
            return Ok(false);
        }

        // Check if any of the BLS public keys are at infinity
        for pubkey in bls_pubkeys.1 {
            info!(
                "BLS pubkey G1: {:?}, G2: {:?}",
                pubkey.g1_pub_key, pubkey.g2_pub_key
            );

            // If either G1 or G2 is at infinity, the BLS public key is not properly registered
            if pubkey.g1_pub_key.g1().x().is_none() || pubkey.g2_pub_key.g2().x().is_none() {
                warn!("BLS public key at infinity");
                return Ok(false);
            }
        }

        Ok(true)
    }

    async fn send_aggregated_response_to_contract(
        &self,
        response: BlsAggregationServiceResponse,
    ) -> Result<(), Error> {
        // First, let's check if there are any registered operators for the quorums
        let contract_addresses = self
            .sdk_config
            .protocol_settings
            .eigenlayer()
            .map_err(|e| {
                Error::Context(format!(
                    "Failed to get Eigenlayer contract addresses: {}",
                    e
                ))
            })?;

        let avs_registry_reader = AvsRegistryChainReader::new(
            eigensdk::logging::get_test_logger(),
            contract_addresses.registry_coordinator_address,
            contract_addresses.operator_state_retriever_address,
            self.http_rpc_url.clone(),
        )
        .await
        .map_err(|e| Error::Context(format!("Failed to create AVS registry reader: {}", e)))?;

        // Check if there are any operators registered for the quorum
        let quorum_numbers = vec![0u8]; // Assuming quorum 0 is being used
        let quorum_count = avs_registry_reader
            .get_quorum_count()
            .await
            .map_err(|e| Error::Context(format!("Failed to quorum count: {}", e)))?;
        info!("Quorum count: {}", quorum_count);

        let quorum_operators = avs_registry_reader
            .get_operators_stake_in_quorums_at_current_block(quorum_numbers.clone().into())
            .await
            .map_err(|e| Error::Context(format!("Failed to get operators for quorums: {}", e)))?;

        for (quorum_num, quorum) in quorum_operators.iter().enumerate() {
            info!(
                "Quorum {} has {} registered operators:",
                quorum_num,
                quorum.len(),
            );
            for (i, operator) in quorum.iter().enumerate() {
                info!(
                    "Operator {}: \n\tAddress: {}\n\tOperator ID: {}\n\tStake: {}",
                    i, operator.operator, operator.operatorId, operator.stake
                );
            }

            // If there are no operators registered for this quorum, the quorum APK will be at infinity
            if quorum.is_empty() {
                return Err(Error::Context(format!(
                    "Quorum {} has no registered operators, which would result in an infinity point for the quorum APK",
                    quorum_num
                )));
            }
        }

        // Verify that BLS public keys are properly registered for the operators
        let bls_keys_registered = self
            .verify_bls_pubkeys_registered(&avs_registry_reader)
            .await?;
        if !bls_keys_registered {
            return Err(Error::Context("BLS public keys are not properly registered for operators, which would result in an infinity point for the quorum APK".to_string()));
        }

        let mut non_signer_pub_keys = Vec::<G1Point>::new();
        info!(
            "Processing {} non-signer public keys",
            response.non_signers_pub_keys_g1.len()
        );

        for (i, pub_key) in response.non_signers_pub_keys_g1.iter().enumerate() {
            info!("Processing non-signer public key {}: {:?}", i, pub_key.g1());

            if pub_key.g1().x().is_some() {
                info!("Non-signer public key {} is not at infinity", i);
                let g1 = match convert_to_g1_point(pub_key.g1()) {
                    Ok(g1) => {
                        info!(
                            "Successfully converted non-signer public key {} to G1 point: X={}, Y={}",
                            i, g1.X, g1.Y
                        );
                        g1
                    }
                    Err(e) => {
                        error!(
                            "Failed to convert non-signer public key {} to G1 point: {}",
                            i, e
                        );
                        return Err(Error::Context(e.to_string()));
                    }
                };
                non_signer_pub_keys.push(G1Point { X: g1.X, Y: g1.Y })
            } else {
                info!(
                    "Non-signer public key {} is at infinity for task index: {:?}",
                    i, response.task_index
                );
            }
        }

        let mut quorum_apks = Vec::<G1Point>::new();
        info!("Processing {} quorum APKs", response.quorum_apks_g1.len());

        for (i, quorum_apk) in response.quorum_apks_g1.iter().enumerate() {
            info!("Processing quorum APK {}: {:?}", i, quorum_apk.g1());

            if quorum_apk.g1().x().is_some() {
                info!("Quorum APK {} is not at infinity", i);
                let g1 = match convert_to_g1_point(quorum_apk.g1()) {
                    Ok(g1) => {
                        info!(
                            "Successfully converted quorum APK {} to G1 point: X={}, Y={}",
                            i, g1.X, g1.Y
                        );
                        g1
                    }
                    Err(e) => {
                        error!("Failed to convert quorum APK {} to G1 point: {}", i, e);
                        return Err(Error::Context(e.to_string()));
                    }
                };
                quorum_apks.push(G1Point { X: g1.X, Y: g1.Y });
            } else {
                warn!(
                    "Quorum APK {} is at infinity for task index: {:?}. Attempting to use registered BLS public keys instead.",
                    i, response.task_index
                );

                // Get the registered BLS public keys
                let port = self.http_rpc_url.split(':').last().unwrap();
                let ws_rpc_url = format!("ws://127.0.0.1:{}", port);
                let bls_pubkeys = avs_registry_reader
                    .query_existing_registered_operator_pub_keys(260u64, 300u64, ws_rpc_url)
                    .await
                    .map_err(|e| {
                        Error::Context(format!(
                            "Failed to get BLS public keys for operators: {}",
                            e
                        ))
                    })?;

                if bls_pubkeys.1.is_empty() {
                    error!("No registered BLS public keys found");
                    return Err(Error::Context(format!(
                        "Quorum APK {} is at infinity and no registered BLS public keys found",
                        i
                    )));
                }

                // Use the first valid BLS public key as the quorum APK
                for pubkey in bls_pubkeys.1 {
                    if pubkey.g1_pub_key.g1().x().is_some() {
                        info!(
                            "Using registered BLS public key as quorum APK: {:?}",
                            pubkey.g1_pub_key
                        );
                        let g1 = match convert_to_g1_point(pubkey.g1_pub_key.g1()) {
                            Ok(g1) => {
                                info!(
                                    "Successfully converted registered BLS public key to G1 point: X={}, Y={}",
                                    g1.X, g1.Y
                                );
                                g1
                            }
                            Err(e) => {
                                error!(
                                    "Failed to convert registered BLS public key to G1 point: {}",
                                    e
                                );
                                continue;
                            }
                        };
                        quorum_apks.push(G1Point { X: g1.X, Y: g1.Y });
                        break;
                    }
                }

                if quorum_apks.len() <= i {
                    error!("Could not find a valid registered BLS public key to use as quorum APK");
                    return Err(Error::Context(format!(
                        "Quorum APK {} is at infinity and no valid registered BLS public key found",
                        i
                    )));
                }
            }
        }

        let non_signer_stakes_and_signature = NonSignerStakesAndSignature {
            nonSignerPubkeys: non_signer_pub_keys,
            nonSignerQuorumBitmapIndices: response.non_signer_quorum_bitmap_indices,
            quorumApks: quorum_apks,
            apkG2: G2Point {
                X: convert_to_g2_point(response.signers_apk_g2.g2())
                    .map_err(|e| Error::Context(e.to_string()))?
                    .X,
                Y: convert_to_g2_point(response.signers_apk_g2.g2())
                    .map_err(|e| Error::Context(e.to_string()))?
                    .Y,
            },
            sigma: G1Point {
                X: convert_to_g1_point(response.signers_agg_sig_g1.g1_point().g1())
                    .map_err(|e| Error::Context(e.to_string()))?
                    .X,
                Y: convert_to_g1_point(response.signers_agg_sig_g1.g1_point().g1())
                    .map_err(|e| Error::Context(e.to_string()))?
                    .Y,
            },
            quorumApkIndices: response.quorum_apk_indices,
            totalStakeIndices: response.total_stake_indices,
            nonSignerStakeIndices: response.non_signer_stake_indices,
        };

        let tasks = self.tasks.lock().await;
        let task_responses = self.tasks_responses.lock().await;
        let task = tasks.get(&response.task_index).expect("Task not found");
        let task_response = task_responses
            .get(&response.task_index)
            .and_then(|responses| responses.get(&response.task_response_digest))
            .expect("Task response not found");

        let provider = get_provider(&self.http_rpc_url);
        let task_manager =
            IncredibleSquaringTaskManager::new(self.task_manager_address, provider.clone());

        let aggregator_address = NetworkWallet::<Ethereum>::default_signer_address(&self.wallet);

        let on_chain_agg = task_manager.aggregator().call().await.unwrap()._0;
        assert_eq!(aggregator_address, on_chain_agg);

        let response_call = task_manager
            .respondToSquaringTask(
                task.clone(),
                task_response.clone(),
                non_signer_stakes_and_signature,
            )
            .from(aggregator_address);

        let response_receipt = get_receipt(response_call).await;
        match response_receipt {
            Ok(receipt) => {
                if receipt.status() {
                    info!(
                        "Successfully sent aggregated response to contract for task index: {}",
                        response.task_index
                    );
                } else {
                    error!("Failed to send aggregated response to contract");
                    return Err(Error::Chain(
                        "Failed to send aggregated response to contract".to_string(),
                    ));
                }
            }
            Err(e) => {
                error!("Failed to get receipt: {}", e);
                return Err(Error::Chain(e.to_string()));
            }
        }

        Ok(())
    }
}

impl BackgroundService for AggregatorContext {
    async fn start(&self) -> Result<oneshot::Receiver<Result<(), RunnerError>>, RunnerError> {
        let handle = self.clone().start().await;
        info!("Aggregator task started");
        let (result_tx, result_rx) = oneshot::channel();

        tokio::spawn(async move {
            match handle.await {
                Ok(_) => {
                    info!("Aggregator task finished");
                    let _ = result_tx.send(Ok(()));
                }
                Err(e) => {
                    error!("Aggregator task failed: {}", e);
                    let _ = result_tx.send(Err(RunnerError::Eigenlayer(format!(
                        "Aggregator task failed: {:?}",
                        e
                    ))));
                }
            }
        });

        Ok(result_rx)
    }
}
