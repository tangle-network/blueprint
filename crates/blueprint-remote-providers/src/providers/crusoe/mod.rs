//! Crusoe Cloud — GPU VMs (L40S, A40, A100, H100).
//!
//! <https://docs.crusoe.ai/api>
//!
//! [`CrusoeManagedAdapter`] rents GPU leases from `ProviderCredentials::Crusoe`. The legacy
//! [`CrusoeAdapter`] reads `CRUSOE_API_KEY`, `CRUSOE_API_SECRET`, `CRUSOE_PROJECT_ID` and
//! `CRUSOE_REGION`.

mod adapter;
mod instance_mapper;
mod managed;

pub use adapter::CrusoeAdapter;
pub use instance_mapper::CrusoeInstanceMapper;
pub use managed::CrusoeManagedAdapter;
