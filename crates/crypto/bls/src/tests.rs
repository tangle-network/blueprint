use super::*;
use crate::{
    bls377::{W3fBls377, W3fBls377Public, W3fBls377Secret},
    bls381::{W3fBls381, W3fBls381Public, W3fBls381Secret},
};
use blueprint_crypto_core::KeyType;
use blueprint_std::string::ToString;

// Helper function to generate test message
fn test_message() -> Vec<u8> {
    b"test message".to_vec()
}

mod bls377_crypto_tests {
    use super::bls377::{W3fBls377, W3fBls377Secret, W3fBls377Signature};
    blueprint_crypto_core::impl_crypto_tests!(W3fBls377, W3fBls377Secret, W3fBls377Signature);
}

mod bls381_crypto_tests {
    use super::bls381::{W3fBls381, W3fBls381Secret, W3fBls381Signature};
    blueprint_crypto_core::impl_crypto_tests!(W3fBls381, W3fBls381Secret, W3fBls381Signature);
}

mod bls377_tests {
    use crate::{bls377::W3fBls377Signature, error::BlsError};

    use super::*;
    use ::tnt_bls::SerializableToBytes;
    use blueprint_crypto_core::aggregation::AggregatableSignature;
    use blueprint_crypto_hashing::sha2_256;

    #[test]
    fn test_key_generation() {
        // Test seed-based generation is deterministic
        let seed = [0u8; 32];
        let secret1 = W3fBls377::generate_with_seed(Some(&seed)).unwrap();
        let public1 = W3fBls377::public_from_secret(&secret1);

        let secret2 = W3fBls377::generate_with_seed(Some(&seed)).unwrap();
        let public2 = W3fBls377::public_from_secret(&secret2);

        // Same seed should produce same keys
        assert_eq!(public1, public2);

        let secret1_hex = hex::encode(secret1.0.to_bytes());
        let secret2_hex = hex::encode(secret2.0.to_bytes());

        // Test string-based generation is deterministic
        let secret_from_str1 = W3fBls377::generate_with_string(secret1_hex).unwrap();
        let public_from_str1 = W3fBls377::public_from_secret(&secret_from_str1);

        let secret_from_str2 = W3fBls377::generate_with_string(secret2_hex).unwrap();
        let public_from_str2 = W3fBls377::public_from_secret(&secret_from_str2);

        // Same string should produce same keys
        assert_eq!(public_from_str1, public_from_str2);
    }

    #[test]
    fn test_signing_and_verification() {
        let seed = [0u8; 32];
        let mut secret = W3fBls377::generate_with_seed(Some(&seed)).unwrap();
        let public = W3fBls377::public_from_secret(&secret);
        let message = test_message();

        // Test normal signing
        let signature = W3fBls377::sign_with_secret(&mut secret, &message).unwrap();
        assert!(W3fBls377::verify(&public, &message, &signature));

        // Test pre-hashed signing
        let hashed_msg = sha2_256(&message);
        let signature_pre_hashed =
            W3fBls377::sign_with_secret_pre_hashed(&mut secret, &hashed_msg).unwrap();
        assert!(W3fBls377::verify(
            &public,
            &hashed_msg,
            &signature_pre_hashed
        ));

        // Test invalid signature
        let wrong_message = b"wrong message".to_vec();
        assert!(!W3fBls377::verify(&public, &wrong_message, &signature));
    }

    #[test]
    fn test_serialization() {
        let seed = [0u8; 32];
        let secret = W3fBls377::generate_with_seed(Some(&seed)).unwrap();
        let public = W3fBls377::public_from_secret(&secret);

        // Test public key serialization
        let public_bytes = to_bytes(public.0);
        let public_deserialized: W3fBls377Public = W3fBls377Public(from_bytes(&public_bytes));
        assert_eq!(public, public_deserialized);

        // Test secret key serialization
        let secret_bytes = to_bytes(secret.0.clone());
        let secret_deserialized: W3fBls377Secret = W3fBls377Secret(from_bytes(&secret_bytes));
        assert_eq!(secret, secret_deserialized);
    }

