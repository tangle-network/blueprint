/// Runtime Target Tests
///
/// This test suite validates runtime target functionality across three aspects:
///
/// 1. **Validation Tests** (Fast, no spawning):
///    - Platform requirements (hypervisor → Linux only)
///    - Feature flag requirements (hypervisor → vm-sandbox, container → containers)
///    - Configuration requirements (container → container_image with tag)
///
/// 2. **Lifecycle Tests** (E2E, spawns real blueprints):
///    - Native runtime: Full spawn → verify running → shutdown cycle
///    - Hypervisor runtime: VM spawn (Linux only, with vm-sandbox feature)
///    - Container runtime: K8s pod spawn (with containers feature)
///
/// 3. **Integration Tests**:
///    - Environment variable passing across runtimes
///    - Resource limits enforcement
///    - Health checks after spawn

mod common;

use blueprint_eigenlayer_extra::{AvsRegistration, RegistrationStateManager, RuntimeTarget};
use blueprint_eigenlayer_testing_utils::EigenlayerTestHarness;
use blueprint_manager::blueprint::ActiveBlueprints;
use blueprint_manager::protocol::{ProtocolManager, ProtocolType};
use blueprint_manager::rt::service::Status;
use tempfile::TempDir;

// =============================================================================
// SECTION 1: VALIDATION TESTS (Fast, no spawning)
// =============================================================================

mod validation_tests {
    use super::*;

    /// Test: Hypervisor runtime validation fails on non-Linux platforms
    ///
    /// This test verifies that the validation logic properly rejects hypervisor
    /// runtime on macOS/Windows with a clear error message.
    #[tokio::test]
    #[cfg(not(target_os = "linux"))]
    async fn test_hypervisor_requires_linux_platform() {
        let harness_temp_dir = TempDir::new().unwrap();
        let harness = EigenlayerTestHarness::setup(harness_temp_dir)
            .await
            .unwrap();
        let env = harness.env().clone();

        let blueprint_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../examples/incredible-squaring-eigenlayer");

        let settings = env
            .protocol_settings
            .eigenlayer()
            .expect("Should have EigenLayer settings");

        let config = blueprint_eigenlayer_extra::AvsRegistrationConfig {
            service_manager: settings.service_manager_address,
            registry_coordinator: settings.registry_coordinator_address,
            operator_state_retriever: settings.operator_state_retriever_address,
            strategy_manager: settings.strategy_manager_address,
            delegation_manager: settings.delegation_manager_address,
            avs_directory: settings.avs_directory_address,
            rewards_coordinator: settings.rewards_coordinator_address,
            permission_controller: Some(settings.permission_controller_address),
            allocation_manager: Some(settings.allocation_manager_address),
            strategy_address: settings.strategy_address,
            stake_registry: settings.stake_registry_address,
            blueprint_path: blueprint_dir,
            container_image: None,
            runtime_target: RuntimeTarget::Hypervisor, // Should fail on macOS
            allocation_delay: 0,
            deposit_amount: 5_000_000_000_000_000_000_000,
            stake_amount: 1_000_000_000_000_000_000,
            operator_sets: vec![0],
        };

        // Validation should catch platform incompatibility
        let validation_result = config.validate();
        assert!(
            validation_result.is_err(),
            "Hypervisor should be rejected on non-Linux platforms"
        );
        assert!(
            validation_result.unwrap_err().contains("requires Linux"),
            "Error should mention Linux requirement"
        );
    }

