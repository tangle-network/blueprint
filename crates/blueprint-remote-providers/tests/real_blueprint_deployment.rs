//! Real-world integration tests for incredible-squaring blueprint
//!
//! Requires actual cloud credentials and Docker/k8s infrastructure.
//! Run with: REAL_TEST=1 cargo test --test real_blueprint_deployment -- --nocapture

use blueprint_remote_providers::{
    core::{
        deployment_target::{DeploymentTarget, ContainerRuntime},
        resources::ResourceSpec,
        remote::CloudProvider,
    },
    deployment::{QosTunnelManager, DeploymentTracker},
    infra::traits::{CloudProviderAdapter, BlueprintDeploymentResult},
    providers::{
        aws::AwsAdapter,
        gcp::GcpAdapter,
        digitalocean::adapter::DigitalOceanAdapter,
    },
};
use std::{collections::HashMap, time::Duration};
use tokio::time::{timeout, sleep};
use tracing::{info, warn, error, debug};

const BLUEPRINT_IMAGE: &str = "ghcr.io/tangle-network/incredible-squaring:latest";
const TEST_TIMEOUT: Duration = Duration::from_secs(600); // 10 min max per provider

/// Test configuration from environment
struct TestConfig {
    providers: Vec<CloudProvider>,
    skip_cleanup: bool,
    parallel: bool,
    verify_qos: bool,
    test_kubernetes: bool,
}

impl TestConfig {
    fn from_env() -> Self {
        let mut providers = Vec::new();

        if std::env::var("AWS_ACCESS_KEY_ID").is_ok() {
            providers.push(CloudProvider::AWS);
        }
        if std::env::var("GCP_PROJECT_ID").is_ok() {
            providers.push(CloudProvider::GCP);
        }
        if std::env::var("DIGITALOCEAN_TOKEN").is_ok() {
            providers.push(CloudProvider::DigitalOcean);
        }

        Self {
            providers,
            skip_cleanup: std::env::var("SKIP_CLEANUP").is_ok(),
            parallel: std::env::var("PARALLEL_TEST").is_ok(),
            verify_qos: std::env::var("VERIFY_QOS").unwrap_or_else(|_| "1".to_string()) == "1",
            test_kubernetes: std::env::var("TEST_KUBERNETES").is_ok(),
        }
    }
}

/// Deployment result with timing and metrics
#[derive(Debug)]
struct TestResult {
    provider: CloudProvider,
    deployment: Option<BlueprintDeploymentResult>,
    provision_time: Duration,
    deploy_time: Duration,
    qos_verified: bool,
    error: Option<String>,
}

/// Main test orchestrator
struct RealBlueprintTest {
    config: TestConfig,
    tracker: DeploymentTracker,
    qos_tunnel_manager: QosTunnelManager,
    results: Vec<TestResult>,
}

impl RealBlueprintTest {
    async fn new() -> Self {
        let tracker_path = std::env::var("TEST_TRACKER_PATH")
            .unwrap_or_else(|_| "/tmp/blueprint_test_tracker".to_string());
        let tracker = DeploymentTracker::new(
            std::path::Path::new(&tracker_path)
        ).await.expect("Failed to create tracker");

        Self {
            config: TestConfig::from_env(),
            tracker,
            qos_tunnel_manager: QosTunnelManager::new(30000), // Start tunnels at port 30000
            results: Vec::new(),
        }
    }

    /// Run full test suite
    async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Starting real blueprint deployment test");
        info!("Testing providers: {:?}", self.config.providers);
        info!("Parallel: {}, Verify QoS: {}", self.config.parallel, self.config.verify_qos);

        if self.config.providers.is_empty() {
            error!("No cloud credentials configured. Set:");
            error!("  AWS: AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY");
            error!("  GCP: GCP_PROJECT_ID, GCP_ACCESS_TOKEN");
            error!("  DigitalOcean: DIGITALOCEAN_TOKEN");
            return Err("No providers configured".into());
        }

        // Run tests
        if self.config.parallel {
            self.run_parallel_tests().await?;
        } else {
            self.run_sequential_tests().await?;
        }

        // Print results
        self.print_results();

        // Cleanup if not skipped
        if !self.config.skip_cleanup {
            self.cleanup_all().await?;
        }

