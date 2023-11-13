use crate::work_manager::TestWorkManager;
use gadget_core::job_manager::{ProtocolMessageMetadata, WorkManagerInterface};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct TestProtocolMessage {
    pub payload: Vec<u8>,
    pub from: UserID,
    pub to: Option<UserID>,
    pub associated_block_id: <TestWorkManager as WorkManagerInterface>::Clock,
    pub associated_session_id: <TestWorkManager as WorkManagerInterface>::SessionID,
    pub associated_ssid: <TestWorkManager as WorkManagerInterface>::SSID,
    pub associated_task_id: <TestWorkManager as WorkManagerInterface>::TaskID,
}

pub type UserID = u32;

impl ProtocolMessageMetadata<TestWorkManager> for TestProtocolMessage {
    fn associated_block_id(&self) -> <TestWorkManager as WorkManagerInterface>::Clock {
        self.associated_block_id
    }

    fn associated_session_id(&self) -> <TestWorkManager as WorkManagerInterface>::SessionID {
        self.associated_session_id
    }

    fn associated_ssid(&self) -> <TestWorkManager as WorkManagerInterface>::SSID {
        self.associated_ssid
    }

    fn associated_task(&self) -> <TestWorkManager as WorkManagerInterface>::TaskID {
        self.associated_task_id
    }
}
