use async_trait::async_trait;
use blueprint_core::{JobCall, JobResult};
use blueprint_faas::{
    DynFaasExecutor, FaasConfig, FaasDeployment, FaasError, FaasExecutor, FaasRegistry,
};
use std::sync::Arc;

#[derive(Debug)]
struct StubExecutor;

#[async_trait]
impl FaasExecutor for StubExecutor {
    async fn invoke(&self, _job_call: JobCall) -> Result<JobResult, FaasError> {
        Ok(JobResult::empty())
    }

    async fn deploy_job(
        &self,
        job_id: u32,
        _binary: &[u8],
        config: &FaasConfig,
    ) -> Result<FaasDeployment, FaasError> {
        Ok(FaasDeployment {
            function_id: format!("stub-{job_id}"),
            job_id,
            endpoint: "stub://executor".to_string(),
            cold_start_ms: Some(0),
            memory_mb: config.memory_mb,
            timeout_secs: config.timeout_secs,
        })
    }

    async fn health_check(&self, _job_id: u32) -> Result<bool, FaasError> {
        Ok(true)
    }

    async fn get_deployment(&self, job_id: u32) -> Result<FaasDeployment, FaasError> {
        Ok(FaasDeployment {
            function_id: format!("stub-{job_id}"),
            job_id,
            endpoint: "stub://executor".to_string(),
            cold_start_ms: Some(0),
            memory_mb: 0,
            timeout_secs: 0,
        })
    }

    async fn undeploy_job(&self, _job_id: u32) -> Result<(), FaasError> {
        Ok(())
    }

    fn provider_name(&self) -> &str {
        "Stub FaaS"
    }
}

#[test]
fn registry_starts_empty() {
    let registry = FaasRegistry::default();
    assert_eq!(registry.job_ids().count(), 0);
    assert!(!registry.is_faas_job(42));
}

#[test]
fn registry_registers_executor() {
    let mut registry = FaasRegistry::new();
    let executor: DynFaasExecutor = Arc::new(StubExecutor);
    registry.register(1, executor);

    assert!(registry.is_faas_job(1));
    let stored = registry.get(1).expect("executor should be stored");
    assert_eq!(stored.provider_name(), "Stub FaaS");
}

#[cfg(feature = "custom")]
#[test]
fn custom_http_executor_exposes_provider_name() {
    use blueprint_faas::custom::HttpFaasExecutor;

    let executor = HttpFaasExecutor::new("http://localhost:8080");
    assert_eq!(executor.provider_name(), "Custom HTTP FaaS");
}

#[cfg(feature = "aws")]
#[test]
fn aws_lambda_executor_type_is_accessible() {
    use blueprint_faas::aws::LambdaExecutor;

    let type_name = std::any::type_name::<LambdaExecutor>();
    assert!(
        type_name.contains("LambdaExecutor"),
        "type should remain accessible for feature-gated builds"
    );
}