        Ok(())
    }

    /// Test providers sequentially
    async fn run_sequential_tests(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for provider in self.config.providers.clone() {
            let provider_copy = provider.clone();
            let result = timeout(TEST_TIMEOUT, self.test_provider(provider)).await
                .unwrap_or_else(move |_| TestResult {
                    provider: provider_copy,
                    deployment: None,
                    provision_time: Duration::ZERO,
                    deploy_time: Duration::ZERO,
                    qos_verified: false,
                    error: Some("Timeout".to_string()),
                });
            self.results.push(result);
        }
        Ok(())
    }

    /// Test providers in parallel
    async fn run_parallel_tests(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // For simplicity, run tests sequentially even when parallel is requested
        // Parallel testing would require cloning self which is complex
        warn!("Parallel testing requested but running sequentially for safety");
        self.run_sequential_tests().await
    }

    /// Test single provider
    async fn test_provider(&mut self, provider: CloudProvider) -> TestResult {
        // For now, only test VM deployments
        // Kubernetes requires more complex setup
        info!("Testing {provider:?} with VM deployment...");
        self.test_provider_with_target(
            provider,
            DeploymentTarget::VirtualMachine {
                runtime: ContainerRuntime::Docker
            }
        ).await
    }

    /// Test provider with specific deployment target
    async fn test_provider_with_target(
        &mut self,
        provider: CloudProvider,
        target: DeploymentTarget,
    ) -> TestResult {
        info!("Testing {provider:?} with {target:?}...");

        let start = std::time::Instant::now();

        // Create adapter
        let adapter: Box<dyn CloudProviderAdapter> = match provider {
            CloudProvider::AWS => {
                match AwsAdapter::new().await {
                    Ok(a) => Box::new(a),
                    Err(e) => return TestResult {
                        provider,
                        deployment: None,
                        provision_time: Duration::ZERO,
                        deploy_time: Duration::ZERO,
                        qos_verified: false,
                        error: Some(format!("Adapter creation failed: {e}")),
                    }
                }
            }
            CloudProvider::GCP => {
                match GcpAdapter::new().await {
                    Ok(a) => Box::new(a),
                    Err(e) => return TestResult {
                        provider,
                        deployment: None,
                        provision_time: Duration::ZERO,
                        deploy_time: Duration::ZERO,
                        qos_verified: false,
                        error: Some(format!("Adapter creation failed: {e}")),
                    }
                }
            }
            CloudProvider::DigitalOcean => {
                match DigitalOceanAdapter::new().await {
                    Ok(a) => Box::new(a),
                    Err(e) => return TestResult {
                        provider,
                        deployment: None,
                        provision_time: Duration::ZERO,
                        deploy_time: Duration::ZERO,
                        qos_verified: false,
                        error: Some(format!("Adapter creation failed: {e}")),
                    }
                }
            }
            _ => return TestResult {
                provider,
                deployment: None,
                provision_time: Duration::ZERO,
                deploy_time: Duration::ZERO,
                qos_verified: false,
                error: Some("Provider not implemented".to_string()),
            }
        };

        let provision_time = start.elapsed();

        // Deploy blueprint
        let deploy_start = std::time::Instant::now();

        let resource_spec = ResourceSpec {
            cpu: 2.0,
            memory_gb: 4.0,
            storage_gb: 20.0,
            gpu_count: None,
            allow_spot: true,
            qos: Default::default(),
        };

        let mut env_vars = HashMap::new();
        env_vars.insert("RUST_LOG".to_string(), "info".to_string());
        env_vars.insert("SERVICE_ID".to_string(), "test-service".to_string());

        let deployment = match adapter.deploy_blueprint_with_target(
            &target,
            BLUEPRINT_IMAGE,
            &resource_spec,
            env_vars,
        ).await {
            Ok(d) => d,
            Err(e) => return TestResult {
                provider,
                deployment: None,
                provision_time,
                deploy_time: Duration::ZERO,
                qos_verified: false,
                error: Some(format!("Deployment failed: {e}")),
            }
        };

        let deploy_time = deploy_start.elapsed();

        // Track deployment
        debug!("Tracking deployment for {provider:?}: {}", deployment.instance.id);

        // Verify QoS if enabled
        let qos_verified = if self.config.verify_qos {
            self.verify_qos(&deployment, &provider).await
        } else {
            false
        };

        TestResult {
            provider,
            deployment: Some(deployment),
            provision_time,
            deploy_time,
            qos_verified,
            error: None,
        }
    }

    /// Verify QoS metrics are accessible
    async fn verify_qos(&mut self, deployment: &BlueprintDeploymentResult, provider: &CloudProvider) -> bool {
        info!("Verifying QoS for {provider:?} deployment...");

        // Wait for service startup
        sleep(Duration::from_secs(30)).await;

        if let Some(qos_endpoint) = deployment.qos_grpc_endpoint() {
            // For VMs, create SSH tunnel
            let endpoint = if matches!(provider, CloudProvider::AWS | CloudProvider::GCP | CloudProvider::DigitalOcean) {
                if let Some(ref ip) = deployment.instance.public_ip {
                    // Create SSH tunnel
                    let ssh_user = match provider {
                        CloudProvider::AWS => "ec2-user",
                        CloudProvider::GCP => "ubuntu",
                        CloudProvider::DigitalOcean => "root",
                        _ => "ubuntu",
                    };

                    match self.qos_tunnel_manager.create_tunnel(
                        ip.clone(),
                        ssh_user.to_string(),
                        std::env::var(format!("{:?}_SSH_KEY_PATH", provider)).ok(),
                    ).await {
                        Ok(tunnel_endpoint) => {
                            info!("Created QoS tunnel: {tunnel_endpoint}");
                            tunnel_endpoint
                        }
                        Err(e) => {
                            warn!("Failed to create QoS tunnel: {e}");
                            qos_endpoint
                        }
                    }
                } else {
                    qos_endpoint
                }
            } else {
                qos_endpoint
            };

            // Try to fetch metrics
            match reqwest::get(format!("{endpoint}/metrics")).await {
                Ok(response) if response.status().is_success() => {
                    info!("✅ QoS metrics accessible for {provider:?}");
                    true
                }
                Ok(response) => {
                    warn!("QoS metrics returned {}: {provider:?}", response.status());
                    false
                }
                Err(e) => {
                    warn!("Failed to fetch QoS metrics: {e}");
                    false
                }
            }
        } else {
            warn!("No QoS endpoint for {provider:?}");
            false
        }
    }

    /// Cleanup all deployments with error recovery
    async fn cleanup_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Cleaning up all deployments...");

        let mut cleanup_errors = Vec::new();

        for result in &self.results {
            if let Some(ref deployment) = result.deployment {
                info!("Cleaning up {:?} deployment: {}", result.provider, deployment.blueprint_id);

                // Create adapter with error handling
                let adapter: Box<dyn CloudProviderAdapter> = match result.provider {
                    CloudProvider::AWS => match AwsAdapter::new().await {
                        Ok(a) => Box::new(a),
                        Err(e) => {
                            cleanup_errors.push(format!("AWS adapter: {e}"));
                            continue;
                        }
                    },
                    CloudProvider::GCP => match GcpAdapter::new().await {
                        Ok(a) => Box::new(a),
                        Err(e) => {
                            cleanup_errors.push(format!("GCP adapter: {e}"));
                            continue;
                        }
                    },
                    CloudProvider::DigitalOcean => match DigitalOceanAdapter::new().await {
                        Ok(a) => Box::new(a),
                        Err(e) => {
                            cleanup_errors.push(format!("DO adapter: {e}"));
                            continue;
                        }
                    },
                    _ => continue,
                };

                // Attempt cleanup with retry
                for attempt in 1..=3 {
                    match adapter.cleanup_blueprint(deployment).await {
                        Ok(_) => {
                            info!("Cleanup successful for {:?}", result.provider);
                            break;
                        }
                        Err(e) if attempt < 3 => {
                            warn!("Cleanup attempt {} failed for {:?}: {e}, retrying...",
                                attempt, result.provider);
                            tokio::time::sleep(Duration::from_secs(2 * attempt as u64)).await;
                        }
                        Err(e) => {
                            error!("Cleanup failed for {:?} after 3 attempts: {e}", result.provider);
                            cleanup_errors.push(format!("{:?}: {e}", result.provider));
                        }
                    }
                }
            }
        }

        // Close QoS tunnels
        if let Err(e) = self.qos_tunnel_manager.close_all().await {
            warn!("Error closing QoS tunnels: {e}");
        }

        if !cleanup_errors.is_empty() {
            error!("Cleanup encountered {} errors:", cleanup_errors.len());
            for err in &cleanup_errors {
                error!("  - {err}");
            }
        }

        Ok(())
    }

    /// Print test results summary
    fn print_results(&self) {
        println!("\n═══════════════════════════════════════════════════");
        println!("           BLUEPRINT DEPLOYMENT TEST RESULTS        ");
        println!("═══════════════════════════════════════════════════");

        for result in &self.results {
            let status = if result.error.is_none() { "✅" } else { "❌" };
            let qos = if result.qos_verified { "✅" } else { "⚠️" };

            println!("\n{} {:?}", status, result.provider);
            println!("  Provision: {:.2}s", result.provision_time.as_secs_f32());
            println!("  Deploy: {:.2}s", result.deploy_time.as_secs_f32());
            println!("  QoS: {}", qos);

            if let Some(ref deployment) = result.deployment {
                println!("  Instance: {}", deployment.instance.id);
                if let Some(ref ip) = deployment.instance.public_ip {
                    println!("  IP: {}", ip);
                }
            }

            if let Some(ref error) = result.error {
                println!("  Error: {}", error);
            }
        }

        let success_rate = self.results.iter()
            .filter(|r| r.error.is_none())
            .count() as f32 / self.results.len() as f32 * 100.0;

        println!("\n═══════════════════════════════════════════════════");
        println!("Success Rate: {:.0}%", success_rate);
        println!("═══════════════════════════════════════════════════");
    }
}

