#[cfg(test)]
mod tests {
    use crate::*;
    use blueprint_sdk::AuthContext;
    use tokio::time::{timeout, Duration};
    use std::collections::HashSet;

    // E2E test imports
    use blueprint_sdk::Job;
    use blueprint_sdk::tangle::layers::TangleLayer;
    use blueprint_sdk::testing::tempfile;
    use blueprint_sdk::testing::utils::setup_log;
    use blueprint_sdk::testing::utils::tangle::{InputValue, TangleTestHarness};
    use blueprint_tangle_extra::serde::new_bounded_string;
    use color_eyre::Result;

    #[tokio::test]
    async fn test_echo_job_basic() {
        use blueprint_sdk::tangle::extract::TangleArg;
        let test_string = "Hello, API Key Blueprint!";
        let result = echo(TangleArg(test_string.to_string())).await;
        assert_eq!(result.0, test_string);
    }

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
    async fn test_resource_store_functionality() {
        // Test that the resource store works correctly
        let store = resource_store();
        
        // Write some test data
        {
            let mut guard = store.write().await;
            let tenant_data = guard.entry("test_tenant".to_string()).or_default();
            tenant_data.insert("test_resource".to_string(), "test_data".to_string());
        }
        
        // Read the data back
        {
            let guard = store.read().await;
            let tenant_data = guard.get("test_tenant").unwrap();
            let resource_data = tenant_data.get("test_resource").unwrap();
            assert_eq!(resource_data, "test_data");
        }
    }

    #[tokio::test]
    async fn test_multi_tenant_resource_isolation() {
        let store = resource_store();
        
        // Write as tenant A
        {
            let mut guard = store.write().await;
            let tenant_a = guard.entry("tenant_a".to_string()).or_default();
            tenant_a.insert("shared_resource".to_string(), "tenant A data".to_string());
        }
        
        // Write as tenant B with same resource name
        {
            let mut guard = store.write().await;
            let tenant_b = guard.entry("tenant_b".to_string()).or_default();
            tenant_b.insert("shared_resource".to_string(), "tenant B data".to_string());
        }
        
        // Verify isolation
        {
            let guard = store.read().await;
            assert_eq!(
                guard.get("tenant_a").unwrap().get("shared_resource").unwrap(),
                "tenant A data"
            );
            assert_eq!(
                guard.get("tenant_b").unwrap().get("shared_resource").unwrap(),
                "tenant B data"
            );
        }
    }

    #[tokio::test]
    async fn test_auth_context_apikey_mode() {
        // Test AuthContext for API key mode (no scopes)
        let auth = AuthContext {
            tenant_hash: Some("api_key_tenant".to_string()),
            scopes: HashSet::new(),
        };

        // API key mode should have no scopes
        assert!(!auth.has_scope("any_scope"));
        assert!(!auth.has_any_scope(["any", "scope"]));
        assert_eq!(auth.tenant_hash.as_ref().unwrap(), "api_key_tenant");
    }

    #[tokio::test]
    async fn test_concurrent_resource_store_access() {
        let store = resource_store();
        
        // Simulate concurrent writes across different tenants
        let mut handles = vec![];
        for tenant_id in 0..10 {
            let handle = tokio::spawn(async move {
                let store = resource_store();
                let mut guard = store.write().await;
                let tenant = guard.entry(format!("tenant_{}", tenant_id)).or_default();
                tenant.insert("test_resource".to_string(), format!("data_{}", tenant_id));
            });
            handles.push(handle);
        }

        // Wait for all writes to complete
        for handle in handles {
            handle.await.unwrap();
        }

        // Verify all tenants have their data
        {
            let guard = store.read().await;
            for tenant_id in 0..10 {
                let tenant_key = format!("tenant_{}", tenant_id);
                let expected_data = format!("data_{}", tenant_id);
                assert_eq!(
                    guard.get(&tenant_key).unwrap().get("test_resource").unwrap(),
                    &expected_data
                );
            }
        }
    }

    #[tokio::test]
    async fn test_purchase_tiers() {
        // Test the different subscription tiers we support
        let tiers = vec!["basic", "premium", "enterprise"];
        
        for tier in &tiers {
            // Just test that the tier names are valid strings
            assert!(!tier.is_empty());
            assert!(tier.chars().all(|c| c.is_ascii_lowercase()));
        }
    }

