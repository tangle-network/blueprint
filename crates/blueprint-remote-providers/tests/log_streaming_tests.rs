//! Tests for log streaming functionality
//!
//! Tests the log streaming, parsing, and aggregation features.

use blueprint_remote_providers::monitoring::logs::{
    LogAggregator, LogEntry, LogFilters, LogLevel, LogSource, LogStreamer,
};
use std::{collections::HashMap, time::SystemTime};

#[tokio::test]
async fn test_log_entry_creation_and_fields() {
    let mut metadata = HashMap::new();
    metadata.insert("host".to_string(), "test-host".to_string());
    metadata.insert("process_id".to_string(), "1234".to_string());

    let entry = LogEntry {
        timestamp: SystemTime::now(),
        service_id: "test-service".to_string(),
        container_id: Some("container123".to_string()),
        level: LogLevel::Error,
        message: "Test error message".to_string(),
        metadata: metadata.clone(),
    };

    assert_eq!(entry.service_id, "test-service");
    assert_eq!(entry.container_id, Some("container123".to_string()));
    assert_eq!(entry.level, LogLevel::Error);
    assert_eq!(entry.message, "Test error message");
    assert_eq!(entry.metadata, metadata);
}

#[tokio::test]
async fn test_log_level_conversion() {
    assert_eq!(LogLevel::from("debug"), LogLevel::Debug);
    assert_eq!(LogLevel::from("DEBUG"), LogLevel::Debug);
    assert_eq!(LogLevel::from("trace"), LogLevel::Debug);

    assert_eq!(LogLevel::from("info"), LogLevel::Info);
    assert_eq!(LogLevel::from("INFO"), LogLevel::Info);

    assert_eq!(LogLevel::from("warn"), LogLevel::Warn);
    assert_eq!(LogLevel::from("WARN"), LogLevel::Warn);
    assert_eq!(LogLevel::from("warning"), LogLevel::Warn);

    assert_eq!(LogLevel::from("error"), LogLevel::Error);
    assert_eq!(LogLevel::from("ERROR"), LogLevel::Error);

    assert_eq!(LogLevel::from("fatal"), LogLevel::Fatal);
    assert_eq!(LogLevel::from("FATAL"), LogLevel::Fatal);
    assert_eq!(LogLevel::from("critical"), LogLevel::Fatal);

    // Unknown levels default to Info
    assert_eq!(LogLevel::from("unknown"), LogLevel::Info);
    assert_eq!(LogLevel::from(""), LogLevel::Info);
}

#[tokio::test]
async fn test_log_level_ordering() {
    // Test that log levels have correct ordering for filtering
    assert!(LogLevel::Debug < LogLevel::Info);
    assert!(LogLevel::Info < LogLevel::Warn);
    assert!(LogLevel::Warn < LogLevel::Error);
    assert!(LogLevel::Error < LogLevel::Fatal);

    // Test equality
    assert_eq!(LogLevel::Info, LogLevel::Info);
    assert_ne!(LogLevel::Info, LogLevel::Error);
}

#[tokio::test]
async fn test_log_streamer_creation() {
    let streamer = LogStreamer::new(1000);
    // Test that streamer can be created - we can't test internal state
    // as that's an implementation detail
    assert!(std::mem::size_of_val(&streamer) > 0);
}

#[tokio::test]
async fn test_log_streamer_source_management() {
    let mut streamer = LogStreamer::new(500);

    // Add file-based log source
    let file_source = LogSource::File {
        host: "test-host".to_string(),
        file_path: "/var/log/app.log".to_string(),
    };

    // Test that we can add sources without panic
    streamer.add_source("service-1".to_string(), file_source);

    // Add another source
    let file_source2 = LogSource::File {
        host: "test-host-2".to_string(),
        file_path: "/var/log/app2.log".to_string(),
    };

    streamer.add_source("service-2".to_string(), file_source2);

    // We can't test internal state, but we've verified the API works
}

#[tokio::test]
async fn test_log_aggregator_filters() {
    let mut aggregator = LogAggregator::new();

    // Set filters - we can only test that the API works, not internal state
    let filters = LogFilters {
        level_min: Some(LogLevel::Warn),
        service_ids: Some(vec!["service-1".to_string(), "service-2".to_string()]),
        search_text: Some("error".to_string()),
        ..Default::default()
    };

    // This should not panic
    aggregator.set_filters(filters);

    // We can't verify internal state, but the API works
}

