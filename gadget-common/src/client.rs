use crate::debug_logger::DebugLogger;
use crate::keystore::{ECDSAKeyStore, KeystoreBackend};
use async_trait::async_trait;
use gadget_core::gadget::substrate::Client;
use pallet_jobs_rpc_runtime_api::JobsApi;
use sc_client_api::{Backend, BlockImportNotification, BlockchainEvents, FinalityNotification};
use sp_api::BlockT as Block;
use sp_api::ProvideRuntimeApi;
use std::sync::Arc;
use tangle_primitives::jobs::RpcResponseJobsData;
use tangle_primitives::jobs::{JobId, JobKey, JobResult};

pub struct MpEcdsaClient<B: Block, BE, KBE: KeystoreBackend, C> {
    client: Arc<C>,
    pub key_store: ECDSAKeyStore<KBE>,
    logger: DebugLogger,
    _block: std::marker::PhantomData<(BE, B)>,
}

impl<B: Block, BE, KBE: KeystoreBackend, C> Clone for MpEcdsaClient<B, BE, KBE, C> {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            key_store: self.key_store.clone(),
            logger: self.logger.clone(),
            _block: self._block,
        }
    }
}

pub async fn create_client<
    B: Block,
    BE: Backend<B>,
    KBE: KeystoreBackend,
    C: ClientWithApi<B, BE>,
>(
    client: Arc<C>,
    logger: DebugLogger,
    key_store: ECDSAKeyStore<KBE>,
) -> MpEcdsaClient<B, BE, KBE, C>
where
    <C as ProvideRuntimeApi<B>>::Api: JobsApi<B, AccountId>,
{
    MpEcdsaClient {
        client,
        key_store,
        logger,
        _block: std::marker::PhantomData,
    }
}

pub type AccountId = sp_core::ecdsa::Public;

pub trait ClientWithApi<B, BE>:
    BlockchainEvents<B> + ProvideRuntimeApi<B> + Send + Sync + Client<B> + 'static
where
    B: Block,
    BE: Backend<B>,
    <Self as ProvideRuntimeApi<B>>::Api: JobsApi<B, AccountId>,
{
}

impl<B, BE, T> ClientWithApi<B, BE> for T
where
    B: Block,
    BE: Backend<B>,
    T: BlockchainEvents<B> + ProvideRuntimeApi<B> + Send + Sync + Client<B> + 'static,
    <T as ProvideRuntimeApi<B>>::Api: JobsApi<B, AccountId>,
{
}

impl<B: Block, BE: Backend<B>, KBE: KeystoreBackend, C: ClientWithApi<B, BE>>
    MpEcdsaClient<B, BE, KBE, C>
where
    <C as ProvideRuntimeApi<B>>::Api: JobsApi<B, AccountId>,
{
    pub async fn query_jobs_by_validator(
        &self,
        at: <B as Block>::Hash,
        validator: AccountId,
    ) -> Result<Vec<RpcResponseJobsData<AccountId>>, crate::Error> {
        exec_client_function(&self.client, move |client| {
            client.runtime_api().query_jobs_by_validator(at, validator)
        })
        .await
        .map_err(|err| crate::Error::ClientError {
            err: format!("Failed to query jobs by validator: {err:?}"),
        })?
        .map_err(|err| crate::Error::ClientError {
            err: format!("Failed to query jobs by validator: {err:?}"),
        })
    }

    // TODO: Proper implementation
    pub async fn submit_job_result(
        &self,
        _job_key: JobKey,
        _job_id: JobId,
        _result: JobResult,
    ) -> Result<(), crate::Error> {
        Ok(())
    }
}

#[async_trait]
impl<B, BE, KBE, C> Client<B> for MpEcdsaClient<B, BE, KBE, C>
where
    B: Block,
    BE: Backend<B>,
    C: ClientWithApi<B, BE>,
    KBE: KeystoreBackend,
    <C as ProvideRuntimeApi<B>>::Api: JobsApi<B, AccountId>,
{
    async fn get_next_finality_notification(&self) -> Option<FinalityNotification<B>> {
        self.client.get_next_finality_notification().await
    }

    async fn get_latest_finality_notification(&self) -> Option<FinalityNotification<B>> {
        self.client.get_latest_finality_notification().await
    }

    async fn get_next_block_import_notification(&self) -> Option<BlockImportNotification<B>> {
        self.client.get_next_block_import_notification().await
    }
}

async fn exec_client_function<C, F, T>(client: &Arc<C>, function: F) -> T
where
    for<'a> F: FnOnce(&'a C) -> T,
    C: Send + Sync + 'static,
    T: Send + 'static,
    F: Send + 'static,
{
    let client = client.clone();
    tokio::task::spawn_blocking(move || function(&client))
        .await
        .expect("Failed to spawn blocking task")
}
