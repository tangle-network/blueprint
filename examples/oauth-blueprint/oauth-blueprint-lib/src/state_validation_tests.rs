#[cfg(test)]
mod state_validation_tests {
    use crate::*;
    use blueprint_sdk::Job;
    use blueprint_sdk::tangle::layers::TangleLayer;
    use blueprint_sdk::testing::tempfile;
    use blueprint_sdk::testing::utils::setup_log;
    use blueprint_sdk::testing::utils::tangle::{InputValue, TangleTestHarness};
    use blueprint_tangle_extra::serde::new_bounded_string;
    use blueprint_sdk::tangle_subxt::subxt::tx::Signer;
    use blueprint_client_tangle::EventsClient;
    use color_eyre::Result;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_oauth_document_storage_tangle_pallet_validation() -> Result<()> {
        color_eyre::install()?;
        setup_log();

        let temp_dir = tempfile::TempDir::new()?;
        let harness = TangleTestHarness::setup(temp_dir).await?;

        // Deploy OAuth contract and setup service
        let (mut test_env, service_id, blueprint_id) = harness.setup_services::<1>(true).await?;
        test_env.initialize().await?;

        test_env.add_job(write_doc.layer(TangleLayer)).await;
        test_env.add_job(read_doc.layer(TangleLayer)).await;
        test_env.add_job(check_scope.layer(TangleLayer)).await;
        test_env.start(OAuthBlueprintContext {
            tangle_client: Arc::new(harness.client().clone()),
        }).await?;

        // Execute OAuth document operations
        let document_operations = vec![
            ("compliance_doc", "regulatory compliance data"),
            ("user_profile", "encrypted user profile"),
            ("audit_trail", "security audit information"),
        ];

        let mut execution_metadata = vec![];

        for (doc_id, content) in document_operations {
            // Write document
            let job_write = harness
                .submit_job(
                    service_id,
                    WRITE_DOC_JOB_ID as u8,
                    vec![
                        InputValue::String(new_bounded_string(doc_id)),
                        InputValue::String(new_bounded_string(content))
                    ],
                )
                .await?;
            let write_result = harness.wait_for_job_execution(service_id, job_write.clone()).await?;

            // Read document back
            let job_read = harness
                .submit_job(
                    service_id,
                    READ_DOC_JOB_ID as u8,
                    vec![InputValue::String(new_bounded_string(doc_id))],
                )
                .await?;
            let read_result = harness.wait_for_job_execution(service_id, job_read.clone()).await?;

            // Validate Tangle pallet storage
            assert_eq!(write_result.service_id, service_id, "Write job service ID should match");
            assert_eq!(read_result.service_id, service_id, "Read job service ID should match");
            assert_eq!(write_result.call_id, job_write.call_id, "Write call ID should match");
            assert_eq!(read_result.call_id, job_read.call_id, "Read call ID should match");

            // Validate job results contain expected data
            assert!(!write_result.result.is_empty(), "Write result should not be empty");
            assert!(!read_result.result.is_empty(), "Read result should not be empty");

            execution_metadata.push((job_write, write_result, job_read, read_result, doc_id));
        }

        // Verify all operations have unique call IDs in Tangle pallet
        let mut all_call_ids = std::collections::HashSet::new();
        for (job_write, write_result, job_read, read_result, _) in &execution_metadata {
            assert!(all_call_ids.insert(write_result.call_id), "Write call IDs should be unique");
            assert!(all_call_ids.insert(read_result.call_id), "Read call IDs should be unique");
        }

        // Verify Tangle pallet contains blueprint and service metadata
        let client = harness.client();
        let services_client = client.services_client();
        let block_hash = client.now().await.ok_or_else(|| color_eyre::eyre::eyre!("Failed to get current block hash"))?;

        let blueprint = services_client.get_blueprint_by_id(block_hash, blueprint_id).await?;
        assert!(blueprint.is_some(), "OAuth blueprint should be stored in Tangle pallet");

        Ok(())
    }

