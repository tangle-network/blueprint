//! The webhook gateway server.
//!
//! Runs an axum HTTP server as a [`BackgroundService`] within the Blueprint runner.
//! Each configured endpoint validates authentication and injects a [`JobCall`] into
//! the runner's producer stream.

use crate::auth;
use crate::config::WebhookConfig;
use crate::error::WebhookError;
use crate::producer::WebhookEvent;

use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use blueprint_runner::BackgroundService;
use blueprint_runner::error::RunnerError;
use bytes::Bytes;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::sync::{mpsc, oneshot};

/// Shared state for the axum handlers.
#[derive(Clone)]
struct GatewayState {
    config: Arc<WebhookConfig>,
    /// Channel to send verified webhook events to the producer.
    event_tx: mpsc::UnboundedSender<WebhookEvent>,
    /// Monotonic call ID counter.
    call_id_counter: Arc<AtomicU64>,
}

/// The webhook gateway.
///
/// Implements [`BackgroundService`] so it can be plugged into the Blueprint runner.
///
/// # Usage
///
/// ```rust,ignore
/// let (gateway, producer) = WebhookGateway::new(config)?;
///
/// BlueprintRunner::builder((), env)
///     .router(router)
///     .producer(producer)
///     .background_service(gateway)
///     .run()
///     .await?;
/// ```
pub struct WebhookGateway {
    config: Arc<WebhookConfig>,
    event_tx: mpsc::UnboundedSender<WebhookEvent>,
}

impl WebhookGateway {
    /// Create a new gateway and its paired [`WebhookProducer`](crate::WebhookProducer).
    pub fn new(config: WebhookConfig) -> Result<(Self, crate::WebhookProducer), WebhookError> {
        if config.endpoints.is_empty() {
            return Err(WebhookError::Config(
                "at least one webhook endpoint must be configured".into(),
            ));
        }

        let (producer, event_tx) = crate::WebhookProducer::channel();

        let gateway = Self {
            config: Arc::new(config),
            event_tx,
        };

        Ok((gateway, producer))
    }

    /// Build the axum router with all configured webhook endpoints.
    fn build_router(&self) -> Router {
        let state = GatewayState {
            config: self.config.clone(),
            event_tx: self.event_tx.clone(),
            call_id_counter: Arc::new(AtomicU64::new(1)),
        };

        let mut router = Router::new().route("/webhooks/health", axum::routing::get(health_check));

        // Register each configured endpoint
        for (idx, endpoint) in self.config.endpoints.iter().enumerate() {
            tracing::info!(
                path = %endpoint.path,
                job_id = endpoint.job_id,
                auth = %endpoint.auth,
                name = %endpoint.display_name(),
                "registering webhook endpoint"
            );

            // Each endpoint gets its own handler that captures its index
            let ep_state = EndpointState {
                index: idx,
                gateway: state.clone(),
            };

            router = router.route(&endpoint.path, post(handle_webhook).with_state(ep_state));
        }

        // The health endpoint uses GatewayState
        router.with_state(state)
    }
}

