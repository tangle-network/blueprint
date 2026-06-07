//! Interop seam toward the in-enclave hardware-quote verifier.
//!
//! This module exists so that deep enclave-quote verification (AMD SEV-SNP /
//! Intel TDX DCAP) is **not duplicated** here. The `blueprint-tee` crate owns
//! that logic behind [`blueprint_tee::AttestationVerifier`]. When a caller has
//! fetched raw hardware evidence (e.g. a forwarded SEV-SNP report or TDX quote)
//! and wants the deep check, this seam converts the fetched material into a
//! [`blueprint_tee::AttestationReport`] and runs it through the supplied
//! verifier — keeping a single source of truth for quote verification.
//!
//! Only compiled under the `tee-attestation-seam` feature, which pulls in
//! `blueprint-tee`. The cloud-token (JWT) and COSE paths in this crate do not
//! depend on it.

use super::AttestationError;
use crate::core::remote::CloudProvider;
use blueprint_tee::{
    AttestationClaims, AttestationFormat, AttestationReport, AttestationVerifier, Measurement,
    TeeProvider, VerifiedAttestation,
};

/// Map a cloud provider to the `blueprint-tee` TEE provider identity.
pub fn tee_provider_for(provider: &CloudProvider) -> Option<TeeProvider> {
    match provider {
        CloudProvider::AWS => Some(TeeProvider::AwsNitro),
        CloudProvider::Azure => Some(TeeProvider::AzureSnp),
        CloudProvider::GCP => Some(TeeProvider::GcpConfidential),
        _ => None,
    }
}

/// Build a `blueprint_tee::AttestationReport` from raw provider evidence so it
/// can be handed to a deep [`AttestationVerifier`].
///
/// `measurement_hex` is the primary measurement (PCR0 / launch measurement) when
/// known; `issued_at_unix` the report issue time.
pub fn report_from_evidence(
    provider: &CloudProvider,
    format: AttestationFormat,
    evidence: Vec<u8>,
    measurement_hex: &str,
    issued_at_unix: u64,
    claims: AttestationClaims,
) -> Result<AttestationReport, AttestationError> {
    let tee_provider =
        tee_provider_for(provider).ok_or_else(|| AttestationError::Unsatisfiable {
            provider: provider.clone(),
            reason: "no blueprint-tee provider mapping".to_string(),
        })?;
    Ok(AttestationReport {
        provider: tee_provider,
        format,
        issued_at_unix,
        measurement: Measurement::sha384(measurement_hex),
        public_key_binding: None,
        claims,
        evidence,
    })
}

/// Run a fetched report through `blueprint-tee`'s deep hardware-quote verifier.
///
/// This is the delegation point for SEV-SNP / TDX DCAP verification: rather than
/// reimplementing quote verification here, hand the report to the crate that
/// owns it. Returns the `blueprint-tee` [`VerifiedAttestation`] on success.
pub fn delegate_to_hardware_verifier(
    report: &AttestationReport,
    verifier: &dyn AttestationVerifier,
) -> Result<VerifiedAttestation, AttestationError> {
    verifier
        .verify(report)
        .map_err(|e| AttestationError::Signature {
            provider: provider_from_tee(report.provider),
            reason: format!("hardware-quote verification failed: {e}"),
        })
}

fn provider_from_tee(p: TeeProvider) -> CloudProvider {
    match p {
        TeeProvider::AwsNitro => CloudProvider::AWS,
        TeeProvider::AzureSnp => CloudProvider::Azure,
        TeeProvider::GcpConfidential => CloudProvider::GCP,
        // Bare-metal SNP/TDX have no cloud-provider identity; label as AWS-less.
        _ => CloudProvider::AWS,
    }
}
