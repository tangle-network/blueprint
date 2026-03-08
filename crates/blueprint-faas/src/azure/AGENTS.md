# azure

## Purpose
Azure Functions executor implementing the `FaasExecutor` trait. Manages Azure Function Apps via ARM REST API with `DefaultAzureCredential` authentication.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - `AzureFunctionExecutor` struct implementing `FaasExecutor`. Invokes via HTTP trigger URL, deploys by creating resource groups and function apps then uploading code via ZipDeploy, health-checks via ARM API status, and undeploys by deleting the resource group.

## Key APIs
- `AzureFunctionExecutor::new(subscription_id, resource_group, location)` - constructor with Azure subscription context
- `FaasExecutor::invoke(job_id, input)` - POST to the function's HTTP trigger endpoint
- `FaasExecutor::deploy_job(job_id, code)` - creates resource group + function app via ARM API, uploads code via ZipDeploy
- `FaasExecutor::health_check(job_id)` - queries function app status via ARM API
- `FaasExecutor::undeploy_job(job_id)` - deletes the containing resource group

## Relationships
- Implements `FaasExecutor` trait defined in the parent `blueprint-faas` crate
- Uses `DefaultAzureCredential` for authentication (env vars, managed identity, CLI)
- Peer to `aws/`, `gcp/`, `digitalocean/`, and `custom/` executor implementations
