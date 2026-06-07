//! AWS Nitro Enclaves attestation verifier.
//!
//! `verify_document` performs cryptographic verification of an NSM COSE_Sign1
//! attestation document: CBOR parse, certificate-chain validation against the
//! pinned AWS Nitro root, ES384 COSE signature verification, PCR/nonce policy,
//! and freshness. The synchronous [`AttestationVerifier`] implementation keeps a
//! structural fallback for legacy reports without raw evidence.

use crate::attestation::policy::now_unix;
use crate::attestation::report::{AttestationFormat, AttestationReport, Measurement};
use crate::attestation::verifier::{AttestationVerifier, VerificationLevel, VerifiedAttestation};
use crate::attestation::{AttestationClaims, AttestationError, AttestationPolicy};
use crate::config::TeeProvider;
use crate::errors::TeeError;
use coset::{CborSerializable, CoseSign1, TaggedCborSerializable};
use std::collections::BTreeMap;
use x509_parser::prelude::*;

/// The AWS Nitro Enclaves Root-G1 certificate (PEM), pinned in-binary.
///
/// Published by AWS at <https://aws-nitro-enclaves.amazonaws.com/AWS_NitroEnclaves_Root-G1.zip>.
/// SHA-256 of this PEM: `6eb9688305e4bbca67f44b59c29a0661ae930f09b5945b5d1d9ae01125c8d6c0`.
pub const AWS_NITRO_ROOT_CERT_PEM: &str = "-----BEGIN CERTIFICATE-----\n\
MIICETCCAZagAwIBAgIRAPkxdWgbkK/hHUbMtOTn+FYwCgYIKoZIzj0EAwMwSTEL\n\
MAkGA1UEBhMCVVMxDzANBgNVBAoMBkFtYXpvbjEMMAoGA1UECwwDQVdTMRswGQYD\n\
VQQDDBJhd3Mubml0cm8tZW5jbGF2ZXMwHhcNMTkxMDI4MTMyODA1WhcNNDkxMDI4\n\
MTQyODA1WjBJMQswCQYDVQQGEwJVUzEPMA0GA1UECgwGQW1hem9uMQwwCgYDVQQL\n\
DANBV1MxGzAZBgNVBAMMEmF3cy5uaXRyby1lbmNsYXZlczB2MBAGByqGSM49AgEG\n\
BSuBBAAiA2IABPwCVOumCMHzaHDimtqQvkY4MpJzbolL//Zy2YlES1BR5TSksfbb\n\
48C8WBoyt7F2Bw7eEtaaP+ohG2bnUs990d0JX28TcPQXCEPZ3BABIeTPYwEoCWZE\n\
h8l5YoQwTcU/9KNCMEAwDwYDVR0TAQH/BAUwAwEB/zAdBgNVHQ4EFgQUkCW1DdkF\n\
R+eWw5b6cp3PmanfS5YwDgYDVR0PAQH/BAQDAgGGMAoGCCqGSM49BAMDA2kAMGYC\n\
MQCjfy+Rocm9Xue4YnwWmNJVA44fA0P5W2OpYow9OYCVRaEevL8uO1XYru5xtMPW\n\
rfMCMQCi85sWBbJwKKXdS6BptQFuZbT73o/gBh1qUxl/nNr12UO8Yfwr6wPLb+6N\n\
IwLz3/Y=\n\
-----END CERTIFICATE-----\n";

const PROVIDER: &str = "aws_nitro";

struct NitroDocument {
    pcrs: BTreeMap<u8, Vec<u8>>,
    certificate: Vec<u8>,
    cabundle: Vec<Vec<u8>>,
    nonce: Option<Vec<u8>>,
    timestamp_ms: u64,
}

/// Verifier for AWS Nitro Enclave attestation documents.
pub struct NitroVerifier {
    /// Expected PCR0 measurement (enclave image hash), if enforced.
    pub expected_pcr0: Option<String>,
    /// Expected PCR8 image digest, if enforced.
    pub expected_pcr8: Option<String>,
    /// Whether to allow debug-mode enclaves.
    pub allow_debug: bool,
    root_cert_pem: String,
}

