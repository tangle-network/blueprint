# Blueprint FaaS

Function-as-a-Service (FaaS) execution support for Blueprint SDK.

## Overview

This crate provides trait-based integration with serverless platforms, allowing blueprints to delegate specific jobs to FaaS providers while running others locally.

### Supported Providers

- **AWS Lambda** (`aws` feature) - Full implementation with deployment
- **GCP Cloud Functions** (`gcp` feature) - Full implementation with Cloud Functions v2 API
- **Azure Functions** (`azure` feature) - Full implementation with ARM API and ZipDeploy
- **DigitalOcean Functions** (`digitalocean` feature) - Full implementation with namespace management
- **Custom HTTP** (`custom` feature) - HTTP-based integration for any platform (see [Custom FaaS Spec](../../docs/custom-faas-platform-spec.md))

## Architecture

### Core Design

The FaaS integration uses a trait-based design that keeps `BlueprintRunner` agnostic of specific providers:

```rust
#[async_trait]
pub trait FaasExecutor: Send + Sync {
    async fn invoke(&self, job_call: JobCall) -> Result<JobResult, FaasError>;
    async fn deploy_job(&self, job_id: u32, binary: &[u8], config: &FaasConfig)
        -> Result<FaasDeployment, FaasError>;
    async fn health_check(&self, job_id: u32) -> Result<bool, FaasError>;
    // ... more methods
}
```

### Delegation Model

Jobs are delegated at runtime based on registration:

1. Developer registers which jobs use FaaS via `.with_faas_executor(job_id, executor)`
2. BlueprintRunner checks `FaasRegistry` when jobs arrive
3. Matching jobs are delegated to FaaS, others run locally

## Usage

### Basic Example

```rust
use blueprint_faas::aws::LambdaExecutor;
use blueprint_runner::BlueprintRunner;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create FaaS executor
    let lambda = LambdaExecutor::new(
        "us-east-1",
        "arn:aws:iam::123456789:role/lambda-execution"
    ).await?;

    // Build runner with mixed execution
    BlueprintRunner::builder(config, env)
        .router(router)
        .producer(producer)
        .consumer(consumer)
        .with_faas_executor(0, lambda)  // Job 0 runs on Lambda
        .run().await?;

    Ok(())
}
```

### GCP Cloud Functions

```rust
use blueprint_faas::gcp::CloudFunctionExecutor;

let gcp_executor = CloudFunctionExecutor::new(
    "my-project-id".to_string(),
    "us-central1".to_string()
).await?;

BlueprintRunner::builder(config, env)
    .with_faas_executor(0, gcp_executor)
    .run().await?;
```

### Azure Functions

```rust
use blueprint_faas::azure::AzureFunctionExecutor;

let azure_executor = AzureFunctionExecutor::new(
    "my-subscription-id".to_string(),
    "eastus".to_string()
).await?;

BlueprintRunner::builder(config, env)
    .with_faas_executor(0, azure_executor)
    .run().await?;
```

### DigitalOcean Functions

```rust
use blueprint_faas::digitalocean::DigitalOceanExecutor;

let do_executor = DigitalOceanExecutor::new(
    "your-digitalocean-api-token".to_string(),
    "nyc1".to_string()  // Region: nyc1, sfo3, ams3, etc.
).await?;

BlueprintRunner::builder(config, env)
    .with_faas_executor(0, do_executor)
    .run().await?;
```

### Custom HTTP FaaS

For custom serverless platforms, implement the [Custom FaaS Platform Spec](../../docs/custom-faas-platform-spec.md) and use:

```rust
use blueprint_faas::custom::HttpFaasExecutor;

let executor = HttpFaasExecutor::new("https://my-faas.com")
    .with_job_endpoint(0, "https://my-faas.com/square")
    .with_job_endpoint(1, "https://my-faas.com/compute");

BlueprintRunner::builder(config, env)
    .with_faas_executor(0, executor.clone())
    .with_faas_executor(1, executor)
    .run().await?;
```

## Implementation Status

### ✅ Completed

