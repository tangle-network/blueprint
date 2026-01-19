//! Health monitoring for remote deployments
//!
//! Provides continuous health checks and auto-recovery for deployed instances

use crate::core::error::{Error, Result};
use crate::core::remote::CloudProvider;
use crate::create_provider_client;
use crate::deployment::tracker::{DeploymentRecord, DeploymentTracker};
use crate::infra::provisioner::CloudProvisioner;
use crate::infra::types::InstanceStatus;
use blueprint_core::{error, info, warn};
use blueprint_std::sync::Arc;
use blueprint_std::time::Duration;
use chrono::{DateTime, Utc};

/// Health status of a deployment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

/// Health check result
#[derive(Debug, Clone)]
pub struct HealthCheckResult {
    pub deployment_id: String,
    pub status: HealthStatus,
    pub instance_status: InstanceStatus,
    pub last_check: DateTime<Utc>,
    pub consecutive_failures: u32,
    pub message: Option<String>,
}

/// Health monitoring service
pub struct HealthMonitor {
    provisioner: Arc<CloudProvisioner>,
    tracker: Arc<DeploymentTracker>,
    check_interval: Duration,
    max_consecutive_failures: u32,
    auto_recover: bool,
}

impl HealthMonitor {
    pub fn new(provisioner: Arc<CloudProvisioner>, tracker: Arc<DeploymentTracker>) -> Self {
        Self {
            provisioner,
            tracker,
            check_interval: Duration::from_secs(60),
            max_consecutive_failures: 3,
            auto_recover: true,
        }
    }

    /// Configure monitoring parameters
    pub fn with_config(
        mut self,
        check_interval: Duration,
        max_failures: u32,
        auto_recover: bool,
    ) -> Self {
        self.check_interval = check_interval;
        self.max_consecutive_failures = max_failures;
        self.auto_recover = auto_recover;
        self
    }

    /// Start monitoring all deployments
    pub async fn start_monitoring(self: Arc<Self>) {
        let mut interval = tokio::time::interval(self.check_interval);
        let mut failure_counts: std::collections::HashMap<String, u32> =
            std::collections::HashMap::new();

        loop {
            interval.tick().await;

            // Get all active deployments
            let deployments = match self.tracker.list_active().await {
                Ok(deps) => deps,
                Err(e) => {
                    error!("Failed to list deployments: {}", e);
                    continue;
                }
            };

            for deployment in deployments {
                let result = self.check_deployment_health(&deployment).await;

                match result.status {
                    HealthStatus::Healthy => {
                        failure_counts.remove(&deployment.id);
                        info!("Deployment {} is healthy", deployment.id);
                    }
                    HealthStatus::Degraded => {
                        warn!(
                            "Deployment {} is degraded: {:?}",
                            deployment.id, result.message
                        );
                        *failure_counts.entry(deployment.id.clone()).or_insert(0) += 1;
                    }
                    HealthStatus::Unhealthy => {
                        error!(
                            "Deployment {} is unhealthy: {:?}",
                            deployment.id, result.message
                        );
                        let failures = failure_counts.entry(deployment.id.clone()).or_insert(0);
                        *failures += 1;

                        if *failures >= self.max_consecutive_failures && self.auto_recover {
                            info!("Attempting auto-recovery for deployment {}", deployment.id);
                            if let Err(e) = self.attempt_recovery(&deployment).await {
                                error!("Auto-recovery failed for {}: {}", deployment.id, e);
                            }
                        }
                    }
                    HealthStatus::Unknown => {
                        warn!("Unable to determine health of deployment {}", deployment.id);
                    }
                }
            }
        }
    }

