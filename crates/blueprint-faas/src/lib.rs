//! FaaS Provider Integrations for Blueprint SDK
//!
//! This crate provides implementations of the `FaasExecutor` trait for various
//! serverless platforms:
//!
//! - **AWS Lambda** - `aws` module
//! - **GCP Cloud Functions** - `gcp` module
//! - **Azure Functions** - `azure` module
//! - **Custom HTTP-based FaaS** - `custom` module
//!
//! ## Features
//!
//! - `aws` - Enable AWS Lambda integration
//! - `gcp` - Enable Google Cloud Functions integration
//! - `azure` - Enable Azure Functions integration
//! - `custom` - Enable custom HTTP-based FaaS integration
//! - `all` - Enable all providers
//!
//! ## Usage
//!
//! ```rust,ignore
//! use blueprint_faas::aws::LambdaExecutor;
//! use blueprint_runner::BlueprintRunner;
//!
//! let lambda = LambdaExecutor::new("us-east-1").await?;
//!
//! BlueprintRunner::builder(config, env)
//!     .router(router)
//!     .with_faas_executor(0, lambda)
//!     .run().await
//! ```

#![cfg_attr(not(test), warn(missing_docs))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

// Core FaaS abstractions
pub mod core;

// Re-export core types for convenience
pub use core::{
    DynFaasExecutor, FaasConfig, FaasDeployment, FaasError, FaasExecutor, FaasMetrics,
    FaasPayload, FaasRegistry, FaasResponse,
};

#[cfg(feature = "aws")]
pub mod aws;

#[cfg(feature = "gcp")]
pub mod gcp;

#[cfg(feature = "azure")]
pub mod azure;

#[cfg(feature = "custom")]
pub mod custom;

/// Factory for creating FaaS executors from provider configuration
#[cfg(any(feature = "aws", feature = "gcp", feature = "azure", feature = "custom"))]
pub mod factory {
    use super::*;
    use std::sync::Arc;

    /// Provider-agnostic FaaS configuration
    #[derive(Debug, Clone)]
    pub struct FaasProviderConfig {
        pub provider: FaasProvider,
        pub default_memory_mb: u32,
        pub default_timeout_secs: u32,
    }

    /// FaaS provider variants
    #[derive(Debug, Clone)]
    pub enum FaasProvider {
        #[cfg(feature = "aws")]
        AwsLambda { region: String, role_arn: String },
        #[cfg(feature = "gcp")]
        GcpFunctions { project_id: String, region: String },
        #[cfg(feature = "azure")]
        AzureFunctions {
            subscription_id: String,
            region: String,
        },
        #[cfg(feature = "custom")]
        Custom { endpoint: String },
    }

    /// Create a FaaS executor from provider configuration
    pub async fn create_executor(
        provider_config: FaasProviderConfig,
    ) -> Result<DynFaasExecutor, FaasError> {
        match provider_config.provider {
            #[cfg(feature = "aws")]
            FaasProvider::AwsLambda { region, role_arn } => {
                let executor = crate::aws::LambdaExecutor::new(&region, role_arn).await?;
                Ok(Arc::new(executor) as DynFaasExecutor)
            }
            #[cfg(feature = "gcp")]
            FaasProvider::GcpFunctions { project_id, region } => {
                let executor =
                    crate::gcp::CloudFunctionExecutor::new(project_id, region).await?;
                Ok(Arc::new(executor) as DynFaasExecutor)
            }
            #[cfg(feature = "azure")]
            FaasProvider::AzureFunctions {
                subscription_id,
                region,
            } => {
                let executor =
                    crate::azure::AzureFunctionExecutor::new(subscription_id, region).await?;
                Ok(Arc::new(executor) as DynFaasExecutor)
            }
            #[cfg(feature = "custom")]
            FaasProvider::Custom { endpoint } => {
                let executor = crate::custom::HttpFaasExecutor::new(endpoint);
                Ok(Arc::new(executor) as DynFaasExecutor)
            }
        }
    }

    /// Deploy a job using provider configuration
    pub async fn deploy_job(
        provider_config: FaasProviderConfig,
        job_id: u32,
        binary: &[u8],
    ) -> Result<FaasDeployment, FaasError> {
        let executor = create_executor(provider_config.clone()).await?;

        let faas_config = FaasConfig {
            memory_mb: provider_config.default_memory_mb,
            timeout_secs: provider_config.default_timeout_secs,
            ..Default::default()
        };

        executor.deploy_job(job_id, binary, &faas_config).await
    }
}

/// Common utilities shared across providers
mod utils {
    #[cfg(feature = "aws")]
    use super::*;

    /// Create a Lambda deployment package from a binary
    #[cfg(feature = "aws")]
    pub(crate) fn create_lambda_package(binary: &[u8]) -> Result<Vec<u8>, FaasError> {
        use std::io::Cursor;
        use std::io::Write;

        let mut zip = zip::ZipWriter::new(Cursor::new(Vec::new()));
        let options = zip::write::FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o755);

        zip.start_file("bootstrap", options)
            .map_err(|e| FaasError::InfrastructureError(format!("Failed to create zip: {}", e)))?;

        zip.write_all(binary).map_err(|e| {
            FaasError::InfrastructureError(format!("Failed to write binary: {}", e))
        })?;

        let cursor = zip.finish().map_err(|e| {
            FaasError::InfrastructureError(format!("Failed to finalize zip: {}", e))
        })?;

        Ok(cursor.into_inner())
    }

    /// Extract job ID from function name
    #[allow(dead_code)]
    pub(crate) fn extract_job_id(function_name: &str, prefix: &str) -> Option<u32> {
        function_name
            .strip_prefix(&format!("{}-job-", prefix))
            .and_then(|s| s.parse().ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_job_id() {
        assert_eq!(
            utils::extract_job_id("blueprint-job-0", "blueprint"),
            Some(0)
        );
        assert_eq!(
            utils::extract_job_id("blueprint-job-42", "blueprint"),
            Some(42)
        );
        assert_eq!(utils::extract_job_id("wrong-format", "blueprint"), None);
    }
}
