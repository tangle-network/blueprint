//! Azure SEV-SNP / MAA attestation verifier.
//!
//! `verify_token` performs cryptographic verification of a Microsoft Azure
//! Attestation token against the MAA JWKS plus nonce/audience/measurement policy
//! checks. The synchronous [`AttestationVerifier`] implementation remains a
//! structural check for already-materialized reports.

use crate::attestation::providers::jwt::{JwksSource, verify_jwt_attestation};
use crate::attestation::report::{AttestationFormat, AttestationReport};
use crate::attestation::verifier::{AttestationVerifier, VerifiedAttestation};
use crate::attestation::{AttestationError, AttestationPolicy};
use crate::config::TeeProvider;
use crate::errors::TeeError;

/// Default shared MAA instance. Per-region instances are preferred in prod.
pub const DEFAULT_MAA_INSTANCE: &str = "https://sharedeus.eus.attest.azure.net";

/// Verifier for Azure SEV-SNP attestation reports.
pub struct AzureSnpVerifier {
    /// Expected measurement digest, if enforced.
    pub expected_measurement: Option<String>,
    /// Whether to allow debug-mode VMs.
    pub allow_debug: bool,
    http: reqwest::Client,
    jwks: JwksSource,
    allowed_issuers: Vec<String>,
}

impl AzureSnpVerifier {
    /// Create a new Azure SNP verifier.
    pub fn new() -> Self {
        Self::for_instance(DEFAULT_MAA_INSTANCE)
    }

    /// Construct against a specific MAA instance URL.
    pub fn for_instance(instance_url: &str) -> Self {
        let instance = instance_url.trim_end_matches('/').to_string();
        Self {
            expected_measurement: None,
            allow_debug: false,
            http: default_http_client(),
            jwks: JwksSource::new(format!("{instance}/certs")),
            allowed_issuers: vec![instance],
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

    /// Verify an Azure MAA token cryptographically.
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
            TeeProvider::AzureSnp,
            AttestationFormat::AzureMaaToken,
            token,
            &self.jwks,
            &issuers,
            &policy,
            &self.http,
        )
        .await
    }
}

impl Default for AzureSnpVerifier {
    fn default() -> Self {
        Self::new()
    }
}

impl AttestationVerifier for AzureSnpVerifier {
    /// Verify an Azure SEV-SNP attestation report.
    ///
    /// # Security Warning
    ///
    /// This verifier performs structural validation only (provider match,
    /// measurement, debug mode). It does **not** verify MAA token signatures
    /// or guest policy enforcement.
    fn verify(&self, report: &AttestationReport) -> Result<VerifiedAttestation, TeeError> {
        if report.provider != TeeProvider::AzureSnp {
            return Err(TeeError::AttestationVerification(format!(
                "expected Azure SNP provider, got {}",
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
            "structural validation only; call verify_token() for cryptographic MAA JWT verification"
        );

        Ok(VerifiedAttestation::new(
            report.clone(),
            TeeProvider::AzureSnp,
        ))
    }

    fn supported_provider(&self) -> TeeProvider {
        TeeProvider::AzureSnp
    }
}

fn default_http_client() -> reqwest::Client {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .unwrap_or_default()
}
