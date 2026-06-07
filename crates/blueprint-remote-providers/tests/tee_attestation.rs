//! Post-launch TEE attestation tests.
//!
//! These exercise the real verification paths with mocked provider responses:
//!
//! * GCP / Azure (JWT): an RSA keypair signs an RS256 attestation token; the
//!   public key is served as a JWKS from a `wiremock` server. Positive cases
//!   (valid token + correct claims) trust; negative cases (bad signature,
//!   expired, wrong audience, wrong nonce, wrong kid) reject.
//! * AWS Nitro (COSE): a synthetic P-384 root + leaf chain signs a COSE_Sign1
//!   attestation document; the verifier trusts it under a test root, and the
//!   `fetch` path fails closed for the out-of-enclave provisioner.
//!
//! Only built with `--features tee-attestation`.
#![cfg(feature = "tee-attestation")]

use base64::Engine;
#[cfg(feature = "tee-attestation-nitro")]
use blueprint_remote_providers::attestation::aws_nitro::AwsNitroGate;
use blueprint_remote_providers::attestation::azure::AzureMaaGate;
use blueprint_remote_providers::attestation::gcp::GcpConfidentialSpaceGate;
use blueprint_remote_providers::attestation::{
    AttestationError, AttestationPolicy, TeeAttestationGate,
};
use blueprint_tee::TeeProvider;
use jsonwebtoken::{EncodingKey, Header, encode};
use rsa::RsaPrivateKey;
use rsa::pkcs1::EncodeRsaPrivateKey;
use rsa::traits::PublicKeyParts;
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// ---------------------------------------------------------------------------
// JWT (GCP / Azure) test scaffolding
// ---------------------------------------------------------------------------

const TEST_KID: &str = "test-key-1";

struct TestJwtSigner {
    private_pem: String,
    n_b64: String,
    e_b64: String,
}

