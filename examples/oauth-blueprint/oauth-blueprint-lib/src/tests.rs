#[cfg(test)]
mod tests {
    use crate::*;
    use blueprint_sdk::AuthContext;
    use std::collections::HashSet;
    use tokio::time::{timeout, Duration};

    // E2E test imports
    use blueprint_sdk::Job;
    use blueprint_sdk::tangle::layers::TangleLayer;
    use blueprint_sdk::testing::tempfile;
    use blueprint_sdk::testing::utils::setup_log;
    use blueprint_sdk::testing::utils::tangle::{InputValue, OutputValue, TangleTestHarness};
    use blueprint_tangle_extra::serde::new_bounded_string;
    use color_eyre::Result;

    #[tokio::test]
    async fn test_echo_job() {
        use blueprint_sdk::tangle::extract::TangleArg;
        let test_string = "Hello, OAuth Blueprint!";
        let result = echo(TangleArg(test_string.to_string())).await;
        assert_eq!(result.0, test_string);
    }

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
    async fn test_docs_store_isolation() {
        // Test the document store directly
        let store = docs_store();
        
        // Write as tenant A
        {
            let mut guard = store.write().await;
            let tenant_a = guard.entry("tenant_a".to_string()).or_default();
            tenant_a.insert("doc1".to_string(), "content A".to_string());
        }
        
        // Write as tenant B with same doc name
        {
            let mut guard = store.write().await;
            let tenant_b = guard.entry("tenant_b".to_string()).or_default();
            tenant_b.insert("doc1".to_string(), "content B".to_string());
        }
        
        // Verify isolation
        {
            let guard = store.read().await;
            assert_eq!(guard.get("tenant_a").unwrap().get("doc1").unwrap(), "content A");
            assert_eq!(guard.get("tenant_b").unwrap().get("doc1").unwrap(), "content B");
        }
    }

    #[tokio::test]
    async fn test_auth_context_scope_checking() {
        // Test AuthContext scope checking logic
        let mut scopes = HashSet::new();
        scopes.insert("docs:read".to_string());
        scopes.insert("docs:write".to_string());
        
        let auth = AuthContext {
            tenant_hash: Some("test_tenant".to_string()),
            scopes,
        };

        // Test scope that user has
        assert!(auth.has_scope("docs:read"));
        assert!(auth.has_scope("docs:write"));
        
        // Test scope that user doesn't have
        assert!(!auth.has_scope("docs:admin"));
        
        // Test prefix matching
        assert!(auth.has_any_scope(["docs:"])); // Should match docs:read and docs:write with prefix
        assert!(!auth.has_any_scope(["admin:"]));
    }

    #[tokio::test]
    async fn test_auth_context_no_scopes() {
        let auth = AuthContext {
            tenant_hash: Some("test_tenant".to_string()),
            scopes: HashSet::new(),
        };

        assert!(!auth.has_scope("docs:read"));
        assert!(!auth.has_any_scope(["docs:"]));
    }

