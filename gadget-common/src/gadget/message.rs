use crate::client::AccountId;
use crate::gadget::work_manager::WebbWorkManager;
use gadget_core::job_manager::{ProtocolMessageMetadata, WorkManagerInterface};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GadgetProtocolMessage {
    pub associated_block_id: <WebbWorkManager as WorkManagerInterface>::Clock,
    pub associated_session_id: <WebbWorkManager as WorkManagerInterface>::SessionID,
    pub associated_retry_id: <WebbWorkManager as WorkManagerInterface>::RetryID,
    // A unique marker for the associated task this message belongs to
    pub task_hash: <WebbWorkManager as WorkManagerInterface>::TaskID,
    pub from: UserID,
    // If None, this is a broadcasted message
    pub to: Option<UserID>,
    pub payload: Vec<u8>,
    // Some protocols need this additional data
    pub from_network_id: Option<AccountId>,
    // Some protocol need this additional data
    pub to_network_id: Option<AccountId>,
}

pub type UserID = u32;

impl ProtocolMessageMetadata<WebbWorkManager> for GadgetProtocolMessage {
    fn associated_block_id(&self) -> <WebbWorkManager as WorkManagerInterface>::Clock {
        self.associated_block_id
    }

    fn associated_session_id(&self) -> <WebbWorkManager as WorkManagerInterface>::SessionID {
        self.associated_session_id
    }

    fn associated_retry_id(&self) -> <WebbWorkManager as WorkManagerInterface>::RetryID {
        self.associated_retry_id
    }

    fn associated_task(&self) -> <WebbWorkManager as WorkManagerInterface>::TaskID {
        self.task_hash
    }

    fn associated_sender_user_id(&self) -> <WebbWorkManager as WorkManagerInterface>::UserID {
        self.from
    }

    fn associated_recipient_user_id(
        &self,
    ) -> Option<<WebbWorkManager as WorkManagerInterface>::UserID> {
        self.to
    }
}

impl GadgetProtocolMessage {
    /// Returns a hash of the message
    pub fn hash(&self) -> Vec<u8> {
        let serialized = bincode2::serialize(&self).expect("Should serialize");
        sp_core::keccak_256(&serialized).to_vec()
    }
}
