# opentelemetry

## Purpose
OpenTelemetry metrics pipeline integration for the QoS system. Sets up an `SdkMeterProvider` backed by a Prometheus exporter, allowing OTel-instrumented metrics to be scraped via the shared Prometheus registry.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Module declaration and re-exports of `OpenTelemetryConfig` and `OpenTelemetryExporter`.
- `exporter.rs` - Pipeline setup and metric instrument factory.
  - `OpenTelemetryConfig` (serializable): service name, version, instance ID, namespace for OTel resource attributes.
  - `OpenTelemetryExporter`: creates a `PrometheusExporter` backed by a shared `prometheus::Registry`, wraps it in `ArcPrometheusReader` (adapter implementing `MetricReader`), builds an `SdkMeterProvider`, and sets it as the global meter provider.
  - Convenience factory methods: `create_counter()`, `create_counter_with_attributes()`, `create_histogram()`, `create_gauge()` (observable), `create_up_down_counter()`.
  - `force_flush()` delegates to the SDK meter provider.

## Key APIs (no snippets)
- **Types**: `OpenTelemetryConfig`, `OpenTelemetryExporter`
- **Functions**: `OpenTelemetryExporter::new(config, shared_registry)`, `.meter()`, `.meter_provider()`, `.force_flush()`, `.create_counter()`, `.create_histogram()`, `.create_gauge()`, `.create_up_down_counter()`

## Relationships
- **Depends on**: `opentelemetry` (global meter provider), `opentelemetry_sdk` (`SdkMeterProvider`, `MetricReader`), `opentelemetry_prometheus` (`PrometheusExporter`), `prometheus` (`Registry`)
- **Used by**: `EnhancedMetricsProvider` (in `provider/enhanced.rs`) creates and owns the exporter; Prometheus server flushes OTel metrics before scraping
