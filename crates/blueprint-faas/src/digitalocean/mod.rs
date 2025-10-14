//! DigitalOcean Functions FaaS integration
//!
//! This module provides integration with DigitalOcean Functions for executing blueprint jobs.
//!
//! # Authentication
//!
//! Uses DigitalOcean API token for authentication. Set `DIGITALOCEAN_TOKEN` environment
//! variable or pass the token during executor creation.
//!
//! # Example
//!
//! ```rust,ignore
//! let executor = DigitalOceanExecutor::new("your-api-token", "nyc1").await?;
//!
//! BlueprintRunner::builder(config, env)
//!     .with_faas_executor(0, executor)
//!     .run().await
//! ```

use super::*;
use blueprint_core::{JobCall, JobResult};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use tracing::{debug, info, warn};

/// DigitalOcean Functions executor for blueprint jobs
///
/// Integrates with DigitalOcean Functions API for deploying and invoking serverless functions.
///
/// # Namespace Model
///
/// DigitalOcean organizes functions into namespaces. This executor manages a single
/// namespace and deploys functions as "triggers" within that namespace.
#[derive(Debug, Clone)]
pub struct DigitalOceanExecutor {
    api_token: String,
    namespace_id: String,
    /// Region where namespace is deployed (kept for debugging and future use)
    #[allow(dead_code)]
    region: String,
    function_prefix: String,
    client: Client,
}

impl DigitalOceanExecutor {
    /// Create a new DigitalOcean Functions executor
    ///
    /// # Arguments
    ///
    /// * `api_token` - DigitalOcean API token with Functions read/write access
    /// * `region` - DigitalOcean region (e.g., "nyc1", "sfo3", "ams3")
    ///
    /// # Namespace Management
    ///
    /// Creates or reuses a namespace named "blueprint-functions" in the specified region.
    pub async fn new(api_token: impl Into<String>, region: impl Into<String>) -> Result<Self, FaasError> {
        let api_token = api_token.into();
        let region = region.into();

        debug!(region = %region, "Creating DigitalOcean Functions executor");

        let client = Client::new();

        // Get or create namespace
        let namespace_id = Self::get_or_create_namespace(&client, &api_token, &region).await?;

        Ok(Self {
            api_token,
            namespace_id,
            region,
            function_prefix: "blueprint".to_string(),
            client,
        })
    }

    /// Set the function name prefix (default: "blueprint")
    #[must_use]
    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.function_prefix = prefix.into();
        self
    }

    pub(crate) fn function_name(&self, job_id: u32) -> String {
        format!("{}-job-{}", self.function_prefix, job_id)
    }

    /// Get or create the functions namespace
    async fn get_or_create_namespace(
        client: &Client,
        api_token: &str,
        region: &str,
    ) -> Result<String, FaasError> {
        let namespace_name = "blueprint-functions";

        // Try to get existing namespace
        let url = "https://api.digitalocean.com/v2/functions/namespaces";
        let response = client
            .get(url)
            .bearer_auth(api_token)
            .send()
            .await
            .map_err(|e| FaasError::InfrastructureError(format!("Failed to list namespaces: {}", e)))?;

        if response.status().is_success() {
            let data: NamespaceListResponse = response
                .json()
                .await
                .map_err(|e| FaasError::SerializationError(e.to_string()))?;

            // Check if our namespace exists
            if let Some(ns) = data.namespaces.iter().find(|n| n.label == namespace_name) {
                debug!(namespace_id = %ns.id, "Using existing namespace");
                return Ok(ns.id.clone());
            }
        }

        // Create new namespace
        debug!(region = %region, "Creating new namespace");

        let create_req = CreateNamespaceRequest {
            label: namespace_name.to_string(),
            region: region.to_string(),
        };

        let response = client
            .post(url)
            .bearer_auth(api_token)
            .json(&create_req)
            .send()
            .await
            .map_err(|e| FaasError::InfrastructureError(format!("Failed to create namespace: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(FaasError::InfrastructureError(format!(
                "Failed to create namespace: {}",
                error_text
            )));
        }

        let data: NamespaceResponse = response
            .json()
            .await
            .map_err(|e| FaasError::SerializationError(e.to_string()))?;

        info!(namespace_id = %data.namespace.id, "Created new namespace");
        Ok(data.namespace.id)
    }

    pub(crate) fn api_endpoint(&self, path: &str) -> String {
        format!(
            "https://api.digitalocean.com/v2/functions/namespaces/{}/{}",
            self.namespace_id,
            path.trim_start_matches('/')
        )
    }
}

