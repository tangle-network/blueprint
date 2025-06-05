use blueprint_core::Job;
use blueprint_router::Router;
use blueprint_runner::BlueprintConfig;
use blueprint_runner::config::BlueprintEnvironment;
use blueprint_runner::error::RunnerError as Error;
// use blueprint_runner::metrics_server::MetricsServerAdapter; // Removed unused import
use blueprint_runner::{BackgroundService, BlueprintRunner, BlueprintRunnerBuilder};
use std::future; // Retained for `Pending` type and potential `future::pending` elsewhere
use std::future::Pending;
use std::sync::Arc;
use tokio::sync::{Mutex, oneshot};

// A background service that never completes on its own.
// Its purpose is to keep the BlueprintRunner's main loop alive in test environments
// where there might not be any active producers, but background services (like QoS)
// need to continue running for the duration of the test.
struct KeepAliveService;

impl BackgroundService for KeepAliveService {
    async fn start(&self) -> Result<oneshot::Receiver<Result<(), Error>>, Error> {
        blueprint_core::error!("!!! KEEPALIVESERVICE STARTING NOW (Forget Strategy) !!!");
        let (tx, rx) = oneshot::channel();

        // Forget the sender. This means the sender is not dropped,
        // and since no message is sent on it, the receiver (rx)
        // will pend indefinitely.
        std::mem::forget(tx);

        blueprint_core::info!(
            "KeepAliveService: tx sender forgotten, returning receiver that should pend indefinitely."
        );
        Ok(rx)
    }
}

pub struct TestRunner<Ctx> {
    router: Option<Router<Ctx>>,
    job_index: usize,
    #[doc(hidden)]
    pub builder: Option<BlueprintRunnerBuilder<Pending<()>>>,
    _phantom: core::marker::PhantomData<Ctx>,
}

