# aws

## Purpose
AWS Lambda executor implementing the `FaasExecutor` trait. Manages Lambda function lifecycle (create, update, invoke, warm, undeploy) using the AWS SDK.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - `LambdaExecutor` struct implementing `FaasExecutor`. Invokes functions via `aws_sdk_lambda::Client`, deploys by creating or updating Lambda functions with zip-packaged code, performs health checks via `get_function`, warms functions with dry-run invocations, and undeploys by deleting the function.

## Key APIs
- `LambdaExecutor::new(client, role_arn, runtime)` - constructor taking an AWS Lambda client, IAM role ARN, and Lambda runtime
- `FaasExecutor::invoke(job_id, input)` - invokes the Lambda function and returns the response payload
- `FaasExecutor::deploy_job(job_id, code)` - creates or updates a Lambda function with zip deployment package
- `FaasExecutor::health_check(job_id)` - checks function existence and state via `get_function`
- `FaasExecutor::warm(job_id)` - sends a dry-run invocation to keep the function warm
- `FaasExecutor::undeploy_job(job_id)` - deletes the Lambda function

## Relationships
- Implements `FaasExecutor` trait defined in the parent `blueprint-faas` crate
- Depends on `aws_sdk_lambda` for AWS API access
- Peer to `azure/`, `gcp/`, `digitalocean/`, and `custom/` executor implementations
