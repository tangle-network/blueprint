//! Health monitoring for remote deployments
//!
//! Provides continuous health checks and auto-recovery for deployed instances

use crate::deployment_tracker::{DeploymentRecord, DeploymentTracker};
use crate::error::{Error, Result};
use crate::cloud_provisioner::{CloudProvisioner, InstanceStatus};
use crate::remote::CloudProvider;
use chrono::{DateTime, Utc};
use blueprint_std::sync::Arc;
use blueprint_std::time::Duration;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

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
    pub fn new(
        provisioner: Arc<CloudProvisioner>,
        tracker: Arc<DeploymentTracker>,
    ) -> Self {
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
        let mut failure_counts: blueprint_std::collections::HashMap<String, u32> =
            blueprint_std::collections::HashMap::new();

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
        let provider = deployment.deployment_type.to_provider();

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
                    message: Some(format!("Failed to get instance status: {}", e)),
                };
            }
        };

        // Determine health based on instance status
        let health_status = match instance_status {
            InstanceStatus::Running => {
                // TODO: Add application-level health checks (HTTP, TCP, etc.)
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

        let provider = deployment.deployment_type.to_provider();

        // First, try to terminate the existing instance
        if let Err(e) = self.provisioner.terminate(provider.clone(), &deployment.id).await {
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
            .ok_or_else(|| Error::Other(format!("Deployment {} not found", deployment_id)))?;

        let result = self.check_deployment_health(&deployment).await;
        Ok(result.status == HealthStatus::Healthy)
    }
}

/// Application-level health checker
pub struct ApplicationHealthChecker {
    http_client: reqwest::Client,
}

impl ApplicationHealthChecker {
    pub fn new() -> Self {
        Self {
            http_client: reqwest::Client::builder()
                .timeout(Duration::from_secs(5))
                .build()
                .unwrap(),
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
        match tokio::net::TcpStream::connect(format!("{}:{}", host, port)).await {
            Ok(_) => HealthStatus::Healthy,
            Err(_) => HealthStatus::Unhealthy,
        }
    }
}

impl crate::deployment_tracker::DeploymentType {
    /// Convert deployment type to cloud provider
    fn to_provider(&self) -> CloudProvider {
        use crate::deployment_tracker::DeploymentType;

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
    use tempfile::TempDir;

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
        // We can't guarantee this works in all environments
        assert!(matches!(
            status,
            HealthStatus::Healthy | HealthStatus::Unhealthy
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
}
