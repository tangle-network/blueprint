//! Integration tests for HTTP-based FaaS executor
//!
//! These tests use a real HTTP server (no mocks).

use blueprint_core::{JobCall, JobResult};
use blueprint_faas::custom::HttpFaasExecutor;
use blueprint_faas::FaasExecutor;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

/// Test job call handler
async fn handle_job_call(call: JobCall) -> Result<impl warp::Reply, warp::Rejection> {
    // Simple squaring job for testing
    let args: Vec<u64> = serde_json::from_slice(call.body().as_ref())
        .unwrap_or_else(|_| vec![]);

    let result = if let Some(&x) = args.first() {
        x * x
    } else {
        0
    };

    let job_result = JobResult::builder(call.job_id())
        .body(serde_json::to_vec(&result).unwrap().into())
        .build();

    Ok(warp::reply::json(&job_result))
}

#[tokio::test]
async fn test_http_faas_real_invocation() {
    // Start a real HTTP server
    let routes = warp::post()
        .and(warp::path("job"))
        .and(warp::path::param::<u32>())
        .and(warp::body::json())
        .and_then(|job_id: u32, call: JobCall| async move {
            handle_job_call(call).await
        });

    let server = warp::serve(routes);
    let addr = ([127, 0, 0, 1], 0); // Random port
    let (addr, server_future) = server.bind_ephemeral(addr);

    // Spawn server in background
    tokio::spawn(server_future);

    // Give server time to start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Create HTTP FaaS executor
    let executor = HttpFaasExecutor::new(format!("http://{}", addr));

    // Create a job call
    let job_call = JobCall::builder(0u32)
        .body(serde_json::to_vec(&vec![5u64]).unwrap().into())
        .build();

    // Invoke the function
    let result = executor.invoke(job_call).await
        .expect("FaaS invocation should succeed");

    // Verify result
    let value: u64 = serde_json::from_slice(result.body().as_ref())
        .expect("Result should be valid JSON");
    assert_eq!(value, 25, "5 squared should equal 25");
}

#[tokio::test]
async fn test_http_faas_custom_endpoint() {
    // Start server with specific endpoint
    let routes = warp::post()
        .and(warp::path("custom"))
        .and(warp::path("square"))
        .and(warp::body::json())
        .and_then(|call: JobCall| async move {
            handle_job_call(call).await
        });

    let server = warp::serve(routes);
    let (addr, server_future) = server.bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server_future);
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Create executor with custom endpoint
    let executor = HttpFaasExecutor::new(format!("http://{}", addr))
        .with_job_endpoint(0, format!("http://{}/custom/square", addr));

    let job_call = JobCall::builder(0u32)
        .body(serde_json::to_vec(&vec![7u64]).unwrap().into())
        .build();

    let result = executor.invoke(job_call).await
        .expect("Custom endpoint invocation should succeed");

    let value: u64 = serde_json::from_slice(result.body().as_ref()).unwrap();
    assert_eq!(value, 49);
}

#[tokio::test]
async fn test_http_faas_error_handling() {
    // Start server that returns errors
    let routes = warp::post()
        .and(warp::path("job"))
        .and(warp::path::param::<u32>())
        .and(warp::body::json())
        .and_then(|_job_id: u32, _call: JobCall| async move {
            Err::<warp::reply::Json, _>(warp::reject::not_found())
        });

    let server = warp::serve(routes);
    let (addr, server_future) = server.bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server_future);
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let executor = HttpFaasExecutor::new(format!("http://{}", addr));
    let job_call = JobCall::builder(0u32)
        .body(vec![].into())
        .build();

    let result = executor.invoke(job_call).await;
    assert!(result.is_err(), "Should fail when server returns error");
}

#[tokio::test]
async fn test_http_faas_health_check() {
    let routes = warp::head()
        .and(warp::path("job"))
        .and(warp::path::param::<u32>())
        .map(|_job_id: u32| {
            warp::reply::with_status("", warp::http::StatusCode::OK)
        });

    let server = warp::serve(routes);
    let (addr, server_future) = server.bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server_future);
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let executor = HttpFaasExecutor::new(format!("http://{}", addr));
    let health = executor.health_check(0).await
        .expect("Health check should succeed");

    assert!(health, "Health check should return true");
}

#[tokio::test]
async fn test_http_faas_concurrent_invocations() {
    // Test concurrent FaaS calls
    let counter = Arc::new(Mutex::new(0u64));
    let counter_clone = counter.clone();

    let routes = warp::post()
        .and(warp::path("job"))
        .and(warp::path::param::<u32>())
        .and(warp::body::json())
        .and(warp::any().map(move || counter_clone.clone()))
        .and_then(|job_id: u32, call: JobCall, counter: Arc<Mutex<u64>>| async move {
            let mut count = counter.lock().await;
            *count += 1;
            drop(count);
            handle_job_call(call).await
        });

    let server = warp::serve(routes);
    let (addr, server_future) = server.bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server_future);
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let executor = Arc::new(HttpFaasExecutor::new(format!("http://{}", addr)));

    // Spawn 10 concurrent invocations
    let mut handles = vec![];
    for i in 1..=10 {
        let exec = executor.clone();
        handles.push(tokio::spawn(async move {
            let job_call = JobCall::builder(0u32)
                .body(serde_json::to_vec(&vec![i as u64]).unwrap().into())
                .build();
            exec.invoke(job_call).await
        }));
    }

    // Wait for all to complete
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok(), "All concurrent invocations should succeed");
    }

    // Verify all 10 were called
    let final_count = *counter.lock().await;
    assert_eq!(final_count, 10, "Should have processed 10 requests");
}
