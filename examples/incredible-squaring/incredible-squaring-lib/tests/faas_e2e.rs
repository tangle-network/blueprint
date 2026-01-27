//! End-to-end test demonstrating FaaS job execution alongside local jobs
//!
//! This test proves that:
//! 1. FaaS-executed jobs follow the same consumer pipeline as local jobs
//! 2. Both local and FaaS results reach onchain via TangleConsumer
//! 3. Minimal mocking: only the FaaS endpoint is simulated, everything else is real
//!
//! Run with: cargo test --test faas_e2e

mod tests {
    use axum::{Router, extract::Json, routing::post};
    use blueprint_faas::custom::HttpFaasExecutor;
    use blueprint_faas::{FaasPayload, FaasResponse};
    use blueprint_sdk::Job;
    use blueprint_sdk::tangle::layers::TangleLayer;
    use blueprint_sdk::testing::tempfile;
    use blueprint_sdk::testing::utils::setup_log;
    use blueprint_sdk::testing::utils::tangle::{InputValue, OutputValue, TangleTestHarness};
    use color_eyre::Result;
    use incredible_squaring_blueprint_lib::{
        XSQUARE_FAAS_JOB_ID, XSQUARE_JOB_ID, square, square_faas,
    };
    use tokio::task::JoinHandle;

    /// Handler for the /square endpoint
    ///
    /// This handler executes ACTUAL COMPILED CODE by spawning the faas_handler binary.
    /// This mimics how AWS Lambda works in production:
    /// 1. Lambda runtime spawns your handler binary
    /// 2. Passes event via stdin
    /// 3. Reads response from stdout
    async fn square_handler(Json(payload): Json<FaasPayload>) -> Json<FaasResponse> {
        eprintln!("[FAAS] Received request for job_id={}", payload.job_id);

        // Locate the compiled faas_handler binary
        let binary_path = std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("faas_handler");

        eprintln!("[FAAS] Executing binary: {}", binary_path.display());

        // Serialize payload to JSON
        let input_json = serde_json::to_string(&payload).expect("Failed to serialize payload");

        // ⚡ SPAWN THE ACTUAL COMPILED BINARY (like Lambda does!)
        use tokio::io::AsyncWriteExt;

        let mut child = tokio::process::Command::new(&binary_path)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to spawn faas_handler binary");

        // Write input to stdin
        let mut stdin = child.stdin.take().expect("Failed to open stdin");
        stdin
            .write_all(input_json.as_bytes())
            .await
            .expect("Failed to write to stdin");
        drop(stdin); // Close stdin to signal EOF

        // Wait for process to complete and collect output
        let output = child
            .wait_with_output()
            .await
            .expect("Failed to wait for faas_handler");

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("🔥 FaaS handler failed: {}", stderr);
            panic!("FaaS handler execution failed");
        }

        // Parse response from stdout
        let response: FaasResponse =
            serde_json::from_slice(&output.stdout).expect("Failed to parse FaaS response");

        let x = u64::from_le_bytes(payload.args[..8].try_into().unwrap());
        let result = u64::from_le_bytes(response.result[..8].try_into().unwrap());
        eprintln!("[FAAS] Computation: {} * {} = {}", x, x, result);

        Json(response)
    }

    /// Start local HTTP server that mimics FaaS runtime
    ///
    /// Spawns compiled faas_handler binary for each request (like AWS Lambda)
    async fn start_test_faas_server() -> (JoinHandle<()>, String) {
        let app = Router::new().route("/square", post(square_handler));

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("Failed to bind to ephemeral port");

        let addr = listener.local_addr().expect("Failed to get local address");
        let url = format!("http://{}", addr);

        let handle = tokio::spawn(async move {
            axum::serve(listener, app).await.expect("Server failed");
        });

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        (handle, url)
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_faas_execution_end_to_end() -> Result<()> {
        let _ = color_eyre::install();
        setup_log();

        // Start HTTP server that mimics FaaS runtime
        let (server_handle, server_url) = start_test_faas_server().await;

        // Setup test harness with real Tangle node
        let temp_dir = tempfile::TempDir::new()?;
        let harness = TangleTestHarness::setup(temp_dir).await?;

        // Setup service
        let (mut test_env, service_id, _blueprint_id) = harness.setup_services::<1>(false).await?;
        test_env.initialize().await?;

        // Configure FaaS executor
        let faas_executor = HttpFaasExecutor::new(&server_url)
            .with_job_endpoint(XSQUARE_FAAS_JOB_ID, format!("{}/square", server_url));

        // Register jobs: Job 0 (local), Job 1 (FaaS)
        test_env.add_job(square.layer(TangleLayer)).await;
        test_env.with_faas_executor(XSQUARE_FAAS_JOB_ID, faas_executor);
        test_env.add_job(square_faas.layer(TangleLayer)).await;

        test_env.start(()).await?;

        // Test LOCAL execution (Job 0)
        let job_local = harness
            .submit_job(
                service_id,
                XSQUARE_JOB_ID as u8,
                vec![InputValue::Uint64(5)],
            )
            .await?;
        let call_id_local = job_local.call_id;

        let results_local = harness
            .wait_for_job_execution(service_id, job_local)
            .await?;

        harness.verify_job(&results_local, vec![OutputValue::Uint64(25)]);

        // Test FAAS execution (Job 1)
        let job_faas = harness
            .submit_job(
                service_id,
                XSQUARE_FAAS_JOB_ID as u8,
                vec![InputValue::Uint64(6)],
            )
            .await?;
        let call_id_faas = job_faas.call_id;

        let results_faas = harness.wait_for_job_execution(service_id, job_faas).await?;

        harness.verify_job(&results_faas, vec![OutputValue::Uint64(36)]);

        // Verify both results reached onchain via same consumer pipeline
        assert_eq!(results_local.service_id, service_id);
        assert_eq!(results_faas.service_id, service_id);
        assert!(!results_local.result.is_empty());
        assert!(!results_faas.result.is_empty());

        // The fact that wait_for_job_execution succeeded proves:
        // 1. JobResult created → 2. TangleLayer wrapped → 3. TangleConsumer received
        // 4. Submitted onchain → 5. JobResultSubmitted event emitted → 6. Retrieved from chain

        server_handle.abort();
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_faas_server_directly() -> Result<()> {
        // This test verifies the FaaS server itself works correctly
        let _ = color_eyre::install();
        setup_log();

        let (server_handle, server_url) = start_test_faas_server().await;

        // Create a FaaS payload
        let payload = FaasPayload {
            job_id: XSQUARE_FAAS_JOB_ID,
            args: 7u64.to_le_bytes().to_vec(),
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

        // Verify result
        let result = u64::from_le_bytes(faas_response.result[..8].try_into()?);
        assert_eq!(result, 49); // 7 * 7 = 49

        server_handle.abort();
        Ok(())
    }
}
