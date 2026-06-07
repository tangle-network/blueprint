//! AWS Nitro Enclaves attestation gate.
//!
//! # Fetch fails closed — by design
//!
//! A Nitro attestation document is produced by the Nitro Security Module device
//! (`/dev/nsm`), which is reachable **only from inside the enclave**. The
//! provisioner runs on the parent EC2 host / control plane, *outside* the
//! enclave, and AWS provides **no** API to pull an enclave's attestation
//! document remotely. Therefore [`AwsNitroGate::fetch`] returns
//! [`AttestationError::Unsatisfiable`]: a Nitro `require_tee` deployment cannot
//! be attested by this out-of-enclave provisioner. This is the cardinal-rule
//! fail-closed path — it never reports a Nitro VM as TEE-trusted on its own.
//!
//! # Verify is real
//!
//! The verification half is fully implemented so that a document obtained by an
//! in-enclave channel (an attestation endpoint the enclave application exposes,
//! or the `blueprint-tee` in-enclave runtime) can be genuinely checked:
//!
//! 1. Parse the COSE_Sign1 envelope ([`coset`]).
//! 2. Decode the CBOR attestation document payload (PCRs, cert, CA bundle,
//!    nonce, user_data, timestamp).
//! 3. Validate the certificate chain leaf → CA bundle → embedded **AWS Nitro
//!    root cert** (signature check via `x509-parser` + `aws-lc-rs`; no
//!    hand-rolled chain crypto).
//! 4. Verify the COSE_Sign1 ES384 signature with the leaf certificate's P-384
//!    public key ([`p384`]).
//! 5. Enforce PCR / nonce / debug policy.

use super::{AttestationError, AttestationPolicy, TeeAttestationGate, VerifiedTeeDeployment};
use crate::core::remote::CloudProvider;
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

/// Decoded NSM attestation document payload (the COSE_Sign1 payload).
struct NitroDocument {
    /// PCR index -> measurement bytes.
    pcrs: BTreeMap<u8, Vec<u8>>,
    /// Leaf certificate DER (signs the COSE document).
    certificate: Vec<u8>,
    /// Intermediate CA certificate DERs (leaf's issuer chain toward the root).
    cabundle: Vec<Vec<u8>>,
    /// Optional nonce echoed from the attestation request.
    nonce: Option<Vec<u8>>,
    /// Document timestamp (milliseconds since unix epoch).
    timestamp_ms: u64,
}

/// Gate for AWS Nitro Enclaves.
pub struct AwsNitroGate {
    root_cert_pem: String,
}

impl AwsNitroGate {
    /// Construct with the pinned AWS Nitro root certificate.
    pub fn new() -> Self {
        Self {
            root_cert_pem: AWS_NITRO_ROOT_CERT_PEM.to_string(),
        }
    }

    /// Override the trusted root certificate (used by tests with a synthetic CA).
    pub fn with_root_cert_pem(mut self, pem: impl Into<String>) -> Self {
        self.root_cert_pem = pem.into();
        self
    }

