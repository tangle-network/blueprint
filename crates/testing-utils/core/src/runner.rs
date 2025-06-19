use blueprint_core::Job;
use blueprint_router::Router;
use blueprint_runner::BlueprintConfig;
use blueprint_runner::config::BlueprintEnvironment;
use blueprint_runner::error::RunnerError as Error;
use blueprint_runner::{BackgroundService, BlueprintRunner, BlueprintRunnerBuilder};
use std::future::{self, Pending};
use std::sync::Arc;
use tokio::sync::oneshot;

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
        let builder =
            BlueprintRunner::builder(config, env).with_shutdown_handler(future::pending::<()>());
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

    /// Integrate the unified `QoS` service (heartbeat, metrics, logging, dashboards) as an always-on background service.
    ///
    /// # Panics
    ///
    /// Panics if the builder is not initialized.
    pub fn qos_service<C>(
        &mut self,
        qos_service: Arc<blueprint_qos::unified_service::QoSService<C>>,
    ) -> &mut Self
    where
        C: blueprint_qos::heartbeat::HeartbeatConsumer + Send + Sync + 'static,
    {
        struct QoSServiceAdapter<
            C: blueprint_qos::heartbeat::HeartbeatConsumer + Send + Sync + 'static,
        > {
            qos_service: Arc<blueprint_qos::unified_service::QoSService<C>>,
        }

        impl<C> BackgroundService for QoSServiceAdapter<C>
        where
            C: blueprint_qos::heartbeat::HeartbeatConsumer + Send + Sync + 'static,
        {
            async fn start(
                &self,
            ) -> Result<tokio::sync::oneshot::Receiver<Result<(), Error>>, Error> {
                blueprint_core::info!(
                    "QoSServiceAdapter: Starting... Will integrate with QoSService lifecycle."
                );
                let (runner_tx, runner_rx) = oneshot::channel();

                let (qos_completion_tx, qos_completion_rx) =
                    tokio::sync::oneshot::channel::<Result<(), blueprint_qos::error::Error>>();

                self.qos_service
                    .set_completion_sender(qos_completion_tx)
                    .await;

                tokio::spawn(async move {
                    match qos_completion_rx.await {
                        Ok(Ok(())) => {
                            if runner_tx.send(Ok(())).is_err() {
                                blueprint_core::warn!(
                                    "QoSServiceAdapter: runner_rx dropped before successful QoS completion signal."
                                );
                            }
                        }
                        Ok(Err(qos_internal_err)) => {
                            blueprint_core::error!(
                                "QoSServiceAdapter: QoSService completed with an internal error: {}. Signaling runner.",
                                qos_internal_err
                            );
                            if runner_tx
                                .send(Err(Error::BackgroundService(format!(
                                    "QoS service internal error: {}",
                                    qos_internal_err
                                ))))
                                .is_err()
                            {
                                blueprint_core::warn!(
                                    "QoSServiceAdapter: runner_rx dropped before QoS internal error signal."
                                );
                            }
                        }
                        Err(_recv_err) => {
                            blueprint_core::error!(
                                "QoSServiceAdapter: QoSService completion sender (qos_completion_tx) was dropped. This typically means QoSService panicked or did not complete cleanly. Signaling runner."
                            );
                            if runner_tx.send(Err(Error::BackgroundService(
                                "QoS service did not signal completion cleanly (sender dropped, possibly due to panic).".to_string(),
                            ))).is_err() {
                                blueprint_core::warn!(
                                    "QoSServiceAdapter: runner_rx dropped after qos_completion_tx was dropped."
                                );
                            }
                        }
                    }
                });

                Ok(runner_rx)
            }
        }

        let adapter = QoSServiceAdapter { qos_service };

        self.builder = Some(
            self.builder
                .take()
                .expect("BlueprintRunnerBuilder should always exist")
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
