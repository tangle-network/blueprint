# tests

## Purpose
Integration and unit tests for the blueprint-faas crate, covering executor creation, the FaaS registry, payload round-tripping, and cloud-provider-specific deployment flows.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `digitalocean_integration.rs` - End-to-end test for `DigitalOceanExecutor`: deploys a function, runs a health check, and undeploys. Gated behind the `digitalocean` feature and `#[ignore]` (requires `DIGITALOCEAN_TOKEN`).
- `http_executor_basic.rs` - Unit tests for `HttpFaasExecutor` creation, custom endpoint configuration, and `FaasPayload`/`FaasResponse` round-trip verification. Gated behind the `custom` feature.
- `reference_server_integration.rs` - Tests for `HttpFaasExecutor` construction and endpoint wiring against the reference server API shape. Gated behind the `custom` feature.
- `registry_test.rs` - Tests for `FaasRegistry`: verifies empty state, executor registration and lookup, provider name accessors, and feature-gated type visibility for `HttpFaasExecutor` and `LambdaExecutor`.

## Key APIs (no snippets)
- Tests exercise `FaasExecutor` trait methods: `deploy_job`, `invoke`, `health_check`, `get_deployment`, `undeploy_job`, `provider_name`.
- Tests exercise `FaasRegistry` methods: `new`, `register`, `is_faas_job`, `get`, `job_ids`.
- Tests verify `FaasPayload`/`FaasResponse` conversion round-trips with `JobCall` and `JobResult`.

## Relationships
- Depends on `blueprint-faas` crate and its feature-gated provider modules (`custom`, `digitalocean`, `aws`).
- Depends on `blueprint-core` for `JobCall` and `JobResult` types.
- The reference server integration tests pair with the example in `crates/blueprint-faas/examples/reference_faas_server.rs`.
