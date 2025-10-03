//! FaaS execution abstraction for BlueprintRunner
//!
//! This module provides a trait-based interface for delegating job execution
//! to Function-as-a-Service platforms (AWS Lambda, GCP Cloud Functions, etc.)
//! or custom serverless runtimes.

use blueprint_core::{JobCall, JobResult};
use std::fmt;
use std::sync::Arc;
use thiserror::Error;

/// Errors that can occur during FaaS execution
#[derive(Debug, Error)]
pub enum FaasError {
    /// The function invocation failed
    #[error("Function invocation failed: {0}")]
    InvocationFailed(String),

    /// The function timed out
    #[error("Function execution timed out after {0:?}")]
    Timeout(std::time::Duration),

    /// Function returned an error
    #[error("Function error: {0}")]
    FunctionError(String),

    /// Serialization/deserialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Network or infrastructure error
    #[error("Infrastructure error: {0}")]
    InfrastructureError(String),

    /// Cold start took too long
    #[error("Cold start latency exceeded threshold: {0:?}")]
    ColdStartLatency(std::time::Duration),

    /// Other errors
    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

/// Information about a deployed FaaS function
#[derive(Debug, Clone)]
pub struct FaasDeployment {
    /// Unique identifier for the deployed function
    pub function_id: String,

    /// The job ID this function handles
    pub job_id: u32,

    /// Provider-specific endpoint or ARN
    pub endpoint: String,

    /// Estimated cold start time
    pub cold_start_ms: Option<u64>,

    /// Memory allocation in MB
    pub memory_mb: u32,

    /// Timeout in seconds
    pub timeout_secs: u32,
}

/// Metrics collected from a FaaS invocation
#[derive(Debug, Clone)]
pub struct FaasMetrics {
    /// Total invocation time including cold start
    pub total_duration_ms: u64,

    /// Actual execution time (excluding cold start)
    pub execution_duration_ms: u64,

    /// Whether this was a cold start
    pub cold_start: bool,

    /// Memory used during execution
    pub memory_used_mb: Option<u32>,

    /// Billable duration (provider-specific rounding)
    pub billed_duration_ms: u64,
}

/// Core trait for FaaS execution
///
/// This trait abstracts over different FaaS providers (AWS Lambda, GCP Cloud Functions,
/// Azure Functions, or custom implementations). The BlueprintRunner uses this trait
/// to delegate job execution without knowing the underlying provider.
///
/// # Example Implementation
///
/// ```ignore
/// struct MyCustomFaas {
///     endpoint: String,
/// }
///
/// #[async_trait::async_trait]
/// impl FaasExecutor for MyCustomFaas {
///     async fn invoke(&self, job_call: JobCall) -> Result<JobResult, FaasError> {
///         // HTTP call to your custom serverless runtime
///         let response = reqwest::post(&self.endpoint)
///             .json(&job_call)
///             .send()
///             .await?;
///
///         Ok(response.json().await?)
///     }
///
///     // ... implement other methods
/// }
/// ```
#[async_trait::async_trait]
pub trait FaasExecutor: Send + Sync + fmt::Debug {
    /// Invoke a job on the FaaS platform
    ///
    /// This is the core method - it takes a JobCall and returns a JobResult.
    /// The implementation handles all provider-specific logic (authentication,
    /// serialization, invocation, deserialization).
    async fn invoke(&self, job_call: JobCall) -> Result<JobResult, FaasError>;

    /// Invoke with metrics collection
    ///
    /// Same as invoke, but also returns metrics about the execution.
    async fn invoke_with_metrics(
        &self,
        job_call: JobCall,
    ) -> Result<(JobResult, FaasMetrics), FaasError> {
        let start = std::time::Instant::now();
        let result = self.invoke(job_call).await?;
        let duration = start.elapsed();

        let metrics = FaasMetrics {
            total_duration_ms: duration.as_millis() as u64,
            execution_duration_ms: duration.as_millis() as u64,
            cold_start: false, // Default, providers can override
            memory_used_mb: None,
            billed_duration_ms: duration.as_millis() as u64,
        };

        Ok((result, metrics))
    }

    /// Deploy a job to the FaaS platform
    ///
    /// This is called during blueprint deployment to create the serverless function.
    /// The binary parameter contains the compiled blueprint binary that will be
    /// packaged and deployed.
    async fn deploy_job(
        &self,
        job_id: u32,
        binary: &[u8],
        config: &FaasConfig,
    ) -> Result<FaasDeployment, FaasError>;