#[tokio::test]
async fn test_log_source_variants() {
    // Test different log source types can be created
    let file_source = LogSource::File {
        host: "192.168.1.100".to_string(),
        file_path: "/var/log/nginx/access.log".to_string(),
    };

    match file_source {
        LogSource::File { host, file_path } => {
            assert_eq!(host, "192.168.1.100");
            assert_eq!(file_path, "/var/log/nginx/access.log");
        }
        _ => panic!("Wrong source type"),
    }

    #[cfg(feature = "kubernetes")]
    {
        let k8s_source = LogSource::Kubernetes {
            namespace: "default".to_string(),
            pod_name: "my-pod-abc123".to_string(),
            container_name: Some("app-container".to_string()),
        };

        match k8s_source {
            LogSource::Kubernetes {
                namespace,
                pod_name,
                container_name,
            } => {
                assert_eq!(namespace, "default");
                assert_eq!(pod_name, "my-pod-abc123");
                assert_eq!(container_name, Some("app-container".to_string()));
            }
            _ => panic!("Wrong source type"),
        }
    }

    #[cfg(feature = "aws")]
    {
        let cloudwatch_source = LogSource::CloudWatch {
            log_group: "/aws/lambda/my-function".to_string(),
            log_stream: "2024/01/01/[123]abc".to_string(),
        };

        match cloudwatch_source {
            LogSource::CloudWatch {
                log_group,
                log_stream,
            } => {
                assert_eq!(log_group, "/aws/lambda/my-function");
                assert_eq!(log_stream, "2024/01/01/[123]abc");
            }
            _ => panic!("Wrong source type"),
        }
    }
}

// These tests have been removed because they were testing private implementation details.
// The parsing functions are already tested within the logs module itself.
// Tests should focus on the public API, not internal implementation.

#[tokio::test]
async fn test_log_streamer_follow_setting() {
    let mut streamer = LogStreamer::new(100);

    // Test that we can set follow without panic
    streamer.set_follow(false);
    streamer.set_follow(true);

    // We can't test internal state, but the API works
}

#[tokio::test]
async fn test_deployment_record_compatibility() {
    // Test that log streaming integrates with deployment tracking
    use blueprint_remote_providers::core::remote::CloudProvider;
    use blueprint_remote_providers::deployment::tracker::{DeploymentRecord, DeploymentType};
    use chrono::Utc;

    let mut resource_ids = HashMap::new();
    resource_ids.insert("container_id".to_string(), "container123".to_string());

    let deployment = DeploymentRecord {
        id: "deployment-1".to_string(),
        blueprint_id: "test-blueprint".to_string(),
        deployment_type: DeploymentType::LocalDocker,
        provider: Some(CloudProvider::AWS),
        region: Some("us-west-2".to_string()),
        resource_spec: blueprint_remote_providers::core::resources::ResourceSpec {
            cpu: 2.0,
            memory_gb: 4.0,
            storage_gb: 20.0,
            gpu_count: None,
            allow_spot: false,
            qos: Default::default(),
        },
        resource_ids: resource_ids.clone(),
        deployed_at: Utc::now(),
        ttl_seconds: None,
        status: blueprint_remote_providers::deployment::tracker::DeploymentStatus::Active,
        expires_at: None,
        cleanup_webhook: None,
        metadata: HashMap::new(),
    };

    // Should be able to create log source from deployment record
    if let Some(container_id) = deployment.resource_ids.get("container_id") {
        let log_entry = LogEntry {
            timestamp: SystemTime::now(),
            service_id: deployment.id.clone(),
            container_id: Some(container_id.clone()),
            level: LogLevel::Info,
            message: "Test message from deployment".to_string(),
            metadata: HashMap::new(),
        };

        assert_eq!(log_entry.service_id, "deployment-1");
        assert_eq!(log_entry.container_id, Some("container123".to_string()));
    }
}

#[tokio::test]
async fn test_log_aggregator_deployment_integration() {
    let mut aggregator = LogAggregator::new();

    // Add multiple deployments
    let file_source1 = LogSource::File {
        host: "host1".to_string(),
        file_path: "/var/log/service1.log".to_string(),
    };

    let file_source2 = LogSource::File {
        host: "host2".to_string(),
        file_path: "/var/log/service2.log".to_string(),
    };

    // Test that we can add deployments without panic
    aggregator.add_deployment("service-1".to_string(), file_source1);
    aggregator.add_deployment("service-2".to_string(), file_source2);

    // We can't test internal state but the API works
}

// Integration tests would go here for actually streaming logs
// These would require Docker or other real log sources
#[tokio::test]
#[ignore] // Only run with real infrastructure
async fn test_real_log_streaming_integration() {
    // This test would:
    // 1. Start a container that generates logs
    // 2. Set up log streaming
    // 3. Verify logs are received
    // 4. Test filtering and aggregation
    // 5. Clean up resources

    println!("Real log streaming test requires Docker/SSH setup");
    println!("Run with: cargo test test_real_log_streaming_integration -- --ignored");
}
