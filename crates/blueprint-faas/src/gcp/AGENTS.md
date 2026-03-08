# gcp

## Purpose
Google Cloud Functions (v2) executor implementing the `FaasExecutor` trait. Manages Cloud Function lifecycle via GCP REST API with token caching for performance.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - `CloudFunctionExecutor` struct implementing `FaasExecutor`. Caches OAuth tokens for reuse. Invokes via the function's HTTP trigger URL, deploys by uploading source to Cloud Storage then creating/updating the function via Cloud Functions v2 API, health-checks by querying function state, and undeploys by deleting the function.

## Key APIs
- `CloudFunctionExecutor::new(project_id, region)` - constructor with GCP project and region
- `FaasExecutor::invoke(job_id, input)` - POST to the function's HTTP trigger URL
- `FaasExecutor::deploy_job(job_id, code)` - uploads to Cloud Storage, creates/updates Cloud Function v2
- `FaasExecutor::health_check(job_id)` - queries function state (ACTIVE check) via REST API
- `FaasExecutor::undeploy_job(job_id)` - deletes the Cloud Function

## Relationships
- Implements `FaasExecutor` trait defined in the parent `blueprint-faas` crate
- Uses GCP Cloud Functions v2 and Cloud Storage REST APIs
- Peer to `aws/`, `azure/`, `digitalocean/`, and `custom/` executor implementations
