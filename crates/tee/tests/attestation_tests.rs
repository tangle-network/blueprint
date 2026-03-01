//! Tests for attestation types and verifier trait.

use blueprint_tee::attestation::claims::AttestationClaims;
use blueprint_tee::attestation::report::*;
use blueprint_tee::attestation::verifier::*;
use blueprint_tee::config::TeeProvider;

fn sample_report(provider: TeeProvider) -> AttestationReport {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    AttestationReport {
        provider,
        format: AttestationFormat::Mock,
        issued_at_unix: now,
        measurement: Measurement::sha256("a".repeat(64)),
        public_key_binding: None,
        claims: AttestationClaims::new(),
        evidence: b"test-evidence".to_vec(),
    }
}

#[test]
fn test_measurement_new() {
    let m = Measurement::new("sha384", "abc123");
    assert_eq!(m.algorithm, "sha384");
    assert_eq!(m.digest, "abc123");
}

#[test]
fn test_measurement_sha256() {
    let m = Measurement::sha256("deadbeef");
    assert_eq!(m.algorithm, "sha256");
    assert_eq!(m.digest, "deadbeef");
}

#[test]
fn test_measurement_display() {
    let m = Measurement::sha256("abcdef");
    assert_eq!(m.to_string(), "sha256:abcdef");
}

#[test]
fn test_attestation_report_evidence_digest() {
    let report = sample_report(TeeProvider::IntelTdx);
    let digest = report.evidence_digest();
    assert!(!digest.is_empty());
    // SHA-256 is always 64 hex chars
    assert_eq!(digest.len(), 64);
}

#[test]
fn test_attestation_report_not_expired() {
    let report = sample_report(TeeProvider::IntelTdx);
    assert!(!report.is_expired(3600));
}

#[test]
fn test_attestation_report_expired() {
    let mut report = sample_report(TeeProvider::IntelTdx);
    // Set issued_at to 2 hours ago
    report.issued_at_unix -= 7200;
    assert!(report.is_expired(3600)); // 1 hour max age
    assert!(!report.is_expired(86400)); // 24 hour max age
}

#[test]
fn test_attestation_claims_default() {
    let claims = AttestationClaims::new();
    assert!(!claims.debug_mode);
    assert!(claims.platform_version.is_none());
    assert!(claims.boot_measurements.is_empty());
    assert!(claims.custom.is_empty());
}

#[test]
fn test_attestation_claims_custom() {
    let claims = AttestationClaims::new()
        .with_custom("pcr0", "abc123")
        .with_custom("version", 42);

    assert_eq!(
        claims.get_custom("pcr0"),
        Some(&serde_json::Value::String("abc123".to_string()))
    );
    assert_eq!(claims.get_custom("version"), Some(&serde_json::json!(42)));
    assert_eq!(claims.get_custom("missing"), None);
}

#[test]
fn test_attestation_claims_serde() {
    let claims = AttestationClaims {
        platform_version: Some("2.0".to_string()),
        debug_mode: true,
        boot_measurements: vec!["m1".to_string(), "m2".to_string()],
        ..Default::default()
    };

    let json = serde_json::to_string(&claims).unwrap();
    let parsed: AttestationClaims = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.platform_version.as_deref(), Some("2.0"));
    assert!(parsed.debug_mode);
    assert_eq!(parsed.boot_measurements.len(), 2);
}

#[test]
fn test_verified_attestation() {
    let report = sample_report(TeeProvider::IntelTdx);
    let verified = VerifiedAttestation::new_for_test(report.clone(), TeeProvider::IntelTdx);

    assert_eq!(verified.verified_by(), TeeProvider::IntelTdx);
    assert_eq!(verified.report().provider, TeeProvider::IntelTdx);

    let inner = verified.into_report();
    assert_eq!(inner.provider, TeeProvider::IntelTdx);
}

