use std::sync::Arc;
use std::time::{Duration, Instant};

use alloy_primitives::Bytes;
use alloy_sol_types::SolValue;
use anyhow::{Context, Result, ensure};
use blueprint_anvil_testing_utils::{
    BlueprintHarness, SeededTangleEvmTestnet, missing_tnt_core_artifacts,
};
use blueprint_core::{Job, info, warn};
use blueprint_qos::heartbeat::HeartbeatConfig;
use blueprint_qos::logging::GrafanaConfig;
use blueprint_qos::logging::grafana::CreateDataSourceRequest;
use blueprint_qos::servers::common::DockerManager;
use blueprint_qos::service_builder::QoSServiceBuilder;
use blueprint_qos::unified_service::QoSService;
use blueprint_qos::{GrafanaServerConfig, LokiServerConfig, PrometheusServerConfig, QoSConfig};
use blueprint_router::Router;
use blueprint_tangle_evm_extra::layers::TangleEvmLayer;
use prometheus::{IntGauge, Opts, Registry};
use tokio::process::Command;
use tokio::time::sleep;

mod utils;
use utils::{MockHeartbeatConsumer, XSQUARE_JOB_ID, square};

const GRAFANA_PORT: u16 = 3001;
const INPUT_VALUE: u64 = 5;
const TOTAL_JOBS_TO_RUN: u64 = 10;
const JOB_INTERVAL_MS: u64 = 2_000;
const PROMETHEUS_BLUEPRINT_UID: &str = "prometheus_blueprint_default";
const LOKI_BLUEPRINT_UID: &str = "loki_blueprint_default";
const CUSTOM_NETWORK_NAME: &str = "blueprint-metrics-network";
const TEST_GRAFANA_CONTAINER_NAME: &str = "blueprint-grafana";
const TEST_LOKI_CONTAINER_NAME: &str = "blueprint-loki";
const TEST_PROMETHEUS_CONTAINER_NAME: &str = "blueprint-test-prometheus";

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_qos_metrics_demo() -> Result<()> {
    init_tracing();
    run_anvil_test("qos_metrics_demo", async {
        cleanup_docker_containers().await?;
        let harness = match BlueprintHarness::builder(router())
            .poll_interval(Duration::from_millis(50))
            .spawn()
            .await
        {
            Ok(harness) => harness,
            Err(err) => {
                if missing_tnt_core_artifacts(&err) {
                    eprintln!("Skipping test_qos_metrics_demo: {err}");
                    return Ok(());
                }
                return Err(err);
            }
        };

        let env = harness.environment().clone();
        let deployment = harness.deployment();
        let service_id = harness.service_id();
        let blueprint_id = harness.blueprint_id();

        let mut qos_config = demo_qos_config(service_id, blueprint_id, deployment);
        ensure_network().await?;
        qos_config.docker_network = Some(CUSTOM_NETWORK_NAME.to_string());
        qos_config.manage_servers = true;

        let heartbeat_consumer = Arc::new(MockHeartbeatConsumer::new());
        let qos_service = Arc::new(
            QoSServiceBuilder::new()
                .with_config(qos_config.clone())
                .with_http_rpc_endpoint(env.http_rpc_endpoint.to_string())
                .with_keystore_uri(env.keystore_uri.clone())
                .with_status_registry_address(deployment.status_registry_contract)
                .with_heartbeat_consumer(Arc::clone(&heartbeat_consumer))
                .build()
                .await
                .context("failed to build QoS service")?,
        );

        if let Some(hb) = qos_service.heartbeat_service() {
            hb.start_heartbeat()
                .await
                .context("failed to start heartbeat service")?;
        }

        qos_service.debug_server_status();
        prepare_grafana(&qos_service, &qos_config).await;

        let otel_job_counter = qos_service
            .provider()
            .context("metrics provider should exist")?
            .get_otel_job_executions_counter();

        info!(
            "Grafana dashboard available at http://127.0.0.1:{}, default admin/admin",
            GRAFANA_PORT
        );

        let registry = Registry::new();
        let job_executions = create_gauge(
            &registry,
            "test_blueprint_job_executions",
            "Job executions for test blueprint",
            blueprint_id,
            service_id,
        );
        let job_success = create_gauge(
            &registry,
            "test_blueprint_job_success",
            "Successful job executions",
            blueprint_id,
            service_id,
        );
        let job_latency = create_gauge(
            &registry,
            "test_blueprint_job_latency_ms",
            "Job execution latency in milliseconds",
            blueprint_id,
            service_id,
        );

        let mut jobs_completed = 0;
        let start_time = Instant::now();
        while jobs_completed < TOTAL_JOBS_TO_RUN {
            let payload = INPUT_VALUE.abi_encode();
            info!(
                "Submitting job #{}, squaring {}",
                jobs_completed + 1,
                INPUT_VALUE
            );
            let submission = harness
                .submit_job(XSQUARE_JOB_ID, Bytes::from(payload))
                .await
                .context("failed to submit demo job")?;

            let job_start = Instant::now();
            let output = harness
                .wait_for_job_result(submission)
                .await
                .context("failed to read job result")?;
            let squared = u64::abi_decode(&output).context("failed to decode job result")?;
            ensure!(
                squared == INPUT_VALUE * INPUT_VALUE,
                "expected {} got {}",
                INPUT_VALUE * INPUT_VALUE,
                squared
            );

            jobs_completed += 1;
            let latency = i64::try_from(job_start.elapsed().as_millis()).unwrap_or_default();
            job_executions.set(jobs_completed as i64);
            job_success.set(jobs_completed as i64);
            job_latency.set(latency);

            qos_service.record_job_execution(
                u64::from(XSQUARE_JOB_ID),
                latency as f64,
                service_id,
                blueprint_id,
            );
            otel_job_counter.add(
                1,
                &[
                    opentelemetry::KeyValue::new("service_id", service_id.to_string()),
                    opentelemetry::KeyValue::new("blueprint_id", blueprint_id.to_string()),
                ],
            );
            sleep(Duration::from_millis(JOB_INTERVAL_MS)).await;
        }

        let total_time = start_time.elapsed();
        info!("QoS metrics demo completed");
        info!("Jobs completed: {}", jobs_completed);
        info!("Total time: {:.2}s", total_time.as_secs_f64());
        info!(
            "Average job latency: {:.2} ms",
            total_time.as_millis() as f64 / jobs_completed as f64
        );

        info!("Keeping servers alive for 30s for dashboard review");
        sleep(Duration::from_secs(30)).await;

        if let Some(hb) = qos_service.heartbeat_service() {
            hb.stop_heartbeat()
                .await
                .context("failed to stop heartbeat service")?;
        }
        qos_service
            .shutdown()
            .context("failed to shut down QoS service")?;
        cleanup_docker_containers().await?;
        harness.shutdown().await;
        Ok(())
    })
    .await
}

