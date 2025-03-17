use crate::contexts::client::AggregatorClient;
use crate::contexts::incredible_task::{IncredibleTask, IncredibleTaskResponse, IncredibleResponseSender};
use crate::contexts::aggregator_server;
use blueprint_sdk::macros::context::KeystoreContext;
use blueprint_sdk::runner::config::BlueprintEnvironment;
use eigenlayer_extra::generic_task_aggregation::{TaskAggregator, AggregatorConfig, AggregationError};
use std::sync::Arc;

#[derive(Clone, KeystoreContext)]
pub struct EigenSquareContext {
    /// Client for sending responses to the aggregator
    pub client: AggregatorClient,
    
    /// Configuration for the context
    #[config]
    pub std_config: BlueprintEnvironment,
    
    /// The generic task aggregator for handling task responses
    pub task_aggregator: Option<Arc<TaskAggregator<IncredibleTask, IncredibleTaskResponse, IncredibleResponseSender>>>,
}

impl EigenSquareContext {
    /// Initialize the task aggregator with the provided configuration and start the RPC server
    pub async fn initialize_and_start_aggregator(
        &mut self, 
        bls_service: eigenlayer_extra::generic_task_aggregation::BlsAggServiceInMemory, 
        response_sender: IncredibleResponseSender,
        config: AggregatorConfig,
        server_address: String,
    ) -> Result<(), AggregationError> {
        // Create and store the aggregator
        let aggregator = TaskAggregator::new(bls_service, response_sender, config);
        let aggregator = Arc::new(aggregator);
        
        // Start the aggregator
        aggregator.start().await?;
        
        // Start the RPC server
        aggregator_server::start_server(Arc::clone(&aggregator), server_address).await?;
        
        // Store the aggregator in the context
        self.task_aggregator = Some(aggregator);
        
        Ok(())
    }
    
    /// Register a new task with the aggregator
    pub async fn register_task(&self, task: IncredibleTask) -> Result<(), AggregationError> {
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
