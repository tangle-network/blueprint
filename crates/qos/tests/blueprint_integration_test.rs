use std::sync::Arc;
use std::time::Duration;

use alloy_primitives::{Address, Bytes};
use alloy_provider::ProviderBuilder;
use alloy_sol_types::SolValue;
use anyhow::{Context, Result, ensure};
use blueprint_anvil_testing_utils::{
    BlueprintHarness, SeededTangleEvmTestnet, missing_tnt_core_artifacts, seed_operator_key,
};
use blueprint_core::{Job, info, warn};
use blueprint_qos::heartbeat::HeartbeatConfig;
use blueprint_qos::metrics::provider::EnhancedMetricsProvider;
use blueprint_qos::proto::{
    GetBlueprintMetricsRequest, GetResourceUsageRequest, GetStatusRequest,
    qos_metrics_client::QosMetricsClient, qos_metrics_server::QosMetricsServer,
};
use blueprint_qos::service::QosMetricsService;
use blueprint_qos::service_builder::QoSServiceBuilder;
use blueprint_qos::{QoSConfig, default_qos_config};
use blueprint_router::Router;
use blueprint_tangle_evm_extra::layers::TangleEvmLayer;
use tempfile::TempDir;
use tokio::process::Command;
use tokio::time::{sleep, timeout};
use tonic::transport::Server;

use tnt_core_bindings::bindings::r#i_operator_status_registry::IOperatorStatusRegistry;

mod utils;

use utils::{MockHeartbeatConsumer, XSQUARE_JOB_ID, connect_to_qos_metrics, square};

const INPUT_VALUE: u64 = 5;
const QOS_PORT: u16 = 18085;
const METRICS_WAIT_MS: u64 = 500;
const MAX_METRICS_RETRIES: u32 = 10;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_qos_integration_on_tangle_evm() -> Result<()> {
    init_tracing();
    run_anvil_test("qos_blueprint_integration", async {
        let harness = match BlueprintHarness::builder(router())
            .poll_interval(Duration::from_millis(50))
            .spawn()
            .await
        {
            Ok(harness) => harness,
            Err(err) => {
                if missing_tnt_core_artifacts(&err) {
                    eprintln!("Skipping test_qos_integration_on_tangle_evm: {err}");
                    return Ok(());
                }
                return Err(err);
            }
        };

        let service_id = harness.service_id();
        let blueprint_id = harness.blueprint_id();
        let env = harness.environment().clone();
        let deployment = harness.deployment();
        let operator_account = harness.client().account();

        let heartbeat_consumer = Arc::new(MockHeartbeatConsumer::new());
        let heartbeat_keystore =
            TempDir::new().context("failed to create heartbeat keystore directory")?;
        seed_operator_key(heartbeat_keystore.path())
            .context("failed to seed heartbeat keystore")?;
        let qos_config = base_qos_config(
            service_id,
            blueprint_id,
            deployment.status_registry_contract,
        );

        let qos_service = QoSServiceBuilder::new()
            .with_config(qos_config)
            .with_http_rpc_endpoint(env.http_rpc_endpoint.to_string())
            .with_keystore_uri(heartbeat_keystore.path().to_string_lossy().to_string())
            .with_status_registry_address(deployment.status_registry_contract)
            .with_heartbeat_consumer(Arc::clone(&heartbeat_consumer))
            .build()
            .await
            .context("failed to build QoS service")?;
        let heartbeat_service = qos_service
            .heartbeat_service()
            .cloned()
            .expect("heartbeat enabled in config");
        heartbeat_service
            .start_heartbeat()
            .await
            .context("failed to start heartbeat service")?;

        let metrics_provider = qos_service
            .provider()
            .context("metrics provider should be configured")?;
        let metrics_addr = format!("127.0.0.1:{QOS_PORT}");
        let grpc_handle = spawn_metrics_server(metrics_provider.clone(), metrics_addr.clone());

        let payload = INPUT_VALUE.abi_encode();
        let submission = harness
            .submit_job(XSQUARE_JOB_ID, Bytes::from(payload))
            .await
            .context("failed to submit job to harness")?;
        let output = harness
            .wait_for_job_result_with_deadline(submission, Duration::from_secs(120))
            .await
            .context("failed to read job result")?;
        let squared = u64::abi_decode(&output).context("failed to decode job output")?;
        ensure!(
            squared == INPUT_VALUE * INPUT_VALUE,
            "expected {} but received {}",
            INPUT_VALUE * INPUT_VALUE,
            squared
        );

        sleep(Duration::from_secs(5)).await;

        ensure!(
            heartbeat_consumer.heartbeat_count().await > 0,
            "no heartbeats captured locally"
        );
        verify_heartbeat_on_chain(deployment, operator_account, service_id).await?;
        verify_qos_metrics(service_id, blueprint_id, &metrics_addr).await?;

        heartbeat_service
            .stop_heartbeat()
            .await
            .context("failed to stop heartbeat service")?;
        grpc_handle.abort();
        let _ = grpc_handle.await;
        harness.shutdown().await;
        Ok(())
    })
    .await
}

