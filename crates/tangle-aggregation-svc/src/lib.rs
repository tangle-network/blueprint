//! Tangle BLS Signature Aggregation Service
//!
//! A simple HTTP service that collects BLS signatures from operators,
//! aggregates them once threshold is met, and provides the aggregated
//! result for submission to the Tangle contract.
//!
//! ## Architecture
//!
//! ```text
//! Operator 1 ──┐
//! Operator 2 ──┼──▶ Aggregation Service ──▶ Aggregated Result
//! Operator 3 ──┘         │
//!                        ▼
//!                  Tangle Contract
//! ```
//!
//! ## Usage
//!
//! ### Starting the service
//!
//! ```rust,ignore
//! use blueprint_tangle_aggregation_svc::{AggregationService, api, ServiceConfig};
//! use std::sync::Arc;
//!
//! #[tokio::main]
//! async fn main() {
//!     let service = Arc::new(AggregationService::new(ServiceConfig::default()));
//!     let app = api::router(service);
//!
//!     let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
//!     axum::serve(listener, app).await.unwrap();
//! }
//! ```
//!
//! ### API Endpoints
//!
//! - `POST /v1/tasks/init` - Initialize an aggregation task
//! - `POST /v1/tasks/submit` - Submit a signature
//! - `POST /v1/tasks/status` - Get task status
//! - `POST /v1/tasks/aggregate` - Get aggregated result
//!
//! ### Operator Flow
//!
//! 1. Someone (often the first operator) initializes the task
//! 2. Each operator signs the output and submits their signature
//! 3. Once threshold is met, anyone can fetch the aggregated result
//! 4. Submit the aggregated result to the Tangle contract

pub mod api;
#[cfg(feature = "client")]
pub mod client;
pub mod persistence;
pub mod service;
pub mod state;
pub mod types;

#[cfg(feature = "client")]
pub use client::{AggregationServiceClient, ClientError};
pub use service::{
    AggregationService, CleanupWorkerHandle, ServiceConfig, ServiceError, ServiceStats,
    create_signing_message,
};
pub use persistence::{
    FilePersistence, NoPersistence, PersistedTaskState, PersistedThresholdType,
    PersistenceBackend, PersistenceError,
};
pub use state::{
    AggregationState, OperatorInfo, TaskConfig, TaskCounts, TaskForAggregation, TaskState,
    TaskStatus, ThresholdType,
};
pub use types::*;

/// Run the aggregation service
pub async fn run(addr: &str, config: ServiceConfig) -> Result<(), Box<dyn std::error::Error>> {
    use std::sync::Arc;
    use tracing::info;

    let service = Arc::new(AggregationService::new(config));
    let app = api::router(service);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("Aggregation service listening on {}", addr);

    axum::serve(listener, app).await?;
    Ok(())
}
