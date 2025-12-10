//! HTTP API endpoints for the aggregation service

use crate::service::AggregationService;
use crate::state::{TaskConfig, ThresholdType};
use crate::types::*;
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use std::collections::HashMap;
use std::sync::Arc;

/// Build the API router
pub fn router(service: Arc<AggregationService>) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/v1/stats", get(get_stats))
        .route("/v1/tasks/init", post(init_task))
        .route("/v1/tasks/submit", post(submit_signature))
        .route("/v1/tasks/status", post(get_status))
        .route("/v1/tasks/aggregate", post(get_aggregated))
        .route("/v1/tasks/mark-submitted", post(mark_submitted))
        .with_state(service)
}

/// Health check endpoint
async fn health() -> &'static str {
    "ok"
}

/// Initialize a new aggregation task
async fn init_task(
    State(service): State<Arc<AggregationService>>,
    Json(req): Json<InitTaskRequest>,
) -> impl IntoResponse {
    let (threshold_type, operator_stakes) = match req.threshold {
        ThresholdConfig::Count { required_signers } => {
            (ThresholdType::Count(required_signers), None)
        }
        ThresholdConfig::StakeWeighted {
            threshold_bps,
            operator_stakes,
        } => {
            let stakes = operator_stakes
                .into_iter()
                .map(|stake| (stake.operator_index, stake.stake))
                .collect::<HashMap<_, _>>();
            (ThresholdType::StakeWeighted(threshold_bps), Some(stakes))
        }
    };

    let config = TaskConfig {
        threshold_type,
        operator_stakes,
        ..Default::default()
    };

    match service.init_task_with_config(
        req.service_id,
        req.call_id,
        req.output,
        req.operator_count,
        config,
    ) {
        Ok(()) => Json(InitTaskResponse {
            success: true,
            error: None,
        }),
        Err(e) => Json(InitTaskResponse {
            success: false,
            error: Some(e.to_string()),
        }),
    }
}

/// Submit a signature for aggregation
async fn submit_signature(
    State(service): State<Arc<AggregationService>>,
    Json(req): Json<SubmitSignatureRequest>,
) -> impl IntoResponse {
    match service.submit_signature(req) {
        Ok(response) => (StatusCode::OK, Json(response)),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(SubmitSignatureResponse {
                accepted: false,
                signatures_collected: 0,
                threshold_required: 0,
                threshold_met: false,
                error: Some(e.to_string()),
            }),
        ),
    }
}

/// Get aggregation status for a task
async fn get_status(
    State(service): State<Arc<AggregationService>>,
    Json(req): Json<GetStatusRequest>,
) -> impl IntoResponse {
    let response = service.get_status(req.service_id, req.call_id);
    Json(response)
}

/// Get aggregated result if threshold is met
async fn get_aggregated(
    State(service): State<Arc<AggregationService>>,
    Json(req): Json<GetStatusRequest>,
) -> impl IntoResponse {
    match service.get_aggregated_result(req.service_id, req.call_id) {
        Some(result) => (StatusCode::OK, Json(Some(result))),
        None => (StatusCode::NOT_FOUND, Json(None)),
    }
}

/// Get service statistics
async fn get_stats(State(service): State<Arc<AggregationService>>) -> impl IntoResponse {
    Json(service.get_stats())
}

/// Mark a task as submitted to the chain
async fn mark_submitted(
    State(service): State<Arc<AggregationService>>,
    Json(req): Json<GetStatusRequest>,
) -> impl IntoResponse {
    match service.mark_submitted(req.service_id, req.call_id) {
        Ok(()) => (StatusCode::OK, Json(serde_json::json!({ "success": true }))),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "success": false, "error": e.to_string() })),
        ),
    }
}
