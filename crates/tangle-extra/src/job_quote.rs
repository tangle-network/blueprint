//! Job Quote signing and verification for Tangle v2 RFQ system
//!
//! Operators use this module to sign per-job quotes that requesters submit on-chain
//! via `submitJobFromQuote`. The EIP-712 structured data matches
//! `tnt-core/src/libraries/SignatureLib.sol`'s `JOB_QUOTE_TYPEHASH`.
//!
//! # v0.13.0 binding to `requester`
//!
//! Since tnt-core v0.13.0 (audit Round 2 economic F1, tnt-core PRs #124/#125),
//! every quote is bound to a specific `requester` address. The on-chain verifier
//! rejects `requester == address(0)` (wildcard quotes are no longer permitted),
//! and the `address requester` field is the first member of the EIP-712 struct.
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
//!     requester: buyer_addr, // MUST be non-zero — wildcard quotes are rejected on-chain
//!     service_id: 1,
//!     job_index: 0,
//!     price: U256::from(1_000_000_000_000_000_000u128), // 1 ETH
//!     timestamp: 1700000000,
//!     expiry: 1700003600,
//!     confidentiality: 0,
//! };
//!
//! let signed = signer.sign(&details)?;
//! assert!(verify_job_quote(&signed, &signer.verifying_key(), domain)?);
//! ```

use alloy_primitives::{Address, B256, ChainId, U256, keccak256};
use alloy_sol_types::SolValue;
use blueprint_crypto::k256::{K256Signature, K256SigningKey, K256VerifyingKey};

/// Errors from job quote signing/verification
#[derive(Debug, thiserror::Error)]
pub enum JobQuoteError {
    #[error("Signing error: {0}")]
    Signing(String),
}

type Result<T> = core::result::Result<T, JobQuoteError>;

/// Typed confidentiality levels for job quotes.
///
/// Maps to the on-chain `uint8 confidentiality` field:
/// - 0 = Any (no TEE requirement)
/// - 1 = Required (must run in TEE)
/// - 2 = Preferred (prefer TEE, allow non-TEE)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Confidentiality {
    Any = 0,
    Required = 1,
    Preferred = 2,
}

impl From<Confidentiality> for u8 {
    fn from(c: Confidentiality) -> u8 {
        c as u8
    }
}

impl TryFrom<u8> for Confidentiality {
    type Error = JobQuoteError;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            0 => Ok(Confidentiality::Any),
            1 => Ok(Confidentiality::Required),
            2 => Ok(Confidentiality::Preferred),
            _ => Err(JobQuoteError::Signing(format!(
                "invalid confidentiality level: {value} (expected 0, 1, or 2)"
            ))),
        }
    }
}

/// Per-job quote details that get EIP-712 signed
///
/// Matches `Types.JobQuoteDetails` in tnt-core (v0.13.0+):
/// ```solidity
/// struct JobQuoteDetails {
///     address requester;
///     uint64 serviceId;
///     uint8 jobIndex;
///     uint256 price;
///     uint64 timestamp;
///     uint64 expiry;
///     uint8 confidentiality;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JobQuoteDetails {
    /// Address allowed to redeem this quote on-chain. MUST be non-zero —
    /// the contract rejects wildcard (`address(0)`) quotes since v0.13.0.
    pub requester: Address,
    pub service_id: u64,
    pub job_index: u8,
    pub price: U256,
    pub timestamp: u64,
    pub expiry: u64,
    /// Confidentiality level bound into the EIP-712 signature.
    /// 0 = Any (no TEE), 1 = Required, 2 = Preferred.
    /// Prevents replay of a non-TEE quote for a TEE-required service.
    pub confidentiality: u8,
}

impl JobQuoteDetails {
    /// Parse the raw `confidentiality` field into a typed enum.
    /// Returns `None` if the value is not a recognized level.
    pub fn confidentiality_level(&self) -> Option<Confidentiality> {
        Confidentiality::try_from(self.confidentiality).ok()
    }
}

