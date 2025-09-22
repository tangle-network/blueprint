//! Kubernetes provider implementation
//!
//! Provides deployment to existing Kubernetes clusters (Generic CloudProvider).
//! Unlike cloud providers, Kubernetes doesn't need provisioning - assumes cluster exists.

pub mod adapter;

pub use adapter::KubernetesAdapter;