impl BackgroundService for WebhookGateway {
    async fn start(&self) -> Result<oneshot::Receiver<Result<(), RunnerError>>, RunnerError> {
        let (tx, rx) = oneshot::channel();
        let router = self.build_router();
        let addr = self.config.bind_address;

        tokio::spawn(async move {
            tracing::info!(%addr, "webhook gateway starting");

            let listener = match tokio::net::TcpListener::bind(addr).await {
                Ok(l) => l,
                Err(e) => {
                    let _ = tx.send(Err(RunnerError::Other(Box::new(e))));
                    return;
                }
            };

            tracing::info!(%addr, "webhook gateway listening");

            if let Err(e) = axum::serve(listener, router).await {
                tracing::error!(error = %e, "webhook gateway server error");
                let _ = tx.send(Err(RunnerError::Other(Box::new(WebhookError::Server(
                    e.to_string(),
                )))));
            }
        });

        Ok(rx)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Axum Handlers
// ═══════════════════════════════════════════════════════════════════════════

/// Per-endpoint state that captures the endpoint index.
#[derive(Clone)]
struct EndpointState {
    index: usize,
    gateway: GatewayState,
}

async fn health_check() -> &'static str {
    "ok"
}

/// Handle an incoming webhook request.
async fn handle_webhook(
    State(ep_state): State<EndpointState>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    let endpoint = match ep_state.gateway.config.endpoints.get(ep_state.index) {
        Some(ep) => ep,
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "endpoint configuration missing" })),
            )
                .into_response();
        }
    };

    // Verify authentication
    if let Err(e) = auth::verify(endpoint, &headers, &body) {
        tracing::warn!(
            path = %endpoint.path,
            error = %e,
            "webhook auth failed"
        );
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({ "error": "unauthorized" })),
        )
            .into_response();
    }

    let call_id = ep_state
        .gateway
        .call_id_counter
        .fetch_add(1, Ordering::Relaxed);

    let event = WebhookEvent {
        service_id: ep_state.gateway.config.service_id,
        job_id: endpoint.job_id,
        body,
        path: endpoint.path.clone(),
        name: endpoint.name.clone(),
        call_id,
    };

    tracing::info!(
        path = %endpoint.path,
        job_id = endpoint.job_id,
        call_id,
        name = %endpoint.display_name(),
        "webhook event accepted"
    );

    if ep_state.gateway.event_tx.send(event).is_err() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({ "error": "service shutting down" })),
        )
            .into_response();
    }

    (
        StatusCode::ACCEPTED,
        Json(serde_json::json!({
            "status": "accepted",
            "job_id": endpoint.job_id,
            "call_id": call_id,
        })),
    )
        .into_response()
}

// ═══════════════════════════════════════════════════════════════════════════
// Integration Tests
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::WebhookConfig;
    use axum::body::Body;
    use http::Request;
    use tower::util::ServiceExt;

    fn test_config() -> WebhookConfig {
        WebhookConfig {
            bind_address: "127.0.0.1:0".parse().unwrap(),
            endpoints: vec![
                crate::config::WebhookEndpoint {
                    path: "/hooks/open".into(),
                    job_id: 1,
                    auth: "none".into(),
                    secret: None,
                    api_key_header: None,
                    name: Some("Open Hook".into()),
                },
                crate::config::WebhookEndpoint {
                    path: "/hooks/secure".into(),
                    job_id: 2,
                    auth: "bearer".into(),
                    secret: Some("test-token".into()),
                    api_key_header: None,
                    name: None,
                },
            ],
            service_id: 42,
        }
    }

    #[tokio::test]
    async fn test_open_endpoint_accepts() {
        let (gateway, mut producer) = WebhookGateway::new(test_config()).unwrap();
        let router = gateway.build_router();

        let req = Request::builder()
            .method("POST")
            .uri("/hooks/open")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"action":"buy"}"#))
            .unwrap();

        let resp = router.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::ACCEPTED);

        // Verify the producer received the event
        use futures::StreamExt;
        let job = producer.next().await.unwrap().unwrap();
        assert_eq!(job.job_id(), blueprint_core::JobId::from(1u64));
    }

    #[tokio::test]
    async fn test_bearer_auth_accepted() {
        let (gateway, _producer) = WebhookGateway::new(test_config()).unwrap();
        let router = gateway.build_router();

        let req = Request::builder()
            .method("POST")
            .uri("/hooks/secure")
            .header("authorization", "Bearer test-token")
            .body(Body::from("payload"))
            .unwrap();

        let resp = router.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::ACCEPTED);
    }

    #[tokio::test]
    async fn test_bearer_auth_rejected() {
        let (gateway, _producer) = WebhookGateway::new(test_config()).unwrap();
        let router = gateway.build_router();

        let req = Request::builder()
            .method("POST")
            .uri("/hooks/secure")
            .header("authorization", "Bearer wrong-token")
            .body(Body::from("payload"))
            .unwrap();

        let resp = router.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_missing_auth_rejected() {
        let (gateway, _producer) = WebhookGateway::new(test_config()).unwrap();
        let router = gateway.build_router();

        let req = Request::builder()
            .method("POST")
            .uri("/hooks/secure")
            .body(Body::from("payload"))
            .unwrap();

        let resp = router.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_health_check() {
        let (gateway, _producer) = WebhookGateway::new(test_config()).unwrap();
        let router = gateway.build_router();

        let req = Request::builder()
            .method("GET")
            .uri("/webhooks/health")
            .body(Body::empty())
            .unwrap();

        let resp = router.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
