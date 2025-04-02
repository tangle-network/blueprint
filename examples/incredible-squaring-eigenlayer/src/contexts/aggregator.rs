use crate::contracts::BN254::{G1Point, G2Point};
use crate::contracts::IBLSSignatureCheckerTypes::NonSignerStakesAndSignature;
use crate::contracts::TaskManager::{Task, TaskResponse};
use crate::error::TaskError as Error;
use crate::{
    contexts::client::SignedTaskResponse,
    contexts::eigen_task::{IndexedTask, SquaringTaskResponseSender},
    contracts::SquaringTask as IncredibleSquaringTaskManager,
};
use alloy_network::{Ethereum, NetworkWallet};
use alloy_primitives::{Address, keccak256};
use alloy_sol_types::SolType;
use jsonrpc_core::{IoHandler, Params, Value};
use jsonrpc_http_server::{AccessControlAllowOrigin, DomainsValidation, ServerBuilder};
use std::{collections::VecDeque, net::SocketAddr, sync::Arc, time::Duration};
use tokio::sync::{Mutex, Notify, oneshot};
use tokio::task::JoinHandle;
use tokio::time::interval;

use alloy_network::EthereumWallet;
use blueprint_sdk::contexts::eigenlayer::EigenlayerContext;
use blueprint_sdk::eigenlayer::generic_task_aggregation::{
    AggregatorConfig, BlsAggServiceInMemory, Result as AggResult,
    SignedTaskResponse as GenericSignedTaskResponse, TaskAggregator,
};
use blueprint_sdk::macros::context::{EigenlayerContext, KeystoreContext};
use blueprint_sdk::runner::{BackgroundService, config::BlueprintEnvironment, error::RunnerError};
use blueprint_sdk::{debug, error, info};
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

#[derive(Clone, EigenlayerContext, KeystoreContext)]
pub struct AggregatorContext {
    pub port_address: String,
    pub task_manager_address: Address,
    pub http_rpc_url: String,
    pub wallet: EthereumWallet,
    pub response_cache: Arc<Mutex<VecDeque<SignedTaskResponse>>>,
    #[config]
    pub env: BlueprintEnvironment,
    shutdown: Arc<(Notify, Mutex<bool>)>,
    // Generic task aggregator
    pub task_aggregator:
        Option<Arc<TaskAggregator<IndexedTask, TaskResponse, SquaringTaskResponseSender>>>,
}

impl AggregatorContext {
    pub async fn new(
        port_address: String,
        task_manager_address: Address,
        wallet: EthereumWallet,
        env: BlueprintEnvironment,
    ) -> Result<Self, Error> {
        let mut aggregator_context = AggregatorContext {
            port_address,
            task_manager_address,
            http_rpc_url: env.http_rpc_endpoint.clone(),
            wallet,
            response_cache: Arc::new(Mutex::new(VecDeque::new())),
            env: env.clone(),
            shutdown: Arc::new((Notify::new(), Mutex::new(false))),
            task_aggregator: None,
        };

        // Initialize the bls registry service
        let bls_service = aggregator_context
            .eigenlayer_client()
            .await
            .map_err(|e| Error::Context(e.to_string()))?
            .bls_aggregation_service_in_memory()
            .await
            .map_err(|e| Error::Context(e.to_string()))?;

        // Create the response sender
        let response_sender = SquaringTaskResponseSender {
            task_manager_address,
            http_rpc_url: env.http_rpc_endpoint.clone(),
        };

        // Create the task aggregator with default config
        let task_aggregator =
            TaskAggregator::new(bls_service, response_sender, AggregatorConfig::default());

        aggregator_context.task_aggregator = Some(Arc::new(task_aggregator));

        Ok(aggregator_context)
    }

    pub async fn start(self) -> JoinHandle<()> {
        let aggregator = Arc::new(Mutex::new(self));

        tokio::spawn(async move {
            info!("Starting aggregator RPC server");

            // Start the task aggregator
            if let Some(task_agg) = &aggregator.lock().await.task_aggregator {
                info!("Starting task aggregator");
                task_agg.start().await;
            }

            let server_handle = tokio::spawn(Self::start_server(Arc::clone(&aggregator)));

            info!("Aggregator server started and running in the background");
            // Wait for server task to complete
            if let Err(e) = server_handle.await {
                error!("Server task failed: {}", e);
            }

            info!("Aggregator shutdown complete");
        })
    }

    pub async fn shutdown(&self) {
        info!("Initiating aggregator shutdown");

        // Stop the task aggregator
        if let Some(task_agg) = &self.task_aggregator {
            if let Err(e) = task_agg.stop().await {
                error!("Error stopping task aggregator: {}", e);
            }
        }

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
            _ = server_rx => {
                info!("Server has been shut down via close handle");
            }
            _ = async {
                let (notify, is_shutdown) = &*shutdown;
                loop {
                    notify.notified().await;
                    if *is_shutdown.lock().await {
                        break;
                    }
                }
            } => {
                info!("Shutdown signal received, stopping server");
                close_handle.close();
            }
        }

        Ok(())
    }

    pub async fn process_signed_task_response(
        &mut self,
        resp: SignedTaskResponse,
    ) -> Result<(), Error> {
        // Convert the SignedTaskResponse to GenericSignedTaskResponse
        let generic_signed_response = GenericSignedTaskResponse {
            response: resp.task_response,
            signature: resp.signature,
            operator_id: resp.operator_id,
        };

        // Process the signed response using the generic task aggregator
        if let Some(task_agg) = &self.task_aggregator {
            task_agg
                .process_signed_response(generic_signed_response)
                .await;
            Ok(())
        } else {
            Err(Error::Context(
                "Task aggregator not initialized".to_string(),
            ))
        }
    }

    // Register a task with the aggregator
    pub async fn register_task(&self, task_index: TaskIndex, task: Task) -> Result<(), Error> {
        if let Some(task_agg) = &self.task_aggregator {
            // Create an indexed task with the task index
            let indexed_task = IndexedTask::new(task, task_index);

            // Register the task with the generic task aggregator
            task_agg
                .register_task(indexed_task)
                .await
                .map_err(|e| Error::Context(e.to_string()))
        } else {
            Err(Error::Context(
                "Task aggregator not initialized".to_string(),
            ))
        }
    }
}

impl BackgroundService for AggregatorContext {
    async fn start(&self) -> Result<oneshot::Receiver<Result<(), RunnerError>>, RunnerError> {
        let (tx, rx) = oneshot::channel();
        let ctx = self.clone();
        tokio::spawn(async move {
            let _ = ctx.start().await;
            let _ = tx.send(Ok(()));
        });
        Ok(rx)
    }
}
