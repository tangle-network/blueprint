//! Job Quote signing and verification for Tangle v2 RFQ system
//!
//! Operators use this module to sign per-job quotes that requesters submit on-chain
//! via `submitJobFromQuote`. The EIP-712 structured data matches
//! `tnt-core/src/libraries/SignatureLib.sol`'s `JOB_QUOTE_TYPEHASH`.
//!
//! # Usage
//!
//! ```rust,ignore
//! use blueprint_tangle_extra::job_quote::{JobQuoteSigner, JobQuoteDetails, QuoteSigningDomain};
//! use blueprint_crypto::k256::K256SigningKey;
//!
//! let domain = QuoteSigningDomain { chain_id: 1, verifying_contract: contract_addr };
//! let mut signer = JobQuoteSigner::new(keypair, domain)?;
//!
//! let details = JobQuoteDetails {
//!     service_id: 1,
//!     job_index: 0,
//!     price: U256::from(1_000_000_000_000_000_000u128), // 1 ETH
//!     timestamp: 1700000000,
//!     expiry: 1700003600,
//! };
//!
//! let signed = signer.sign(&details)?;
//! assert!(verify_job_quote(&signed, &signer.verifying_key(), domain)?);
//! ```

use alloy_primitives::{Address, B256, ChainId, U256, keccak256};
use alloy_sol_types::SolValue;
use blueprint_crypto::KeyType;
use blueprint_crypto::k256::{K256Ecdsa, K256Signature, K256SigningKey, K256VerifyingKey};

/// Errors from job quote signing/verification
#[derive(Debug, thiserror::Error)]
pub enum JobQuoteError {
    #[error("Signing error: {0}")]
    Signing(String),
}

type Result<T> = core::result::Result<T, JobQuoteError>;

/// Per-job quote details that get EIP-712 signed
///
/// Matches `Types.JobQuoteDetails` in tnt-core:
/// ```solidity
/// struct JobQuoteDetails {
///     uint64 serviceId;
///     uint8 jobIndex;
///     uint256 price;
///     uint64 timestamp;
///     uint64 expiry;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JobQuoteDetails {
    pub service_id: u64,
    pub job_index: u8,
    pub price: U256,
    pub timestamp: u64,
    pub expiry: u64,
}

/// A signed job quote ready for on-chain submission
#[derive(Debug, Clone)]
pub struct SignedJobQuote {
    pub details: JobQuoteDetails,
    pub signature: K256Signature,
    pub operator: Address,
}

/// EIP-712 domain for signing job quotes
///
/// Uses the same domain as service creation quotes: `TangleQuote` v1.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuoteSigningDomain {
    pub chain_id: ChainId,
    pub verifying_contract: Address,
}

/// Operator signer for per-job RFQ quotes
///
/// Holds the operator's ECDSA keypair and computes EIP-712 structured
/// signatures that match `SignatureLib.verifyAndMarkJobQuoteUsed()` on-chain.
pub struct JobQuoteSigner {
    keypair: K256SigningKey,
    operator: Address,
    domain: QuoteSigningDomain,
}

impl JobQuoteSigner {
    /// Create a new signer from a keypair and domain configuration
    pub fn new(keypair: K256SigningKey, domain: QuoteSigningDomain) -> Result<Self> {
        let operator = keypair.alloy_address().map_err(|e| {
            JobQuoteError::Signing(format!("Failed to derive operator address: {e}"))
        })?;

        Ok(Self {
            keypair,
            operator,
            domain,
        })
    }

    /// Sign a job quote, producing a `SignedJobQuote` for on-chain submission
    pub fn sign(&mut self, details: &JobQuoteDetails) -> Result<SignedJobQuote> {
        let digest = job_quote_digest_eip712(details, self.domain);
        let signature = K256Ecdsa::sign_with_secret(&mut self.keypair, &digest)
            .map_err(|e| JobQuoteError::Signing(format!("ECDSA signing failed: {e}")))?;

        Ok(SignedJobQuote {
            details: details.clone(),
            signature,
            operator: self.operator,
        })
    }

    /// The operator's on-chain address
    pub fn operator(&self) -> Address {
        self.operator
    }

