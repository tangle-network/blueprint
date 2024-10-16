use crate::config::GadgetConfiguration;
use crate::event_listener::markers;
use crate::events_watcher::InitializableEventHandler;
use crate::keystore::sp_core_subxt::Pair;
use crate::{info, tx};
use std::future::Future;
use std::pin::Pin;
use tangle_subxt::tangle_testnet_runtime::api;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::sp_core::ecdsa;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::PriceTargets;

pub struct MultiJobRunner {
    pub(crate) enqueued_job_runners: EnqueuedJobRunners,
    pub(crate) env: GadgetConfiguration<parking_lot::RawRwLock>,
}

pub type EnqueuedJobRunners = Vec<
    Pin<
        Box<
            dyn SendFuture<
                Output = Option<tokio::sync::oneshot::Receiver<Result<(), crate::Error>>>,
            >,
        >,
    >,
>;

pub trait SendFuture: Send + Future + 'static {}
impl<T: Send + Future + 'static> SendFuture for T {}

pub struct JobBuilder<'a, K> {
    pub(crate) register_call: Option<RegisterCall>,
    runner: &'a mut MultiJobRunner,
    _pd: std::marker::PhantomData<K>,
}

pub(crate) type RegisterCall = Pin<Box<dyn SendFuture<Output = Result<(), crate::Error>>>>;

impl<K: InitializableEventHandler + Send + 'static> JobBuilder<'_, K> {
    pub fn with_registration<'a, Fut: SendFuture<Output = Result<(), crate::Error>>, Input: 'a>(
        &'a mut self,
        context: Input,
        register_call: fn(Input) -> Fut,
    ) -> &'a mut Self {
        let future = register_call(context);
        self.register_call = Some(Box::pin(future));
        self
    }

    pub fn with_price_targets(&mut self, price_targets: PriceTargets) -> &mut Self
    where
        K: markers::IsTangle,
    {
        self.with_registration(
            (self.runner.env.clone(), price_targets),
            tangle_registration,
        )
    }

    pub fn with_default_price_targets(&mut self) -> &mut Self
    where
        K: markers::IsTangle,
    {
        self.with_price_targets(PriceTargets {
            cpu: 0,
            mem: 0,
            storage_hdd: 0,
            storage_ssd: 0,
            storage_nvme: 0,
        })
    }

    pub fn finish(&mut self, job_runner: K) -> &mut MultiJobRunner {
        let registration = self.register_call.take();
        let test_mode = self.runner.env.test_mode;

        let task = Box::pin(async move {
            if let Some(registration) = registration {
                // Skip registration if in test mode
                if !test_mode {
                    if let Err(err) = registration.await {
                        crate::error!("Failed to register job: {err:?}");
                        return None;
                    }
                }
            }

            job_runner.init_event_handler().await
        });
        self.runner.enqueued_job_runners.push(task);

        self.runner
    }
}

impl MultiJobRunner {
    pub fn new(env: &GadgetConfiguration<parking_lot::RawRwLock>) -> Self {
        Self {
            enqueued_job_runners: Vec::new(),
            env: env.clone(),
        }
    }

    /// Add a job to the job runner
    /// #[gadget_sdk::main(env)]
    /// async fn main() {
    ///     let x_square = blueprint::XsquareEventHandler {
    ///         service_id: env.service_id.unwrap(),
    ///         client: client.clone(),
    ///         signer,
    ///     };
    ///
    ///     let x_square2 = blueprint::XsquareEventHandler {
    ///         service_id: env.service_id.unwrap(),
    ///         client: client.clone(),
    ///         signer,
    ///     };
    ///
    ///     MultiJobRunner::new(&env)
    ///         .with_job()
    ///         .with_default_price_targets()
    ///         .finish(x_square)
    ///         .with_job()
    ///         .with_default_price_targets()
    ///         .finish(x_square2)
    ///         .run()
    ///         .await?;
    /// }
    pub fn with_job<K>(&mut self) -> JobBuilder<K> {
        JobBuilder {
            register_call: None,
            runner: self,
            _pd: Default::default(),
        }
    }

    pub async fn run(&mut self) -> Result<(), crate::Error> {
        if self.enqueued_job_runners.is_empty() {
            return Err(crate::Error::Other(
                "No jobs registered. Make sure to add a job with `MultiJobRunner::add_job` "
                    .to_string(),
            ));
        }

        let mut futures = Vec::new();
        let tasks = std::mem::take(&mut self.enqueued_job_runners);
        for job_runner in tasks {
            futures.push(job_runner);
        }

        let receivers = futures::future::join_all(futures).await;

        // For each receiver, take the value out of it and add it to an ordered futures list. If any values
        // is None, return an error stating that the job already initialized
        let mut ordered_futures = Vec::new();
        for receiver in receivers {
            let receiver = receiver
                .ok_or_else(|| crate::Error::Other("Job already initialized".to_string()))?;
            ordered_futures.push(receiver);
        }

        let res = futures::future::select_all(ordered_futures).await;
        let job_n = res.1;
        let err = res
            .0
            .map_err(|err| crate::Error::Other(err.to_string()))
            .map_err(|_err| {
                crate::Error::Other(format!("Job {job_n} exited prematurely (channel dropped)"))
            })?;

        Err(crate::Error::Other(format!(
            "Job {job_n} exited prematurely: {err:?}"
        )))
    }
}

async fn tangle_registration(
    this: (GadgetConfiguration<parking_lot::RawRwLock>, PriceTargets),
) -> Result<(), crate::Error> {
    let (this, price_targets) = this;
    let client = this
        .client()
        .await
        .map_err(|err| crate::Error::Other(err.to_string()))?;
    let signer = this
        .first_sr25519_signer()
        .map_err(|err| crate::Error::Other(err.to_string()))?;
    let ecdsa_pair = this
        .first_ecdsa_signer()
        .map_err(|err| crate::Error::Other(err.to_string()))?;

    let xt = api::tx().services().register(
        this.blueprint_id,
        services::OperatorPreferences {
            key: ecdsa::Public(ecdsa_pair.signer().public().0),
            approval: services::ApprovalPrefrence::None,
            price_targets,
        },
        Default::default(),
    );

    // send the tx to the tangle and exit.
    let result = tx::tangle::send(&client, &signer, &xt).await?;
    info!("Registered operator with hash: {:?}", result);
    Ok(())
}