#[async_trait::async_trait]
impl FaasExecutor for DigitalOceanExecutor {
    async fn invoke(&self, job_call: JobCall) -> Result<JobResult, FaasError> {
        let job_id: u32 = job_call.job_id().into();
        let function_name = self.function_name(job_id);

        debug!(
            job_id = job_id,
            function = %function_name,
            "Invoking DigitalOcean Function"
        );

        // Get function URL
        let function_url = self.get_function_url(job_id).await?;

        // Convert JobCall to payload
        let payload: FaasPayload = job_call.into();

        let start = Instant::now();

        // Invoke function via HTTP
        let response = self
            .client
            .post(&function_url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                warn!(error = %e, "Function invocation failed");
                FaasError::InvocationFailed(e.to_string())
            })?;

        let duration = start.elapsed();

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(FaasError::FunctionError(format!("HTTP {} - {}", status, body)));
        }

        let faas_response: FaasResponse = response
            .json()
            .await
            .map_err(|e| FaasError::SerializationError(e.to_string()))?;

        info!(
            job_id = job_id,
            duration_ms = duration.as_millis(),
            "Function invocation successful"
        );

        Ok(faas_response.into())
    }

    async fn invoke_with_metrics(
        &self,
        job_call: JobCall,
    ) -> Result<(JobResult, FaasMetrics), FaasError> {
        let start = Instant::now();
        let result = self.invoke(job_call).await?;
        let total_duration = start.elapsed();

        // DigitalOcean Functions have ~200ms typical cold start
        let cold_start = total_duration.as_millis() > 800;

        let metrics = FaasMetrics {
            total_duration_ms: total_duration.as_millis() as u64,
            execution_duration_ms: total_duration.as_millis() as u64,
            cold_start,
            memory_used_mb: None,
            billed_duration_ms: ((total_duration.as_millis() as u64 + 99) / 100) * 100,
        };

        Ok((result, metrics))
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
            "Deploying DigitalOcean Function"
        );

        // Package binary for deployment
        let zip_package = crate::utils::create_lambda_package(binary)?;
        use base64::Engine;
        let base64_package = base64::engine::general_purpose::STANDARD.encode(&zip_package);

        // Create function specification
        let function_spec = FunctionSpec {
            name: function_name.clone(),
            runtime: "go:1.21".to_string(), // DigitalOcean supports Go runtime for custom binaries
            limits: Limits {
                memory: config.memory_mb,
                timeout: config.timeout_secs * 1000, // Convert to milliseconds
            },
            binary: Some(BinarySpec {
                data: base64_package,
                main: "bootstrap".to_string(),
            }),
            environment: config.env_vars.clone(),
        };

        // Try to update existing function
        let update_url = self.api_endpoint(&format!("triggers/{}", function_name));
        let update_response = self
            .client
            .put(&update_url)
            .bearer_auth(&self.api_token)
            .json(&function_spec)
            .send()
            .await;

        let function_url = if update_response.is_ok()
            && update_response.as_ref().unwrap().status().is_success()
        {
            info!(function = %function_name, "Updated existing function");
            update_response.unwrap()
                .json::<FunctionResponse>()
                .await
                .map_err(|e| FaasError::SerializationError(e.to_string()))?
                .trigger
                .url
        } else {
            // Create new function
            debug!(function = %function_name, "Creating new function");

            let create_url = self.api_endpoint("triggers");
            let response = self
                .client
                .post(&create_url)
                .bearer_auth(&self.api_token)
                .json(&function_spec)
                .send()
                .await
                .map_err(|e| FaasError::InfrastructureError(format!("Failed to create function: {}", e)))?;

            if !response.status().is_success() {
                let error_text = response.text().await.unwrap_or_default();
                return Err(FaasError::InfrastructureError(format!(
                    "Failed to create function: {}",
                    error_text
                )));
            }

            let data: FunctionResponse = response
                .json()
                .await
                .map_err(|e| FaasError::SerializationError(e.to_string()))?;

            info!(function = %function_name, "Created new function");
            data.trigger.url
        };

        Ok(FaasDeployment {
            function_id: function_name.clone(),
            job_id,
            endpoint: function_url,
            cold_start_ms: Some(200), // Typical DigitalOcean cold start
            memory_mb: config.memory_mb,
            timeout_secs: config.timeout_secs,
        })
    }

    async fn health_check(&self, job_id: u32) -> Result<bool, FaasError> {
        let function_name = self.function_name(job_id);
        let url = self.api_endpoint(&format!("triggers/{}", function_name));

        self.client
            .get(&url)
            .bearer_auth(&self.api_token)
            .send()
            .await
            .map(|r| r.status().is_success())
            .map_err(|e| FaasError::InfrastructureError(format!("Health check failed: {}", e)))
    }

    async fn warm(&self, job_id: u32) -> Result<(), FaasError> {
        debug!(job_id, "Warming DigitalOcean Function");

        // Create minimal invocation to warm the function
        let warm_call = JobCall::new(job_id as u8, bytes::Bytes::from_static(b"{}"));
        let _ = self.invoke(warm_call).await;

        Ok(())
    }

    async fn get_deployment(&self, job_id: u32) -> Result<FaasDeployment, FaasError> {
        let function_name = self.function_name(job_id);
        let url = self.api_endpoint(&format!("triggers/{}", function_name));

        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.api_token)
            .send()
            .await
            .map_err(|e| FaasError::InfrastructureError(format!("Failed to get function: {}", e)))?;

        if !response.status().is_success() {
            return Err(FaasError::InfrastructureError(format!(
                "Function not found: {}",
                function_name
            )));
        }

        let data: FunctionResponse = response
            .json()
            .await
            .map_err(|e| FaasError::SerializationError(e.to_string()))?;

        Ok(FaasDeployment {
            function_id: function_name.clone(),
            job_id,
            endpoint: data.trigger.url,
            cold_start_ms: Some(200),
            memory_mb: data.trigger.limits.memory,
            timeout_secs: data.trigger.limits.timeout / 1000, // Convert from milliseconds
        })
    }

    async fn undeploy_job(&self, job_id: u32) -> Result<(), FaasError> {
        let function_name = self.function_name(job_id);

        info!(function = %function_name, "Deleting DigitalOcean Function");

        let url = self.api_endpoint(&format!("triggers/{}", function_name));

        self.client
            .delete(&url)
            .bearer_auth(&self.api_token)
            .send()
            .await
            .map_err(|e| FaasError::InfrastructureError(format!("Failed to delete function: {}", e)))?;

        Ok(())
    }

    fn provider_name(&self) -> &str {
        "DigitalOcean Functions"
    }
}

