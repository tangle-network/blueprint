// SSH provider implementation stub
// TODO: Implement SSH-based deployment for bare metal infrastructure

use crate::error::Result;

pub struct SshProvider {
    name: String,
}

impl SshProvider {
    pub async fn new(name: impl Into<String>) -> Result<Self> {
        Ok(Self {
            name: name.into(),
        })
    }
}