use crate::contexts::client::SignedTaskResponse;
use crate::contexts::client::{create_client, IncredibleSquaringAggregatorClient};
use blueprint_eigenlayer_extra::generic_task_aggregation::{
    AggregationError, AggregatorConfig, BlsAggServiceInMemory,
    SignedTaskResponse as GenericSignedTaskResponse, TaskAggregator,
};
use blueprint_eigenlayer_extra::rpc_server::TaskAggregatorServer;
use blueprint_sdk::info;
use blueprint_sdk::macros::context::KeystoreContext;
use blueprint_sdk::runner::config::BlueprintEnvironment;
use std::sync::Arc;
use task::{
    IncredibleSquaringResponseSender, IncredibleSquaringTask, IncredibleSquaringTaskResponse,
};

pub mod client;
pub mod task;

/// Start the task aggregator server for Incredible Squaring Service
pub async fn start_server(
    aggregator: Arc<
        TaskAggregator<
            IncredibleSquaringTask,
            IncredibleSquaringTaskResponse,
            IncredibleSquaringResponseSender,
        >,
    >,
    address: String,
) -> Result<(), AggregationError> {
    info!(
        "Starting Incredible Squaring Task aggregator server at {}",
        address
    );

    // Create a simple converter function from our client-specific SignedTaskResponse to the generic one
    let mut server =
        TaskAggregatorServer::new(aggregator, address, |response: SignedTaskResponse| {
            GenericSignedTaskResponse::new(
                response.task_response.clone(),
                response.signature.clone(),
                response.operator_id,
            )
        })?;

    server.start().await?;
    info!("Incredible Squaring Task aggregator server started successfully");

    Ok(())
}

#[derive(Clone)]
pub struct EigenSquareContext {
    /// Client for sending responses to the aggregator
    pub client: IncredibleSquaringAggregatorClient,

    /// Configuration for the context
    pub config: BlueprintEnvironment,

    /// The generic task aggregator for handling task responses
    pub task_aggregator: Option<
        Arc<
            TaskAggregator<
                IncredibleSquaringTask,
                IncredibleSquaringTaskResponse,
                IncredibleSquaringResponseSender,
            >,
        >,
    >,
}

impl EigenSquareContext {
    /// Initialize the task aggregator with the provided configuration and start the RPC server
    pub async fn initialize_and_start_aggregator(
        &mut self,
        bls_service: BlsAggServiceInMemory,
        response_sender: IncredibleSquaringResponseSender,
        config: AggregatorConfig,
        server_address: String,
    ) -> Result<(), AggregationError> {
        // Create and store the aggregator
        let aggregator = TaskAggregator::new(bls_service, response_sender, config);
        let aggregator = Arc::new(aggregator);

        // Start the aggregator
        aggregator.start().await?;

        // Start the RPC server
        start_server(Arc::clone(&aggregator), server_address).await?;

        // Store the aggregator in the context
        self.task_aggregator = Some(aggregator);

        Ok(())
    }

    /// Register a new task with the aggregator
    pub async fn register_task(
        &self,
        task: IncredibleSquaringTask,
    ) -> Result<(), AggregationError> {
        if let Some(aggregator) = &self.task_aggregator {
            aggregator.register_task(task).await?;
        }
        Ok(())
    }

    /// Stop the task aggregator processing
    pub async fn stop_task_aggregator(&self) -> Result<(), AggregationError> {
        if let Some(aggregator) = &self.task_aggregator {
            aggregator.stop().await?;
        }
        Ok(())
    }
}
