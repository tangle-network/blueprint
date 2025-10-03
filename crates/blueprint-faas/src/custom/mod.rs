//! Custom HTTP-based FaaS integration
//!
//! This module provides a simple HTTP-based FaaS executor that can work with
//! any custom serverless runtime that accepts HTTP requests.

use super::*;
use blueprint_core::{JobCall, JobResult};
use reqwest::Client;
use std::collections::HashMap;
use std::time::Instant;
use tracing::{debug, info, warn};

/// HTTP-based FaaS executor for custom runtimes
///
/// This executor works with any HTTP-based serverless platform or custom
/// implementation. It sends JobCall as JSON via HTTP POST and expects
/// JobResult as JSON response.
///
/// # Example
///
/// ```rust,ignore
/// let executor = HttpFaasExecutor::new("https://my-faas.example.com");
///
/// BlueprintRunner::builder(config, env)
///     .with_faas_executor(0, executor)
///     .run().await
/// ```
#[derive(Debug, Clone)]
pub struct HttpFaasExecutor {
    base_url: String,
    client: Client,
    job_endpoints: HashMap<u32, String>,
}

impl HttpFaasExecutor {
    /// Create a new HTTP FaaS executor
    ///
    /// # Arguments
    ///
    /// * `base_url` - Base URL of the FaaS platform (e.g., "https://faas.example.com")
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            client: Client::new(),
            job_endpoints: HashMap::new(),
        }
    }

    /// Register a custom endpoint for a specific job
    ///
    /// By default, jobs are invoked at `{base_url}/job/{job_id}`.
    /// This allows overriding that for specific jobs.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let executor = HttpFaasExecutor::new("https://faas.example.com")
    ///     .with_job_endpoint(0, "https://special.example.com/square");
    /// ```
    #[must_use]
    pub fn with_job_endpoint(mut self, job_id: u32, endpoint: impl Into<String>) -> Self {
        self.job_endpoints.insert(job_id, endpoint.into());
        self
    }

    fn endpoint(&self, job_id: u32) -> String {
        self.job_endpoints
            .get(&job_id)
            .cloned()
            .unwrap_or_else(|| format!("{}/job/{}", self.base_url, job_id))
    }
}

#[async_trait::async_trait]
impl FaasExecutor for HttpFaasExecutor {
    async fn invoke(&self, job_call: JobCall) -> Result<JobResult, FaasError> {
        let endpoint = self.endpoint(job_call.job_id);

        debug!(
            job_id = job_call.job_id,
            endpoint = %endpoint,
            "Invoking HTTP FaaS function"
        );

        let start = Instant::now();

        let response = self
            .client
            .post(&endpoint)
            .json(&job_call)
            .send()
            .await
            .map_err(|e| {
                warn!(error = %e, "HTTP FaaS invocation failed");
                FaasError::InvocationFailed(e.to_string())
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(FaasError::FunctionError(format!(
                "HTTP {} - {}",
                status, body
            )));
        }

        let result: JobResult = response
            .json()
            .await
            .map_err(|e| FaasError::SerializationError(e.to_string()))?;

        let duration = start.elapsed();

        info!(
            job_id = job_call.job_id,
            duration_ms = duration.as_millis(),
            "HTTP FaaS invocation successful"
        );

        Ok(result)
    }

    async fn deploy_job(
        &self,
        _job_id: u32,
        _binary: &[u8],
        _config: &FaasConfig,
    ) -> Result<FaasDeployment, FaasError> {
        // Custom HTTP FaaS doesn't support automated deployment
        // User must deploy manually
        Err(FaasError::InfrastructureError(
            "Custom HTTP FaaS does not support automated deployment. \
             Deploy your function manually and register its endpoint."
                .into(),
        ))
    }

    async fn health_check(&self, job_id: u32) -> Result<bool, FaasError> {
        let endpoint = self.endpoint(job_id);

        debug!(endpoint = %endpoint, "Checking HTTP FaaS health");

        // Try to reach the endpoint with a HEAD request
        self.client
            .head(&endpoint)
            .send()
            .await
            .map(|r| r.status().is_success())
            .map_err(|e| FaasError::InfrastructureError(format!("Health check failed: {}", e)))
    }

    async fn get_deployment(&self, job_id: u32) -> Result<FaasDeployment, FaasError> {
        Ok(FaasDeployment {
            function_id: format!("http-job-{}", job_id),
            job_id,
            endpoint: self.endpoint(job_id),
            cold_start_ms: None,
            memory_mb: 0,    // Unknown
            timeout_secs: 0, // Unknown
        })
    }

    async fn undeploy_job(&self, _job_id: u32) -> Result<(), FaasError> {
        // Custom HTTP FaaS doesn't support automated undeployment
        Ok(())
    }

    fn provider_name(&self) -> &str {
        "Custom HTTP FaaS"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endpoint_generation() {
        let executor = HttpFaasExecutor::new("https://faas.example.com");
        assert_eq!(executor.endpoint(0), "https://faas.example.com/job/0");
        assert_eq!(executor.endpoint(5), "https://faas.example.com/job/5");
    }

    #[test]
    fn test_custom_endpoint() {
        let executor = HttpFaasExecutor::new("https://faas.example.com")
            .with_job_endpoint(0, "https://custom.example.com/square");

        assert_eq!(executor.endpoint(0), "https://custom.example.com/square");
        assert_eq!(executor.endpoint(1), "https://faas.example.com/job/1");
    }

    #[tokio::test]
    #[ignore = "Requires running HTTP server"]
    async fn test_http_invocation() {
        // This test would require a mock HTTP server
        // Will implement with wiremock in actual testing
    }
}
