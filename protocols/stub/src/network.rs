use async_trait::async_trait;
use gadget_common::config::Network;
use gadget_common::gadget::work_manager::WebbWorkManager;
use gadget_common::{Error, WorkManagerInterface};

#[derive(Clone)]
pub struct StubNetworkService;

#[async_trait]
impl Network for StubNetworkService {
    async fn next_message(
        &self,
    ) -> Option<<WebbWorkManager as WorkManagerInterface>::ProtocolMessage> {
        futures::future::pending().await
    }

    async fn send_message(
        &self,
        _message: <WebbWorkManager as WorkManagerInterface>::ProtocolMessage,
    ) -> Result<(), Error> {
        Ok(())
    }
}