- [x] Core FaaS trait abstraction (`FaasExecutor`)
- [x] FaaS registry for job-to-executor mapping
- [x] Runtime delegation in BlueprintRunner event loop
- [x] AWS Lambda full implementation
  - [x] Function deployment with binary packaging
  - [x] Job invocation with error handling
  - [x] Health checks and pre-warming
  - [x] Metrics collection
- [x] GCP Cloud Functions full implementation
  - [x] Cloud Functions v2 API integration
  - [x] Cloud Storage for deployment packages
  - [x] Token caching and refresh
  - [x] Full deployment lifecycle
- [x] Azure Functions full implementation
  - [x] ARM API integration
  - [x] Resource group and function app management
  - [x] ZipDeploy for function code
  - [x] DefaultAzureCredential authentication
- [x] Custom HTTP FaaS executor
  - [x] Configurable endpoints per job
  - [x] JSON serialization of JobCall/JobResult
  - [x] Custom FaaS Platform Specification
- [x] DigitalOcean Functions full implementation
  - [x] Namespace management and function deployment
  - [x] Binary packaging with base64 encoding
  - [x] Function lifecycle management
  - [x] Health checks and warming
- [x] Builder API (`.with_faas_executor()`)
- [x] Comprehensive documentation

### 📋 Testing Status

**Test Coverage:**
- ✅ 14 unit and integration tests passing
- ✅ HTTP FaaS executor tests with endpoint configuration
- ✅ Function naming and resource management tests
- ✅ Reference server for local development
- 🔒 11 tests require cloud credentials (ignored in CI)

**Run Tests:**
```bash
# Run all tests (credential tests ignored)
cargo test -p blueprint-faas --all-features

# Run custom HTTP tests
cargo test -p blueprint-faas --features custom

# Run reference server for manual testing
cargo run --example reference_faas_server --features custom
```

### 🚧 Future Enhancements

- E2E tests for GCP, Azure, and DigitalOcean providers with real cloud deployments
- Additional providers: Vercel Functions, Netlify Functions, Cloudflare Workers (with WASM support)
- Performance benchmarks and optimization

## Features

```toml
[dependencies]
blueprint-faas = { version = "0.1", features = ["aws"] }
```

Available features:
- `aws` - AWS Lambda integration
- `gcp` - Google Cloud Functions integration
- `azure` - Azure Functions integration
- `digitalocean` - DigitalOcean Functions integration
- `custom` - Custom HTTP FaaS
- `all` - All providers

## Provider Configuration

### AWS Lambda

**Authentication**: Uses AWS SDK credentials (IAM roles, environment variables, or `~/.aws/credentials`)

**Requirements**:
- AWS account with Lambda access
- IAM role with Lambda execution permissions
- Binary deployment region configuration

**Setup**:
```bash
export AWS_REGION=us-east-1
export AWS_ACCESS_KEY_ID=your-key-id
export AWS_SECRET_ACCESS_KEY=your-secret-key
```

### GCP Cloud Functions

**Authentication**: Uses Application Default Credentials

**Requirements**:
- GCP project with Cloud Functions API enabled
- Cloud Storage API enabled (for function deployment)
- Service account with appropriate permissions

**Setup**:
```bash
gcloud auth application-default login
export GOOGLE_PROJECT_ID=my-project-id
```

Or use service account:
```bash
export GOOGLE_APPLICATION_CREDENTIALS=/path/to/service-account-key.json
```

### Azure Functions

**Authentication**: Uses DefaultAzureCredential (supports multiple auth methods)

**Requirements**:
- Azure subscription
- Resource group for function deployment
- Azure Functions runtime access

**Setup**:
```bash
az login
export AZURE_SUBSCRIPTION_ID=your-subscription-id
```

Or use service principal:
```bash
export AZURE_CLIENT_ID=your-client-id
export AZURE_CLIENT_SECRET=your-client-secret
export AZURE_TENANT_ID=your-tenant-id
```

### DigitalOcean Functions

**Authentication**: Uses DigitalOcean API token

**Requirements**:
- DigitalOcean account with Functions access
- API token with read/write permissions
- Namespace is automatically created if not exists

**Setup**:
```bash
export DIGITALOCEAN_TOKEN=your-api-token
```

**Supported Regions**: `nyc1`, `nyc3`, `sfo3`, `ams3`, `sgp1`, `fra1`, `tor1`, `blr1`, `syd1`

