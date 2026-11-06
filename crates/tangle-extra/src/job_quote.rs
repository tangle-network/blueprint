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
//!     inputs_hash: keccak256(&job_inputs), // binds the price to the exact submitted inputs
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
/// - 1 = TeeRequired
/// - 2 = StandardRequired
/// - 3 = TeePreferred
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Confidentiality {
    Any = 0,
    TeeRequired = 1,
    StandardRequired = 2,
    TeePreferred = 3,
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
            1 => Ok(Confidentiality::TeeRequired),
            2 => Ok(Confidentiality::StandardRequired),
            3 => Ok(Confidentiality::TeePreferred),
            _ => Err(JobQuoteError::Signing(format!(
                "invalid confidentiality level: {value} (expected 0 through 3)"
            ))),
        }
    }
}

/// Per-job quote details that get EIP-712 signed
///
/// Matches `Types.JobQuoteDetails` in tnt-core (v0.18.0+):
/// ```solidity
/// struct JobQuoteDetails {
///     address requester;
///     uint64 serviceId;
///     uint8 jobIndex;
///     uint256 price;
///     uint64 timestamp;
///     uint64 expiry;
///     uint8 confidentiality;
///     bytes32 inputsHash;
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
    /// Canonical `Types.ConfidentialityPolicy` value, validated before signing.
    /// Prevents replay of a non-TEE quote for a TEE-required service.
    pub confidentiality: u8,
    /// `keccak256(inputs)` of the exact job inputs the requester will submit
    /// on-chain, binding the operator's price to those inputs. There is NO
    /// wildcard: submission reverts `JobQuoteInputsMismatch` unless
    /// `keccak256(submitted inputs)` equals this value (tnt-core v0.18.0+).
    pub inputs_hash: B256,
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
        self.sign_digest(details, job_quote_digest_eip712(details, self.domain))
    }

    fn sign_digest(&self, details: &JobQuoteDetails, digest: [u8; 32]) -> Result<SignedJobQuote> {
        Confidentiality::try_from(details.confidentiality)?;
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
    const JOB_QUOTE_TYPEHASH_STR: &str = "JobQuoteDetails(address requester,uint64 serviceId,uint8 jobIndex,uint256 price,uint64 timestamp,uint64 expiry,uint8 confidentiality,bytes32 inputsHash)";

    let typehash = keccak256(JOB_QUOTE_TYPEHASH_STR.as_bytes());

    // abi.encode pads each field to 32 bytes, matching Solidity's abi.encode behavior.
    // `requester` is the first field after the typehash (v0.13.0 binding);
    // `inputsHash` is the trailing field (v0.18.0 input binding).
    let encoded = (
        typehash,
        details.requester,
        U256::from(details.service_id),
        U256::from(details.job_index),
        details.price,
        U256::from(details.timestamp),
        U256::from(details.expiry),
        U256::from(details.confidentiality),
        details.inputs_hash,
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
                inputsHash: quote.details.inputs_hash,
            },
            signature: sig_bytes.into(),
            operator: quote.operator,
        }
    }
}

/// A validated catalog allocation bound into a GPU job quote.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GpuResourceCommitment {
    class: String,
    count: u64,
    duration_secs: u64,
}

impl GpuResourceCommitment {
    pub fn new(class: &str, count: u64, duration_secs: u64) -> Result<Self> {
        let gpu = blueprint_core::gpu::find(class)
            .ok_or_else(|| JobQuoteError::Signing("unknown GPU catalog class".into()))?;
        if count == 0 || count > u64::from(gpu.max_per_node) || duration_secs == 0 {
            return Err(JobQuoteError::Signing(
                "invalid GPU count or duration".into(),
            ));
        }
        Ok(Self {
            class: class.into(),
            count,
            duration_secs,
        })
    }

    pub fn class(&self) -> &str {
        &self.class
    }
    pub fn count(&self) -> u64 {
        self.count
    }
    pub fn duration_secs(&self) -> u64 {
        self.duration_secs
    }
    pub fn class_hash(&self) -> B256 {
        keccak256(self.class.as_bytes())
    }

    pub fn abi_tuple(&self) -> (u8, u64, B256, u64) {
        (5, self.count, self.class_hash(), self.duration_secs)
    }
}

#[derive(Debug, Clone)]
pub struct SignedGpuJobQuote {
    pub quote: SignedJobQuote,
    pub resource: GpuResourceCommitment,
}

impl JobQuoteSigner {
    pub fn sign_with_resources(
        &mut self,
        details: &JobQuoteDetails,
        resource: GpuResourceCommitment,
    ) -> Result<SignedGpuJobQuote> {
        let digest = gpu_job_quote_digest_eip712(details, &resource, self.domain);
        Ok(SignedGpuJobQuote {
            quote: self.sign_digest(details, digest)?,
            resource,
        })
    }
}

pub fn verify_gpu_job_quote(
    quote: &SignedGpuJobQuote,
    public_key: &K256VerifyingKey,
    domain: QuoteSigningDomain,
) -> Result<bool> {
    use alloy::signers::k256::ecdsa::signature::hazmat::PrehashVerifier;
    let digest = gpu_job_quote_digest_eip712(&quote.quote.details, &quote.resource, domain);
    Ok(public_key
        .0
        .verify_prehash(&digest, &quote.quote.signature.0)
        .is_ok())
}

mod gpu_eip712 {
    alloy_sol_types::sol! {
        struct GpuResourceCommitment {
            uint8 kind;
            uint64 count;
            bytes32 gpuClass;
            uint64 durationSecs;
        }
        struct GpuJobQuoteDetails {
            address requester;
            uint64 serviceId;
            uint8 jobIndex;
            uint256 price;
            uint64 timestamp;
            uint64 expiry;
            uint8 confidentiality;
            bytes32 inputsHash;
            GpuResourceCommitment resourceCommitment;
        }
    }
}

/// Canonical GPU schema; the legacy job digest above retains its original schema.
pub fn gpu_job_quote_digest_eip712(
    d: &JobQuoteDetails,
    resource: &GpuResourceCommitment,
    domain: QuoteSigningDomain,
) -> [u8; 32] {
    use alloy_sol_types::SolStruct;
    let wire = gpu_eip712::GpuJobQuoteDetails {
        requester: d.requester,
        serviceId: d.service_id,
        jobIndex: d.job_index,
        price: d.price,
        timestamp: d.timestamp,
        expiry: d.expiry,
        confidentiality: d.confidentiality,
        inputsHash: d.inputs_hash,
        resourceCommitment: gpu_eip712::GpuResourceCommitment {
            kind: 5,
            count: resource.count,
            gpuClass: resource.class_hash(),
            durationSecs: resource.duration_secs,
        },
    };
    let domain = alloy_sol_types::eip712_domain! {
        name: "TangleQuote", version: "1", chain_id: domain.chain_id,
        verifying_contract: domain.verifying_contract,
    };
    wire.eip712_signing_hash(&domain).into()
}
