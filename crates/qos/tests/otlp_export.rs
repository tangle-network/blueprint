//! End-to-end proof that `init_telemetry` actually ships `tracing` spans as
//! OTLP/HTTP-JSON over the wire — serialization, the `/v1/traces` path, the
//! `Authorization: Bearer` header, `Content-Type: application/json`, and the
//! `service.name` resource attribute the Tangle Intelligence dashboard groups by.
//!
//! The collector is a one-shot local TCP server (a legitimate process-boundary
//! mock): it captures the exact bytes the exporter sends, which is the ground
//! truth no static review can give. This is the regression guard for the bug this
//! crate fixed — an `init_otel_tracer` that built an exporter-less provider and a
//! tracing subscriber that was never installed, so spans went nowhere.
//!
//! Runs in its own test binary so the global tracing subscriber / tracer provider
//! it installs don't collide with other suites. Export is driven through
//! `OtelConfig` (not process env) so the test is hermetic and deterministic.

use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::time::Duration;

use blueprint_qos::logging::loki::{LokiConfig, OtelConfig, init_telemetry};

/// Read a full HTTP/1.1 request (headers + Content-Length body) from a stream.
fn read_http_request(stream: &mut std::net::TcpStream) -> String {
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    let mut buf = Vec::new();
    let mut chunk = [0u8; 4096];
    loop {
        let n = match stream.read(&mut chunk) {
            Ok(0) => break,
            Ok(n) => n,
            Err(_) => break,
        };
        buf.extend_from_slice(&chunk[..n]);
        let text = String::from_utf8_lossy(&buf);
        if let Some(hdr_end) = text.find("\r\n\r\n") {
            let header = &text[..hdr_end];
            let content_len = header
                .lines()
                .find_map(|l| {
                    let (k, v) = l.split_once(':')?;
                    k.trim()
                        .eq_ignore_ascii_case("content-length")
                        .then(|| v.trim().parse::<usize>().ok())
                        .flatten()
                })
                .unwrap_or(0);
            let body_have = buf.len() - (hdr_end + 4);
            if body_have >= content_len {
                break;
            }
        }
    }
    String::from_utf8_lossy(&buf).into_owned()
}

#[test]
fn operator_exports_spans_as_otlp_http_json() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let (tx, rx) = mpsc::channel::<String>();

    // One-shot collector thread: capture the first POST, ack it, hand it back.
    let server = std::thread::spawn(move || {
        if let Ok((mut stream, _)) = listener.accept() {
            let req = read_http_request(&mut stream);
            // OTLP/HTTP-JSON success envelope the exporter expects.
            let body = br#"{"partialSuccess":{}}"#;
            let resp = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                body.len()
            );
            let _ = stream.write_all(resp.as_bytes());
            let _ = stream.write_all(body);
            let _ = stream.flush();
            let _ = tx.send(req);
        }
    });

    // Point the exporter at the mock collector and give it a tenant key — entirely
    // through config, so no process-global env is mutated.
    let mut loki_config = LokiConfig::default();
    let mut labels = std::collections::HashMap::new();
    labels.insert("service".to_string(), "blueprint-operator-test".to_string());
    loki_config.labels = labels;
    loki_config.otel_config = Some(OtelConfig {
        endpoint: Some(format!("http://127.0.0.1:{port}")),
        bearer_token: Some("sk-tan-proof-key".to_string()),
        ..Default::default()
    });

    {
        let _guard = init_telemetry(&loki_config, "blueprint-operator-test");
        // Emit a realistic job span; closing it queues the export.
        let span = tracing::info_span!("job.execute", job_id = 7u64, service_id = 42u64);
        span.in_scope(|| {
            tracing::info!(outcome = "ok", "job tick fired");
        });
        // Drop the guard → force_flush + shutdown → blocking OTLP POST fires.
    }

    let req = rx
        .recv_timeout(Duration::from_secs(10))
        .expect("exporter never POSTed to the collector — export path is broken");
    server.join().ok();

    let head = req.split("\r\n\r\n").next().unwrap().to_lowercase();
    // Request line + path: OTLP/HTTP traces endpoint.
    assert!(
        head.starts_with("post /v1/traces "),
        "expected POST to /v1/traces, got request line: {:?}",
        req.lines().next()
    );
    // Auth: the Tangle tenant bearer token.
    assert!(
        head.contains("authorization: bearer sk-tan-proof-key"),
        "missing/!= Authorization bearer header; headers:\n{head}"
    );
    // JSON wire (the Intelligence adapter rejects protobuf).
    assert!(
        head.contains("content-type: application/json"),
        "expected application/json content-type; headers:\n{head}"
    );
    // Body: OTLP envelope carrying our service identity + the span.
    let body = req.split("\r\n\r\n").nth(1).unwrap_or("");
    assert!(
        body.contains("resourceSpans"),
        "body not OTLP-shaped: {body}"
    );
    assert!(
        body.contains("service.name") && body.contains("blueprint-operator-test"),
        "service.name resource attribute missing from export body: {body}"
    );
    assert!(
        body.contains("job.execute"),
        "span name missing from export body: {body}"
    );
}
