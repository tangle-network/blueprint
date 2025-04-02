use std::ops::Deref;

use crate::contexts::combined::CombinedContext;
use crate::contracts::SquaringTask::NewTaskCreated;
use crate::error::TaskError;
use alloy_sol_types::SolEvent;
use blueprint_sdk::evm::extract::BlockEvents;
use blueprint_sdk::extract::Context;
use blueprint_sdk::{info, warn};
use eigensdk::services_blsaggregation::bls_agg::TaskMetadata;
// use eigensdk::types::avs::QuorumThresholdPercentage;

const TASK_CHALLENGE_WINDOW_BLOCK: u32 = 100;
const BLOCK_TIME_SECONDS: u32 = 12;
pub const INITIALIZE_TASK_JOB_ID: u32 = 1;

/// Initializes the task for the aggregator server
#[blueprint_sdk::macros::debug_job]
pub async fn initialize_bls_task(
    Context(ctx): Context<CombinedContext>,
    BlockEvents(events): BlockEvents,
) -> Result<(), TaskError> {
    let task_created_events = events.iter().filter_map(|log| {
        NewTaskCreated::decode_log(&log.inner, true)
            .map(|event| event.data)
            .ok()
    });

    for task_created in task_created_events {
        let task = task_created.task;
        let task_index = task_created.taskIndex;

        info!("Initializing task {} for BLS aggregation", task_index);

        if let Some(aggregator_ctx) = &ctx.aggregator_context {
            let task_aggregator =
                aggregator_ctx
                    .task_aggregator
                    .as_ref()
                    .ok_or(TaskError::Aggregation(
                        "Task aggregator not found".to_string(),
                    ))?;

            // Register the task with the task aggregator, passing both task_index and task
            aggregator_ctx
                .register_task(task_index, task.clone())
                .await
                .map_err(|e| TaskError::Aggregation(e.to_string()))?;

            info!(
                "Successfully registered task {} with the task aggregator",
                task_index
            );
        } else {
            warn!("Aggregator context not available, skipping task initialization");
        }
    }
    Ok(())
}
