//! Azure Functions FaaS integration
//!
//! This module provides full integration with Azure Functions for executing blueprint jobs.

use super::*;
use azure_core::auth::TokenCredential;
use azure_identity::{DefaultAzureCredential, TokenCredentialOptions};
use blueprint_core::{JobCall, JobResult};
use reqwest::Client;
use serde::Deserialize;
use std::sync::Arc;
use std::time::Instant;
use tracing::{debug, info, warn};

/// Azure Functions executor for blueprint jobs
///
/// Integrates with Azure Functions REST API for deploying and invoking serverless functions.
///
/// # Authentication
///
/// Uses Azure Default Credentials (Environment, Managed Identity, or Azure CLI).
/// Set these environment variables:
/// - `AZURE_TENANT_ID` - Your Azure AD tenant ID
/// - `AZURE_CLIENT_ID` - Service principal client ID
/// - `AZURE_CLIENT_SECRET` - Service principal secret
///
/// Or use Azure CLI: `az login`
///
/// # Example
///
/// ```rust,ignore
/// let executor = AzureFunctionExecutor::new("my-subscription-id", "eastus").await?;
///
/// BlueprintRunner::builder(config, env)
///     .with_faas_executor(0, executor)
///     .run().await
/// ```
#[derive(Debug, Clone)]
pub struct AzureFunctionExecutor {
    subscription_id: String,
    region: String,
    resource_group: String,
    function_app_name: String,
    client: Client,
    credential: Arc<DefaultAzureCredential>,
}

impl AzureFunctionExecutor {
    /// Create a new Azure Functions executor
    ///
    /// # Arguments
    ///
    /// * `subscription_id` - Azure subscription ID
    /// * `region` - Azure region (e.g., "eastus", "westus2")
    ///
    /// # Authentication
    ///
    /// Requires Azure credentials via environment variables or Azure CLI.
    /// The service principal needs Contributor role on the subscription.
    pub async fn new(
        subscription_id: impl Into<String>,
        region: impl Into<String>,
    ) -> Result<Self, FaasError> {
        let subscription_id = subscription_id.into();
        let region = region.into();

        debug!(
            subscription_id = %subscription_id,
            region = %region,
            "Creating Azure Functions executor"
        );

        let credential = DefaultAzureCredential::create(TokenCredentialOptions::default())
            .map_err(|e| FaasError::InfrastructureError(format!("Failed to create Azure credentials: {}", e)))?;

        Ok(Self {
            subscription_id,
            region: region.clone(),
            resource_group: format!("blueprint-rg-{}", region),
            function_app_name: format!("blueprint-functions-{}", region),
            client: Client::new(),
            credential: Arc::new(credential),
        })
    }

    /// Set the resource group name (default: "blueprint-rg-{region}")
    #[must_use]
    pub fn with_resource_group(mut self, resource_group: impl Into<String>) -> Self {
        self.resource_group = resource_group.into();
        self
    }

    /// Set the function app name (default: "blueprint-functions-{region}")
    #[must_use]
    pub fn with_app_name(mut self, app_name: impl Into<String>) -> Self {
        self.function_app_name = app_name.into();
        self
    }

    fn function_name(&self, job_id: u32) -> String {
        format!("job{}", job_id)
    }

    /// Get an authenticated access token for Azure ARM API calls
    async fn get_access_token(&self) -> Result<String, FaasError> {
        let token = self
            .credential
            .get_token(&["https://management.azure.com/.default"])
            .await
            .map_err(|e| {
                FaasError::InfrastructureError(format!(
                    "Failed to get Azure access token: {}. \
                    Set AZURE_TENANT_ID, AZURE_CLIENT_ID, AZURE_CLIENT_SECRET or use 'az login'.",
                    e
                ))
            })?;

        Ok(token.token.secret().to_string())
    }

    /// Build Azure Resource Manager API endpoint
    fn arm_endpoint(&self, path: &str) -> String {
        format!(
            "https://management.azure.com{}?api-version=2022-03-01",
            path.trim_start_matches('/')
        )
    }

    /// Get the HTTP trigger URL for a function
    async fn get_function_url(&self, job_id: u32) -> Result<String, FaasError> {
        let function_name = self.function_name(job_id);

        // Get function keys to construct invoke URL
        let token = self.get_access_token().await?;

        let keys_url = self.arm_endpoint(&format!(
            "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Web/sites/{}/functions/{}/keys",
            self.subscription_id, self.resource_group, self.function_app_name, function_name
        ));

        let response = self
            .client
            .post(&keys_url.replace("?api-version", "/listKeys?api-version"))
            .bearer_auth(&token)
            .send()
            .await
            .map_err(|e| {
                FaasError::InfrastructureError(format!("Failed to get function keys: {}", e))
            })?;

        if !response.status().is_success() {
            return Err(FaasError::InfrastructureError(format!(
                "Function not found or not deployed: {}",
                function_name
            )));
        }

        let keys_info: FunctionKeysInfo = response
            .json()
            .await
            .map_err(|e| FaasError::SerializationError(e.to_string()))?;

        let default_key = keys_info.default.ok_or_else(|| {
            FaasError::InfrastructureError("No default function key available".into())
        })?;

        // Construct function URL with key
        Ok(format!(
            "https://{}.azurewebsites.net/api/{}?code={}",
            self.function_app_name, function_name, default_key
        ))
    }

