use blueprint_core::info;
use std::sync::Arc;

use crate::error::Result;
use crate::metrics::opentelemetry::OpenTelemetryConfig;
use crate::metrics::provider::EnhancedMetricsProvider;
use crate::metrics::types::MetricsConfig;

/// Service responsible for metrics collection, processing, and exposure.
///
/// The `MetricsService` orchestrates the metrics collection pipeline, managing the
/// lifecycle of the underlying metrics provider. It serves as the main entry point
/// for recording application metrics (like job execution statistics) and provides
/// access to the configured metrics infrastructure. The service works with Prometheus
/// and OpenTelemetry to provide comprehensive observability for Blueprint services.
#[derive(Clone)]
pub struct MetricsService {
    /// Metrics provider
    provider: Arc<EnhancedMetricsProvider>,

    /// Configuration
    #[allow(dead_code)]
    config: MetricsConfig,
}

impl MetricsService {
    /// Creates a new metrics service with default OpenTelemetry configuration.
    ///
    /// Initializes the metrics collection infrastructure with the specified configuration,
    /// creating an `EnhancedMetricsProvider` with default OpenTelemetry settings. This
    /// provider will collect system metrics, application metrics, and expose them through
    /// Prometheus and OpenTelemetry.
    ///
    /// # Parameters
    /// * `config` - Configuration for metrics collection, retention and exposure
    ///
    /// # Errors
    /// Returns an error if the metrics provider initialization fails, which could occur
    /// due to invalid configuration or resource allocation issues
    pub fn new(config: MetricsConfig) -> Result<Self> {
        let otel_config = OpenTelemetryConfig::default();
        let provider = Arc::new(EnhancedMetricsProvider::new(config.clone(), &otel_config)?);

        Ok(Self { provider, config })
    }

    /// Creates a new metrics service with custom OpenTelemetry configuration.
    ///
    /// Similar to `new()`, but allows for customized OpenTelemetry settings, enabling
    /// fine-tuning of the tracing and metrics export behavior. Use this constructor when
    /// you need to customize the OpenTelemetry pipeline for advanced observability requirements.
    ///
    /// # Parameters
    /// * `config` - Configuration for metrics collection, retention and exposure
    /// * `otel_config` - Custom OpenTelemetry configuration for trace and metrics export
    ///
    /// # Errors
    /// Returns an error if the metrics provider initialization fails, which could occur
    /// due to invalid configuration or resource allocation issues
    pub fn with_otel_config(
        config: MetricsConfig,
        otel_config: &OpenTelemetryConfig,
    ) -> Result<Self> {
        let provider = Arc::new(EnhancedMetricsProvider::new(config.clone(), otel_config)?);

        Ok(Self { provider, config })
    }

    /// Returns a reference to the underlying metrics provider.
    ///
    /// Provides access to the `EnhancedMetricsProvider` which handles the actual collection,
    /// storage, and exposure of metrics. This can be used for advanced metrics operations
    /// not directly exposed by the `MetricsService` interface.
    #[must_use]
    pub fn provider(&self) -> Arc<EnhancedMetricsProvider> {
        self.provider.clone()
    }

    /// Returns a clone of the OpenTelemetry job executions counter.
    ///
    /// This counter tracks the total number of job executions across the Blueprint service.
    /// It can be used directly to increment execution counts from components that have
    /// access to the metrics service but not the full provider.
    #[must_use]
    pub fn get_otel_job_executions_counter(&self) -> opentelemetry::metrics::Counter<u64> {
        self.provider.get_otel_job_executions_counter()
    }

    /// Records metrics for a successful job execution.
    ///
    /// Updates both Prometheus and OpenTelemetry metrics with information about a completed job.
    /// This information is used to track job throughput, execution time distributions, and
    /// success rates for the specified job, service, and blueprint IDs.
    ///
    /// # Parameters
    /// * `job_id` - Unique identifier for the executed job
    /// * `execution_time` - Duration of the job execution in seconds
    /// * `service_id` - Identifier for the service that executed the job
    /// * `blueprint_id` - Identifier for the blueprint that executed the job
    pub fn record_job_execution(
        &self,
        job_id: u64,
        execution_time: f64,
        service_id: u64,
        blueprint_id: u64,
    ) {
        self.provider
            .record_job_execution(job_id, execution_time, service_id, blueprint_id);
    }

