# Custom FaaS Platform Integration - Complete Specification

## Overview

This document specifies the complete HTTP API that a custom FaaS platform must implement to integrate with Blueprint SDK's `FaasExecutor` trait. This enables full lifecycle management: deployment, invocation, health checks, and teardown.

## Why This Spec?

Blueprint SDK supports AWS Lambda, GCP Cloud Functions, and Azure Functions out of the box. This spec allows ANY custom serverless platform to be a **first-class citizen** with the same capabilities.

## Architecture

```
Blueprint Manager
      ↓
FaasExecutor Trait
      ↓
HttpFaasExecutor (your integration)
      ↓
Your FaaS Platform (implements this spec)
```

---

## Required HTTP Endpoints

### 1. **Deploy Function**

Upload and deploy a new function.

```http
PUT /api/functions/{function_id}
Content-Type: application/zip
X-Blueprint-Config: <base64-encoded-json>

{binary zip package containing 'bootstrap' executable}
```

**Headers:**
- `Content-Type: application/zip` - Binary zip package
- `X-Blueprint-Config` (optional) - Base64-encoded JSON config

**Config JSON Format** (before base64 encoding):
```json
{
  "memory_mb": 512,
  "timeout_secs": 300,
  "max_concurrency": 10,
  "env_vars": {
    "KEY": "value"
  }
}
```

**Response (200 OK):**
```json
{
  "function_id": "job0",
  "endpoint": "https://your-platform.com/api/functions/job0/invoke",
  "status": "deployed",
  "cold_start_ms": 500,
  "memory_mb": 512,
  "timeout_secs": 300
}
```

**Response (409 Conflict):**
```json
{
  "error": "Function already exists",
  "function_id": "job0"
}
```

**Binary Package Format:**

The zip must contain a `bootstrap` executable at the root:
```
job0.zip
└── bootstrap  (executable, chmod +x)
```

Your platform should:
1. Extract the zip
2. Make `bootstrap` executable
3. Run it when invoked with JSON via stdin

---

### 2. **Invoke Function**

Execute a deployed function.

```http
POST /api/functions/{function_id}/invoke
Content-Type: application/json

{
  "job_id": 0,
  "args": [1, 2, 3, 4, 5, 6, 7, 8]
}
```

**Request Payload:**
- `job_id` (number): Job identifier
- `args` (array of bytes): Serialized job arguments

**Response (200 OK):**
```json
{
  "job_id": 0,
  "result": [25, 0, 0, 0, 0, 0, 0, 0],
  "success": true,
  "execution_ms": 45,
  "memory_used_mb": 128
}
```

**Response (500 Internal Server Error):**
```json
{
  "job_id": 0,
  "result": [],
  "success": false,
  "error": "Function timeout after 300s"
}
```

**Execution Model:**

When invoked, your platform should:
1. Spawn the `bootstrap` binary
2. Write the request JSON to stdin
3. Read the response JSON from stdout
4. Return the response

Example execution:
```bash
echo '{"job_id":0,"args":[1,2,3,4,5,6,7,8]}' | ./bootstrap
# outputs: {"job_id":0,"result":[25,0,0,0,0,0,0,0],"success":true}
```

---

### 3. **Health Check**

Check if a function is deployed and healthy.

```http
GET /api/functions/{function_id}/health
```

**Response (200 OK):**
```json
{
  "function_id": "job0",
  "status": "healthy",
  "last_invocation": "2024-10-13T12:34:56Z",
  "total_invocations": 1523
}
```

**Response (404 Not Found):**
```json
{
  "error": "Function not found",
  "function_id": "job0"
}
```

**Response (503 Service Unavailable):**
```json
{
  "function_id": "job0",
  "status": "unhealthy",
  "error": "Binary crashed on startup"
}
```

---

### 4. **Get Deployment Info**

Retrieve information about a deployed function.

```http
GET /api/functions/{function_id}
```

**Response (200 OK):**
```json
{
  "function_id": "job0",
  "endpoint": "https://your-platform.com/api/functions/job0/invoke",
  "status": "deployed",
  "cold_start_ms": 500,
  "memory_mb": 512,
  "timeout_secs": 300,
  "deployed_at": "2024-10-13T10:00:00Z",
  "binary_size_bytes": 15728640
}
```

**Response (404 Not Found):**
```json
{
  "error": "Function not found",
  "function_id": "job0"
}
```

---

### 5. **Undeploy Function**

Remove a deployed function.

```http
DELETE /api/functions/{function_id}
```

**Response (200 OK):**
```json
{
  "function_id": "job0",
  "status": "deleted"
}
```

**Response (404 Not Found):**
```json
{
  "error": "Function not found",
  "function_id": "job0"
}
```

---

### 6. **Warm Function** (Optional but Recommended)

Pre-warm a function to avoid cold starts.

```http
POST /api/functions/{function_id}/warm
```

**Response (200 OK):**
```json
{
  "function_id": "job0",
  "status": "warm",
  "instances_warmed": 3
}
```

