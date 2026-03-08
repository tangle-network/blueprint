# proto

## Purpose
Defines the gRPC service contract for exposing QoS metrics via `qos.proto`. Enables remote monitoring of blueprint status, resource usage, custom metrics, and historical data.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `qos.proto` - Protobuf service definition (184 lines) for the `QosMetrics` gRPC service with 4 RPCs.
  - **Key items**: `GetStatus` (blueprint/service status, uptime, heartbeat), `GetResourceUsage` (CPU %, memory, disk, network I/O), `GetBlueprintMetrics` (custom key-value metrics), `GetHistoricalMetrics` (time-series with SYSTEM/BLUEPRINT type filter)
  - **Interactions**: Compiled by `build.rs` via `tonic-build` -> `$OUT_DIR/qos.rs`; re-exported in `lib.rs` as `pub mod proto`

## Key APIs (no snippets)
- **Service**: `QosMetrics` with 4 RPCs: `GetStatus`, `GetResourceUsage`, `GetBlueprintMetrics`, `GetHistoricalMetrics`
- **Messages**: Request/response pairs for each RPC, `SystemMetrics`, `BlueprintMetrics` with Unix timestamps
- **Enums**: `MetricsType` (SYSTEM=0, BLUEPRINT=1)

## Relationships
- **Depends on**: `tonic-build` (compilation), `prost` (codegen)
- **Used by**: `src/service.rs` (`QosMetricsService<T>` implements proto trait), `src/remote.rs` (`RemoteMetricsProvider` uses proto client), `tests/blueprint_integration_test.rs`
- **Data/control flow**:
  - `build.rs` compiles proto with `--experimental_allow_proto3_optional`
  - `QoSServiceBuilder` creates service, binds gRPC endpoint (port 9615)
  - `RemoteMetricsProvider` creates client to collect metrics from deployed blueprints

## Files (detailed)

### `qos.proto`
- **Role**: Complete gRPC API definition for QoS metrics collection and querying.
- **Key items**: `QosMetrics` service, request filtering by `blueprint_id` + `service_id`, custom metrics as untyped key-value map, optional fields for time ranges
- **Knobs / invariants**: Proto3 optional fields for backward compatibility; port 9615 by convention; blueprint/service ID filtering prevents cross-blueprint metric leakage

## End-to-end flow
1. `build.rs` compiles `qos.proto` via `tonic-build`
2. `QoSServiceBuilder` creates `QosMetricsService` wrapper, binds gRPC on port 9615
3. RPC requests routed to `MetricsProvider` trait methods
4. `RemoteMetricsProvider` connects as client to collect metrics from remote blueprints
5. Integration tests spawn local server, verify RPC roundtrips

## Notes
- Proto3 optional required for backward-compatible optional fields
- Security: blueprint_id + service_id filtering per-request
- Custom metrics use map<string, string> for extensibility