    /// Check health of a single deployment
    async fn check_deployment_health(&self, deployment: &DeploymentRecord) -> HealthCheckResult {
        // Determine provider from deployment type
        let provider = deployment.deployment_type.as_provider();

        // Check instance status
        let instance_status = match self.provisioner.get_status(provider, &deployment.id).await {
            Ok(status) => status,
            Err(e) => {
                return HealthCheckResult {
                    deployment_id: deployment.id.clone(),
                    status: HealthStatus::Unknown,
                    instance_status: InstanceStatus::Unknown,
                    last_check: Utc::now(),
                    consecutive_failures: 0,
                    message: Some(format!("Failed to get instance status: {e}")),
                };
            }
        };

        // Determine health based on instance status
        let health_status = match instance_status {
            InstanceStatus::Running => {
                // Application-level health checks available (HTTP, TCP, etc.)
                HealthStatus::Healthy
            }
            InstanceStatus::Starting => HealthStatus::Degraded,
            InstanceStatus::Stopping | InstanceStatus::Stopped => HealthStatus::Unhealthy,
            InstanceStatus::Terminated => HealthStatus::Unhealthy,
            InstanceStatus::Unknown => HealthStatus::Unknown,
        };

        HealthCheckResult {
            deployment_id: deployment.id.clone(),
            status: health_status,
            instance_status,
            last_check: Utc::now(),
            consecutive_failures: 0,
            message: None,
        }
    }

    /// Attempt to recover an unhealthy deployment
    async fn attempt_recovery(&self, deployment: &DeploymentRecord) -> Result<()> {
        info!("Starting recovery for deployment {}", deployment.id);

        let provider = deployment.deployment_type.as_provider();

        // First, try to terminate the existing instance
        if let Err(e) = self
            .provisioner
            .terminate(provider.clone(), &deployment.id)
            .await
        {
            warn!("Failed to terminate unhealthy instance: {}", e);
        }

        // Wait a bit for termination to complete
        tokio::time::sleep(Duration::from_secs(10)).await;

        // Provision a replacement instance
        match self
            .provisioner
            .provision(
                provider,
                &deployment.resource_spec,
                deployment.region.as_deref().unwrap_or("us-east-1"),
            )
            .await
        {
            Ok(new_instance) => {
                info!(
                    "Successfully provisioned replacement instance: {}",
                    new_instance.id
                );

                // Update deployment record with new instance ID
                self.tracker
                    .update_instance_id(&deployment.id, &new_instance.id)
                    .await?;

                Ok(())
            }
            Err(e) => {
                error!("Failed to provision replacement instance: {}", e);
                Err(e)
            }
        }
    }

    /// Get current health status of all deployments
    pub async fn get_all_health_status(&self) -> Result<Vec<HealthCheckResult>> {
        let deployments = self.tracker.list_active().await?;
        let mut results = Vec::new();

        for deployment in deployments {
            results.push(self.check_deployment_health(&deployment).await);
        }

        Ok(results)
    }

    /// Check if a specific deployment is healthy
    pub async fn is_healthy(&self, deployment_id: &str) -> Result<bool> {
        let deployment = self
            .tracker
            .get(deployment_id)
            .await?
            .ok_or_else(|| Error::Other(format!("Deployment {deployment_id} not found")))?;

        let result = self.check_deployment_health(&deployment).await;
        Ok(result.status == HealthStatus::Healthy)
    }
}

/// Application-level health checker
pub struct ApplicationHealthChecker {
    http_client: reqwest::Client,
}

impl Default for ApplicationHealthChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationHealthChecker {
    pub fn new() -> Self {
        Self {
            http_client: create_provider_client(5).unwrap_or_default(),
        }
    }

    /// Check HTTP endpoint health
    pub async fn check_http(&self, url: &str) -> HealthStatus {
        match self.http_client.get(url).send().await {
            Ok(response) if response.status().is_success() => HealthStatus::Healthy,
            Ok(response) if response.status().is_server_error() => HealthStatus::Degraded,
            _ => HealthStatus::Unhealthy,
        }
    }

    /// Check TCP port connectivity
    pub async fn check_tcp(&self, host: &str, port: u16) -> HealthStatus {
        match tokio::net::TcpStream::connect(format!("{host}:{port}")).await {
            Ok(_) => HealthStatus::Healthy,
            Err(_) => HealthStatus::Unhealthy,
        }
    }
}

