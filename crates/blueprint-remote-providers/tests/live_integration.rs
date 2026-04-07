//! Live integration tests for GPU provider adapters.
//!
//! These tests call REAL provider APIs and cost real money.
//! Skip by default — run with credentials set:
//!
//!   LAMBDA_LABS_API_KEY=... cargo test --test live_integration -- --ignored
//!   RUNPOD_API_KEY=... cargo test --test live_integration -- --ignored
//!   TENSORDOCK_API_KEY=... TENSORDOCK_API_TOKEN=... cargo test --test live_integration -- --ignored
//!
//! Each test provisions the cheapest instance, verifies it starts,
//! then immediately terminates. Cost: <$0.02 per test run.

use blueprint_remote_providers::infra::traits::CloudProviderAdapter;
use blueprint_remote_providers::infra::types::InstanceStatus;

fn skip_unless_env(var: &str) -> bool {
    std::env::var(var).is_err()
}

// ---------------------------------------------------------------------------
// Lambda Labs — gpu_1x_a10 ~$0.75/hr
// ---------------------------------------------------------------------------

#[tokio::test]
#[ignore]
async fn lambda_labs_provision_and_terminate() {
    if skip_unless_env("LAMBDA_LABS_API_KEY") {
        eprintln!("Skipping: LAMBDA_LABS_API_KEY not set");
        return;
    }

    let adapter = blueprint_remote_providers::providers::lambda_labs::LambdaLabsAdapter::new()
        .await
        .expect("adapter creation failed");

    // Provision cheapest instance
    let instance = adapter
        .provision_instance("gpu_1x_a10", "us-west-1", false)
        .await
        .expect("provision failed");

    assert!(!instance.id.is_empty(), "instance ID should be non-empty");
    assert_eq!(instance.status, InstanceStatus::Running);
    assert!(instance.public_ip.is_some(), "should have a public IP");

    eprintln!("Provisioned: {} at {:?}", instance.id, instance.public_ip);

    // Verify status polling works
    let status = adapter
        .get_instance_status(&instance.id)
        .await
        .expect("get_instance_status failed");
    assert_eq!(status, InstanceStatus::Running);

    // Terminate immediately
    adapter
        .terminate_instance(&instance.id)
        .await
        .expect("terminate failed");

    eprintln!("Terminated: {}", instance.id);
}

// ---------------------------------------------------------------------------
// RunPod — cheapest community GPU pod ~$0.20/hr
// ---------------------------------------------------------------------------

#[tokio::test]
#[ignore]
async fn runpod_provision_and_terminate() {
    if skip_unless_env("RUNPOD_API_KEY") {
        eprintln!("Skipping: RUNPOD_API_KEY not set");
        return;
    }

    // Use community cloud for cheapest pricing.
    // SAFETY: test runs single-threaded; no other thread reads this var concurrently.
    unsafe { std::env::set_var("RUNPOD_CLOUD_TYPE", "COMMUNITY") };

    let adapter = blueprint_remote_providers::providers::runpod::RunPodAdapter::new()
        .await
        .expect("adapter creation failed");

    let instance = adapter
        .provision_instance("NVIDIA GeForce RTX 4090", "", false)
        .await
        .expect("provision failed");

    assert!(!instance.id.is_empty(), "instance ID should be non-empty");
    assert_eq!(instance.status, InstanceStatus::Running);

    eprintln!("Provisioned: {} at {:?}", instance.id, instance.public_ip);

    let status = adapter
        .get_instance_status(&instance.id)
        .await
        .expect("get_instance_status failed");
    assert_eq!(status, InstanceStatus::Running);

    adapter
        .terminate_instance(&instance.id)
        .await
        .expect("terminate failed");

    eprintln!("Terminated: {}", instance.id);
}

// ---------------------------------------------------------------------------
// TensorDock — cheapest GPU ~$0.10/hr
// ---------------------------------------------------------------------------

#[tokio::test]
#[ignore]
async fn tensordock_provision_and_terminate() {
    if skip_unless_env("TENSORDOCK_API_KEY") || skip_unless_env("TENSORDOCK_API_TOKEN") {
        eprintln!("Skipping: TENSORDOCK_API_KEY or TENSORDOCK_API_TOKEN not set");
        return;
    }

    let adapter = blueprint_remote_providers::providers::tensordock::TensorDockAdapter::new()
        .await
        .expect("adapter creation failed");

    let instance = adapter
        .provision_instance("rtx4090-pcie-1", "", false)
        .await
        .expect("provision failed");

    assert!(!instance.id.is_empty(), "instance ID should be non-empty");
    assert_eq!(instance.status, InstanceStatus::Running);

    eprintln!("Provisioned: {} at {:?}", instance.id, instance.public_ip);

    let status = adapter
        .get_instance_status(&instance.id)
        .await
        .expect("get_instance_status failed");
    assert_eq!(status, InstanceStatus::Running);

    adapter
        .terminate_instance(&instance.id)
        .await
        .expect("terminate failed");

    eprintln!("Terminated: {}", instance.id);
}
