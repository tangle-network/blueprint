use std::{fs, process::Command, time::Duration};

use blueprint_core::{Job, info, warn};
use blueprint_qos::proto::{
    GetBlueprintMetricsRequest, GetResourceUsageRequest, GetStatusRequest,
    qos_metrics_client::QosMetricsClient,
};
use blueprint_tangle_extra::layers::TangleLayer;
use blueprint_testing_utils::{
    Error, setup_log,
    tangle::multi_node::NodeSlot,
    tangle::{
        InputValue, OutputValue, TangleTestHarness, blueprint::create_test_blueprint,
        harness::SetupServicesOpts,
    },
};
use tokio::time::sleep;
use tonic::transport::Channel;

mod utils;

const QOS_PORT: u16 = 8085;
const OPERATOR_COUNT: usize = 1;
const INPUT_VALUE: u64 = 5;

fn cleanup_docker_containers() -> Result<(), Error> {
    let containers = ["loki", "grafana", "prometheus", "qos-test"];
    for container_name in &containers {
        info!("Cleaning up container: {}", container_name);
        let output = Command::new("docker")
            .args(["rm", "-f", container_name])
            .output()
            .map_err(|e| Error::Setup(format!("Failed to run docker command: {}", e)))?;

        if !output.status.success() {
            info!("Container {} might not exist, continuing", container_name);
        }
    }
    Ok(())
}

#[tokio::test]
async fn test_qos_integration() -> Result<(), Error> {
    setup_log();
    info!("Starting QoS Blueprint integration test");

    info!("Creating test blueprint with QoS integration");
    let (temp_dir, blueprint_dir) = create_test_blueprint();

    let harness: TangleTestHarness<()> = TangleTestHarness::setup(temp_dir).await?;

    std::env::set_current_dir(&blueprint_dir).unwrap();

    cleanup_docker_containers()?;

    info!("Setting up test service with {} operators", OPERATOR_COUNT);
    let setup_services_opts = SetupServicesOpts {
        exit_after_registration: false,
        skip_service_request: false,
        registration_args: vec![Vec::default(); OPERATOR_COUNT].try_into().unwrap(),
        request_args: Vec::default(),
    };

    let (mut test_env, service_id, blueprint_id) = harness
        .setup_services_with_options::<OPERATOR_COUNT>(setup_services_opts)
        .await?;

    let main_rs_content = fs::read_to_string(blueprint_dir.join("src/main.rs"))
        .map_err(|e| Error::Setup(format!("Failed to read main.rs: {}", e)))?;

    assert!(
        main_rs_content.contains("blueprint_qos"),
        "Blueprint should include QoS imports"
    );
    info!("Blueprint includes QoS integration");

    info!("Initializing test environment");
    test_env.initialize().await?;

    let operator_index = 0;
    info!("Using operator index {} for testing", operator_index);

    let node_handle = {
        let nodes = test_env.nodes.read().await;
        match &nodes[operator_index] {
            NodeSlot::Occupied(node) => node.clone(),
            NodeSlot::Empty => {
                return Err(Error::Setup(format!(
                    "Node {} is not initialized",
                    operator_index
                )));
            }
        }
    };

    info!("Adding square job to node handle");
    node_handle.add_job(utils::square.layer(TangleLayer)).await;

    let metrics_addr = format!("127.0.0.1:{}", QOS_PORT);

    info!("Starting BlueprintRunner with node handle");
    node_handle
        .start_runner(())
        .await
        .map_err(|e| Error::Setup(format!("Failed to start runner: {}", e)))?;

    info!(
        "BlueprintRunner started successfully - QoS service and heartbeat service should be running internally"
    );

    info!("Starting metrics server on {}", metrics_addr);
    let metrics_addr_clone = metrics_addr.clone();

    let _metrics_server_handle = tokio::spawn(async move {
        use blueprint_qos::metrics::{
            opentelemetry::OpenTelemetryConfig, provider::EnhancedMetricsProvider,
            types::MetricsConfig,
        };
        use blueprint_qos::proto::qos_metrics_server::QosMetricsServer;
        use blueprint_qos::service::QosMetricsService;
        use std::sync::Arc;

        let metrics_config = MetricsConfig {
            collection_interval_secs: 1,
            ..Default::default()
        };

        let provider =
            match EnhancedMetricsProvider::new(metrics_config, OpenTelemetryConfig::default()) {
                Ok(provider) => provider,
                Err(e) => {
                    panic!("Failed to create metrics provider: {}", e);
                }
            };

        let service = QosMetricsService::new(Arc::new(provider));

        if let Err(e) = tonic::transport::Server::builder()
            .add_service(QosMetricsServer::new(service))
            .serve(metrics_addr_clone.parse().unwrap())
            .await
        {
            panic!("Metrics server error: {}", e);
        }
    });

    sleep(Duration::from_secs(5)).await;

    info!("Submitting test job to square {}", INPUT_VALUE);
    let job = harness
        .submit_job(
            service_id,
            utils::XSQUARE_JOB_ID,
            vec![InputValue::Uint64(INPUT_VALUE)],
        )
        .await
        .map_err(|e| Error::Setup(format!("Failed to submit job: {}", e)))?;

    let results = harness
        .wait_for_job_execution(service_id, job)
        .await
        .map_err(|e| Error::Setup(format!("Failed to wait for job execution: {}", e)))?;

    harness.verify_job(
        &results,
        vec![OutputValue::Uint64(INPUT_VALUE * INPUT_VALUE)],
    );
    info!(
        "Job executed successfully: {} squared = {}",
        INPUT_VALUE,
        INPUT_VALUE * INPUT_VALUE
    );

    info!("Checking on-chain storage for heartbeat records");
    let mut found_heartbeat_on_chain = false;

    let client = harness.client().clone();

    info!("Checking latest block for heartbeat-related events");

    if let Ok(latest_block) = client.rpc_client.blocks().at_latest().await {
        if let Ok(events) = latest_block.events().await {
            for event in events.iter() {
                let event_str = format!("{:?}", event);
                if event_str.contains("heartbeat") || event_str.contains("Heartbeat") {
                    info!("Found heartbeat event in block");
                    found_heartbeat_on_chain = true;
                    break;
                }
            }
        } else {
            panic!("Could not retrieve events from the latest block");
        }

        if !found_heartbeat_on_chain {
            panic!("No heartbeat events found on-chain in the latest block");
        }
    } else {
        panic!("Failed to get latest block from the chain");
    }

    sleep(Duration::from_secs(3)).await;
    verify_qos_metrics(service_id, blueprint_id, metrics_addr.clone()).await;

    info!("QoS Blueprint integration test completed successfully");
    Ok(())
}

