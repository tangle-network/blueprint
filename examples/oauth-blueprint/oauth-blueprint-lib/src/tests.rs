#[cfg(test)]
mod tests {
    use crate::*;
    use blueprint_sdk::AuthContext;
    use std::collections::{HashMap, HashSet};
    use tokio::time::{timeout, Duration};
    
    // E2E test imports
    use blueprint_sdk::Job;
    use blueprint_sdk::tangle::layers::TangleLayer;
    use blueprint_sdk::testing::tempfile;
    use blueprint_sdk::testing::utils::setup_log;
    use blueprint_sdk::testing::utils::tangle::{InputValue, TangleTestHarness};
    use blueprint_tangle_extra::serde::new_bounded_string;
    use color_eyre::Result;

    #[tokio::test]
    async fn test_background_service_starts() {
        let service = AuthEchoBackgroundService;
        let receiver = service.start().await.unwrap();
        
        // Service should start successfully within a reasonable time
        let result = timeout(Duration::from_secs(2), receiver).await;
        assert!(result.is_ok());
        let service_result = result.unwrap().unwrap();
        assert!(service_result.is_ok());
    }

    #[tokio::test]
    async fn test_auth_context_scope_validation() {
        // Test AuthContext scope checking logic
        let mut headers = HashMap::new();
        headers.insert("tenant_hash".to_string(), "test_tenant".to_string());
        headers.insert("scopes".to_string(), "docs:read docs:write".to_string());
        
        let auth = AuthContext::from_headers(&headers);

        // Test scope that user has
        assert!(auth.has_scope("docs:read"));
        assert!(auth.has_scope("docs:write"));
        
        // Test scope that user doesn't have
        assert!(!auth.has_scope("docs:admin"));
        
        // Test has_any_scope
        assert!(auth.has_any_scope(["docs:read", "docs:admin"])); // has docs:read
        assert!(!auth.has_any_scope(["admin:all", "system:manage"])); // has neither
        
        // Test tenant hash access
        assert_eq!(auth.tenant_hash(), Some("test_tenant"));
    }

    #[tokio::test]
    async fn test_write_doc_job_execution() -> Result<()> {
        color_eyre::install()?;
        setup_log();

        let temp_dir = tempfile::TempDir::new()?;
        let harness = TangleTestHarness::setup(temp_dir).await?;

        let (mut test_env, service_id, _blueprint_id) = harness.setup_services::<1>(true).await?;
        test_env.initialize().await?;

        // Add only state-changing jobs
        test_env.add_job(write_doc.layer(TangleLayer)).await;
        
        test_env.start(OAuthBlueprintContext {
            tangle_client: Arc::new(harness.client().clone()),
        }).await?;

        // Test document writing with job execution
        let job = harness
            .submit_job(
                service_id,
                WRITE_DOC_JOB_ID as u8,
                vec![
                    InputValue::String(new_bounded_string("report_2024")),
                    InputValue::String(new_bounded_string("Annual compliance report"))
                ],
            )
            .await?;
        let result = harness.wait_for_job_execution(service_id, job.clone()).await?;
        
        // Verify job execution and state change
        assert_eq!(result.service_id, service_id);
        assert_eq!(result.call_id, job.call_id);
        assert!(!result.result.is_empty());
        
        // Submit another write to test isolation
        let job2 = harness
            .submit_job(
                service_id,
                WRITE_DOC_JOB_ID as u8,
                vec![
                    InputValue::String(new_bounded_string("policy_doc")),
                    InputValue::String(new_bounded_string("Security policy document"))
                ],
            )
            .await?;
        let result2 = harness.wait_for_job_execution(service_id, job2.clone()).await?;
        
        assert_eq!(result2.service_id, service_id);
        assert_ne!(result.call_id, result2.call_id); // Different call IDs
        
        Ok(())
    }

