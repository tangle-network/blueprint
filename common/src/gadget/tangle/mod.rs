use crate::config::GadgetProtocol;
use crate::environments::{GadgetEnvironment, TangleEnvironment};
use crate::gadget::{EventHandler, GeneralModule, MetricizedJob};
use crate::prelude::{TangleProtocolMessage, TangleWorkManager};
use crate::tangle_runtime::AccountId32;
use async_trait::async_trait;
use gadget_core::gadget::general::Client;
use gadget_core::gadget::manager::AbstractGadget;
use gadget_core::gadget::substrate::FinalityNotification;
use gadget_core::job_manager::{PollMethod, WorkManagerInterface};
use sp_core::{ecdsa, keccak_256, sr25519};
use std::ops::Deref;
use std::sync::Arc;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::{jobs, roles};
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_testnet_runtime::{
    MaxAdditionalParamsLen, MaxParticipants, MaxSubmissionLen,
};
use crate::Network;

pub mod runtime;

/// Endow the general module with event-handler specific code tailored towards interacting with tangle
pub struct TangleGadget<N, M> {
    module: GeneralModule<N, M, TangleEnvironment>,
}

impl<N, M> Deref for TangleGadget<N, M> {
    type Target = GeneralModule<N, M, TangleEnvironment>;

    fn deref(&self) -> &Self::Target {
        &self.module
    }
}

impl<N, M> From<GeneralModule<N, M, TangleEnvironment>> for TangleGadget<N, M> {
    fn from(module: GeneralModule<N, M, TangleEnvironment>) -> Self {
        TangleGadget { module }
    }
}

#[async_trait]
impl<N: Send + Sync + Network<TangleEnvironment> + 'static, M: GadgetProtocol<TangleEnvironment> + Send + 'static> AbstractGadget
    for TangleGadget<N, M>
where
    Self: AbstractGadget<
        Event = <TangleEnvironment as GadgetEnvironment>::Event,
        Error = <TangleEnvironment as GadgetEnvironment>::Error,
        ProtocolMessage = <TangleEnvironment as GadgetEnvironment>::ProtocolMessage,
    >,
{
    type Event = TangleEvent;
    type ProtocolMessage = TangleProtocolMessage;
    type Error = crate::Error;

    async fn next_event(&self) -> Option<Self::Event> {
        self.protocol.client().client.next_event().await
    }

    async fn get_next_protocol_message(&self) -> Option<Self::ProtocolMessage> {
        self.network.next_message().await
    }

    async fn on_event_received(&self, notification: Self::Event) -> Result<(), Self::Error> {
        self.process_event(notification).await
    }

    async fn process_protocol_message(
        &self,
        message: Self::ProtocolMessage,
    ) -> Result<(), Self::Error> {
        self.job_manager
            .deliver_message(message)
            .map(|_| ())
            .map_err(|err| crate::Error::WorkManagerError { err })
    }

    async fn process_error(&self, _error: Self::Error) {}
}

pub type TangleEvent = FinalityNotification;
pub type TangleNetworkMessage = TangleProtocolMessage;

#[async_trait]
impl<N: Send + Sync + 'static, M: GadgetProtocol<TangleEnvironment> + Send + 'static>
    EventHandler<TangleEnvironment> for TangleGadget<N, M>