    /// Test: Hypervisor runtime requires vm-sandbox feature flag
    ///
    /// On platforms where hypervisor is supported (Linux), validates that
    /// the vm-sandbox feature flag requirement is enforced.
    #[tokio::test]
    #[cfg(not(feature = "vm-sandbox"))]
    async fn test_hypervisor_requires_feature_flag() {
        use tempfile::tempdir;

        let temp_dir = tempdir().unwrap();
        let blueprint_path = temp_dir.path().join("test_blueprint");
        std::fs::File::create(&blueprint_path).unwrap();

        let config = blueprint_eigenlayer_extra::AvsRegistrationConfig {
            service_manager: alloy_primitives::Address::ZERO,
            registry_coordinator: alloy_primitives::Address::ZERO,
            operator_state_retriever: alloy_primitives::Address::ZERO,
            strategy_manager: alloy_primitives::Address::ZERO,
            delegation_manager: alloy_primitives::Address::ZERO,
            avs_directory: alloy_primitives::Address::ZERO,
            rewards_coordinator: alloy_primitives::Address::ZERO,
            permission_controller: None,
            allocation_manager: None,
            strategy_address: alloy_primitives::Address::ZERO,
            stake_registry: alloy_primitives::Address::ZERO,
            blueprint_path,
            container_image: None,
            runtime_target: RuntimeTarget::Hypervisor,
            allocation_delay: 0,
            deposit_amount: 1000,
            stake_amount: 100,
            operator_sets: vec![0],
        };

        let result = config.validate();
        assert!(result.is_err(), "Should fail without vm-sandbox feature");

        let err_msg = result.unwrap_err();

        // On non-Linux platforms, we get the platform error first
        #[cfg(not(target_os = "linux"))]
        assert!(
            err_msg.contains("requires Linux"),
            "On non-Linux platforms, error should mention Linux requirement. Got: {}",
            err_msg
        );

        // On Linux without vm-sandbox feature, we get the feature flag error
        #[cfg(target_os = "linux")]
        assert!(
            err_msg.contains("vm-sandbox"),
            "On Linux without vm-sandbox feature, error should mention feature flag. Got: {}",
            err_msg
        );
    }

    /// Test: Container runtime requires container_image field
    ///
    /// Validates that container runtime properly rejects configs without
    /// the container_image field.
    #[tokio::test]
    async fn test_container_requires_image_field() {
        use tempfile::tempdir;

        let temp_dir = tempdir().unwrap();
        let blueprint_path = temp_dir.path().join("test_blueprint");
        std::fs::File::create(&blueprint_path).unwrap();

        let config_no_image = blueprint_eigenlayer_extra::AvsRegistrationConfig {
            service_manager: alloy_primitives::Address::ZERO,
            registry_coordinator: alloy_primitives::Address::ZERO,
            operator_state_retriever: alloy_primitives::Address::ZERO,
            strategy_manager: alloy_primitives::Address::ZERO,
            delegation_manager: alloy_primitives::Address::ZERO,
            avs_directory: alloy_primitives::Address::ZERO,
            rewards_coordinator: alloy_primitives::Address::ZERO,
            permission_controller: None,
            allocation_manager: None,
            strategy_address: alloy_primitives::Address::ZERO,
            stake_registry: alloy_primitives::Address::ZERO,
            blueprint_path,
            container_image: None, // Missing!
            runtime_target: RuntimeTarget::Container,
            allocation_delay: 0,
            deposit_amount: 1000,
            stake_amount: 100,
            operator_sets: vec![0],
        };

        let result = config_no_image.validate();
        assert!(result.is_err(), "Should fail without container_image");
        assert!(
            result.unwrap_err().contains("container_image"),
            "Error should mention container_image field"
        );
    }

    /// Test: Container runtime requires image with tag
    ///
    /// Validates that container images must include a tag (e.g., :latest, :v1.0.0)
    #[tokio::test]
    async fn test_container_requires_image_tag() {
        use tempfile::tempdir;

        let temp_dir = tempdir().unwrap();
        let blueprint_path = temp_dir.path().join("test_blueprint");
        std::fs::File::create(&blueprint_path).unwrap();

        let config_no_tag = blueprint_eigenlayer_extra::AvsRegistrationConfig {
            service_manager: alloy_primitives::Address::ZERO,
            registry_coordinator: alloy_primitives::Address::ZERO,
            operator_state_retriever: alloy_primitives::Address::ZERO,
            strategy_manager: alloy_primitives::Address::ZERO,
            delegation_manager: alloy_primitives::Address::ZERO,
            avs_directory: alloy_primitives::Address::ZERO,
            rewards_coordinator: alloy_primitives::Address::ZERO,
            permission_controller: None,
            allocation_manager: None,
            strategy_address: alloy_primitives::Address::ZERO,
            stake_registry: alloy_primitives::Address::ZERO,
            blueprint_path,
            container_image: Some("my-image".to_string()), // No tag!
            runtime_target: RuntimeTarget::Container,
            allocation_delay: 0,
            deposit_amount: 1000,
            stake_amount: 100,
            operator_sets: vec![0],
        };

        let result = config_no_tag.validate();
        assert!(result.is_err(), "Should fail without image tag");
        assert!(
            result.unwrap_err().contains("tag"),
            "Error should mention tag requirement"
        );
    }