fn b64url(bytes: &[u8]) -> String {
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

fn make_signer() -> TestJwtSigner {
    let mut rng = rand::thread_rng();
    let key = RsaPrivateKey::new(&mut rng, 2048).expect("generate RSA key");
    let private_pem = key
        .to_pkcs1_pem(rsa::pkcs1::LineEnding::LF)
        .unwrap()
        .to_string();
    let pubkey = key.to_public_key();
    TestJwtSigner {
        private_pem,
        n_b64: b64url(&pubkey.n().to_bytes_be()),
        e_b64: b64url(&pubkey.e().to_bytes_be()),
    }
}

impl TestJwtSigner {
    fn jwks_json(&self) -> serde_json::Value {
        json!({
            "keys": [{
                "kty": "RSA",
                "use": "sig",
                "alg": "RS256",
                "kid": TEST_KID,
                "n": self.n_b64,
                "e": self.e_b64,
            }]
        })
    }

    fn sign(&self, claims: &serde_json::Value, kid: &str) -> String {
        let mut header = Header::new(jsonwebtoken::Algorithm::RS256);
        header.kid = Some(kid.to_string());
        let key = EncodingKey::from_rsa_pem(self.private_pem.as_bytes()).unwrap();
        encode(&header, claims, &key).unwrap()
    }
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Stand up a mock JWKS server returning the signer's public key.
async fn jwks_server(signer: &TestJwtSigner) -> MockServer {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/jwks"))
        .respond_with(ResponseTemplate::new(200).set_body_json(signer.jwks_json()))
        .mount(&server)
        .await;
    server
}

// ---------------------------------------------------------------------------
// GCP Confidential Space (positive + negative)
// ---------------------------------------------------------------------------

fn gcp_gate(jwks_url: String) -> GcpConfidentialSpaceGate {
    GcpConfidentialSpaceGate::new()
        .with_jwks_url(jwks_url)
        .with_allowed_issuers(vec!["https://issuer.test".to_string()])
}

fn gcp_claims(nonce: &str, aud: &str, iat: u64, exp: u64) -> serde_json::Value {
    json!({
        "iss": "https://issuer.test",
        "aud": aud,
        "iat": iat,
        "exp": exp,
        "eat_nonce": nonce,
        "dbgstat": "disabled-since-boot",
        "submods": { "container": { "image_digest": "sha256:cafef00d" } },
    })
}

#[tokio::test]
async fn gcp_valid_token_is_trusted() {
    let signer = make_signer();
    let server = jwks_server(&signer).await;
    let gate = gcp_gate(format!("{}/jwks", server.uri()));

    let token = signer.sign(
        &gcp_claims("nonce-123", "blueprint-svc", now(), now() + 600),
        TEST_KID,
    );

    let policy = AttestationPolicy::production()
        .with_audience("blueprint-svc")
        .with_nonce("nonce-123")
        .with_image_digest("sha256:cafef00d");

    let verified = gate
        .verify(token.as_bytes(), &policy)
        .await
        .expect("valid token + correct claims must be trusted");
    assert_eq!(verified.verified_by(), TeeProvider::GcpConfidential);
    assert_eq!(
        verified
            .report()
            .claims
            .get_custom("eat_nonce")
            .and_then(serde_json::Value::as_str),
        Some("nonce-123")
    );
}

#[tokio::test]
async fn gcp_wrong_nonce_is_rejected() {
    let signer = make_signer();
    let server = jwks_server(&signer).await;
    let gate = gcp_gate(format!("{}/jwks", server.uri()));
    let token = signer.sign(
        &gcp_claims("attacker-nonce", "blueprint-svc", now(), now() + 600),
        TEST_KID,
    );
    let policy = AttestationPolicy::production()
        .with_audience("blueprint-svc")
        .with_nonce("nonce-123");
    let err = gate.verify(token.as_bytes(), &policy).await.unwrap_err();
    assert!(matches!(err, AttestationError::Claim { .. }), "got {err:?}");
}

#[tokio::test]
async fn gcp_wrong_audience_is_rejected() {
    let signer = make_signer();
    let server = jwks_server(&signer).await;
    let gate = gcp_gate(format!("{}/jwks", server.uri()));
    let token = signer.sign(
        &gcp_claims("nonce-123", "some-other-aud", now(), now() + 600),
        TEST_KID,
    );
    let policy = AttestationPolicy::production()
        .with_audience("blueprint-svc")
        .with_nonce("nonce-123");
    let err = gate.verify(token.as_bytes(), &policy).await.unwrap_err();
    assert!(matches!(err, AttestationError::Claim { .. }), "got {err:?}");
}

#[tokio::test]
async fn gcp_expired_token_is_rejected() {
    let signer = make_signer();
    let server = jwks_server(&signer).await;
    let gate = gcp_gate(format!("{}/jwks", server.uri()));
    // Issued and expired well in the past.
    let token = signer.sign(
        &gcp_claims("nonce-123", "blueprint-svc", now() - 10_000, now() - 9_000),
        TEST_KID,
    );
    let policy = AttestationPolicy::production()
        .with_audience("blueprint-svc")
        .with_nonce("nonce-123");
    let err = gate.verify(token.as_bytes(), &policy).await.unwrap_err();
    assert!(matches!(err, AttestationError::Claim { .. }), "got {err:?}");
}

#[tokio::test]
async fn gcp_debug_mode_is_rejected() {
    let signer = make_signer();
    let server = jwks_server(&signer).await;
    let gate = gcp_gate(format!("{}/jwks", server.uri()));
    let mut claims = gcp_claims("nonce-123", "blueprint-svc", now(), now() + 600);
    claims["dbgstat"] = json!("enabled"); // debug enclave
    let token = signer.sign(&claims, TEST_KID);
    let policy = AttestationPolicy::production()
        .with_audience("blueprint-svc")
        .with_nonce("nonce-123");
    let err = gate.verify(token.as_bytes(), &policy).await.unwrap_err();
    assert!(matches!(err, AttestationError::Claim { .. }), "got {err:?}");
}

#[tokio::test]
async fn gcp_bad_signature_is_rejected() {
    // Token signed by signer A, JWKS served from signer B → signature mismatch.
    let signer_a = make_signer();
    let signer_b = make_signer();
    let server = jwks_server(&signer_b).await;
    let gate = gcp_gate(format!("{}/jwks", server.uri()));
    let token = signer_a.sign(
        &gcp_claims("nonce-123", "blueprint-svc", now(), now() + 600),
        TEST_KID,
    );
    let policy = AttestationPolicy::production()
        .with_audience("blueprint-svc")
        .with_nonce("nonce-123");
    let err = gate.verify(token.as_bytes(), &policy).await.unwrap_err();
    assert!(
        matches!(err, AttestationError::Signature { .. }),
        "got {err:?}"
    );
}

#[tokio::test]
async fn gcp_unknown_kid_is_rejected() {
    let signer = make_signer();
    let server = jwks_server(&signer).await;
    let gate = gcp_gate(format!("{}/jwks", server.uri()));
    let token = signer.sign(
        &gcp_claims("nonce-123", "blueprint-svc", now(), now() + 600),
        "some-other-kid",
    );
    let policy = AttestationPolicy::production()
        .with_audience("blueprint-svc")
        .with_nonce("nonce-123");
    let err = gate.verify(token.as_bytes(), &policy).await.unwrap_err();
    assert!(matches!(err, AttestationError::Keys { .. }), "got {err:?}");
}

#[tokio::test]
async fn gcp_alg_none_is_rejected() {
    // A token with alg=none must never be accepted even before JWKS fetch.
    let server = MockServer::start().await;
    let gate = gcp_gate(format!("{}/jwks", server.uri()));
    // alg=none token: header.payload. (empty signature)
    let header = b64url(br#"{"alg":"none","kid":"test-key-1"}"#);
    let payload = b64url(br#"{"aud":"blueprint-svc","nonce":"nonce-123"}"#);
    let token = format!("{header}.{payload}.");
    let policy = AttestationPolicy::production().with_audience("blueprint-svc");
    let err = gate.verify(token.as_bytes(), &policy).await.unwrap_err();
    assert!(
        matches!(
            err,
            AttestationError::Signature { .. } | AttestationError::Malformed { .. }
        ),
        "got {err:?}"
    );
}

// ---------------------------------------------------------------------------
// Azure MAA (positive + negative) — same JWT machinery, Azure claim shapes
// ---------------------------------------------------------------------------

fn azure_gate(jwks_url: String) -> AzureMaaGate {
    AzureMaaGate::for_instance("https://maa.test")
        .with_jwks_url(jwks_url)
        .with_allowed_issuers(vec!["https://maa.test".to_string()])
}

fn azure_claims(nonce: &str, aud: &str, debuggable: bool) -> serde_json::Value {
    json!({
        "iss": "https://maa.test",
        "aud": aud,
        "iat": now(),
        "exp": now() + 600,
        "nonce": nonce,
        "x-ms-sevsnpvm-is-debuggable": debuggable,
        "x-ms-sevsnpvm-launchmeasurement": "aabbccdd",
    })
}

#[tokio::test]
async fn azure_valid_token_is_trusted() {
    let signer = make_signer();
    let server = jwks_server(&signer).await;
    let gate = azure_gate(format!("{}/jwks", server.uri()));
    let token = signer.sign(&azure_claims("maa-nonce", "blueprint-svc", false), TEST_KID);
    let policy = AttestationPolicy::production()
        .with_audience("blueprint-svc")
        .with_nonce("maa-nonce");
    let verified = gate
        .verify(token.as_bytes(), &policy)
        .await
        .expect("valid MAA token must be trusted");
    assert_eq!(verified.verified_by(), TeeProvider::AzureSnp);
    assert_eq!(verified.report().measurement.digest, "aabbccdd");
}

#[tokio::test]
async fn azure_debuggable_cvm_is_rejected() {
    let signer = make_signer();
    let server = jwks_server(&signer).await;
    let gate = azure_gate(format!("{}/jwks", server.uri()));
    let token = signer.sign(&azure_claims("maa-nonce", "blueprint-svc", true), TEST_KID);
    let policy = AttestationPolicy::production()
        .with_audience("blueprint-svc")
        .with_nonce("maa-nonce");
    let err = gate.verify(token.as_bytes(), &policy).await.unwrap_err();
    assert!(matches!(err, AttestationError::Claim { .. }), "got {err:?}");
}

#[tokio::test]
async fn azure_tampered_payload_is_rejected() {
    let signer = make_signer();
    let server = jwks_server(&signer).await;
    let gate = azure_gate(format!("{}/jwks", server.uri()));
    let token = signer.sign(&azure_claims("maa-nonce", "blueprint-svc", false), TEST_KID);
    // Tamper: swap the middle (payload) segment for a forged one.
    let mut parts: Vec<&str> = token.split('.').collect();
    let forged = b64url(
        br#"{"iss":"https://maa.test","aud":"blueprint-svc","nonce":"maa-nonce","exp":9999999999}"#,
    );
    parts[1] = &forged;
    let tampered = parts.join(".");
    let policy = AttestationPolicy::production()
        .with_audience("blueprint-svc")
        .with_nonce("maa-nonce");
    let err = gate.verify(tampered.as_bytes(), &policy).await.unwrap_err();
    assert!(
        matches!(err, AttestationError::Signature { .. }),
        "got {err:?}"
    );
}

// ---------------------------------------------------------------------------
// Response-body size cap (untrusted-source DoS protection)
// ---------------------------------------------------------------------------
// The token and JWKS are fetched from sources this module distrusts (the
// provisioned VM whose trust is in question, and the JWKS URL). A hostile
// endpoint that streams a multi-GB body must not OOM the control plane before
// any signature check runs. These tests drive the cap through the real public
// fetch/verify paths, not the private helper.

/// 1 MiB body — over the 64 KiB token cap and the 256 KiB JWKS cap.
fn oversized_body() -> Vec<u8> {
    vec![b'a'; 1024 * 1024]
}

#[tokio::test]
async fn oversized_jwks_is_rejected_not_buffered() {
    // The JWKS endpoint returns a 1 MiB body. Verification must abort at the cap
    // with a Keys error instead of buffering the whole body.
    let signer = make_signer();
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/jwks"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(oversized_body()))
        .mount(&server)
        .await;
    let gate = gcp_gate(format!("{}/jwks", server.uri()));
    // A well-formed token so we reach the JWKS fetch; the cap fires before parse.
    let token = signer.sign(
        &gcp_claims("nonce-123", "blueprint-svc", now(), now() + 600),
        TEST_KID,
    );
    let policy = AttestationPolicy::production()
        .with_audience("blueprint-svc")
        .with_nonce("nonce-123");
    let err = gate.verify(token.as_bytes(), &policy).await.unwrap_err();
    match err {
        AttestationError::Keys { reason, .. } => {
            assert!(
                reason.contains("cap") || reason.contains("too large"),
                "expected a size-cap reason, got: {reason}"
            );
        }
        other => panic!("expected a Keys size-cap error, got {other:?}"),
    }
}

#[tokio::test]
async fn oversized_token_fetch_is_rejected_not_buffered() {
    // The VM's token endpoint streams a 1 MiB body. fetch() must abort at the
    // 64 KiB token cap with a Fetch error.
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/.well-known/attestation-token"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(oversized_body()))
        .mount(&server)
        .await;
    let gate = gcp_gate(format!("{}/jwks-unused", server.uri()));
    let err = gate.fetch(&server.uri(), "nonce-123").await.unwrap_err();
    match err {
        AttestationError::Fetch { reason, .. } => {
            assert!(
                reason.contains("cap") || reason.contains("too large"),
                "expected a size-cap reason, got: {reason}"
            );
        }
        other => panic!("expected a Fetch size-cap error, got {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// AWS Nitro (COSE) — synthetic chain, fail-closed fetch
// ---------------------------------------------------------------------------
// Gated on `tee-attestation-nitro`: the COSE verifier and its crypto deps ship
// only under that feature.

#[cfg(feature = "tee-attestation-nitro")]
mod nitro {
    use super::*;
    use ciborium::value::Value as Cbor;
    use coset::{CborSerializable, CoseSign1Builder, HeaderBuilder};
    use p384::ecdsa::{Signature, SigningKey, signature::Signer};
    use p384::pkcs8::DecodePrivateKey;
    use rcgen::{CertificateParams, KeyPair, PKCS_ECDSA_P384_SHA384};

    /// A synthetic Nitro root + leaf, with the leaf's P-384 key for COSE signing.
    struct SyntheticChain {
        root_pem: String,
        leaf_der: Vec<u8>,
        signing_key: SigningKey,
    }

    fn build_chain() -> SyntheticChain {
        // Root CA (self-signed P-384).
        let root_kp = KeyPair::generate_for(&PKCS_ECDSA_P384_SHA384).unwrap();
        let mut root_params = CertificateParams::new(vec!["nitro-test-root".into()]).unwrap();
        root_params.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
        let root_cert = root_params.self_signed(&root_kp).unwrap();

        // Leaf signed by root; its key signs the COSE document.
        let leaf_kp = KeyPair::generate_for(&PKCS_ECDSA_P384_SHA384).unwrap();
        let leaf_params = CertificateParams::new(vec!["nitro-test-leaf".into()]).unwrap();
        let leaf_cert = leaf_params
            .signed_by(&leaf_kp, &root_cert, &root_kp)
            .unwrap();

        let signing_key = SigningKey::from_pkcs8_der(leaf_kp.serialized_der()).unwrap();

        SyntheticChain {
            root_pem: root_cert.pem(),
            leaf_der: leaf_cert.der().to_vec(),
            signing_key,
        }
    }

    /// Build the CBOR attestation document payload.
    fn build_document(
        leaf_der: &[u8],
        pcr0: Vec<u8>,
        nonce: Option<&str>,
        timestamp_ms: u64,
    ) -> Vec<u8> {
        let mut pcr_entries = vec![(Cbor::Integer(0.into()), Cbor::Bytes(pcr0))];
        pcr_entries.push((Cbor::Integer(8.into()), Cbor::Bytes(vec![0x11; 48])));
        let mut map = vec![
            (Cbor::Text("module_id".into()), Cbor::Text("test".into())),
            (Cbor::Text("digest".into()), Cbor::Text("SHA384".into())),
            (
                Cbor::Text("timestamp".into()),
                Cbor::Integer(timestamp_ms.into()),
            ),
            (Cbor::Text("pcrs".into()), Cbor::Map(pcr_entries)),
            (
                Cbor::Text("certificate".into()),
                Cbor::Bytes(leaf_der.to_vec()),
            ),
            (Cbor::Text("cabundle".into()), Cbor::Array(vec![])),
        ];
        if let Some(n) = nonce {
            map.push((
                Cbor::Text("nonce".into()),
                Cbor::Bytes(n.as_bytes().to_vec()),
            ));
        }
        let mut buf = Vec::new();
        ciborium::ser::into_writer(&Cbor::Map(map), &mut buf).unwrap();
        buf
    }

    /// Sign a COSE_Sign1 over the document with the leaf key (ES384).
    fn build_cose(chain: &SyntheticChain, doc_payload: Vec<u8>) -> Vec<u8> {
        let protected = HeaderBuilder::new()
            .algorithm(coset::iana::Algorithm::ES384)
            .build();
        CoseSign1Builder::new()
            .protected(protected)
            .payload(doc_payload)
            .create_signature(b"", |tbs| {
                let sig: Signature = chain.signing_key.sign(tbs);
                sig.to_vec()
            })
            .build()
            .to_vec()
            .unwrap()
    }

    fn ts_now_ms() -> u64 {
        super::now() * 1000
    }

    #[test]
    fn valid_nitro_document_is_trusted() {
        let chain = build_chain();
        let doc = build_document(
            &chain.leaf_der,
            vec![0xAB; 48],
            Some("nitro-nonce"),
            ts_now_ms(),
        );
        let cose = build_cose(&chain, doc);

        let gate = AwsNitroGate::new().with_root_cert_pem(chain.root_pem.clone());
        let policy = AttestationPolicy::production().with_nonce("nitro-nonce");

        let verified = gate
            .verify_document(&cose, &policy)
            .expect("valid COSE doc with good chain must be trusted");
        assert_eq!(verified.verified_by(), TeeProvider::AwsNitro);
        assert_eq!(
            verified.report().measurement.digest,
            hex::encode([0xAB; 48])
        );
    }

    #[test]
    fn tampered_signature_is_rejected() {
        let chain = build_chain();
        let doc = build_document(
            &chain.leaf_der,
            vec![0xAB; 48],
            Some("nitro-nonce"),
            ts_now_ms(),
        );
        let mut cose = build_cose(&chain, doc);
        // Flip a byte deep in the COSE bytes (hits the signature region).
        let n = cose.len();
        cose[n - 5] ^= 0xFF;

        let gate = AwsNitroGate::new().with_root_cert_pem(chain.root_pem.clone());
        let policy = AttestationPolicy::production().with_nonce("nitro-nonce");
        let err = gate.verify_document(&cose, &policy).unwrap_err();
        assert!(
            matches!(
                err,
                AttestationError::Signature { .. } | AttestationError::Malformed { .. }
            ),
            "got {err:?}"
        );
    }

    #[test]
    fn untrusted_root_is_rejected() {
        let chain = build_chain();
        let doc = build_document(
            &chain.leaf_der,
            vec![0xAB; 48],
            Some("nitro-nonce"),
            ts_now_ms(),
        );
        let cose = build_cose(&chain, doc);

        // Verify against the real pinned AWS root — the synthetic leaf does not
        // chain to it, so the chain check must fail.
        let gate = AwsNitroGate::new();
        let policy = AttestationPolicy::production().with_nonce("nitro-nonce");
        let err = gate.verify_document(&cose, &policy).unwrap_err();
        assert!(
            matches!(err, AttestationError::Signature { .. }),
            "got {err:?}"
        );
    }

    #[test]
    fn non_ca_leaf_cannot_act_as_issuer() {
        // Attack: a legitimate non-CA leaf that chains to the trusted root is used
        // to sign a forged sub-leaf carrying forged PCRs. The presented chain is
        // [forged_subleaf, real_leaf, root]; the COSE doc is signed by the forged
        // sub-leaf's key. Per-link signature verification alone would accept this,
        // because real_leaf->root and forged_subleaf->real_leaf both verify. The
        // CA-constraint check must reject it: real_leaf is not a CA.
        let root_kp = KeyPair::generate_for(&PKCS_ECDSA_P384_SHA384).unwrap();
        let mut root_params = CertificateParams::new(vec!["nitro-test-root".into()]).unwrap();
        root_params.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
        let root_cert = root_params.self_signed(&root_kp).unwrap();

        // A real, non-CA leaf (the kind every Nitro customer legitimately holds).
        let real_leaf_kp = KeyPair::generate_for(&PKCS_ECDSA_P384_SHA384).unwrap();
        let real_leaf_params = CertificateParams::new(vec!["real-leaf".into()]).unwrap();
        let real_leaf = real_leaf_params
            .signed_by(&real_leaf_kp, &root_cert, &root_kp)
            .unwrap();

        // Forged sub-leaf signed by the non-CA real leaf; its key signs the COSE.
        let forged_kp = KeyPair::generate_for(&PKCS_ECDSA_P384_SHA384).unwrap();
        let forged_params = CertificateParams::new(vec!["forged-subleaf".into()]).unwrap();
        let forged = forged_params
            .signed_by(&forged_kp, &real_leaf, &real_leaf_kp)
            .unwrap();

        let signing_key = SigningKey::from_pkcs8_der(forged_kp.serialized_der()).unwrap();
        let chain = SyntheticChain {
            root_pem: root_cert.pem(),
            leaf_der: forged.der().to_vec(),
            signing_key,
        };

        // cabundle carries the real (non-CA) leaf as the would-be intermediate.
        let doc = {
            let mut pcr_entries = vec![(Cbor::Integer(0.into()), Cbor::Bytes(vec![0xAB; 48]))];
            pcr_entries.push((Cbor::Integer(8.into()), Cbor::Bytes(vec![0x11; 48])));
            let map = vec![
                (Cbor::Text("module_id".into()), Cbor::Text("test".into())),
                (Cbor::Text("digest".into()), Cbor::Text("SHA384".into())),
                (
                    Cbor::Text("timestamp".into()),
                    Cbor::Integer(ts_now_ms().into()),
                ),
                (Cbor::Text("pcrs".into()), Cbor::Map(pcr_entries)),
                (
                    Cbor::Text("certificate".into()),
                    Cbor::Bytes(forged.der().to_vec()),
                ),
                (
                    Cbor::Text("cabundle".into()),
                    // root-first ordering, as AWS emits it.
                    Cbor::Array(vec![Cbor::Bytes(real_leaf.der().to_vec())]),
                ),
                (
                    Cbor::Text("nonce".into()),
                    Cbor::Bytes(b"nitro-nonce".to_vec()),
                ),
            ];
            let mut buf = Vec::new();
            ciborium::ser::into_writer(&Cbor::Map(map), &mut buf).unwrap();
            buf
        };
        let cose = build_cose(&chain, doc);

        let gate = AwsNitroGate::new().with_root_cert_pem(chain.root_pem.clone());
        let policy = AttestationPolicy::production().with_nonce("nitro-nonce");
        let err = gate
            .verify_document(&cose, &policy)
            .expect_err("non-CA leaf must not be accepted as an issuer");
        assert!(
            matches!(err, AttestationError::Signature { .. }),
            "got {err:?}"
        );
    }

    #[test]
    fn debug_pcr0_all_zero_is_rejected() {
        let chain = build_chain();
        let doc = build_document(
            &chain.leaf_der,
            vec![0x00; 48],
            Some("nitro-nonce"),
            ts_now_ms(),
        );
        let cose = build_cose(&chain, doc);

        let gate = AwsNitroGate::new().with_root_cert_pem(chain.root_pem.clone());
        let policy = AttestationPolicy::production().with_nonce("nitro-nonce");
        let err = gate.verify_document(&cose, &policy).unwrap_err();
        assert!(matches!(err, AttestationError::Claim { .. }), "got {err:?}");
    }

    #[test]
    fn nonce_mismatch_is_rejected() {
        let chain = build_chain();
        let doc = build_document(
            &chain.leaf_der,
            vec![0xAB; 48],
            Some("actual-nonce"),
            ts_now_ms(),
        );
        let cose = build_cose(&chain, doc);

        let gate = AwsNitroGate::new().with_root_cert_pem(chain.root_pem.clone());
        let policy = AttestationPolicy::production().with_nonce("expected-nonce");
        let err = gate.verify_document(&cose, &policy).unwrap_err();
        assert!(matches!(err, AttestationError::Claim { .. }), "got {err:?}");
    }

    #[tokio::test]
    async fn fetch_fails_closed() {
        let gate = AwsNitroGate::new();
        let err = gate.fetch("1.2.3.4", "n").await.unwrap_err();
        assert!(
            matches!(err, AttestationError::Unsatisfiable { .. }),
            "got {err:?}"
        );
    }
}
