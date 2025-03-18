use super::TaskError;
use crate::contexts::aggregator::EigenSquareContext;
use crate::contexts::task::IncredibleSquaringTask;
use crate::contracts::task_manager::SquaringTask::NewTaskCreated;
use blueprint_sdk::core::info;
use blueprint_sdk::eigensdk::services_blsaggregation::bls_agg::TaskMetadata;
use blueprint_sdk::eigensdk::types::operator::QuorumThresholdPercentage;
use blueprint_sdk::evm::extract::Events;
use blueprint_sdk::extract::Context;
use blueprint_sdk::job_result::Void;

const TASK_CHALLENGE_WINDOW_BLOCK: u32 = 100;
const BLOCK_TIME_SECONDS: u32 = 12;

pub async fn initialize_bls_task(
    Context(ctx): Context<EigenSquareContext>,
    Events(ev): Events<NewTaskCreated>,
) -> Result<Void, TaskError> {
    info!("Initializing task for BLS aggregation");

    let aggregator = if let Some(aggregator) = &ctx.task_aggregator {
        aggregator
    } else {
        return Err(TaskError::TaskAggregatorNotInitialized);
    };

    for ev in ev {
        let task_index = ev.taskIndex;
        let task_created_block = ev.task.taskCreatedBlock;
        let task = ev.task;
        let mut tasks = aggregator.tasks.write().await;
        tasks.insert(
            task_index,
            IncredibleSquaringTask {
                task_index,
                contract_task: task.clone(),
            },
        );
        let time_to_expiry = std::time::Duration::from_secs(
            (TASK_CHALLENGE_WINDOW_BLOCK * BLOCK_TIME_SECONDS).into(),
        );

        let quorum_threshold_percentage =
            vec![QuorumThresholdPercentage::try_from(task.quorumThresholdPercentage).unwrap()];

        let task_metadata = TaskMetadata::new(
            task_index,
            task_created_block as u64,
            task.quorumNumbers.0.to_vec(),
            quorum_threshold_percentage,
            time_to_expiry,
        );

        aggregator
            .bls_service
            .initialize_task(task_metadata)
            .await?;
    }

    Ok(Void)
}
