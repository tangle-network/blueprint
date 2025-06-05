use axum::{
    Router,
    extract::State,
    http::StatusCode,
    response::Json,
    response::{IntoResponse, Response},
    routing::get,
};
use blueprint_core::{error, info};
use prometheus::{Registry, TextEncoder};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::oneshot;

use crate::error::{Error, Result};

/// Prometheus metrics server state
#[derive(Clone)]
struct ServerState {
    registry: Arc<Registry>,
}

/// Prometheus metrics server
pub struct PrometheusServer {
    registry: Arc<Registry>,
    bind_address: String,
    shutdown_tx: Option<oneshot::Sender<()>>,
}

impl PrometheusServer {
    /// Create a new Prometheus metrics server
    #[must_use]
    pub fn new(registry: Arc<Registry>, bind_address: String) -> Self {
        Self {
            registry,
            bind_address,
            shutdown_tx: None,
        }
    }

    /// Start the Prometheus metrics server
    ///
    /// # Errors
    /// Returns an error if the bind address cannot be parsed or the server fails to start
    ///
    /// # Panics
    /// Panics if the TCP listener cannot be bound to the address or if the server encounters an error
    #[allow(clippy::unused_async)]
    pub async fn start(&mut self) -> Result<()> {
        let addr: SocketAddr = self
            .bind_address
            .parse()
            .map_err(|e| Error::Other(format!("Failed to parse bind address: {}", e)))?;

        let state = ServerState {
            registry: self.registry.clone(),
        };

        let app = Router::new()
            .route("/metrics", get(metrics_handler))
            .route("/health", get(health_handler))
            .route("/api/v1/query", get(api_v1_query_handler))
            .route("/api/v1/labels", get(api_v1_labels_handler))
            .route("/api/v1/metadata", get(api_v1_metadata_handler))
            .with_state(state);

        let (tx, rx) = oneshot::channel();
        self.shutdown_tx = Some(tx);

        info!("Starting Prometheus metrics server on {}", addr);

        let listener = match tokio::net::TcpListener::bind(addr).await {
            Ok(l) => l,
            Err(e) => {
                error!(
                    "Failed to bind Prometheus metrics server to {}: {}",
                    addr, e
                );
                return Err(Error::Other(format!(
                    "Failed to bind Prometheus server to {}: {}",
                    addr, e
                )));
            }
        };

        tokio::spawn(async move {
            axum::serve(listener, app)
                .with_graceful_shutdown(async {
                    rx.await.ok();
                })
                .await
                .unwrap_or_else(|e| {
                    // This unwrap is for the server's execution, not bind
                    error!("Prometheus metrics server execution error: {}", e);
                });
        });

        Ok(())
    }

    /// Stop the Prometheus metrics server
    pub fn stop(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
            info!("Stopped Prometheus metrics server");
        }
    }
}

impl Drop for PrometheusServer {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Handler for /metrics endpoint
///
/// Returns the current metrics in Prometheus text format
async fn metrics_handler(State(state): State<ServerState>) -> Response {
    let encoder = TextEncoder::new();
    let metric_families = state.registry.gather();

    match encoder.encode_to_string(&metric_families) {
        Ok(metrics) => (StatusCode::OK, metrics).into_response(),
        Err(e) => {
            error!("Failed to encode metrics: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to encode metrics",
            )
                .into_response()
        }
    }
}

/// Handler for /health endpoint
///
/// Returns a simple OK response to indicate the server is running
async fn health_handler() -> Response {
    (StatusCode::OK, "OK").into_response()
}

/// Handler for /api/v1/query endpoint (minimal for Grafana health check)
///
/// Returns a static successful response to mimic Prometheus API for simple queries.
async fn api_v1_query_handler() -> (StatusCode, Json<serde_json::Value>) {
    let response_body = serde_json::json!({
        "status": "success",
        "data": {
            "resultType": "scalar",
            "result": [
                0, // Placeholder timestamp
                "1" // Placeholder value
            ]
        }
    });
    (StatusCode::OK, Json(response_body))
}

/// Handler for /api/v1/labels endpoint
///
/// Returns an empty list of labels, conforming to Prometheus API.
async fn api_v1_labels_handler() -> (StatusCode, Json<serde_json::Value>) {
    let response_body = serde_json::json!({
        "status": "success",
        "data": []
    });
    (StatusCode::OK, Json(response_body))
}

/// Handler for /api/v1/metadata endpoint
///
/// Returns an empty map of metadata, conforming to Prometheus API.
async fn api_v1_metadata_handler() -> (StatusCode, Json<serde_json::Value>) {
    let response_body = serde_json::json!({
        "status": "success",
        "data": {}
    });
    (StatusCode::OK, Json(response_body))
}