    #[test]
    fn test_error_handling() {
        // Test invalid seed
        let result = W3fBls377::generate_with_string("invalid hex".to_string());
        assert!(result.is_err());

        // Test empty seed
        let result = W3fBls377::generate_with_string("".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_signature_aggregation_success() {
        let message = test_message();

        // Generate 3 test keys
        let secrets: Vec<W3fBls377Secret> = (0..3)
            .map(|i| W3fBls377::generate_with_seed(Some(&[i as u8; 32])).unwrap())
            .collect();
        let publics: Vec<W3fBls377Public> =
            secrets.iter().map(W3fBls377::public_from_secret).collect();

        // Create individual signatures
        let signatures: Vec<W3fBls377Signature> = secrets
            .iter()
            .map(|s| {
                let mut secret = s.clone();
                W3fBls377::sign_with_secret(&mut secret, &message).unwrap()
            })
            .collect();

        // Aggregate signatures
        let (aggregated_sig, aggregated_public) =
            W3fBls377::aggregate(&signatures, &publics).unwrap();

        // Verify aggregate signature against all public keys
        assert!(
            W3fBls377::verify_aggregate(&message, &aggregated_sig, &aggregated_public).unwrap()
        );
    }

    #[test]
    fn test_signature_aggregation_failure() {
        let message = test_message();
        let different_message = b"completely different message".to_vec();

        // Generate 3 valid keys
        let secrets = vec![
            W3fBls377::generate_with_seed(Some(&[1u8; 32])).unwrap(),
            W3fBls377::generate_with_seed(Some(&[2u8; 32])).unwrap(),
            W3fBls377::generate_with_seed(Some(&[3u8; 32])).unwrap(),
        ];

        let publics: Vec<W3fBls377Public> =
            secrets.iter().map(W3fBls377::public_from_secret).collect();

        // Create two valid signatures for original message
        let mut valid_signatures: Vec<W3fBls377Signature> = secrets[0..2]
            .iter()
            .map(|s| {
                let mut secret = s.clone();
                W3fBls377::sign_with_secret(&mut secret, &message).unwrap()
            })
            .collect();

        // Create one signature for different message
        let mut different_secret = secrets[2].clone();
        let different_signature =
            W3fBls377::sign_with_secret(&mut different_secret, &different_message).unwrap();
        valid_signatures.push(different_signature);

        // Properly aggregate the mismatched signatures with their publics
        let (aggregated_sig, aggregated_public) =
            W3fBls377::aggregate(&valid_signatures, &publics).unwrap();

        // Verification should fail because aggregation included invalid signature
        assert!(
            !W3fBls377::verify_aggregate(&message, &aggregated_sig, &aggregated_public).unwrap()
        );
    }

    #[test]
    fn test_aggregation_with_mismatched_keys() {
        let message = test_message();

        // Generate valid set
        let valid_secrets = (0..2)
            .map(|i| W3fBls377::generate_with_seed(Some(&[i as u8; 32])).unwrap())
            .collect::<Vec<_>>();
        let valid_publics = valid_secrets
            .iter()
            .map(W3fBls377::public_from_secret)
            .collect::<Vec<_>>();

        // Generate unrelated key
        let unrelated_secret = W3fBls377::generate_with_seed(Some(&[99u8; 32])).unwrap();
        let unrelated_public = W3fBls377::public_from_secret(&unrelated_secret);

        // Create signatures with one invalid public key
        let mut mixed_publics = valid_publics.clone();
        mixed_publics[1] = unrelated_public;

        // Create valid signatures
        let signatures = valid_secrets
            .iter()
            .map(|s| {
                let mut secret = s.clone();
                W3fBls377::sign_with_secret(&mut secret, &message).unwrap()
            })
            .collect::<Vec<_>>();

        // Aggregate with mismatched public keys
        let (aggregated_sig, aggregated_public) =
            W3fBls377::aggregate(&signatures, &mixed_publics).unwrap();

        // Verification should fail because public keys don't match signatures
        assert!(
            !W3fBls377::verify_aggregate(&message, &aggregated_sig, &aggregated_public).unwrap()
        );
    }

    #[test]
    fn test_empty_aggregation() {
        let empty_sigs: Vec<W3fBls377Signature> = vec![];
        let empty_publics: Vec<W3fBls377Public> = vec![];
        let agg_result = W3fBls377::aggregate(&empty_sigs, &empty_publics);
        assert!(matches!(agg_result, Err(BlsError::InvalidInput(_))));
    }
}

mod bls381_tests {
    use crate::{bls381::W3fBls381Signature, error::BlsError};

    use super::*;
    use ::tnt_bls::SerializableToBytes;
    use blueprint_crypto_core::aggregation::AggregatableSignature;
    use blueprint_crypto_hashing::sha2_256;

    #[test]
    fn test_key_generation() {
        // Test seed-based generation is deterministic
        let seed = [0u8; 32];
        let secret1 = W3fBls381::generate_with_seed(Some(&seed)).unwrap();
        let public1 = W3fBls381::public_from_secret(&secret1);

        let secret2 = W3fBls381::generate_with_seed(Some(&seed)).unwrap();
        let public2 = W3fBls381::public_from_secret(&secret2);

        // Same seed should produce same keys
        assert_eq!(public1, public2);

        let secret1_hex = hex::encode(secret1.0.to_bytes());
        let secret2_hex = hex::encode(secret2.0.to_bytes());

        // Test string-based generation is deterministic
        let secret_from_str1 = W3fBls381::generate_with_string(secret1_hex).unwrap();
        let public_from_str1 = W3fBls381::public_from_secret(&secret_from_str1);

        let secret_from_str2 = W3fBls381::generate_with_string(secret2_hex).unwrap();
        let public_from_str2 = W3fBls381::public_from_secret(&secret_from_str2);

        // Same string should produce same keys
        assert_eq!(public_from_str1, public_from_str2);
    }

    #[test]
    fn test_signing_and_verification() {
        let seed = [0u8; 32];
        let mut secret = W3fBls381::generate_with_seed(Some(&seed)).unwrap();
        let public = W3fBls381::public_from_secret(&secret);
        let message = test_message();

        // Test normal signing
        let signature = W3fBls381::sign_with_secret(&mut secret, &message).unwrap();
        assert!(W3fBls381::verify(&public, &message, &signature));

        // Test pre-hashed signing
        let hashed_msg = sha2_256(&message);
        let signature_pre_hashed =
            W3fBls381::sign_with_secret_pre_hashed(&mut secret, &hashed_msg).unwrap();
        // For pre-hashed messages, we need to hash the verification message as well
        assert!(W3fBls381::verify(
            &public,
            &hashed_msg,
            &signature_pre_hashed
        ));

        // Test invalid signature
        let wrong_message = b"wrong message".to_vec();
        assert!(!W3fBls381::verify(&public, &wrong_message, &signature));
    }

    #[test]
    fn test_serialization() {
        let seed = b"test seed for serialization";
        let secret = W3fBls381::generate_with_seed(Some(seed)).unwrap();
        let public = W3fBls381::public_from_secret(&secret);

        // Test public key serialization
        let public_bytes = to_bytes(public.0);
        let public_deserialized: W3fBls381Public = W3fBls381Public(from_bytes(&public_bytes));
        assert_eq!(public, public_deserialized);

        // Test secret key serialization
        let secret_bytes = to_bytes(secret.0.clone());
        let secret_deserialized: W3fBls381Secret = W3fBls381Secret(from_bytes(&secret_bytes));
        assert_eq!(secret, secret_deserialized);
    }

    #[test]
    fn test_error_handling() {
        // Test invalid seed
        let result = W3fBls381::generate_with_string("invalid hex".to_string());
        assert!(result.is_err());

        // Test empty seed
        let result = W3fBls381::generate_with_string("".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_signature_aggregation_success() {
        let message = test_message();

        // Generate 3 test keys
        let secrets: Vec<W3fBls381Secret> = (0..3)
            .map(|i| W3fBls381::generate_with_seed(Some(&[i as u8; 32])).unwrap())
            .collect();
        let publics: Vec<W3fBls381Public> =
            secrets.iter().map(W3fBls381::public_from_secret).collect();

        // Create individual signatures
        let signatures: Vec<W3fBls381Signature> = secrets
            .iter()
            .map(|s| {
                let mut secret = s.clone();
                W3fBls381::sign_with_secret(&mut secret, &message).unwrap()
            })
            .collect();

        // Aggregate signatures
        let (aggregated_sig, aggregated_public) =
            W3fBls381::aggregate(&signatures, &publics).unwrap();

        // Verify aggregate signature against all public keys
        assert!(
            W3fBls381::verify_aggregate(&message, &aggregated_sig, &aggregated_public).unwrap()
        );
    }

    #[test]
    fn test_signature_aggregation_failure() {
        let message = test_message();
        let different_message = b"completely different message".to_vec();

        // Generate 3 valid keys
        let secrets = vec![
            W3fBls381::generate_with_seed(Some(&[1u8; 32])).unwrap(),
            W3fBls381::generate_with_seed(Some(&[2u8; 32])).unwrap(),
            W3fBls381::generate_with_seed(Some(&[3u8; 32])).unwrap(),
        ];

        let publics: Vec<W3fBls381Public> =
            secrets.iter().map(W3fBls381::public_from_secret).collect();

        // Create two valid signatures and one invalid
        let mut signatures: Vec<W3fBls381Signature> = secrets[0..2]
            .iter()
            .map(|s| {
                let mut secret = s.clone();
                W3fBls381::sign_with_secret(&mut secret, &message).unwrap()
            })
            .collect();

        // Add signature for different message
        let mut different_secret = secrets[2].clone();
        let different_signature =
            W3fBls381::sign_with_secret(&mut different_secret, &different_message).unwrap();
        signatures.push(different_signature);

        // Properly aggregate the mismatched signatures with their publics
        let (aggregated_sig, aggregated_public) =
            W3fBls381::aggregate(&signatures, &publics).unwrap();

        // Verification should fail because aggregation included invalid signature
        assert!(
            !W3fBls381::verify_aggregate(&message, &aggregated_sig, &aggregated_public).unwrap()
        );
    }

    #[test]
    fn test_aggregation_with_mismatched_keys() {
        let message = test_message();

        // Generate valid set
        let valid_secrets = (0..2)
            .map(|i| W3fBls381::generate_with_seed(Some(&[i as u8; 32])).unwrap())
            .collect::<Vec<_>>();
        let valid_publics = valid_secrets
            .iter()
            .map(W3fBls381::public_from_secret)
            .collect::<Vec<_>>();

        // Generate unrelated key
        let unrelated_secret = W3fBls381::generate_with_seed(Some(&[99u8; 32])).unwrap();
        let unrelated_public = W3fBls381::public_from_secret(&unrelated_secret);

        // Create signatures with one invalid public key
        let mut mixed_publics = valid_publics.clone();
        mixed_publics[1] = unrelated_public;

        // Create valid signatures
        let signatures = valid_secrets
            .iter()
            .map(|s| {
                let mut secret = s.clone();
                W3fBls381::sign_with_secret(&mut secret, &message).unwrap()
            })
            .collect::<Vec<_>>();

        // Aggregate with mismatched public keys
        let (aggregated_sig, aggregated_public) =
            W3fBls381::aggregate(&signatures, &mixed_publics).unwrap();

        // Verification should fail because public keys don't match signatures
        assert!(
            !W3fBls381::verify_aggregate(&message, &aggregated_sig, &aggregated_public).unwrap()
        );
    }

    #[test]
    fn test_empty_aggregation() {
        let empty_sigs: Vec<W3fBls381Signature> = vec![];
        let empty_publics: Vec<W3fBls381Public> = vec![];
        let agg_result = W3fBls381::aggregate(&empty_sigs, &empty_publics);
        assert!(matches!(agg_result, Err(BlsError::InvalidInput(_))));
    }
}