impl DigitalOceanExecutor {
    /// Get the HTTP URL for invoking a function
    async fn get_function_url(&self, job_id: u32) -> Result<String, FaasError> {
        let deployment = self.get_deployment(job_id).await?;
        Ok(deployment.endpoint)
    }
}

// DigitalOcean API types

#[derive(Debug, Serialize)]
struct CreateNamespaceRequest {
    label: String,
    region: String,
}

#[derive(Debug, Deserialize)]
struct NamespaceListResponse {
    namespaces: Vec<Namespace>,
}

#[derive(Debug, Deserialize)]
struct NamespaceResponse {
    namespace: Namespace,
}

#[derive(Debug, Deserialize)]
struct Namespace {
    id: String,
    label: String,
}

#[derive(Debug, Serialize)]
struct FunctionSpec {
    name: String,
    runtime: String,
    limits: Limits,
    #[serde(skip_serializing_if = "Option::is_none")]
    binary: Option<BinarySpec>,
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]
    environment: std::collections::HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Limits {
    memory: u32,
    timeout: u32,
}

#[derive(Debug, Serialize)]
struct BinarySpec {
    data: String, // Base64-encoded binary
    main: String, // Entry point
}

#[derive(Debug, Deserialize)]
struct FunctionResponse {
    trigger: TriggerInfo,
}

#[derive(Debug, Deserialize)]
struct TriggerInfo {
    url: String,
    limits: Limits,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "Requires DigitalOcean API token"]
    async fn test_digitalocean_executor_creation() {
        let token = std::env::var("DIGITALOCEAN_TOKEN").expect("DIGITALOCEAN_TOKEN not set");
        let executor = DigitalOceanExecutor::new(token, "nyc1").await;

        assert!(executor.is_ok());
        let exec = executor.unwrap();
        assert_eq!(exec.provider_name(), "DigitalOcean Functions");
    }

    #[test]
    fn test_function_naming() {
        let executor = DigitalOceanExecutor {
            api_token: "test-token".to_string(),
            namespace_id: "test-namespace".to_string(),
            region: "nyc1".to_string(),
            function_prefix: "blueprint".to_string(),
            client: Client::new(),
        };

        assert_eq!(executor.function_name(0), "blueprint-job-0");
        assert_eq!(executor.function_name(42), "blueprint-job-42");
    }

    #[test]
    fn test_api_endpoint() {
        let executor = DigitalOceanExecutor {
            api_token: "test-token".to_string(),
            namespace_id: "ns-123".to_string(),
            region: "nyc1".to_string(),
            function_prefix: "blueprint".to_string(),
            client: Client::new(),
        };

        assert_eq!(
            executor.api_endpoint("triggers"),
            "https://api.digitalocean.com/v2/functions/namespaces/ns-123/triggers"
        );

        assert_eq!(
            executor.api_endpoint("/triggers/test"),
            "https://api.digitalocean.com/v2/functions/namespaces/ns-123/triggers/test"
        );
    }
}
