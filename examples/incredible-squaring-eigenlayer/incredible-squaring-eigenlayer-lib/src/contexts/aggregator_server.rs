use crate::contexts::client::SignedTaskResponse;
use crate::contexts::incredible_task::{IncredibleTask, IncredibleTaskResponse, IncredibleResponseSender};
use eigenlayer_extra::generic_task_aggregation::{AggregationError, TaskAggregator};
use eigenlayer_extra::rpc_server::TaskAggregatorServer;
use std::sync::Arc;
use blueprint_sdk::{info};

/// Start the task aggregator server for Incredible Squaring Service
pub async fn start_server(
    aggregator: Arc<TaskAggregator<IncredibleTask, IncredibleTaskResponse, IncredibleResponseSender>>,
    address: String,
) -> Result<(), AggregationError> {
    info!("Starting Incredible Squaring Task aggregator server at {}", address);
    
    // Create a simple converter function from our client-specific SignedTaskResponse to the generic one
    let mut server = TaskAggregatorServer::new(
        aggregator,
        address,
        |response: SignedTaskResponse| response.to_generic(),
    )?;
    
    server.start().await?;
    info!("Incredible Squaring Task aggregator server started successfully");
    
    Ok(())
}

/// Helper function to create a task from the contract task event
pub fn create_incredible_task(
    task_index: u32,
    contract_task: crate::contracts::IIncredibleSquaringTaskManager::Task,
) -> IncredibleTask {
    IncredibleTask {
        task_index,
        contract_task,
    }
} 