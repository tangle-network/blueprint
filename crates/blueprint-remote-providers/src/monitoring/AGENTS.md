# monitoring

## Purpose
Health monitoring, log aggregation, machine type discovery, and Loki integration for remote provider deployments. Provides continuous health checks with auto-recovery, multi-source log streaming, and cloud instance type lookups with caching.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Re-exports from discovery, health, logs, and loki submodules.
- `discovery.rs` - `MachineTypeDiscovery` querying AWS/GCP/Azure/DO/Vultr APIs for available instance types. `MachineType` struct (name, vCPUs, memory, GPU, price). Caching layer with TTL. Fallback hardcoded catalogs when API calls fail. `find_best_match(spec)` for optimal instance selection. `CloudCredentials` loaded from env vars.
- `health.rs` - `HealthMonitor` running continuous health checks on deployed instances with configurable intervals. `HealthStatus` enum (Healthy, Degraded, Unhealthy, Unknown). `HealthCheckResult` with latency and details. Auto-recovery: restarts unhealthy containers or re-provisions instances. `ApplicationHealthChecker` for HTTP and TCP endpoint probes.
- `logs.rs` - `LogStreamer` for real-time log streaming from a single source. `LogAggregator` for merging logs from multiple sources. `LogEntry` with timestamp, level, source, message. `LogLevel` enum. `LogSource` enum: Docker, Kubernetes, SSH, CloudWatch, CloudLogging, File. Structured JSON and plain text log parsing.
- `loki.rs` - `LokiClient` for pushing log entries to and querying from Grafana Loki. `LogAggregationPipeline` with buffered batch flushing (configurable batch size and flush interval). `setup_local_loki()` helper to start a Loki Docker container for development.

## Key APIs
- `MachineTypeDiscovery::discover(provider) -> Vec<MachineType>` - list instance types with caching
- `MachineTypeDiscovery::find_best_match(provider, spec) -> MachineType` - optimal instance selection
- `HealthMonitor::start(instance, interval)` - begin continuous health monitoring with auto-recovery
- `ApplicationHealthChecker::check_http(url)` / `check_tcp(addr)` - endpoint health probes
- `LogAggregator::stream(sources) -> Stream<LogEntry>` - merged multi-source log stream
- `LokiClient::push(entries)` / `query(logql)` - Loki push and query
- `LogAggregationPipeline::new(loki_client, batch_size)` - buffered log pipeline

## Relationships
- `discovery` is used by `infra/provisioner` for machine type lookups during provisioning
- `health` monitors instances provisioned via `infra/` adapters
- `logs` streams from Docker, K8s, SSH, and cloud-native log sources
- `loki` integrates with Grafana Loki for centralized log storage; `LogAggregationPipeline` consumes `LogEntry` from `logs`
