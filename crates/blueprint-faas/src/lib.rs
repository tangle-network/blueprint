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
    FaasConfig, FaasDeployment, FaasError, FaasExecutor, FaasMetrics, FaasRegistry,
    DynFaasExecutor,
};

#[cfg(feature = "aws")]
pub mod aws;

#[cfg(feature = "gcp")]
pub mod gcp;

#[cfg(feature = "azure")]
pub mod azure;

#[cfg(feature = "custom")]
pub mod custom;

/// Common utilities shared across providers
mod utils {
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