    /// The domain this signer targets
    pub fn domain(&self) -> QuoteSigningDomain {
        self.domain
    }

    /// The public verifying key
    pub fn verifying_key(&self) -> K256VerifyingKey {
        self.keypair.verifying_key()
    }
}

/// Verify a signed job quote against a public key
pub fn verify_job_quote(
    quote: &SignedJobQuote,
    public_key: &K256VerifyingKey,
    domain: QuoteSigningDomain,
) -> Result<bool> {
    let digest = job_quote_digest_eip712(&quote.details, domain);
    Ok(K256Ecdsa::verify(public_key, &digest, &quote.signature))
}

/// Compute the full EIP-712 digest for a job quote
///
/// Matches `SignatureLib.computeJobQuoteDigest()` in tnt-core:
/// ```text
/// keccak256(abi.encodePacked("\x19\x01", domainSeparator, hashJobQuote(details)))
/// ```
pub fn job_quote_digest_eip712(details: &JobQuoteDetails, domain: QuoteSigningDomain) -> [u8; 32] {
    let domain_separator = compute_domain_separator(domain);
    let struct_hash = hash_job_quote_details(details);

    let mut payload = Vec::with_capacity(2 + 32 + 32);
    payload.extend_from_slice(b"\x19\x01");
    payload.extend_from_slice(domain_separator.as_slice());
    payload.extend_from_slice(struct_hash.as_slice());

    keccak256(payload).into()
}

/// Compute the EIP-712 domain separator
///
/// Matches `Base.sol`'s domain: name="TangleQuote", version="1"
fn compute_domain_separator(domain: QuoteSigningDomain) -> B256 {
    const DOMAIN_TYPEHASH_STR: &str =
        "EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)";
    const NAME: &str = "TangleQuote";
    const VERSION: &str = "1";

    let domain_typehash = keccak256(DOMAIN_TYPEHASH_STR.as_bytes());
    let name_hash = keccak256(NAME.as_bytes());
    let version_hash = keccak256(VERSION.as_bytes());

    let encoded = (
        domain_typehash,
        name_hash,
        version_hash,
        U256::from(domain.chain_id),
        domain.verifying_contract,
    )
        .abi_encode();

    keccak256(encoded)
}

