# qos

## Purpose
Quality of Service module (`blueprint-qos`). Provides heartbeat, metrics, and logging integrations for operator observability. Can manage local Docker-based observability stacks (Grafana, Loki, Prometheus) or connect to external monitoring infrastructure. Includes on-chain heartbeat submission and gRPC health reporting.

## Contents (one hop)
### Subdirectories
- [x] `config/` - Static configuration files: `grafana_dashboard.json` (pre-built Grafana dashboard), `loki-config.yaml` (Loki server configuration)
- [x] `proto/` - Protobuf service definition (`qos.proto`) for gRPC health/metrics API
- [x] `src/` - Crate source: `heartbeat.rs` (heartbeat service and config), `metrics/` (metrics collection with OpenTelemetry and Prometheus sub-modules, provider abstraction), `logging/` (Loki and Grafana integrations), `servers/` (managed Docker containers for Grafana, Loki, Prometheus), `service.rs` / `service_builder.rs` / `unified_service.rs` (QoS service composition), `remote.rs` (remote QoS endpoints), `error.rs`
- [x] `tests/` - Integration tests: blueprint integration, default metrics provider, QoS metrics demo; test configs in `config/` subdirectory

### Files
- `CHANGELOG.md` - Release history
- `Cargo.toml` - Crate manifest (`blueprint-qos`); depends on `blueprint-core`, `blueprint-crypto`, `blueprint-keystore`, `blueprint-std`, `bollard` (Docker API), `sysinfo`, OpenTelemetry stack (`opentelemetry`, `opentelemetry-prometheus`, `opentelemetry_sdk`), `tracing-loki`, `tracing-opentelemetry`, `axum`, `tonic`/`prost` for gRPC, `prometheus`, Alloy EVM stack for on-chain heartbeats; optional `blueprint-remote-providers`
- `README.md` - Overview of heartbeat, metrics, logging integrations, managed server configs
- `build.rs` - tonic-build protobuf compilation

## Key APIs (no snippets)
- `QoSService` / `QoSServiceBuilder` -- unified service that composes heartbeat, metrics, and logging
- `QoSConfig` / `default_qos_config()` -- configuration struct with optional heartbeat, metrics, Loki, Grafana, and managed server settings
- `HeartbeatConfig` -- heartbeat interval and submission configuration
- `MetricsConfig` -- OpenTelemetry/Prometheus metrics configuration
- `LokiConfig` / `GrafanaConfig` / `GrafanaClient` -- logging backend configuration
- `GrafanaServerConfig` / `LokiServerConfig` / `PrometheusServerConfig` -- managed Docker container server configs
- `proto` module -- generated gRPC service types from `qos.proto`

## Relationships
- Depends on `blueprint-core`, `blueprint-crypto`, `blueprint-keystore`, `blueprint-std`
- Optionally consumed by `blueprint-manager` (via `qos` feature)
- Optional `remote` feature adds `blueprint-remote-providers` for cloud QoS
- Uses `bollard` for Docker container management of observability servers
- Uses Alloy/Tangle contracts for on-chain heartbeat submission
- Requires serial test execution due to Docker/port resource conflicts
