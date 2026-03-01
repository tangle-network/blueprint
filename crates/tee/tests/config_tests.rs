//! Tests for TEE configuration types.

use blueprint_tee::config::*;
use blueprint_tee::errors::TeeError;

#[test]
fn test_default_config() {
    let config = TeeConfig::default();
    assert_eq!(config.mode, TeeMode::Disabled);
    assert_eq!(config.requirement, TeeRequirement::Preferred);
    assert!(!config.is_enabled());
}

#[test]
fn test_builder_basic() {
    let config = TeeConfig::builder()
        .mode(TeeMode::Direct)
        .requirement(TeeRequirement::Required)
        .build()
        .expect("should build valid config");

    assert_eq!(config.mode, TeeMode::Direct);
    assert_eq!(config.requirement, TeeRequirement::Required);
    assert!(config.is_enabled());
}

#[test]
fn test_builder_with_providers() {
    let config = TeeConfig::builder()
        .mode(TeeMode::Direct)
        .allow_providers([TeeProvider::IntelTdx, TeeProvider::AmdSevSnp])
        .build()
        .expect("should build valid config");

    match &config.provider_selector {
        TeeProviderSelector::AllowList(providers) => {
            assert_eq!(providers.len(), 2);
            assert!(providers.contains(&TeeProvider::IntelTdx));
            assert!(providers.contains(&TeeProvider::AmdSevSnp));
        }
        TeeProviderSelector::Any => panic!("expected AllowList"),
    }
}

#[test]
fn test_builder_required_disabled_fails() {
    let result = TeeConfig::builder()
        .requirement(TeeRequirement::Required)
        .mode(TeeMode::Disabled)
        .build();

    assert!(result.is_err());
    match result.unwrap_err() {
        TeeError::Config(msg) => {
            assert!(msg.contains("Required"));
            assert!(msg.contains("Disabled"));
        }
        other => panic!("expected Config error, got: {other:?}"),
    }
}

#[test]
fn test_builder_defaults() {
    let config = TeeConfig::builder()
        .build()
        .expect("should build with defaults");
    assert_eq!(config.mode, TeeMode::Disabled);
    assert_eq!(config.requirement, TeeRequirement::Preferred);
    assert_eq!(config.max_attestation_age_secs, 3600);
    assert_eq!(config.key_exchange.session_ttl_secs, 300);
    assert_eq!(config.key_exchange.max_sessions, 64);
}

#[test]
fn test_provider_selector_accepts() {
    let any = TeeProviderSelector::Any;
    assert!(any.accepts(TeeProvider::IntelTdx));
    assert!(any.accepts(TeeProvider::AwsNitro));

    let allow = TeeProviderSelector::AllowList(vec![TeeProvider::IntelTdx]);
    assert!(allow.accepts(TeeProvider::IntelTdx));
    assert!(!allow.accepts(TeeProvider::AwsNitro));
}

#[test]
fn test_tee_mode_serde() {
    let json = serde_json::to_string(&TeeMode::Direct).unwrap();
    assert_eq!(json, "\"direct\"");
    let parsed: TeeMode = serde_json::from_str("\"hybrid\"").unwrap();
    assert_eq!(parsed, TeeMode::Hybrid);
}

#[test]
fn test_tee_provider_display() {
    assert_eq!(TeeProvider::AwsNitro.to_string(), "aws_nitro");
    assert_eq!(TeeProvider::IntelTdx.to_string(), "intel_tdx");
    assert_eq!(TeeProvider::AmdSevSnp.to_string(), "amd_sev_snp");
    assert_eq!(TeeProvider::AzureSnp.to_string(), "azure_snp");
    assert_eq!(TeeProvider::GcpConfidential.to_string(), "gcp_confidential");
}

#[test]
fn test_config_serde_roundtrip() {
    let config = TeeConfig::builder()
        .mode(TeeMode::Direct)
        .requirement(TeeRequirement::Required)
        .max_attestation_age_secs(7200)
        .build()
        .unwrap();

    let json = serde_json::to_string(&config).unwrap();
    let parsed: TeeConfig = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.mode, TeeMode::Direct);
    assert_eq!(parsed.requirement, TeeRequirement::Required);
    assert_eq!(parsed.max_attestation_age_secs, 7200);
}

#[test]
fn test_key_exchange_config_defaults() {
    let config = TeeKeyExchangeConfig::default();
    assert_eq!(config.session_ttl_secs, 300);
    assert_eq!(config.max_sessions, 64);
}
