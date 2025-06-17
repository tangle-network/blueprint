//! Blueprint SDK job runners
//!
//! This crate provides the core functionality for configuring and running blueprints.
//!
//! ## Features
#![doc = document_features::document_features!()]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// TODO: #![warn(missing_docs)]

extern crate alloc;

pub mod config;
pub mod error;
pub mod metrics_server;

#[cfg(feature = "eigenlayer")]
pub mod eigenlayer;
#[cfg(feature = "symbiotic")]
mod symbiotic;
#[cfg(feature = "tangle")]
pub mod tangle;

use crate::error::RunnerError;
use crate::error::{JobCallError, ProducerError};
use blueprint_core::error::BoxError;
use blueprint_core::{JobCall, JobResult};
use blueprint_qos::heartbeat::HeartbeatConsumer;
use blueprint_router::Router;
use config::BlueprintEnvironment;
use core::future::{self, poll_fn};
use core::pin::Pin;
use error::RunnerError as Error;
use futures::{Future, Sink};
use futures_core::Stream;
use futures_util::stream::FuturesUnordered;
use futures_util::{SinkExt, StreamExt, TryStreamExt, stream};
use std::sync::Arc;
use tokio::sync::{Mutex, oneshot};
use tower::Service;
use tracing::error;

/// Configuration for the blueprint registration procedure
#[dynosaur::dynosaur(DynBlueprintConfig)]
pub trait BlueprintConfig: Send + Sync {
    /// The registration logic for this protocol
    ///
    /// By default, this will do nothing.
    fn register(
        &self,
        env: &BlueprintEnvironment,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        let _ = env;
        async { Ok(()) }
    }

    /// Determines whether this protocol requires registration
    ///
    /// This determines whether [`Self::register()`] is called.
    ///
    /// By default, this will return `true`.
    fn requires_registration(
        &self,
        env: &BlueprintEnvironment,
    ) -> impl Future<Output = Result<bool, Error>> + Send {
        let _ = env;
        async { Ok(true) }
    }

    /// Determines whether the runner should exit after registration
    ///
    /// By default, this will return `true`.
    fn should_exit_after_registration(&self) -> bool {
        true // By default, runners exit after registration
    }
}

unsafe impl Send for DynBlueprintConfig<'_> {}
unsafe impl Sync for DynBlueprintConfig<'_> {}

impl BlueprintConfig for () {}

/// A background service to be handled by a [`BlueprintRunner`]
///
/// # Usage
///
/// ```rust
/// use blueprint_runner::BackgroundService;
/// use blueprint_runner::error::RunnerError;
/// use tokio::sync::oneshot::{self, Receiver};
///
/// // A dummy background service that immediately returns
/// #[derive(Clone)]
/// pub struct FooBackgroundService;
///
/// impl BackgroundService for FooBackgroundService {
///     async fn start(&self) -> Result<Receiver<Result<(), RunnerError>>, RunnerError> {
///         let (tx, rx) = oneshot::channel();
///         tokio::spawn(async move {
///             let _ = tx.send(Ok(()));
///         });
///         Ok(rx)
///     }
/// }
/// ```
#[dynosaur::dynosaur(DynBackgroundService)]
pub trait BackgroundService: Send + Sync {
    /// Start this background service
    ///
    /// This method returns a one-shot [Receiver](oneshot::Receiver), that is used to indicate when
    /// the service stops running, either by finishing (`Ok(())`) or by error (`Err(e)`).
    fn start(
        &self,
    ) -> impl Future<Output = Result<oneshot::Receiver<Result<(), Error>>, Error>> + Send;
}

unsafe impl Send for DynBackgroundService<'_> {}
unsafe impl Sync for DynBackgroundService<'_> {}

type Producer =
    Arc<Mutex<Box<dyn Stream<Item = Result<JobCall, BoxError>> + Send + Unpin + 'static>>>;
type Consumer = Arc<Mutex<Box<dyn Sink<JobResult, Error = BoxError> + Send + Unpin + 'static>>>;

