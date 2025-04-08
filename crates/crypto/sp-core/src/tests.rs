use blueprint_crypto_core::{KeyType, aggregation::AggregatableSignature};

use crate::error::SpCoreError;

use super::*;

mod ecdsa_crypto_tests {
    use super::*;
    blueprint_crypto_core::impl_crypto_tests!(SpEcdsa, SpEcdsaPair, SpEcdsaSignature);
}

mod ed25519_crypto_tests {
    use super::*;
    blueprint_crypto_core::impl_crypto_tests!(SpEd25519, SpEd25519Pair, SpEd25519Signature);
}

mod sr25519_crypto_tests {
    use super::*;
    blueprint_crypto_core::impl_crypto_tests!(SpSr25519, SpSr25519Pair, SpSr25519Signature);
}

mod bls381_tests {
    use crate::error::SpCoreError;

    use super::*;
    use blueprint_crypto_core::{KeyType, aggregation::AggregatableSignature};
    use sp_core::Pair;

    #[test]
    fn test_bls381_key_generation() {
        // Test random key generation
        let secret = SpBls381::generate_with_seed(None).unwrap();
        let public = SpBls381::public_from_secret(&secret);

        // Test generation with seed
        let seed: [u8; 32] = [1u8; 32];
        let secret_with_seed = SpBls381::generate_with_seed(Some(&seed)).unwrap();
        let public_with_seed = SpBls381::public_from_secret(&secret_with_seed);

        assert_ne!(
            secret.to_raw_vec(),
            secret_with_seed.to_raw_vec(),
            "Random and seeded keys should be different"
        );
        assert_ne!(public, public_with_seed, "Public keys should be different");
    }

    #[test]
    fn test_bls381_sign_and_verify() {
        let seed: [u8; 32] = [1u8; 32];
        let mut secret = SpBls381::generate_with_seed(Some(&seed)).unwrap();
        let public = SpBls381::public_from_secret(&secret);

        // Test normal signing
        let message = b"Hello, world!";
        let signature = SpBls381::sign_with_secret(&mut secret, message).unwrap();
        assert!(
            SpBls381::verify(&public, message, &signature),
            "Signature verification failed"
        );

        // Test pre-hashed signing
        let hashed_msg = [42u8; 32];
        let signature = SpBls381::sign_with_secret_pre_hashed(&mut secret, &hashed_msg).unwrap();

        // Verify with wrong message should fail
        let wrong_message = b"Wrong message";
        assert!(
            !SpBls381::verify(&public, wrong_message, &signature),
            "Verification should fail with wrong message"
        );
    }

    #[test]
    fn test_bls381_key_serialization() {
        let seed: [u8; 32] = [1u8; 32];
        let secret = SpBls381::generate_with_seed(Some(&seed)).unwrap();
        let public = SpBls381::public_from_secret(&secret);

        // Test signing key serialization using seed
        let serialized = serde_json::to_vec(&seed).unwrap();
        let deserialized: SpBls381Pair = serde_json::from_slice(&serialized).unwrap();
        assert_eq!(
            secret.to_raw_vec(),
            deserialized.to_raw_vec(),
            "SigningKey serialization roundtrip failed"
        );

        // Test verifying key serialization
        let serialized = serde_json::to_string(&public).unwrap();
        let deserialized = serde_json::from_str(&serialized).unwrap();
        assert_eq!(
            public, deserialized,
            "VerifyingKey serialization roundtrip failed"
        );
    }

    #[test]
    fn test_bls381_signature_serialization() {
        let seed: [u8; 32] = [1u8; 32];
        let mut secret = SpBls381::generate_with_seed(Some(&seed)).unwrap();
        let message = b"Test message";
        let signature = SpBls381::sign_with_secret(&mut secret, message).unwrap();

        // Test signature serialization
        let serialized = serde_json::to_string(&signature).unwrap();
        let deserialized: SpBls381Signature = serde_json::from_str(&serialized).unwrap();
        assert_eq!(
            signature, deserialized,
            "Signature serialization roundtrip failed"
        );
    }

    #[test]
    fn test_bls381_aggregation_success() {
        let message = b"Test message";

        // Generate 3 test keys
        let secrets: Vec<SpBls381Pair> = (0..3)
            .map(|i| SpBls381::generate_with_seed(Some(&[i as u8; 32])).unwrap())
            .collect();
        let publics: Vec<SpBls381Public> =
            secrets.iter().map(SpBls381::public_from_secret).collect();

        // Create individual signatures
        let signatures: Vec<SpBls381Signature> = secrets
            .iter()
            .map(|s| {
                let mut secret = s.clone();
                SpBls381::sign_with_secret(&mut secret, message).unwrap()
            })
            .collect();

        // Aggregate signatures
        let (aggregated_sig, aggregated_public) =
            SpBls381::aggregate(&signatures, &publics).unwrap();

        // Verify aggregate signature against all public keys
        assert!(
            SpBls381::verify_aggregate(message, &aggregated_sig, &aggregated_public).unwrap(),
            "Aggregate verification failed with valid signatures"
        );
    }

