//! Deployment tracking and lifecycle management
//!
//! Maps Blueprint service instances to their actual deployed infrastructure
//! and handles cleanup when services are terminated or TTL expires.

mod cleanup;
mod core;
mod types;

// Re-export public API
pub use self::core::{DeploymentTracker, ttl_checker_task};
pub use types::{CleanupHandler, DeploymentRecord, DeploymentStatus, DeploymentType};

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_deployment_registration() {
        let temp_dir = TempDir::new().unwrap();
        let tracker = DeploymentTracker::new(temp_dir.path()).await.unwrap();

        let mut record = DeploymentRecord::new(
            "blueprint-123".to_string(),
            DeploymentType::LocalDocker,
            crate::core::resources::ResourceSpec::default(),
            Some(3600),
        );
        record.add_resource("container_id".to_string(), "abc123".to_string());

        tracker
            .register_deployment("blueprint-123".to_string(), record)
            .await
            .unwrap();

        let status = tracker.get_deployment_status("blueprint-123").await;
        assert!(matches!(status, Some(DeploymentStatus::Active)));
    }

    #[tokio::test]
    async fn test_ttl_expiry() {
        let temp_dir = TempDir::new().unwrap();
        let tracker = DeploymentTracker::new(temp_dir.path()).await.unwrap();

        let mut record = DeploymentRecord::new(
            "blueprint-ttl".to_string(),
            DeploymentType::LocalDocker,
            crate::core::resources::ResourceSpec::default(),
            Some(0), // Immediate expiry
        );
        record.expires_at = Some(Utc::now() - Duration::seconds(1));
        record.add_resource("container_id".to_string(), "expired123".to_string());

        tracker
            .register_deployment("blueprint-ttl".to_string(), record)
            .await
            .unwrap();

        // Check TTLs
        tracker.check_all_ttls().await.unwrap();

        // Should be cleaned up
        let status = tracker.get_deployment_status("blueprint-ttl").await;
        assert!(status.is_none());
    }
}
