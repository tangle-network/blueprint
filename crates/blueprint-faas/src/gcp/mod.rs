//! Google Cloud Functions FaaS integration
//!
//! This module provides full integration with Google Cloud Functions v2 for executing blueprint jobs.

use super::*;
use blueprint_core::{JobCall, JobResult};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use tracing::{debug, info, warn};

/// GCP Cloud Functions executor for blueprint jobs
///
/// Integrates with Cloud Functions v2 API for deploying and invoking serverless functions.
///
/// # Authentication
///
/// Uses Application Default Credentials (ADC) via service account key file or
/// workload identity. Set `GOOGLE_APPLICATION_CREDENTIALS` environment variable
/// to point to your service account JSON key file.
///
/// # Example
///
/// ```rust,ignore
/// let executor = CloudFunctionExecutor::new("my-project", "us-central1").await?;
///
/// BlueprintRunner::builder(config, env)
///     .with_faas_executor(0, executor)
///     .run().await
/// ```
#[derive(Debug, Clone)]
pub struct CloudFunctionExecutor {
    project_id: String,
    region: String,
    function_prefix: String,
    client: Client,
    token_manager: std::sync::Arc<tokio::sync::Mutex<Option<std::sync::Arc<gcp_auth::Token>>>>,
}

impl CloudFunctionExecutor {
    /// Create a new Cloud Functions executor
    ///
    /// # Arguments
    ///
    /// * `project_id` - GCP project ID
    /// * `region` - GCP region (e.g., "us-central1")
    ///
    /// # Authentication
    ///
    /// Requires `GOOGLE_APPLICATION_CREDENTIALS` environment variable pointing to
    /// service account JSON key with Cloud Functions Admin role.
    pub async fn new(
        project_id: impl Into<String>,
        region: impl Into<String>,
    ) -> Result<Self, FaasError> {
        let project_id = project_id.into();
        let region = region.into();

        debug!(
            project_id = %project_id,
            region = %region,
            "Creating GCP Cloud Functions executor"
        );

        Ok(Self {
            project_id,
            region,
            function_prefix: "blueprint".to_string(),
            client: Client::new(),
            token_manager: std::sync::Arc::new(tokio::sync::Mutex::new(None)),
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

    fn function_full_name(&self, job_id: u32) -> String {
        format!(
            "projects/{}/locations/{}/functions/{}",
            self.project_id,
            self.region,
            self.function_name(job_id)
        )
    }

    /// Get an authenticated access token for GCP API calls
    async fn get_access_token(&self) -> Result<String, FaasError> {
        let mut token_guard = self.token_manager.lock().await;

        // Check if we have a valid cached token
        if let Some(token) = token_guard.as_ref() {
            if !token.has_expired() {
                return Ok(token.as_str().to_string());
            }
        }

        // Token expired or doesn't exist, get a new one
        debug!("Fetching new GCP access token");

        let scopes = &["https://www.googleapis.com/auth/cloud-platform"];

        // Get authentication provider
        let auth = gcp_auth::provider()
            .await
            .map_err(|e| {
                FaasError::InfrastructureError(format!(
                    "Failed to initialize GCP auth: {}. \
                    Set GOOGLE_APPLICATION_CREDENTIALS environment variable.",
                    e
                ))
            })?;

        // Get token for the required scopes
        let token = auth.token(scopes)
            .await
            .map_err(|e| {
                FaasError::InfrastructureError(format!("Failed to get GCP access token: {}", e))
            })?;

        let token_str = token.as_str().to_string();
        *token_guard = Some(token);

        Ok(token_str)
    }

    /// Build the Cloud Functions API endpoint URL
    fn api_endpoint(&self, path: &str) -> String {
        format!(
            "https://cloudfunctions.googleapis.com/v2/{}",
            path.trim_start_matches('/')
        )
    }

    /// Invoke function via HTTP trigger
    async fn invoke_http_trigger(&self, job_call: JobCall) -> Result<JobResult, FaasError> {
        let job_id: u32 = job_call.job_id().into();
        let function_name = self.function_name(job_id);

        // Get function URL
        let function_url = self.get_function_url(job_id).await?;

        debug!(
            job_id = job_id,
            function = %function_name,
            url = %function_url,
            "Invoking Cloud Function via HTTP"
        );

        // Convert JobCall to FaasPayload
        let payload: FaasPayload = job_call.into();

        let start = Instant::now();

        // Invoke via HTTP POST
        let token = self.get_access_token().await?;

        let response = self
            .client
            .post(&function_url)
            .bearer_auth(&token)
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                warn!(error = %e, "Cloud Function HTTP invocation failed");
                FaasError::InvocationFailed(e.to_string())
            })?;

        let duration = start.elapsed();

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(FaasError::FunctionError(format!(
                "HTTP {} - {}",
                status, body
            )));
        }

        let faas_response: FaasResponse = response
            .json()
            .await
            .map_err(|e| FaasError::SerializationError(e.to_string()))?;

        info!(
            job_id = job_id,
            duration_ms = duration.as_millis(),
            "Cloud Function invocation successful"
        );

        Ok(faas_response.into())
    }

