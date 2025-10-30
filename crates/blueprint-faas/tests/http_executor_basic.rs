//! Basic HTTP executor tests
//! These tests verify the HTTP executor structure without full integration

#[cfg(feature = "custom")]
#[test]
fn test_http_executor_creation() {
    use blueprint_faas::FaasExecutor;
    use blueprint_faas::custom::HttpFaasExecutor;

    let executor = HttpFaasExecutor::new("http://localhost:8080");
    let provider = executor.provider_name();
    assert_eq!(provider, "Custom HTTP FaaS");
}

#[cfg(feature = "custom")]
#[test]
fn test_http_executor_with_custom_endpoint() {
    use blueprint_faas::FaasExecutor;
    use blueprint_faas::custom::HttpFaasExecutor;

    let executor = HttpFaasExecutor::new("http://localhost:8080")
        .with_job_endpoint(0, "http://custom.com/job0")
        .with_job_endpoint(5, "http://custom.com/job5");

    // Executor should be created successfully
    assert_eq!(executor.provider_name(), "Custom HTTP FaaS");
}

#[test]
fn faas_payload_round_trip_preserves_body() {
    use blueprint_core::{JobCall, JobResult};
    use blueprint_faas::{FaasPayload, FaasResponse};
    use bytes::Bytes;

    let job_call = JobCall::new(7u32, Bytes::from_static(b"payload"));
    let payload = FaasPayload::from(job_call);
    assert_eq!(payload.job_id, 7);
    assert_eq!(payload.args, b"payload");

    let job_result = JobResult::new(Bytes::from_static(b"result"));
    let response = FaasResponse::from(job_result.clone());
    assert_eq!(response.result, b"result");

    let round_trip: JobResult = response.into();
    let body = round_trip.body().expect("result should be ok");
    assert_eq!(body.as_ref(), b"result");
}