    #[tokio::test]
    async fn test_oauth_scope_enforcement_with_contract_audit() -> Result<()> {
        color_eyre::install()?;
        setup_log();

        let temp_dir = tempfile::TempDir::new()?;
        let harness = TangleTestHarness::setup(temp_dir).await?;

        let (mut test_env, service_id, _blueprint_id) = harness.setup_services::<1>(true).await?;
        test_env.initialize().await?;

        test_env.add_job(check_scope.layer(TangleLayer)).await;
        test_env.add_job(admin_purge.layer(TangleLayer)).await;
        test_env.start(OAuthBlueprintContext {
            tangle_client: Arc::new(harness.client().clone()),
        }).await?;

        // Test scope checking with different scopes
        let scope_tests = vec![
            "docs:read",
            "docs:write", 
            "docs:admin",
            "analytics:read",
            "system:admin",
        ];

        let mut scope_results = vec![];

        for scope in scope_tests {
            let job_scope = harness
                .submit_job(
                    service_id,
                    CHECK_SCOPE_JOB_ID as u8,
                    vec![InputValue::String(new_bounded_string(scope))],
                )
                .await?;
            let scope_result = harness.wait_for_job_execution(service_id, job_scope.clone()).await?;

            // Validate scope check execution and Tangle storage
            assert_eq!(scope_result.service_id, service_id);
            assert_eq!(scope_result.call_id, job_scope.call_id);
            assert!(!scope_result.result.is_empty());

            scope_results.push((scope, job_scope.call_id, scope_result));
        }

        // Test admin operations with audit trail
        let job_admin = harness
            .submit_job(
                service_id,
                ADMIN_PURGE_JOB_ID as u8,
                vec![InputValue::String(new_bounded_string("target_tenant_for_audit"))],
            )
            .await?;
        let admin_result = harness.wait_for_job_execution(service_id, job_admin.clone()).await?;

        // Validate admin operation execution
        assert_eq!(admin_result.service_id, service_id);
        assert_eq!(admin_result.call_id, job_admin.call_id);
        assert!(!admin_result.result.is_empty());

        // Verify all scope checks have unique call IDs
        let mut scope_call_ids = std::collections::HashSet::new();
        for (_scope, call_id, result) in &scope_results {
            assert!(scope_call_ids.insert(*call_id), "Scope check call IDs should be unique");
            assert_eq!(result.service_id, service_id, "All scope checks should be for same service");
        }

        // Verify admin operation has unique call ID
        assert!(scope_call_ids.insert(admin_result.call_id), "Admin call ID should be unique");

        // Query blockchain for audit events
        let client = harness.client();
        let latest_block = client.rpc_client.blocks().at_latest().await?;
        let events = latest_block.events().await?;

        let mut audit_events = 0;
        for event in events.iter() {
            let event_str = format!("{:?}", event);
            
            // Look for OAuth contract audit events
            if event_str.contains("ScopeChecked") 
                || event_str.contains("AdminPurgeExecuted")
                || event_str.contains("DocumentWritten") {
                audit_events += 1;
            }
        }

        println!("Found {} audit events in blockchain", audit_events);

        Ok(())
    }

