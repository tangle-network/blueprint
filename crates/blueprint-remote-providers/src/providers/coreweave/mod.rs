//! Kubernetes GPU leases with caller-owned Kubernetes credentials.
//!
//! `with_client` accepts an authenticated tenant client; `with_credentials`
//! uses the caller's kubeconfig server/CA with the supplied token. The
//! `kubernetes` feature is required. Allocation creates a Pod and an SSH
//! LoadBalancer service; termination confirms both resources are gone.
//!
//! Public node and dedicated-inference prices do not establish a tenant Pod's
//! account price. [`crate::pricing::dated_gpu_rates`] records those unknowns
//! and their sources. Supply exact all-in account contract prices through
//! `with_prices`; unpriced allocations cannot enter provider selection.

mod adapter;
mod instance_mapper;

pub use adapter::CoreWeaveAdapter;
pub use instance_mapper::CoreWeaveInstanceMapper;