fn router() -> Router<()> {
    Router::new().route(XSQUARE_JOB_ID, square.layer(TangleEvmLayer))
}

fn demo_qos_config(
    service_id: u64,
    blueprint_id: u64,
    deployment: &SeededTangleEvmTestnet,
) -> QoSConfig {
    let mut config = QoSConfig::default();
    config.service_id = Some(service_id);
    config.blueprint_id = Some(blueprint_id);
    config.metrics = default_metrics_config();
    config.heartbeat = Some(HeartbeatConfig {
        interval_secs: 5,
        jitter_percent: 5,
        service_id,
        blueprint_id,
        max_missed_heartbeats: 3,
        status_registry_address: deployment.status_registry_contract,
    });
    config.grafana_server = Some(GrafanaServerConfig {
        port: GRAFANA_PORT,
        container_name: TEST_GRAFANA_CONTAINER_NAME.to_string(),
        health_check_timeout_secs: 180,
        ..Default::default()
    });

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let prometheus_config_path = format!("{}/tests/config/prometheus.yml", manifest_dir);
    config.prometheus_server = Some(PrometheusServerConfig {
        use_docker: true,
        docker_container_name: TEST_PROMETHEUS_CONTAINER_NAME.to_string(),
        config_path: Some(prometheus_config_path),
        ..Default::default()
    });

    let loki_config_path = format!("{}/config/loki-config.yaml", manifest_dir);
    config.loki_server = Some(LokiServerConfig {
        config_path: Some(loki_config_path),
        container_name: TEST_LOKI_CONTAINER_NAME.to_string(),
        ..Default::default()
    });

    let prometheus_datasource_url = format!("http://{}:9090", TEST_PROMETHEUS_CONTAINER_NAME);
    config.grafana = Some(GrafanaConfig {
        url: format!("http://localhost:{}", GRAFANA_PORT),
        admin_user: Some("admin".into()),
        admin_password: Some("admin".into()),
        prometheus_datasource_url: Some(prometheus_datasource_url),
        ..Default::default()
    });

    config.loki = Some(blueprint_qos::logging::loki::LokiConfig {
        url: "http://blueprint-loki:3100".to_string(),
        username: Some("test-tenant".to_string()),
        ..Default::default()
    });

    config
}