#[test]
fn test_attestation_report_serde_roundtrip() {
    let report = AttestationReport {
        provider: TeeProvider::AwsNitro,
        format: AttestationFormat::NitroDocument,
        issued_at_unix: 1700000000,
        measurement: Measurement::sha256("a".repeat(64)),
        public_key_binding: Some(PublicKeyBinding {
            public_key: vec![1, 2, 3, 4],
            key_type: "x25519".to_string(),
            binding_digest: "digest123".to_string(),
        }),
        claims: AttestationClaims::new().with_custom("test", true),
        evidence: vec![0xDE, 0xAD],
    };

    let json = serde_json::to_string(&report).unwrap();
    let parsed: AttestationReport = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.provider, TeeProvider::AwsNitro);
    assert_eq!(parsed.format, AttestationFormat::NitroDocument);
    assert!(parsed.public_key_binding.is_some());
    let binding = parsed.public_key_binding.unwrap();
    assert_eq!(binding.key_type, "x25519");
}

// Provider verifier tests (feature-gated)

#[cfg(feature = "tdx")]
mod tdx_verifier {
    use super::*;
    use blueprint_tee::attestation::providers::tdx::TdxVerifier;
    use blueprint_tee::errors::TeeError;

    #[test]
    fn test_tdx_verifier_accepts_tdx_report() {
        let verifier = TdxVerifier::new();
        let report = sample_report(TeeProvider::IntelTdx);
        assert!(verifier.verify(&report).is_ok());
    }

    #[test]
    fn test_tdx_verifier_rejects_wrong_provider() {
        let verifier = TdxVerifier::new();
        let report = sample_report(TeeProvider::AwsNitro);
        assert!(verifier.verify(&report).is_err());
    }

    #[test]
    fn test_tdx_verifier_rejects_debug_mode() {
        let verifier = TdxVerifier::new();
        let mut report = sample_report(TeeProvider::IntelTdx);
        report.claims.debug_mode = true;
        assert!(verifier.verify(&report).is_err());
    }

    #[test]
    fn test_tdx_verifier_allows_debug_when_configured() {
        let mut report = sample_report(TeeProvider::IntelTdx);
        report.claims.debug_mode = true;

        let verifier = TdxVerifier {
            expected_mrtd: None,
            allow_debug: true,
        };
        assert!(verifier.verify(&report).is_ok());
    }

    #[test]
    fn test_tdx_verifier_measurement_check() {
        let verifier = TdxVerifier::new().with_expected_mrtd("x".repeat(64));
        let report = sample_report(TeeProvider::IntelTdx);
        // Report has "a" repeated measurement, verifier expects "x"
        let result = verifier.verify(&report);
        assert!(result.is_err());
        match result.unwrap_err() {
            TeeError::MeasurementMismatch { .. } => {}
            other => panic!("expected MeasurementMismatch, got: {other:?}"),
        }
    }
}

#[cfg(feature = "aws-nitro")]
mod nitro_verifier {
    use super::*;
    use blueprint_tee::attestation::providers::aws_nitro::NitroVerifier;

    #[test]
    fn test_nitro_verifier_accepts_nitro_report() {
        let verifier = NitroVerifier::new();
        let report = sample_report(TeeProvider::AwsNitro);
        assert!(verifier.verify(&report).is_ok());
    }

    #[test]
    fn test_nitro_verifier_rejects_wrong_provider() {
        let verifier = NitroVerifier::new();
        let report = sample_report(TeeProvider::IntelTdx);
        assert!(verifier.verify(&report).is_err());
    }
}

#[cfg(feature = "sev-snp")]
mod sev_snp_verifier {
    use super::*;
    use blueprint_tee::attestation::providers::sev_snp::SevSnpVerifier;
    use blueprint_tee::errors::TeeError;

    #[test]
    fn test_sev_snp_verifier_accepts_sev_report() {
        let verifier = SevSnpVerifier::new();
        let report = sample_report(TeeProvider::AmdSevSnp);
        assert!(verifier.verify(&report).is_ok());
    }

    #[test]
    fn test_sev_snp_verifier_rejects_wrong_provider() {
        let verifier = SevSnpVerifier::new();
        let report = sample_report(TeeProvider::AwsNitro);
        assert!(verifier.verify(&report).is_err());
    }

    #[test]
    fn test_sev_snp_verifier_rejects_debug_mode() {
        let verifier = SevSnpVerifier::new();
        let mut report = sample_report(TeeProvider::AmdSevSnp);
        report.claims.debug_mode = true;
        assert!(verifier.verify(&report).is_err());
    }