    /// Records metrics for a failed job execution.
    ///
    /// Updates error counters and metrics when a job fails, categorizing the error by type.
    /// This allows for tracking error rates and the distribution of different failure modes
    /// across jobs to help with debugging and reliability improvements.
    ///
    /// # Parameters
    /// * `job_id` - Unique identifier for the failed job
    /// * `error_type` - Classification of the error that occurred
    pub fn record_job_error(&self, job_id: u64, error_type: &str) {
        self.provider.record_job_error(job_id, error_type);
    }
}

/// Runs a standalone metrics server with the given configuration.
///
/// This function initializes the metrics collection infrastructure and starts a
/// metrics server that exposes collected metrics via HTTP endpoints compatible with
/// Prometheus scraping. It also initializes the background tasks for collecting
/// system and application metrics at regular intervals.
///
/// # Parameters
/// * `config` - Configuration for metrics collection, retention, and server settings
///
/// # Returns
/// A reference to the initialized and started metrics provider on success
///
/// # Errors
/// Returns an error if the metrics provider initialization or server startup fails
pub async fn run_metrics_server(config: MetricsConfig) -> Result<Arc<EnhancedMetricsProvider>> {
    let otel_config = OpenTelemetryConfig::default();
    let provider = Arc::new(EnhancedMetricsProvider::new(config, &otel_config)?);

    // Start the metrics collection
    provider.clone().start_collection().await?;

    info!("Started metrics server");

    Ok(provider)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::servers::prometheus::PrometheusServerConfig;

    /// Tests that a new `MetricsService` can be created with a valid configuration.
    ///
    /// ```
    /// MetricsConfig -> MetricsService
    /// ```
    ///
    /// Expected outcome: `MetricsService` is created with the provided config
    #[test]
    fn test_metrics_service_creation() {
        let config = MetricsConfig {
            prometheus_server: Some(PrometheusServerConfig::default()),
            service_id: 42,
            blueprint_id: 24,
            collection_interval_secs: 60,
            max_history: 100,
        };

        let service = MetricsService::new(config.clone());
        assert!(service.is_ok());

        let service = service.unwrap();
        assert!(std::sync::Arc::strong_count(&service.provider()) >= 1);
    }

    /// Tests that a new `MetricsService` can be created with custom OpenTelemetry configuration.
    ///
    /// ```
    /// MetricsConfig + OpenTelemetryConfig -> MetricsService
    /// ```
    ///
    /// Expected outcome: `MetricsService` is created with the provided configs
    #[test]
    fn test_metrics_service_with_otel_config() {
        let config = MetricsConfig {
            prometheus_server: Some(PrometheusServerConfig::default()),
            service_id: 42,
            blueprint_id: 24,
            collection_interval_secs: 60,
            max_history: 100,
        };

        let otel_config = OpenTelemetryConfig::default();

        let service = MetricsService::with_otel_config(config.clone(), &otel_config);
        assert!(service.is_ok());

        let service = service.unwrap();
        assert!(std::sync::Arc::strong_count(&service.provider()) >= 1);
    }

    /// Tests that the `MetricsService` can record job executions.
    ///
    /// ```
    /// MetricsService.record_job_execution() -> Job execution recorded
    /// ```
    ///
    /// Expected outcome: Job execution is recorded in the metrics provider
    #[test]
    fn test_metrics_service_record_job_execution() {
        let config = MetricsConfig {
            prometheus_server: Some(PrometheusServerConfig::default()),
            service_id: 42,
            blueprint_id: 24,
            collection_interval_secs: 60,
            max_history: 100,
        };

        let service = MetricsService::new(config.clone()).unwrap();

        service.record_job_execution(1, 0.5, 42, 24);
    }

    /// Tests that the `MetricsService` can record job errors.
    ///
    /// ```
    /// MetricsService.record_job_error() -> Job error recorded
    /// ```
    ///
    /// Expected outcome: Job error is recorded in the metrics provider
    #[test]
    fn test_metrics_service_record_job_error() {
        let config = MetricsConfig {
            prometheus_server: Some(PrometheusServerConfig::default()),
            service_id: 42,
            blueprint_id: 24,
            collection_interval_secs: 60,
            max_history: 100,
        };

        let service = MetricsService::new(config.clone()).unwrap();

        service.record_job_error(1, "test_error");
    }
}
