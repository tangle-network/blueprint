# digitalocean

## Purpose
DigitalOcean Functions executor implementing the `FaasExecutor` trait. Manages serverless functions on DigitalOcean's Functions platform via its REST API.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - `DigitalOceanExecutor` struct implementing `FaasExecutor`. Invokes functions via namespace trigger endpoints, deploys by creating namespaces and uploading base64-encoded binary packages, health-checks by querying function status, warms via trigger invocation, and undeploys by deleting the function from its namespace.

## Key APIs
- `DigitalOceanExecutor::new(api_token, region)` - constructor with DO API token and region
- `FaasExecutor::invoke(job_id, input)` - POST to the function's trigger endpoint
- `FaasExecutor::deploy_job(job_id, code)` - creates namespace if needed, uploads function as base64 binary package
- `FaasExecutor::health_check(job_id)` - queries function status via DO API
- `FaasExecutor::undeploy_job(job_id)` - deletes the function from its namespace

## Relationships
- Implements `FaasExecutor` trait defined in the parent `blueprint-faas` crate
- Uses DigitalOcean Functions REST API with bearer token auth
- Peer to `aws/`, `azure/`, `gcp/`, and `custom/` executor implementations