This should:
1. Start N instances of the function
2. Keep them in memory for subsequent invocations
3. Reduce cold start latency

---

## Authentication

Your platform can implement any auth scheme. Common options:

### API Key (Simplest)

```http
POST /api/functions/job0/invoke
Authorization: Bearer your-api-key-here
```

Blueprint operators configure:
```rust
let executor = HttpFaasExecutor::new("https://your-platform.com")
    .with_auth_header("Authorization", "Bearer your-api-key");
```

### OAuth 2.0

```http
POST /api/functions/job0/invoke
Authorization: Bearer eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9...
```

### mTLS (Most Secure)

Use client certificates for authentication. Blueprint SDK supports this via `reqwest` client configuration.

---

## Error Handling

### HTTP Status Codes

| Code | Meaning | When to Use |
|------|---------|-------------|
| 200 OK | Success | Function deployed/invoked/deleted successfully |
| 400 Bad Request | Invalid request | Malformed JSON, missing fields |
| 401 Unauthorized | Auth failed | Invalid API key, expired token |
| 404 Not Found | Function doesn't exist | Invoking undeployed function |
| 409 Conflict | Resource exists | Deploying function that already exists |
| 413 Payload Too Large | Binary too large | Exceeds platform limits |
| 500 Internal Server Error | Platform error | Infrastructure failure |
| 503 Service Unavailable | Temporarily unavailable | Function unhealthy, overloaded |

### Error Response Format

All error responses should use this format:

```json
{
  "error": "Human-readable error message",
  "code": "ERROR_CODE",
  "details": {
    "additional": "context"
  }
}
```

Example:
```json
{
  "error": "Function execution timed out after 300 seconds",
  "code": "TIMEOUT",
  "details": {
    "function_id": "job0",
    "timeout_secs": 300,
    "execution_secs": 301
  }
}
```

---

## Performance Requirements

### Invocation Latency

| Metric | Target | Max Acceptable |
|--------|--------|----------------|
| Cold Start (P50) | <500ms | <2s |
| Cold Start (P99) | <1s | <5s |
| Warm Invocation (P50) | <50ms | <200ms |
| Warm Invocation (P99) | <100ms | <500ms |

### Throughput

- **Minimum**: 100 req/s per function
- **Recommended**: 1000 req/s per function
- **Concurrency**: Support at least 100 concurrent invocations

### Reliability

- **Uptime**: 99.9% (3 nines)
- **Error Rate**: <0.1% for deployed functions
- **Deployment Success Rate**: >99%

---

## Resource Limits

Your platform should enforce these limits:

### Binary Size

- **Maximum**: 250 MB (uncompressed)
- **Recommended Limit**: 50 MB
- **Compressed Limit**: 50 MB (for upload)

### Memory

- **Minimum**: 128 MB
- **Maximum**: 10 GB (AWS Lambda parity)
- **Granularity**: 64 MB increments

### Timeout

- **Minimum**: 1 second
- **Maximum**: 900 seconds (15 minutes)
- **Default**: 300 seconds (5 minutes)

### Concurrency

- **Per Function**: 1-1000 concurrent executions
- **Per Account**: 1000-10000 concurrent executions

---

## Integration Example

### Operator Side (Blueprint SDK)

```rust
use blueprint_faas::custom::HttpFaasExecutor;
use blueprint_runner::BlueprintRunner;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create executor pointing to your platform
    let executor = HttpFaasExecutor::new("https://your-platform.com")
        .with_auth_header("Authorization", "Bearer your-api-key")
        .with_job_endpoint(0, "https://your-platform.com/api/functions/job0");

    // Deploy a job
    let binary = std::fs::read("./target/release/blueprint-job")?;
    let config = FaasConfig {
        memory_mb: 512,
        timeout_secs: 300,
        ..Default::default()
    };

    executor.deploy_job(0, &binary, &config).await?;

    // Register with blueprint runner
    BlueprintRunner::builder(config, env)
        .router(router)
        .producer(producer)
        .consumer(consumer)
        .with_faas_executor(0, executor)
        .run().await
}
```

### Platform Side (Your Implementation)

Example in Python (FastAPI):

