# tests

## Purpose
Integration tests for the `blueprint-qos` crate, validating QoS metrics collection, gRPC service endpoints, blueprint integration with the runner, and resource monitoring in end-to-end scenarios using Anvil testnets.

## Contents (one hop)
### Subdirectories
- [x] `config/` - Test configuration files: `prometheus.yml` for Prometheus scrape targets used in integration tests

### Files
- `blueprint_integration_test.rs` - End-to-end integration test that deploys a blueprint on an Anvil testnet with QoS services (heartbeat, metrics, gRPC), exercises the `QosMetricsClient` for status/resource/blueprint queries, and validates the full QoS lifecycle
- `default_metrics_provider_tests.rs` - Tests for the default metrics provider implementation
- `qos_metrics_demo_test.rs` - Demonstration test for QoS metrics collection
- `utils.rs` - Shared test utilities: `XSQUARE_JOB_ID` constant, `square` job handler, `MockHeartbeatConsumer` for recording heartbeats, gRPC client helpers

## Key APIs (no snippets)
- `square` - Minimal job handler used across QoS integration tests
- `MockHeartbeatConsumer` - Records heartbeat status for test assertions
- Test helper functions for gRPC client setup

## Relationships
- Depends on `blueprint-qos` (the crate under test)
- Depends on `blueprint-anvil-testing-utils` for `BlueprintHarness` and `SeededTangleTestnet`
- Depends on `blueprint-tangle-extra` for `TangleArg`/`TangleResult` extractors and `TangleLayer`
- Uses tonic gRPC clients to validate the `QosMetricsService` endpoints