### Custom HTTP FaaS

**Authentication**: Configurable (API Key, OAuth 2.0, mTLS)

**Requirements**:
- HTTP endpoint implementing the [Custom FaaS Platform Spec](../../docs/custom-faas-platform-spec.md)

**Setup**: Provider-specific (see your platform documentation)

## When to Use FaaS

### Good Use Cases ✅

- **Infrequent, expensive jobs** - Save costs by not running 24/7
- **Bursty workloads** - Auto-scaling handles traffic spikes
- **Isolated computation** - CPU/memory-intensive tasks
- **Cost optimization** - Pay-per-use vs always-on

### Keep Local ❌

- **Frequent, cheap jobs** - FaaS invocation overhead not worth it
- **Stateful operations** - FaaS functions are stateless
- **Low latency requirements** - Cold starts add latency
- **Large binary deployments** - Lambda has size limits

## Building Custom FaaS Platforms

Want to integrate your own serverless platform? Blueprint SDK provides a **complete specification** for custom FaaS platforms.

See the **[Custom FaaS Platform Specification](../../docs/custom-faas-platform-spec.md)** for:

- **HTTP API Requirements**: Full lifecycle endpoints (deploy, invoke, health, undeploy)
- **Request/Response Formats**: Complete JSON schemas with examples
- **Authentication Options**: API Key, OAuth 2.0, mTLS
- **Performance Requirements**: Latency targets, throughput, reliability
- **Resource Limits**: Binary size, memory, timeout, concurrency
- **Reference Implementation**: Complete Python (FastAPI) example
- **Testing Procedures**: Step-by-step integration testing

The specification enables **ANY** serverless platform to become a first-class FaaS provider in Blueprint SDK, with the same capabilities as AWS Lambda, GCP Cloud Functions, and Azure Functions.

Example usage:
```rust
use blueprint_faas::custom::HttpFaasExecutor;

// Point to your custom platform implementing the spec
let executor = HttpFaasExecutor::new("https://your-platform.com")
    .with_auth_header("Authorization", "Bearer your-api-key");

BlueprintRunner::builder(config, env)
    .with_faas_executor(0, executor)
    .run().await?;
```

## Architecture Decisions

### Why Trait-Based?

Provider-agnostic design allows:
- Easy switching between FaaS providers
- Testing with mock executors
- Custom platform integration

### Why Registry Pattern?

Centralized job-to-executor mapping enables:
- Clear visibility of FaaS vs local execution
- Runtime reconfiguration
- Per-job provider selection

### Why Programmatic Delegation?

Explicit `.with_faas_executor()` registration provides:
- Fine-grained control
- No magic/detection logic
- Clear developer intent

## Development

### Adding a New Provider

1. Create module in `src/` (e.g., `cloudflare.rs`)
2. Implement `FaasExecutor` trait
3. Add feature flag to `Cargo.toml`
4. Re-export in `lib.rs`

### Testing

Due to the sp-io issue, use:
```bash
# Verify structure compiles
cargo check -p blueprint-faas --features all

# Run basic tests (when sp-io is fixed)
cargo test -p blueprint-faas
```

## Examples

See `examples/` directory:
- `reference_faas_server.rs` - Reference HTTP FaaS server implementing the Custom FaaS Platform Spec

Run the reference server for local testing:
```bash
cargo run --example reference_faas_server --features custom
```

The server runs on `http://localhost:8080` and implements all endpoints from the [Custom FaaS Platform Specification](../../docs/custom-faas-platform-spec.md).

## Contributing

All major cloud providers (AWS, GCP, Azure, DigitalOcean) are now fully implemented! Contributions welcome for:

- **E2E Tests**: Integration tests for GCP, Azure, and DigitalOcean providers
- **Additional Providers**: Vercel Functions, Netlify Functions, Cloudflare Workers (with WASM), Deno Deploy, etc.
- **Performance Optimizations**: Token caching, connection pooling
- **Documentation**: More examples and tutorials
- **Custom FaaS Platforms**: Build your own using the [specification](../../docs/custom-faas-platform-spec.md)

## License

Same as parent project.