    /// Invoke function via HTTP trigger
    async fn invoke_http_trigger(&self, job_call: JobCall) -> Result<JobResult, FaasError> {
        let job_id: u32 = job_call.job_id().into();
        let function_name = self.function_name(job_id);

        // Get function URL with auth key
        let function_url = self.get_function_url(job_id).await?;

        debug!(
            job_id = job_id,
            function = %function_name,
            "Invoking Azure Function via HTTP"
        );

        // Convert JobCall to FaasPayload
        let payload: FaasPayload = job_call.into();

        let start = Instant::now();

        // Invoke via HTTP POST (no Bearer token needed, URL contains function key)
        let response = self
            .client
            .post(&function_url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                warn!(error = %e, "Azure Function HTTP invocation failed");
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
            "Azure Function invocation successful"
        );

        Ok(faas_response.into())
    }
}

#[async_trait::async_trait]
impl FaasExecutor for AzureFunctionExecutor {
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

        // Azure Functions cold start detection via duration heuristic
        let cold_start = total_duration.as_millis() > 800;

        let metrics = FaasMetrics {
            total_duration_ms: total_duration.as_millis() as u64,
            execution_duration_ms: total_duration.as_millis() as u64,
            cold_start,
            memory_used_mb: None,
            billed_duration_ms: total_duration.as_millis() as u64, // Azure bills per ms
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
            "Deploying Azure Function"
        );

        // Package binary for Azure Functions (zip format)
        let zip_package = crate::utils::create_lambda_package(binary)?;

        // Ensure resource group exists
        self.ensure_resource_group().await?;

        // Ensure function app exists
        self.ensure_function_app(config).await?;

        // Upload function code via ZipDeploy API
        self.upload_function_code(job_id, &zip_package).await?;

        // Create function.json for the function
        self.create_function_config(job_id, config).await?;

        // Wait for deployment to complete
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        Ok(FaasDeployment {
            function_id: format!("{}/{}", self.function_app_name, function_name),
            job_id,
            endpoint: format!(
                "https://{}.azurewebsites.net/api/{}",
                self.function_app_name, function_name
            ),
            cold_start_ms: Some(600), // Typical Azure Functions cold start
            memory_mb: config.memory_mb,
            timeout_secs: config.timeout_secs,
        })
    }

    async fn health_check(&self, job_id: u32) -> Result<bool, FaasError> {
        let function_name = self.function_name(job_id);
        let token = self.get_access_token().await?;

        let url = self.arm_endpoint(&format!(
            "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Web/sites/{}/functions/{}",
            self.subscription_id, self.resource_group, self.function_app_name, function_name
        ));

        self.client
            .get(&url)
            .bearer_auth(&token)
            .send()
            .await
            .map(|r| r.status().is_success())
            .map_err(|e| FaasError::InfrastructureError(format!("Health check failed: {}", e)))
    }

    async fn warm(&self, job_id: u32) -> Result<(), FaasError> {
        debug!(job_id, "Warming Azure Function");

        // Create a minimal JobCall for warming
        let warm_call = JobCall::new(job_id as u8, bytes::Bytes::from_static(b"{}"));

        // Invoke the function (ignore result)
        let _ = self.invoke(warm_call).await;

        Ok(())
    }

    async fn get_deployment(&self, job_id: u32) -> Result<FaasDeployment, FaasError> {
        let function_name = self.function_name(job_id);
        let token = self.get_access_token().await?;

        let url = self.arm_endpoint(&format!(
            "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Web/sites/{}/functions/{}",
            self.subscription_id, self.resource_group, self.function_app_name, function_name
        ));

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
                function_name
            )));
        }

        Ok(FaasDeployment {
            function_id: format!("{}/{}", self.function_app_name, function_name),
            job_id,
            endpoint: format!(
                "https://{}.azurewebsites.net/api/{}",
                self.function_app_name, function_name
            ),
            cold_start_ms: Some(600),
            memory_mb: 512, // Default
            timeout_secs: 300, // Default
        })
    }

    async fn undeploy_job(&self, job_id: u32) -> Result<(), FaasError> {
        let function_name = self.function_name(job_id);

        info!(function = %function_name, "Deleting Azure Function");

        let token = self.get_access_token().await?;

        let url = self.arm_endpoint(&format!(
            "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Web/sites/{}/functions/{}",
            self.subscription_id, self.resource_group, self.function_app_name, function_name
        ));

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
        "Azure Functions"
    }
}

