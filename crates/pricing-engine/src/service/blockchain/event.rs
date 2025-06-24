use blueprint_core::error;
use tangle_subxt::{
    subxt::{Config, events::Events},
    tangle_testnet_runtime::api::services::events::{
        Registered, RpcAddressUpdated, ServiceInitiated, ServiceRequestApproved,
        ServiceRequestRejected, ServiceRequested, ServiceTerminated, Unregistered,
    },
};

/// Events from the blockchain that are relevant to the pricing engine
#[derive(Debug, Clone)]
pub enum BlockchainEvent {
    /// An operator has been registered for a service blueprint
    Registered(Registered),

    /// An operator has been unregistered
    Unregistered(Unregistered),

    /// An operator's RPC address has been updated
    RpcAddressUpdated(RpcAddressUpdated),

    /// A new service has been requested
    ServiceRequested(ServiceRequested),

    /// A service request has been approved
    ServiceRequestApproved(ServiceRequestApproved),

    /// A service request has been rejected
    ServiceRequestRejected(ServiceRequestRejected),

    /// A service has been initiated
    ServiceInitiated(ServiceInitiated),

    /// A service has been terminated
    ServiceTerminated(ServiceTerminated),

    /// Unknown event type (for testing and handling unexpected events)
    Unknown(String),
}

pub async fn handle_events<T: Config>(events: Events<T>) -> Vec<BlockchainEvent> {
    let mut blockchain_events = Vec::new();

    for event in events.find::<Registered>() {
        match event {
            Ok(event) => blockchain_events.push(BlockchainEvent::Registered(event)),
            Err(e) => error!("Error processing Registered event: {}", e),
        }
    }

    for event in events.find::<Unregistered>() {
        match event {
            Ok(event) => blockchain_events.push(BlockchainEvent::Unregistered(event)),
            Err(e) => error!("Error processing Unregistered event: {}", e),
        }
    }

    for event in events.find::<RpcAddressUpdated>() {
        match event {
            Ok(event) => blockchain_events.push(BlockchainEvent::RpcAddressUpdated(event)),
            Err(e) => error!("Error processing RpcAddressUpdated event: {}", e),
        }
    }

    for event in events.find::<ServiceRequested>() {
        match event {
            Ok(event) => blockchain_events.push(BlockchainEvent::ServiceRequested(event)),
            Err(e) => error!("Error processing ServiceRequested event: {}", e),
        }
    }

    for event in events.find::<ServiceRequestApproved>() {
        match event {
            Ok(event) => blockchain_events.push(BlockchainEvent::ServiceRequestApproved(event)),
            Err(e) => error!("Error processing ServiceRequestApproved event: {}", e),
        }
    }

    for event in events.find::<ServiceRequestRejected>() {
        match event {
            Ok(event) => blockchain_events.push(BlockchainEvent::ServiceRequestRejected(event)),
            Err(e) => error!("Error processing ServiceRequestRejected event: {}", e),
        }
    }

    for event in events.find::<ServiceInitiated>() {
        match event {
            Ok(event) => blockchain_events.push(BlockchainEvent::ServiceInitiated(event)),
            Err(e) => error!("Error processing ServiceInitiated event: {}", e),
        }
    }

    for event in events.find::<ServiceTerminated>() {
        match event {
            Ok(event) => blockchain_events.push(BlockchainEvent::ServiceTerminated(event)),
            Err(e) => error!("Error processing ServiceTerminated event: {}", e),
        }
    }

    blockchain_events
}
