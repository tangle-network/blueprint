# remote

## Purpose
Remote deployment integration layer for the blueprint manager. Provides serverless (FaaS) and traditional cloud deployment capabilities, including blueprint analysis for deployment strategy selection, metadata fetching from Tangle, cloud provider selection, deployment policy configuration, and operator pricing services.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Module declarations and re-exports for all submodules. Includes integration test module.
- `blueprint_analyzer.rs` - Pure functions for analyzing blueprint jobs against FaaS platform limits. `analyze_blueprint()` recommends `DeploymentStrategy` (Serverless, Hybrid, or Traditional) based on job profiling data (duration, memory, statefulness, persistent connections). Includes `FaasLimits` presets for AWS Lambda, GCP Functions, Azure Functions, and DigitalOcean Functions. `ResourceSizing` estimates CPU/memory from profiles.
- `blueprint_fetcher.rs` - `BlueprintMetadata` and `JobProfile` types. `fetch_blueprint_metadata()` queries Tangle chain for blueprint structure and loads profiling data from chain metadata (profilingData or description field) or filesystem (`target/blueprint-profiles.json`). Decodes base64+gzip compressed profiles. Converts `JobProfile` to `BenchmarkProfile` for pricing engine integration.
- `policy_loader.rs` - `DeploymentPolicy` and `ServerlessSettings` configuration loaded from `~/.config/tangle/deployment-policy.json`. Defines `FaasProviderDef` enum (AWS Lambda, GCP Functions, Azure Functions, Custom). Converts to runtime `ServerlessConfig`.
- `pricing_service.rs` - `OperatorPricingService` for calculating deployment costs. `calculate_quote()` fetches metadata, analyzes strategy, and computes costs using real pricing APIs via `FaasPricingFetcher` and `PricingFetcher` from `blueprint_pricing_engine_lib`. Returns `PricingQuote` with per-provider cost breakdowns.
- `provider_selector.rs` - `CloudProvider` enum and `ProviderSelector` with first-match strategy based on `ProviderPreferences` (GPU, CPU-intensive, memory-intensive, cost-optimized, TEE-capable provider lists). `ResourceSpec` defines deployment requirements. `DeploymentTarget` enum (CloudInstance, Kubernetes, Hybrid).
- `serverless.rs` - `deploy_serverless()` orchestrates FaaS deployment: deploys a lightweight orchestrator process and optionally auto-deploys jobs to FaaS platforms via `blueprint_faas::factory` (feature-gated). `ServerlessConfig` and `FaasProviderConfig` types.
- `service.rs` - `RemoteDeploymentService` (large) managing the full lifecycle of remote cloud deployments including resource estimation, provider selection, health monitoring, TTL management, and deployment tracking. `RemoteDeploymentPolicy` struct.
- `integration_test.rs` - Integration tests for provider selection and deployment service initialization.
- `PRICING_INTEGRATION.md` - Documentation on pricing engine integration.
- `SERVERLESS.md` - Documentation on serverless deployment architecture.

## Key APIs (no snippets)
- `analyze_blueprint(job_count, profiles, limits, serverless_enabled) -> BlueprintAnalysis` - pure deployment strategy analysis
- `fetch_blueprint_metadata(blueprint_id, rpc_url, binary_path) -> BlueprintMetadata` - chain + filesystem metadata fetcher
- `OperatorPricingService::calculate_quote(blueprint_id) -> PricingQuote` - end-to-end pricing calculation
- `ProviderSelector::select_target(requirements) -> DeploymentTarget` - cloud provider selection
- `deploy_serverless(ctx, ..., config) -> Service` - FaaS deployment orchestration
- `RemoteDeploymentService` - full remote deployment lifecycle manager

## Relationships
- Used by `executor/remote_provider_integration.rs` for cloud deployment on service initiation
- `blueprint_analyzer` and `blueprint_fetcher` feed into `pricing_service` for cost calculation
- `provider_selector` used by both `service.rs` and `executor/remote_provider_integration.rs`
- Depends on `blueprint_pricing_engine_lib`, `blueprint_remote_providers`, `blueprint_profiling`, `blueprint_faas`
- `serverless.rs` creates `Service` instances via `rt/service.rs`
