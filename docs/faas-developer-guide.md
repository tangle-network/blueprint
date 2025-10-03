# FaaS Execution for Blueprints - Developer Guide

## Overview

Blueprints can now delegate specific jobs to Function-as-a-Service (FaaS) platforms like AWS Lambda, Google Cloud Functions, or custom serverless runtimes. This is useful for:

- **Sporadic jobs** - Called rarely, wasteful to run 24/7
- **Burst workloads** - Need to scale quickly for short periods
- **Cost optimization** - Pay per execution instead of per hour

## Architecture

```
Tangle Blockchain
    ↓ Event: JobCalled(job_id=0)
BlueprintRunner (long-running)
    ↓ Checks: "Is job 0 FaaS?"
    ↓ Yes! Delegate to Lambda
AWS Lambda
    ↓ Executes job function
    ↓ Returns result
BlueprintRunner
    ↓ Submits result to Tangle
```

## The Core Trait

All FaaS providers implement this trait:

```rust
#[async_trait::async_trait]
pub trait FaasExecutor: Send + Sync {
    /// Invoke a job and get the result
    async fn invoke(&self, job_call: JobCall) -> Result<JobResult, FaasError>;

    /// Deploy a job to the FaaS platform
    async fn deploy_job(
        &self,
        job_id: u32,
        binary: &[u8],
        config: &FaasConfig,
    ) -> Result<FaasDeployment, FaasError>;

    /// Check if the function is healthy
    async fn health_check(&self, job_id: u32) -> Result<bool, FaasError>;

    /// Get provider name
    fn provider_name(&self) -> &str;
}
```

## Example: Mixed Execution Blueprint

### Job Functions (No Changes!)

```rust
// incredible-analytics-lib/src/lib.rs
use blueprint_sdk::tangle::extract::{TangleArg, TangleResult};

// Job 0: Heavy computation, called once per day
pub async fn generate_daily_report(
    TangleArg(date): TangleArg<String>
) -> TangleResult<String> {
    // Expensive analytics, perfect for FaaS
    let report = expensive_analytics(&date).await;
    TangleResult(report)
}

// Job 1: Needs continuous monitoring
pub async fn monitor_price_feed(
    TangleArg(asset): TangleArg<String>
) -> TangleResult<f64> {
    // Runs every block, needs to be long-running
    let price = fetch_price(&asset).await;
    TangleResult(price)
}
```

### Blueprint Binary (With FaaS)

```rust
// incredible-analytics-bin/src/main.rs
use blueprint_faas_lambda::LambdaExecutor;
use blueprint_runner::BlueprintRunner;
use blueprint_router::Router;

const REPORT_JOB: u32 = 0;
const MONITOR_JOB: u32 = 1;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let env = BlueprintEnvironment::load()?;

    // Create Lambda executor for sporadic jobs
    let lambda = LambdaExecutor::new("us-east-1").await?;

    // Build router (same as before)
    let router = Router::new()
        .route(REPORT_JOB, generate_daily_report.layer(TangleLayer))
        .route(MONITOR_JOB, monitor_price_feed.layer(TangleLayer));

    BlueprintRunner::builder(config, env)
        .router(router)
        .producer(tangle_producer)
        .consumer(tangle_consumer)
        // NEW: Declare which jobs use FaaS
        .with_faas_executor(REPORT_JOB, lambda)  // Job 0 → Lambda
        // Job 1 runs locally (no FaaS executor registered)
        .run().await
}
```

### What Happens at Runtime

**Job 0 Called (FaaS)**:
```
1. Tangle emits: JobCalled { job_id: 0, date: "2024-10-03" }
2. BlueprintRunner receives event
3. Runner checks: faas_registry.is_faas_job(0) → true
4. Runner invokes: lambda.invoke(job_call).await
5. Lambda executes generate_daily_report("2024-10-03")
6. Lambda returns result
7. Runner submits result to Tangle via consumer
```

**Job 1 Called (Long-Running)**:
```
1. Tangle emits: JobCalled { job_id: 1, asset: "BTC" }
2. BlueprintRunner receives event
3. Runner checks: faas_registry.is_faas_job(1) → false
4. Runner executes locally: router.call(job_call).await
5. Runner submits result to Tangle
```

## Provider Implementations

### AWS Lambda (Separate Crate)