/// A builder for a [`BlueprintRunner`]
///
/// This is created with [`BlueprintRunner::builder()`].
pub struct BlueprintRunnerBuilder<F> {
    config: Box<DynBlueprintConfig<'static>>,
    env: BlueprintEnvironment,
    producers: Vec<Producer>,
    consumers: Vec<Consumer>,
    router: Option<Router>,
    background_services: Vec<Box<DynBackgroundService<'static>>>,
    shutdown_handler: F,
}

impl<F> BlueprintRunnerBuilder<F>
where
    F: Future<Output = ()> + Send + 'static,
{
    /// Set the [`Router`] for this runner
    ///
    /// A [`Router`] is the only required field in a [`BlueprintRunner`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use blueprint_runner::config::BlueprintEnvironment;
    /// use blueprint_router::Router;
    /// use blueprint_runner::BlueprintRunner;
    /// use futures::future;
    /// use tokio::sync::oneshot;
    /// use tokio::sync::oneshot::Receiver;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = /* ... */
    ///     # ();
    ///     let router = Router::new().route(0, async || "Hello, world!");
    ///
    ///     // Load the blueprint environment
    ///     let blueprint_env = BlueprintEnvironment::default();
    ///
    ///     let result = BlueprintRunner::builder(config, blueprint_env)
    ///         // Add the router to the runner
    ///         .router(router)
    ///         // Then start it up...
    ///         .run()
    ///         .await;
    ///
    ///     // ...
    /// }
    /// ```
    #[must_use]
    pub fn router(mut self, router: Router) -> Self {
        self.router = Some(router);
        self
    }

    /// Append a [producer] to the list
    ///
    /// [producer]: https://docs.rs/blueprint_sdk/latest/blueprint_sdk/producers/index.html
    #[must_use]
    pub fn producer<E>(
        mut self,
        producer: impl Stream<Item = Result<JobCall, E>> + Send + Unpin + 'static,
    ) -> Self
    where
        E: Into<BoxError> + 'static,
    {
        let producer: Producer = Arc::new(Mutex::new(Box::new(producer.map_err(Into::into))));
        self.producers.push(producer);
        self
    }

    /// Append a [consumer] to the list
    ///
    /// [consumer]: https://docs.rs/blueprint_sdk/latest/blueprint_sdk/consumers/index.html
    #[must_use]
    pub fn consumer<E>(
        mut self,
        consumer: impl Sink<JobResult, Error = E> + Send + Unpin + 'static,
    ) -> Self
    where
        E: Into<BoxError> + 'static,
    {
        let consumer: Consumer = Arc::new(Mutex::new(Box::new(consumer.sink_map_err(Into::into))));
        self.consumers.push(consumer);
        self
    }

    /// Add a custom heartbeat service as a background service
    ///
    /// This method is a convenience wrapper around `background_service` specifically for
    /// adding a custom heartbeat service from the `QoS` crate. The core heartbeat logic from the
    /// unified `QoS` service is always running if `.qos_service(...)` is called. This method should only
    /// be used to add additional, blueprint-specific health checks.
    /// periodic heartbeats to the chain or other monitoring systems.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use blueprint_qos::heartbeat::{HeartbeatConfig, HeartbeatConsumer, HeartbeatService};
    /// use blueprint_router::Router;
    /// use blueprint_runner::BlueprintRunner;
    /// use blueprint_runner::config::BlueprintEnvironment;
    /// use std::sync::Arc;
    ///
    /// // Define a custom heartbeat consumer
    /// struct MyHeartbeatConsumer;
    ///
    /// #[tonic::async_trait]
    /// impl HeartbeatConsumer for MyHeartbeatConsumer {
    ///     async fn send_heartbeat(
    ///         &self,
    ///         status: &blueprint_qos::heartbeat::HeartbeatStatus,
    ///     ) -> Result<(), blueprint_qos::error::Error> {
    ///         // Send heartbeat to the chain or other monitoring systems
    ///         Ok(())
    ///     }
    /// }
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let env = BlueprintEnvironment::load()?;
    /// let router = Router::new();
    ///
    /// // Create a heartbeat service with custom consumer
    /// let config = HeartbeatConfig::default();
    /// let consumer = Arc::new(MyHeartbeatConsumer);
    /// let heartbeat_service = HeartbeatService::new(config, consumer);
    ///
    /// BlueprintRunner::builder((), env)
    ///     .router(router)
    ///     .heartbeat_service(heartbeat_service)
    ///     .run()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn heartbeat_service<C: HeartbeatConsumer + Send + Sync + 'static>(
        mut self,
        service: blueprint_qos::heartbeat::HeartbeatService<C>,
    ) -> Self {
        #[derive(Clone)]
        struct HeartbeatServiceAdapter<C: HeartbeatConsumer + Send + Sync + 'static> {
            service: blueprint_qos::heartbeat::HeartbeatService<C>,
        }

        impl<C: HeartbeatConsumer + Send + Sync + 'static> BackgroundService
            for HeartbeatServiceAdapter<C>
        {
            async fn start(&self) -> Result<oneshot::Receiver<Result<(), Error>>, Error> {
                self.service
                    .start_heartbeat()
                    .await
                    .map_err(|e| RunnerError::Other(e.into()))?;
                let (_tx, rx) = oneshot::channel();
                Ok(rx)
            }
        }

        let adapter = HeartbeatServiceAdapter { service };
        self.background_services
            .push(DynBackgroundService::boxed(adapter));
        self
    }

    /// Add a metrics server as a background service
    ///
    /// This method is a convenience wrapper around `background_service` specifically for
    /// adding a metrics server from the `QoS` crate. The metrics server will serve
    /// Prometheus metrics on the configured endpoint.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use blueprint_runner::config::BlueprintEnvironment;
    /// use blueprint_router::Router;
    /// use blueprint_qos::servers::prometheus::PrometheusServerConfig;
    /// use blueprint_qos::QoSServiceBuilder;
    /// use std::sync::Arc;
    ///
    /// #[derive(Clone)]
    /// struct MyContext;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), blueprint_runner::error::RunnerError> {
    ///     let env = BlueprintEnvironment::load()?;
    ///     let context = Arc::new(MyContext);
    ///     let router = Router::new().with_context(context.clone());
    ///
    ///     let qos_service = QoSServiceBuilder::new(env.clone())
    ///         .with_prometheus_server_config(PrometheusServerConfig::default())
    ///         .manage_servers(true)
    ///         .build()
    ///         .await?;
    ///
    ///     if let Some(prometheus_server) = qos_service.prometheus_server {
    ///         blueprint_runner::BlueprintRunner::builder((), env)
    ///             .router(router)
    ///             .metrics_server(prometheus_server)
    ///             .run()
    ///             .await?;
    ///     }
    ///     # Ok(())
    ///     # }
    /// ```
    #[must_use]
    pub fn metrics_server(
        mut self,
        server: Arc<blueprint_qos::servers::prometheus::PrometheusServer>,
    ) -> Self {
        // Create a background service adapter for the metrics server
        let adapter = self::metrics_server::MetricsServerAdapter::new(server);
        self.background_services
            .push(DynBackgroundService::boxed(adapter));
        self
    }

    /// Integrate the unified `QoS` service (heartbeat, metrics, logging, dashboards) as an always-on background service.
    ///
    /// This method instantiates and manages a `QoSService` internally, automatically starting its heartbeat and metrics
    /// background tasks if configured. Custom heartbeat and metrics logic can still be added via `.heartbeat_service` and
    /// `.metrics_server`, but the core `QoS` logic is always running if this is called.
    #[must_use]
    pub fn qos_service<C: HeartbeatConsumer + Send + Sync + 'static>(
        mut self,
        config: blueprint_qos::QoSConfig,
        heartbeat_consumer: Option<Arc<C>>,
    ) -> Self {
        struct QoSServiceAdapter<C: HeartbeatConsumer + Send + Sync + 'static> {
            qos_service: Arc<Mutex<Option<blueprint_qos::QoSService<C>>>>,
        }

        impl<C: HeartbeatConsumer + Send + Sync + 'static> BackgroundService for QoSServiceAdapter<C> {
            async fn start(
                &self,
            ) -> Result<
                tokio::sync::oneshot::Receiver<Result<(), crate::error::RunnerError>>,
                crate::error::RunnerError,
            > {
                let (tx, rx) = tokio::sync::oneshot::channel();
                let qos_arc_for_adapter_task = self.qos_service.clone();

                tokio::spawn(async move {
                    blueprint_core::debug!(target: "blueprint-runner", "QoS Adapter Task (Task 2): Waiting for QoS Service build...");

                    let mut attempt_count = 0;
                    let max_attempts = 300; // Approx 30 seconds with 100ms polling

                    let mut service_guard = loop {
                        let guard = qos_arc_for_adapter_task.lock().await;
                        if guard.is_some() {
                            blueprint_core::debug!(target: "blueprint-runner", "QoS Adapter Task (Task 2): QoS Service is built. Proceeding to start components.");
                            break guard;
                        }
                        drop(guard);

                        attempt_count += 1;
                        if attempt_count >= max_attempts {
                            let err_msg = "QoS Adapter Task (Task 2): QoS service did not initialize (build task may have failed or timed out).";
                            blueprint_core::error!(target: "blueprint-runner", "{}", err_msg);
                            let _ = tx.send(Err(crate::error::RunnerError::Other(Box::new(
                                std::io::Error::new(std::io::ErrorKind::Other, err_msg),
                            ))));
                            return;
                        }
                        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                    };

                    let qos_service_mut = service_guard
                        .as_mut()
                        .expect("QoS service was Some but now None, this should not happen");

                    if let Some(hb_service) = qos_service_mut.heartbeat_service() {
                        blueprint_core::debug!(target: "blueprint-runner", "QoS Adapter Task (Task 2): Starting heartbeat service...");
                        if let Err(e) = hb_service.start_heartbeat().await {
                            let err_msg = format!(
                                "QoS Adapter Task (Task 2): Heartbeat service failed to start: {:?}",
                                e
                            );
                            blueprint_core::error!(target: "blueprint-runner", "{}", err_msg);
                            let _ = tx.send(Err(crate::error::RunnerError::Other(Box::new(
                                std::io::Error::new(std::io::ErrorKind::Other, err_msg),
                            ))));
                            return;
                        }
                        blueprint_core::info!(target: "blueprint-runner", "QoS Adapter Task (Task 2): Heartbeat service started.");
                    } else {
                        blueprint_core::debug!(target: "blueprint-runner", "QoS Adapter Task (Task 2): No heartbeat service configured in QoSService.");
                    }

                    blueprint_core::info!(target: "blueprint-runner", "QoS Adapter Task (Task 2): QoS Service components initialized successfully.");
                    let _ = tx.send(Ok(()));
                });

                Ok(rx)
            }
        }

        let qos_service_arc = Arc::new(Mutex::new(None::<blueprint_qos::QoSService<C>>));

        let builder_task_qos_arc = qos_service_arc.clone();
        tokio::spawn(async move {
            blueprint_core::debug!(target: "blueprint-runner", "QoS Builder Task (Task 1): Initializing QoS Service...");
            let mut builder = blueprint_qos::QoSServiceBuilder::<C>::new()
                .with_config(config)
                .manage_servers(true);

            if let Some(consumer) = heartbeat_consumer {
                builder = builder.with_heartbeat_consumer(consumer);
            }

            match builder.build().await {
                Ok(service) => {
                    let mut guard = builder_task_qos_arc.lock().await;
                    *guard = Some(service);
                    blueprint_core::info!(target: "blueprint-runner", "QoS Builder Task (Task 1): QoS Service built and stored.");
                }
                Err(e) => {
                    blueprint_core::error!(target: "blueprint-runner", "QoS Builder Task (Task 1): Failed to build QoS service: {:?}", e);
                }
            }
        });

        let adapter = QoSServiceAdapter::<C> {
            qos_service: qos_service_arc,
        };
        self.background_services
            .push(DynBackgroundService::boxed(adapter));
        self
    }

    /// Append a background service to the list
    ///
    /// # Examples
    ///
    /// ```rust
    /// use blueprint_runner::config::BlueprintEnvironment;
    /// use blueprint_router::Router;
    /// use blueprint_runner::error::RunnerError;
    /// use blueprint_runner::{BackgroundService, BlueprintRunner};
    /// use futures::future;
    /// use tokio::sync::oneshot;
    /// use tokio::sync::oneshot::Receiver;
    ///
    /// // A dummy background service that immediately returns
    /// #[derive(Clone)]
    /// pub struct FooBackgroundService;
    ///
    /// impl BackgroundService for FooBackgroundService {
    ///     async fn start(&self) -> Result<Receiver<Result<(), RunnerError>>, RunnerError> {
    ///         let (tx, rx) = oneshot::channel();
    ///         tokio::spawn(async move {
    ///             let _ = tx.send(Ok(()));
    ///         });
    ///         Ok(rx)
    ///     }
    /// }
    ///
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = /* ... */
    ///     # ();
    ///     let router = /* ... */
    ///     # Router::new().route(0, async || "Hello, world!");
    ///
    ///     // Load the blueprint environment
    ///     let blueprint_env = BlueprintEnvironment::default();
    ///
    ///     let result = BlueprintRunner::builder(config, blueprint_env)
    ///         .router(router)
    ///         // Add potentially many background services
    ///         .background_service(FooBackgroundService)
    ///         // Then start it up...
    ///         .run()
    ///         .await;
    ///
    ///     // ...
    /// }
    /// ```
    #[must_use]
    pub fn background_service(mut self, service: impl BackgroundService + 'static) -> Self {
        self.background_services
            .push(DynBackgroundService::boxed(service));
        self
    }

    /// Set the shutdown handler
    ///
    /// This will be run **before** the runner terminates any of the following:
    ///
    /// * [Producers]
    /// * [Consumers]
    /// * [Background Services]
    ///
    /// Meaning it is a good place to do cleanup logic, such as finalizing database transactions.
    ///
    /// [Producers]: https://docs.rs/blueprint_sdk/latest/blueprint_sdk/producers/index.html
    /// [Consumers]: https://docs.rs/blueprint_sdk/latest/blueprint_sdk/consumers/index.html
    /// [Background Services]: crate::BackgroundService
    ///
    /// # Examples
    ///
    /// ```rust
    /// use blueprint_runner::config::BlueprintEnvironment;
    /// use blueprint_router::Router;
    /// use blueprint_runner::BlueprintRunner;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = /* ... */
    ///     # ();
    ///     let router = /* ... */
    ///     # Router::new().route(0, async || "Hello, world!");
    ///
    ///     // Load the blueprint environment
    ///     let blueprint_env = BlueprintEnvironment::default();
    ///
    ///     let result = BlueprintRunner::builder(config, blueprint_env)
    ///         .router(router)
    ///         // Specify what to do when an error occurs and the runner is shutting down.
    ///         // That can be cleanup logic, finalizing database transactions, etc.
    ///         .with_shutdown_handler(async { println!("Shutting down!") })
    ///         // Then start it up...
    ///         .run()
    ///         .await;
    ///
    ///     // ...
    /// }
    /// ```
    pub fn with_shutdown_handler<F2>(self, handler: F2) -> BlueprintRunnerBuilder<F2>
    where
        F2: Future<Output = ()> + Send + 'static,
    {
        BlueprintRunnerBuilder {
            config: self.config,
            env: self.env,
            producers: self.producers,
            consumers: self.consumers,
            router: self.router,
            background_services: self.background_services,
            shutdown_handler: handler,
        }
    }

    /// Start the runner
    ///
    /// This will block until the runner finishes.
    ///
    /// # Errors
    ///
    /// If at any point the runner fails, an error will be returned. See [`Self::with_shutdown_handler`]
    /// to understand what this means for your running services.
    pub async fn run(self) -> Result<(), Error> {
        let Some(router) = self.router else {
            return Err(Error::NoRouter);
        };

        if self.producers.is_empty() {
            return Err(Error::NoProducers);
        }

        let runner = FinalizedBlueprintRunner {
            config: self.config,
            producers: self.producers,
            consumers: self.consumers,
            router,
            env: self.env,
            background_services: self.background_services,
            shutdown_handler: self.shutdown_handler,
        };

        runner.run().await
    }
}

