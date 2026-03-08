# examples

## Purpose
Provides a reference HTTP FaaS server implementing the Custom FaaS Platform Specification for local testing of the HTTP FaaS executor without requiring cloud credentials.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `reference_faas_server.rs` - A fully functional warp-based HTTP server (port 8080) that implements deploy, invoke, health check, get deployment info, undeploy, and warm endpoints. Stores function binaries on disk under `/tmp/blueprint-faas-test/functions/`, extracts a `bootstrap` executable from uploaded zip files, and executes it with JSON stdin/stdout for invocations.

## Key APIs (no snippets)
- `PUT /api/functions/{id}` -- deploys a function from a zip binary with optional base64-encoded config in `x-blueprint-config` header.
- `POST /api/functions/{id}/invoke` -- invokes a deployed function with `InvokeRequest` JSON (job_id, args).
- `GET /api/functions/{id}/health` -- returns health status and invocation count.
- `GET /api/functions/{id}` -- returns deployment metadata.
- `DELETE /api/functions/{id}` -- removes a deployed function.
- `POST /api/functions/{id}/warm` -- pre-warms function instances.

## Relationships
- Requires the `custom` feature flag: `cargo run --example reference_faas_server --features custom`.
- Serves as the test backend for integration tests in `crates/blueprint-faas/tests/`.
- Implements the same REST API that `HttpFaasExecutor` in `crates/blueprint-faas/` targets.
