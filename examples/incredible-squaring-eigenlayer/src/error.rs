use std::net::AddrParseError;

use eigensdk::{
    services_blsaggregation::bls_aggregation_service_error::BlsAggregationServiceError,
    types::operator::OperatorTypesError,
};

#[expect(clippy::large_enum_variant, reason = "SDK error is large currently")]
#[derive(Debug, thiserror::Error)]
pub enum TaskError {
    #[error("Blueprint SDK: {0}")]
    BlueprintSDK(#[from] blueprint_sdk::Error),
    #[error(transparent)]
    SolType(#[from] alloy_sol_types::Error),
    #[error(transparent)]
    BlsAggregationService(#[from] BlsAggregationServiceError),
    #[error("Aggregation: {0}")]
    Aggregation(String),
    #[error(transparent)]
    OperatorTypesError(#[from] OperatorTypesError),
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
