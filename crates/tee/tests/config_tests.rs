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
fn test_attestation_freshness_provision_time_only() {
    let config = TeeConfig::builder()
        .mode(TeeMode::Direct)
        .attestation_freshness(AttestationFreshnessPolicy::ProvisionTimeOnly)
        .build()
        .unwrap();

    assert_eq!(
        config.attestation_freshness,
        AttestationFreshnessPolicy::ProvisionTimeOnly
    );
}

#[test]
fn test_hybrid_routing_source_default() {
    let config = TeeConfig::default();
    assert!(matches!(
        config.hybrid_routing_source,
        HybridRoutingSource::PolicyFile(_)
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

    let HybridRoutingSource::PolicyFile(path) = &config.hybrid_routing_source;
    assert_eq!(path.to_str().unwrap(), "/etc/tee/routing.yaml");
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
fn test_config_serde_provision_time_only() {
    let config = TeeConfig::builder()
        .mode(TeeMode::Direct)
        .attestation_freshness(AttestationFreshnessPolicy::ProvisionTimeOnly)
        .build()
        .unwrap();

    let json = serde_json::to_string(&config).unwrap();
    let parsed: TeeConfig = serde_json::from_str(&json).unwrap();

    assert_eq!(
        parsed.attestation_freshness,
        AttestationFreshnessPolicy::ProvisionTimeOnly
    );
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
        .attestation_freshness(AttestationFreshnessPolicy::ProvisionTimeOnly)
        .public_key_policy(TeePublicKeyPolicy::Optional)
        .hybrid_routing_source(HybridRoutingSource::PolicyFile(
            "/etc/tee/routing.json".into(),
        ))
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
        HybridRoutingSource::PolicyFile(_)
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

#[test]
fn test_config_from_json_string() {
    let json = r#"{
        "requirement": "required",
        "mode": "direct",
        "provider_selector": "any",
        "key_exchange": {
            "session_ttl_secs": 120,
            "max_sessions": 10,
            "on_chain_verification": false
        },
        "max_attestation_age_secs": 1800,
        "secret_injection": "sealed_only",
        "attestation_freshness": "provision_time_only",
        "public_key_policy": "required",
        "hybrid_routing_source": { "policy_file": "/etc/tee/routing.json" }
    }"#;

    let config: TeeConfig = serde_json::from_str(json).unwrap();
    assert_eq!(config.mode, TeeMode::Direct);
    assert_eq!(config.requirement, TeeRequirement::Required);
    assert_eq!(config.max_attestation_age_secs, 1800);
    assert_eq!(config.key_exchange.session_ttl_secs, 120);
    assert_eq!(config.key_exchange.max_sessions, 10);
}

#[test]
fn test_config_preferred_disabled_is_valid() {
    // Preferred + Disabled is fine — TEE is simply not used
    let config = TeeConfig::builder()
        .requirement(TeeRequirement::Preferred)
        .mode(TeeMode::Disabled)
        .build();
    assert!(config.is_ok());
    assert!(!config.unwrap().is_enabled());
}

#[test]
fn test_builder_mode_without_requirement() {
    // Setting mode without requirement should default to Preferred
    let config = TeeConfig::builder().mode(TeeMode::Remote).build().unwrap();
    assert_eq!(config.requirement, TeeRequirement::Preferred);
    assert_eq!(config.mode, TeeMode::Remote);
    assert!(config.is_enabled());
}

#[test]
fn test_provider_selector_serde() {
    let any = TeeProviderSelector::Any;
    let json = serde_json::to_string(&any).unwrap();
    let parsed: TeeProviderSelector = serde_json::from_str(&json).unwrap();
    assert!(matches!(parsed, TeeProviderSelector::Any));

    let allowlist = TeeProviderSelector::AllowList(vec![TeeProvider::IntelTdx]);
    let json = serde_json::to_string(&allowlist).unwrap();
    let parsed: TeeProviderSelector = serde_json::from_str(&json).unwrap();
    match parsed {
        TeeProviderSelector::AllowList(list) => assert_eq!(list.len(), 1),
        _ => panic!("expected AllowList"),
    }
}

#[test]
fn test_lifecycle_policy_serde() {
    for (policy, expected) in [
        (RuntimeLifecyclePolicy::Container, "\"container\""),
        (RuntimeLifecyclePolicy::CloudManaged, "\"cloud_managed\""),
    ] {
        let json = serde_json::to_string(&policy).unwrap();
        assert_eq!(json, expected);
        let parsed: RuntimeLifecyclePolicy = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, policy);
    }
}

#[test]
fn test_secret_injection_policy_serde() {
    for (policy, expected) in [
        (SecretInjectionPolicy::EnvOrSealed, "\"env_or_sealed\""),
        (SecretInjectionPolicy::SealedOnly, "\"sealed_only\""),
    ] {
        let json = serde_json::to_string(&policy).unwrap();
        assert_eq!(json, expected);
        let parsed: SecretInjectionPolicy = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, policy);
    }
}

#[test]
fn test_public_key_policy_serde() {
    for (policy, expected) in [
        (TeePublicKeyPolicy::Required, "\"required\""),
        (TeePublicKeyPolicy::Optional, "\"optional\""),
    ] {
        let json = serde_json::to_string(&policy).unwrap();
        assert_eq!(json, expected);
        let parsed: TeePublicKeyPolicy = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, policy);
    }
}