/// The blueprint runner
///
/// This is responsible for orchestrating the following:
///
/// * [Producers]
/// * [Consumers]
/// * [Background Services](crate::BackgroundService)
/// * [`Router`]
///
/// # Usage
///
/// Note that this is a **full** example. All fields, with the exception of the [`Router`] can be
/// omitted.
///
/// ```no_run
/// use blueprint_router::Router;
/// use blueprint_runner::config::BlueprintEnvironment;
/// use blueprint_runner::error::RunnerError;
/// use blueprint_runner::{BackgroundService, BlueprintRunner};
/// use futures::future;
/// use tokio::sync::oneshot;
/// use tokio::sync::oneshot::Receiver;
///
/// // A dummy background service that immediately returns
/// #[derive(Clone)]
/// pub struct FooBackgroundService;
///
/// impl BackgroundService for FooBackgroundService {
///     async fn start(&self) -> Result<Receiver<Result<(), RunnerError>>, RunnerError> {
///         let (tx, rx) = oneshot::channel();
///         tokio::spawn(async move {
///             let _ = tx.send(Ok(()));
///         });
///         Ok(rx)
///     }
/// }
///
/// #[tokio::main]
/// async fn main() {
///     // The config is any type implementing the [BlueprintConfig] trait.
///     // In this case, () works.
///     let config = ();
///
///     // Load the blueprint environment
///     let blueprint_env = BlueprintEnvironment::default();
///
///     // Create some producer(s)
///     let some_producer = /* ... */
///     # blueprint_sdk::tangle::producer::TangleProducer::finalized_blocks(todo!()).await.unwrap();
///     # struct S;
///     # use blueprint_sdk::tangle::subxt_core::config::{PolkadotConfig, Config};
///     # impl blueprint_sdk::tangle::subxt_core::tx::signer::Signer<PolkadotConfig> for S {
///     #     fn account_id(&self) -> <PolkadotConfig as Config>::AccountId {
///     #         todo!()
///     #     }
///     #
///     #     fn address(&self) -> <PolkadotConfig as Config>::Address {
///     #         todo!()
///     #     }
///     #
///     #     fn sign(&self, signer_payload: &[u8]) -> <PolkadotConfig as Config>::Signature {
///     #         todo!()
///     #     }
///     # }
///     // Create some consumer(s)
///     let some_consumer = /* ... */
///     # blueprint_sdk::tangle::consumer::TangleConsumer::<S>::new(todo!(), todo!());
///
///     let result = BlueprintRunner::builder(config, blueprint_env)
///         .router(
///             // Add a `Router`, where each "route" is a job ID and the job function.
///             Router::new().route(0, async || "Hello, world!"),
///         )
///         // Add potentially many producers
///         .producer(some_producer)
///         // Add potentially many consumers
///         .consumer(some_consumer)
///         // Add potentially many background services
///         .background_service(FooBackgroundService)
///         // Specify what to do when an error occurs and the runner is shutting down.
///         // That can be cleanup logic, finalizing database transactions, etc.
///         .with_shutdown_handler(async { println!("Shutting down!") })
///         // Then start it up...
///         .run()
///         .await;
///
///     // The runner, after running the shutdown handler, will return
///     // an error if something goes wrong
///     if let Err(e) = result {
///         eprintln!("Runner failed! {e:?}");
///     }
/// }
/// ```
///
/// [Consumers]: https://docs.rs/blueprint_sdk/latest/blueprint_sdk/consumers/index.html
/// [Producers]: https://docs.rs/blueprint_sdk/latest/blueprint_sdk/producers/index.html
pub struct BlueprintRunner {}

