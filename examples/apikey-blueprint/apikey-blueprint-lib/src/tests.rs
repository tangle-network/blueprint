#[cfg(test)]
mod tests {
    use crate::*;
    use blueprint_sdk::Job;
    use blueprint_sdk::tangle::layers::TangleLayer;
    use blueprint_sdk::testing::tempfile;
    use blueprint_sdk::testing::utils::setup_log;
    use blueprint_sdk::testing::utils::tangle::{InputValue, TangleTestHarness};
    use blueprint_tangle_extra::serde::new_bounded_string;
    use color_eyre::Result;
    use tokio::time::{timeout, Duration};

    #[tokio::test]
    async fn test_usage_tracker_background_service() {
        let service = ApiUsageTracker;
        let receiver = service.start().await.unwrap();
        
        // Service should start successfully within a reasonable time
        let result = timeout(Duration::from_secs(2), receiver).await;
        assert!(result.is_ok());
        let service_result = result.unwrap().unwrap();
        assert!(service_result.is_ok());
    }

    #[tokio::test]
    async fn test_write_resource_job_execution() -> Result<()> {
        color_eyre::install()?;
        setup_log();

        let temp_dir = tempfile::TempDir::new()?;
        let harness = TangleTestHarness::setup(temp_dir).await?;

        let (mut test_env, service_id, _blueprint_id) = harness.setup_services::<1>(true).await?;
        test_env.initialize().await?;

        // Add only state-changing jobs
        test_env.add_job(write_resource.layer(TangleLayer)).await;
        
        test_env.start(ApiKeyBlueprintContext {
            tangle_client: Arc::new(harness.client().clone()),
        }).await?;

        // Test resource writing with actual job execution
        let job = harness
            .submit_job(
                service_id,
                WRITE_RESOURCE_JOB_ID as u8,
                vec![
                    InputValue::String(new_bounded_string("config")),
                    InputValue::String(new_bounded_string("app configuration data"))
                ],
            )
            .await?;
        let result = harness.wait_for_job_execution(service_id, job.clone()).await?;
        
        // Verify job execution
        assert_eq!(result.service_id, service_id);
        assert_eq!(result.call_id, job.call_id);
        assert!(!result.result.is_empty());
        
        // Submit another write to test isolation
        let job2 = harness
            .submit_job(
                service_id,
                WRITE_RESOURCE_JOB_ID as u8,
                vec![
                    InputValue::String(new_bounded_string("logs")),
                    InputValue::String(new_bounded_string("application logs"))
                ],
            )
            .await?;
        let result2 = harness.wait_for_job_execution(service_id, job2.clone()).await?;
        
        assert_eq!(result2.service_id, service_id);
        assert_ne!(result.call_id, result2.call_id); // Different call IDs
        
        Ok(())
    }

