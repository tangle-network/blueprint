# src

## Purpose
Implementation of the `blueprint-qos` crate, providing quality-of-service infrastructure for blueprint services including metrics collection, heartbeat monitoring, log aggregation (Loki/Grafana), and managed observability server lifecycle (Prometheus, Loki, Grafana via Docker).

## Contents (one hop)
### Subdirectories
- [x] `logging/` - Log aggregation clients: `GrafanaClient` for dashboard provisioning and `LokiConfig` for structured log shipping to Loki
- [x] `metrics/` - Metrics collection subsystem with `opentelemetry/`, `prometheus/`, and `provider/` subdirectories; defines `MetricsProvider` trait, ABI types, and metric service implementations
- [x] `servers/` - Docker-managed observability server lifecycle: `PrometheusServer`, `LokiServer`, `GrafanaServer` with common container management utilities; each server implements `ServerManager` trait

### Files
- `lib.rs` - Crate root; declares modules, defines `QoSConfig` struct (heartbeat, metrics, Loki, Grafana, server configs, Docker network settings), provides `default_qos_config()`, re-exports key types and `QoSService`/`QoSServiceBuilder`
- `service.rs` - `QosMetricsService<T>` gRPC service implementation exposing metrics via tonic; endpoints for status, resource usage, blueprint metrics, and historical data
- `service_builder.rs` - `QoSServiceBuilder` for fluent construction of the QoS subsystem with optional components
- `unified_service.rs` - `QoSService` that orchestrates all QoS components (heartbeat, metrics, servers) as a unified lifecycle
- `heartbeat.rs` - `HeartbeatConfig` and `HeartbeatConsumer` trait for periodic liveness checks
- `metrics.rs` - Top-level metrics re-exports and `MetricsConfig`
- `remote.rs` - Remote metrics reporting
- `error.rs` - QoS error types

## Key APIs (no snippets)
- `QoSConfig` - Central configuration struct for all QoS components
- `QoSService` - Unified service orchestrating heartbeat, metrics, and server management
- `QoSServiceBuilder` - Builder pattern for constructing QoS services
- `QosMetricsService` - gRPC service implementing the `QosMetrics` protobuf interface
- `MetricsProvider` trait - Abstraction for supplying metric data
- `HeartbeatConsumer` trait - Interface for receiving heartbeat status updates
- `default_qos_config()` - Sensible defaults for all QoS components

## Relationships
- Depends on protobuf-generated types from `qos.proto` (built via `build.rs`)
- Used by `blueprint-runner` for metrics server integration (`MetricsServerAdapter`)
- Used by `blueprint-core-testing-utils` for test QoS setup
- Servers module manages Docker containers for Prometheus, Loki, and Grafana