impl<Ctx> TestRunner<Ctx>
where
    Ctx: Clone + Send + Sync + 'static,
{
    pub fn new<C>(config: C, env: BlueprintEnvironment) -> Self
    where
        C: BlueprintConfig + 'static,
    {
        let builder = BlueprintRunner::builder(config, env)
            .with_shutdown_handler(future::pending())
            .background_service(KeepAliveService); // Add KeepAliveService
        blueprint_core::error!("!!! TestRunner::new - KeepAliveService ADDED to builder !!!");
        TestRunner {
            router: Some(Router::<Ctx>::new()),
            job_index: 0,
            builder: Some(builder),
            _phantom: core::marker::PhantomData,
        }
    }

    #[expect(clippy::missing_panics_doc)]
    pub fn add_job<J, T>(&mut self, job: J) -> &mut Self
    where
        J: Job<T, Ctx> + Send + Sync + 'static,
        T: 'static,
    {
        self.router = Some(
            self.router
                .take()
                .expect("router should always exist")
                .route(self.job_index, job),
        );
        self.job_index += 1;
        self
    }

    #[expect(clippy::missing_panics_doc)]
    pub fn add_background_service<B>(&mut self, service: B) -> &mut Self
    where
        B: BackgroundService + Send + 'static,
    {
        self.builder = Some(
            self.builder
                .take()
                .expect("router should always exist")
                .background_service(service),
        );
        self
    }

    /// Integrate the unified QoS service (heartbeat, metrics, logging, dashboards) as an always-on background service.
    pub async fn qos_service<C>(
        &mut self,
        qos_service: Arc<Mutex<Option<blueprint_qos::unified_service::QoSService<C>>>>,
        // config: blueprint_qos::QoSConfig,
        // heartbeat_consumer: std::sync::Arc<C>,
    ) -> &mut Self
    where
        C: blueprint_qos::heartbeat::HeartbeatConsumer + Send + Sync + 'static,
    {
        struct QoSServiceAdapter<
            C: blueprint_qos::heartbeat::HeartbeatConsumer + Send + Sync + 'static,
        > {
            qos_service: Arc<Mutex<Option<blueprint_qos::unified_service::QoSService<C>>>>,
        }

        impl<C> BackgroundService for QoSServiceAdapter<C>
        where
            C: blueprint_qos::heartbeat::HeartbeatConsumer + Send + Sync + 'static,
        {
            async fn start(
                &self,
            ) -> Result<tokio::sync::oneshot::Receiver<Result<(), Error>>, Error> {
                let (runner_tx, runner_rx) = tokio::sync::oneshot::channel::<Result<(), Error>>();

                let mut service_guard = self.qos_service.lock().await;
                if let Some(service_instance) = service_guard.as_mut() {
                    // Create the channel for QoSService
                    let (qos_tx, qos_rx) =
                        tokio::sync::oneshot::channel::<blueprint_qos::error::Result<()>>();

                    // Start heartbeat if applicable
                    if let Some(hb) = service_instance.heartbeat_service() {
                        if let Err(e) = hb.start_heartbeat().await {
                            blueprint_core::error!(
                                "QoSServiceAdapter: Failed to start heartbeat: {:?}",
                                e
                            );
                            // This error is logged but doesn't prevent the adapter from starting.
                            // Depending on requirements, this could be made a fatal error for start.
                        }
                    }
                    service_instance.set_completion_sender(qos_tx);
                    blueprint_core::info!("QoSServiceAdapter: start: qos_tx passed to QoSService.");

                    tokio::spawn(async move {
                        match qos_rx.await {
                            Ok(qos_result) => {
                                // Convert blueprint_qos::error::Error to RunnerError (aliased as Error here)
                                let runner_result = qos_result.map_err(|qos_err| {
                                    Error::BackgroundService(format!(
                                        "QoS Service error: {}",
                                        qos_err
                                    ))
                                });
                                if runner_tx.send(runner_result).is_err() {
                                    blueprint_core::error!(
                                        "QoSServiceAdapter bridge: runner_rx was dropped before completion signal could be sent."
                                    );
                                }
                            }
                            Err(_recv_error) => {
                                // qos_tx was dropped without sending a value
                                blueprint_core::error!(
                                    "QoSServiceAdapter bridge: qos_rx received an error. This means qos_tx was dropped, possibly due to QoSService panic or not calling set_completion_sender properly."
                                );
                                if runner_tx
                                    .send(Err(Error::BackgroundService(
                                        "QoS service did not signal completion (sender dropped)."
                                            .to_string(),
                                    )))
                                    .is_err()
                                {
                                    blueprint_core::error!(
                                        "QoSServiceAdapter bridge: runner_rx was also dropped after qos_tx drop."
                                    );
                                }
                            }
                        }
                    });
                } else {
                    blueprint_core::error!(
                        "QoSServiceAdapter: start: QoSService instance was None. Cannot initialize."
                    );
                    // Immediately send an error on runner_tx as the service isn't there.
                    if runner_tx
                        .send(Err(Error::BackgroundService(
                            "QoSService instance not available at start".to_string(),
                        )))
                        .is_err()
                    {
                        blueprint_core::error!(
                            "QoSServiceAdapter: start: runner_rx was dropped while reporting service_instance None."
                        );
                    }
                }
                // Drop the guard to release the Mutex lock before returning the receiver
                drop(service_guard);
                Ok(runner_rx)
            }
        }

        let adapter = QoSServiceAdapter { qos_service };

        self.builder = Some(
            self.builder
                .take()
                .expect("failed to add QoS service")
                .background_service(adapter),
        );
        self
    }

    /// Start the runner
    ///
    /// # Errors
    ///
    /// See [`BlueprintRunnerBuilder::run()`]
    #[expect(clippy::missing_panics_doc)]
    pub async fn run(self, context: Ctx) -> Result<(), Error> {
        let router: Router = self.router.unwrap().with_context(context);

        self.builder.unwrap().router(router).run().await
    }
}

pub trait TestEnv: Sized {
    type Config: BlueprintConfig;
    type Context: Clone + Send + Sync + 'static;

    /// Create a new test environment
    ///
    /// # Errors
    ///
    /// Errors depend on the implementation.
    fn new(config: Self::Config, env: BlueprintEnvironment) -> Result<Self, Error>;
    fn add_job<J, T>(&mut self, job: J)
    where
        J: Job<T, Self::Context> + Send + Sync + 'static,
        T: 'static;
    fn add_background_service<B>(&mut self, service: B)
    where
        B: BackgroundService + Send + 'static;
    fn get_blueprint_config(&self) -> BlueprintEnvironment;

    /// Start the runner
    ///
    /// # Panics
    ///
    /// Will panic if the runner is already started
    fn run_runner(
        &mut self,
        context: Self::Context,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;
}
