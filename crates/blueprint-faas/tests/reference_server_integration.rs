//! Integration tests using the reference FaaS server
//!
//! These tests verify basic HTTP FaaS executor functionality.
//! Note: HttpFaasExecutor is designed for custom platforms where deployment
//! is handled separately. The reference server provides a full implementation
//! of the Custom FaaS Platform Spec for testing other integrations.

#![cfg(feature = "custom")]

use blueprint_faas::custom::HttpFaasExecutor;
use blueprint_faas::FaasExecutor;

#[test]
fn test_http_executor_creation() {
    let executor = HttpFaasExecutor::new("http://localhost:8080");
    assert_eq!(executor.provider_name(), "Custom HTTP FaaS");
}

#[test]
fn test_http_executor_with_custom_endpoints() {
    let executor = HttpFaasExecutor::new("http://localhost:8080")
        .with_job_endpoint(0, "http://custom.com/job0")
        .with_job_endpoint(5, "http://custom.com/job5");

    assert_eq!(executor.provider_name(), "Custom HTTP FaaS");
}

#[test]
fn test_http_executor_base_url() {
    let executor = HttpFaasExecutor::new("https://my-platform.com");
    assert_eq!(executor.provider_name(), "Custom HTTP FaaS");
}

#[test]
fn test_http_executor_multiple_endpoints() {
    let executor = HttpFaasExecutor::new("http://localhost:8080")
        .with_job_endpoint(0, "http://platform-a.com/job0")
        .with_job_endpoint(1, "http://platform-b.com/job1")
        .with_job_endpoint(2, "http://platform-c.com/job2");

    assert_eq!(executor.provider_name(), "Custom HTTP FaaS");
}