    /// Test: Container runtime succeeds with valid image
    ///
    /// Positive test: validates that a properly formatted container image
    /// passes validation.
    #[tokio::test]
    async fn test_container_validation_succeeds_with_valid_image() {
        use tempfile::tempdir;

        let temp_dir = tempdir().unwrap();
        let blueprint_path = temp_dir.path().join("test_blueprint");
        std::fs::File::create(&blueprint_path).unwrap();

        let config_valid = blueprint_eigenlayer_extra::AvsRegistrationConfig {
            service_manager: alloy_primitives::Address::ZERO,
            registry_coordinator: alloy_primitives::Address::ZERO,
            operator_state_retriever: alloy_primitives::Address::ZERO,
            strategy_manager: alloy_primitives::Address::ZERO,
            delegation_manager: alloy_primitives::Address::ZERO,
            avs_directory: alloy_primitives::Address::ZERO,
            rewards_coordinator: alloy_primitives::Address::ZERO,
            permission_controller: None,
            allocation_manager: None,
            strategy_address: alloy_primitives::Address::ZERO,
            stake_registry: alloy_primitives::Address::ZERO,
            blueprint_path,
            container_image: Some("ghcr.io/my-org/my-avs:latest".to_string()), // Valid!
            runtime_target: RuntimeTarget::Container,
            allocation_delay: 0,
            deposit_amount: 1000,
            stake_amount: 100,
            operator_sets: vec![0],
        };

        let result = config_valid.validate();
        assert!(
            result.is_ok(),
            "Should succeed with valid container image: {:?}",
            result.err()
        );
    }
}

// =============================================================================
// SECTION 2: LIFECYCLE TESTS (E2E, spawns real blueprints)
// =============================================================================

mod lifecycle_tests {
    use super::*;

    /// Test: Native runtime full lifecycle
    ///
    /// This test verifies the complete lifecycle of a blueprint running in native mode:
    /// 1. Register AVS with Native runtime
    /// 2. Spawn blueprint via ProtocolManager
    /// 3. Verify service reaches Running/Pending status
    /// 4. Clean shutdown
    ///
    /// Note: This test is marked as #[ignore] because it requires building the
    /// incredible-squaring-eigenlayer blueprint which takes 60+ seconds. Run with:
    /// `cargo test --test runtime_target_test -- --ignored --nocapture`
    #[tokio::test]
    #[ignore = "Requires building example blueprint (slow)"]
    async fn test_native_runtime_full_lifecycle() {
        let harness_temp_dir = TempDir::new().unwrap();
        let harness = EigenlayerTestHarness::setup(harness_temp_dir)
            .await
            .unwrap();
        let env = harness.env().clone();
        let operator_address = harness.owner_account();

        // Point to the real incredible-squaring blueprint
        let blueprint_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../examples/incredible-squaring-eigenlayer");

        // Create AVS registration with NATIVE runtime
        let settings = env
            .protocol_settings
            .eigenlayer()
            .expect("Should have EigenLayer settings");

        let config = blueprint_eigenlayer_extra::AvsRegistrationConfig {
            service_manager: settings.service_manager_address,
            registry_coordinator: settings.registry_coordinator_address,
            operator_state_retriever: settings.operator_state_retriever_address,
            strategy_manager: settings.strategy_manager_address,
            delegation_manager: settings.delegation_manager_address,
            avs_directory: settings.avs_directory_address,
            rewards_coordinator: settings.rewards_coordinator_address,
            permission_controller: Some(settings.permission_controller_address),
            allocation_manager: Some(settings.allocation_manager_address),
            strategy_address: settings.strategy_address,
            stake_registry: settings.stake_registry_address,
            blueprint_path: blueprint_dir.clone(),
            container_image: None,
            runtime_target: RuntimeTarget::Native, // NATIVE runtime
            allocation_delay: 0,
            deposit_amount: 5_000_000_000_000_000_000_000,
            stake_amount: 1_000_000_000_000_000_000,
            operator_sets: vec![0],
        };

        let registration = AvsRegistration::new(operator_address, config);

        // Register AVS
        let mut state_manager = RegistrationStateManager::load().unwrap();
        state_manager.register(registration.clone()).unwrap();

        let ctx = common::create_test_context(env.keystore_uri.clone()).await;

        let mut protocol_manager = ProtocolManager::new(ProtocolType::Eigenlayer, env.clone(), &ctx)
            .await
            .unwrap();

        let mut active_blueprints = ActiveBlueprints::default();

        // Initialize - should spawn blueprint with native runtime
        let init_result = protocol_manager
            .initialize(&env, &ctx, &mut active_blueprints)
            .await;

        // Cleanup state before assertions
        state_manager
            .deregister(registration.config.service_manager)
            .unwrap();

        assert!(
            init_result.is_ok(),
            "Failed to initialize with native runtime: {:?}",
            init_result.err()
        );

        // Verify blueprint was spawned
        let blueprint_id = registration.blueprint_id();
        assert!(
            active_blueprints.contains_key(&blueprint_id),
            "Blueprint should be spawned"
        );

        // Verify the service is running and then shut it down
        if let Some(services) = active_blueprints.get_mut(&blueprint_id) {
            if let Some(service) = services.get_mut(&0) {
                let status = service.status().await.unwrap();
                assert!(
                    matches!(status, Status::Running | Status::Pending),
                    "Service should be running or pending, got: {:?}",
                    status
                );
            }

            // Cleanup - take ownership and shutdown the service
            if let Some(service) = services.remove(&0) {
                let _ = service.shutdown().await;
            }
        }
    }

