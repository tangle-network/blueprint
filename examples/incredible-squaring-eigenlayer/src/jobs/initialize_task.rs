use crate::contexts::aggregator::AggregatorContext;
use crate::contracts::TaskManager::Task;
use blueprint_sdk::info;
use eigensdk::services_blsaggregation::bls_agg::TaskMetadata;
use eigensdk::types::operator::QuorumThresholdPercentage;
use std::convert::Infallible;

const TASK_CHALLENGE_WINDOW_BLOCK: u32 = 100;
const BLOCK_TIME_SECONDS: u32 = 12;

/// Initializes the task for the aggregator server
pub async fn initialize_bls_task(
    ctx: AggregatorContext,
    task: Task,
    task_index: u32,
) -> Result<u32, Infallible> {
    info!("Initializing task for BLS aggregation");

    let mut tasks = ctx.tasks.lock().await;
    tasks.insert(task_index, task.clone());
    let time_to_expiry =
        std::time::Duration::from_secs((TASK_CHALLENGE_WINDOW_BLOCK * BLOCK_TIME_SECONDS).into());

    let quorum_threshold_percentage =
        vec![QuorumThresholdPercentage::try_from(task.quorumThresholdPercentage).unwrap()];

    let task_metadata = TaskMetadata::new(
        task_index,
        task.taskCreatedBlock as u64,
        task.quorumNumbers.0.to_vec(),
        quorum_threshold_percentage,
        time_to_expiry,
    );

    if let Some(service) = &ctx.service_handle {
        service
            .lock()
            .await
            .initialize_task(task_metadata)
            .await
            .unwrap()
    }

    Ok(1)
}
