use crate::IIncredibleSquaringTaskManager::Task;
use crate::{
    TaskError, FirstEvent, Context,
    contexts::aggregator::AggregatorContext, IncredibleSquaringTaskManager,
    INCREDIBLE_SQUARING_TASK_MANAGER_ABI_STRING,
};
use blueprint_sdk::info;
use eigensdk::services_blsaggregation::bls_agg::TaskMetadata;
use eigensdk::types::operator::QuorumThresholdPercentage;
use std::convert::Infallible;

const TASK_CHALLENGE_WINDOW_BLOCK: u32 = 100;
const BLOCK_TIME_SECONDS: u32 = 12;

pub async fn initialize_bls_task(
    Context(ctx): Context<AggregatorContext>,
    FirstEvent(ev): FirstEvent<NewTaskCreated>,
) -> Result<Tx, TaskError> {
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

/// Converts the event to inputs.
///
/// Uses a tuple to represent the return type because
/// the macro will index all values in the #[job] function
/// and parse the return type by the index.
pub async fn convert_event_to_inputs(
    event: (
        IncredibleSquaringTaskManager::NewTaskCreated,
        blueprint_sdk::alloy::rpc::types::Log,
    ),
) -> Result<Option<(Task, u32)>, ProcessorError> {
    let task_index = event.0.taskIndex;
    Ok(Some((event.0.task, task_index)))
}
