use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};
use std::time::Duration;

use alloy_primitives::Address;
use alloy_rpc_types::{Filter, Log};
use alloy_sol_types::SolEvent;
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use blueprint_client_tangle_evm::{
    TangleEvmClient,
    contracts::{ITangle, ITangleTypes},
};
use blueprint_core::{info, warn};
use tokio::sync::mpsc::Sender;
use tokio::time::sleep;

use crate::service::blockchain::event::BlockchainEvent;

#[async_trait]
pub trait EvmEventClient: Send + Sync {
    fn contract_address(&self) -> Address;
    async fn get_logs(&self, filter: &Filter) -> Result<Vec<Log>>;
    async fn get_service(&self, service_id: u64) -> Result<ITangleTypes::Service>;
}

#[async_trait]
impl EvmEventClient for Arc<TangleEvmClient> {
    fn contract_address(&self) -> Address {
        self.config.settings.tangle_contract
    }

    async fn get_logs(&self, filter: &Filter) -> Result<Vec<Log>> {
        TangleEvmClient::get_logs(self, filter)
            .await
            .map_err(|e| anyhow!(e.to_string()))
    }

    async fn get_service(&self, service_id: u64) -> Result<ITangleTypes::Service> {
        TangleEvmClient::get_service(self, service_id)
            .await
            .map_err(|e| anyhow!(e.to_string()))
    }
}

pub struct EvmEventListener<C = Arc<TangleEvmClient>>
where
    C: EvmEventClient + Clone + Send + Sync + 'static,
{
    client: C,
    event_tx: Sender<BlockchainEvent>,
    last_block: AtomicU64,
    poll_interval: Duration,
}

impl EvmEventListener<Arc<TangleEvmClient>> {
    pub fn new(client: Arc<TangleEvmClient>, event_tx: Sender<BlockchainEvent>) -> Self {
        Self::with_client(client, event_tx)
    }
}

impl<C> EvmEventListener<C>
where
    C: EvmEventClient + Clone + Send + Sync + 'static,
{
    pub fn with_client(client: C, event_tx: Sender<BlockchainEvent>) -> Self {
        Self {
            client,
            event_tx,
            last_block: AtomicU64::new(0),
            poll_interval: Duration::from_secs(5),
        }
    }

    pub async fn run(&self) -> Result<()> {
        info!("Starting EVM event listener");
        loop {
            if let Err(e) = self.poll_once().await {
                warn!("EVM listener poll error: {e:?}");
            }
            sleep(self.poll_interval).await;
        }
    }

    pub async fn poll_once(&self) -> Result<()> {
        let from_block = self.last_block.load(Ordering::Relaxed);
        let filter = Filter::new()
            .address(self.client.contract_address())
            .from_block(from_block)
            .events([
                ITangle::ServiceActivated::SIGNATURE_HASH,
                ITangle::ServiceTerminated::SIGNATURE_HASH,
            ]);

        let logs = self.client.get_logs(&filter).await?;
        for log in logs {
            if let Some(block) = log.block_number {
                self.last_block.store(block + 1, Ordering::Relaxed);
            }

            if let Some(event) = self.decode_event(&log).await {
                if self.event_tx.send(event).await.is_err() {
                    warn!("Event channel closed; stopping listener");
                    return Ok(());
                }
            }
        }
        Ok(())
    }

    async fn decode_event(&self, log: &Log) -> Option<BlockchainEvent> {
        if let Ok(event) = log.log_decode::<ITangle::ServiceActivated>() {
            let service_id = event.inner.serviceId;
            match self.client.get_service(service_id).await {
                Ok(service) => {
                    return Some(BlockchainEvent::ServiceActivated {
                        service_id,
                        blueprint_id: service.blueprintId,
                    });
                }
                Err(e) => warn!("Failed to fetch service {}: {}", service_id, e),
            }
        }

        if let Ok(event) = log.log_decode::<ITangle::ServiceTerminated>() {
            return Some(BlockchainEvent::ServiceTerminated {
                service_id: event.inner.serviceId,
            });
        }

        None
    }
}
