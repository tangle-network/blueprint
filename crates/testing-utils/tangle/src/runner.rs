#![allow(dead_code)]

use blueprint_contexts::tangle::TangleClientContext;
use blueprint_core::Job;
use blueprint_core_testing_utils::runner::{TestEnv, TestRunner};
use blueprint_crypto_tangle_pair_signer::TanglePairSigner;
use blueprint_keystore::backends::Backend;
use blueprint_keystore::crypto::sp_core::SpSr25519;
use blueprint_qos::heartbeat::HeartbeatConsumer;
use blueprint_qos::heartbeat::HeartbeatStatus;
use blueprint_qos::{QoSConfig, QoSService};
use blueprint_runner::BackgroundService;
use blueprint_runner::config::BlueprintEnvironment;
use blueprint_runner::config::Multiaddr;
use blueprint_runner::error::{JobCallError, RunnerError as Error};
use blueprint_runner::tangle::config::TangleConfig;
use blueprint_tangle_extra::consumer::TangleConsumer;
use blueprint_tangle_extra::producer::TangleProducer;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

pub struct TangleTestEnv<Ctx, C>
where
    C: HeartbeatConsumer + Send + Sync + 'static,
{
    pub runner: Option<TestRunner<Ctx>>,
    pub config: TangleConfig,
    pub env: BlueprintEnvironment,
    pub runner_handle: Mutex<Option<JoinHandle<Result<(), Error>>>>,
    pub qos_config: Option<QoSConfig>,
    pub qos_service: Option<Arc<QoSService<C>>>,
}

impl<Ctx, C> TangleTestEnv<Ctx, C>
where
    C: HeartbeatConsumer + Send + Sync + 'static,
{
    pub(crate) fn update_networking_config(
        &mut self,
        bootnodes: Vec<Multiaddr>,
        network_bind_port: u16,
    ) {
        self.env.bootnodes = bootnodes;
        self.env.network_bind_port = network_bind_port;
    }

    /// Set the QoS config for this test environment
    pub fn set_qos_config(&mut self, config: QoSConfig) {
        self.qos_config = Some(config);
    }
    /// Set the QoS service for this test environment
    pub fn set_qos_service(&mut self, service: Arc<QoSService<C>>) {
        self.qos_service = Some(service);
    }

    // TODO(serial): This needs to return errors. Too many chances to panic here. Not helpful.
    pub(crate) async fn set_tangle_producer_consumer(&mut self) {
        let runner = self.runner.as_mut().expect("Runner already running");
        let builder = runner.builder.take().expect("Runner already running");
        let tangle_client = self
            .env
            .tangle_client()
            .await
            .expect("Tangle node should be running");
        let producer = TangleProducer::finalized_blocks(tangle_client.rpc_client.clone())
            .await
            .expect("Failed to create producer");

        let sr25519_signer = self
            .env
            .keystore()
            .first_local::<SpSr25519>()
            .expect("key not found");
        let sr25519_pair = self
            .env
            .keystore()
            .get_secret::<SpSr25519>(&sr25519_signer)
            .expect("key not found");
        let sr25519_signer = TanglePairSigner::new(sr25519_pair.0);
        let consumer = TangleConsumer::new(tangle_client.rpc_client.clone(), sr25519_signer);
        runner.builder = Some(builder.producer(producer).consumer(consumer));
    }
}

impl<Ctx, C> Debug for TangleTestEnv<Ctx, C>
where
    C: HeartbeatConsumer + Send + Sync + 'static,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TangleTestEnv")
            .field("config", &self.config)
            .field("env", &self.env)
            .finish_non_exhaustive()
    }
}

impl<Ctx, C> TestEnv for TangleTestEnv<Ctx, C>
where
    Ctx: Clone + Send + Sync + 'static,
    C: HeartbeatConsumer + Send + Sync + 'static,
{
    type Config = TangleConfig;
    type Context = Ctx;

    fn new(config: Self::Config, env: BlueprintEnvironment) -> Result<Self, Error> {
        let runner = TestRunner::<Ctx>::new::<Self::Config>(config.clone(), env.clone());

        Ok(Self {
            runner: Some(runner),
            config,
            env,
            runner_handle: Mutex::new(None),
            qos_config: None,
            qos_service: None,
        })
    }

    fn add_job<J, T>(&mut self, job: J)
    where
        J: Job<T, Self::Context> + Send + Sync + 'static,
        T: 'static,
    {
        self.runner
            .as_mut()
            .expect("Runner already running")
            .add_job(job);
    }

    fn add_background_service<B>(&mut self, service: B)
    where
        B: BackgroundService + Send + 'static,
    {
        self.runner
            .as_mut()
            .expect("Runner already running")
            .add_background_service(service);
    }

    fn get_blueprint_config(&self) -> BlueprintEnvironment {
        self.env.clone()
    }

    async fn run_runner(&mut self, context: Self::Context) -> Result<(), Error> {
        // Spawn the runner in a background task
        let mut runner = self.runner.take().expect("Runner already running");
        if let Some(qos_service_arc) = &self.qos_service {
            runner.qos_service(qos_service_arc.clone());
        }

        let handle = tokio::spawn(async move { runner.run(context).await });

        let mut guard = self.runner_handle.lock().await;
        *guard = Some(handle);

        // Brief delay to allow for startup
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Just check if it failed immediately
        let handle = guard.take().expect("was just set");
        if !handle.is_finished() {
            // Put the handle back since the runner is still running
            *guard = Some(handle);
            blueprint_core::info!("Runner started successfully");
            return Ok(());
        }

        blueprint_core::info!("Runner task finished");
        match handle.await {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => {
                blueprint_core::error!("Runner failed during startup: {}", e);
                Err(e)
            }
            Err(e) => {
                blueprint_core::error!("Runner task panicked: {}", e);
                Err(JobCallError::JobDidntFinish(e).into())
            }
        }
    }
}

impl<Ctx, C> Drop for TangleTestEnv<Ctx, C>
where
    C: HeartbeatConsumer + Send + Sync + 'static,
{
    fn drop(&mut self) {
        futures::executor::block_on(async {
            let mut guard = self.runner_handle.lock().await;
            if let Some(handle) = guard.take() {
                handle.abort();
            }
        });
    }
}

/// Mock implementation of the `HeartbeatConsumer` for testing
#[derive(Clone, Default)]
pub struct MockHeartbeatConsumer {
    pub heartbeats: Arc<Mutex<Vec<HeartbeatStatus>>>,
}

impl MockHeartbeatConsumer {
    #[must_use]
    pub fn new() -> Self {
        Self {
            heartbeats: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Returns the number of heartbeats received
    ///
    /// # Panics
    ///
    /// Panics if the heartbeats mutex is poisoned
    #[must_use]
    pub async fn heartbeat_count(&self) -> usize {
        self.heartbeats.lock().await.len()
    }

    /// Gets a copy of all received heartbeat statuses
    ///
    /// # Panics
    ///
    /// Panics if the heartbeats mutex is poisoned
    #[must_use]
    pub async fn get_heartbeats(&self) -> Vec<HeartbeatStatus> {
        self.heartbeats.lock().await.clone()
    }
}

impl HeartbeatConsumer for MockHeartbeatConsumer {
    fn send_heartbeat(
        &self,
        status: &HeartbeatStatus,
    ) -> impl std::future::Future<Output = Result<(), blueprint_qos::error::Error>> + Send {
        let status = status.clone();
        let heartbeats = self.heartbeats.clone();

        async move {
            heartbeats.lock().await.push(status);
            Ok(())
        }
    }
}
