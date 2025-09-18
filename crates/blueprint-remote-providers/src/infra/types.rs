//! Types and structures for infrastructure provisioning

use crate::core::remote::CloudProvider;
use serde::{Deserialize, Serialize};

/// Provisioned instance details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvisionedInstance {
    pub id: String,
    pub provider: CloudProvider,
    pub instance_type: String,
    pub region: String,
    pub public_ip: Option<String>,
    pub private_ip: Option<String>,
    pub status: InstanceStatus,
}

/// Instance status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InstanceStatus {
    Starting,
    Running,
    Stopping,
    Stopped,
    Terminated,
    Unknown,
}

/// Retry policy configuration
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    pub max_retries: usize,
    pub base_delay: blueprint_std::time::Duration,
    pub max_delay: blueprint_std::time::Duration,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay: blueprint_std::time::Duration::from_secs(1),
            max_delay: blueprint_std::time::Duration::from_secs(30),
        }
    }
}

impl RetryPolicy {
    pub fn delay_for_attempt(&self, attempt: usize) -> blueprint_std::time::Duration {
        let delay = self.base_delay * 2u32.pow(attempt as u32);
        delay.min(self.max_delay)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retry_policy() {
        let policy = RetryPolicy::default();

        assert_eq!(
            policy.delay_for_attempt(0),
            blueprint_std::time::Duration::from_secs(1)
        );
        assert_eq!(
            policy.delay_for_attempt(1),
            blueprint_std::time::Duration::from_secs(2)
        );
        assert_eq!(
            policy.delay_for_attempt(2),
            blueprint_std::time::Duration::from_secs(4)
        );
        assert_eq!(
            policy.delay_for_attempt(5),
            blueprint_std::time::Duration::from_secs(30)
        ); // Max delay
    }
}