impl BlueprintRunner {
    /// Create a new [`BlueprintRunnerBuilder`]
    ///
    /// See the usage section of [`BlueprintRunner`]
    pub fn builder<Conf: BlueprintConfig + 'static>(
        config: Conf,
        env: BlueprintEnvironment,
    ) -> BlueprintRunnerBuilder<impl Future<Output = ()> + Send + 'static> {
        BlueprintRunnerBuilder {
            config: DynBlueprintConfig::boxed(config),
            env,
            producers: Vec::new(),
            consumers: Vec::new(),
            router: None,
            background_services: Vec::new(),
            shutdown_handler: future::pending(),
        }
    }
}

struct FinalizedBlueprintRunner<F> {
    config: Box<DynBlueprintConfig<'static>>,
    producers: Vec<Producer>,
    consumers: Vec<Consumer>,
    router: Router,
    env: BlueprintEnvironment,
    background_services: Vec<Box<DynBackgroundService<'static>>>,
    shutdown_handler: F,
}

impl<F> FinalizedBlueprintRunner<F>
where
    F: Future<Output = ()> + Send + 'static,
{
    #[allow(trivial_casts)]
    async fn run(self) -> Result<(), Error> {
        if self.config.requires_registration(&self.env).await? {
            self.config.register(&self.env).await?;
            if self.config.should_exit_after_registration() {
                return Ok(());
            }
        }

        // TODO: Config is unused
        let FinalizedBlueprintRunner {
            config: _,
            producers,
            mut consumers,
            mut router,
            env,
            background_services,
            shutdown_handler,
        } = self;

        let mut router = router.as_service();

        let has_background_services = !background_services.is_empty();
        let mut background_futures = Vec::with_capacity(background_services.len());

        // Startup background services
        // Iterate by reference (&service_box) over `background_services` (the Vec of Boxes)
        // This ensures that the `Box<dyn BackgroundService>` instances themselves remain owned by
        // the `background_services` vector and are not dropped after `start()` is called.
        for service_box in &background_services {
            // service_box is &Box<dyn BackgroundService>
            let receiver = service_box.start().await?;
            background_futures.push(Box::pin(async move {
                receiver
                    .await
                    .map_err(|e| Error::BackgroundService(e.to_string()))
                    .and(Ok(()))
            })
                as Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>);
        }
        // The `background_services` Vec (containing the Boxed service instances) is still alive here
        // and will be dropped only when `FinalizedBlueprintRunner::run` exits.

        let (mut shutdown_tx, shutdown_rx) = oneshot::channel();
        tokio::spawn(async move {
            let _ = shutdown_rx.await;
            blueprint_core::info!(target: "blueprint-runner", "Received graceful shutdown signal. Calling shutdown handler");
            shutdown_handler.await;
        });

        poll_fn(|ctx| router.poll_ready(ctx)).await.unwrap_or(());

        let producers = producers.into_iter().map(|producer| {
            futures::stream::unfold(producer, |producer| async move {
                let result;
                {
                    let mut guard = producer.lock().await;
                    result = guard.next().await;
                }
                result.map(|job_call| (job_call, producer))
            })
            .boxed()
        });
        let mut producer_stream = futures::stream::select_all(producers);

        let mut background_services = if background_futures.is_empty() {
            futures::future::select_all(vec![Box::pin(futures::future::ready(Ok(())))
                as Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>])
        } else {
            futures::future::select_all(background_futures)
        };

        let mut pending_jobs = FuturesUnordered::new();

        let bridge = env.bridge().await.map_err(|e| {
            error!("[FATAL] Unable to establish bridge connection, aborting runner: {e}");
            e
        })?;
        bridge.ping().await.map_err(|e| {
            error!("[FATAL] Unable to establish bridge connection, aborting runner: {e}");
            e
        })?;

        loop {
            tokio::select! {
                // Receive job calls from producer
                producer_result = producer_stream.next() => {
                    match producer_result {
                        Some(Ok(job_call)) => {
                            blueprint_core::trace!(
                                target: "blueprint-runner",
                                ?job_call,
                                "Received a job call"
                            );
                            pending_jobs.push(tokio::task::spawn(router.call(job_call)));
                        },
                        Some(Err(e)) => {
                            blueprint_core::error!(target: "blueprint-runner", "Producer error: {:?}", e);
                            let _ = shutdown_tx.send(true);
                            return Err(ProducerError::Failed(e).into());
                        },
                        None => {
                            blueprint_core::error!(target: "blueprint-runner", "Producer stream ended unexpectedly");
                            let _ = shutdown_tx.send(true);
                            return Err(ProducerError::StreamEnded.into());
                        }
                    }
                },

                // Job call finished
                Some(job_result) = pending_jobs.next() => {
                    match job_result {
                        Ok(Ok(Some(results))) => {
                            blueprint_core::trace!(
                                target: "blueprint-runner",
                                count = %results.len(),
                                "Job call(s) processed by router"
                            );
                            let result_stream = stream::iter(results.into_iter().map(Ok));

                            // Broadcast results to all consumers
                            let send_futures = consumers.iter_mut().map(|consumer| {
                                let mut stream_clone = result_stream.clone();
                                async move {
                                    let mut guard = consumer.lock().await;
                                    guard.send_all(&mut stream_clone).await
                                }
                            });

                            let result = futures::future::try_join_all(send_futures).await;
                            blueprint_core::trace!(
                                target: "blueprint-runner",
                                results = ?result.as_ref().map(|_| "success"),
                                "Job call results were broadcast to consumers"
                            );
                            if let Err(e) = result {
                                let _ = shutdown_tx.send(true);
                                return Err(Error::Consumer(e));
                            }
                        },
                        Ok(Ok(None)) => {
                            blueprint_core::debug!(target: "blueprint-runner", "Job call was ignored by router");
                        },
                        Ok(Err(e)) => {
                            blueprint_core::error!(target: "blueprint-runner", "Job call task failed: {:?}", e);
                            return Err(JobCallError::JobFailed(e).into());
                        },
                        Err(e) => {
                            blueprint_core::error!(target: "blueprint-runner", "Job call failed: {:?}", e);
                            let _ = shutdown_tx.send(true);
                            return Err(JobCallError::JobDidntFinish(e).into());
                        },
                    }
                }

                // Background service status updates
                result = &mut background_services => {
                    let (result, _, remaining_background_services) = result;
                    match result {
                        Ok(()) => {
                            if has_background_services {
                                blueprint_core::warn!(target: "blueprint-runner", "A background service has finished running");
                            }
                        },
                        Err(e) => {
                            blueprint_core::error!(target: "blueprint-runner", "A background service failed: {:?}", e);
                            let _ = shutdown_tx.send(true);
                            return Err(e);
                        }
                    }

                    if remaining_background_services.is_empty() {
                        if has_background_services {
                            blueprint_core::warn!(target: "blueprint-runner", "All background services have ended");
                        }
                        continue;
                    }

                    background_services = futures::future::select_all(remaining_background_services);
                }

                // Shutdown handler run, we're done
                () = shutdown_tx.closed() => {
                    break;
                }
            }
        }

        Ok(())
    }
}