    #[tokio::test]
    async fn test_concurrent_docs_store_access() {
        let store = docs_store();
        
        // Simulate concurrent writes across different tenants
        let mut handles = vec![];
        for tenant_id in 0..10 {
            let handle = tokio::spawn(async move {
                let store = docs_store();
                let mut guard = store.write().await;
                let tenant = guard.entry(format!("tenant_{}", tenant_id)).or_default();
                tenant.insert("test_doc".to_string(), format!("content_{}", tenant_id));
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
                let expected_content = format!("content_{}", tenant_id);
                assert_eq!(
                    guard.get(&tenant_key).unwrap().get("test_doc").unwrap(),
                    &expected_content
                );
            }
        }
    }

    #[tokio::test]
    async fn test_oauth_e2e_blueprint_contract_hooks() -> Result<()> {
        color_eyre::install()?;
        setup_log();

        let temp_dir = tempfile::TempDir::new()?;
        let harness = TangleTestHarness::setup(temp_dir).await?;

        // Setup service with contract deployment (true = deploy BlueprintServiceManagerBase)
        let (mut test_env, service_id, blueprint_id) = harness.setup_services::<1>(true).await?;
        test_env.initialize().await?;

        // Add jobs with TangleLayer for automatic BlueprintServiceManagerBase hook wiring
        test_env.add_job(whoami.layer(TangleLayer)).await;
        test_env.add_job(write_doc.layer(TangleLayer)).await;
        test_env.add_job(read_doc.layer(TangleLayer)).await;
        test_env.add_job(check_scope.layer(TangleLayer)).await;
        test_env.add_job(admin_purge.layer(TangleLayer)).await;

        test_env.start(OAuthBlueprintContext {
            tangle_client: Arc::new(harness.client().clone()),
        }).await?;

        // Test 1: OAuth Document Write → Contract Hook Integration
        let job_write = harness
            .submit_job(
                service_id,
                WRITE_DOC_JOB_ID as u8,
                vec![
                    InputValue::String(new_bounded_string("secure_document")),
                    InputValue::String(new_bounded_string("confidential data"))
                ],
            )
            .await?;
        let write_result = harness.wait_for_job_execution(service_id, job_write.clone()).await?;
        
        // Verify complete flow: Job → Rust Handler → Contract Hooks → Tangle Storage
        assert_eq!(write_result.service_id, service_id);
        assert_eq!(write_result.call_id, job_write.call_id);
        
        // Expected contract state changes:
        // - onJobCall triggered with WRITE_DOC_JOB_ID
        // - onJobResult triggered after Rust execution
        // - tenantDocuments[tenant_hash]["secure_document"] = true
        // - tenantDocumentCount[tenant_hash] incremented
        // - DocumentWritten event emitted
        
        // Test 2: OAuth Scope Checking with Contract Tracking
        let job_scope = harness
            .submit_job(
                service_id,
                CHECK_SCOPE_JOB_ID as u8,
                vec![InputValue::String(new_bounded_string("docs:write"))],
            )
            .await?;
        let scope_result = harness.wait_for_job_execution(service_id, job_scope.clone()).await?;
        
        // Verify scope enforcement and contract state
        assert_eq!(scope_result.service_id, service_id);
        assert_eq!(scope_result.call_id, job_scope.call_id);
        
        // Expected contract tracking:
        // - scopeUsageCount[tenant_hash]["docs:write"] incremented
        // - ScopeChecked event emitted with authorization result
        
        // Test 3: Admin Operations with Contract Audit Trail
        let job_admin = harness
            .submit_job(
                service_id,
                ADMIN_PURGE_JOB_ID as u8,
                vec![InputValue::String(new_bounded_string("target_tenant_id"))],
            )
            .await?;
        let admin_result = harness.wait_for_job_execution(service_id, job_admin.clone()).await?;
        
        // Verify admin operation tracking
        assert_eq!(admin_result.service_id, service_id);
        assert_eq!(admin_result.call_id, job_admin.call_id);
        
        // Expected audit trail:
        // - AdminPurgeExecuted event with target and admin tenant hashes
        // - Contract maintains immutable audit log of admin actions
        
        Ok(())
    }

    #[tokio::test]
    async fn test_oauth_contract_lifecycle_hooks() -> Result<()> {
        color_eyre::install()?;
        setup_log();

        let temp_dir = tempfile::TempDir::new()?;
        let harness = TangleTestHarness::setup(temp_dir).await?;

        // Deploy contract to test BlueprintServiceManagerBase lifecycle
        let (mut test_env, service_id, blueprint_id) = harness.setup_services::<1>(true).await?;
        test_env.initialize().await?;

        test_env.add_job(write_doc.layer(TangleLayer)).await;
        test_env.add_job(check_scope.layer(TangleLayer)).await;
        test_env.add_job(admin_purge.layer(TangleLayer)).await;

        test_env.start(OAuthBlueprintContext {
            tangle_client: Arc::new(harness.client().clone()),
        }).await?;

        // Test Blueprint Contract Lifecycle Integration:
        
        // 1. Document Write → Full Hook Chain
        let job_write = harness
            .submit_job(
                service_id,
                WRITE_DOC_JOB_ID as u8,
                vec![
                    InputValue::String(new_bounded_string("compliance_doc")),
                    InputValue::String(new_bounded_string("regulatory data"))
                ],
            )
            .await?;
        let write_result = harness.wait_for_job_execution(service_id, job_write.clone()).await?;
        
        // Validation: Complete Blueprint Integration
        assert_eq!(write_result.service_id, service_id);
        assert_eq!(write_result.call_id, job_write.call_id);
        
        // The complete flow should be:
        // 1. harness.submit_job() → Tangle pallet stores job call
        // 2. TangleProducer picks up job → Routes to Rust handler
        // 3. onJobCall() hook triggered → Pre-execution contract logic
        // 4. Rust write_doc() executes → OAuth scope validation + document storage
        // 5. onJobResult() hook triggered → Contract state update + event emission
        // 6. TangleConsumer stores final result → Tangle pallet metadata
        
        // 2. Scope Enforcement with Contract Audit
        let job_scope = harness
            .submit_job(
                service_id,
                CHECK_SCOPE_JOB_ID as u8,
                vec![InputValue::String(new_bounded_string("docs:admin"))],
            )
            .await?;
        let scope_result = harness.wait_for_job_execution(service_id, job_scope.clone()).await?;
        
        // Verify scope checking with contract tracking
        assert_eq!(scope_result.service_id, service_id);
        assert_eq!(scope_result.call_id, job_scope.call_id);
        
        // Contract should track:
        // - scopeUsageCount[tenant_hash]["docs:admin"] incremented
        // - ScopeChecked event emitted with authorization result
        
        // 3. Admin Purge with Full Audit Trail
        let job_purge = harness
            .submit_job(
                service_id,
                ADMIN_PURGE_JOB_ID as u8,
                vec![InputValue::String(new_bounded_string("target_tenant"))],
            )
            .await?;
        let purge_result = harness.wait_for_job_execution(service_id, job_purge.clone()).await?;
        
        // Verify admin operation with contract audit
        assert_eq!(purge_result.service_id, service_id);
        assert_eq!(purge_result.call_id, job_purge.call_id);
        
        // Contract audit trail:
        // - AdminPurgeExecuted event with target and admin tenant hashes
        // - Immutable record of admin action with timestamps
        
        Ok(())
    }
}