```python
from fastapi import FastAPI, UploadFile, HTTPException
from pydantic import BaseModel
import subprocess
import json
import zipfile
import base64

app = FastAPI()

# Store deployed functions
functions = {}

class DeployResponse(BaseModel):
    function_id: str
    endpoint: str
    status: str
    cold_start_ms: int
    memory_mb: int
    timeout_secs: int

class InvokeRequest(BaseModel):
    job_id: int
    args: list[int]

class InvokeResponse(BaseModel):
    job_id: int
    result: list[int]
    success: bool
    execution_ms: int = 0

@app.put("/api/functions/{function_id}")
async def deploy_function(function_id: str, file: UploadFile):
    """Deploy a new function"""

    # Read config from header
    config_b64 = request.headers.get("X-Blueprint-Config", "")
    if config_b64:
        config = json.loads(base64.b64decode(config_b64))
    else:
        config = {"memory_mb": 512, "timeout_secs": 300}

    # Save and extract zip
    zip_path = f"/functions/{function_id}.zip"
    with open(zip_path, "wb") as f:
        f.write(await file.read())

    # Extract bootstrap
    with zipfile.ZipFile(zip_path) as zf:
        zf.extract("bootstrap", f"/functions/{function_id}/")

    # Make executable
    os.chmod(f"/functions/{function_id}/bootstrap", 0o755)

    # Store function metadata
    functions[function_id] = {
        "binary_path": f"/functions/{function_id}/bootstrap",
        "config": config
    }

    return DeployResponse(
        function_id=function_id,
        endpoint=f"https://your-platform.com/api/functions/{function_id}/invoke",
        status="deployed",
        cold_start_ms=500,
        **config
    )

@app.post("/api/functions/{function_id}/invoke")
async def invoke_function(function_id: str, request: InvokeRequest):
    """Invoke a deployed function"""

    if function_id not in functions:
        raise HTTPException(status_code=404, detail="Function not found")

    func = functions[function_id]
    binary_path = func["binary_path"]
    timeout = func["config"]["timeout_secs"]

    # Execute binary with JSON input
    input_json = json.dumps({"job_id": request.job_id, "args": request.args})

    try:
        result = subprocess.run(
            [binary_path],
            input=input_json,
            capture_output=True,
            text=True,
            timeout=timeout
        )

        if result.returncode != 0:
            return InvokeResponse(
                job_id=request.job_id,
                result=[],
                success=False
            )

        # Parse output
        output = json.loads(result.stdout)

        return InvokeResponse(
            job_id=output["job_id"],
            result=output["result"],
            success=output["success"],
            execution_ms=output.get("execution_ms", 0)
        )

    except subprocess.TimeoutExpired:
        return InvokeResponse(
            job_id=request.job_id,
            result=[],
            success=False
        )

@app.get("/api/functions/{function_id}/health")
async def health_check(function_id: str):
    """Check function health"""

    if function_id not in functions:
        raise HTTPException(status_code=404, detail="Function not found")

    return {
        "function_id": function_id,
        "status": "healthy",
        "last_invocation": functions[function_id].get("last_invocation"),
        "total_invocations": functions[function_id].get("invocations", 0)
    }

@app.delete("/api/functions/{function_id}")
async def undeploy_function(function_id: str):
    """Undeploy a function"""

    if function_id not in functions:
        raise HTTPException(status_code=404, detail="Function not found")

    # Clean up files
    import shutil
    shutil.rmtree(f"/functions/{function_id}/")

    del functions[function_id]

    return {"function_id": function_id, "status": "deleted"}
```

---

## Testing Your Implementation

### 1. Deploy Test

```bash
# Create test binary
echo '#!/bin/bash
cat' > bootstrap
chmod +x bootstrap
zip test.zip bootstrap

# Deploy
curl -X PUT https://your-platform.com/api/functions/test \
  -H "Content-Type: application/zip" \
  -H "X-Blueprint-Config: $(echo '{"memory_mb":512,"timeout_secs":60}' | base64)" \
  --data-binary @test.zip
```

### 2. Invoke Test

```bash
curl -X POST https://your-platform.com/api/functions/test/invoke \
  -H "Content-Type: application/json" \
  -d '{"job_id":0,"args":[1,2,3,4,5,6,7,8]}'
```

### 3. Health Check Test

```bash
curl https://your-platform.com/api/functions/test/health
```

### 4. Undeploy Test

```bash
curl -X DELETE https://your-platform.com/api/functions/test
```

---

## Reference Implementation

Blueprint SDK provides a reference implementation for testing:

```bash
# Clone the repo
git clone https://github.com/tangle-network/blueprint
cd blueprint/crates/blueprint-faas

# Run reference FaaS server
cargo run --example reference_faas_server --features custom

# Server runs on http://localhost:8080
# Implements this full spec for local testing
```

---

## Security Considerations

### 1. Binary Validation

- Scan uploaded binaries for malware
- Validate zip structure (no path traversal)
- Enforce binary size limits

### 2. Resource Isolation

- Use containers (Docker) or VMs for isolation
- Enforce memory/CPU limits via cgroups
- Network isolation between functions

### 3. Authentication

- Use TLS for all endpoints (HTTPS)
- Rotate API keys regularly
- Support OAuth 2.0 for enterprise customers

### 4. Rate Limiting

- Per-function rate limits
- Per-account rate limits
- Backpressure for overload scenarios

---

## Questions?

For more information:
- **GitHub**: https://github.com/tangle-network/blueprint
- **Docs**: https://docs.tangle.tools/developers/blueprints/faas
- **Discord**: https://discord.gg/cv8EfJu3Tn

This spec is versioned at **v1.0**. We're committed to backward compatibility.
