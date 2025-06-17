#[cfg(test)]
mod default_metrics_provider_tests {
    use blueprint_qos::metrics::provider::default::DefaultMetricsProvider;
    use blueprint_qos::metrics::types::{MetricsConfig, MetricsProvider};
    use std::sync::Arc;

    use std::time::Duration;

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_collection_skip_and_resume_system_metrics() {
        let config = MetricsConfig {
            collection_interval_secs: 1,
            max_history: 20,
            ..Default::default()
        };
        let provider = Arc::new(DefaultMetricsProvider::new(config));

        let system_metrics_lock_clone = provider.system_metrics_arc_clone();

        provider
            .start_collection()
            .await
            .expect("Failed to start collection");
        println!("Test: Collection started.");

        tokio::time::sleep(Duration::from_millis(500)).await;

        println!("Test: Attempting to acquire system_metrics lock...");
        let guard = system_metrics_lock_clone.write().await;
        println!("Test: system_metrics lock acquired.");
        tokio::task::yield_now().await;

        println!("Test: Sleeping for 3s while lock is held...");
        tokio::time::sleep(Duration::from_secs(3)).await;

        let history_while_locked = provider.get_system_metrics_history().await;
        assert!(
            history_while_locked.len() <= 1,
            "System metrics history should not grow significantly while locked. Len: {}",
            history_while_locked.len()
        );

        let bp_history_while_sys_locked = provider.get_blueprint_metrics_history().await;
        assert!(
            bp_history_while_sys_locked.len() > 1,
            "Blueprint metrics should have been collected. Len: {}",
            bp_history_while_sys_locked.len()
        );

        println!("Test: Releasing system_metrics lock...");
        drop(guard);
        println!("Test: system_metrics lock released.");
        tokio::task::yield_now().await;

        println!("Test: Sleeping for 5s to allow collections to resume...");
        tokio::time::sleep(Duration::from_secs(5)).await;

        let history_after_release = provider.get_system_metrics_history().await;
        assert!(
            history_after_release.len() > history_while_locked.len(),
            "System metrics history should grow after lock is released. Before: {}, After: {}",
            history_while_locked.len(),
            history_after_release.len()
        );
        assert!(
            history_after_release.len() > 2,
            "Expected more than 2 system metrics collections after release, got {}",
            history_after_release.len()
        );

        println!("Test test_collection_skip_and_resume_system_metrics passed.");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_metrics_history_limit() {
        let max_hist = 5;
        let config = MetricsConfig {
            collection_interval_secs: 1,
            max_history: max_hist,
            ..Default::default()
        };
        let provider = Arc::new(DefaultMetricsProvider::new(config.clone()));

        provider
            .start_collection()
            .await
            .expect("Failed to start collection");

        let num_cycles_to_wait = max_hist + 2;
        tokio::time::sleep(Duration::from_secs(
            config.collection_interval_secs * num_cycles_to_wait as u64,
        ))
        .await;

        let system_history = provider.get_system_metrics_history().await;
        let blueprint_history = provider.get_blueprint_metrics_history().await;

        assert_eq!(
            system_history.len(),
            max_hist,
            "System metrics history should be limited to max_history. Expected {}, got {}. History: {:?}",
            max_hist,
            system_history.len(),
            system_history
        );
        assert_eq!(
            blueprint_history.len(),
            max_hist,
            "Blueprint metrics history should be limited to max_history. Expected {}, got {}. History: {:?}",
            max_hist,
            blueprint_history.len(),
            blueprint_history
        );

        println!("Test test_metrics_history_limit passed.");
    }
}