    #[test]
    fn test_sev_snp_verifier_measurement_check() {
        let verifier = SevSnpVerifier::new().with_expected_measurement("x".repeat(64));
        let report = sample_report(TeeProvider::AmdSevSnp);
        let result = verifier.verify(&report);
        assert!(result.is_err());
        match result.unwrap_err() {
            TeeError::MeasurementMismatch { .. } => {}
            other => panic!("expected MeasurementMismatch, got: {other:?}"),
        }
    }
}

#[cfg(feature = "azure-snp")]
mod azure_snp_verifier {
    use super::*;
    use blueprint_tee::attestation::providers::azure_snp::AzureSnpVerifier;
    use blueprint_tee::errors::TeeError;

    #[test]
    fn test_azure_snp_verifier_accepts_azure_report() {
        let verifier = AzureSnpVerifier::new();
        let report = sample_report(TeeProvider::AzureSnp);
        assert!(verifier.verify(&report).is_ok());
    }

    #[test]
    fn test_azure_snp_verifier_rejects_wrong_provider() {
        let verifier = AzureSnpVerifier::new();
        let report = sample_report(TeeProvider::IntelTdx);
        assert!(verifier.verify(&report).is_err());
    }

    #[test]
    fn test_azure_snp_verifier_rejects_debug_mode() {
        let verifier = AzureSnpVerifier::new();
        let mut report = sample_report(TeeProvider::AzureSnp);
        report.claims.debug_mode = true;
        assert!(verifier.verify(&report).is_err());
    }

    #[test]
    fn test_azure_snp_verifier_measurement_check() {
        let verifier = AzureSnpVerifier::new().with_expected_measurement("x".repeat(64));
        let report = sample_report(TeeProvider::AzureSnp);
        let result = verifier.verify(&report);
        assert!(result.is_err());
        match result.unwrap_err() {
            TeeError::MeasurementMismatch { .. } => {}
            other => panic!("expected MeasurementMismatch, got: {other:?}"),
        }
    }
}

#[cfg(feature = "gcp-confidential")]
mod gcp_confidential_verifier {
    use super::*;
    use blueprint_tee::attestation::providers::gcp_confidential::GcpConfidentialVerifier;
    use blueprint_tee::errors::TeeError;

    #[test]
    fn test_gcp_verifier_accepts_gcp_report() {
        let verifier = GcpConfidentialVerifier::new();
        let report = sample_report(TeeProvider::GcpConfidential);
        assert!(verifier.verify(&report).is_ok());
    }

    #[test]
    fn test_gcp_verifier_rejects_wrong_provider() {
        let verifier = GcpConfidentialVerifier::new();
        let report = sample_report(TeeProvider::IntelTdx);
        assert!(verifier.verify(&report).is_err());
    }

    #[test]
    fn test_gcp_verifier_rejects_debug_mode() {
        let verifier = GcpConfidentialVerifier::new();
        let mut report = sample_report(TeeProvider::GcpConfidential);
        report.claims.debug_mode = true;
        assert!(verifier.verify(&report).is_err());
    }

    #[test]
    fn test_gcp_verifier_allows_debug_when_configured() {
        let mut report = sample_report(TeeProvider::GcpConfidential);
        report.claims.debug_mode = true;
        let verifier = GcpConfidentialVerifier::new().allow_debug(true);
        assert!(verifier.verify(&report).is_ok());
    }

    #[test]
    fn test_gcp_verifier_measurement_check() {
        let verifier = GcpConfidentialVerifier::new().with_expected_measurement("x".repeat(64));
        let report = sample_report(TeeProvider::GcpConfidential);
        let result = verifier.verify(&report);
        assert!(result.is_err());
        match result.unwrap_err() {
            TeeError::MeasurementMismatch { .. } => {}
            other => panic!("expected MeasurementMismatch, got: {other:?}"),
        }
    }
}

// Edge case tests

#[test]
fn test_attestation_report_expired_at_boundary() {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let mut report = sample_report(TeeProvider::IntelTdx);
    // Set issued_at to exactly max_age ago
    report.issued_at_unix = now - 3600;
    // At the boundary, saturating_sub == max_age, so `> max_age` is false
    assert!(!report.is_expired(3600));
    // One second past the boundary
    report.issued_at_unix = now - 3601;
    assert!(report.is_expired(3600));
}