// Deserialization invariant enforcement tests (Fix #9)

#[test]
fn test_config_deser_rejects_required_disabled() {
    // JSON with Required + Disabled should fail deserialization
    let json = r#"{
        "requirement": "required",
        "mode": "disabled",
        "provider_selector": "any",
        "key_exchange": { "session_ttl_secs": 300, "max_sessions": 64, "on_chain_verification": false }
    }"#;
    let result: Result<TeeConfig, _> = serde_json::from_str(json);
    assert!(
        result.is_err(),
        "Required + Disabled should be rejected on deserialization"
    );
}

#[test]
fn test_config_deser_rejects_tee_enabled_with_env_or_sealed() {
    // JSON with TEE enabled but EnvOrSealed should fail deserialization
    let json = r#"{
        "requirement": "preferred",
        "mode": "direct",
        "provider_selector": "any",
        "key_exchange": { "session_ttl_secs": 300, "max_sessions": 64, "on_chain_verification": false },
        "secret_injection": "env_or_sealed"
    }"#;
    let result: Result<TeeConfig, _> = serde_json::from_str(json);
    assert!(
        result.is_err(),
        "TEE-enabled config with EnvOrSealed should be rejected"
    );
}

#[test]
fn test_config_deser_accepts_valid_tee_config() {
    let json = r#"{
        "requirement": "required",
        "mode": "direct",
        "provider_selector": "any",
        "key_exchange": { "session_ttl_secs": 300, "max_sessions": 64, "on_chain_verification": false },
        "secret_injection": "sealed_only"
    }"#;
    let config: TeeConfig = serde_json::from_str(json).expect("valid config should deserialize");
    assert_eq!(config.mode, TeeMode::Direct);
    assert_eq!(config.secret_injection, SecretInjectionPolicy::SealedOnly);
}

// Error type tests

#[test]
fn test_error_display_variants() {
    let errors: Vec<TeeError> = vec![
        TeeError::Config("bad config".into()),
        TeeError::AttestationVerification("bad attestation".into()),
        TeeError::AttestationExpired {
            issued_at: 1000,
            max_age_secs: 3600,
        },
        TeeError::UnsupportedProvider("unknown".into()),
        TeeError::DeploymentFailed("deploy failed".into()),
        TeeError::RuntimeUnavailable("runtime down".into()),
        TeeError::KeyExchange("key exchange failed".into()),
        TeeError::SealedSecret("sealed secret error".into()),
        TeeError::MeasurementMismatch {
            expected: "aaa".into(),
            actual: "bbb".into(),
        },
        TeeError::PublicKeyBinding("binding error".into()),
        TeeError::Backend("backend error".into()),
        TeeError::Serialization("ser error".into()),
    ];

    for err in &errors {
        // Every error variant should have a non-empty Display
        let msg = err.to_string();
        assert!(!msg.is_empty(), "error display should be non-empty");
    }

    // Check specific formatting
    let config_err = TeeError::Config("test".into());
    assert!(config_err.to_string().contains("test"));

    let mismatch_err = TeeError::MeasurementMismatch {
        expected: "abc".into(),
        actual: "def".into(),
    };
    let msg = mismatch_err.to_string();
    assert!(msg.contains("abc"));
    assert!(msg.contains("def"));

    let expired_err = TeeError::AttestationExpired {
        issued_at: 1000,
        max_age_secs: 3600,
    };
    let msg = expired_err.to_string();
    assert!(msg.contains("1000"));
    assert!(msg.contains("3600"));
}

#[test]
fn test_error_from_io() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let tee_err: TeeError = io_err.into();
    assert!(tee_err.to_string().contains("file not found"));
}

// TeeRequirements tests

#[test]
fn test_tee_requirements_default() {
    let req = TeeRequirements::default();
    assert_eq!(req.requirement, TeeRequirement::Preferred);
    assert!(matches!(req.providers, TeeProviderSelector::Any));
    assert!(req.min_attestation_age_secs.is_none());
    assert!(!req.is_required());
}

#[test]
fn test_tee_requirements_required() {
    let req = TeeRequirements::required();
    assert_eq!(req.requirement, TeeRequirement::Required);
    assert!(req.is_required());
}

