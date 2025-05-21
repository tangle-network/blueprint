use async_trait::async_trait;
use blueprint_qos::{
    QoSConfig,
    QoSService, // Added for direct access to service fields if public
    QoSServiceBuilder,
    default_qos_config,
    error::Error as QosError,
    heartbeat::{Heartbeat, HeartbeatConfig, HeartbeatConsumer},
    logging::{GrafanaConfig, LokiConfig},
    metrics::MetricsConfig,
};
use std::sync::Arc;

// Mock HeartbeatConsumer for testing purposes
#[derive(Clone, Debug)]
struct MockHeartbeatConsumer;

#[async_trait]
impl HeartbeatConsumer for MockHeartbeatConsumer {
    async fn consume_heartbeat(&self, _heartbeat: Heartbeat) -> Result<(), QosError> {
        Ok(())
    }
}

// Helper function to access internal fields if they are private
// This is a placeholder; actual access depends on QoSService's design.
// If fields are public, this function is not strictly needed for these tests.
fn check_service_components(service: &QoSService<MockHeartbeatConsumer>) -> (bool, bool, bool) {
    // Assuming direct field access for now. If fields are private,
    // this would need to use public accessor methods if they exist.
    let has_heartbeat = service.heartbeat_service.is_some();
    let has_metrics = service.metrics_service.is_some();
    let has_grafana = service.grafana_client.is_some();
    (has_heartbeat, has_metrics, has_grafana)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qos_config_default() {
        let config = QoSConfig::default();
        assert!(config.heartbeat.is_none());
        assert!(config.metrics.is_none());
        assert!(config.loki.is_none());
        assert!(config.grafana.is_none());
    }

    #[test]
    fn test_default_qos_config_initialization() {
        let config = default_qos_config();
        assert!(config.heartbeat.is_some());
        assert!(config.metrics.is_some());
        assert!(config.loki.is_some());
        assert!(config.grafana.is_some());
    }

    #[tokio::test]
    async fn test_qos_service_builder_requires_consumer() {
        let builder = QoSServiceBuilder::<MockHeartbeatConsumer>::new();
        let result = builder.build().await;
        assert!(result.is_err());
        if let Err(QosError::Other(msg)) = result {
            assert_eq!(msg, "Heartbeat consumer is required");
        } else {
            panic!(
                "Expected Other error for missing consumer, got {:?}",
                result
            );
        }
    }

    #[tokio::test]
    async fn test_qos_service_build_with_default_config() {
        let consumer = Arc::new(MockHeartbeatConsumer);
        let service_result = QoSServiceBuilder::new()
            .with_heartbeat_consumer(consumer)
            .build()
            .await;
        assert!(
            service_result.is_ok(),
            "Service build failed: {:?}",
            service_result.err()
        );
        let service = service_result.unwrap();

        let (has_heartbeat, has_metrics, has_grafana) = check_service_components(&service);
        assert!(
            !has_heartbeat,
            "Heartbeat service should be None with default QoSConfig"
        );
        assert!(
            !has_metrics,
            "Metrics service should be None with default QoSConfig"
        );
        assert!(
            !has_grafana,
            "Grafana client should be None with default QoSConfig"
        );
    }

    #[tokio::test]
    async fn test_qos_service_build_with_full_default_components_config() {
        let consumer = Arc::new(MockHeartbeatConsumer);
        let qos_components_config = default_qos_config();

        let service_result = QoSServiceBuilder::new()
            .with_heartbeat_consumer(consumer.clone())
            .with_config(qos_components_config)
            .build()
            .await;

        assert!(
            service_result.is_ok(),
            "Service build failed: {:?}",
            service_result.err()
        );
        let service = service_result.unwrap();

        let (has_heartbeat, has_metrics, has_grafana) = check_service_components(&service);
        assert!(has_heartbeat, "Heartbeat service should be Some");
        assert!(has_metrics, "Metrics service should be Some");
        assert!(has_grafana, "Grafana client should be Some");
    }

    #[tokio::test]
    async fn test_qos_service_builder_individual_component_configs() {
        let consumer = Arc::new(MockHeartbeatConsumer);

        // Test with Heartbeat enabled
        let service_hb_res = QoSServiceBuilder::new()
            .with_heartbeat_consumer(consumer.clone())
            .with_heartbeat_config(HeartbeatConfig::default())
            .build()
            .await;
        assert!(
            service_hb_res.is_ok(),
            "HB service build failed: {:?}",
            service_hb_res.err()
        );
        let service_hb = service_hb_res.unwrap();
        let (has_heartbeat_hb, has_metrics_hb, _) = check_service_components(&service_hb);
        assert!(has_heartbeat_hb);
        assert!(!has_metrics_hb);

        // Test with Metrics enabled
        let service_metrics_res = QoSServiceBuilder::new()
            .with_heartbeat_consumer(consumer.clone())
            .with_metrics_config(MetricsConfig::default())
            .build()
            .await;
        assert!(
            service_metrics_res.is_ok(),
            "Metrics service build failed: {:?}",
            service_metrics_res.err()
        );
        let service_metrics = service_metrics_res.unwrap();
        let (has_heartbeat_metrics, has_metrics_metrics, _) =
            check_service_components(&service_metrics);
        assert!(!has_heartbeat_metrics);
        assert!(has_metrics_metrics);

        // Test with Grafana enabled
        let service_grafana_res = QoSServiceBuilder::new()
            .with_heartbeat_consumer(consumer.clone())
            .with_grafana_config(GrafanaConfig::default())
            .build()
            .await;
        assert!(
            service_grafana_res.is_ok(),
            "Grafana service build failed: {:?}",
            service_grafana_res.err()
        );
        let service_grafana = service_grafana_res.unwrap();
        let (_, _, has_grafana_gf) = check_service_components(&service_grafana);
        assert!(has_grafana_gf);

        // Test with Loki enabled (no direct field to check, but builder should accept it)
        let service_loki_res = QoSServiceBuilder::new()
            .with_heartbeat_consumer(consumer.clone())
            .with_loki_config(LokiConfig::default())
            .build()
            .await;
        assert!(
            service_loki_res.is_ok(),
            "Loki service build failed: {:?}",
            service_loki_res.err()
        );
    }
}
