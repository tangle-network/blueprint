//! Subscribe to job events via SSE from a Rust client.
//!
//! ```sh
//! cargo run --example consume_sse -- sse http://operator:8080 job-123
//! cargo run --example consume_sse -- webhook 9090 my-secret
//! ```
//!
//! Dependencies (add to your Cargo.toml):
//! ```toml
//! reqwest = { version = "0.12", features = ["stream"] }
//! reqwest-eventsource = "0.6"
//! tokio = { version = "1", features = ["full"] }
//! serde = { version = "1", features = ["derive"] }
//! serde_json = "1"
//! hmac = "0.12"
//! sha2 = "0.10"
//! hex = "0.4"
//! axum = "0.8"
//! ```

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct JobEvent {
    status: String,
    #[serde(default)]
    progress: Option<u8>,
    #[serde(default)]
    result: Option<serde_json::Value>,
    #[serde(default)]
    error: Option<String>,
    timestamp: u64,
}

// ── SSE Consumer ────────────────────────────────────────────────────────

/// Watch a job's SSE stream until terminal status.
///
/// Uses reqwest-eventsource for robust SSE handling with auto-reconnect.
///
/// ```rust,ignore
/// use reqwest_eventsource::{Event, EventSource};
/// use futures::StreamExt;
///
/// async fn watch_job(base_url: &str, job_id: &str) -> Result<JobEvent, Box<dyn std::error::Error>> {
///     let url = format!("{base_url}/v1/jobs/{job_id}/events");
///     let mut es = EventSource::get(&url);
///
///     while let Some(event) = es.next().await {
///         match event {
///             Ok(Event::Message(msg)) => {
///                 let event: JobEvent = serde_json::from_str(&msg.data)?;
///                 println!("[{}] {:?}", msg.event, event);
///
///                 match event.status.as_str() {
///                     "completed" | "failed" | "cancelled" => {
///                         es.close();
///                         return Ok(event);
///                     }
///                     _ => {}
///                 }
///             }
///             Ok(Event::Open) => println!("SSE connection opened"),
///             Err(e) => {
///                 eprintln!("SSE error: {e}");
///                 es.close();
///                 return Err(e.into());
///             }
///         }
///     }
///     Err("stream ended without terminal event".into())
/// }
/// ```

// ── Webhook Receiver ────────────────────────────────────────────────────

/// Verify HMAC-SHA256 signature from operator webhook delivery.
///
/// ```rust,ignore
/// use hmac::{Hmac, Mac};
/// use sha2::Sha256;
/// use subtle::ConstantTimeEq;
///
/// fn verify_signature(body: &[u8], signature: &str, secret: &str) -> bool {
///     let sig_hex = signature.strip_prefix("sha256=").unwrap_or(signature);
///     let Ok(sig_bytes) = hex::decode(sig_hex) else { return false };
///
///     let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
///     mac.update(body);
///     let expected = mac.finalize().into_bytes();
///     expected.as_slice().ct_eq(&sig_bytes).into()
/// }
/// ```

/// Minimal axum webhook receiver:
///
/// ```rust,ignore
/// use axum::{Router, routing::post, extract::State, http::HeaderMap, Json};
///
/// #[derive(Clone)]
/// struct AppState { secret: String }
///
/// async fn webhook_handler(
///     State(state): State<AppState>,
///     headers: HeaderMap,
///     body: axum::body::Bytes,
/// ) -> &'static str {
///     let sig = headers.get("x-webhook-signature")
///         .and_then(|v| v.to_str().ok())
///         .unwrap_or("");
///
///     if !verify_signature(&body, sig, &state.secret) {
///         return "invalid signature";
///     }
///
///     let payload: serde_json::Value = serde_json::from_slice(&body).unwrap();
///     let job_id = payload["job_id"].as_str().unwrap_or("?");
///     let status = payload["event"]["status"].as_str().unwrap_or("?");
///     println!("[webhook] job={job_id} status={status}");
///
///     if let Some(result) = payload["event"].get("result") {
///         println!("  result: {result}");
///     }
///     "ok"
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let app = Router::new()
///         .route("/webhook", post(webhook_handler))
///         .with_state(AppState { secret: "my-secret".into() });
///
///     let listener = tokio::net::TcpListener::bind("0.0.0.0:9090").await.unwrap();
///     axum::serve(listener, app).await.unwrap();
/// }
/// ```

fn main() {
    println!("This file contains documented examples in doc comments.");
    println!("See the code for SSE consumer and webhook receiver patterns.");
    println!();
    println!("Usage:");
    println!("  # SSE — watch a job's event stream");
    println!("  curl -N http://operator:8080/v1/jobs/job-123/events");
    println!();
    println!("  # Webhook — the operator POSTs to YOUR server:");
    println!("  # POST /webhook HTTP/1.1");
    println!("  # X-Webhook-Signature: sha256=<hex>");
    println!("  # X-Job-Id: job-123");
    println!("  # Content-Type: application/json");
    println!("  # {{\"job_id\":\"job-123\",\"event\":{{\"status\":\"completed\",...}}}}");
}
