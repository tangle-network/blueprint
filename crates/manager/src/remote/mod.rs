//! Remote deployment integration for Blueprint Manager.
//!
//! This module extends Blueprint Manager to support remote cloud deployments
//! using the configured deployment policies.

pub mod provider_selector;
pub mod service;

#[cfg(test)]
mod integration_test;

pub use provider_selector::{DeploymentTarget, ProviderSelector};
pub use service::RemoteDeploymentService;