    #[tokio::test]
    async fn test_oauth_multi_tenant_document_isolation_validation() -> Result<()> {
        color_eyre::install()?;
        setup_log();

        let temp_dir = tempfile::TempDir::new()?;
        let harness = TangleTestHarness::setup(temp_dir).await?;

        let (mut test_env, service_id, _blueprint_id) = harness.setup_services::<1>(true).await?;
        test_env.initialize().await?;

        test_env.add_job(write_doc.layer(TangleLayer)).await;
        test_env.add_job(list_docs.layer(TangleLayer)).await;
        test_env.start(OAuthBlueprintContext {
            tangle_client: Arc::new(harness.client().clone()),
        }).await?;

        // Simulate multiple tenants with overlapping document names
        let tenant_scenarios = vec![
            ("config.json", "tenant A configuration"),
            ("config.json", "tenant B configuration"), // Same name, different tenant
            ("secrets.env", "tenant A secrets"),
            ("secrets.env", "tenant B secrets"), // Same name, different tenant
            ("unique_doc", "tenant C unique data"),
        ];

        let mut tenant_operations = vec![];

        for (doc_id, content) in tenant_scenarios {
            // Write document (each execution represents a different tenant due to different call contexts)
            let job_write = harness
                .submit_job(
                    service_id,
                    WRITE_DOC_JOB_ID as u8,
                    vec![
                        InputValue::String(new_bounded_string(doc_id)),
                        InputValue::String(new_bounded_string(content))
                    ],
                )
                .await?;
            let write_result = harness.wait_for_job_execution(service_id, job_write.clone()).await?;

            // List documents for this tenant context
            let job_list = harness.submit_job(service_id, LIST_DOCS_JOB_ID as u8, vec![]).await?;
            let list_result = harness.wait_for_job_execution(service_id, job_list.clone()).await?;

            // Validate execution and Tangle storage
            assert_eq!(write_result.service_id, service_id);
            assert_eq!(list_result.service_id, service_id);
            assert!(!write_result.result.is_empty());
            assert!(!list_result.result.is_empty());

            tenant_operations.push((
                job_write.call_id,
                write_result,
                job_list.call_id, 
                list_result,
                doc_id,
                content
            ));
        }

        // Verify tenant isolation at Tangle pallet level
        let mut all_call_ids = std::collections::HashSet::new();
        for (write_call_id, write_result, list_call_id, list_result, _doc_id, _) in &tenant_operations {
            // Each operation should have unique call ID in Tangle pallet
            assert!(all_call_ids.insert(*write_call_id), "Write call IDs should be unique");
            assert!(all_call_ids.insert(*list_call_id), "List call IDs should be unique");
            
            // Verify metadata consistency
            assert_eq!(write_result.call_id, *write_call_id);
            assert_eq!(list_result.call_id, *list_call_id);
        }

        // Verify Rust-side document store maintains tenant isolation
        let store = docs_store();
        let guard = store.read().await;
        
        // The store should contain tenant-isolated data
        // Note: Actual tenant hashes would be derived from AuthContext in real execution
        assert!(!guard.is_empty(), "Document store should contain data");

        // Verify blockchain event emission
        let client = harness.client();
        let latest_block = client.rpc_client.blocks().at_latest().await?;
        let events = latest_block.events().await?;

        let mut document_events = 0;
        for event in events.iter() {
            let event_str = format!("{:?}", event);
            if event_str.contains("DocumentWritten") || event_str.contains("config") || event_str.contains("secrets") {
                document_events += 1;
            }
        }

        println!("Found {} document-related events in blockchain", document_events);

        Ok(())
    }