where
    Self: AbstractGadget<
        Event = TangleEvent,
        ProtocolMessage = TangleNetworkMessage,
        Error = crate::Error,
    >,
{
    async fn process_event(&self, notification: TangleEvent) -> Result<(), crate::Error> {
        let now: u64 = notification.number;
        *self.clock.write() = Some(now);
        self.protocol.logger().trace(format!(
            "Processing finality notification at block number {now}",
        ));

        let jobs = self
            .protocol
            .client()
            .query_jobs_by_validator(notification.hash, *self.protocol.account_id())
            .await?;

        self.protocol.logger().trace(format!(
            "Found {} potential job(s) for initialization",
            jobs.len()
        ));
        let mut relevant_jobs = Vec::new();

        for job in jobs {
            // Job is expired.
            if job.expiry < now {
                self.protocol.logger().trace(format!(
                    "[{}] The job requested for initialization is expired, skipping submission",
                    self.protocol.name()
                ));
                continue;
            }
            let role_type = match &job.job_type {
                jobs::JobType::DKGTSSPhaseOne(p) => roles::RoleType::Tss(p.role_type.clone()),
                jobs::JobType::DKGTSSPhaseTwo(p) => roles::RoleType::Tss(p.role_type.clone()),
                jobs::JobType::DKGTSSPhaseThree(p) => roles::RoleType::Tss(p.role_type.clone()),
                jobs::JobType::DKGTSSPhaseFour(p) => roles::RoleType::Tss(p.role_type.clone()),
                jobs::JobType::ZkSaaSPhaseOne(p) => roles::RoleType::ZkSaaS(p.role_type.clone()),
                jobs::JobType::ZkSaaSPhaseTwo(p) => roles::RoleType::ZkSaaS(p.role_type.clone()),
            };
            // Job is not for this role
            if !self.protocol.role_filter(role_type.clone()) {
                self.protocol.logger().trace(
                    format!(
                        "[{}] The job {} requested for initialization is not for this role {:?}, skipping submission",
                        self.protocol.name(),
                        job.job_id,
                        role_type
                    )
                );
                continue;
            }
            // Job is not for this phase
            if !self.protocol.phase_filter(job.job_type.clone()) {
                self.protocol.logger().trace(
                    format!(
                        "[{}] The job {} requested for initialization is not for this phase {:?}, skipping submission",
                        self.protocol.name(),
                        job.job_id,
                        job.job_type
                    )
                );
                continue;
            }

            let job_id = job.job_id;
            let task_id = job_id.to_be_bytes();
            let task_id = keccak_256(&task_id);
            if self.job_manager.job_exists(&task_id) {
                self.protocol.logger().trace(format!(
                    "[{}] The job {} is already running or enqueued, skipping submission",
                    self.protocol.name(),
                    job.job_id,
                ));
                continue;
            }

            let retry_id = self
                .job_manager
                .latest_retry_id(&task_id)
                .map(|r| r + 1)
                .unwrap_or(0);

            let is_phase_one = matches!(
                job.job_type,
                jobs::JobType::DKGTSSPhaseOne(_) | jobs::JobType::ZkSaaSPhaseOne(_)
            );

            let phase1_job = if is_phase_one {
                None
            } else {
                let phase_one_job_id = match &job.job_type {
                    jobs::JobType::DKGTSSPhaseOne(_) => unreachable!(),
                    jobs::JobType::ZkSaaSPhaseOne(_) => unreachable!(),
                    jobs::JobType::DKGTSSPhaseTwo(p) => p.phase_one_id,
                    jobs::JobType::DKGTSSPhaseThree(p) => p.phase_one_id,
                    jobs::JobType::DKGTSSPhaseFour(p) => p.phase_one_id,
                    jobs::JobType::ZkSaaSPhaseTwo(p) => p.phase_one_id,
                };
                let phase1_job = self
                    .protocol
                    .client()
                    .query_job_result(notification.hash, role_type.clone(), phase_one_job_id)
                    .await?
                    .ok_or_else(|| crate::Error::ClientError {
                        err: format!("Corresponding phase one job {phase_one_job_id} not found for phase two job {job_id}"),
                    })?;
                Some(phase1_job.job_type)
            };

            let participants = match phase1_job {
                Some(ref j) => match j {
                    jobs::JobType::DKGTSSPhaseOne(p) => p.participants.0.clone(),
                    jobs::JobType::ZkSaaSPhaseOne(p) => p.participants.0.clone(),
                    _ => unreachable!(),
                },
                None => match &job.job_type {
                    jobs::JobType::DKGTSSPhaseOne(p) => p.participants.0.clone(),
                    jobs::JobType::ZkSaaSPhaseOne(p) => p.participants.0.clone(),
                    _ => unreachable!(),
                },
            };

            let participants_role_ids = {
                let mut out = Vec::new();
                for p in participants {
                    let maybe_role_key = self
                        .protocol
                        .client()
                        .query_restaker_role_key(notification.hash, sr25519::Public(p.0))
                        .await?;
                    if let Some(role_key) = maybe_role_key {
                        out.push(role_key);
                    } else {
                        self.protocol.logger().warn(format!(
                            "Participant {p} not found in the restaker registry",
                        ));
                    }
                }
                out
            };
            relevant_jobs.push(JobInitMetadata {
                role_type,
                job_type: job.job_type,
                phase1_job,
                participants_role_ids,
                task_id,
                retry_id,
                now,
                job_id,
                at: notification.hash,
            });
        }

        for relevant_job in relevant_jobs {
            let task_id = relevant_job.task_id;
            let retry_id = relevant_job.retry_id;
            self.protocol.logger().info(format!(
                "Creating job for task {task_id} with retry id {retry_id}",
                task_id = hex::encode(task_id),
                retry_id = retry_id
            ));
            match self
                .protocol
                .create_next_job(relevant_job, &self.job_manager)
                .await
            {
                Ok(params) => {
                    match self
                        .protocol
                        .create(0, now, retry_id, task_id, params)
                        .await
                    {
                        Ok(job) => {
                            let (remote, protocol) = job;
                            let protocol = protocol.with_metrics();

                            if let Err(err) = self.job_manager.push_task(
                                task_id,
                                false,
                                Arc::new(remote),
                                protocol,
                            ) {
                                self.protocol
                                    .process_error(
                                        crate::Error::WorkManagerError { err },
                                        &self.job_manager,
                                    )
                                    .await;
                            }
                        }

                        Err(err) => {
                            self.protocol
                                .logger()
                                .error(format!("Failed to create async protocol: {err:?}"));
                        }
                    }
                }

                Err(crate::Error::ParticipantNotSelected { id, reason }) => {
                    self.protocol.logger().debug(format!("Participant {id} not selected for job {task_id} with retry id {retry_id} because {reason}", id = id, task_id = hex::encode(task_id), retry_id = retry_id, reason = reason));
                }

                Err(err) => {
                    self.protocol
                        .logger()
                        .error(format!("Failed to generate job parameters: {err:?}"));
                }
            }
        }

        // Poll jobs on each finality notification if we're using manual polling.
        // This helps synchronize the actions of nodes in the network
        if self.job_manager.poll_method() == PollMethod::Manual {
            self.job_manager.poll();
        }

        Ok(())
    }
}

pub struct JobInitMetadata {
    pub job_type:
        jobs::JobType<AccountId32, MaxParticipants, MaxSubmissionLen, MaxAdditionalParamsLen>,
    pub role_type: roles::RoleType,
    /// This value only exists if this is a stage2 job
    pub phase1_job: Option<
        jobs::JobType<AccountId32, MaxParticipants, MaxSubmissionLen, MaxAdditionalParamsLen>,
    >,
    pub participants_role_ids: Vec<ecdsa::Public>,
    pub task_id: <TangleWorkManager as WorkManagerInterface>::TaskID,
    pub retry_id: <TangleWorkManager as WorkManagerInterface>::RetryID,
    pub job_id: u64,
    pub now: <TangleWorkManager as WorkManagerInterface>::Clock,
    pub at: [u8; 32],
}
