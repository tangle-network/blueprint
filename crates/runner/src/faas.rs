//! FaaS execution abstraction for BlueprintRunner
//!
//! This module re-exports FaaS types from the `blueprint-faas` crate.
//! The FaaS trait and types are defined in `blueprint-faas` to avoid
//! circular dependencies and substrate dependencies.

#[cfg(feature = "faas")]
pub use blueprint_faas::*;

#[cfg(not(feature = "faas"))]
mod stub {
    //! Stub implementation when faas feature is disabled
    use blueprint_core::{JobCall, JobResult};
    use std::fmt;
    use std::sync::Arc;

    /// Stub FaasError
    #[derive(Debug)]
    pub struct FaasError;

    impl std::fmt::Display for FaasError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "FaaS feature not enabled")
        }
    }

    impl std::error::Error for FaasError {}

    /// Stub FaasExecutor trait
    #[async_trait::async_trait]
    pub trait FaasExecutor: Send + Sync + fmt::Debug {
        async fn invoke(&self, _job_call: JobCall) -> Result<JobResult, FaasError> {
            Err(FaasError)
        }
        fn provider_name(&self) -> &str {
            "stub"
        }
    }

    /// Stub FaasRegistry
    #[derive(Default, Debug)]
    pub struct FaasRegistry;

    impl FaasRegistry {
        pub fn new() -> Self {
            Self
        }

        pub fn register(&mut self, _job_id: u32, _executor: Arc<dyn FaasExecutor>) {}

        pub fn get(&self, _job_id: u32) -> Option<&Arc<dyn FaasExecutor>> {
            None
        }

        pub fn is_faas_job(&self, _job_id: u32) -> bool {
            false
        }

        pub fn job_ids(&self) -> impl Iterator<Item = u32> {
            std::iter::empty()
        }
    }
}

#[cfg(not(feature = "faas"))]
pub use stub::*;