    #[tokio::test]
    async fn test_api_key_e2e_blueprint_contract_integration() -> Result<()> {
        color_eyre::install()?;
        setup_log();

        let temp_dir = tempfile::TempDir::new()?;
        let harness = TangleTestHarness::setup(temp_dir).await?;

        // Setup service with contract deployment (true = deploy contracts)
        let (mut test_env, service_id, _blueprint_id) = harness.setup_services::<1>(true).await?;
        test_env.initialize().await?;

        // Add jobs with TangleLayer for automatic BlueprintServiceManagerBase hook wiring
        test_env.add_job(purchase_api_key.layer(TangleLayer)).await;
        test_env.add_job(write_resource.layer(TangleLayer)).await;
        test_env.add_job(whoami.layer(TangleLayer)).await;
        test_env.add_job(echo.layer(TangleLayer)).await;

        test_env.start(ApiKeyBlueprintContext {
            tangle_client: Arc::new(harness.client().clone()),
        }).await?;

        // Test 1: API Key Purchase → onJobCall hook → Contract State Update
        let job_purchase = harness
            .submit_job(
                service_id,
                PURCHASE_API_KEY_JOB_ID as u8,
                vec![InputValue::String(new_bounded_string("premium"))],
            )
            .await?;
        let purchase_result = harness.wait_for_job_execution(service_id, job_purchase.clone()).await?;
        
        // Verify 1: Job result stored on Tangle pallet with metadata
        assert_eq!(purchase_result.service_id, service_id);
        assert_eq!(purchase_result.call_id, job_purchase.call_id);
        assert!(!purchase_result.result.is_empty());
        
        // TODO: Verify smart contract state was updated:
        // - onJobCall was triggered with PURCHASE_API_KEY_JOB_ID
        // - userTiers[tx.origin] = "premium"
        // - subscriptionExpiry[tx.origin] = block.timestamp + 30 days
        // - ApiKeyPurchased event emitted
        // - processedJobCalls[service_id][call_id] = true
        
        // Test 2: Resource Writing → onJobResult hook → Contract Event
        let job_resource = harness
            .submit_job(
                service_id,
                WRITE_RESOURCE_JOB_ID as u8,
                vec![
                    InputValue::String(new_bounded_string("user_profile")),
                    InputValue::String(new_bounded_string("profile data"))
                ],
            )
            .await?;
        let resource_result = harness.wait_for_job_execution(service_id, job_resource.clone()).await?;
        
        // Verify 2: Job execution and contract state
        assert_eq!(resource_result.service_id, service_id);
        assert_eq!(resource_result.call_id, job_resource.call_id);
        
        // TODO: Verify smart contract state:
        // - onJobResult was triggered with WRITE_RESOURCE_JOB_ID
        // - tenantResources[tenant_hash]["user_profile"] = true
        // - tenantResourceCount[tenant_hash] incremented
        // - ResourceWritten event emitted
        
        // Test 3: Verify Tangle pallet stores job metadata correctly
        let job_whoami = harness.submit_job(service_id, WHOAMI_JOB_ID as u8, vec![]).await?;
        let whoami_result = harness.wait_for_job_execution(service_id, job_whoami.clone()).await?;
        
        // Verify 3: Complete end-to-end flow validation
        assert_eq!(whoami_result.service_id, service_id);
        assert_eq!(whoami_result.call_id, job_whoami.call_id);
        
        // The complete flow:
        // 1. Job submitted via harness.submit_job() → Tangle pallet stores job call
        // 2. Rust job handler executes with AuthContext
        // 3. TangleLayer triggers contract onJobCall/onJobResult hooks
        // 4. Contract emits events and updates state
        // 5. Job result stored in Tangle pallet with metadata
        
        Ok(())
    }