impl NitroVerifier {
    /// Create a new Nitro verifier with the pinned AWS Nitro root certificate.
    pub fn new() -> Self {
        Self {
            expected_pcr0: None,
            expected_pcr8: None,
            allow_debug: false,
            root_cert_pem: AWS_NITRO_ROOT_CERT_PEM.to_string(),
        }
    }

    /// Set the expected PCR0 measurement.
    pub fn with_expected_pcr0(mut self, pcr0: impl Into<String>) -> Self {
        self.expected_pcr0 = Some(pcr0.into());
        self
    }

    /// Set the expected PCR8 image digest.
    pub fn with_expected_pcr8(mut self, pcr8: impl Into<String>) -> Self {
        self.expected_pcr8 = Some(pcr8.into());
        self
    }

    /// Allow debug-mode enclaves (not recommended for production).
    pub fn allow_debug(mut self, allow: bool) -> Self {
        self.allow_debug = allow;
        self
    }

    /// Override the trusted root certificate, primarily for tests.
    pub fn with_root_cert_pem(mut self, pem: impl Into<String>) -> Self {
        self.root_cert_pem = pem.into();
        self
    }

    /// Verify a Nitro COSE_Sign1 attestation document cryptographically.
    pub fn verify_document(
        &self,
        cose_bytes: &[u8],
        policy: &AttestationPolicy,
    ) -> Result<VerifiedAttestation, AttestationError> {
        let sign1 = CoseSign1::from_tagged_slice(cose_bytes)
            .or_else(|_| CoseSign1::from_slice(cose_bytes))
            .map_err(|e| AttestationError::Malformed {
                provider: PROVIDER.to_string(),
                reason: format!("not a valid COSE_Sign1: {e:?}"),
            })?;

        let payload = sign1
            .payload
            .as_ref()
            .ok_or_else(|| AttestationError::Malformed {
                provider: PROVIDER.to_string(),
                reason: "COSE_Sign1 has no payload".to_string(),
            })?;
        let doc = decode_nitro_document(payload)?;

        verify_cert_chain(&doc, &self.root_cert_pem)?;
        verify_cose_signature(&sign1, &doc.certificate)?;

        let mut policy = policy.clone();
        if self.allow_debug {
            policy.allow_debug = true;
        }
        if policy.expected_measurement.is_none() {
            policy.expected_measurement = self.expected_pcr0.clone();
        }
        if policy.expected_image_digest.is_none() {
            policy.expected_image_digest = self.expected_pcr8.clone();
        }
        enforce_pcr_and_nonce_policy(&doc, &policy)?;

        let issued_at = doc.timestamp_ms / 1000;
        if let Some(max_age) = policy.max_age_secs {
            if now_unix().saturating_sub(issued_at) > max_age {
                return Err(AttestationError::Claim {
                    provider: PROVIDER.to_string(),
                    reason: format!("Nitro document older than {max_age}s"),
                });
            }
        }

        let pcr0 = doc.pcrs.get(&0).map(hex::encode).unwrap_or_default();
        let debug_mode = doc
            .pcrs
            .get(&0)
            .map(|pcr| pcr.iter().all(|b| *b == 0))
            .unwrap_or(false);
        let mut claims = AttestationClaims::new();
        claims.debug_mode = debug_mode;
        claims.boot_measurements = doc
            .pcrs
            .iter()
            .map(|(idx, value)| format!("pcr{idx}:{}", hex::encode(value)))
            .collect();
        for (idx, value) in &doc.pcrs {
            claims.custom.insert(
                format!("pcr{idx}"),
                serde_json::Value::String(hex::encode(value)),
            );
        }
        if let Some(nonce) = &doc.nonce {
            claims.custom.insert(
                "nonce".to_string(),
                serde_json::Value::String(String::from_utf8_lossy(nonce).to_string()),
            );
        }

        let report = AttestationReport {
            provider: TeeProvider::AwsNitro,
            format: AttestationFormat::NitroDocument,
            issued_at_unix: issued_at,
            measurement: Measurement::sha384(pcr0),
            public_key_binding: None,
            claims,
            evidence: cose_bytes.to_vec(),
        };
        Ok(VerifiedAttestation::new(report, TeeProvider::AwsNitro))
    }
}