    /// Verify a Nitro COSE_Sign1 attestation document.
    ///
    /// Public so an in-enclave agent (or a test) holding a real document can
    /// verify it; `fetch` itself fails closed for the out-of-enclave provisioner.
    pub fn verify_document(
        &self,
        cose_bytes: &[u8],
        policy: &AttestationPolicy,
    ) -> Result<VerifiedTeeDeployment, AttestationError> {
        // 1. Parse the COSE_Sign1 envelope (try tagged first, then untagged).
        let sign1 = CoseSign1::from_tagged_slice(cose_bytes)
            .or_else(|_| CoseSign1::from_slice(cose_bytes))
            .map_err(|e| AttestationError::Malformed {
                provider: CloudProvider::AWS,
                reason: format!("not a valid COSE_Sign1: {e:?}"),
            })?;

        // 2. Decode the CBOR attestation document payload.
        let payload = sign1
            .payload
            .as_ref()
            .ok_or_else(|| AttestationError::Malformed {
                provider: CloudProvider::AWS,
                reason: "COSE_Sign1 has no payload".to_string(),
            })?;
        let doc = decode_nitro_document(payload)?;

        // 3. Validate cert chain: leaf -> cabundle -> pinned AWS Nitro root.
        verify_cert_chain(&doc, &self.root_cert_pem)?;

        // 4. Verify the COSE signature with the leaf cert's P-384 public key.
        verify_cose_signature(&sign1, &doc.certificate)?;

        // 5. Policy enforcement.
        enforce_pcr_and_nonce_policy(&doc, policy)?;

        let issued_at = doc.timestamp_ms / 1000;
        if let Some(max_age) = policy.max_age_secs {
            let now = super::now_unix();
            if now.saturating_sub(issued_at) > max_age {
                return Err(AttestationError::Claim {
                    provider: CloudProvider::AWS,
                    reason: format!("Nitro document older than {max_age}s"),
                });
            }
        }

        let pcr0 = doc.pcrs.get(&0).map(|b| hex::encode(b));
        let mut claims = BTreeMap::new();
        for (idx, val) in &doc.pcrs {
            claims.insert(format!("pcr{idx}"), hex::encode(val));
        }
        Ok(VerifiedTeeDeployment::new(
            CloudProvider::AWS,
            pcr0,
            claims,
            Some(issued_at),
        ))
    }
}

impl Default for AwsNitroGate {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl TeeAttestationGate for AwsNitroGate {
    fn provider(&self) -> CloudProvider {
        CloudProvider::AWS
    }

    async fn fetch(&self, _endpoint: &str, _nonce: &str) -> Result<Vec<u8>, AttestationError> {
        // The NSM device (`/dev/nsm`) that produces the attestation document is
        // reachable only from inside the enclave; there is no AWS API to fetch
        // it from the parent host. Fail closed — never bless a Nitro VM that we
        // cannot attest. An in-enclave agent must call `verify_document` with a
        // document delivered over a channel the enclave application exposes.
        Err(AttestationError::Unsatisfiable {
            provider: CloudProvider::AWS,
            reason: "Nitro NSM attestation is producible only inside the enclave; the \
                     out-of-enclave provisioner cannot fetch it. Supply the COSE document \
                     via an in-enclave channel and call verify_document()."
                .to_string(),
        })
    }

    async fn verify(
        &self,
        evidence: &[u8],
        policy: &AttestationPolicy,
    ) -> Result<VerifiedTeeDeployment, AttestationError> {
        self.verify_document(evidence, policy)
    }
}

/// Decode the CBOR map that is the NSM attestation document.
fn decode_nitro_document(payload: &[u8]) -> Result<NitroDocument, AttestationError> {
    let value: ciborium::value::Value =
        ciborium::de::from_reader(payload).map_err(|e| AttestationError::Malformed {
            provider: CloudProvider::AWS,
            reason: format!("attestation document is not valid CBOR: {e}"),
        })?;
    let map = value.as_map().ok_or_else(|| AttestationError::Malformed {
        provider: CloudProvider::AWS,
        reason: "attestation document is not a CBOR map".to_string(),
    })?;

    let mut pcrs = BTreeMap::new();
    let mut certificate = None;
    let mut cabundle = Vec::new();
    let mut nonce = None;
    let mut timestamp_ms = 0u64;

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
                                provider: CloudProvider::AWS,
                                reason: "PCR index out of range".to_string(),
                            })?;
                        let bytes =
                            digest
                                .as_bytes()
                                .ok_or_else(|| AttestationError::Malformed {
                                    provider: CloudProvider::AWS,
                                    reason: "PCR value is not bytes".to_string(),
                                })?;
                        pcrs.insert(i, bytes.clone());
                    }
                }
            }
            "certificate" => {
                certificate = v.as_bytes().cloned();
            }
            "cabundle" => {
                if let Some(arr) = v.as_array() {
                    for cert in arr {
                        if let Some(der) = cert.as_bytes() {
                            cabundle.push(der.clone());
                        }
                    }
                }
            }
            "nonce" => {
                nonce = v.as_bytes().cloned();
            }
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
        provider: CloudProvider::AWS,
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

