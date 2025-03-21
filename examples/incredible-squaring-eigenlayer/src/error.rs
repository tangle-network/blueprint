use std::net::AddrParseError;

use eigensdk::services_blsaggregation::bls_aggregation_service_error::BlsAggregationServiceError;

#[derive(Debug, Clone, thiserror::Error)]
pub enum TaskError {
    #[error(transparent)]
    SolType(#[from] alloy_sol_types::Error),
    #[error(transparent)]
    BlsAggregationService(#[from] BlsAggregationServiceError),
    #[error("Aggregated response receiver closed")]
    AggregatedResponseReceiverClosed,
    #[error("Context: {0}")]
    Context(String),
    #[error(transparent)]
    Parse(#[from] AddrParseError),
    #[error("Runtime: {0}")]
    Runtime(String),
    #[error("Chain: {0}")]
    Chain(String),
    #[error("Task: {0}")]
    Task(String),
}