    /// Get the HTTP URL for a deployed function
    async fn get_function_url(&self, job_id: u32) -> Result<String, FaasError> {
        let full_name = self.function_full_name(job_id);
        let token = self.get_access_token().await?;

        let url = self.api_endpoint(&format!("{}", full_name));

        let response = self
            .client
            .get(&url)
            .bearer_auth(&token)
            .send()
            .await
            .map_err(|e| {
                FaasError::InfrastructureError(format!("Failed to get function details: {}", e))
            })?;

        if !response.status().is_success() {
            return Err(FaasError::InfrastructureError(format!(
                "Function not found: {}",
                full_name
            )));
        }

        let function_info: CloudFunctionInfo = response
            .json()
            .await
            .map_err(|e| FaasError::SerializationError(e.to_string()))?;

        function_info
            .service_config
            .and_then(|sc| sc.uri)
            .ok_or_else(|| {
                FaasError::InfrastructureError("Function has no HTTP trigger URL".into())
            })
    }
}

#[async_trait::async_trait]
impl FaasExecutor for CloudFunctionExecutor {
    async fn invoke(&self, job_call: JobCall) -> Result<JobResult, FaasError> {
        self.invoke_http_trigger(job_call).await
    }

    async fn invoke_with_metrics(
        &self,
        job_call: JobCall,
    ) -> Result<(JobResult, FaasMetrics), FaasError> {
        let start = Instant::now();
        let result = self.invoke(job_call).await?;
        let total_duration = start.elapsed();

        // Cloud Functions doesn't expose cold start info via response headers,
        // so we estimate based on duration
        let cold_start = total_duration.as_millis() > 1000;

        let metrics = FaasMetrics {
            total_duration_ms: total_duration.as_millis() as u64,
            execution_duration_ms: total_duration.as_millis() as u64,
            cold_start,
            memory_used_mb: None,
            billed_duration_ms: ((total_duration.as_millis() as u64 + 99) / 100) * 100, // Round up to nearest 100ms
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
        let full_name = self.function_full_name(job_id);

        info!(
            job_id,
            function = %function_name,
            memory_mb = config.memory_mb,
            timeout_secs = config.timeout_secs,
            "Deploying Cloud Function"
        );

        // Package binary for Cloud Functions (zip format)
        let zip_package = crate::utils::create_lambda_package(binary)?;

        // Upload to Cloud Storage first (required for Cloud Functions v2)
        let _storage_url = self.upload_to_storage(job_id, &zip_package).await?;

        // Create or update the function
        let token = self.get_access_token().await?;

        let function_spec = CloudFunctionSpec {
            name: full_name.clone(),
            description: Some(format!("Blueprint job {}", job_id)),
            build_config: Some(BuildConfig {
                runtime: "go122".to_string(), // Using Go runtime for custom binaries
                entry_point: "bootstrap".to_string(),
                source: Source {
                    storage_source: Some(StorageSource {
                        bucket: format!("{}-blueprint-functions", self.project_id),
                        object: format!("job-{}.zip", job_id),
                    }),
                },
            }),
            service_config: Some(ServiceConfig {
                available_memory: Some(format!("{}Mi", config.memory_mb)),
                timeout_seconds: Some(config.timeout_secs as i32),
                environment_variables: Some(config.env_vars.clone()),
                max_instance_count: config.max_concurrency.map(|c| c as i32),
                uri: None, // Will be populated by GCP after creation
            }),
        };

        // Try to update existing function first
        let update_url = self.api_endpoint(&full_name);
        let update_response = self
            .client
            .patch(&update_url)
            .bearer_auth(&token)
            .query(&[("updateMask", "buildConfig,serviceConfig")])
            .json(&function_spec)
            .send()
            .await;

        if update_response.is_ok()
            && update_response
                .as_ref()
                .unwrap()
                .status()
                .is_success()
        {
            info!(function = %function_name, "Updated existing Cloud Function");
        } else {
            // Function doesn't exist, create it
            debug!(function = %function_name, "Creating new Cloud Function");

            let create_url = self.api_endpoint(&format!(
                "projects/{}/locations/{}/functions",
                self.project_id, self.region
            ));

            self.client
                .post(&create_url)
                .bearer_auth(&token)
                .query(&[("functionId", function_name.as_str())])
                .json(&function_spec)
                .send()
                .await
                .map_err(|e| {
                    FaasError::InfrastructureError(format!("Failed to create function: {}", e))
                })?;

            info!(function = %function_name, "Created new Cloud Function");
        }

        // Wait for deployment to complete (operations API)
        // In production, you'd poll the operation status
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        Ok(FaasDeployment {
            function_id: full_name.clone(),
            job_id,
            endpoint: format!(
                "https://{}-{}.cloudfunctions.net/{}",
                self.region, self.project_id, function_name
            ),
            cold_start_ms: Some(500), // Typical Cloud Functions cold start
            memory_mb: config.memory_mb,
            timeout_secs: config.timeout_secs,
        })
    }

    async fn health_check(&self, job_id: u32) -> Result<bool, FaasError> {
        let full_name = self.function_full_name(job_id);
        let token = self.get_access_token().await?;

        let url = self.api_endpoint(&full_name);

        self.client
            .get(&url)
            .bearer_auth(&token)
            .send()
            .await
            .map(|r| r.status().is_success())
            .map_err(|e| FaasError::InfrastructureError(format!("Health check failed: {}", e)))
    }

    async fn warm(&self, job_id: u32) -> Result<(), FaasError> {
        debug!(job_id, "Warming Cloud Function");

        // Create a minimal JobCall for warming
        let warm_call = JobCall::new(job_id as u8, bytes::Bytes::from_static(b"{}"));

        // Invoke the function (ignore result)
        let _ = self.invoke(warm_call).await;

        Ok(())
    }

    async fn get_deployment(&self, job_id: u32) -> Result<FaasDeployment, FaasError> {
        let full_name = self.function_full_name(job_id);
        let token = self.get_access_token().await?;

        let url = self.api_endpoint(&full_name);

        let response = self
            .client
            .get(&url)
            .bearer_auth(&token)
            .send()
            .await
            .map_err(|e| {
                FaasError::InfrastructureError(format!("Failed to get function: {}", e))
            })?;

        if !response.status().is_success() {
            return Err(FaasError::InfrastructureError(format!(
                "Function not found: {}",
                full_name
            )));
        }

        let function_info: CloudFunctionInfo = response
            .json()
            .await
            .map_err(|e| FaasError::SerializationError(e.to_string()))?;

        let service_config = function_info
            .service_config
            .ok_or_else(|| FaasError::InfrastructureError("No service config".into()))?;

        Ok(FaasDeployment {
            function_id: full_name.clone(),
            job_id,
            endpoint: service_config
                .uri
                .unwrap_or_else(|| format!("https://{}.cloudfunctions.net", full_name)),
            cold_start_ms: Some(500),
            memory_mb: service_config
                .available_memory
                .and_then(|m| m.trim_end_matches("Mi").parse().ok())
                .unwrap_or(512),
            timeout_secs: service_config.timeout_seconds.unwrap_or(300) as u32,
        })
    }

    async fn undeploy_job(&self, job_id: u32) -> Result<(), FaasError> {
        let full_name = self.function_full_name(job_id);
        let function_name = self.function_name(job_id);

        info!(function = %function_name, "Deleting Cloud Function");

        let token = self.get_access_token().await?;
        let url = self.api_endpoint(&full_name);

        self.client
            .delete(&url)
            .bearer_auth(&token)
            .send()
            .await
            .map_err(|e| {
                FaasError::InfrastructureError(format!("Failed to delete function: {}", e))
            })?;

        Ok(())
    }

    fn provider_name(&self) -> &str {
        "GCP Cloud Functions"
    }
}

impl CloudFunctionExecutor {
    /// Upload function code to Cloud Storage
    ///
    /// Cloud Functions v2 requires function code to be in Cloud Storage.
    async fn upload_to_storage(&self, job_id: u32, zip_data: &[u8]) -> Result<String, FaasError> {
        let bucket = format!("{}-blueprint-functions", self.project_id);
        let object_name = format!("job-{}.zip", job_id);

        debug!(
            bucket = %bucket,
            object = %object_name,
            size_bytes = zip_data.len(),
            "Uploading function code to Cloud Storage"
        );

        let token = self.get_access_token().await?;

        // Upload to Cloud Storage using resumable upload API
        let upload_url = format!(
            "https://storage.googleapis.com/upload/storage/v1/b/{}/o?uploadType=media&name={}",
            bucket, object_name
        );

        self.client
            .post(&upload_url)
            .bearer_auth(&token)
            .header("Content-Type", "application/zip")
            .body(zip_data.to_vec())
            .send()
            .await
            .map_err(|e| {
                FaasError::InfrastructureError(format!(
                    "Failed to upload to Cloud Storage: {}. \
                    Ensure bucket '{}' exists and service account has write permissions.",
                    e, bucket
                ))
            })?;

        Ok(format!("gs://{}/{}", bucket, object_name))
    }
}

// GCP Cloud Functions API types

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CloudFunctionSpec {
    name: String,
    description: Option<String>,
    build_config: Option<BuildConfig>,
    service_config: Option<ServiceConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BuildConfig {
    runtime: String,
    entry_point: String,
    source: Source,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Source {
    storage_source: Option<StorageSource>,
}

#[derive(Debug, Serialize, Deserialize)]
struct StorageSource {
    bucket: String,
    object: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ServiceConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    available_memory: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_seconds: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_variables: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_instance_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<String>,
}

/// Cloud Function information returned by GCP API
///
/// This struct deserializes responses from Cloud Functions API.
/// Some fields are included for API compatibility but may not be used directly.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CloudFunctionInfo {
    /// Full resource name of the function (required by API, used for validation)
    #[allow(dead_code)]
    name: String,
    /// Service configuration containing runtime settings and HTTP trigger URL
    service_config: Option<ServiceConfig>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "Requires GCP credentials"]
    async fn test_gcp_executor_creation() {
        let executor = CloudFunctionExecutor::new("test-project", "us-central1").await;

        assert!(executor.is_ok());
        let exec = executor.unwrap();
        assert_eq!(exec.provider_name(), "GCP Cloud Functions");
    }

    #[test]
    fn test_function_naming() {
        let executor = CloudFunctionExecutor {
            project_id: "test-project".to_string(),
            region: "us-central1".to_string(),
            function_prefix: "blueprint".to_string(),
            client: Client::new(),
            token_manager: std::sync::Arc::new(tokio::sync::Mutex::new(None)),
        };

        assert_eq!(executor.function_name(0), "blueprint-job-0");
        assert_eq!(executor.function_name(42), "blueprint-job-42");

        let full_name = executor.function_full_name(0);
        assert_eq!(
            full_name,
            "projects/test-project/locations/us-central1/functions/blueprint-job-0"
        );
    }
}