    #[test]
    fn test_bls381_aggregation_failure() {
        let message = b"Test message";
        let different_message = b"Different message";

        // Generate 3 valid keys
        let secrets = vec![
            SpBls381::generate_with_seed(Some(&[1u8; 32])).unwrap(),
            SpBls381::generate_with_seed(Some(&[2u8; 32])).unwrap(),
            SpBls381::generate_with_seed(Some(&[3u8; 32])).unwrap(),
        ];

        let publics: Vec<SpBls381Public> =
            secrets.iter().map(SpBls381::public_from_secret).collect();

        // Create two valid signatures and one invalid
        let mut signatures: Vec<SpBls381Signature> = secrets[0..2]
            .iter()
            .map(|s| {
                let mut secret = s.clone();
                SpBls381::sign_with_secret(&mut secret, message).unwrap()
            })
            .collect();

        // Add signature for different message
        let mut different_secret = secrets[2].clone();
        let different_signature =
            SpBls381::sign_with_secret(&mut different_secret, different_message).unwrap();
        signatures.push(different_signature);

        let (aggregated_sig, aggregated_public) =
            SpBls381::aggregate(&signatures, &publics).unwrap();
        assert!(
            !SpBls381::verify_aggregate(message, &aggregated_sig, &aggregated_public).unwrap(),
            "Aggregate verification should fail with mixed messages"
        );
    }

    #[test]
    fn test_bls381_aggregation_mismatched_keys() {
        let message = b"Test message";

        // Generate valid set
        let valid_secrets = (0..2)
            .map(|i| SpBls381::generate_with_seed(Some(&[i as u8; 32])).unwrap())
            .collect::<Vec<_>>();
        let valid_publics = valid_secrets
            .iter()
            .map(SpBls381::public_from_secret)
            .collect::<Vec<_>>();

        // Generate unrelated key
        let unrelated_secret = SpBls381::generate_with_seed(Some(&[99u8; 32])).unwrap();
        let unrelated_public = SpBls381::public_from_secret(&unrelated_secret);

        // Create signatures with one invalid public key
        let mut mixed_publics = valid_publics.clone();
        mixed_publics[1] = unrelated_public;

        // Create valid signatures
        let signatures = valid_secrets
            .iter()
            .map(|s| {
                let mut secret = s.clone();
                SpBls381::sign_with_secret(&mut secret, message).unwrap()
            })
            .collect::<Vec<_>>();

        let (aggregated_sig, aggregated_public) =
            SpBls381::aggregate(&signatures, &mixed_publics).unwrap();
        assert!(
            !SpBls381::verify_aggregate(message, &aggregated_sig, &aggregated_public).unwrap(),
            "Aggregate verification should fail with mismatched keys"
        );
    }

    #[test]
    fn test_bls381_empty_aggregation() {
        let empty_sigs: Vec<SpBls381Signature> = vec![];
        let agg_result = SpBls381::aggregate(&empty_sigs, &[]);
        assert!(
            matches!(agg_result, Err(SpCoreError::InvalidInput(_))),
            "Empty aggregation should return InvalidInput error"
        );
    }
}

mod bls377_tests {
    use super::*;
    use blueprint_crypto_core::KeyType;
    use sp_core::Pair;

    #[test]
    fn test_bls377_key_generation() {
        // Test random key generation
        let secret = SpBls377::generate_with_seed(None).unwrap();
        let public = SpBls377::public_from_secret(&secret);

        // Test generation with seed
        let seed: [u8; 32] = [1u8; 32];
        let secret_with_seed = SpBls377::generate_with_seed(Some(&seed)).unwrap();
        let public_with_seed = SpBls377::public_from_secret(&secret_with_seed);

        assert_ne!(
            secret.to_raw_vec(),
            secret_with_seed.to_raw_vec(),
            "Random and seeded keys should be different"
        );
        assert_ne!(public, public_with_seed, "Public keys should be different");
    }

    #[test]
    fn test_bls377_sign_and_verify() {
        let seed: [u8; 32] = [1u8; 32];
        let mut secret = SpBls377::generate_with_seed(Some(&seed)).unwrap();
        let public = SpBls377::public_from_secret(&secret);

        // Test normal signing
        let message = b"Hello, world!";
        let signature = SpBls377::sign_with_secret(&mut secret, message).unwrap();
        assert!(
            SpBls377::verify(&public, message, &signature),
            "Signature verification failed"
        );

        // Test pre-hashed signing
        let hashed_msg = [42u8; 32];
        let signature = SpBls377::sign_with_secret_pre_hashed(&mut secret, &hashed_msg).unwrap();

        // Verify with wrong message should fail
        let wrong_message = b"Wrong message";
        assert!(
            !SpBls377::verify(&public, wrong_message, &signature),
            "Verification should fail with wrong message"
        );
    }