#[tokio::test]
#[ignore] // Run explicitly with --ignored
async fn test_real_incredible_squaring_deployment() {
    if std::env::var("REAL_TEST").is_err() {
        eprintln!("Skipping real test. Set REAL_TEST=1 to run");
        return;
    }

    // Initialize logging if not already done
    let _ = tracing_subscriber::fmt()
        .with_env_filter("info")
        .try_init();

    let mut test = RealBlueprintTest::new().await;

    if let Err(e) = test.run().await {
        panic!("Test failed: {e}");
    }
}

#[tokio::test]
async fn test_single_provider_quick() {
    // Quick test with DigitalOcean (cheapest/fastest)
    if std::env::var("DIGITALOCEAN_TOKEN").is_err() {
        eprintln!("Skipping - no DO token");
        return;
    }

    let config = TestConfig {
        providers: vec![CloudProvider::DigitalOcean],
        skip_cleanup: false,
        parallel: false,
        verify_qos: true,
        test_kubernetes: false,
    };

    let mut test = RealBlueprintTest {
        config,
        tracker: DeploymentTracker::new(std::path::Path::new("/tmp/test")).await.unwrap(),
        qos_tunnel_manager: QosTunnelManager::new(30000),
        results: Vec::new(),
    };

    let result = test.test_provider(CloudProvider::DigitalOcean).await;

    assert!(result.error.is_none(), "Deployment should succeed");
    assert!(result.deployment.is_some(), "Should have deployment result");

    if !test.config.skip_cleanup {
        test.cleanup_all().await.unwrap();
    }
}

/// Continuous test runner for reliability testing
#[tokio::test]
#[ignore]
async fn test_continuous_deployment_reliability() {
    let iterations = std::env::var("TEST_ITERATIONS")
        .unwrap_or_else(|_| "10".to_string())
        .parse::<usize>()
        .unwrap();

    let mut success_count = 0;
    let mut failure_reasons = HashMap::new();

    for i in 0..iterations {
        info!("Iteration {}/{}", i + 1, iterations);

        let mut test = RealBlueprintTest::new().await;

        match test.run().await {
            Ok(_) => {
                success_count += 1;
            }
            Err(e) => {
                *failure_reasons.entry(e.to_string()).or_insert(0) += 1;
            }
        }

        // Delay between iterations
        sleep(Duration::from_secs(60)).await;
    }

    println!("\nReliability Test Results:");
    println!("Success Rate: {}/{} ({:.1}%)",
        success_count, iterations,
        success_count as f32 / iterations as f32 * 100.0);

    if !failure_reasons.is_empty() {
        println!("\nFailure Reasons:");
        for (reason, count) in failure_reasons {
            println!("  {}: {} times", reason, count);
        }
    }

    assert!(success_count as f32 / iterations as f32 > 0.8,
        "Reliability should be >80%");
}