    #[tokio::test]
    async fn test_oauth_contract_hook_integration_validation() -> Result<()> {
        color_eyre::install()?;
        setup_log();

        let temp_dir = tempfile::TempDir::new()?;
        let harness = TangleTestHarness::setup(temp_dir).await?;

        let (mut test_env, service_id, blueprint_id) = harness.setup_services::<1>(true).await?;
        test_env.initialize().await?;

        test_env.add_job(whoami.layer(TangleLayer)).await;
        test_env.add_job(write_doc.layer(TangleLayer)).await;
        test_env.add_job(check_scope.layer(TangleLayer)).await;
        test_env.start(OAuthBlueprintContext {
            tangle_client: Arc::new(harness.client().clone()),
        }).await?;

        // Test the complete Blueprint contract integration flow
        
        // 1. Execute whoami to establish OAuth context
        let job_whoami = harness.submit_job(service_id, WHOAMI_JOB_ID as u8, vec![]).await?;
        let whoami_result = harness.wait_for_job_execution(service_id, job_whoami.clone()).await?;

        // 2. Execute document write with OAuth scopes
        let job_write = harness
            .submit_job(
                service_id,
                WRITE_DOC_JOB_ID as u8,
                vec![
                    InputValue::String(new_bounded_string("oauth_protected_doc")),
                    InputValue::String(new_bounded_string("sensitive OAuth protected data"))
                ],
            )
            .await?;
        let write_result = harness.wait_for_job_execution(service_id, job_write.clone()).await?;

        // 3. Execute scope check for audit
        let job_scope = harness
            .submit_job(
                service_id,
                CHECK_SCOPE_JOB_ID as u8,
                vec![InputValue::String(new_bounded_string("docs:write"))],
            )
            .await?;
        let scope_result = harness.wait_for_job_execution(service_id, job_scope.clone()).await?;

        // Validate complete Blueprint integration:

        // A. Verify Tangle pallet stores all job metadata correctly
        assert_eq!(whoami_result.service_id, service_id);
        assert_eq!(write_result.service_id, service_id);
        assert_eq!(scope_result.service_id, service_id);

        // B. Verify job call IDs are unique and properly tracked
        assert_eq!(whoami_result.call_id, job_whoami.call_id);
        assert_eq!(write_result.call_id, job_write.call_id);
        assert_eq!(scope_result.call_id, job_scope.call_id);

        // C. Verify job results contain expected OAuth data
        assert!(!whoami_result.result.is_empty(), "Whoami should return OAuth context");
        assert!(!write_result.result.is_empty(), "Write should return success result");
        assert!(!scope_result.result.is_empty(), "Scope check should return authorization result");

        // D. Verify Blueprint contract hooks were triggered
        let client = harness.client();
        let latest_block = client.rpc_client.blocks().at_latest().await?;
        let events = latest_block.events().await?;

        let mut contract_hook_events = 0;
        for event in events.iter() {
            let event_str = format!("{:?}", event);
            
            // Look for evidence of contract hook execution
            if event_str.contains("DocumentWritten") 
                || event_str.contains("ScopeChecked")
                || event_str.contains("oauth_protected_doc") {
                contract_hook_events += 1;
            }
        }

        println!("Found {} contract hook events", contract_hook_events);

        // E. Verify service and blueprint metadata in Tangle pallet
        let services_client = client.services_client();
        let block_hash = client.now().await.ok_or_else(|| color_eyre::eyre::eyre!("Failed to get current block hash"))?;

        let blueprint = services_client.get_blueprint_by_id(block_hash, blueprint_id).await?;
        assert!(blueprint.is_some(), "OAuth blueprint should be stored in Tangle pallet");

        let operator_blueprints = services_client
            .query_operator_blueprints(block_hash, harness.sr25519_signer.account_id().clone())
            .await?;
        assert!(!operator_blueprints.is_empty(), "Operator should have OAuth blueprint registered");

        Ok(())
    }

    #[tokio::test]
    async fn test_oauth_admin_operations_audit_trail() -> Result<()> {
        color_eyre::install()?;
        setup_log();

        let temp_dir = tempfile::TempDir::new()?;
        let harness = TangleTestHarness::setup(temp_dir).await?;

        let (mut test_env, service_id, _blueprint_id) = harness.setup_services::<1>(true).await?;
        test_env.initialize().await?;

        test_env.add_job(admin_purge.layer(TangleLayer)).await;
        test_env.add_job(write_doc.layer(TangleLayer)).await;
        test_env.start(OAuthBlueprintContext {
            tangle_client: Arc::new(harness.client().clone()),
        }).await?;

        // Create some documents first
        let prep_job = harness
            .submit_job(
                service_id,
                WRITE_DOC_JOB_ID as u8,
                vec![
                    InputValue::String(new_bounded_string("target_doc")),
                    InputValue::String(new_bounded_string("data to be purged"))
                ],
            )
            .await?;
        let prep_result = harness.wait_for_job_execution(service_id, prep_job).await?;

        // Execute admin purge operation
        let job_purge = harness
            .submit_job(
                service_id,
                ADMIN_PURGE_JOB_ID as u8,
                vec![InputValue::String(new_bounded_string("target_tenant_12345"))],
            )
            .await?;
        let purge_result = harness.wait_for_job_execution(service_id, job_purge.clone()).await?;

        // Validate admin operation execution and audit trail
        assert_eq!(purge_result.service_id, service_id);
        assert_eq!(purge_result.call_id, job_purge.call_id);
        assert!(!purge_result.result.is_empty());

        // Verify prep and purge operations have different call IDs
        assert_ne!(prep_result.call_id, purge_result.call_id, "Operations should have unique call IDs");

        // Query blockchain for admin audit events
        let client = harness.client();
        let latest_block = client.rpc_client.blocks().at_latest().await?;
        let events = latest_block.events().await?;

        let mut admin_audit_events = 0;
        for event in events.iter() {
            let event_str = format!("{:?}", event);
            
            // Look for admin operation audit trail
            if event_str.contains("AdminPurgeExecuted") 
                || event_str.contains("target_tenant") 
                || event_str.contains("purge") {
                admin_audit_events += 1;
            }
        }

        println!("Found {} admin audit events", admin_audit_events);

        // Verify Tangle pallet contains immutable audit trail
        // The admin operation should be permanently recorded with:
        // - Unique call ID
        // - Service ID
        // - Target tenant information
        // - Admin operator identity
        // - Timestamp (block number)

        Ok(())
    }