    #[test]
    fn test_bls377_key_serialization() {
        let seed: [u8; 32] = [1u8; 32];
        let secret = SpBls377::generate_with_seed(Some(&seed)).unwrap();
        let public = SpBls377::public_from_secret(&secret);

        // Test signing key serialization using seed
        let serialized = serde_json::to_vec(&seed).unwrap();
        let deserialized: SpBls377Pair = serde_json::from_slice(&serialized).unwrap();
        assert_eq!(
            secret.to_raw_vec(),
            deserialized.to_raw_vec(),
            "SigningKey serialization roundtrip failed"
        );

        // Test verifying key serialization
        let serialized = serde_json::to_string(&public).unwrap();
        let deserialized = serde_json::from_str(&serialized).unwrap();
        assert_eq!(
            public, deserialized,
            "VerifyingKey serialization roundtrip failed"
        );
    }

    #[test]
    fn test_bls377_signature_serialization() {
        let seed: [u8; 32] = [1u8; 32];
        let mut secret = SpBls377::generate_with_seed(Some(&seed)).unwrap();
        let message = b"Test message";
        let signature = SpBls377::sign_with_secret(&mut secret, message).unwrap();

        // Test signature serialization
        let serialized = serde_json::to_string(&signature).unwrap();
        let deserialized: SpBls377Signature = serde_json::from_str(&serialized).unwrap();
        assert_eq!(
            signature, deserialized,
            "Signature serialization roundtrip failed"
        );
    }
}
#[test]
fn test_bls377_signature_aggregation() {
    let message = b"Test aggregation message";
    let mut secrets = (0..3)
        .map(|i| SpBls377::generate_with_seed(Some(&[i as u8; 32])).unwrap())
        .collect::<Vec<_>>();
    let publics = secrets
        .iter()
        .map(SpBls377::public_from_secret)
        .collect::<Vec<_>>();

    // Generate signatures
    let signatures = secrets
        .iter_mut()
        .map(|s| SpBls377::sign_with_secret(s, message).unwrap())
        .collect::<Vec<_>>();

    // Aggregate and verify
    let (aggregated_sig, aggregated_public) = SpBls377::aggregate(&signatures, &publics).unwrap();
    assert!(
        SpBls377::verify_aggregate(message, &aggregated_sig, &aggregated_public).unwrap(),
        "Valid aggregate signature should verify"
    );
}

#[test]
fn test_bls377_aggregation_with_invalid_signature() {
    let message = b"Test aggregation message";
    let mut secrets = (0..2)
        .map(|i| SpBls377::generate_with_seed(Some(&[i as u8; 32])).unwrap())
        .collect::<Vec<_>>();
    let publics = secrets
        .iter()
        .map(SpBls377::public_from_secret)
        .collect::<Vec<_>>();

    // Generate one valid and one invalid signature
    let signatures = vec![
        SpBls377::sign_with_secret(&mut secrets[0], message).unwrap(),
        SpBls377::sign_with_secret(&mut secrets[1], b"Different message").unwrap(),
    ];

    // Aggregation should succeed but verification should fail
    let (aggregated_sig, aggregated_public) = SpBls377::aggregate(&signatures, &publics).unwrap();
    assert!(
        !SpBls377::verify_aggregate(message, &aggregated_sig, &aggregated_public).unwrap(),
        "Aggregate with invalid signature should fail verification"
    );
}

#[test]
fn test_bls377_empty_aggregation() {
    let empty_sigs: Vec<SpBls377Signature> = vec![];
    let agg_result = SpBls377::aggregate(&empty_sigs, &[]);
    assert!(
        matches!(agg_result, Err(SpCoreError::InvalidInput(_))),
        "Empty aggregation should return InvalidInput error"
    );
}

#[test]
fn test_bls377_aggregation_with_mismatched_keys() {
    let message = b"Test message";

    // Generate valid set
    let mut valid_secrets = (0..2)
        .map(|i| SpBls377::generate_with_seed(Some(&[i as u8; 32])).unwrap())
        .collect::<Vec<_>>();
    let valid_publics = valid_secrets
        .iter()
        .map(SpBls377::public_from_secret)
        .collect::<Vec<_>>();

    // Generate unrelated key
    let unrelated_secret = SpBls377::generate_with_seed(Some(&[99u8; 32])).unwrap();
    let unrelated_public = SpBls377::public_from_secret(&unrelated_secret);

    // Create signatures with one invalid public key
    let mut mixed_publics = valid_publics.clone();
    mixed_publics[1] = unrelated_public;

    // Create valid signatures
    let signatures = valid_secrets
        .iter_mut()
        .map(|s| SpBls377::sign_with_secret(&mut s.clone(), message).unwrap())
        .collect::<Vec<_>>();

    let (aggregated_sig, aggregated_public) =
        SpBls377::aggregate(&signatures, &mixed_publics).unwrap();
    assert!(
        !SpBls377::verify_aggregate(message, &aggregated_sig, &aggregated_public).unwrap(),
        "Aggregate verification should fail with mismatched public keys"
    );
}