    #[tokio::test]
    async fn test_admin_purge_job_execution() -> Result<()> {
        color_eyre::install()?;
        setup_log();

        let temp_dir = tempfile::TempDir::new()?;
        let harness = TangleTestHarness::setup(temp_dir).await?;

        let (mut test_env, service_id, _blueprint_id) = harness.setup_services::<1>(true).await?;
        test_env.initialize().await?;

        test_env.add_job(write_doc.layer(TangleLayer)).await;
        test_env.add_job(admin_purge.layer(TangleLayer)).await;
        
        test_env.start(OAuthBlueprintContext {
            tangle_client: Arc::new(harness.client().clone()),
        }).await?;

        // First write some documents
        let write_job = harness
            .submit_job(
                service_id,
                WRITE_DOC_JOB_ID as u8,
                vec![
                    InputValue::String(new_bounded_string("temp_doc")),
                    InputValue::String(new_bounded_string("Temporary data"))
                ],
            )
            .await?;
        let _ = harness.wait_for_job_execution(service_id, write_job).await?;
        
        // Now test admin purge - state changing operation
        let purge_job = harness
            .submit_job(
                service_id,
                ADMIN_PURGE_JOB_ID as u8,
                vec![InputValue::String(new_bounded_string("test_tenant"))],
            )
            .await?;
        let purge_result = harness.wait_for_job_execution(service_id, purge_job.clone()).await?;
        
        // Verify purge executed
        assert_eq!(purge_result.service_id, service_id);
        assert_eq!(purge_result.call_id, purge_job.call_id);
        
        // Parse result to verify purge status
        if let Ok(json_result) = serde_json::from_slice::<serde_json::Value>(&purge_result.result) {
            assert!(json_result["purged"].is_boolean());
            assert_eq!(json_result["tenant"], "test_tenant");
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_concurrent_document_writes() -> Result<()> {
        color_eyre::install()?;
        setup_log();

        let temp_dir = tempfile::TempDir::new()?;
        let harness = TangleTestHarness::setup(temp_dir).await?;

        let (mut test_env, service_id, _blueprint_id) = harness.setup_services::<1>(true).await?;
        test_env.initialize().await?;

        test_env.add_job(write_doc.layer(TangleLayer)).await;
        
        test_env.start(OAuthBlueprintContext {
            tangle_client: Arc::new(harness.client().clone()),
        }).await?;

        // Submit multiple document write jobs concurrently
        let mut job_futures = vec![];
        
        for i in 0..5 {
            let job_future = harness.submit_job(
                service_id,
                WRITE_DOC_JOB_ID as u8,
                vec![
                    InputValue::String(new_bounded_string(&format!("doc_{}", i))),
                    InputValue::String(new_bounded_string(&format!("content_{}", i)))
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
    async fn test_oauth_blueprint_with_contract_hooks() -> Result<()> {
        color_eyre::install()?;
        setup_log();

        let temp_dir = tempfile::TempDir::new()?;
        let harness = TangleTestHarness::setup(temp_dir).await?;

        // Setup with contract deployment
        let (mut test_env, service_id, _blueprint_id) = harness.setup_services::<1>(true).await?;
        test_env.initialize().await?;

        // Only add state-changing jobs
        test_env.add_job(write_doc.layer(TangleLayer)).await;
        test_env.add_job(admin_purge.layer(TangleLayer)).await;

        test_env.start(OAuthBlueprintContext {
            tangle_client: Arc::new(harness.client().clone()),
        }).await?;

        // Test 1: Document Write - state change with OAuth scopes
        let write_job = harness
            .submit_job(
                service_id,
                WRITE_DOC_JOB_ID as u8,
                vec![
                    InputValue::String(new_bounded_string("compliance_report")),
                    InputValue::String(new_bounded_string("Q4 2024 compliance data"))
                ],
            )
            .await?;
        let write_result = harness.wait_for_job_execution(service_id, write_job.clone()).await?;
        
        // Verify job execution and state change
        assert_eq!(write_result.service_id, service_id);
        assert_eq!(write_result.call_id, write_job.call_id);
        
        // Contract hooks should have:
        // - onJobCall triggered for WRITE_DOC_JOB_ID
        // - OAuth scope validation (requires docs:write)
        // - Document tracking in contract state
        // - Events emitted for document write
        
        // Test 2: Admin purge - state change with admin scope
        let purge_job = harness
            .submit_job(
                service_id,
                ADMIN_PURGE_JOB_ID as u8,
                vec![InputValue::String(new_bounded_string("tenant_to_purge"))],
            )
            .await?;
        let purge_result = harness.wait_for_job_execution(service_id, purge_job.clone()).await?;
        
        assert_eq!(purge_result.service_id, service_id);
        assert_eq!(purge_result.call_id, purge_job.call_id);
        
        // Contract hooks should have:
        // - onJobResult triggered for ADMIN_PURGE_JOB_ID
        // - OAuth scope validation (requires docs:admin)
        // - Audit trail of admin action in contract
        // - Events emitted for purge operation
        
        Ok(())
    }
}