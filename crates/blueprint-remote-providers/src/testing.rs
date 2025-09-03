use crate::{
    cost::{CostEstimator, CostReport},
    networking::{NetworkingMode, TunnelConnection, TunnelManager, TunnelType},
    remote::{CloudProvider, RemoteClusterManager, RemoteDeploymentConfig},
};

/// Test helpers for remote deployments
pub struct TestHelper;

impl TestHelper {
    /// Create a test cluster manager with mock clusters
    pub async fn create_test_manager() -> RemoteClusterManager {
        let manager = RemoteClusterManager::new();
        
        // Add mock AWS cluster config
        let aws_config = RemoteDeploymentConfig {
            namespace: "test-aws".to_string(),
            provider: CloudProvider::AWS,
            region: Some("us-west-2".to_string()),
            ..Default::default()
        };
        
        // This will fail without real kubeconfig, but sets up the structure
        let _ = manager.add_cluster("aws-test".to_string(), aws_config).await;
        
        manager
    }
    
    /// Create test cost estimator with sample data
    pub fn create_test_estimator() -> CostEstimator {
        CostEstimator::new()
    }
    
    /// Create test tunnel manager
    pub fn create_test_tunnel_manager() -> TunnelManager {
        TunnelManager::new(NetworkingMode::Direct)
    }
    
    /// Verify cost report is reasonable
    pub fn validate_cost_report(report: &CostReport) -> bool {
        report.estimated_hourly >= 0.0 &&
        report.estimated_monthly >= report.estimated_hourly * 24.0 * 28.0 &&
        report.currency == "USD" &&
        !report.breakdown.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_helper_creates_manager() {
        let manager = TestHelper::create_test_manager().await;
        let clusters = manager.list_clusters().await;
        // Should be empty since we don't have valid kubeconfig
        assert_eq!(clusters.len(), 0);
    }
    
    #[test]
    fn test_helper_creates_estimator() {
        let estimator = TestHelper::create_test_estimator();
        let report = estimator.estimate(
            &CloudProvider::Generic,
            1.0,
            1.0,
            1.0,
            1,
        );
        assert!(TestHelper::validate_cost_report(&report));
    }
}