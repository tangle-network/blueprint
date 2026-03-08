# config

## Purpose
Static configuration assets for the QoS observability stack: a Grafana dashboard template (embedded at compile-time) and a Loki server configuration (loaded at runtime for Docker container mounting).

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `grafana_dashboard.json` - Pre-built Grafana dashboard template with job execution metrics panel querying `sum(rate(otel_job_executions_total[$__rate_interval]))`.
  - **Key items**: 5-minute time window, stat panel, title injected at runtime per blueprint name
  - **Interactions**: Embedded via `include_str!()` in `unified_service.rs`, deserialized as `Dashboard`, sent to Grafana API
- `loki-config.yaml` - Loki server configuration for log aggregation in dev/test mode.
  - **Key items**: auth disabled, port 3100, filesystem storage at `/loki/chunks`, BoltDB index, 7-day retention, WAL enabled
  - **Interactions**: Read from disk in `LokiServer::start()`, mounted into Docker container at `/etc/loki/config.yaml`

## Key APIs (no snippets)
- No direct Rust APIs; files are static assets consumed by:
  - `include_str!()` (dashboard, compile-time)
  - Docker volume mount (Loki config, runtime)

## Relationships
- **Depends on**: Grafana dashboard JSON schema, Loki YAML config schema
- **Used by**: `src/servers/unified_service.rs` (dashboard embed), `src/servers/loki.rs` (config mount), `tests/qos_metrics_demo_test.rs`

## Files (detailed)

### `grafana_dashboard.json`
- **Role**: Zero-config observability dashboard template for blueprint metrics.
- **Key items**: single stat panel, `otel_job_executions_total` metric, browser timezone
- **Knobs / invariants**: Title updated dynamically; schema v1 (Grafana 7+)

### `loki-config.yaml`
- **Role**: Development-profile Loki server config (no auth, local storage).
- **Key items**: v11 schema, in-memory ring, filesystem chunks, BoltDB index, 7-day retention
- **Knobs / invariants**: Production needs TLS, auth, persistent volumes; path overridable before container startup

## End-to-end flow
1. `QoSConfig` enables `manage_servers: true`
2. Loki config read from disk -> mounted into Docker -> container launched on port 3100
3. Dashboard deserialized from embedded JSON -> title customized -> provisioned to Grafana
4. Metrics scraped to Prometheus, logs sent to Loki, visualized in Grafana

## Notes
- Compile-time embed (dashboard) vs runtime file (Loki config) for flexibility
- Development profile only: auth disabled, local storage
- Serial test requirement due to Docker resource conflicts