```rust
// blueprint-faas-lambda/src/lib.rs

use blueprint_runner::faas::{FaasExecutor, FaasError, FaasDeployment, FaasConfig};
use aws_sdk_lambda::Client as LambdaClient;

#[derive(Debug)]
pub struct LambdaExecutor {
    client: LambdaClient,
    function_prefix: String,
}

impl LambdaExecutor {
    pub async fn new(region: &str) -> Result<Self, FaasError> {
        let config = aws_config::from_env()
            .region(Region::new(region.to_string()))
            .load()
            .await;

        Ok(Self {
            client: LambdaClient::new(&config),
            function_prefix: "blueprint".to_string(),
        })
    }
}

#[async_trait::async_trait]
impl FaasExecutor for LambdaExecutor {
    async fn invoke(&self, job_call: JobCall) -> Result<JobResult, FaasError> {
        let function_name = format!("{}-job-{}", self.function_prefix, job_call.job_id);

        let payload = serde_json::to_vec(&job_call)
            .map_err(|e| FaasError::SerializationError(e.to_string()))?;

        let response = self.client
            .invoke()
            .function_name(&function_name)
            .payload(Blob::new(payload))
            .send()
            .await
            .map_err(|e| FaasError::InvocationFailed(e.to_string()))?;

        let result: JobResult = serde_json::from_slice(
            response.payload.as_ref().ok_or(FaasError::FunctionError("No payload".into()))?
        ).map_err(|e| FaasError::SerializationError(e.to_string()))?;

        Ok(result)
    }

    async fn deploy_job(
        &self,
        job_id: u32,
        binary: &[u8],
        config: &FaasConfig,
    ) -> Result<FaasDeployment, FaasError> {
        // Package binary into Lambda deployment package
        let zip_package = create_lambda_package(binary)?;

        let function_name = format!("{}-job-{}", self.function_prefix, job_id);

        // Create or update Lambda function
        self.client
            .create_function()
            .function_name(&function_name)
            .runtime(Runtime::ProvidedAl2023)
            .role("arn:aws:iam::123456789:role/lambda-role")
            .handler("bootstrap")
            .code(FunctionCode::builder().zip_file(Blob::new(zip_package)).build())
            .memory_size(config.memory_mb as i32)
            .timeout(config.timeout_secs as i32)
            .send()
            .await
            .map_err(|e| FaasError::InfrastructureError(e.to_string()))?;

        Ok(FaasDeployment {
            function_id: function_name.clone(),
            job_id,
            endpoint: function_name,
            cold_start_ms: Some(300),
            memory_mb: config.memory_mb,
            timeout_secs: config.timeout_secs,
        })
    }

    async fn health_check(&self, job_id: u32) -> Result<bool, FaasError> {
        let function_name = format!("{}-job-{}", self.function_prefix, job_id);

        self.client
            .get_function()
            .function_name(&function_name)
            .send()
            .await
            .map(|_| true)
            .map_err(|_| FaasError::InfrastructureError("Function not found".into()))
    }

    fn provider_name(&self) -> &str {
        "AWS Lambda"
    }
}
```

### Custom Serverless Runtime (Example)

```rust
// blueprint-faas-custom/src/lib.rs

#[derive(Debug)]
pub struct CustomFaasExecutor {
    endpoint: String,
    http_client: reqwest::Client,
}

#[async_trait::async_trait]
impl FaasExecutor for CustomFaasExecutor {
    async fn invoke(&self, job_call: JobCall) -> Result<JobResult, FaasError> {
        // Simple HTTP-based invocation
        let response = self.http_client
            .post(&format!("{}/invoke", self.endpoint))
            .json(&job_call)
            .send()
            .await
            .map_err(|e| FaasError::InvocationFailed(e.to_string()))?;

        let result: JobResult = response.json().await
            .map_err(|e| FaasError::SerializationError(e.to_string()))?;

        Ok(result)
    }

    // ... implement other methods
}
```

## Deployment Workflow

```bash
# 1. Build blueprint binary
cargo build --release -p incredible-analytics-bin

# 2. Deploy (manager handles FaaS setup)
cargo tangle blueprint deploy \
    --faas-job 0:lambda \     # Job 0 → AWS Lambda
    --long-running-job 1      # Job 1 → K8s/VM

# Manager does:
# - Deploys binary to Lambda for job 0
# - Creates SQS queue for job 0 events
# - Deploys same binary to K8s for job 1
# - Configures event routing
```

## Testing

```rust
// Test with mock FaaS executor
#[derive(Debug)]
struct MockFaas {
    responses: HashMap<u32, JobResult>,
}

#[async_trait::async_trait]
impl FaasExecutor for MockFaas {
    async fn invoke(&self, job_call: JobCall) -> Result<JobResult, FaasError> {
        self.responses.get(&job_call.job_id)
            .cloned()
            .ok_or(FaasError::FunctionError("No mock response".into()))
    }

    // ... minimal implementation
}

#[tokio::test]
async fn test_faas_delegation() {
    let mut mock = MockFaas::default();
    mock.responses.insert(0, JobResult { /* ... */ });

    let runner = BlueprintRunner::builder(config, env)
        .router(router)
        .with_faas_executor(0, mock)
        .run_test()
        .await;

    // Verify job 0 was delegated
}
```

## Cost Comparison

**Long-Running (Current)**:
- t3.medium: $30/month running 24/7
- Job called 10x/day: $30/month

**FaaS (New)**:
- Lambda: $0.20 per 1M requests
- 10 calls/day = 300/month: ~$0.00006/month
- **~500,000x cheaper!**

## Next Steps

1. ✅ Core trait defined (`FaasExecutor`)
2. ✅ Builder API added (`.with_faas_executor()`)
3. ⬜ Implement delegation logic in runner
4. ⬜ Create `blueprint-faas-lambda` crate
5. ⬜ Add manager support for FaaS deployment
6. ⬜ Testing with real Lambda