/// Hash job quote details per the JOB_QUOTE_TYPEHASH
///
/// Matches `SignatureLib.hashJobQuote()`:
/// ```text
/// keccak256(abi.encode(JOB_QUOTE_TYPEHASH, serviceId, jobIndex, price, timestamp, expiry))
/// ```
fn hash_job_quote_details(details: &JobQuoteDetails) -> B256 {
    const JOB_QUOTE_TYPEHASH_STR: &str = "JobQuoteDetails(uint64 serviceId,uint8 jobIndex,uint256 price,uint64 timestamp,uint64 expiry)";

    let typehash = keccak256(JOB_QUOTE_TYPEHASH_STR.as_bytes());

    // abi.encode pads each field to 32 bytes, matching Solidity's abi.encode behavior
    let encoded = (
        typehash,
        U256::from(details.service_id),
        U256::from(details.job_index),
        details.price,
        U256::from(details.timestamp),
        U256::from(details.expiry),
    )
        .abi_encode();

    keccak256(encoded)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::address;
    use blueprint_crypto::BytesEncoding;

    fn test_domain() -> QuoteSigningDomain {
        QuoteSigningDomain {
            chain_id: 31337, // anvil
            verifying_contract: address!("0000000000000000000000000000000000000001"),
        }
    }

    #[test]
    fn test_domain_separator_deterministic() {
        let domain = test_domain();
        let sep1 = compute_domain_separator(domain);
        let sep2 = compute_domain_separator(domain);
        assert_eq!(sep1, sep2);
    }

    #[test]
    fn test_hash_job_quote_deterministic() {
        let details = JobQuoteDetails {
            service_id: 1,
            job_index: 0,
            price: U256::from(1_000_000_000_000_000_000u128),
            timestamp: 1700000000,
            expiry: 1700003600,
        };
        let h1 = hash_job_quote_details(&details);
        let h2 = hash_job_quote_details(&details);
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_different_quotes_produce_different_hashes() {
        let details1 = JobQuoteDetails {
            service_id: 1,
            job_index: 0,
            price: U256::from(100u64),
            timestamp: 1700000000,
            expiry: 1700003600,
        };
        let details2 = JobQuoteDetails {
            service_id: 1,
            job_index: 1, // different job
            price: U256::from(100u64),
            timestamp: 1700000000,
            expiry: 1700003600,
        };
        assert_ne!(
            hash_job_quote_details(&details1),
            hash_job_quote_details(&details2)
        );
    }

    #[test]
    fn test_digest_differs_across_domains() {
        let details = JobQuoteDetails {
            service_id: 1,
            job_index: 0,
            price: U256::from(100u64),
            timestamp: 1700000000,
            expiry: 1700003600,
        };
        let domain1 = test_domain();
        let domain2 = QuoteSigningDomain {
            chain_id: 1, // mainnet
            verifying_contract: domain1.verifying_contract,
        };
        assert_ne!(
            job_quote_digest_eip712(&details, domain1),
            job_quote_digest_eip712(&details, domain2),
        );
    }

    #[test]
    fn test_sign_and_verify_roundtrip() {
        // Use a deterministic test key
        let secret_bytes = [1u8; 32];
        let keypair = K256SigningKey::from_bytes(&secret_bytes).expect("valid key");
        let verifying_key = keypair.verifying_key();
        let domain = test_domain();

        let mut signer = JobQuoteSigner::new(keypair, domain).unwrap();

        let details = JobQuoteDetails {
            service_id: 42,
            job_index: 3,
            price: U256::from(500_000_000_000_000_000u128), // 0.5 ETH
            timestamp: 1700000000,
            expiry: 1700003600,
        };

        let signed = signer.sign(&details).unwrap();
        assert_eq!(signed.details, details);
        assert_eq!(signed.operator, signer.operator());

        let valid = verify_job_quote(&signed, &verifying_key, domain).unwrap();
        assert!(valid, "signature should verify against the correct key");
    }

    #[test]
    fn test_wrong_key_fails_verification() {
        let keypair = K256SigningKey::from_bytes(&[1u8; 32]).unwrap();
        let wrong_key = K256SigningKey::from_bytes(&[2u8; 32]).unwrap();
        let domain = test_domain();

        let mut signer = JobQuoteSigner::new(keypair, domain).unwrap();
        let details = JobQuoteDetails {
            service_id: 1,
            job_index: 0,
            price: U256::from(100u64),
            timestamp: 1700000000,
            expiry: 1700003600,
        };

        let signed = signer.sign(&details).unwrap();
        let valid = verify_job_quote(&signed, &wrong_key.verifying_key(), domain).unwrap();
        assert!(!valid, "signature should not verify against wrong key");
    }

    #[test]
    fn test_tampered_details_fails_verification() {
        let keypair = K256SigningKey::from_bytes(&[1u8; 32]).unwrap();
        let verifying_key = keypair.verifying_key();
        let domain = test_domain();

        let mut signer = JobQuoteSigner::new(keypair, domain).unwrap();
        let details = JobQuoteDetails {
            service_id: 1,
            job_index: 0,
            price: U256::from(100u64),
            timestamp: 1700000000,
            expiry: 1700003600,
        };

        let mut signed = signer.sign(&details).unwrap();
        // Tamper with the price
        signed.details.price = U256::from(999u64);

        let valid = verify_job_quote(&signed, &verifying_key, domain).unwrap();
        assert!(!valid, "tampered quote should not verify");
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Cross-repo EIP-712 compatibility tests
    //
    // These digests MUST match the Solidity test vectors in
    // tnt-core/test/tangle/EIP712Compatibility.t.sol
    //
    // Domain: name="TangleQuote", version="1", chainId=31337,
    //         verifyingContract=0xDeaDbeefdEAdbeefdEadbEEFdeadbeEFdEaDbeeF
    // ═══════════════════════════════════════════════════════════════════════════

    fn compat_domain() -> QuoteSigningDomain {
        QuoteSigningDomain {
            chain_id: 31337,
            verifying_contract: address!("DeaDbeefdEAdbeefdEadbEEFdeadbeEFdEaDbeeF"),
        }
    }

    #[test]
    fn test_eip712_compat_domain_separator() {
        let domain = compat_domain();
        let sep = compute_domain_separator(domain);
        assert_eq!(
            sep,
            B256::from(hex_literal::hex!(
                "14a60a86c57fe72bdcbdc59af9a05606ca542a7ed2eeb732756b210d3306f149"
            )),
            "domain separator must match Solidity EIP712CompatibilityTest"
        );
    }

    #[test]
    fn test_eip712_compat_vector1_basic() {
        let domain = compat_domain();
        let details = JobQuoteDetails {
            service_id: 42,
            job_index: 3,
            price: U256::from(1_000_000_000_000_000_000u128), // 1 ether
            timestamp: 1700000000,
            expiry: 1700003600,
        };

        let struct_hash = hash_job_quote_details(&details);
        assert_eq!(
            struct_hash,
            B256::from(hex_literal::hex!(
                "2208c3cc800f0d0c2f7fccdf0d30b393a2949eb302b951a9e3468e60b7de9bd3"
            )),
            "struct hash must match Solidity Vector 1"
        );

        let digest = job_quote_digest_eip712(&details, domain);
        assert_eq!(
            digest,
            hex_literal::hex!("43852f97be3d1f638c99ae231f2790f2476effab2de03e5a6536762c94da2a7b"),
            "EIP-712 digest must match Solidity Vector 1"
        );
    }

    #[test]
    fn test_eip712_compat_vector2_zero_price() {
        let domain = compat_domain();
        let details = JobQuoteDetails {
            service_id: 1,
            job_index: 0,
            price: U256::ZERO,
            timestamp: 1000000,
            expiry: 1003600,
        };

        let digest = job_quote_digest_eip712(&details, domain);
        assert_eq!(
            digest,
            hex_literal::hex!("2e5dfc598e6f1767b01024dd1dd7010623fbf5ed3c6f43f3da16f2fb07fc1bc3"),
            "zero-price digest must match Solidity Vector 2"
        );
    }

    #[test]
    fn test_eip712_compat_vector3_large_price() {
        let domain = compat_domain();
        let details = JobQuoteDetails {
            service_id: 999,
            job_index: 7,
            price: U256::from(u128::MAX), // type(uint128).max
            timestamp: 1700000000,
            expiry: 1700007200,
        };

        let digest = job_quote_digest_eip712(&details, domain);
        assert_eq!(
            digest,
            hex_literal::hex!("a007fedc1503dbe6f87b5dca5c00bef6a306ab0d8e49681e6d8ea81e3ec6d56b"),
            "large-price digest must match Solidity Vector 3"
        );
    }

    #[test]
    fn test_eip712_compat_vector4_signature_roundtrip() {
        // Private key 0x01 padded to 32 bytes
        let mut secret = [0u8; 32];
        secret[31] = 1;
        let keypair = K256SigningKey::from_bytes(&secret).expect("valid key");
        let domain = compat_domain();

        let details = JobQuoteDetails {
            service_id: 42,
            job_index: 3,
            price: U256::from(1_000_000_000_000_000_000u128),
            timestamp: 1700000000,
            expiry: 1700003600,
        };

        let digest = job_quote_digest_eip712(&details, domain);
        assert_eq!(
            digest,
            hex_literal::hex!("43852f97be3d1f638c99ae231f2790f2476effab2de03e5a6536762c94da2a7b"),
            "digest must match Vector 1 / Vector 4"
        );

        // Sign and verify the digest recovers to the expected address
        let mut signer = JobQuoteSigner::new(keypair, domain).unwrap();
        let signed = signer.sign(&details).unwrap();

        // Private key 0x01 => address 0x7E5F4552091A69125d5DfCb7b8C2659029395Bdf
        assert_eq!(
            signed.operator,
            address!("7E5F4552091A69125d5DfCb7b8C2659029395Bdf"),
            "signer address must match Solidity Vector 4"
        );

        let valid = verify_job_quote(&signed, &signer.verifying_key(), domain).unwrap();
        assert!(valid, "signature must verify");
    }
}