// Verify QoS metrics via gRPC API
async fn verify_qos_metrics(service_id: u64, blueprint_id: u64, metrics_addr: String) {
    let mut client_result: Option<QosMetricsClient<Channel>> = None;
    let qos_addr = metrics_addr;
    let max_retries = 10;
    let base_wait_ms = 500;

    info!("Testing QoS metrics API");

    let port_str = QOS_PORT.to_string();
    let port = qos_addr.split(':').nth(1).unwrap_or(&port_str);
    if let Ok(output) = Command::new("nc").args(["-z", "127.0.0.1", port]).output() {
        if !output.status.success() {
            warn!("Port {} does not appear to be open yet", port);
        }
    }

    for attempt in 1..=max_retries {
        match utils::connect_to_qos_metrics(&qos_addr).await {
            Ok(client) => {
                client_result = Some(client);
                info!("Connected to QoS metrics service");
                break;
            }
            Err(e) => {
                if attempt == max_retries {
                    warn!("Failed to connect after {} attempts", max_retries);
                    panic!("Failed to connect to QoS metrics service: {}", e)
                } else {
                    let wait_time = base_wait_ms * 2u64.pow(attempt - 1);
                    sleep(Duration::from_millis(wait_time)).await;
                }
            }
        }
    }

    if let Some(mut client) = client_result {
        if let Ok(response) = client
            .get_status(GetStatusRequest {
                service_id,
                blueprint_id,
            })
            .await
        {
            let resp = response.into_inner();
            info!("Status: code={}, uptime={}s", resp.status_code, resp.uptime);
        }

        if let Ok(response) = client
            .get_resource_usage(GetResourceUsageRequest {
                service_id,
                blueprint_id,
            })
            .await
        {
            let resp = response.into_inner();
            info!(
                "Resources: CPU={}%, Memory={}B, Disk={}B",
                resp.cpu_usage, resp.memory_usage, resp.disk_usage
            );
        }

        if let Ok(response) = client
            .get_blueprint_metrics(GetBlueprintMetricsRequest {
                service_id,
                blueprint_id,
            })
            .await
        {
            let resp = response.into_inner();
            if resp.custom_metrics.is_empty() {
                info!("No blueprint-specific metrics available");
            } else {
                for (key, value) in resp.custom_metrics {
                    info!("Metric {}: {}", key, value);
                }
            }
        }
        info!("QoS metrics API check completed");
    } else {
        panic!("Could not connect to metrics service");
    }
}
