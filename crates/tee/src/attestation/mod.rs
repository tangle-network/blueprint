//! TEE attestation types and verification.
//!
//! This module provides the core attestation type system for TEE environments,
//! including typed attestation reports, claims, and a pluggable verifier trait.

pub mod claims;
pub mod providers;
pub mod report;
pub mod verifier;

pub use claims::AttestationClaims;
pub use report::{AttestationFormat, AttestationReport, Measurement, PublicKeyBinding};
pub use verifier::{AttestationVerifier, VerifiedAttestation};
