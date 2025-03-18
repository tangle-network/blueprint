use crate::contracts::task_manager::SquaringTask::NewTaskCreated;
use blueprint_sdk::{
    alloy::{primitives::Address, sol_types::SolEvent},
    eigensdk::{
        crypto_bls,
        services_blsaggregation::bls_aggregation_service_error::BlsAggregationServiceError,
    },
    evm::filters::{contract::MatchesContract, event::MatchesEvent},
    Router,
};
use tower::filter::FilterLayer;

pub mod initialize_task;
pub mod x_square;

#[derive(Debug, thiserror::Error)]
pub enum TaskError {
    #[error(transparent)]
    SolType(#[from] blueprint_sdk::alloy::sol_types::Error),
    #[error(transparent)]
    BlsAggregationService(#[from] BlsAggregationServiceError),
    #[error("Aggregated response receiver closed")]
    AggregatedResponseReceiverClosed,
    #[error("Task aggregator not initialized")]
    TaskAggregatorNotInitialized,
    #[error("Keystore error: {0}")]
    KeystoreError(#[from] gadget_keystore::Error),
    #[error("BLS error: {0}")]
    BlsError(#[from] crypto_bls::error::BlsError),
}

/// Creates a router with task event filters
///
/// This router is used by the AVS operator to compute the x^2 of a task.
pub fn x_square_create_contract_router(
    ctx: x_square::IncredibleSquaringClientContext,
    contract_address: Address,
) -> Router {
    let sig = NewTaskCreated::SIGNATURE_HASH;
    Router::new()
        .route(*sig, x_square::compute_x_square)
        .with_context(ctx)
        .layer(FilterLayer::new(MatchesEvent(sig)))
        .layer(FilterLayer::new(MatchesContract(contract_address)))
}

/// Creates a router with task event filters
///
/// This router is used by the task / BLS signature aggregator to register tasks.
pub fn initialize_task_create_contract_router(
    ctx: crate::contexts::aggregator::EigenSquareContext,
    contract_address: Address,
) -> Router {
    let sig = NewTaskCreated::SIGNATURE_HASH;
    Router::new()
        .route(*sig, initialize_task::initialize_bls_task)
        .with_context(ctx)
        .layer(FilterLayer::new(MatchesEvent(sig)))
        .layer(FilterLayer::new(MatchesContract(contract_address)))
}
