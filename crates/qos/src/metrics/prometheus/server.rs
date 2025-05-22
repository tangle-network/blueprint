use axum::{
    Router,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use prometheus::{Registry, TextEncoder};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::oneshot;
use tracing::{error, info};

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
    #[must_use] pub fn new(registry: Registry, bind_address: String) -> Self {
        Self {
            registry: Arc::new(registry),
            bind_address,
            shutdown_tx: None,
        }
    }

    /// Start the Prometheus metrics server
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
            .with_state(state);

        let (tx, rx) = oneshot::channel();
        self.shutdown_tx = Some(tx);

        info!("Starting Prometheus metrics server on {}", addr);

        tokio::spawn(async move {
            let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
            axum::serve(listener, app)
                .with_graceful_shutdown(async {
                    rx.await.ok();
                })
                .await
                .unwrap_or_else(|e| {
                    error!("Prometheus metrics server error: {}", e);
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
async fn health_handler() -> Response {
    (StatusCode::OK, "OK").into_response()
}
