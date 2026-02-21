use alloy_primitives::{Address, B256, ChainId, U256, keccak256};
use alloy_sol_types::{SolType, SolValue};
use blueprint_client_tangle::contracts::ITangleTypes;
use blueprint_crypto::k256::{K256Signature, K256SigningKey, K256VerifyingKey};
use rust_decimal::Decimal;

use crate::config::OperatorConfig;
use crate::error::{PricingError, Result};
use crate::pricing_engine::{self, asset::AssetType};
use crate::types::ResourceUnit;
use crate::utils::{decimal_to_scaled_amount, percent_to_bps};

pub type BlueprintId = u64;
pub type OperatorId = Address;

#[derive(Debug, Clone)]
pub struct SignedQuote {
    pub quote_details: pricing_engine::QuoteDetails,
    pub abi_details: ITangleTypes::QuoteDetails,
    pub signature: K256Signature,
    /// ECDSA recovery byte (0 or 1). The Ethereum `v` value is `27 + recovery_id`.
    pub recovery_id: u8,
    pub operator_id: OperatorId,
    pub proof_of_work: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuoteSigningDomain {
    pub chain_id: ChainId,
    pub verifying_contract: Address,
}

#[derive(Debug)]
pub struct SignableQuote {
    proto_details: pricing_engine::QuoteDetails,
    abi_details: ITangleTypes::QuoteDetails,
}

impl SignableQuote {
    pub fn new(details: pricing_engine::QuoteDetails, total_cost: Decimal) -> Result<Self> {
        let abi_details = build_abi_quote_details(&details, total_cost)?;
        Ok(Self {
            proto_details: details,
            abi_details,
        })
    }

    #[must_use]
    pub fn abi_details(&self) -> &ITangleTypes::QuoteDetails {
        &self.abi_details
    }
}

pub struct OperatorSigner {
    keypair: K256SigningKey,
    operator_id: OperatorId,
    domain: QuoteSigningDomain,
}

impl OperatorSigner {
    /// Creates a new Operator Signer
    pub fn new(
        config: &OperatorConfig,
        keypair: K256SigningKey,
        domain: QuoteSigningDomain,
    ) -> Result<Self> {
        let operator_id = keypair.alloy_address().map_err(|e| {
            PricingError::Signing(format!("Failed to derive operator address: {e}"))
        })?;

        let _ = config; // reserved for future use

        Ok(OperatorSigner {
            keypair,
            operator_id,
            domain,
        })
    }

    /// Returns a signed quote made up of the quote details, signature, operator ID, and proof of work.
    ///
    /// Uses `sign_prehash_recoverable` to sign the raw EIP-712 keccak256 digest directly,
    /// avoiding the double-hash that `SignerMut::sign()` would introduce (SHA-256 on top of keccak256).
    pub fn sign_quote(
        &mut self,
        quote: SignableQuote,
        proof_of_work: Vec<u8>,
    ) -> Result<SignedQuote> {
        let hash = quote_digest_eip712(&quote.abi_details, self.domain)?;
        let (signature, recovery_id) = self
            .keypair
            .0
            .sign_prehash_recoverable(&hash)
            .map_err(|e| PricingError::Signing(format!("Error signing quote hash: {e}")))?;

        Ok(SignedQuote {
            quote_details: quote.proto_details,
            abi_details: quote.abi_details,
            signature: K256Signature(signature),
            recovery_id: recovery_id.to_byte(),
            operator_id: self.operator_id,
            proof_of_work,
        })
    }

    /// Returns the operator ID
    pub fn operator_id(&self) -> OperatorId {
        self.operator_id
    }

    #[must_use]
    pub fn domain(&self) -> QuoteSigningDomain {
        self.domain
    }