impl Default for NitroVerifier {
    fn default() -> Self {
        Self::new()
    }
}

impl AttestationVerifier for NitroVerifier {
    fn verify(&self, report: &AttestationReport) -> Result<VerifiedAttestation, TeeError> {
        if report.provider != TeeProvider::AwsNitro {
            return Err(TeeError::AttestationVerification(format!(
                "expected AWS Nitro provider, got {}",
                report.provider
            )));
        }

        if report.format == AttestationFormat::NitroDocument && !report.evidence.is_empty() {
            let mut policy = AttestationPolicy::production();
            policy.expected_measurement = self.expected_pcr0.clone();
            policy.expected_image_digest = self.expected_pcr8.clone();
            policy.allow_debug = self.allow_debug;
            return self
                .verify_document(&report.evidence, &policy)
                .map_err(Into::into);
        }

        if report.claims.debug_mode && !self.allow_debug {
            return Err(TeeError::AttestationVerification(
                "debug mode enclaves are not permitted".to_string(),
            ));
        }

        if let Some(expected) = &self.expected_pcr0 {
            if report.measurement.digest != *expected {
                return Err(TeeError::MeasurementMismatch {
                    expected: expected.clone(),
                    actual: report.measurement.digest.clone(),
                });
            }
        }

        tracing::debug!(
            "structural validation only; provide raw Nitro COSE evidence for cryptographic verification"
        );

        Ok(VerifiedAttestation::new(
            report.clone(),
            TeeProvider::AwsNitro,
        ))
    }

    fn supported_provider(&self) -> TeeProvider {
        TeeProvider::AwsNitro
    }

    fn verification_level(&self) -> VerificationLevel {
        VerificationLevel::Cryptographic
    }
}

fn decode_nitro_document(payload: &[u8]) -> Result<NitroDocument, AttestationError> {
    let value: ciborium::value::Value =
        ciborium::de::from_reader(payload).map_err(|e| AttestationError::Malformed {
            provider: PROVIDER.to_string(),
            reason: format!("attestation document is not valid CBOR: {e}"),
        })?;
    let map = value.as_map().ok_or_else(|| AttestationError::Malformed {
        provider: PROVIDER.to_string(),
        reason: "attestation document is not a CBOR map".to_string(),
    })?;

    let mut pcrs = BTreeMap::new();
    let mut certificate = None;
    let mut cabundle = Vec::new();
    let mut nonce = None;
    let mut timestamp_ms = 0_u64;

    for (k, v) in map {
        let Some(key) = k.as_text() else { continue };
        match key {
            "pcrs" => {
                if let Some(entries) = v.as_map() {
                    for (idx, digest) in entries {
                        let i = idx
                            .as_integer()
                            .and_then(|n| u8::try_from(n).ok())
                            .ok_or_else(|| AttestationError::Malformed {
                                provider: PROVIDER.to_string(),
                                reason: "PCR index out of range".to_string(),
                            })?;
                        let bytes =
                            digest
                                .as_bytes()
                                .ok_or_else(|| AttestationError::Malformed {
                                    provider: PROVIDER.to_string(),
                                    reason: "PCR value is not bytes".to_string(),
                                })?;
                        pcrs.insert(i, bytes.clone());
                    }
                }
            }
            "certificate" => certificate = v.as_bytes().cloned(),
            "cabundle" => {
                if let Some(arr) = v.as_array() {
                    for cert in arr {
                        if let Some(der) = cert.as_bytes() {
                            cabundle.push(der.clone());
                        }
                    }
                }
            }
            "nonce" => nonce = v.as_bytes().cloned(),
            "timestamp" => {
                timestamp_ms = v
                    .as_integer()
                    .and_then(|n| u64::try_from(n).ok())
                    .unwrap_or(0);
            }
            _ => {}
        }
    }

    let certificate = certificate.ok_or_else(|| AttestationError::Malformed {
        provider: PROVIDER.to_string(),
        reason: "attestation document missing leaf certificate".to_string(),
    })?;

    Ok(NitroDocument {
        pcrs,
        certificate,
        cabundle,
        nonce,
        timestamp_ms,
    })
}