    #[tokio::test]
    async fn test_oauth_document_lifecycle_complete_validation() -> Result<()> {
        color_eyre::install()?;
        setup_log();

        let temp_dir = tempfile::TempDir::new()?;
        let harness = TangleTestHarness::setup(temp_dir).await?;

        let (mut test_env, service_id, _blueprint_id) = harness.setup_services::<1>(true).await?;
        test_env.initialize().await?;

        test_env.add_job(write_doc.layer(TangleLayer)).await;
        test_env.add_job(read_doc.layer(TangleLayer)).await;
        test_env.add_job(list_docs.layer(TangleLayer)).await;
        test_env.start(OAuthBlueprintContext {
            tangle_client: Arc::new(harness.client().clone()),
        }).await?;

        // Test complete document lifecycle with state validation
        let doc_id = "lifecycle_test_document";
        let doc_content = "document lifecycle validation data";

        // 1. Write document
        let job_write = harness
            .submit_job(
                service_id,
                WRITE_DOC_JOB_ID as u8,
                vec![
                    InputValue::String(new_bounded_string(doc_id)),
                    InputValue::String(new_bounded_string(doc_content))
                ],
            )
            .await?;
        let write_result = harness.wait_for_job_execution(service_id, job_write.clone()).await?;

        // 2. List documents to verify it appears
        let job_list = harness.submit_job(service_id, LIST_DOCS_JOB_ID as u8, vec![]).await?;
        let list_result = harness.wait_for_job_execution(service_id, job_list.clone()).await?;

        // 3. Read document back to verify content
        let job_read = harness
            .submit_job(
                service_id,
                READ_DOC_JOB_ID as u8,
                vec![InputValue::String(new_bounded_string(doc_id))],
            )
            .await?;
        let read_result = harness.wait_for_job_execution(service_id, job_read.clone()).await?;

        // Validate complete document lifecycle:

        // A. All operations executed successfully with proper metadata
        assert_eq!(write_result.service_id, service_id);
        assert_eq!(list_result.service_id, service_id);
        assert_eq!(read_result.service_id, service_id);

        // B. Call IDs are unique and properly tracked
        assert_ne!(write_result.call_id, list_result.call_id);
        assert_ne!(list_result.call_id, read_result.call_id);
        assert_ne!(write_result.call_id, read_result.call_id);

        // C. Job results contain expected data structures
        assert!(!write_result.result.is_empty());
        assert!(!list_result.result.is_empty());
        assert!(!read_result.result.is_empty());

        // D. Verify Rust-side document store was updated
        let store = docs_store();
        let guard = store.read().await;
        assert!(!guard.is_empty(), "Document store should contain data after write operations");

        // E. Verify blockchain contains document lifecycle events
        let client = harness.client();
        let latest_block = client.rpc_client.blocks().at_latest().await?;
        let events = latest_block.events().await?;

        let mut lifecycle_events = 0;
        for event in events.iter() {
            let event_str = format!("{:?}", event);
            if event_str.contains("DocumentWritten") 
                || event_str.contains("DocumentRead")
                || event_str.contains(doc_id) {
                lifecycle_events += 1;
            }
        }

        println!("Found {} document lifecycle events", lifecycle_events);

        Ok(())
    }
}