impl AzureFunctionExecutor {
    /// Ensure the resource group exists, create if not
    async fn ensure_resource_group(&self) -> Result<(), FaasError> {
        let token = self.get_access_token().await?;

        let url = self.arm_endpoint(&format!(
            "/subscriptions/{}/resourceGroups/{}",
            self.subscription_id, self.resource_group
        ));

        // Check if exists
        let exists = self
            .client
            .get(&url)
            .bearer_auth(&token)
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false);

        if !exists {
            debug!(
                resource_group = %self.resource_group,
                "Creating resource group"
            );

            // Create resource group
            let create_body = serde_json::json!({
                "location": self.region
            });

            self.client
                .put(&url)
                .bearer_auth(&token)
                .json(&create_body)
                .send()
                .await
                .map_err(|e| {
                    FaasError::InfrastructureError(format!("Failed to create resource group: {}", e))
                })?;
        }

        Ok(())
    }

    /// Ensure the function app exists, create if not
    async fn ensure_function_app(&self, config: &FaasConfig) -> Result<(), FaasError> {
        let token = self.get_access_token().await?;

        let url = self.arm_endpoint(&format!(
            "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Web/sites/{}",
            self.subscription_id, self.resource_group, self.function_app_name
        ));

        // Check if exists
        let exists = self
            .client
            .get(&url)
            .bearer_auth(&token)
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false);

        if !exists {
            debug!(
                function_app = %self.function_app_name,
                "Creating function app"
            );

            // Create function app
            let create_body = serde_json::json!({
                "location": self.region,
                "kind": "functionapp",
                "properties": {
                    "reserved": true, // Linux
                    "siteConfig": {
                        "linuxFxVersion": "CUSTOM",
                        "appSettings": config.env_vars.iter().map(|(k, v)| {
                            serde_json::json!({
                                "name": k,
                                "value": v
                            })
                        }).collect::<Vec<_>>()
                    }
                }
            });

            self.client
                .put(&url)
                .bearer_auth(&token)
                .json(&create_body)
                .send()
                .await
                .map_err(|e| {
                    FaasError::InfrastructureError(format!("Failed to create function app: {}", e))
                })?;

            // Wait for function app to be ready
            tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
        }

        Ok(())
    }

    /// Upload function code via ZipDeploy
    async fn upload_function_code(&self, _job_id: u32, zip_data: &[u8]) -> Result<(), FaasError> {
        let token = self.get_access_token().await?;

        let url = format!(
            "https://{}.scm.azurewebsites.net/api/zipdeploy",
            self.function_app_name
        );

        debug!(
            size_bytes = zip_data.len(),
            "Uploading function code via ZipDeploy"
        );

        self.client
            .post(&url)
            .bearer_auth(&token)
            .header("Content-Type", "application/zip")
            .body(zip_data.to_vec())
            .send()
            .await
            .map_err(|e| {
                FaasError::InfrastructureError(format!("Failed to upload function code: {}", e))
            })?;

        Ok(())
    }

    /// Create function.json configuration file
    async fn create_function_config(&self, job_id: u32, _config: &FaasConfig) -> Result<(), FaasError> {
        let function_name = self.function_name(job_id);
        let token = self.get_access_token().await?;

        // Create function.json for HTTP trigger
        let function_json = serde_json::json!({
            "bindings": [
                {
                    "authLevel": "function",
                    "type": "httpTrigger",
                    "direction": "in",
                    "name": "req",
                    "methods": ["post"]
                },
                {
                    "type": "http",
                    "direction": "out",
                    "name": "res"
                }
            ]
        });

        let url = format!(
            "https://{}.scm.azurewebsites.net/api/vfs/site/wwwroot/{}/function.json",
            self.function_app_name, function_name
        );

        self.client
            .put(&url)
            .bearer_auth(&token)
            .json(&function_json)
            .send()
            .await
            .map_err(|e| {
                FaasError::InfrastructureError(format!("Failed to create function config: {}", e))
            })?;

        Ok(())
    }
}

// Azure ARM API types

#[derive(Debug, Deserialize)]
struct FunctionKeysInfo {
    default: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "Requires Azure credentials"]
    async fn test_azure_executor_creation() {
        let executor = AzureFunctionExecutor::new("test-subscription", "eastus").await;

        assert!(executor.is_ok());
        let exec = executor.unwrap();
        assert_eq!(exec.provider_name(), "Azure Functions");
    }

    #[test]
    fn test_function_naming() {
        // Create a credential without requiring actual authentication
        let credential = DefaultAzureCredential::create(TokenCredentialOptions::default())
            .expect("Failed to create test credential (this is expected in test environment)");

        let executor = AzureFunctionExecutor {
            subscription_id: "test-subscription".to_string(),
            region: "eastus".to_string(),
            resource_group: "test-rg".to_string(),
            function_app_name: "test-app".to_string(),
            client: Client::new(),
            credential: Arc::new(credential),
        };

        assert_eq!(executor.function_name(0), "job0");
        assert_eq!(executor.function_name(42), "job42");
    }
}
