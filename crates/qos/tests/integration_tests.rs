use blueprint_qos::{
    QoSConfig, QoSService, QoSServiceBuilder, default_qos_config,
    error::Error as QosError,
    heartbeat::{HeartbeatConsumer, HeartbeatStatus},
    logging::{GrafanaConfig, LokiConfig},
    metrics::MetricsConfig,
};
use std::future::Future;
use std::sync::Arc;

// Mock HeartbeatConsumer for testing purposes
#[derive(Clone, Debug)]
struct MockHeartbeatConsumer;

impl HeartbeatConsumer for MockHeartbeatConsumer {
    fn send_heartbeat(
        &self,
        _status: &HeartbeatStatus,
    ) -> impl Future<Output = Result<(), QosError>> + Send {
        async { Ok(()) }
    }
}

// Helper function to check if service components exist using public methods
fn check_service_components(
    service: &QoSService<MockHeartbeatConsumer>,
    config: &QoSConfig,
) -> (bool, bool, bool) {
    let has_metrics = service.metrics_provider().is_some();
    let has_grafana = config.grafana.is_some();
    let has_heartbeat = config.heartbeat.is_some();

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
            panic!("Expected Other error for missing consumer");
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

        let config = QoSConfig::default();
        let (_, has_metrics, has_grafana) = check_service_components(&service, &config);
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
            .with_config(qos_components_config.clone())
            .build()
            .await;

        assert!(
            service_result.is_ok(),
            "Service build failed: {:?}",
            service_result.err()
        );
        let service = service_result.unwrap();

        let (_, has_metrics, has_grafana) =
            check_service_components(&service, &qos_components_config);
        assert!(has_metrics, "Metrics service should be Some");
        assert!(has_grafana, "Grafana client should be Some");
    }

    #[tokio::test]
    async fn test_qos_service_builder_individual_component_configs() {
        let consumer = Arc::new(MockHeartbeatConsumer);

        // Test with Metrics enabled
        let service_metrics_res = QoSServiceBuilder::new()
            .with_heartbeat_consumer(consumer.clone())
            .with_metrics_config(MetricsConfig::default())
            .build()
            .await;
        assert!(
            service_metrics_res.is_ok(),
            "Metrics service build failed: {}",
            service_metrics_res.err().unwrap()
        );
        let service_metrics = service_metrics_res.unwrap();
        let metrics_config = QoSConfig {
            metrics: Some(MetricsConfig::default()),
            ..QoSConfig::default()
        };
        let (_, has_metrics_metrics, _) =
            check_service_components(&service_metrics, &metrics_config);
        assert!(has_metrics_metrics);

        // Test with Grafana enabled
        let service_grafana_res = QoSServiceBuilder::new()
            .with_heartbeat_consumer(consumer.clone())
            .with_grafana_config(GrafanaConfig::default())
            .build()
            .await;
        assert!(
            service_grafana_res.is_ok(),
            "Grafana service build failed: {}",
            service_grafana_res.err().unwrap()
        );
        let service_grafana = service_grafana_res.unwrap();
        let grafana_config = QoSConfig {
            grafana: Some(GrafanaConfig::default()),
            ..QoSConfig::default()
        };
        let (_, _, has_grafana_gf) = check_service_components(&service_grafana, &grafana_config);
        assert!(has_grafana_gf);

        // Test with Loki enabled
        let service_loki_res = QoSServiceBuilder::new()
            .with_heartbeat_consumer(consumer.clone())
            .with_loki_config(LokiConfig::default())
            .build()
            .await;
        assert!(
            service_loki_res.is_ok(),
            "Loki service build failed: {}",
            service_loki_res.err().unwrap()
        );
    }
}
