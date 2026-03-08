# metrics

## Purpose
Metrics collection, processing, and exposure subsystem for the QoS crate. Defines the metrics data model (system metrics, blueprint metrics, status), the `MetricsProvider` trait, ABI encoding for on-chain metric submission, and the `MetricsService` orchestrator that wires together Prometheus and OpenTelemetry providers.

## Contents (one hop)
### Subdirectories
- [x] `opentelemetry/` - OpenTelemetry configuration and metric exporter implementation.
- [x] `prometheus/` - Prometheus metric collector and embedded HTTP server for the `/metrics` endpoint.
- [x] `provider/` - Default and enhanced `MetricsProvider` implementations that combine Prometheus and OpenTelemetry.

### Files
- `abi.rs` - Solidity ABI encoding for on-chain metric pairs. Defines a `MetricPair` sol struct (name: string, value: uint256) and `encode_metric_pairs` that ABI-encodes a `Vec<(String, u64)>` into bytes compatible with `abi.decode(data, (MetricPair[]))`.
- `service.rs` - `MetricsService` wraps an `EnhancedMetricsProvider` in an Arc. Constructors: `new` (default OTel config) and `with_otel_config` (custom). Methods: `record_job_execution`, `record_job_error`, `provider`, `get_otel_job_executions_counter`. Also provides `run_metrics_server` async function that creates a provider and starts background collection.
- `types.rs` - Core types: `MetricsConfig` (prometheus server config, collection interval, max history, service/blueprint IDs), `SystemMetrics` (CPU, memory, disk, network, timestamp), `BlueprintMetrics` (custom string metrics, timestamp), `BlueprintStatus` (service state, uptime, heartbeat). `MetricsProvider` trait with async methods for getting/setting metrics, managing on-chain metrics, and starting collection.

## Key APIs (no snippets)
- `MetricsProvider` trait - Core interface for metric collection and retrieval (system, blueprint, status, on-chain metrics).
- `MetricsService::new` / `with_otel_config` - Create metrics service with default or custom OpenTelemetry config.
- `MetricsService::record_job_execution` / `record_job_error` - Record job metrics to both Prometheus and OpenTelemetry.
- `run_metrics_server` - Standalone function to start a metrics collection server.
- `encode_metric_pairs` - ABI-encode metrics for on-chain submission via heartbeat.
- `MetricsConfig` - Configuration for collection interval, history, and Prometheus server.

## Relationships
- `service.rs` depends on `opentelemetry/` for `OpenTelemetryConfig` and `provider/` for `EnhancedMetricsProvider`.
- `types.rs` depends on `crate::servers::prometheus::PrometheusServerConfig`.
- Used by `crate::servers::prometheus` for the embedded Prometheus server.
- The `MetricsProvider` trait is implemented in `provider/` subdirectory.