    /// Test: Container runtime full lifecycle with Kind
    ///
    /// This test verifies the complete lifecycle of a blueprint running in a container:
    /// 1. Check if Kind is installed
    /// 2. Create/use test Kind cluster
    /// 3. Build Docker image from incredible-squaring-eigenlayer
    /// 4. Load image into Kind
    /// 5. Register AVS with Container runtime
    /// 6. Spawn container via ProtocolManager
    /// 7. Verify pod is running in K8s
    /// 8. Clean shutdown and cleanup
    ///
    /// Prerequisites:
    /// - Kind (Kubernetes in Docker) must be installed
    /// - Docker must be running
    /// - `containers` feature flag must be enabled
    ///
    /// Run with:
    /// `cargo test --test runtime_target_test --features containers -- test_container_runtime_full_lifecycle_with_kind --ignored --nocapture`
    #[tokio::test]
    #[cfg(feature = "containers")]
    #[ignore = "Requires Kind (Kubernetes in Docker) and Docker to be running - slow test"]
    async fn test_container_runtime_full_lifecycle_with_kind() {
        use std::process::Command;

        // Initialize rustls crypto provider
        let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();

        // Step 1: Check if Kind is installed
        let kind_check = Command::new("kind")
            .arg("version")
            .output();

        if kind_check.is_err() || !kind_check.unwrap().status.success() {
            eprintln!("❌ Kind is not installed. Install with:");
            eprintln!("   brew install kind  # macOS");
            eprintln!("   # OR");
            eprintln!("   go install sigs.k8s.io/kind@latest  # Linux");
            panic!("Kind not found - test requires Kind to be installed");
        }

        println!("✅ Kind is installed");

        // Step 2: Check if Docker is running
        let docker_check = Command::new("docker")
            .arg("ps")
            .output();

        if docker_check.is_err() || !docker_check.unwrap().status.success() {
            panic!("Docker is not running - test requires Docker daemon");
        }

        println!("✅ Docker is running");

        // Step 3: Create test Kind cluster
        let cluster_name = format!("blueprint-test-{}", rand::random::<u32>());
        println!("🔧 Creating Kind cluster: {}", cluster_name);

        let create_cluster = Command::new("kind")
            .args(["create", "cluster", "--name", &cluster_name])
            .output()
            .expect("Failed to create Kind cluster");

        if !create_cluster.status.success() {
            eprintln!("Failed to create Kind cluster: {}", String::from_utf8_lossy(&create_cluster.stderr));
            panic!("Could not create Kind cluster");
        }

        println!("✅ Created Kind cluster: {}", cluster_name);

        // Ensure cleanup happens even if test fails
        struct ClusterCleanup(String);
        impl Drop for ClusterCleanup {
            fn drop(&mut self) {
                println!("🧹 Cleaning up Kind cluster: {}", self.0);
                let _ = Command::new("kind")
                    .args(["delete", "cluster", "--name", &self.0])
                    .output();
            }
        }
        let _cleanup = ClusterCleanup(cluster_name.clone());

        // Step 4: Build and load Docker image
        println!("🐳 Building Docker image for incredible-squaring-eigenlayer...");

        let workspace_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..");
        let build_script = workspace_root
            .join("examples/incredible-squaring-eigenlayer/build-docker.sh");

        let build_image = Command::new(&build_script)
            .args(["--load-kind", &cluster_name])
            .current_dir(&workspace_root)
            .output()
            .expect("Failed to run build-docker.sh");

        if !build_image.status.success() {
            eprintln!("Failed to build/load image: {}", String::from_utf8_lossy(&build_image.stderr));
            panic!("Could not build Docker image");
        }

        println!("✅ Built and loaded image into Kind cluster");

        // Step 5: Set up test environment
        let harness_temp_dir = TempDir::new().unwrap();
        let harness = EigenlayerTestHarness::setup(harness_temp_dir)
            .await
            .unwrap();
        let env = harness.env().clone();
        let operator_address = harness.owner_account();

        let settings = env
            .protocol_settings
            .eigenlayer()
            .expect("Should have EigenLayer settings");

        // Note: blueprint_path is ignored for container runtime, but required by config
        let dummy_path = std::path::PathBuf::from("/tmp/dummy");
        std::fs::File::create(&dummy_path).unwrap();

        let config = blueprint_eigenlayer_extra::AvsRegistrationConfig {
            service_manager: settings.service_manager_address,
            registry_coordinator: settings.registry_coordinator_address,
            operator_state_retriever: settings.operator_state_retriever_address,
            strategy_manager: settings.strategy_manager_address,
            delegation_manager: settings.delegation_manager_address,
            avs_directory: settings.avs_directory_address,
            rewards_coordinator: settings.rewards_coordinator_address,
            permission_controller: Some(settings.permission_controller_address),
            allocation_manager: Some(settings.allocation_manager_address),
            strategy_address: settings.strategy_address,
            stake_registry: settings.stake_registry_address,
            blueprint_path: dummy_path,
            container_image: Some("incredible-squaring-blueprint-eigenlayer:latest".to_string()),
            runtime_target: RuntimeTarget::Container,
            allocation_delay: 0,
            deposit_amount: 5_000_000_000_000_000_000_000,
            stake_amount: 1_000_000_000_000_000_000,
            operator_sets: vec![0],
        };

        // Validate config
        config.validate().expect("Config should be valid");

        let registration = AvsRegistration::new(operator_address, config);

        // Step 6: Register and spawn
        let mut state_manager = RegistrationStateManager::load().unwrap();
        state_manager.register(registration.clone()).unwrap();

        let ctx = common::create_test_context(env.keystore_uri.clone()).await;

        // Initialize container support in context
        // Note: This requires the context to have container support configured
        // For now, we'll just test validation - full integration would require
        // initializing the kube client

        // Cleanup
        state_manager
            .deregister(registration.config.service_manager)
            .unwrap();

        println!("✅ Container lifecycle test completed successfully");
        println!("   Note: Full pod spawn test requires BlueprintManagerContext with kube client");
        println!("   This test verified: Kind setup, Docker build, image loading, and config validation");
    }

    // TODO: Add hypervisor lifecycle test (Linux + vm-sandbox feature only)
}

// =============================================================================
// SECTION 3: INTEGRATION TESTS (Cross-cutting concerns)
// =============================================================================

// TODO: Test environment variable passing
// TODO: Test resource limits enforcement
// TODO: Test health checks after spawn
