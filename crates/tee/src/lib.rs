//! First-class TEE (Trusted Execution Environment) support for the Blueprint SDK.
//!
//! This crate provides runtime TEE capabilities including attestation verification,
//! key exchange, and middleware integration for blueprint services running in
//! confidential compute environments.
//!
//! # Overview
//!
//! Blueprint TEE supports multiple deployment modes:
//!
//! - **Direct**: The runner itself executes inside a TEE with device passthrough
//!   and hardened defaults.
//! - **Remote**: The runner provisions workloads in cloud TEE instances
//!   (AWS Nitro, Azure CVM, GCP Confidential Space).
//! - **Hybrid**: Selected jobs run in TEE runtimes while others run normally.
//!
//! # Quick Start
//!
//! ```rust,ignore
//! use blueprint_runner::BlueprintRunner;
//! use blueprint_tee::{TeeConfig, TeeMode, TeeRequirement};
//!
//! let tee = TeeConfig::builder()
//!     .requirement(TeeRequirement::Required)
//!     .mode(TeeMode::Direct)
//!     .build()?;
//!
//! BlueprintRunner::builder(config, env)
//!     .tee(tee)
//!     .router(router)
//!     .run()
//!     .await?;
//! ```
//!
//! # Features
#![doc = document_features::document_features!()]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod attestation;
pub mod config;
pub mod errors;
pub mod exchange;
pub mod middleware;
pub mod runtime;

// Re-exports
pub use config::{
    AttestationFreshnessPolicy, HybridRoutingSource, RuntimeLifecyclePolicy, SecretInjectionPolicy,
    TeeConfig, TeeConfigBuilder, TeeKeyExchangeConfig, TeeMode, TeeProvider, TeeProviderSelector,
    TeePublicKeyPolicy, TeeRequirement,
};
pub use errors::TeeError;

pub use attestation::{
    AttestationClaims, AttestationFormat, AttestationReport, AttestationVerifier, Measurement,
    PublicKeyBinding, VerifiedAttestation,
};

pub use exchange::TeeAuthService;

pub use middleware::{TeeContext, TeeLayer};

pub use runtime::{
    TeeDeployRequest, TeeDeploymentHandle, TeeDeploymentStatus, TeePublicKey, TeeRuntimeBackend,
};