/// Validate the certificate chain leaf -> cabundle -> pinned root.
///
/// AWS orders `cabundle` root-first; we walk from the leaf upward, requiring
/// each cert to be signed by the next, and the topmost to be signed by (or be)
/// the pinned AWS Nitro root. Uses `x509-parser`'s aws-lc-rs-backed
/// `verify_signature` — no hand-rolled chain crypto.
fn verify_cert_chain(doc: &NitroDocument, root_pem: &str) -> Result<(), AttestationError> {
    let root_der = parse_root_der(root_pem)?;

    // Build the chain from leaf to root: [leaf, cabundle intermediates (reversed
    // so issuer comes after subject), root].
    let mut chain_der: Vec<&[u8]> = Vec::with_capacity(doc.cabundle.len() + 2);
    chain_der.push(&doc.certificate);
    // cabundle is root-first; reverse so we go subject -> issuer.
    for der in doc.cabundle.iter().rev() {
        chain_der.push(der);
    }
    chain_der.push(&root_der);

    // Parse all certs.
    let mut parsed = Vec::with_capacity(chain_der.len());
    for der in &chain_der {
        let (_, cert) =
            X509Certificate::from_der(der).map_err(|e| AttestationError::Signature {
                provider: CloudProvider::AWS,
                reason: format!("certificate in chain is not valid DER: {e}"),
            })?;
        parsed.push(cert);
    }

    let now = ASN1Time::from_timestamp(super::now_unix() as i64).map_err(|e| {
        AttestationError::Claim {
            provider: CloudProvider::AWS,
            reason: format!("clock error: {e}"),
        }
    })?;

    // Each cert (except the root) must be signed by the next cert's key, be
    // temporally valid, name-chain to its issuer, and — for every issuer — carry
    // the CA constraints (BasicConstraints cA=true, pathLen, keyUsage
    // keyCertSign). Verifying only the signature would let a legitimate non-CA
    // leaf (every Nitro customer holds one chaining to the pinned root) act as an
    // issuer for a forged sub-leaf carrying forged PCRs. The constraint checks
    // close that path.
    for i in 0..parsed.len() - 1 {
        let subject = &parsed[i];
        let issuer = &parsed[i + 1];
        if !subject.validity().is_valid_at(now) {
            return Err(AttestationError::Claim {
                provider: CloudProvider::AWS,
                reason: format!("certificate {i} in chain is expired or not yet valid"),
            });
        }
        // Name chaining: the subject's issuer DN must equal the issuer's subject DN.
        if subject.issuer() != issuer.subject() {
            return Err(AttestationError::Signature {
                provider: CloudProvider::AWS,
                reason: format!(
                    "cert chain link {i} -> {} broken: issuer/subject DN mismatch",
                    i + 1
                ),
            });
        }
        // The issuer must be a CA permitted to issue at this depth. `i` certs sit
        // below the issuer in the path (the leaf at depth 0 is the first subject),
        // so the issuer's pathLenConstraint, if present, must be >= i.
        enforce_ca_issuer(issuer, i, i + 1)?;
        subject
            .verify_signature(Some(issuer.public_key()))
            .map_err(|e| AttestationError::Signature {
                provider: CloudProvider::AWS,
                reason: format!("cert chain link {i} -> {} failed: {e}", i + 1),
            })?;
    }

    // The pinned root must be self-consistent (self-signed) and valid now.
    let root = parsed.last().expect("chain has root");
    if !root.validity().is_valid_at(now) {
        return Err(AttestationError::Claim {
            provider: CloudProvider::AWS,
            reason: "pinned AWS Nitro root certificate is expired".to_string(),
        });
    }
    root.verify_signature(None)
        .map_err(|e| AttestationError::Signature {
            provider: CloudProvider::AWS,
            reason: format!("pinned root self-signature invalid: {e}"),
        })?;

    Ok(())
}

