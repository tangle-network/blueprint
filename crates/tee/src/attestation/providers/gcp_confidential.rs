//! GCP Confidential Space attestation verifier.
//!
//! `verify_token` performs cryptographic verification of the Confidential Space
//! OIDC attestation token against Google's JWKS plus nonce/audience/workload
//! policy checks. The synchronous [`AttestationVerifier`] implementation remains
//! a structural check for already-materialized reports.

use crate::attestation::providers::jwt::{JwksSource, verify_jwt_attestation};
use crate::attestation::report::{AttestationFormat, AttestationReport};
use crate::attestation::verifier::{AttestationVerifier, VerifiedAttestation};
use crate::attestation::{AttestationError, AttestationPolicy};
use crate::config::TeeProvider;
use crate::errors::TeeError;

/// Issuer of GCP Confidential Space attestation tokens.
pub const GCP_CONFIDENTIAL_ISSUER: &str = "https://confidentialcomputing.googleapis.com";

/// Google's JWKS endpoint for Confidential Space attestation tokens.
pub const GCP_CONFIDENTIAL_JWKS_URL: &str =
    "https://confidentialcomputing.googleapis.com/.well-known/jwks";

/// Verifier for GCP Confidential Space attestation tokens.
pub struct GcpConfidentialVerifier {
    /// Expected measurement digest, if enforced.
    pub expected_measurement: Option<String>,
    /// Whether to allow debug-mode VMs.
    pub allow_debug: bool,
    http: reqwest::Client,
    jwks: JwksSource,
    allowed_issuers: Vec<String>,
}

impl GcpConfidentialVerifier {
    /// Create a new GCP Confidential Space verifier.
    pub fn new() -> Self {
        Self {
            expected_measurement: None,
            allow_debug: false,
            http: default_http_client(),
            jwks: JwksSource::new(GCP_CONFIDENTIAL_JWKS_URL),
            allowed_issuers: vec![GCP_CONFIDENTIAL_ISSUER.to_string()],
        }
    }

    /// Set the expected measurement.
    pub fn with_expected_measurement(mut self, measurement: impl Into<String>) -> Self {
        self.expected_measurement = Some(measurement.into());
        self
    }

    /// Allow debug-mode VMs (not recommended for production).
    pub fn allow_debug(mut self, allow: bool) -> Self {
        self.allow_debug = allow;
        self
    }

    /// Override the JWKS URL, primarily for tests.
    pub fn with_jwks_url(mut self, url: impl Into<String>) -> Self {
        self.jwks = JwksSource::new(url);
        self
    }

    /// Override allowed token issuers, primarily for tests.
    pub fn with_allowed_issuers(mut self, issuers: Vec<String>) -> Self {
        self.allowed_issuers = issuers;
        self
    }

    /// Verify a Confidential Space OIDC attestation token cryptographically.
    pub async fn verify_token(
        &self,
        token: &str,
        policy: &AttestationPolicy,
    ) -> Result<VerifiedAttestation, AttestationError> {
        let mut policy = policy.clone();
        if self.allow_debug {
            policy.allow_debug = true;
        }
        if policy.expected_measurement.is_none() {
            policy.expected_measurement = self.expected_measurement.clone();
        }
        let issuers: Vec<&str> = self.allowed_issuers.iter().map(String::as_str).collect();
        verify_jwt_attestation(
            TeeProvider::GcpConfidential,
            AttestationFormat::GcpConfidentialToken,
            token,
            &self.jwks,
            &issuers,
            &policy,
            &self.http,
        )
        .await
    }
}

impl Default for GcpConfidentialVerifier {
    fn default() -> Self {
        Self::new()
    }
}

impl AttestationVerifier for GcpConfidentialVerifier {
    /// Verify a GCP Confidential Space attestation report.
    ///
    /// # Security Warning
    ///
    /// This verifier performs structural validation only (provider match,
    /// measurement, debug mode). It does **not** verify token signatures
    /// or workload identity.
    fn verify(&self, report: &AttestationReport) -> Result<VerifiedAttestation, TeeError> {
        if report.provider != TeeProvider::GcpConfidential {
            return Err(TeeError::AttestationVerification(format!(
                "expected GCP Confidential provider, got {}",
                report.provider
            )));
        }

        if report.claims.debug_mode && !self.allow_debug {
            return Err(TeeError::AttestationVerification(
                "debug mode VMs are not permitted".to_string(),
            ));
        }

        if let Some(expected) = &self.expected_measurement {
            if report.measurement.digest != *expected {
                return Err(TeeError::MeasurementMismatch {
                    expected: expected.clone(),
                    actual: report.measurement.digest.clone(),
                });
            }
        }

        tracing::debug!(
            "structural validation only; call verify_token() for cryptographic JWT verification"
        );

        Ok(VerifiedAttestation::new(
            report.clone(),
            TeeProvider::GcpConfidential,
        ))
    }

    fn supported_provider(&self) -> TeeProvider {
        TeeProvider::GcpConfidential
    }
}

fn default_http_client() -> reqwest::Client {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .unwrap_or_default()
}