/// A signed job quote ready for on-chain submission
#[derive(Debug, Clone)]
pub struct SignedJobQuote {
    pub details: JobQuoteDetails,
    pub signature: K256Signature,
    /// ECDSA recovery byte (0 or 1). The Ethereum `v` value is `27 + recovery_id`.
    pub recovery_id: u8,
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

    /// Sign a job quote, producing a `SignedJobQuote` for on-chain submission.
    ///
    /// Uses `sign_prehash_recoverable` to sign the raw EIP-712 keccak256 digest directly,
    /// avoiding the double-hash that `SignerMut::sign()` would introduce (SHA-256 on top of keccak256).
    pub fn sign(&mut self, details: &JobQuoteDetails) -> Result<SignedJobQuote> {
        let digest = job_quote_digest_eip712(details, self.domain);
        let (signature, recovery_id) = self
            .keypair
            .0
            .sign_prehash_recoverable(&digest)
            .map_err(|e| JobQuoteError::Signing(format!("ECDSA signing failed: {e}")))?;

        Ok(SignedJobQuote {
            details: details.clone(),
            signature: K256Signature(signature),
            recovery_id: recovery_id.to_byte(),
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

/// Verify a signed job quote against a public key.
///
/// Uses `verify_prehash` to verify against the raw keccak256 digest directly,
/// matching the `sign_prehash_recoverable` used in `JobQuoteSigner::sign`.
pub fn verify_job_quote(
    quote: &SignedJobQuote,
    public_key: &K256VerifyingKey,
    domain: QuoteSigningDomain,
) -> Result<bool> {
    use alloy::signers::k256::ecdsa::signature::hazmat::PrehashVerifier;
    let digest = job_quote_digest_eip712(&quote.details, domain);
    Ok(public_key
        .0
        .verify_prehash(&digest, &quote.signature.0)
        .is_ok())
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
/// Matches `SignatureLib.hashJobQuote()` in tnt-core v0.13.0+:
/// ```text
/// keccak256(abi.encode(JOB_QUOTE_TYPEHASH, requester, serviceId, jobIndex, price, timestamp, expiry, confidentiality))
/// ```
fn hash_job_quote_details(details: &JobQuoteDetails) -> B256 {
    const JOB_QUOTE_TYPEHASH_STR: &str = "JobQuoteDetails(address requester,uint64 serviceId,uint8 jobIndex,uint256 price,uint64 timestamp,uint64 expiry,uint8 confidentiality)";

    let typehash = keccak256(JOB_QUOTE_TYPEHASH_STR.as_bytes());

    // abi.encode pads each field to 32 bytes, matching Solidity's abi.encode behavior.
    // `requester` is the first field after the typehash (v0.13.0 binding).
    let encoded = (
        typehash,
        details.requester,
        U256::from(details.service_id),
        U256::from(details.job_index),
        details.price,
        U256::from(details.timestamp),
        U256::from(details.expiry),
        U256::from(details.confidentiality),
    )
        .abi_encode();

    keccak256(encoded)
}

// ---- Conversion to on-chain types ----

/// Convert an SDK `SignedJobQuote` to the on-chain `ITangleTypes::SignedJobQuote`
/// for submission via `TangleClient::submit_job_from_quote`.
///
/// Produces a 65-byte ECDSA signature (r || s || v) where v = 27 + recovery_id.
impl From<SignedJobQuote> for blueprint_client_tangle::contracts::ITangleTypes::SignedJobQuote {
    fn from(quote: SignedJobQuote) -> Self {
        use blueprint_crypto::BytesEncoding;
        let mut sig_bytes = quote.signature.to_bytes();
        sig_bytes.push(27 + quote.recovery_id);
        Self {
            details: blueprint_client_tangle::contracts::ITangleTypes::JobQuoteDetails {
                requester: quote.details.requester,
                serviceId: quote.details.service_id,
                jobIndex: quote.details.job_index,
                price: quote.details.price,
                timestamp: quote.details.timestamp,
                expiry: quote.details.expiry,
                confidentiality: quote.details.confidentiality,
            },
            signature: sig_bytes.into(),
            operator: quote.operator,
        }
    }
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

    /// Non-zero placeholder for tests that don't assert against cross-repo vectors.
    fn placeholder_requester() -> Address {
        address!("000000000000000000000000000000000000bEEF")
    }

    #[test]
    fn test_hash_job_quote_deterministic() {
        let details = JobQuoteDetails {
            requester: placeholder_requester(),
            service_id: 1,
            job_index: 0,
            price: U256::from(1_000_000_000_000_000_000u128),
            timestamp: 1700000000,
            expiry: 1700003600,
            confidentiality: 0,
        };
        let h1 = hash_job_quote_details(&details);
        let h2 = hash_job_quote_details(&details);
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_different_quotes_produce_different_hashes() {
        let details1 = JobQuoteDetails {
            requester: placeholder_requester(),
            service_id: 1,
            job_index: 0,
            price: U256::from(100u64),
            timestamp: 1700000000,
            expiry: 1700003600,
            confidentiality: 0,
        };
        let details2 = JobQuoteDetails {
            requester: placeholder_requester(),
            service_id: 1,
            job_index: 1, // different job
            price: U256::from(100u64),
            timestamp: 1700000000,
            expiry: 1700003600,
            confidentiality: 0,
        };
        assert_ne!(
            hash_job_quote_details(&details1),
            hash_job_quote_details(&details2)
        );
    }

    #[test]
    fn test_requester_changes_hash() {
        // A quote bound to one requester must NOT verify as a quote for another.
        let base = JobQuoteDetails {
            requester: placeholder_requester(),
            service_id: 1,
            job_index: 0,
            price: U256::from(100u64),
            timestamp: 1700000000,
            expiry: 1700003600,
            confidentiality: 0,
        };
        let other = JobQuoteDetails {
            requester: address!("00000000000000000000000000000000DeadBeef"),
            ..base.clone()
        };
        assert_ne!(
            hash_job_quote_details(&base),
            hash_job_quote_details(&other),
            "different requester must yield different struct hash"
        );
    }

    #[test]
    fn test_confidentiality_changes_hash() {
        let base = JobQuoteDetails {
            requester: placeholder_requester(),
            service_id: 1,
            job_index: 0,
            price: U256::from(100u64),
            timestamp: 1700000000,
            expiry: 1700003600,
            confidentiality: 0,
        };
        let tee_required = JobQuoteDetails {
            confidentiality: 1,
            ..base.clone()
        };
        // A non-TEE quote must NOT be replayable as a TEE-required quote
        assert_ne!(
            hash_job_quote_details(&base),
            hash_job_quote_details(&tee_required),
        );
    }

    #[test]
    fn test_digest_differs_across_domains() {
        let details = JobQuoteDetails {
            requester: placeholder_requester(),
            service_id: 1,
            job_index: 0,
            price: U256::from(100u64),
            timestamp: 1700000000,
            expiry: 1700003600,
            confidentiality: 0,
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
            requester: placeholder_requester(),
            service_id: 42,
            job_index: 3,
            price: U256::from(500_000_000_000_000_000u128), // 0.5 ETH
            timestamp: 1700000000,
            expiry: 1700003600,
            confidentiality: 0,
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
            requester: placeholder_requester(),
            service_id: 1,
            job_index: 0,
            price: U256::from(100u64),
            timestamp: 1700000000,
            expiry: 1700003600,
            confidentiality: 0,
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
            requester: placeholder_requester(),
            service_id: 1,
            job_index: 0,
            price: U256::from(100u64),
            timestamp: 1700000000,
            expiry: 1700003600,
            confidentiality: 0,
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
            requester: address!("000000000000000000000000000000000000bEEF"),
            service_id: 42,
            job_index: 3,
            price: U256::from(1_000_000_000_000_000_000u128), // 1 ether
            timestamp: 1700000000,
            expiry: 1700003600,
            confidentiality: 0,
        };

        let struct_hash = hash_job_quote_details(&details);
        assert_eq!(
            struct_hash,
            B256::from(hex_literal::hex!(
                "81efa1579f66bc16802d9c482eb23561fa1a86e1288cb65902b4619005a04a87"
            )),
            "struct hash must match Solidity Vector 1 (v0.13.0 typehash with requester)"
        );

        let digest = job_quote_digest_eip712(&details, domain);
        assert_eq!(
            digest,
            hex_literal::hex!("fd2339fda45c2e7e30f8d5dbcc062f82af12757ad80175cbdd6972627fb3c54c"),
            "EIP-712 digest must match Solidity Vector 1 (v0.13.0 typehash with requester)"
        );
    }

    #[test]
    fn test_eip712_compat_vector2_zero_price() {
        let domain = compat_domain();
        let details = JobQuoteDetails {
            requester: address!("0000000000000000000000000000000000C0FFEE"),
            service_id: 1,
            job_index: 0,
            price: U256::ZERO,
            timestamp: 1000000,
            expiry: 1003600,
            confidentiality: 0,
        };

        let digest = job_quote_digest_eip712(&details, domain);
        assert_eq!(
            digest,
            hex_literal::hex!("c21c630f71383acd4d8f5465a13264f9e376dfb323acfe97d5202bc9a5baa221"),
            "zero-price digest must match Solidity Vector 2 (v0.13.0 typehash with requester)"
        );
    }

    #[test]
    fn test_eip712_compat_vector3_large_price() {
        let domain = compat_domain();
        let details = JobQuoteDetails {
            requester: address!("000000000000000000000000000000000000bEEF"),
            service_id: 999,
            job_index: 7,
            price: U256::from(u128::MAX), // type(uint128).max
            timestamp: 1700000000,
            expiry: 1700007200,
            confidentiality: 0,
        };

        let digest = job_quote_digest_eip712(&details, domain);
        assert_eq!(
            digest,
            hex_literal::hex!("ebd98b504cfdbe392ddf9813148e2f7808bb6f7ef85c376315fe0446c2ffc9ee"),
            "large-price digest must match Solidity Vector 3 (v0.13.0 typehash with requester)"
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
            requester: address!("000000000000000000000000000000000000bEEF"),
            service_id: 42,
            job_index: 3,
            price: U256::from(1_000_000_000_000_000_000u128),
            timestamp: 1700000000,
            expiry: 1700003600,
            confidentiality: 0,
        };

        // Sign and verify the digest recovers to the expected address
        let mut signer = JobQuoteSigner::new(keypair, domain).unwrap();
        let signed = signer.sign(&details).unwrap();

        // Private key 0x01 => address 0x7E5F4552091A69125d5DfCb7b8C2659029395Bdf
        assert_eq!(
            signed.operator,
            address!("7E5F4552091A69125d5DfCb7b8C2659029395Bdf"),
            "signer address must match Solidity Vector 4"
        );

        // The first 32 bytes of the signature must match Solidity Vector 4's `r`
        // (load-bearing cross-repo digest pin: same private key + same digest =>
        // identical canonical (r, s) under deterministic ECDSA).
        use blueprint_crypto::BytesEncoding;
        let sig_bytes = signed.signature.to_bytes();
        let mut r = [0u8; 32];
        r.copy_from_slice(&sig_bytes[..32]);
        assert_eq!(
            r,
            hex_literal::hex!("9d22c9909f6ebbcadc4ec85467c487e3d29afa8409f058371894af17f176db4c"),
            "signature `r` must match Solidity Vector 4 (v0.13.0 with requester)"
        );

        let valid = verify_job_quote(&signed, &signer.verifying_key(), domain).unwrap();
        assert!(valid, "signature must verify");
    }
}