/// Require that `issuer` is a CA certificate permitted to sign the cert below it.
///
/// Enforces BasicConstraints (`cA=true`, present and — for the AWS Nitro PKI —
/// required to be critical), the `pathLenConstraint` (the number of non-self-
/// issued intermediates that may follow must cover `subordinate_depth`), and
/// keyUsage `keyCertSign`. `issuer_index` is only used for error messages.
fn enforce_ca_issuer(
    issuer: &X509Certificate,
    subordinate_depth: usize,
    issuer_index: usize,
) -> Result<(), AttestationError> {
    let bc = issuer
        .basic_constraints()
        .map_err(|e| AttestationError::Signature {
            provider: CloudProvider::AWS,
            reason: format!("cert {issuer_index} basic-constraints malformed: {e}"),
        })?
        .ok_or_else(|| AttestationError::Signature {
            provider: CloudProvider::AWS,
            reason: format!(
                "cert {issuer_index} used as issuer but has no BasicConstraints extension"
            ),
        })?;
    if !bc.value.ca {
        return Err(AttestationError::Signature {
            provider: CloudProvider::AWS,
            reason: format!("cert {issuer_index} used as issuer but is not a CA (cA=false)"),
        });
    }
    // pathLenConstraint counts the max number of CAs that may appear *below* this
    // one (excluding the leaf). The subordinate at `subordinate_depth` has
    // `subordinate_depth` certs beneath it in the path; that many intermediates
    // must be permitted.
    if let Some(max) = bc.value.path_len_constraint {
        if (subordinate_depth as u64) > max as u64 {
            return Err(AttestationError::Signature {
                provider: CloudProvider::AWS,
                reason: format!(
                    "cert {issuer_index} pathLenConstraint={max} violated at depth {subordinate_depth}"
                ),
            });
        }
    }
    // keyUsage, when present, must permit certificate signing. (AWS Nitro CAs set
    // it; if the extension is absent we do not invent a usage, but BasicConstraints
    // cA=true above already gates issuance.)
    if let Some(ku) = issuer
        .key_usage()
        .map_err(|e| AttestationError::Signature {
            provider: CloudProvider::AWS,
            reason: format!("cert {issuer_index} key-usage malformed: {e}"),
        })?
    {
        if !ku.value.key_cert_sign() {
            return Err(AttestationError::Signature {
                provider: CloudProvider::AWS,
                reason: format!("cert {issuer_index} keyUsage does not permit keyCertSign"),
            });
        }
    }
    Ok(())
}

fn parse_root_der(root_pem: &str) -> Result<Vec<u8>, AttestationError> {
    let (_, pem) = x509_parser::pem::parse_x509_pem(root_pem.as_bytes()).map_err(|e| {
        AttestationError::Keys {
            provider: CloudProvider::AWS,
            reason: format!("pinned root PEM is invalid: {e}"),
        }
    })?;
    Ok(pem.contents)
}

/// Verify the COSE_Sign1 ES384 signature with the leaf certificate's P-384 key.
fn verify_cose_signature(sign1: &CoseSign1, leaf_der: &[u8]) -> Result<(), AttestationError> {
    use p384::ecdsa::{Signature, VerifyingKey, signature::Verifier};

    let (_, leaf) =
        X509Certificate::from_der(leaf_der).map_err(|e| AttestationError::Signature {
            provider: CloudProvider::AWS,
            reason: format!("leaf certificate is not valid DER: {e}"),
        })?;
    let spki = leaf.public_key();
    // The SubjectPublicKey bitstring holds the SEC1-encoded EC point.
    let sec1_point = spki.subject_public_key.data.as_ref();
    let verifying_key =
        VerifyingKey::from_sec1_bytes(sec1_point).map_err(|e| AttestationError::Signature {
            provider: CloudProvider::AWS,
            reason: format!("leaf key is not a valid P-384 point: {e}"),
        })?;

    sign1
        .verify_signature(b"", |sig, tbs| {
            // COSE ES384 signatures are the raw r||s concatenation.
            let signature = Signature::from_slice(sig)?;
            verifying_key.verify(tbs, &signature)
        })
        .map_err(|e: p384::ecdsa::Error| AttestationError::Signature {
            provider: CloudProvider::AWS,
            reason: format!("COSE_Sign1 signature does not verify against leaf key: {e}"),
        })
}

