//! SSE (Server-Sent Events) endpoint for real-time job status streaming.
//!
//! Provides an axum router that blueprints mount to serve job events:
//!
//! ```text
//! GET /v1/jobs/:job_id/events
//! Accept: text/event-stream
//!
//! data: {"status":"queued","timestamp":"..."}
//!
//! data: {"status":"processing","progress":42,"timestamp":"..."}
//!
//! data: {"status":"completed","result":{"video_url":"..."},"timestamp":"..."}
//! ```

use crate::notifier::JobNotifier;
use axum::Router;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::routing::get;
use std::sync::Arc;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::BroadcastStream;

/// Build an axum router for SSE job event streaming.
///
/// Mount this on your blueprint's HTTP server:
///
/// ```rust,ignore
/// let notifier = Arc::new(JobNotifier::new(config));
/// let sse_router = blueprint_webhooks::sse::router(notifier.clone());
///
/// let app = Router::new()
///     .merge(your_api_routes)
///     .merge(sse_router);
/// ```
pub fn router(notifier: Arc<JobNotifier>) -> Router {
    Router::new()
        .route("/v1/jobs/{job_id}/events", get(sse_handler))
        .with_state(notifier)
}

/// Extract and validate the bearer token from the Authorization header.
fn extract_bearer_token(headers: &HeaderMap) -> Option<&str> {
    headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
}

async fn sse_handler(
    State(notifier): State<Arc<JobNotifier>>,
    Path(job_id): Path<String>,
    headers: HeaderMap,
) -> impl IntoResponse {
    // H1: Require bearer token authentication
    let token = match extract_bearer_token(&headers) {
        Some(t) => t.to_string(),
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                "missing or malformed Authorization: Bearer <token> header",
            )
                .into_response();
        }
    };

    if !notifier.validate_job_token(&job_id, &token).await {
        return (StatusCode::FORBIDDEN, "invalid token for this job").into_response();
    }

    let rx = match notifier.subscribe(&job_id).await {
        Some(rx) => rx,
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                "SSE channel capacity exceeded",
            )
                .into_response();
        }
    };

    let stream = BroadcastStream::new(rx).filter_map(|result| match result {
        Ok(event) => {
            let data = serde_json::to_string(&event)
                .unwrap_or_else(|_| r#"{"error":"serialize"}"#.to_string());
            let sse_event = Event::default().event(event.status.to_string()).data(data);
            Some(Ok::<_, std::convert::Infallible>(sse_event))
        }
        Err(tokio_stream::wrappers::errors::BroadcastStreamRecvError::Lagged(n)) => {
            tracing::warn!(skipped = n, "SSE subscriber lagged, sending error event");
            let sse_event = Event::default()
                .event("error")
                .data(format!(r#"{{"error":"lagged","skipped":{n}}}"#));
            Some(Ok(sse_event))
        }
    });

    Sse::new(stream)
        .keep_alive(
            KeepAlive::new()
                .interval(std::time::Duration::from_secs(15))
                .event(Event::default().comment("ping")),
        )
        .into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::notifier::{JobEvent, JobStatus, NotifierConfig};
    use axum::body::Body;
    use http::Request;
    use tower::ServiceExt;

    fn test_notifier() -> Arc<JobNotifier> {
        Arc::new(JobNotifier::new(NotifierConfig {
            signing_secret: "test".into(),
            max_retries: 0,
            ..Default::default()
        }))
    }

    #[tokio::test]
    async fn sse_endpoint_returns_event_stream() {
        let notifier = test_notifier();
        let app = router(notifier.clone());

        // Register the job and get a token
        let token = notifier.register_job("test-job").await;

        // Subscribe first so the channel exists
        let _rx = notifier.subscribe("test-job").await;

        // Send an event
        notifier
            .notify(
                "test-job",
                JobEvent {
                    status: JobStatus::Queued,
                    ..Default::default()
                },
                None,
            )
            .await
            .unwrap();

        let req = Request::get("/v1/jobs/test-job/events")
            .header("Accept", "text/event-stream")
            .header("Authorization", format!("Bearer {token}"))
            .body(Body::empty())
            .unwrap();

        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), 200);

        let content_type = resp
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        assert!(
            content_type.contains("text/event-stream"),
            "expected text/event-stream, got {content_type}"
        );
    }

    #[tokio::test]
    async fn sse_endpoint_404_path_format() {
        let notifier = test_notifier();
        let app = router(notifier);

        // Wrong path should 404
        let req = Request::get("/v1/jobs/").body(Body::empty()).unwrap();

        let resp = app.oneshot(req).await.unwrap();
        // axum returns 404 for unmatched routes
        assert_eq!(resp.status(), 404);
    }

    #[tokio::test]
    async fn sse_auth_rejects_missing_token() {
        let notifier = test_notifier();
        let app = router(notifier.clone());
        notifier.register_job("auth-job").await;

        let req = Request::get("/v1/jobs/auth-job/events")
            .body(Body::empty())
            .unwrap();

        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn sse_auth_rejects_invalid_token() {
        let notifier = test_notifier();
        let app = router(notifier.clone());
        notifier.register_job("auth-job2").await;

        let req = Request::get("/v1/jobs/auth-job2/events")
            .header("Authorization", "Bearer totally-wrong-token")
            .body(Body::empty())
            .unwrap();

        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn sse_auth_rejects_unregistered_job() {
        let notifier = test_notifier();
        let app = router(notifier.clone());

        // Don't register the job — any token should be rejected
        let req = Request::get("/v1/jobs/unknown-job/events")
            .header("Authorization", "Bearer some-token")
            .body(Body::empty())
            .unwrap();

        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    }
}