#[test]
fn test_tee_requirements_preferred() {
    let req = TeeRequirements::preferred();
    assert_eq!(req.requirement, TeeRequirement::Preferred);
    assert!(!req.is_required());
}

#[test]
fn test_tee_requirements_serde_roundtrip() {
    let req = TeeRequirements {
        requirement: TeeRequirement::Required,
        providers: TeeProviderSelector::AllowList(vec![
            TeeProvider::AwsNitro,
            TeeProvider::GcpConfidential,
        ]),
        min_attestation_age_secs: Some(7200),
    };

    let json = serde_json::to_string(&req).unwrap();
    let parsed: TeeRequirements = serde_json::from_str(&json).unwrap();

    assert_eq!(parsed.requirement, TeeRequirement::Required);
    assert!(parsed.is_required());
    assert_eq!(parsed.min_attestation_age_secs, Some(7200));
    match &parsed.providers {
        TeeProviderSelector::AllowList(list) => {
            assert_eq!(list.len(), 2);
            assert!(list.contains(&TeeProvider::AwsNitro));
            assert!(list.contains(&TeeProvider::GcpConfidential));
        }
        _ => panic!("expected AllowList"),
    }
}

#[test]
fn test_tee_requirements_serde_minimal() {
    // min_attestation_age_secs should be omitted when None
    let req = TeeRequirements::preferred();
    let json = serde_json::to_string(&req).unwrap();
    assert!(!json.contains("min_attestation_age_secs"));
}

#[test]
fn test_tee_requirements_from_json() {
    let json = r#"{
        "requirement": "required",
        "providers": { "allow_list": ["aws_nitro"] },
        "min_attestation_age_secs": 3600
    }"#;
    let req: TeeRequirements = serde_json::from_str(json).unwrap();
    assert!(req.is_required());
    assert_eq!(req.min_attestation_age_secs, Some(3600));
}

// BackendRegistry::from_env tests
//
// These tests mutate the process environment and must run sequentially
// in a single test to avoid races with other tests.

#[tokio::test]
async fn test_backend_registry_from_env() {
    // SAFETY: all env mutations are in a single test to avoid races.

    // 1. No TEE_BACKEND set → empty registry
    unsafe { std::env::remove_var("TEE_BACKEND") };
    let registry = blueprint_tee::BackendRegistry::from_env().await.unwrap();
    assert!(registry.providers().is_empty());

    // 2. "direct" → IntelTdx registered
    unsafe { std::env::set_var("TEE_BACKEND", "direct") };
    let registry = blueprint_tee::BackendRegistry::from_env().await.unwrap();
    assert!(registry.has_provider(TeeProvider::IntelTdx));

    // 3. "direct-sev-snp" → AmdSevSnp registered
    unsafe { std::env::set_var("TEE_BACKEND", "direct-sev-snp") };
    let registry = blueprint_tee::BackendRegistry::from_env().await.unwrap();
    assert!(registry.has_provider(TeeProvider::AmdSevSnp));

    // 4. Multiple backends → both registered
    unsafe { std::env::set_var("TEE_BACKEND", "direct,direct-sev-snp") };
    let registry = blueprint_tee::BackendRegistry::from_env().await.unwrap();
    assert!(registry.has_provider(TeeProvider::IntelTdx));
    assert!(registry.has_provider(TeeProvider::AmdSevSnp));

    // 5. Unknown backend → error with the backend name
    unsafe { std::env::set_var("TEE_BACKEND", "nonexistent-backend") };
    let result = blueprint_tee::BackendRegistry::from_env().await;
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("nonexistent-backend"));

    // 6. Disabled feature → error mentioning the feature flag
    #[cfg(not(feature = "aws-nitro"))]
    {
        unsafe { std::env::set_var("TEE_BACKEND", "aws-nitro") };
        let result = blueprint_tee::BackendRegistry::from_env().await;
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("aws-nitro"));
    }

    // Cleanup
    unsafe { std::env::remove_var("TEE_BACKEND") };
}

// Attestation freshness is now a single-variant enum
#[test]
fn test_attestation_freshness_serde() {
    let policy = AttestationFreshnessPolicy::ProvisionTimeOnly;
    let json = serde_json::to_string(&policy).unwrap();
    assert_eq!(json, "\"provision_time_only\"");
    let parsed: AttestationFreshnessPolicy = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, AttestationFreshnessPolicy::ProvisionTimeOnly);
}

// HybridRoutingSource serde
#[test]
fn test_hybrid_routing_source_policy_file_serde() {
    let source = HybridRoutingSource::PolicyFile("/etc/tee/routing.json".into());
    let json = serde_json::to_string(&source).unwrap();
    let parsed: HybridRoutingSource = serde_json::from_str(&json).unwrap();
    let HybridRoutingSource::PolicyFile(path) = parsed;
    assert_eq!(path.to_str().unwrap(), "/etc/tee/routing.json");
}
