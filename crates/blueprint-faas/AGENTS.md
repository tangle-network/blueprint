# blueprint-faas

## Purpose
Crate `blueprint-faas`: FaaS (Function-as-a-Service) provider integrations for the Blueprint SDK. Defines the `FaasExecutor` trait and provides implementations for deploying and invoking Blueprint jobs on serverless platforms: AWS Lambda, GCP Cloud Functions, Azure Functions, DigitalOcean Functions, and custom HTTP-based endpoints. Includes a factory module for provider-agnostic executor creation.

## Contents (one hop)
### Subdirectories
- [x] `examples/` - Reference FaaS server implementation (`reference_faas_server.rs`).
- [x] `src/` - Core FaaS abstractions (`core.rs`), provider-specific modules (`aws/`, `gcp/`, `azure/`, `custom/`, `digitalocean/`), factory module, and shared utilities for zip packaging and job ID extraction.
- [x] `tests/` - Integration tests: DigitalOcean integration, HTTP executor basics, reference server integration, and registry tests.

### Files
- `Cargo.toml` - Crate manifest (`blueprint-faas`). Deps: `blueprint-core`, `tokio`, `serde`/`serde_json`, `zip`, optional cloud SDKs (`aws-sdk-lambda`, `gcp_auth`, `azure_core`/`azure_identity`, `reqwest`). Features: `aws`, `gcp`, `azure`, `custom`, `digitalocean`, `all`.
- `README.md` - Crate documentation.

## Key APIs (no snippets)
- `FaasExecutor` trait -- async interface for deploying, invoking, and removing FaaS jobs.
- `DynFaasExecutor` -- `Arc<dyn FaasExecutor>` type alias for dynamic dispatch.
- `FaasConfig` -- configuration for memory limits, timeouts, and environment variables.
- `FaasDeployment` -- deployment metadata (function name, region, endpoint).
- `FaasPayload` / `FaasResponse` -- request/response types for job invocation.
- `FaasRegistry` -- registry for managing multiple FaaS executors by job ID.
- `FaasMetrics` -- execution metrics (duration, memory, invocation count).
- `factory::create_executor(config)` -- creates a provider-specific executor from `FaasProviderConfig`.
- `factory::deploy_job(config, job_id, binary)` -- deploys a job binary using provider configuration.
- Provider implementations: `aws::LambdaExecutor`, `gcp::CloudFunctionExecutor`, `azure::AzureFunctionExecutor`, `custom::HttpFaasExecutor`, `digitalocean::DigitalOceanExecutor`.

## Relationships
- Depends on `blueprint-core` for core types.
- Used by `blueprint-runner` to offload job execution to serverless platforms.
- Each provider feature gates its cloud SDK dependencies to keep builds lean.