#[test]
fn test_attestation_report_zero_max_age() {
    let report = sample_report(TeeProvider::IntelTdx);
    // With max_age=0, any elapsed time makes it expired
    // Since the report was just created, the elapsed time is 0,
    // and 0 > 0 is false, so it's not expired
    assert!(!report.is_expired(0));
}

#[test]
fn test_measurement_sha384() {
    let m = Measurement::sha384("abc123");
    assert_eq!(m.algorithm, "sha384");
    assert_eq!(m.digest, "abc123");
    assert_eq!(m.to_string(), "sha384:abc123");
}

#[test]
fn test_attestation_claims_all_fields() {
    let claims = AttestationClaims {
        platform_version: Some("3.0".to_string()),
        debug_mode: false,
        boot_measurements: vec!["pcr0".to_string()],
        signer_id: Some("signer-abc".to_string()),
        product_id: Some("product-xyz".to_string()),
        user_data: Some(vec![1, 2, 3]),
        custom: Default::default(),
    };

    let json = serde_json::to_string(&claims).unwrap();
    let parsed: AttestationClaims = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.signer_id.as_deref(), Some("signer-abc"));
    assert_eq!(parsed.product_id.as_deref(), Some("product-xyz"));
    assert_eq!(parsed.user_data, Some(vec![1, 2, 3]));
}

#[test]
fn test_public_key_binding_serde() {
    let binding = PublicKeyBinding {
        public_key: vec![10, 20, 30],
        key_type: "ed25519".to_string(),
        binding_digest: "deadbeef".to_string(),
    };
    let json = serde_json::to_string(&binding).unwrap();
    let parsed: PublicKeyBinding = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.key_type, "ed25519");
    assert_eq!(parsed.binding_digest, "deadbeef");
}

#[test]
fn test_attestation_format_serde_all_variants() {
    for (format, expected_str) in [
        (AttestationFormat::NitroDocument, "\"nitro_document\""),
        (AttestationFormat::TdxQuote, "\"tdx_quote\""),
        (AttestationFormat::SevSnpReport, "\"sev_snp_report\""),
        (AttestationFormat::AzureMaaToken, "\"azure_maa_token\""),
        (
            AttestationFormat::GcpConfidentialToken,
            "\"gcp_confidential_token\"",
        ),
        (AttestationFormat::Mock, "\"mock\""),
    ] {
        let json = serde_json::to_string(&format).unwrap();
        assert_eq!(json, expected_str, "format {format:?}");
        let parsed: AttestationFormat = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, format);
    }
}

#[test]
fn test_attestation_report_future_issued_at() {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let mut report = sample_report(TeeProvider::IntelTdx);
    // Set issued_at to the future — should never be expired
    report.issued_at_unix = now + 3600;
    assert!(
        !report.is_expired(0),
        "report from the future should not be expired even with max_age=0"
    );
}

#[test]
fn test_measurement_equality() {
    let m1 = Measurement::sha256("abc");
    let m2 = Measurement::sha256("abc");
    let m3 = Measurement::sha256("def");
    assert_eq!(m1, m2);
    assert_ne!(m1, m3);
}

#[test]
fn test_measurement_digest_normalized_to_lowercase() {
    let upper = Measurement::sha256("AABBCC");
    assert_eq!(upper.digest, "aabbcc", "digest should be normalized to lowercase");

    let mixed = Measurement::new("sha384", "AaBbCc");
    assert_eq!(mixed.digest, "aabbcc", "mixed case should be normalized");

    let already_lower = Measurement::sha256("aabbcc");
    assert_eq!(already_lower.digest, "aabbcc", "lowercase should be unchanged");
}

#[test]
fn test_attestation_report_evidence_digest_deterministic() {
    let r1 = sample_report(TeeProvider::IntelTdx);
    let r2 = r1.clone();
    assert_eq!(
        r1.evidence_digest(),
        r2.evidence_digest(),
        "same evidence should produce same digest"
    );
}