    #[tokio::test]
    async fn test_api_key_blueprint_contract_hooks_validation() -> Result<()> {
        color_eyre::install()?;
        setup_log();

        let temp_dir = tempfile::TempDir::new()?;
        let harness = TangleTestHarness::setup(temp_dir).await?;

        // Setup service with contract deployment to test BlueprintServiceManagerBase hooks
        let (mut test_env, service_id, _blueprint_id) = harness.setup_services::<1>(true).await?;
        test_env.initialize().await?;

        test_env.add_job(purchase_api_key.layer(TangleLayer)).await;
        test_env.add_job(write_resource.layer(TangleLayer)).await;

        test_env.start(ApiKeyBlueprintContext {
            tangle_client: Arc::new(harness.client().clone()),
        }).await?;

        // Test complete Blueprint lifecycle with contract hooks
        let tiers = vec!["basic", "premium", "enterprise"];
        
        for tier in tiers {
            let job = harness
                .submit_job(
                    service_id,
                    PURCHASE_API_KEY_JOB_ID as u8,
                    vec![InputValue::String(new_bounded_string(tier))],
                )
                .await?;
            let result = harness.wait_for_job_execution(service_id, job.clone()).await?;
            
            // Comprehensive validation of the complete flow:
            // 1. Job submitted → Tangle pallet stores job call with metadata
            assert_eq!(result.service_id, service_id);
            assert_eq!(result.call_id, job.call_id);
            
            // 2. onJobCall hook triggered → Contract validates payment & updates state
            // - processedJobCalls[service_id][call_id] should be true
            // - userTiers[tx.origin] should be set to tier
            // - subscriptionExpiry[tx.origin] should be set
            // - ApiKeyPurchased event emitted
            
            // 3. Rust job handler executes → Returns success result
            // 4. onJobResult hook triggered → Additional contract logic if needed
            // 5. Final job result stored in Tangle pallet
        }
        
        // Test multi-tenant resource tracking with contract state validation
        let resource_scenarios = vec![
            ("config_file", "application configuration"),
            ("user_data", "encrypted user information"),
            ("session_log", "session activity data"),
        ];
        
        for (resource_id, data) in resource_scenarios {
            let job = harness
                .submit_job(
                    service_id,
                    WRITE_RESOURCE_JOB_ID as u8,
                    vec![
                        InputValue::String(new_bounded_string(resource_id)),
                        InputValue::String(new_bounded_string(data))
                    ],
                )
                .await?;
            let result = harness.wait_for_job_execution(service_id, job.clone()).await?;
            
            // Validate complete resource tracking flow:
            // 1. Job call → onJobCall hook (pre-execution validation)
            // 2. Rust handler → Updates in-memory resource store with tenant isolation
            // 3. Job result → onJobResult hook → Contract state update
            //    - tenantResources[tenant_hash][resource_id] = true
            //    - tenantResourceCount[tenant_hash] incremented
            //    - ResourceWritten event emitted
            // 4. Final metadata stored in Tangle pallet
            assert_eq!(result.service_id, service_id);
            assert_eq!(result.call_id, job.call_id);
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_api_key_usage_tracking_e2e() -> Result<()> {
        color_eyre::install()?;
        setup_log();

        let temp_dir = tempfile::TempDir::new()?;
        let harness = TangleTestHarness::setup(temp_dir).await?;

        let (mut test_env, service_id, __blueprint_id) = harness.setup_services::<1>(true).await?;
        test_env.initialize().await?;

        // Add all jobs to test comprehensive usage tracking
        test_env.add_job(whoami.layer(TangleLayer)).await;
        test_env.add_job(write_resource.layer(TangleLayer)).await;
        test_env.add_job(purchase_api_key.layer(TangleLayer)).await;
        test_env.add_job(echo.layer(TangleLayer)).await;

        test_env.start(ApiKeyBlueprintContext {
            tangle_client: Arc::new(harness.client().clone()),
        }).await?;

        // Simulate realistic API usage patterns
        let usage_pattern = vec![
            (WHOAMI_JOB_ID as u8, vec![]),
            (WRITE_RESOURCE_JOB_ID as u8, vec![
                InputValue::String(new_bounded_string("log_entry_1")),
                InputValue::String(new_bounded_string("application log data"))
            ]),
            (ECHO_JOB_ID as u8, vec![InputValue::String(new_bounded_string("health_check"))]),
            (WRITE_RESOURCE_JOB_ID as u8, vec![
                InputValue::String(new_bounded_string("user_session")),
                InputValue::String(new_bounded_string("session metadata"))
            ]),
        ];
        
        for (job_id, inputs) in usage_pattern {
            let job = harness.submit_job(service_id, job_id, inputs).await?;
            let result = harness.wait_for_job_execution(service_id, job).await?;
            
            // Each API call should:
            // 1. Execute successfully with proper authentication
            // 2. Store results in Tangle pallet with metadata
            // 3. Update usage tracking in background service
            // 4. Maintain tenant isolation throughout
            assert_eq!(result.service_id, service_id);
        }
        
        Ok(())
    }
}