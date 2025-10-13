//! AWS Lambda FaaS integration
//!
//! This module provides integration with AWS Lambda for executing blueprint jobs.

use super::*;
use aws_config::{BehaviorVersion, Region};
use aws_sdk_lambda::Client as LambdaClient;
use aws_sdk_lambda::primitives::Blob;
use aws_sdk_lambda::types::{FunctionCode, Runtime};
use blueprint_core::{JobCall, JobResult};
use std::time::Instant;
use tracing::{debug, info, warn};

/// AWS Lambda executor for blueprint jobs
///
/// This executor delegates job execution to AWS Lambda functions.
/// Each job ID maps to a separate Lambda function.
#[derive(Debug, Clone)]
pub struct LambdaExecutor {
    client: LambdaClient,
    function_prefix: String,
    role_arn: String,
}

impl LambdaExecutor {
    /// Create a new Lambda executor for a specific region
    ///
    /// # Arguments
    ///
    /// * `region` - AWS region (e.g., "us-east-1")
    /// * `role_arn` - IAM role ARN for Lambda execution
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let executor = LambdaExecutor::new(
    ///     "us-east-1",
    ///     "arn:aws:iam::123456789:role/lambda-execution"
    /// ).await?;
    /// ```
    pub async fn new(region: &str, role_arn: impl Into<String>) -> Result<Self, FaasError> {
        let config = aws_config::defaults(BehaviorVersion::latest())
            .region(Region::new(region.to_owned()))
            .load()
            .await;

        Ok(Self {
            client: LambdaClient::new(&config),
            function_prefix: "blueprint".to_string(),
            role_arn: role_arn.into(),
        })
    }

    /// Set the function name prefix (default: "blueprint")
    #[must_use]
    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.function_prefix = prefix.into();
        self
    }

    fn function_name(&self, job_id: u32) -> String {
        format!("{}-job-{}", self.function_prefix, job_id)
    }
}

#[async_trait::async_trait]
impl FaasExecutor for LambdaExecutor {
    async fn invoke(&self, job_call: JobCall) -> Result<JobResult, FaasError> {
        let job_id: u32 = job_call.job_id().into();
        let function_name = self.function_name(job_id);

        debug!(
            job_id = job_id,
            function = %function_name,
            "Invoking Lambda function"
        );

        // Convert JobCall to serializable payload
        let faas_payload: super::FaasPayload = job_call.into();
        let payload = serde_json::to_vec(&faas_payload)
            .map_err(|e| FaasError::SerializationError(e.to_string()))?;

        let start = Instant::now();

        let response = self
            .client
            .invoke()
            .function_name(&function_name)
            .payload(Blob::new(payload))
            .send()
            .await
            .map_err(|e| {
                warn!(error = %e, "Lambda invocation failed");
                FaasError::InvocationFailed(e.to_string())
            })?;

        let duration = start.elapsed();

        // Check for function errors
        if let Some(error) = response.function_error {
            return Err(FaasError::FunctionError(error));
        }

        let payload = response
            .payload
            .ok_or_else(|| FaasError::FunctionError("No payload returned".into()))?;

        let faas_response: super::FaasResponse = serde_json::from_slice(payload.as_ref())
            .map_err(|e| FaasError::SerializationError(e.to_string()))?;

        info!(
            job_id = job_id,
            duration_ms = duration.as_millis(),
            "Lambda invocation successful"
        );

        Ok(faas_response.into())
    }

    async fn invoke_with_metrics(
        &self,
        job_call: JobCall,
    ) -> Result<(JobResult, FaasMetrics), FaasError> {
        let job_id: u32 = job_call.job_id().into();
        let function_name = self.function_name(job_id);

        // Convert JobCall to serializable payload
        let faas_payload: super::FaasPayload = job_call.into();
        let payload = serde_json::to_vec(&faas_payload)
            .map_err(|e| FaasError::SerializationError(e.to_string()))?;

        let start = Instant::now();

        let response = self
            .client
            .invoke()
            .function_name(&function_name)
            .payload(Blob::new(payload))
            .send()
            .await
            .map_err(|e| FaasError::InvocationFailed(e.to_string()))?;

        let total_duration = start.elapsed();

        if let Some(error) = response.function_error {
            return Err(FaasError::FunctionError(error));
        }

        let payload = response
            .payload
            .ok_or_else(|| FaasError::FunctionError("No payload returned".into()))?;

        let faas_response: super::FaasResponse = serde_json::from_slice(payload.as_ref())
            .map_err(|e| FaasError::SerializationError(e.to_string()))?;

        // Extract Lambda-specific metrics
        let log_result = response.log_result.unwrap_or_default();

        let metrics = FaasMetrics {
            total_duration_ms: total_duration.as_millis() as u64,
            execution_duration_ms: total_duration.as_millis() as u64, // Lambda doesn't separate this
            cold_start: log_result.contains("Init Duration"),
            memory_used_mb: None, // Would need to parse from logs
            billed_duration_ms: total_duration.as_millis() as u64,
        };

        Ok((faas_response.into(), metrics))
    }