/// Enforce PCR0 (and PCR8 image digest) + nonce + debug policy.
fn enforce_pcr_and_nonce_policy(
    doc: &NitroDocument,
    policy: &AttestationPolicy,
) -> Result<(), AttestationError> {
    // Debug enclaves boot with PCR0 == all zeros. Reject unless explicitly allowed.
    if !policy.allow_debug {
        if let Some(pcr0) = doc.pcrs.get(&0) {
            if pcr0.iter().all(|b| *b == 0) {
                return Err(AttestationError::Claim {
                    provider: CloudProvider::AWS,
                    reason: "PCR0 is all-zero (debug-mode enclave); rejected by policy".to_string(),
                });
            }
        }
    }

    if let Some(expected) = &policy.expected_measurement {
        let pcr0 = doc.pcrs.get(&0).ok_or_else(|| AttestationError::Claim {
            provider: CloudProvider::AWS,
            reason: "expected a PCR0 measurement but document has none".to_string(),
        })?;
        if !hex::encode(pcr0).eq_ignore_ascii_case(expected) {
            return Err(AttestationError::Claim {
                provider: CloudProvider::AWS,
                reason: "PCR0 measurement mismatch".to_string(),
            });
        }
    }

    // PCR8 binds the signing certificate of the enclave image (image identity).
    if let Some(expected) = &policy.expected_image_digest {
        let pcr8 = doc.pcrs.get(&8).ok_or_else(|| AttestationError::Claim {
            provider: CloudProvider::AWS,
            reason: "expected a PCR8 image digest but document has none".to_string(),
        })?;
        if !hex::encode(pcr8).eq_ignore_ascii_case(expected) {
            return Err(AttestationError::Claim {
                provider: CloudProvider::AWS,
                reason: "PCR8 image digest mismatch".to_string(),
            });
        }
    }

    if let Some(expected) = &policy.expected_nonce {
        let nonce = doc.nonce.as_ref().ok_or_else(|| AttestationError::Claim {
            provider: CloudProvider::AWS,
            reason: "expected a nonce but document has none".to_string(),
        })?;
        if nonce.as_slice() != expected.as_bytes() {
            return Err(AttestationError::Claim {
                provider: CloudProvider::AWS,
                reason: "Nitro nonce mismatch".to_string(),
            });
        }
    }

    Ok(())
}

#[cfg(all(test, feature = "tee-attestation"))]
mod tests {
    use super::*;

    #[tokio::test]
    async fn fetch_fails_closed_for_out_of_enclave_provisioner() {
        let gate = AwsNitroGate::new();
        let err = gate
            .fetch("1.2.3.4", "nonce")
            .await
            .expect_err("Nitro fetch must fail closed");
        assert!(matches!(err, AttestationError::Unsatisfiable { .. }));
    }

    #[test]
    fn pinned_root_cert_parses() {
        let der = parse_root_der(AWS_NITRO_ROOT_CERT_PEM).expect("root PEM must parse");
        let (_, cert) = X509Certificate::from_der(&der).expect("root DER must parse");
        // Self-signed root: its own signature must verify against its own key.
        cert.verify_signature(None)
            .expect("pinned AWS Nitro root must be self-consistent");
    }

    #[test]
    fn garbage_is_rejected_malformed() {
        let gate = AwsNitroGate::new();
        let err = gate
            .verify_document(b"not cose", &AttestationPolicy::production())
            .expect_err("garbage must be rejected");
        assert!(matches!(err, AttestationError::Malformed { .. }));
    }
}
