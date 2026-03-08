# servers

## Purpose
Docker-based and embedded server lifecycle management for the QoS observability stack. Provides the `ServerManager` trait and implementations for Grafana, Loki, and Prometheus servers. Includes a generic Docker container manager for image pulling, container creation/start/stop/removal, network management, and health checking.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Module root declaring submodules and defining the `ServerManager` trait with async methods: `start` (with optional Docker network and bind IP), `stop`, `url`, `is_running`, `wait_until_ready`.
- `common.rs` - `DockerManager` wrapping the `bollard` Docker client. Provides: `ensure_image` (pull if missing), `run_container` (create and start with env vars, port bindings, volumes, health checks, extra hosts, bind IP), `stop_and_remove_container`, `create_network`, `connect_to_network`, `is_container_running`, `dump_container_logs`, `wait_for_container_health` (polls health status with timeout, dumps logs on failure). Validates volume paths to prevent directory traversal.
- `grafana.rs` - `GrafanaServerConfig` (port, admin credentials, anonymous access, container name, Loki config, health check timeout) and `GrafanaServer` implementing `ServerManager`. Starts Grafana 10.4.3 Docker container with configurable env vars, waits for both container health and API responsiveness (`/api/health`). Provides `client_config` for creating a `GrafanaClient`.
- `loki.rs` - `LokiServerConfig` (port, data dir, container name, config path) and `LokiServer` implementing `ServerManager`. Starts `grafana/loki:latest` Docker container with temp data directory (world-writable) and optional config file mount. Health check via `wget` to `/ready`. Provides `client_config` for creating a `LokiConfig`.
- `prometheus.rs` - `PrometheusServerConfig` (port, host, Docker vs embedded mode, image, container name, config/data paths) and `PrometheusServer` implementing `ServerManager`. Supports two modes: Docker container with config/data volume mounts and health check, or embedded in-process HTTP server using `PrometheusMetricsServer` with a Prometheus registry. Handles container reuse, network connection, and graceful shutdown.

## Key APIs (no snippets)
- `ServerManager` trait - Async interface: `start`, `stop`, `url`, `is_running`, `wait_until_ready`.
- `DockerManager::run_container` - Creates and starts a Docker container with full configuration.
- `DockerManager::wait_for_container_health` - Polls container health with timeout and log dumping.
- `GrafanaServer::client_config` - Returns `GrafanaConfig` for API client creation.
- `LokiServer::client_config` - Returns `LokiConfig` for log shipping.
- `PrometheusServer::new` - Creates server with registry and metrics provider for embedded mode.

## Relationships
- `grafana.rs` uses `crate::logging::{GrafanaConfig, LokiConfig}` for client configuration types.
- `loki.rs` uses `crate::logging::LokiConfig` for client configuration.
- `prometheus.rs` uses `crate::metrics::EnhancedMetricsProvider` and `crate::metrics::prometheus::server::PrometheusServer` for embedded mode.
- All server implementations use `common::DockerManager` for Docker operations.
- Depends on `bollard` for Docker API, `reqwest` for health check HTTP requests.
