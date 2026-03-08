# prometheus

## Purpose
Prometheus metrics collection and HTTP server for the QoS system. Registers system, blueprint, and job metrics with a shared Prometheus registry and exposes them via an Axum HTTP server with Grafana-compatible API endpoints.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Module declarations and re-exports of `PrometheusCollector` and `PrometheusServer`.
- `collector.rs` - `PrometheusCollector` registers and manages Prometheus metric instruments against a shared `Registry`:
  - System metrics: `cpu_usage` (Gauge), `memory_usage`/`total_memory`/`disk_usage`/`total_disk` (IntGauge), `network_rx_bytes`/`network_tx_bytes` (IntCounter)
  - Blueprint metrics: `job_executions` (IntCounterVec by job/service/blueprint), `job_execution_time` (HistogramVec), `job_errors` (IntCounterVec with error_type)
  - Status metrics: `uptime`, `last_heartbeat`, `status_code` (IntGauge)
  - Update methods: `update_system_metrics()`, `update_blueprint_status()`, `record_job_execution()`, `record_job_error()`, async `add_custom_metric()`/`get_custom_metrics()`
- `server.rs` - `PrometheusServer` runs an Axum HTTP server with graceful shutdown:
  - `/metrics` -- gathers from registry (force-flushes OTel first via `EnhancedMetricsProvider`), encodes with `TextEncoder`
  - `/health` -- simple OK response
  - `/api/v1/query`, `/api/v1/labels`, `/api/v1/metadata`, `/api/v1/series`, `/api/v1/query_range` -- minimal Prometheus-compatible API stubs for Grafana datasource health checks

## Key APIs (no snippets)
- **Types**: `PrometheusCollector`, `PrometheusServer`
- **Functions**: `PrometheusCollector::new(config, registry)`, `.update_system_metrics()`, `.record_job_execution()`, `.record_job_error()`, `PrometheusServer::new(registry, enhanced_provider, bind_address)`, `.start()`, `.stop()`

## Relationships
- **Depends on**: `prometheus` crate (Registry, metric types, TextEncoder), `axum` (HTTP server), `EnhancedMetricsProvider` (OTel flush on scrape), `crate::metrics::types` (SystemMetrics, BlueprintStatus, MetricsConfig)
- **Used by**: `EnhancedMetricsProvider` creates and drives both collector and server; external Prometheus/Grafana instances scrape the `/metrics` endpoint
