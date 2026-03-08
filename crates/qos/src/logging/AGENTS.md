# logging

## Purpose
Provides centralized logging integration with the Grafana observability stack, including Loki log aggregation with OpenTelemetry tracing support, and a Grafana HTTP API client for programmatic dashboard and datasource management.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Module root re-exporting `GrafanaClient`, `GrafanaConfig`, `LokiConfig`, `OtelConfig`, `init_loki_logging`, `init_otel_tracer`.
- `loki.rs` - Defines `LokiConfig` (URL, auth, labels, batch size, timeout, optional `OtelConfig`) and `OtelConfig` (max attributes per span). Provides `init_loki_logging` (sets up tracing-loki layer with background task), `init_loki_with_opentelemetry` (adds OpenTelemetry tracing layer on top), and `init_otel_tracer` (configures OpenTelemetry SDK tracer provider with service name and version resource attributes). Note: global subscriber setup is currently commented out (TODO).
- `grafana.rs` - `GrafanaConfig` with URL, API key or basic auth, org/folder settings, Loki and Prometheus datasource URLs. `GrafanaClient` wraps an HTTP client and provides: `create_dashboard` (create/update dashboards), `create_folder`, `create_blueprint_dashboard` (pre-configured dashboard with system metrics, job executions, logs, heartbeat, status, uptime, and custom test metrics panels), `check_datasource_health`, `get_datasource`, `create_or_update_datasource`. Also defines data models: `Dashboard`, `Panel`, `DataSource`, `GridPos`, `Target`, `FieldConfig`, `Thresholds`, etc. Supports API key or basic auth with warnings for default passwords.

## Key APIs (no snippets)
- `init_loki_logging` - Initializes Loki log forwarding with tracing-loki.
- `init_loki_with_opentelemetry` - Initializes Loki + OpenTelemetry combined observability.
- `init_otel_tracer` - Sets up OpenTelemetry tracer provider.
- `GrafanaClient::new` - Creates client with config-based authentication.
- `GrafanaClient::create_blueprint_dashboard` - Creates a comprehensive monitoring dashboard for a blueprint service.
- `GrafanaClient::create_or_update_datasource` - Manages Grafana datasources (Loki, Prometheus).

## Relationships
- Used by `crate::servers::grafana` and `crate::servers::loki` for server configuration types.
- Used by the QoS system to set up observability infrastructure for blueprint services.
- Depends on `tracing_loki`, `tracing_opentelemetry`, `opentelemetry_sdk`, `reqwest`.