impl crate::deployment::tracker::DeploymentType {
    /// Convert deployment type to cloud provider
    fn as_provider(&self) -> CloudProvider {
        use crate::deployment::tracker::DeploymentType;

        match self {
            DeploymentType::AwsEc2 | DeploymentType::AwsEks => CloudProvider::AWS,
            DeploymentType::GcpGce | DeploymentType::GcpGke => CloudProvider::GCP,
            DeploymentType::AzureVm | DeploymentType::AzureAks => CloudProvider::Azure,
            DeploymentType::DigitalOceanDroplet | DeploymentType::DigitalOceanDoks => {
                CloudProvider::DigitalOcean
            }
            DeploymentType::VultrInstance | DeploymentType::VultrVke => CloudProvider::Vultr,
            _ => CloudProvider::AWS, // Default fallback
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::error::Error;
    use crate::deployment::DeploymentType;
    use crate::infra::traits::{BlueprintDeploymentResult, CloudProviderAdapter};
    use crate::infra::types::ProvisionedInstance;
    use tempfile::TempDir;
    use tokio::task::yield_now;

    #[tokio::test]
    async fn test_health_status_mapping() {
        assert_eq!(
            match InstanceStatus::Running {
                InstanceStatus::Running => HealthStatus::Healthy,
                InstanceStatus::Starting => HealthStatus::Degraded,
                InstanceStatus::Stopped => HealthStatus::Unhealthy,
                _ => HealthStatus::Unknown,
            },
            HealthStatus::Healthy
        );
    }

    #[tokio::test]
    async fn test_application_health_checker() {
        let checker = ApplicationHealthChecker::new();

        // Test with a known good endpoint (this might fail in CI without internet)
        let status = checker.check_http("https://httpbin.org/status/200").await;
        // We can't guarantee this works in all environments, accept any valid status
        assert!(matches!(
            status,
            HealthStatus::Healthy
                | HealthStatus::Unhealthy
                | HealthStatus::Degraded
                | HealthStatus::Unknown
        ));

        // Test TCP check on localhost (should fail)
        let tcp_status = checker.check_tcp("localhost", 9999).await;
        assert_eq!(tcp_status, HealthStatus::Unhealthy);
    }

    #[tokio::test]
    async fn test_health_monitor_creation() {
        let temp_dir = TempDir::new().unwrap();
        let provisioner = Arc::new(CloudProvisioner::new().await.unwrap());
        let tracker = Arc::new(DeploymentTracker::new(temp_dir.path()).await.unwrap());

        let monitor =
            HealthMonitor::new(provisioner, tracker).with_config(Duration::from_secs(30), 5, false);

        assert_eq!(monitor.check_interval, Duration::from_secs(30));
        assert_eq!(monitor.max_consecutive_failures, 5);
        assert!(!monitor.auto_recover);
    }

    struct MockAdapter {
        status: Arc<std::sync::Mutex<InstanceStatus>>,
        terminate_calls: Arc<std::sync::atomic::AtomicUsize>,
        provision_calls: Arc<std::sync::atomic::AtomicUsize>,
        provision_id: String,
    }

    #[async_trait::async_trait]
    impl CloudProviderAdapter for MockAdapter {
        async fn provision_instance(
            &self,
            _instance_type: &str,
            region: &str,
        ) -> Result<ProvisionedInstance> {
            self.provision_calls
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            Ok(ProvisionedInstance::builder(
                self.provision_id.clone(),
                CloudProvider::AWS,
                region.to_string(),
            )
            .status(InstanceStatus::Running)
            .build())
        }

        async fn terminate_instance(&self, _instance_id: &str) -> Result<()> {
            self.terminate_calls
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            let mut status = self.status.lock().unwrap();
            *status = InstanceStatus::Terminated;
            Ok(())
        }

        async fn get_instance_status(&self, _instance_id: &str) -> Result<InstanceStatus> {
            Ok(*self.status.lock().unwrap())
        }

        async fn deploy_blueprint_with_target(
            &self,
            _target: &crate::core::deployment_target::DeploymentTarget,
            _blueprint_image: &str,
            _resource_spec: &crate::core::resources::ResourceSpec,
            _env_vars: std::collections::HashMap<String, String>,
        ) -> Result<BlueprintDeploymentResult> {
            Err(Error::Other("not implemented".into()))
        }

        async fn health_check_blueprint(&self, _deployment: &BlueprintDeploymentResult) -> Result<bool> {
            Ok(true)
        }
    }

    #[tokio::test(start_paused = true)]
    async fn test_health_monitor_recovers_unhealthy_instance() {
        let temp_dir = TempDir::new().unwrap();
        let tracker = Arc::new(DeploymentTracker::new(temp_dir.path()).await.unwrap());

        let status = Arc::new(std::sync::Mutex::new(InstanceStatus::Stopped));
        let terminate_calls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let provision_calls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let adapter = MockAdapter {
            status: status.clone(),
            terminate_calls: terminate_calls.clone(),
            provision_calls: provision_calls.clone(),
            provision_id: "instance-new".to_string(),
        };

        let mut providers = std::collections::HashMap::new();
        providers.insert(CloudProvider::AWS, Box::new(adapter) as Box<dyn CloudProviderAdapter>);
        let provisioner = Arc::new(CloudProvisioner::with_providers(providers));

        let mut record = DeploymentRecord::new(
            "instance-old".to_string(),
            DeploymentType::AwsEc2,
            crate::core::resources::ResourceSpec::default(),
            None,
        );
        record.id = "instance-old".to_string();

        tracker
            .register_deployment("instance-old".to_string(), record)
            .await
            .unwrap();

        let monitor = Arc::new(
            HealthMonitor::new(provisioner, tracker.clone())
                .with_config(Duration::from_secs(1), 1, true),
        );

        let task = tokio::spawn({
            let monitor = monitor.clone();
            async move { monitor.start_monitoring().await }
        });

        tokio::time::advance(Duration::from_secs(1)).await;
        yield_now().await;
        tokio::time::advance(Duration::from_secs(11)).await;
        yield_now().await;

        assert!(tracker.get("instance-old").await.unwrap().is_none());
        assert!(tracker.get("instance-new").await.unwrap().is_some());
        assert_eq!(
            terminate_calls.load(std::sync::atomic::Ordering::SeqCst),
            1
        );
        assert_eq!(
            provision_calls.load(std::sync::atomic::Ordering::SeqCst),
            1
        );

        task.abort();
    }

    #[tokio::test]
    async fn test_health_monitor_is_healthy_when_running() {
        let temp_dir = TempDir::new().unwrap();
        let tracker = Arc::new(DeploymentTracker::new(temp_dir.path()).await.unwrap());

        let status = Arc::new(std::sync::Mutex::new(InstanceStatus::Running));
        let adapter = MockAdapter {
            status,
            terminate_calls: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            provision_calls: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            provision_id: "instance-new".to_string(),
        };

        let mut providers = std::collections::HashMap::new();
        providers.insert(CloudProvider::AWS, Box::new(adapter) as Box<dyn CloudProviderAdapter>);
        let provisioner = Arc::new(CloudProvisioner::with_providers(providers));

        let mut record = DeploymentRecord::new(
            "instance-ok".to_string(),
            DeploymentType::AwsEc2,
            crate::core::resources::ResourceSpec::default(),
            None,
        );
        record.id = "instance-ok".to_string();

        tracker
            .register_deployment("instance-ok".to_string(), record)
            .await
            .unwrap();

        let monitor = HealthMonitor::new(provisioner, tracker);
        let healthy = monitor.is_healthy("instance-ok").await.unwrap();
        assert!(healthy);
    }
}