    /// Returns the verifying key associated with the signer.
    pub fn verifying_key(&self) -> K256VerifyingKey {
        self.keypair.verifying_key()
    }
}

/// Compute the full EIP-712 digest for a quote, matching `tnt-core/src/v2/libraries/SignatureLib.sol`.
pub fn quote_digest_eip712(
    quote_details: &ITangleTypes::QuoteDetails,
    domain: QuoteSigningDomain,
) -> Result<[u8; 32]> {
    let domain_separator = compute_domain_separator(domain);
    let quote_hash = hash_quote_details(quote_details);

    let mut payload = Vec::with_capacity(2 + 32 + 32);
    payload.extend_from_slice(b"\x19\x01");
    payload.extend_from_slice(domain_separator.as_slice());
    payload.extend_from_slice(quote_hash.as_slice());

    Ok(keccak256(payload).into())
}

/// Verify a quote signature by checking the prehashed EIP-712 digest against the public key.
///
/// Uses `verify_prehash` to verify against the raw keccak256 digest directly,
/// matching the `sign_prehash_recoverable` used in `sign_quote`.
pub fn verify_quote(
    quote: &SignedQuote,
    public_key: &K256VerifyingKey,
    domain: QuoteSigningDomain,
) -> Result<bool> {
    use k256::ecdsa::signature::hazmat::PrehashVerifier;
    let hash = quote_digest_eip712(&quote.abi_details, domain)?;
    Ok(public_key
        .0
        .verify_prehash(&hash, &quote.signature.0)
        .is_ok())
}

fn compute_domain_separator(domain: QuoteSigningDomain) -> B256 {
    const DOMAIN_TYPEHASH_STR: &str =
        "EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)";

    // `tnt-core/src/v2/core/Base.sol` hardcodes this domain for quote signing.
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

fn hash_quote_details(quote_details: &ITangleTypes::QuoteDetails) -> B256 {
    const ASSET_TYPEHASH_STR: &str = "Asset(uint8 kind,address token)";
    const COMMITMENT_TYPEHASH_STR: &str =
        "AssetSecurityCommitment(Asset asset,uint16 exposureBps)Asset(uint8 kind,address token)";
    const QUOTE_TYPEHASH_STR: &str = "QuoteDetails(uint64 blueprintId,uint64 ttlBlocks,uint256 totalCost,uint64 timestamp,uint64 expiry,AssetSecurityCommitment[] securityCommitments)AssetSecurityCommitment(Asset asset,uint16 exposureBps)Asset(uint8 kind,address token)";

    let quote_typehash = keccak256(QUOTE_TYPEHASH_STR.as_bytes());
    let commitment_typehash = keccak256(COMMITMENT_TYPEHASH_STR.as_bytes());
    let asset_typehash = keccak256(ASSET_TYPEHASH_STR.as_bytes());

    let mut commitment_hashes: Vec<u8> =
        Vec::with_capacity(quote_details.securityCommitments.len() * 32);
    for commitment in quote_details.securityCommitments.iter() {
        type AssetEncodeTuple = (
            alloy_sol_types::sol_data::FixedBytes<32>,
            alloy_sol_types::sol_data::Uint<8>,
            alloy_sol_types::sol_data::Address,
        );
        let asset_hash = keccak256(<AssetEncodeTuple as SolType>::abi_encode(&(
            asset_typehash,
            commitment.asset.kind,
            commitment.asset.token,
        )));

        let commitment_hash =
            keccak256((commitment_typehash, asset_hash, commitment.exposureBps).abi_encode());

        commitment_hashes.extend_from_slice(commitment_hash.as_slice());
    }
    let commitments_hash = keccak256(commitment_hashes);

    keccak256(
        (
            quote_typehash,
            quote_details.blueprintId,
            quote_details.ttlBlocks,
            quote_details.totalCost,
            quote_details.timestamp,
            quote_details.expiry,
            commitments_hash,
        )
            .abi_encode(),
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// Per-Job RFQ Signing
//
// The canonical EIP-712 implementation lives in `blueprint_tangle_extra::job_quote`.
// This module provides thin wrappers that convert proto types → native types
// and delegate to the canonical implementation.
// ═══════════════════════════════════════════════════════════════════════════

use blueprint_tangle_extra::job_quote as jq;

/// Signed per-job quote ready for on-chain submission via `submitJobFromQuote`
#[derive(Debug, Clone)]
pub struct SignedJobQuote {
    pub quote_details: crate::pricing_engine::JobQuoteDetails,
    pub signature: K256Signature,
    /// ECDSA recovery byte (0 or 1). The Ethereum `v` value is `27 + recovery_id`.
    pub recovery_id: u8,
    pub operator_id: OperatorId,
    pub proof_of_work: Vec<u8>,
}

impl OperatorSigner {
    /// Sign a per-job quote for the RFQ system.
    ///
    /// The EIP-712 digest matches `SignatureLib.computeJobQuoteDigest()` in tnt-core.
    /// Uses `sign_prehash_recoverable` to sign the raw digest directly.
    pub fn sign_job_quote(
        &mut self,
        details: &crate::pricing_engine::JobQuoteDetails,
        proof_of_work: Vec<u8>,
    ) -> Result<SignedJobQuote> {
        let digest = job_quote_digest_eip712(details, self.domain)?;
        let (signature, recovery_id) = self
            .keypair
            .0
            .sign_prehash_recoverable(&digest)
            .map_err(|e| PricingError::Signing(format!("Error signing job quote: {e}")))?;

        Ok(SignedJobQuote {
            quote_details: details.clone(),
            signature: K256Signature(signature),
            recovery_id: recovery_id.to_byte(),
            operator_id: self.operator_id,
            proof_of_work,
        })
    }
}

/// Convert proto `JobQuoteDetails` (bytes price) → native `job_quote::JobQuoteDetails` (U256 price).
fn proto_to_native_job_quote(
    details: &crate::pricing_engine::JobQuoteDetails,
) -> Result<jq::JobQuoteDetails> {
    let price = if details.price.is_empty() {
        U256::ZERO
    } else {
        U256::from_be_slice(&details.price)
    };

    let job_index = u8::try_from(details.job_index).map_err(|_| {
        PricingError::Signing(format!(
            "job_index {} exceeds u8 range (max 255)",
            details.job_index
        ))
    })?;

    Ok(jq::JobQuoteDetails {
        service_id: details.service_id,
        job_index,
        price,
        timestamp: details.timestamp,
        expiry: details.expiry,
    })
}

/// Convert this crate's `QuoteSigningDomain` to the canonical `job_quote::QuoteSigningDomain`.
fn to_jq_domain(domain: QuoteSigningDomain) -> jq::QuoteSigningDomain {
    jq::QuoteSigningDomain {
        chain_id: domain.chain_id,
        verifying_contract: domain.verifying_contract,
    }
}

/// Compute the full EIP-712 digest for a proto job quote.
///
/// Delegates to `blueprint_tangle_extra::job_quote::job_quote_digest_eip712`.
pub fn job_quote_digest_eip712(
    details: &crate::pricing_engine::JobQuoteDetails,
    domain: QuoteSigningDomain,
) -> Result<[u8; 32]> {
    let native = proto_to_native_job_quote(details)?;
    Ok(jq::job_quote_digest_eip712(&native, to_jq_domain(domain)))
}

fn build_abi_quote_details(
    details: &pricing_engine::QuoteDetails,
    total_cost: Decimal,
) -> Result<ITangleTypes::QuoteDetails> {
    let security_commitments = details
        .security_commitments
        .iter()
        .map(proto_commitment_to_abi)
        .collect::<Result<Vec<_>>>()?;
    let resource_commitments = details
        .resources
        .iter()
        .map(proto_resource_commitment_to_abi)
        .collect::<Result<Vec<_>>>()?;

    Ok(ITangleTypes::QuoteDetails {
        blueprintId: details.blueprint_id,
        ttlBlocks: details.ttl_blocks,
        totalCost: decimal_to_scaled_amount(total_cost)?,
        timestamp: details.timestamp,
        expiry: details.expiry,
        securityCommitments: security_commitments.into(),
        resourceCommitments: resource_commitments.into(),
    })
}

fn proto_commitment_to_abi(
    commitment: &pricing_engine::AssetSecurityCommitment,
) -> Result<ITangleTypes::AssetSecurityCommitment> {
    let asset = commitment
        .asset
        .as_ref()
        .ok_or_else(|| PricingError::Signing("Missing commitment asset".to_string()))?;

    Ok(ITangleTypes::AssetSecurityCommitment {
        asset: proto_asset_to_abi(asset)?,
        exposureBps: percent_to_bps(commitment.exposure_percent)?,
    })
}

fn proto_asset_to_abi(asset: &pricing_engine::Asset) -> Result<ITangleTypes::Asset> {
    let asset_type = asset
        .asset_type
        .as_ref()
        .ok_or_else(|| PricingError::Signing("missing asset type".to_string()))?;

    match asset_type {
        AssetType::Erc20(bytes) => {
            const ERC20_ADDRESS_LEN: usize = 20;
            const ERC20_KIND: u8 = ITangleTypes::AssetKind::from_underlying(1).into_underlying();
            if bytes.len() != ERC20_ADDRESS_LEN {
                return Err(PricingError::Signing(format!(
                    "ERC20 address must be 20 bytes, got {}",
                    bytes.len()
                )));
            }
            let mut addr = [0u8; ERC20_ADDRESS_LEN];
            addr.copy_from_slice(bytes);
            Ok(ITangleTypes::Asset {
                kind: ERC20_KIND,
                token: Address::from(addr),
            })
        }
        AssetType::Custom(_) => Err(PricingError::Signing(
            "Custom assets are not supported on Tangle EVM".to_string(),
        )),
    }
}

fn proto_resource_commitment_to_abi(
    resource: &pricing_engine::ResourcePricing,
) -> Result<ITangleTypes::ResourceCommitment> {
    // Matches tnt-core Types.ResourceCommitment.kind:
    // 0=CPU, 1=MemoryMB, 2=StorageMB, 3=NetworkEgressMB, 4=NetworkIngressMB, 5=GPU
    let kind =
        match resource.kind.parse::<ResourceUnit>().map_err(|_| {
            PricingError::Signing(format!("Invalid resource kind: {}", resource.kind))
        })? {
            ResourceUnit::CPU => 0,
            ResourceUnit::MemoryMB => 1,
            ResourceUnit::StorageMB => 2,
            ResourceUnit::NetworkEgressMB => 3,
            ResourceUnit::NetworkIngressMB => 4,
            ResourceUnit::GPU => 5,
            _ => {
                return Err(PricingError::Signing(format!(
                    "Unsupported resource kind for tnt-core quote commitments: {}",
                    resource.kind
                )));
            }
        };

    Ok(ITangleTypes::ResourceCommitment {
        kind,
        count: resource.count,
    })
}
