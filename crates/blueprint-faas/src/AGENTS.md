# src

## Purpose
Source directory for the `blueprint-faas` crate, providing FaaS (Function-as-a-Service) integrations for the Blueprint SDK. Defines a provider-agnostic `FaasExecutor` trait and implements it for multiple serverless platforms, allowing blueprint jobs to be deployed and invoked on cloud FaaS providers.

## Contents (one hop)
### Subdirectories
- [x] `aws/` - AWS Lambda executor implementation (`LambdaExecutor`). Feature-gated behind `aws`.
- [x] `azure/` - Azure Functions executor implementation (`AzureFunctionExecutor`). Feature-gated behind `azure`.
- [x] `custom/` - Custom HTTP-based FaaS executor (`HttpFaasExecutor`) for self-hosted platforms. Feature-gated behind `custom`.
- [x] `digitalocean/` - DigitalOcean Functions executor implementation (`DigitalOceanExecutor`). Feature-gated behind `digitalocean`.
- [x] `gcp/` - Google Cloud Functions executor implementation (`CloudFunctionExecutor`). Feature-gated behind `gcp`.

### Files
- `core.rs` - Core FaaS abstractions: `FaasExecutor` trait, `FaasConfig`, `FaasDeployment`, `FaasError`, `FaasPayload`/`FaasResponse` serializable types, `FaasRegistry` for job-to-executor mapping, and `DynFaasExecutor` type alias for runtime polymorphism
- `lib.rs` - Crate root; re-exports core types, declares feature-gated provider modules, defines the `factory` module for provider-agnostic executor creation, and a `utils` module for shared zip packaging and job ID extraction

## Key APIs
- `FaasExecutor` trait -- async trait with `invoke`, `deploy_job`, `health_check`, `warm`, `get_deployment`, `undeploy_job` methods
- `FaasRegistry` -- maps job IDs to `DynFaasExecutor` instances for runtime dispatch
- `FaasPayload` / `FaasResponse` -- serializable types bridging `JobCall`/`JobResult` to FaaS invocations
- `factory::create_executor(config)` -- creates a provider-specific executor from `FaasProviderConfig`
- `factory::deploy_job(config, job_id, binary)` -- one-shot deploy a job binary to a FaaS provider

## Relationships
- Depends on `blueprint-core` for `JobCall` and `JobResult` types
- Integrated with `blueprint-runner` via `BlueprintRunner::with_faas_executor()` to delegate job execution to FaaS platforms
- Each provider subdirectory implements the `FaasExecutor` trait for its respective cloud platform
