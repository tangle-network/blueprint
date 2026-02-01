//! End-to-end test demonstrating FaaS job execution alongside local jobs
//!
//! This test proves that:
//! 1. FaaS-executed jobs follow the same consumer pipeline as local jobs
//! 2. Both local and FaaS results reach onchain via TangleConsumer
//! 3. The same job logic can run either locally or on FaaS
//!
//! Run with: cargo test --test faas_e2e

use alloy_primitives::Bytes;
use alloy_sol_types::SolValue;
use anyhow::{Context, Result};
use axum::{Router as AxumRouter, extract::Json, routing::post};
use blueprint_anvil_testing_utils::{BlueprintHarness, missing_tnt_core_artifacts};
use blueprint_faas::{FaasPayload, FaasResponse};
use incredible_squaring_blueprint_lib::{XSQUARE_FAAS_JOB_ID, XSQUARE_JOB_ID, router};
use std::time::Duration;
use tokio::task::JoinHandle;

/// Handler for the /square endpoint
///
/// This handler computes the square directly (same logic as the local job).
/// In production, this would be deployed to AWS Lambda, GCP Cloud Functions, etc.
async fn square_handler(Json(payload): Json<FaasPayload>) -> Json<FaasResponse> {
    eprintln!("[FAAS] Received request for job_id={}", payload.job_id);

    // Decode the input (ABI-encoded u64)
    let x = u64::abi_decode(&payload.args).expect("Failed to decode u64 input");
    let result = x * x;

    eprintln!("[FAAS] Computation: {} * {} = {}", x, x, result);

    // Encode the result (ABI-encoded u64)
    let result_bytes = result.abi_encode();

    Json(FaasResponse {
        result: result_bytes,
    })
}

/// Start local HTTP server that mimics FaaS runtime
async fn start_test_faas_server() -> (JoinHandle<()>, String) {
    let app = AxumRouter::new().route("/square", post(square_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind to ephemeral port");

    let addr = listener.local_addr().expect("Failed to get local address");
    let url = format!("http://{}", addr);

    let handle = tokio::spawn(async move {
        axum::serve(listener, app).await.expect("Server failed");
    });

    // Give the server a moment to start
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    (handle, url)
}

/// Full E2E test: Local job and FaaS job, both reaching on-chain
#[tokio::test]
#[serial_test::serial]
async fn test_faas_and_local_execution_e2e() -> Result<()> {
    use blueprint_faas::custom::HttpFaasExecutor;

    let _ = color_eyre::install();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init()
        .ok();

    // Start HTTP server that mimics FaaS runtime
    let (server_handle, server_url) = start_test_faas_server().await;
    eprintln!("[TEST] FaaS server started at {}", server_url);

    // Configure FaaS executor for XSQUARE_FAAS_JOB_ID
    let faas_executor = HttpFaasExecutor::new(&server_url).with_job_endpoint(
        u32::from(XSQUARE_FAAS_JOB_ID),
        format!("{}/square", server_url),
    );

    // Build harness with both local and FaaS jobs
    let harness = match BlueprintHarness::builder(router())
        .poll_interval(Duration::from_millis(50))
        .with_faas_executor(u32::from(XSQUARE_FAAS_JOB_ID), faas_executor)
        .spawn()
        .await
    {
        Ok(harness) => harness,
        Err(err) => {
            if missing_tnt_core_artifacts(&err) {
                eprintln!("Skipping test_faas_and_local_execution_e2e: {err}");
                server_handle.abort();
                return Ok(());
            }
            return Err(err.into());
        }
    };

    eprintln!(
        "[TEST] Harness spawned, service_id={}",
        harness.service_id()
    );

    // Test 1: LOCAL execution (Job 0 - XSQUARE_JOB_ID)
    eprintln!("[TEST] Testing LOCAL execution (job {})", XSQUARE_JOB_ID);
    let input_local: u64 = 5;
    let payload_local = Bytes::from(input_local.abi_encode());

    let submission_local = harness
        .submit_job(XSQUARE_JOB_ID, payload_local)
        .await
        .context("failed to submit local job")?;

    let result_local = harness
        .wait_for_job_result_with_deadline(submission_local, Duration::from_secs(30))
        .await
        .context("failed to get local job result")?;

    let decoded_local = u64::abi_decode(&result_local)
        .map_err(|e| anyhow::anyhow!("failed to decode local result: {e}"))?;
    assert_eq!(
        decoded_local,
        input_local * input_local,
        "Local job result mismatch"
    );
    eprintln!(
        "[TEST] LOCAL job passed: {} * {} = {}",
        input_local, input_local, decoded_local
    );

    // Test 2: FAAS execution (Job 3 - XSQUARE_FAAS_JOB_ID)
    eprintln!(
        "[TEST] Testing FAAS execution (job {})",
        XSQUARE_FAAS_JOB_ID
    );
    let input_faas: u64 = 7;
    let payload_faas = Bytes::from(input_faas.abi_encode());

    let submission_faas = harness
        .submit_job(XSQUARE_FAAS_JOB_ID, payload_faas)
        .await
        .context("failed to submit FaaS job")?;

    let result_faas = harness
        .wait_for_job_result_with_deadline(submission_faas, Duration::from_secs(30))
        .await
        .context("failed to get FaaS job result")?;

    let decoded_faas = u64::abi_decode(&result_faas)
        .map_err(|e| anyhow::anyhow!("failed to decode FaaS result: {e}"))?;
    assert_eq!(
        decoded_faas,
        input_faas * input_faas,
        "FaaS job result mismatch"
    );
    eprintln!(
        "[TEST] FAAS job passed: {} * {} = {}",
        input_faas, input_faas, decoded_faas
    );

    // Cleanup
    server_handle.abort();
    harness.shutdown().await;

    eprintln!("[TEST] Both local and FaaS jobs completed successfully!");
    Ok(())
}

/// Test the FaaS HTTP server directly (without harness)
#[tokio::test]
#[serial_test::serial]
async fn test_faas_server_directly() -> Result<()> {
    let _ = color_eyre::install();

    let (server_handle, server_url) = start_test_faas_server().await;

    // Create a FaaS payload with ABI-encoded input
    let input: u64 = 9;
    let payload = FaasPayload {
        job_id: u32::from(XSQUARE_FAAS_JOB_ID),
        args: input.abi_encode(),
    };

    // Send request to FaaS server
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/square", server_url))
        .json(&payload)
        .send()
        .await?;

    assert!(response.status().is_success());

    let faas_response: FaasResponse = response.json().await?;

    // Verify result (ABI-encoded u64)
    let result = u64::abi_decode(&faas_response.result)
        .map_err(|e| anyhow::anyhow!("failed to decode result: {e}"))?;
    assert_eq!(result, 81); // 9 * 9 = 81

    server_handle.abort();
    Ok(())
}