fn default_metrics_config() -> Option<blueprint_qos::metrics::types::MetricsConfig> {
    let mut metrics = blueprint_qos::metrics::types::MetricsConfig::default();
    metrics.collection_interval_secs = 1;
    metrics.max_history = 20;
    Some(metrics)
}

async fn ensure_network() -> Result<()> {
    let docker = DockerManager::new().context("docker unavailable")?;
    info!(
        "Ensuring custom Docker network '{}' exists",
        CUSTOM_NETWORK_NAME
    );
    match docker.create_network(CUSTOM_NETWORK_NAME).await {
        Ok(()) => info!("Docker network '{}' created", CUSTOM_NETWORK_NAME),
        Err(err) if err.to_string().contains("already exists") => {
            info!("Docker network '{}' already exists", CUSTOM_NETWORK_NAME);
        }
        Err(err) => return Err(err).context("failed to create docker network"),
    }
    Ok(())
}

async fn prepare_grafana(qos_service: &Arc<QoSService<MockHeartbeatConsumer>>, config: &QoSConfig) {
    if let Some(url) = qos_service.grafana_server_url() {
        info!("Grafana reachable at {}", url);
    }
    if let Some(client) = qos_service.grafana_client() {
        info!("Configuring Grafana datasources");
        let loki_request = CreateDataSourceRequest {
            name: "Blueprint Loki (Test)".to_string(),
            ds_type: "loki".to_string(),
            url: "http://localhost:3100".to_string(),
            access: "proxy".to_string(),
            uid: Some(LOKI_BLUEPRINT_UID.to_string()),
            is_default: Some(false),
            json_data: Some(serde_json::json!({"maxLines": 1000})),
        };
        if let Err(err) = client.create_or_update_datasource(loki_request).await {
            warn!("Failed to create Loki datasource: {err}");
        }

        let prometheus_port = config
            .prometheus_server
            .as_ref()
            .map(|p| p.port)
            .unwrap_or(9090);
        let prometheus_host = config
            .prometheus_server
            .as_ref()
            .map(|p| {
                if p.use_docker {
                    p.docker_container_name.clone()
                } else {
                    "127.0.0.1".to_string()
                }
            })
            .unwrap_or_else(|| "127.0.0.1".to_string());
        let prometheus_request = CreateDataSourceRequest {
            name: "Blueprint Prometheus (Test)".to_string(),
            ds_type: "prometheus".to_string(),
            url: format!("http://{}:{}", prometheus_host, prometheus_port),
            access: "proxy".to_string(),
            uid: Some(PROMETHEUS_BLUEPRINT_UID.to_string()),
            is_default: Some(false),
            json_data: None,
        };
        if let Err(err) = client.create_or_update_datasource(prometheus_request).await {
            warn!("Failed to create Prometheus datasource: {err}");
        }
    } else {
        warn!("No Grafana client available; skipping datasource setup");
    }
}

fn create_gauge(
    registry: &Registry,
    name: &str,
    help: &str,
    blueprint_id: u64,
    service_id: u64,
) -> IntGauge {
    let opts = Opts::new(name, help)
        .const_label("service_id", service_id.to_string())
        .const_label("blueprint_id", blueprint_id.to_string());
    let gauge =
        IntGauge::with_opts(opts).unwrap_or_else(|err| panic!("failed to create gauge: {err}"));
    if let Err(err) = registry.register(Box::new(gauge.clone())) {
        warn!("failed to register gauge {name}: {err}");
    }
    gauge
}

async fn cleanup_docker_containers() -> Result<()> {
    info!("Cleaning up metrics demo containers");
    for container in [
        TEST_GRAFANA_CONTAINER_NAME,
        TEST_LOKI_CONTAINER_NAME,
        TEST_PROMETHEUS_CONTAINER_NAME,
    ] {
        let _ = Command::new("docker")
            .args(["rm", "-f", container])
            .output()
            .await;
    }

    let _ = Command::new("docker")
        .args(["network", "rm", CUSTOM_NETWORK_NAME])
        .output()
        .await;
    sleep(Duration::from_secs(2)).await;
    Ok(())
}

fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init();
}

async fn run_anvil_test<F>(name: &str, fut: F) -> Result<()>
where
    F: std::future::Future<Output = Result<()>>,
{
    tokio::time::timeout(Duration::from_secs(1_800), fut)
        .await
        .with_context(|| format!("{name} timed out"))?
}