fn verify_cert_chain(doc: &NitroDocument, root_pem: &str) -> Result<(), AttestationError> {
    let root_der = parse_root_der(root_pem)?;

    let mut chain_der: Vec<&[u8]> = Vec::with_capacity(doc.cabundle.len() + 2);
    chain_der.push(&doc.certificate);
    for der in doc.cabundle.iter().rev() {
        chain_der.push(der);
    }
    chain_der.push(&root_der);

    let mut parsed = Vec::with_capacity(chain_der.len());
    for der in &chain_der {
        let (_, cert) =
            X509Certificate::from_der(der).map_err(|e| AttestationError::Signature {
                provider: PROVIDER.to_string(),
                reason: format!("certificate in chain is not valid DER: {e}"),
            })?;
        parsed.push(cert);
    }

    let now = ASN1Time::from_timestamp(now_unix() as i64).map_err(|e| AttestationError::Claim {
        provider: PROVIDER.to_string(),
        reason: format!("clock error: {e}"),
    })?;

    for i in 0..parsed.len() - 1 {
        let subject = &parsed[i];
        let issuer = &parsed[i + 1];
        if !subject.validity().is_valid_at(now) {
            return Err(AttestationError::Claim {
                provider: PROVIDER.to_string(),
                reason: format!("certificate {i} in chain is expired or not yet valid"),
            });
        }
        if subject.issuer() != issuer.subject() {
            return Err(AttestationError::Signature {
                provider: PROVIDER.to_string(),
                reason: format!(
                    "cert chain link {i} -> {} broken: issuer/subject DN mismatch",
                    i + 1
                ),
            });
        }
        enforce_ca_issuer(issuer, i, i + 1)?;
        subject
            .verify_signature(Some(issuer.public_key()))
            .map_err(|e| AttestationError::Signature {
                provider: PROVIDER.to_string(),
                reason: format!("cert chain link {i} -> {} failed: {e}", i + 1),
            })?;
    }

    let root = parsed.last().expect("chain has root");
    if !root.validity().is_valid_at(now) {
        return Err(AttestationError::Claim {
            provider: PROVIDER.to_string(),
            reason: "pinned AWS Nitro root certificate is expired".to_string(),
        });
    }
    root.verify_signature(None)
        .map_err(|e| AttestationError::Signature {
            provider: PROVIDER.to_string(),
            reason: format!("pinned root self-signature invalid: {e}"),
        })?;

    Ok(())
}

fn enforce_ca_issuer(
    issuer: &X509Certificate,
    subordinate_depth: usize,
    issuer_index: usize,
) -> Result<(), AttestationError> {
    let bc = issuer
        .basic_constraints()
        .map_err(|e| AttestationError::Signature {
            provider: PROVIDER.to_string(),
            reason: format!("cert {issuer_index} basic-constraints malformed: {e}"),
        })?
        .ok_or_else(|| AttestationError::Signature {
            provider: PROVIDER.to_string(),
            reason: format!(
                "cert {issuer_index} used as issuer but has no BasicConstraints extension"
            ),
        })?;
    if !bc.value.ca {
        return Err(AttestationError::Signature {
            provider: PROVIDER.to_string(),
            reason: format!("cert {issuer_index} used as issuer but is not a CA (cA=false)"),
        });
    }
    if let Some(max) = bc.value.path_len_constraint {
        if (subordinate_depth as u64) > max as u64 {
            return Err(AttestationError::Signature {
                provider: PROVIDER.to_string(),
                reason: format!(
                    "cert {issuer_index} pathLenConstraint={max} violated at depth {subordinate_depth}"
                ),
            });
        }
    }
    if let Some(ku) = issuer
        .key_usage()
        .map_err(|e| AttestationError::Signature {
            provider: PROVIDER.to_string(),
            reason: format!("cert {issuer_index} key-usage malformed: {e}"),
        })?
    {
        if !ku.value.key_cert_sign() {
            return Err(AttestationError::Signature {
                provider: PROVIDER.to_string(),
                reason: format!("cert {issuer_index} keyUsage does not permit keyCertSign"),
            });
        }
    }
    Ok(())
}