#[test]
fn test_attestation_report_empty_evidence_digest() {
    let mut report = sample_report(TeeProvider::IntelTdx);
    report.evidence = Vec::new();
    let digest = report.evidence_digest();
    assert_eq!(digest.len(), 64, "SHA-256 produces 64 hex chars");
    // SHA-256 of empty input is the well-known value
    assert_eq!(
        digest,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
}

// NativeVerifier direct usage tests (feature-gated)

#[cfg(feature = "tdx")]
mod native_verifier_tdx {
    use super::*;
    use blueprint_tee::attestation::providers::native::NativeVerifier;

    #[test]
    fn test_native_tdx_verifier_basic() {
        let verifier = NativeVerifier::tdx();
        let report = sample_report(TeeProvider::IntelTdx);
        assert!(verifier.verify(&report).is_ok());
    }

    #[test]
    fn test_native_tdx_with_measurement() {
        let verifier = NativeVerifier::tdx().with_expected_measurement("a".repeat(64));
        let report = sample_report(TeeProvider::IntelTdx);
        assert!(verifier.verify(&report).is_ok());
    }

    #[test]
    fn test_native_tdx_debug_control() {
        let mut report = sample_report(TeeProvider::IntelTdx);
        report.claims.debug_mode = true;

        let verifier = NativeVerifier::tdx();
        assert!(verifier.verify(&report).is_err());

        let verifier = NativeVerifier::tdx().with_allow_debug(true);
        assert!(verifier.verify(&report).is_ok());
    }
}

#[cfg(feature = "sev-snp")]
mod native_verifier_sev {
    use super::*;
    use blueprint_tee::attestation::providers::native::NativeVerifier;

    #[test]
    fn test_native_sev_verifier_basic() {
        let verifier = NativeVerifier::sev_snp();
        let report = sample_report(TeeProvider::AmdSevSnp);
        assert!(verifier.verify(&report).is_ok());
    }

    #[test]
    fn test_native_sev_rejects_tdx_report() {
        let verifier = NativeVerifier::sev_snp();
        let report = sample_report(TeeProvider::IntelTdx);
        assert!(verifier.verify(&report).is_err());
    }
}

// Test the new allow_debug() builder methods

#[cfg(feature = "tdx")]
#[test]
fn test_tdx_verifier_allow_debug_builder() {
    use blueprint_tee::attestation::providers::tdx::TdxVerifier;

    let mut report = sample_report(TeeProvider::IntelTdx);
    report.claims.debug_mode = true;

    let verifier = TdxVerifier::new().allow_debug(true);
    assert!(verifier.verify(&report).is_ok());
}

#[cfg(feature = "sev-snp")]
#[test]
fn test_sev_snp_verifier_allow_debug_builder() {
    use blueprint_tee::attestation::providers::sev_snp::SevSnpVerifier;

    let mut report = sample_report(TeeProvider::AmdSevSnp);
    report.claims.debug_mode = true;

    let verifier = SevSnpVerifier::new().allow_debug(true);
    assert!(verifier.verify(&report).is_ok());
}

#[cfg(feature = "azure-snp")]
#[test]
fn test_azure_snp_verifier_allow_debug_builder() {
    use blueprint_tee::attestation::providers::azure_snp::AzureSnpVerifier;

    let mut report = sample_report(TeeProvider::AzureSnp);
    report.claims.debug_mode = true;

    let verifier = AzureSnpVerifier::new().allow_debug(true);
    assert!(verifier.verify(&report).is_ok());
}

#[cfg(feature = "aws-nitro")]
#[test]
fn test_nitro_verifier_measurement_check() {
    use blueprint_tee::attestation::providers::aws_nitro::NitroVerifier;

    let verifier = NitroVerifier::new().with_expected_pcr0("x".repeat(64));
    let report = sample_report(TeeProvider::AwsNitro);
    let result = verifier.verify(&report);
    assert!(result.is_err());
    match result.unwrap_err() {
        blueprint_tee::errors::TeeError::MeasurementMismatch { .. } => {}
        other => panic!("expected MeasurementMismatch, got: {other:?}"),
    }
}

#[cfg(feature = "aws-nitro")]
#[test]
fn test_nitro_verifier_debug_mode() {
    use blueprint_tee::attestation::providers::aws_nitro::NitroVerifier;

    let mut report = sample_report(TeeProvider::AwsNitro);
    report.claims.debug_mode = true;

    let verifier = NitroVerifier::new();
    assert!(verifier.verify(&report).is_err());

    let verifier = NitroVerifier::new().allow_debug(true);
    assert!(verifier.verify(&report).is_ok());
}