fn router() -> Router<()> {
    Router::new().route(XSQUARE_JOB_ID, square.layer(TangleEvmLayer))
}

fn base_qos_config(service_id: u64, blueprint_id: u64, status_registry: Address) -> QoSConfig {
    let mut config = default_qos_config();
    config.manage_servers = false;
    config.grafana = None;
    config.loki = None;
    config.grafana_server = None;
    config.loki_server = None;
    config.prometheus_server = None;
    config.docker_network = None;
    config.service_id = Some(service_id);
    config.blueprint_id = Some(blueprint_id);
    config.metrics.as_mut().map(|m| {
        m.collection_interval_secs = 1;
        m.max_history = 10;
    });
    config.heartbeat = Some(HeartbeatConfig {
        interval_secs: 5,
        jitter_percent: 5,
        service_id,
        blueprint_id,
        max_missed_heartbeats: 3,
        status_registry_address: status_registry,
    });
    config
}

fn spawn_metrics_server(
    provider: Arc<EnhancedMetricsProvider>,
    addr: String,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let service = QosMetricsService::new(provider);
        if let Err(err) = Server::builder()
            .add_service(QosMetricsServer::new(service))
            .serve(addr.parse().expect("valid socket addr"))
            .await
        {
            warn!("metrics server exited: {err:?}");
        }
    })
}

async fn verify_qos_metrics(service_id: u64, blueprint_id: u64, addr: &str) -> Result<()> {
    info!("Connecting to QoS metrics service at {}", addr);
    if let Some((_, port)) = addr.split_once(':') {
        if let Ok(output) = Command::new("nc")
            .args(["-z", "127.0.0.1", port])
            .output()
            .await
        {
            if !output.status.success() {
                warn!("Port {} is not open yet", addr);
            }
        }
    }

    let mut client: Option<QosMetricsClient<_>> = None;
    for attempt in 1..=MAX_METRICS_RETRIES {
        match connect_to_qos_metrics(addr).await {
            Ok(conn) => {
                client = Some(conn);
                break;
            }
            Err(err) => {
                if attempt == MAX_METRICS_RETRIES {
                    return Err(err.context("failed to connect to QoS metrics service"));
                }
                sleep(Duration::from_millis(
                    METRICS_WAIT_MS * 2u64.pow(attempt - 1),
                ))
                .await;
            }
        }
    }

    let mut client = client.expect("metrics client available");
    let status = client
        .get_status(GetStatusRequest {
            service_id,
            blueprint_id,
        })
        .await
        .context("failed to fetch QoS status")?
        .into_inner();
    info!(
        "Metrics status: code={}, uptime={}s",
        status.status_code, status.uptime
    );

    let resources = client
        .get_resource_usage(GetResourceUsageRequest {
            service_id,
            blueprint_id,
        })
        .await
        .context("failed to fetch QoS resource usage")?
        .into_inner();
    info!(
        "Resource usage: cpu={}%, memory={}B, disk={}B",
        resources.cpu_usage, resources.memory_usage, resources.disk_usage
    );

    let metrics = client
        .get_blueprint_metrics(GetBlueprintMetricsRequest {
            service_id,
            blueprint_id,
        })
        .await
        .context("failed to fetch custom blueprint metrics")?
        .into_inner();
    if metrics.custom_metrics.is_empty() {
        info!("No custom blueprint metrics reported");
    } else {
        for (key, value) in metrics.custom_metrics {
            info!("Metric {} => {}", key, value);
        }
    }
    Ok(())
}

async fn verify_heartbeat_on_chain(
    deployment: &SeededTangleEvmTestnet,
    operator: Address,
    service_id: u64,
) -> Result<()> {
    info!("Querying OperatorStatusRegistry for heartbeat data");
    let provider = ProviderBuilder::new()
        .connect(deployment.http_endpoint().as_str())
        .await
        .context("failed to connect to anvil provider")?;

    let registry =
        IOperatorStatusRegistry::new(deployment.status_registry_contract.into(), provider);

    let last = registry
        .getLastHeartbeat(service_id, operator.into())
        .call()
        .await
        .context("failed to fetch last heartbeat")?;
    ensure!(!last.is_zero(), "no on-chain heartbeat recorded");
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
    timeout(Duration::from_secs(1_200), fut)
        .await
        .with_context(|| format!("{name} timed out"))?
}