fn parse_root_der(root_pem: &str) -> Result<Vec<u8>, AttestationError> {
    let (_, pem) = x509_parser::pem::parse_x509_pem(root_pem.as_bytes()).map_err(|e| {
        AttestationError::Keys {
            provider: PROVIDER.to_string(),
            reason: format!("pinned root PEM is invalid: {e}"),
        }
    })?;
    Ok(pem.contents)
}

fn verify_cose_signature(sign1: &CoseSign1, leaf_der: &[u8]) -> Result<(), AttestationError> {
    use p384::ecdsa::{Signature, VerifyingKey, signature::Verifier};

    let (_, leaf) =
        X509Certificate::from_der(leaf_der).map_err(|e| AttestationError::Signature {
            provider: PROVIDER.to_string(),
            reason: format!("leaf certificate is not valid DER: {e}"),
        })?;
    let sec1_point = leaf.public_key().subject_public_key.data.as_ref();
    let verifying_key =
        VerifyingKey::from_sec1_bytes(sec1_point).map_err(|e| AttestationError::Signature {
            provider: PROVIDER.to_string(),
            reason: format!("leaf key is not a valid P-384 point: {e}"),
        })?;

    sign1
        .verify_signature(b"", |sig, tbs| {
            let signature = Signature::from_slice(sig)?;
            verifying_key.verify(tbs, &signature)
        })
        .map_err(|e: p384::ecdsa::Error| AttestationError::Signature {
            provider: PROVIDER.to_string(),
            reason: format!("COSE_Sign1 signature does not verify against leaf key: {e}"),
        })
}

fn enforce_pcr_and_nonce_policy(
    doc: &NitroDocument,
    policy: &AttestationPolicy,
) -> Result<(), AttestationError> {
    if !policy.allow_debug {
        if let Some(pcr0) = doc.pcrs.get(&0) {
            if pcr0.iter().all(|b| *b == 0) {
                return Err(AttestationError::Claim {
                    provider: PROVIDER.to_string(),
                    reason: "PCR0 is all-zero (debug-mode enclave); rejected by policy".to_string(),
                });
            }
        }
    }

    if let Some(expected) = &policy.expected_measurement {
        let pcr0 = doc.pcrs.get(&0).ok_or_else(|| AttestationError::Claim {
            provider: PROVIDER.to_string(),
            reason: "expected a PCR0 measurement but document has none".to_string(),
        })?;
        if !hex::encode(pcr0).eq_ignore_ascii_case(expected) {
            return Err(AttestationError::Claim {
                provider: PROVIDER.to_string(),
                reason: "PCR0 measurement mismatch".to_string(),
            });
        }
    }

    if let Some(expected) = &policy.expected_image_digest {
        let pcr8 = doc.pcrs.get(&8).ok_or_else(|| AttestationError::Claim {
            provider: PROVIDER.to_string(),
            reason: "expected a PCR8 image digest but document has none".to_string(),
        })?;
        if !hex::encode(pcr8).eq_ignore_ascii_case(expected) {
            return Err(AttestationError::Claim {
                provider: PROVIDER.to_string(),
                reason: "PCR8 image digest mismatch".to_string(),
            });
        }
    }

    if let Some(expected) = &policy.expected_nonce {
        let nonce = doc.nonce.as_ref().ok_or_else(|| AttestationError::Claim {
            provider: PROVIDER.to_string(),
            reason: "expected a nonce but document has none".to_string(),
        })?;
        if nonce.as_slice() != expected.as_bytes() {
            return Err(AttestationError::Claim {
                provider: PROVIDER.to_string(),
                reason: "Nitro nonce mismatch".to_string(),
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pinned_root_cert_parses() {
        let der = parse_root_der(AWS_NITRO_ROOT_CERT_PEM).expect("root PEM must parse");
        let (_, cert) = X509Certificate::from_der(&der).expect("root DER must parse");
        cert.verify_signature(None)
            .expect("pinned AWS Nitro root must be self-consistent");
    }

    #[test]
    fn garbage_is_rejected_malformed() {
        let verifier = NitroVerifier::new();
        let err = verifier
            .verify_document(b"not cose", &AttestationPolicy::production())
            .expect_err("garbage must be rejected");
        assert!(matches!(err, AttestationError::Malformed { .. }));
    }
}
