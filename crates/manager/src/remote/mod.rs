//! Remote deployment integration for Blueprint Manager.
//!
//! This module extends Blueprint Manager to support remote cloud deployments
//! using the configured deployment policies.

pub mod blueprint_analyzer;
pub mod blueprint_fetcher;
pub mod policy_loader;
pub mod pricing_service;
pub mod provider_selector;
pub mod serverless;
pub mod service;

#[cfg(test)]
mod integration_test;

pub use blueprint_analyzer::{BlueprintAnalysis, DeploymentStrategy, FaasLimits, analyze_blueprint};
pub use blueprint_fetcher::{BlueprintMetadata, fetch_blueprint_metadata};
pub use policy_loader::{DeploymentPolicy, load_policy};
pub use pricing_service::{OperatorPricingService, PricingQuote, ProviderCost};
pub use provider_selector::{DeploymentTarget, ProviderSelector};
pub use serverless::{FaasProviderConfig, ServerlessConfig, deploy_serverless};
pub use service::RemoteDeploymentService;
