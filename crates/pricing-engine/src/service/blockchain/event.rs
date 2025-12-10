#[derive(Debug, Clone)]
pub enum BlockchainEvent {
    ServiceActivated { service_id: u64, blueprint_id: u64 },
    ServiceTerminated { service_id: u64 },
}
