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
    use std::collections::HashMap;
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };
    use tempfile::TempDir;

    #[tokio::test(start_paused = true)]
    async fn test_handle_termination_retries_then_succeeds() {
        let temp_dir = TempDir::new().unwrap();
        let tracker = DeploymentTracker::new(temp_dir.path()).await.unwrap();

        struct FlakyCleanup {
            attempts: Arc<AtomicUsize>,
            fail_count: usize,
        }

        #[async_trait::async_trait]
        impl CleanupHandler for FlakyCleanup {
            async fn cleanup(
                &self,
                _deployment: &DeploymentRecord,
            ) -> crate::core::error::Result<()> {
                let attempt = self.attempts.fetch_add(1, Ordering::SeqCst) + 1;
                if attempt <= self.fail_count {
                    Err(crate::core::error::Error::Other("cleanup failed".into()))
                } else {
                    Ok(())
                }
            }
        }

        let attempts = Arc::new(AtomicUsize::new(0));
        tracker
            .set_cleanup_handler(
                DeploymentType::LocalDocker,
                Box::new(FlakyCleanup {
                    attempts: attempts.clone(),
                    fail_count: 2,
                }),
            )
            .await;

        let mut record = DeploymentRecord::new(
            "blueprint-retry".to_string(),
            DeploymentType::LocalDocker,
            crate::core::resources::ResourceSpec::default(),
            None,
        );
        record.add_resource("container_id".to_string(), "retry123".to_string());

        tracker
            .register_deployment("blueprint-retry".to_string(), record)
            .await
            .unwrap();

        let tracker_clone = tracker.clone();
        let task = tokio::spawn(async move { tracker_clone.handle_termination("blueprint-retry").await });

        tokio::time::advance(blueprint_std::time::Duration::from_secs(20)).await;
        let result = task.await.unwrap();

        assert!(result.is_ok());
        assert_eq!(attempts.load(Ordering::SeqCst), 3);
        assert!(tracker.get_deployment_status("blueprint-retry").await.is_none());
    }

    #[tokio::test(start_paused = true)]
    async fn test_handle_termination_retries_and_preserves_record_on_failure() {
        let temp_dir = TempDir::new().unwrap();
        let tracker = DeploymentTracker::new(temp_dir.path()).await.unwrap();

        struct AlwaysFailCleanup {
            attempts: Arc<AtomicUsize>,
        }

        #[async_trait::async_trait]
        impl CleanupHandler for AlwaysFailCleanup {
            async fn cleanup(
                &self,
                _deployment: &DeploymentRecord,
            ) -> crate::core::error::Result<()> {
                self.attempts.fetch_add(1, Ordering::SeqCst);
                Err(crate::core::error::Error::Other("cleanup failed".into()))
            }
        }

        let attempts = Arc::new(AtomicUsize::new(0));
        tracker
            .set_cleanup_handler(
                DeploymentType::LocalDocker,
                Box::new(AlwaysFailCleanup {
                    attempts: attempts.clone(),
                }),
            )
            .await;

        let mut record = DeploymentRecord::new(
            "blueprint-fail".to_string(),
            DeploymentType::LocalDocker,
            crate::core::resources::ResourceSpec::default(),
            None,
        );
        record.add_resource("container_id".to_string(), "fail123".to_string());

        tracker
            .register_deployment("blueprint-fail".to_string(), record)
            .await
            .unwrap();

        let tracker_clone = tracker.clone();
        let task = tokio::spawn(async move { tracker_clone.handle_termination("blueprint-fail").await });

        tokio::time::advance(blueprint_std::time::Duration::from_secs(20)).await;
        let result = task.await.unwrap();

        assert!(result.is_err());
        assert_eq!(attempts.load(Ordering::SeqCst), 3);
        assert!(matches!(
            tracker.get_deployment_status("blueprint-fail").await,
            Some(DeploymentStatus::Active)
        ));
    }

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
    async fn test_deployment_registration_persists_state() {
        let temp_dir = TempDir::new().unwrap();
        let tracker = DeploymentTracker::new(temp_dir.path()).await.unwrap();

        let mut record = DeploymentRecord::new(
            "blueprint-456".to_string(),
            DeploymentType::LocalDocker,
            crate::core::resources::ResourceSpec::default(),
            None,
        );
        record.add_resource("container_id".to_string(), "persist123".to_string());

        tracker
            .register_deployment("blueprint-456".to_string(), record)
            .await
            .unwrap();

        let state_path = temp_dir.path().join("deployment_state.json");
        let content = tokio::fs::read_to_string(&state_path).await.unwrap();
        let state: HashMap<String, DeploymentRecord> = serde_json::from_str(&content).unwrap();

        let stored = state.get("blueprint-456").unwrap();
        assert_eq!(stored.blueprint_id, "blueprint-456");
        assert_eq!(
            stored.resource_ids.get("container_id").map(String::as_str),
            Some("persist123")
        );
    }

    #[tokio::test]
    async fn test_handle_termination_runs_cleanup_and_removes_record() {
        let temp_dir = TempDir::new().unwrap();
        let tracker = DeploymentTracker::new(temp_dir.path()).await.unwrap();

        struct CountingCleanup {
            calls: Arc<AtomicUsize>,
        }

        #[async_trait::async_trait]
        impl CleanupHandler for CountingCleanup {
            async fn cleanup(
                &self,
                _deployment: &DeploymentRecord,
            ) -> crate::core::error::Result<()> {
                self.calls.fetch_add(1, Ordering::SeqCst);
                Ok(())
            }
        }

        let calls = Arc::new(AtomicUsize::new(0));
        tracker
            .set_cleanup_handler(
                DeploymentType::LocalDocker,
                Box::new(CountingCleanup { calls: calls.clone() }),
            )
            .await;

        let mut record = DeploymentRecord::new(
            "blueprint-terminate".to_string(),
            DeploymentType::LocalDocker,
            crate::core::resources::ResourceSpec::default(),
            None,
        );
        record.add_resource("container_id".to_string(), "cleanup123".to_string());

        tracker
            .register_deployment("blueprint-terminate".to_string(), record)
            .await
            .unwrap();

        tracker
            .handle_termination("blueprint-terminate")
            .await
            .unwrap();

        assert_eq!(calls.load(Ordering::SeqCst), 1);
        let status = tracker.get_deployment_status("blueprint-terminate").await;
        assert!(status.is_none());

        let state_path = temp_dir.path().join("deployment_state.json");
        let content = tokio::fs::read_to_string(&state_path).await.unwrap();
        let state: HashMap<String, DeploymentRecord> = serde_json::from_str(&content).unwrap();
        assert!(!state.contains_key("blueprint-terminate"));
    }

    #[tokio::test]
    async fn test_handle_termination_missing_record_returns_error() {
        let temp_dir = TempDir::new().unwrap();
        let tracker = DeploymentTracker::new(temp_dir.path()).await.unwrap();

        let err = tracker
            .handle_termination("missing-blueprint")
            .await
            .unwrap_err();

        assert!(matches!(
            err,
            crate::core::error::Error::ConfigurationError(_)
        ));
    }

    #[tokio::test]
    async fn test_ttl_expiry() {
        let temp_dir = TempDir::new().unwrap();
        let tracker = DeploymentTracker::new(temp_dir.path()).await.unwrap();

        struct NoopCleanup;
        #[async_trait::async_trait]
        impl CleanupHandler for NoopCleanup {
            async fn cleanup(
                &self,
                _deployment: &DeploymentRecord,
            ) -> crate::core::error::Result<()> {
                Ok(())
            }
        }

        tracker
            .set_cleanup_handler(DeploymentType::LocalDocker, Box::new(NoopCleanup))
            .await;

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
