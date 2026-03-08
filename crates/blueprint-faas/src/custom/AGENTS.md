# custom

## Purpose
Generic HTTP-based FaaS executor for arbitrary serverless or self-hosted runtimes. Invokes functions via configurable HTTP endpoints without managing deployment lifecycle.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - `HttpFaasExecutor` struct implementing `FaasExecutor`. Supports a base URL with optional per-job endpoint overrides. Invokes via HTTP POST, health-checks via GET to the job endpoint. `deploy_job` and `undeploy_job` are no-ops (returns `Ok`) since deployment is managed externally.

## Key APIs
- `HttpFaasExecutor::new(base_url)` - constructor with base URL for function invocations
- `HttpFaasExecutor::with_job_endpoint(job_id, url)` - register a custom endpoint for a specific job
- `FaasExecutor::invoke(job_id, input)` - POST JSON payload to the job's endpoint
- `FaasExecutor::health_check(job_id)` - GET request to verify endpoint availability
- `FaasExecutor::deploy_job` / `undeploy_job` - no-op implementations

## Relationships
- Implements `FaasExecutor` trait defined in the parent `blueprint-faas` crate
- Designed for self-hosted or non-standard serverless platforms (OpenFaaS, Knative, custom HTTP services)
- Peer to `aws/`, `azure/`, `gcp/`, and `digitalocean/` executor implementations
