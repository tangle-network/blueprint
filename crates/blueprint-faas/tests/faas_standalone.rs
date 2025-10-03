//! Standalone FaaS tests that don't require full blueprint stack
//!
//! This tests the core FaaS traits and registry without substrate dependencies

use blueprint_faas::{FaasExecutor, FaasRegistry, FaasError, FaasDeployment, FaasConfig};
use blueprint_core::{JobCall, JobResult};
use std::sync::Arc;

/// Minimal test executor for verification
#[derive(Debug, Clone)]
struct TestExecutor {
    name: String,
    job_id: u32,
}

#[async_trait::async_trait]
impl FaasExecutor for TestExecutor {
    async fn invoke(&self, job_call: JobCall) -> Result<JobResult, FaasError> {
        // Echo back the call ID
        let result = JobResult::builder(job_call.job_id())
            .body(job_call.body().clone())
            .build();
        Ok(result)
    }

    async fn deploy_job(
        &self,
        job_id: u32,
        _binary: &[u8],
        config: &FaasConfig,
    ) -> Result<FaasDeployment, FaasError> {
        Ok(FaasDeployment {
            function_id: format!("{}-job-{}", self.name, job_id),
            job_id,
            endpoint: format!("test://{}/job/{}", self.name, job_id),
            cold_start_ms: Some(100),
            memory_mb: config.memory_mb,
            timeout_secs: config.timeout_secs,
        })
    }

    async fn health_check(&self, _job_id: u32) -> Result<bool, FaasError> {
        Ok(true)
    }

    async fn get_deployment(&self, job_id: u32) -> Result<FaasDeployment, FaasError> {
        Ok(FaasDeployment {
            function_id: format!("{}-job-{}", self.name, job_id),
            job_id,
            endpoint: format!("test://{}/job/{}", self.name, job_id),
            cold_start_ms: Some(100),
            memory_mb: 512,
            timeout_secs: 300,
        })
    }

    async fn undeploy_job(&self, _job_id: u32) -> Result<(), FaasError> {
        Ok(())
    }

    fn provider_name(&self) -> &str {
        &self.name
    }
}

#[tokio::test]
async fn test_faas_registry_basic() {
    let mut registry = FaasRegistry::new();

    // Initially empty
    assert!(!registry.is_faas_job(0));
    assert!(!registry.is_faas_job(1));

    // Register job 0
    let executor = Arc::new(TestExecutor {
        name: "test-provider".to_string(),
        job_id: 0,
    });
    registry.register(0, executor);

    // Now job 0 is registered
    assert!(registry.is_faas_job(0));
    assert!(!registry.is_faas_job(1));

    // Can retrieve executor
    let retrieved = registry.get(0).expect("Should find executor");
    assert_eq!(retrieved.provider_name(), "test-provider");
}

#[tokio::test]
async fn test_faas_registry_multiple_jobs() {
    let mut registry = FaasRegistry::new();

    // Register multiple jobs to same executor
    let executor = Arc::new(TestExecutor {
        name: "shared-provider".to_string(),
        job_id: 0,
    });

    registry.register(0, executor.clone());
    registry.register(1, executor.clone());
    registry.register(5, executor.clone());

    assert!(registry.is_faas_job(0));
    assert!(registry.is_faas_job(1));
    assert!(!registry.is_faas_job(2));
    assert!(registry.is_faas_job(5));

    // Check all job IDs
    let job_ids: Vec<u32> = registry.job_ids().collect();
    assert_eq!(job_ids.len(), 3);
    assert!(job_ids.contains(&0));
    assert!(job_ids.contains(&1));
    assert!(job_ids.contains(&5));
}

#[tokio::test]
async fn test_executor_invoke() {
    let executor = TestExecutor {
        name: "test".to_string(),
        job_id: 0,
    };

    let job_call = JobCall::builder(0u32)
        .body(b"test data".to_vec().into())
        .build();

    let result = executor.invoke(job_call).await
        .expect("Invocation should succeed");

    assert_eq!(result.body().as_ref(), b"test data");
}

#[tokio::test]
async fn test_executor_deploy() {
    let executor = TestExecutor {
        name: "deploy-test".to_string(),
        job_id: 0,
    };

    let config = FaasConfig {
        memory_mb: 1024,
        timeout_secs: 60,
        ..Default::default()
    };

    let deployment = executor.deploy_job(0, b"binary", &config).await
        .expect("Deployment should succeed");

    assert_eq!(deployment.job_id, 0);
    assert_eq!(deployment.function_id, "deploy-test-job-0");
    assert_eq!(deployment.memory_mb, 1024);
    assert_eq!(deployment.timeout_secs, 60);
}

#[tokio::test]
async fn test_executor_health_check() {
    let executor = TestExecutor {
        name: "health-test".to_string(),
        job_id: 0,
    };

    let health = executor.health_check(0).await
        .expect("Health check should succeed");

    assert!(health, "Should be healthy");
}

#[tokio::test]
async fn test_multiple_executors_different_providers() {
    let mut registry = FaasRegistry::new();

    let lambda = Arc::new(TestExecutor {
        name: "AWS Lambda".to_string(),
        job_id: 0,
    });

    let cloudrun = Arc::new(TestExecutor {
        name: "GCP Cloud Run".to_string(),
        job_id: 1,
    });

    registry.register(0, lambda.clone());
    registry.register(1, cloudrun.clone());

    let exec0 = registry.get(0).unwrap();
    let exec1 = registry.get(1).unwrap();

    assert_eq!(exec0.provider_name(), "AWS Lambda");
    assert_eq!(exec1.provider_name(), "GCP Cloud Run");
}