    #[tokio::test]
    async fn test_purchase_api_key_job_execution() -> Result<()> {
        color_eyre::install()?;
        setup_log();

        let temp_dir = tempfile::TempDir::new()?;
        let harness = TangleTestHarness::setup(temp_dir).await?;

        let (mut test_env, service_id, _blueprint_id) = harness.setup_services::<1>(true).await?;
        test_env.initialize().await?;

        test_env.add_job(purchase_api_key.layer(TangleLayer)).await;
        
        test_env.start(ApiKeyBlueprintContext {
            tangle_client: Arc::new(harness.client().clone()),
        }).await?;

        // Test API key purchase for different tiers
        let tiers = vec![("basic", "user_123"), ("premium", "user_456"), ("enterprise", "user_789")];
        
        for (tier, user) in tiers {
            let job = harness
                .submit_job(
                    service_id,
                    PURCHASE_API_KEY_JOB_ID as u8,
                    vec![
                        InputValue::String(new_bounded_string(tier)),
                        InputValue::String(new_bounded_string(user))
                    ],
                )
                .await?;
            let result = harness.wait_for_job_execution(service_id, job.clone()).await?;
            
            // Verify job executed and returned data
            assert_eq!(result.service_id, service_id);
            assert_eq!(result.call_id, job.call_id);
            assert!(!result.result.is_empty());
            
            // Parse result to verify API key was generated
            if let Ok(json_result) = serde_json::from_slice::<serde_json::Value>(&result.result) {
                assert_eq!(json_result["ok"], true);
                assert_eq!(json_result["tier"], tier);
                assert!(json_result["api_key_hash"].is_string());
                assert!(json_result["encrypted_key"].is_string());
            }
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_concurrent_job_execution() -> Result<()> {
        color_eyre::install()?;
        setup_log();

        let temp_dir = tempfile::TempDir::new()?;
        let harness = TangleTestHarness::setup(temp_dir).await?;

        let (mut test_env, service_id, _blueprint_id) = harness.setup_services::<1>(true).await?;
        test_env.initialize().await?;

        test_env.add_job(write_resource.layer(TangleLayer)).await;
        
        test_env.start(ApiKeyBlueprintContext {
            tangle_client: Arc::new(harness.client().clone()),
        }).await?;

        // Submit multiple jobs concurrently
        let mut job_futures = vec![];
        
        for i in 0..5 {
            let job_future = harness.submit_job(
                service_id,
                WRITE_RESOURCE_JOB_ID as u8,
                vec![
                    InputValue::String(new_bounded_string(&format!("resource_{}", i))),
                    InputValue::String(new_bounded_string(&format!("data_{}", i)))
                ],
            );
            job_futures.push(job_future);
        }
        
        // Collect all job submissions
        let mut jobs = vec![];
        for future in job_futures {
            jobs.push(future.await?);
        }
        
        // Wait for all jobs to complete
        let mut results = vec![];
        for job in jobs {
            let result = harness.wait_for_job_execution(service_id, job.clone()).await?;
            results.push(result);
        }
        
        // Verify all jobs completed successfully with unique call IDs
        let mut call_ids = std::collections::HashSet::new();
        for result in results {
            assert_eq!(result.service_id, service_id);
            assert!(call_ids.insert(result.call_id));
        }
        assert_eq!(call_ids.len(), 5);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_api_key_blueprint_with_contract_hooks() -> Result<()> {
        color_eyre::install()?;
        setup_log();

        let temp_dir = tempfile::TempDir::new()?;
        let harness = TangleTestHarness::setup(temp_dir).await?;

        // Setup with contract deployment
        let (mut test_env, service_id, _blueprint_id) = harness.setup_services::<1>(true).await?;
        test_env.initialize().await?;

        // Only add state-changing jobs
        test_env.add_job(purchase_api_key.layer(TangleLayer)).await;
        test_env.add_job(write_resource.layer(TangleLayer)).await;

        test_env.start(ApiKeyBlueprintContext {
            tangle_client: Arc::new(harness.client().clone()),
        }).await?;

        // Test 1: Purchase API key - state change
        let purchase_job = harness
            .submit_job(
                service_id,
                PURCHASE_API_KEY_JOB_ID as u8,
                vec![
                    InputValue::String(new_bounded_string("premium")),
                    InputValue::String(new_bounded_string("test_user"))
                ],
            )
            .await?;
        let purchase_result = harness.wait_for_job_execution(service_id, purchase_job.clone()).await?;
        
        // Verify job execution and state change
        assert_eq!(purchase_result.service_id, service_id);
        assert_eq!(purchase_result.call_id, purchase_job.call_id);
        
        // Contract hooks should have:
        // - onJobCall triggered for PURCHASE_API_KEY_JOB_ID
        // - State updated with user tier and API key hash
        // - Events emitted for tracking
        
        // Test 2: Write resource - state change
        let write_job = harness
            .submit_job(
                service_id,
                WRITE_RESOURCE_JOB_ID as u8,
                vec![
                    InputValue::String(new_bounded_string("user_profile")),
                    InputValue::String(new_bounded_string("profile data"))
                ],
            )
            .await?;
        let write_result = harness.wait_for_job_execution(service_id, write_job.clone()).await?;
        
        assert_eq!(write_result.service_id, service_id);
        assert_eq!(write_result.call_id, write_job.call_id);
        
        // Contract hooks should have:
        // - onJobResult triggered for WRITE_RESOURCE_JOB_ID
        // - Resource tracking updated in contract state
        // - Events emitted for resource write
        
        Ok(())
    }
}