    async fn deploy_job(
        &self,
        job_id: u32,
        binary: &[u8],
        config: &FaasConfig,
    ) -> Result<FaasDeployment, FaasError> {
        let function_name = self.function_name(job_id);

        info!(
            job_id,
            function = %function_name,
            memory_mb = config.memory_mb,
            timeout_secs = config.timeout_secs,
            "Deploying Lambda function"
        );

        // Package binary for Lambda (Custom Runtime)
        let zip_package = crate::utils::create_lambda_package(binary)?;

        // Try to update existing function first
        let update_result = self
            .client
            .update_function_code()
            .function_name(&function_name)
            .zip_file(Blob::new(zip_package.clone()))
            .send()
            .await;

        if update_result.is_ok() {
            info!(function = %function_name, "Updated existing Lambda function");
        } else {
            // Function doesn't exist, create it
            debug!(function = %function_name, "Creating new Lambda function");

            self.client
                .create_function()
                .function_name(&function_name)
                .runtime(Runtime::Providedal2023)
                .role(&self.role_arn)
                .handler("bootstrap")
                .code(
                    FunctionCode::builder()
                        .zip_file(Blob::new(zip_package))
                        .build(),
                )
                .memory_size(config.memory_mb as i32)
                .timeout(config.timeout_secs as i32)
                .environment(
                    aws_sdk_lambda::types::Environment::builder()
                        .set_variables(Some(
                            config
                                .env_vars
                                .iter()
                                .map(|(k, v)| (k.clone(), v.clone()))
                                .collect(),
                        ))
                        .build(),
                )
                .send()
                .await
                .map_err(|e| {
                    FaasError::InfrastructureError(format!("Failed to create function: {}", e))
                })?;

            info!(function = %function_name, "Created new Lambda function");
        }

        // Update function configuration
        self.client
            .update_function_configuration()
            .function_name(&function_name)
            .memory_size(config.memory_mb as i32)
            .timeout(config.timeout_secs as i32)
            .send()
            .await
            .map_err(|e| {
                FaasError::InfrastructureError(format!("Failed to update configuration: {}", e))
            })?;

        Ok(FaasDeployment {
            function_id: function_name.clone(),
            job_id,
            endpoint: function_name,
            cold_start_ms: Some(300), // Typical Lambda cold start
            memory_mb: config.memory_mb,
            timeout_secs: config.timeout_secs,
        })
    }

    async fn health_check(&self, job_id: u32) -> Result<bool, FaasError> {
        let function_name = self.function_name(job_id);

        self.client
            .get_function()
            .function_name(&function_name)
            .send()
            .await
            .map(|_| true)
            .map_err(|e| FaasError::InfrastructureError(format!("Health check failed: {}", e)))
    }

    async fn warm(&self, job_id: u32) -> Result<(), FaasError> {
        let function_name = self.function_name(job_id);

        debug!(function = %function_name, "Warming Lambda function");

        // Create a no-op invocation to warm the function
        let _response = self
            .client
            .invoke()
            .function_name(&function_name)
            .payload(Blob::new(b"{}"))
            .send()
            .await
            .map_err(|e| {
                FaasError::InfrastructureError(format!("Failed to warm function: {}", e))
            })?;

        Ok(())
    }

    async fn get_deployment(&self, job_id: u32) -> Result<FaasDeployment, FaasError> {
        let function_name = self.function_name(job_id);

        let function = self
            .client
            .get_function()
            .function_name(&function_name)
            .send()
            .await
            .map_err(|e| {
                FaasError::InfrastructureError(format!("Failed to get function: {}", e))
            })?;

        let config = function
            .configuration
            .ok_or_else(|| FaasError::InfrastructureError("No configuration in response".into()))?;

        Ok(FaasDeployment {
            function_id: function_name.clone(),
            job_id,
            endpoint: function_name,
            cold_start_ms: Some(300),
            memory_mb: config.memory_size.unwrap_or(512) as u32,
            timeout_secs: config.timeout.unwrap_or(300) as u32,
        })
    }

    async fn undeploy_job(&self, job_id: u32) -> Result<(), FaasError> {
        let function_name = self.function_name(job_id);

        info!(function = %function_name, "Deleting Lambda function");

        self.client
            .delete_function()
            .function_name(&function_name)
            .send()
            .await
            .map_err(|e| {
                FaasError::InfrastructureError(format!("Failed to delete function: {}", e))
            })?;

        Ok(())
    }

    fn provider_name(&self) -> &str {
        "AWS Lambda"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "Requires AWS credentials"]
    async fn test_lambda_executor_creation() {
        let executor =
            LambdaExecutor::new("us-east-1", "arn:aws:iam::123456789:role/lambda-execution").await;

        assert!(executor.is_ok());
    }
}
