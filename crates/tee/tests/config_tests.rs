//! Tests for TEE configuration types.

use blueprint_tee::config::*;
use blueprint_tee::errors::TeeError;
use std::time::Duration;

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
    assert!(!config.on_chain_verification);
}

#[test]
fn test_secret_injection_sealed_only_when_tee_enabled() {
    let config = TeeConfig::builder().mode(TeeMode::Direct).build().unwrap();
    assert_eq!(config.secret_injection, SecretInjectionPolicy::SealedOnly);
}

#[test]
fn test_secret_injection_env_or_sealed_when_disabled() {
    let config = TeeConfig::builder()
        .mode(TeeMode::Disabled)
        .build()
        .unwrap();
    assert_eq!(config.secret_injection, SecretInjectionPolicy::EnvOrSealed);
}

#[test]
fn test_lifecycle_policy_cloud_managed_when_enabled() {
    let config = TeeConfig::builder().mode(TeeMode::Remote).build().unwrap();
    assert_eq!(
        config.lifecycle_policy(),
        RuntimeLifecyclePolicy::CloudManaged
    );
}

#[test]
fn test_lifecycle_policy_container_when_disabled() {
    let config = TeeConfig::default();
    assert_eq!(config.lifecycle_policy(), RuntimeLifecyclePolicy::Container);
}

#[test]
fn test_attestation_freshness_default() {
    let config = TeeConfig::default();
    assert!(matches!(
        config.attestation_freshness,
        AttestationFreshnessPolicy::ProvisionTimeOnly
    ));
}

#[test]
fn test_attestation_freshness_periodic() {
    let config = TeeConfig::builder()
        .mode(TeeMode::Direct)
        .attestation_freshness(AttestationFreshnessPolicy::PeriodicRefresh {
            interval: Duration::from_secs(3600),
        })
        .build()
        .unwrap();

    match &config.attestation_freshness {
        AttestationFreshnessPolicy::PeriodicRefresh { interval } => {
            assert_eq!(interval.as_secs(), 3600);
        }
        _ => panic!("expected PeriodicRefresh"),
    }
}

#[test]
fn test_hybrid_routing_source_default() {
    let config = TeeConfig::default();
    assert!(matches!(
        config.hybrid_routing_source,
        HybridRoutingSource::ContractDriven
    ));
}

#[test]
fn test_hybrid_routing_source_policy_file() {
    let config = TeeConfig::builder()
        .mode(TeeMode::Hybrid)
        .hybrid_routing_source(HybridRoutingSource::PolicyFile(
            "/etc/tee/routing.yaml".into(),
        ))
        .build()
        .unwrap();

    match &config.hybrid_routing_source {
        HybridRoutingSource::PolicyFile(path) => {
            assert_eq!(path.to_str().unwrap(), "/etc/tee/routing.yaml");
        }
        _ => panic!("expected PolicyFile"),
    }
}

#[test]
fn test_public_key_policy_default() {
    let config = TeeConfig::default();
    assert_eq!(config.public_key_policy, TeePublicKeyPolicy::Required);
}

// Edge case tests

#[test]
fn test_provider_selector_empty_allowlist() {
    let allow = TeeProviderSelector::AllowList(vec![]);
    // An empty allowlist accepts nothing
    assert!(!allow.accepts(TeeProvider::IntelTdx));
    assert!(!allow.accepts(TeeProvider::AwsNitro));
}

#[test]
fn test_config_serde_with_periodic_refresh() {
    let config = TeeConfig::builder()
        .mode(TeeMode::Direct)
        .attestation_freshness(AttestationFreshnessPolicy::PeriodicRefresh {
            interval: Duration::from_secs(1800),
        })
        .build()
        .unwrap();

    let json = serde_json::to_string(&config).unwrap();
    let parsed: TeeConfig = serde_json::from_str(&json).unwrap();

    match &parsed.attestation_freshness {
        AttestationFreshnessPolicy::PeriodicRefresh { interval } => {
            assert_eq!(interval.as_secs(), 1800);
        }
        _ => panic!("expected PeriodicRefresh"),
    }
}

#[test]
fn test_secret_injection_sealed_only_for_all_tee_modes() {
    for mode in [TeeMode::Direct, TeeMode::Remote, TeeMode::Hybrid] {
        let config = TeeConfig::builder().mode(mode).build().unwrap();
        assert_eq!(
            config.secret_injection,
            SecretInjectionPolicy::SealedOnly,
            "mode {mode:?} should enforce SealedOnly"
        );
    }
}

#[test]
fn test_lifecycle_policy_for_all_tee_modes() {
    for mode in [TeeMode::Direct, TeeMode::Remote, TeeMode::Hybrid] {
        let config = TeeConfig::builder().mode(mode).build().unwrap();
        assert_eq!(
            config.lifecycle_policy(),
            RuntimeLifecyclePolicy::CloudManaged,
            "mode {mode:?} should use CloudManaged lifecycle"
        );
    }
}

#[test]
fn test_builder_all_options() {
    let config = TeeConfig::builder()
        .mode(TeeMode::Hybrid)
        .requirement(TeeRequirement::Required)
        .provider_selector(TeeProviderSelector::AllowList(vec![
            TeeProvider::IntelTdx,
            TeeProvider::AmdSevSnp,
        ]))
        .key_exchange(TeeKeyExchangeConfig {
            session_ttl_secs: 600,
            max_sessions: 128,
            on_chain_verification: true,
        })
        .max_attestation_age_secs(7200)
        .attestation_freshness(AttestationFreshnessPolicy::PeriodicRefresh {
            interval: Duration::from_secs(900),
        })
        .public_key_policy(TeePublicKeyPolicy::Optional)
        .hybrid_routing_source(HybridRoutingSource::ContractDriven)
        .build()
        .unwrap();

    assert_eq!(config.mode, TeeMode::Hybrid);
    assert_eq!(config.requirement, TeeRequirement::Required);
    assert_eq!(config.max_attestation_age_secs, 7200);
    assert_eq!(config.key_exchange.session_ttl_secs, 600);
    assert_eq!(config.key_exchange.max_sessions, 128);
    assert!(config.key_exchange.on_chain_verification);
    assert_eq!(config.public_key_policy, TeePublicKeyPolicy::Optional);
    assert!(matches!(
        config.hybrid_routing_source,
        HybridRoutingSource::ContractDriven
    ));
}

#[test]
fn test_tee_mode_all_variants_serde() {
    for (mode, expected) in [
        (TeeMode::Disabled, "\"disabled\""),
        (TeeMode::Direct, "\"direct\""),
        (TeeMode::Remote, "\"remote\""),
        (TeeMode::Hybrid, "\"hybrid\""),
    ] {
        let json = serde_json::to_string(&mode).unwrap();
        assert_eq!(json, expected);
        let parsed: TeeMode = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, mode);
    }
}

#[test]
fn test_tee_requirement_serde() {
    for (req, expected) in [
        (TeeRequirement::Preferred, "\"preferred\""),
        (TeeRequirement::Required, "\"required\""),
    ] {
        let json = serde_json::to_string(&req).unwrap();
        assert_eq!(json, expected);
        let parsed: TeeRequirement = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, req);
    }
}

#[test]
fn test_key_exchange_config_serde() {
    let config = TeeKeyExchangeConfig {
        session_ttl_secs: 600,
        max_sessions: 32,
        on_chain_verification: true,
    };
    let json = serde_json::to_string(&config).unwrap();
    let parsed: TeeKeyExchangeConfig = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.session_ttl_secs, 600);
    assert_eq!(parsed.max_sessions, 32);
    assert!(parsed.on_chain_verification);
}
