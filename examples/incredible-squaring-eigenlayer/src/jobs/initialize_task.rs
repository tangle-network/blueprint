use crate::contexts::combined::CombinedContext;
use crate::contracts::SquaringTask::NewTaskCreated;
use crate::error::TaskError;
use alloy_sol_types::SolEvent;
use blueprint_sdk::evm::extract::BlockEvents;
use blueprint_sdk::extract::Context;
use blueprint_sdk::{info, warn};
use eigensdk::services_blsaggregation::bls_agg::TaskMetadata;
use eigensdk::types::operator::QuorumThresholdPercentage;

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
            let mut tasks = aggregator_ctx.tasks.lock().await;
            tasks.insert(task_index, task.clone());
            let time_to_expiry = std::time::Duration::from_secs(
                (TASK_CHALLENGE_WINDOW_BLOCK * BLOCK_TIME_SECONDS).into(),
            );

            let quorum_threshold_percentage =
                vec![QuorumThresholdPercentage::try_from(task.quorumThresholdPercentage).unwrap()];

            let task_metadata = TaskMetadata::new(
                task_index,
                task.taskCreatedBlock as u64,
                task.quorumNumbers.0.to_vec(),
                quorum_threshold_percentage,
                time_to_expiry,
            );

            if let Some(service) = &aggregator_ctx.service_handle {
                service
                    .lock()
                    .await
                    .initialize_task(task_metadata)
                    .await
                    .unwrap()
            }
        } else {
            warn!("Aggregator context not available, skipping task initialization");
        }
    }
    Ok(())
}