    /// Check if the FaaS function is healthy and responsive
    async fn health_check(&self, job_id: u32) -> Result<bool, FaasError>;

    /// Pre-warm the function to reduce cold start latency
    ///
    /// This is optional - not all providers support this.
    async fn warm(&self, job_id: u32) -> Result<(), FaasError> {
        let _ = job_id;
        Ok(()) // Default: no-op
    }

    /// Get information about a deployed function
    async fn get_deployment(&self, job_id: u32) -> Result<FaasDeployment, FaasError>;

    /// Remove a deployed function
    async fn undeploy_job(&self, job_id: u32) -> Result<(), FaasError>;

    /// Get the display name of this FaaS provider (e.g., "AWS Lambda", "GCP Cloud Functions")
    fn provider_name(&self) -> &str;
}

/// Configuration for FaaS deployment
#[derive(Debug, Clone)]
pub struct FaasConfig {
    /// Memory allocation in MB
    pub memory_mb: u32,

    /// Timeout in seconds
    pub timeout_secs: u32,

    /// Environment variables to pass to the function
    pub env_vars: std::collections::HashMap<String, String>,

    /// Concurrency limit (max concurrent executions)
    pub max_concurrency: Option<u32>,

    /// Pre-warm settings
    pub keep_warm: bool,

    /// Provider-specific configuration (JSON)
    pub provider_config: Option<serde_json::Value>,
}

impl Default for FaasConfig {
    fn default() -> Self {
        Self {
            memory_mb: 512,
            timeout_secs: 300, // 5 minutes
            env_vars: std::collections::HashMap::new(),
            max_concurrency: None,
            keep_warm: false,
            provider_config: None,
        }
    }
}

/// Type-erased FaaS executor for runtime polymorphism
pub type DynFaasExecutor = Arc<dyn FaasExecutor>;

/// Registry of FaaS executors by job ID
///
/// This is used by BlueprintRunner to manage which jobs are delegated to which
/// FaaS executors.
#[derive(Default)]
pub struct FaasRegistry {
    executors: std::collections::HashMap<u32, DynFaasExecutor>,
}

impl FaasRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a FaaS executor for a specific job ID
    pub fn register(&mut self, job_id: u32, executor: DynFaasExecutor) {
        self.executors.insert(job_id, executor);
    }

    /// Get the executor for a job ID
    pub fn get(&self, job_id: u32) -> Option<&DynFaasExecutor> {
        self.executors.get(&job_id)
    }

    /// Check if a job should be delegated to FaaS
    pub fn is_faas_job(&self, job_id: u32) -> bool {
        self.executors.contains_key(&job_id)
    }

    /// Get all registered job IDs
    pub fn job_ids(&self) -> impl Iterator<Item = u32> + '_ {
        self.executors.keys().copied()
    }
}

impl fmt::Debug for FaasRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FaasRegistry")
            .field("job_count", &self.executors.len())
            .field("job_ids", &self.executors.keys().collect::<Vec<_>>())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct MockFaasExecutor {
        name: String,
    }

    #[async_trait::async_trait]
    impl FaasExecutor for MockFaasExecutor {
        async fn invoke(&self, _job_call: JobCall) -> Result<JobResult, FaasError> {
            unimplemented!("mock")
        }

        async fn deploy_job(
            &self,
            _job_id: u32,
            _binary: &[u8],
            _config: &FaasConfig,
        ) -> Result<FaasDeployment, FaasError> {
            unimplemented!("mock")
        }

        async fn health_check(&self, _job_id: u32) -> Result<bool, FaasError> {
            Ok(true)
        }

        async fn get_deployment(&self, _job_id: u32) -> Result<FaasDeployment, FaasError> {
            unimplemented!("mock")
        }

        async fn undeploy_job(&self, _job_id: u32) -> Result<(), FaasError> {
            Ok(())
        }

        fn provider_name(&self) -> &str {
            &self.name
        }
    }

    #[test]
    fn test_registry() {
        let mut registry = FaasRegistry::new();

        assert!(!registry.is_faas_job(0));

        let executor = Arc::new(MockFaasExecutor {
            name: "test".to_string(),
        });
        registry.register(0, executor);

        assert!(registry.is_faas_job(0));
        assert!(!registry.is_faas_job(1));
    }
}
