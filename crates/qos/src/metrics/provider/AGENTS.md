# provider

## Purpose
Metrics provider implementations for the QoS system. Offers a basic in-memory provider for standalone use and an enhanced provider that integrates Prometheus collection, OpenTelemetry export, and an embedded HTTP server.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Module declarations and re-exports of `DefaultMetricsProvider` and `EnhancedMetricsProvider`.
- `default.rs` - `DefaultMetricsProvider` implements the `MetricsProvider` trait with in-memory storage using `tokio::sync::RwLock`-wrapped `Vec`s and `HashMap`s. Collects system metrics (CPU, memory, disk, network via `sysinfo` crate) on a periodic background task. Stores metric history with configurable max size. Provides test helper methods for accessing internal Arc-wrapped state.
- `enhanced.rs` - `EnhancedMetricsProvider` is the full-featured production provider. Wraps `PrometheusCollector` and `OpenTelemetryExporter`, manages a shared `prometheus::Registry`, and optionally runs an embedded `PrometheusServer`. Implements `MetricsProvider` and `MetricsSource` (for heartbeat). On each collection cycle, updates both Prometheus metrics and OTel counters (e.g., `otel_job_executions_counter`). Supports `force_flush_otel_metrics()` for on-demand metric export.

## Key APIs (no snippets)
- **Types**: `DefaultMetricsProvider`, `EnhancedMetricsProvider`
- **Trait impls**: Both implement `MetricsProvider` (get/set system metrics, blueprint metrics, blueprint status, custom metrics, on-chain metrics, start_collection)
- **Functions**: `EnhancedMetricsProvider::new(metrics_config, otel_config)`, `.force_flush_otel_metrics()`, `.record_job_execution()`, `.record_job_error()`, `DefaultMetricsProvider::new(config)`

## Relationships
- **Depends on**: `crate::metrics::types` (`MetricsProvider` trait, `SystemMetrics`, `BlueprintMetrics`, `BlueprintStatus`, `MetricsConfig`), `crate::metrics::prometheus` (`PrometheusCollector`), `crate::metrics::opentelemetry` (`OpenTelemetryExporter`, `OpenTelemetryConfig`), `crate::heartbeat` (`MetricsSource`), `sysinfo`, `tokio`
- **Used by**: `PrometheusServer` holds an `Arc<EnhancedMetricsProvider>` for OTel flush; QoS system initialization creates the provider and starts collection
