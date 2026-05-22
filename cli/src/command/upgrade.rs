//! Blueprint binary version upgrade flow.
//!
//! Wraps the `BlueprintsBinaryVersions`, `BlueprintsBinaryAttestations`, and
//! `BlueprintAuditors` contracts shipped in tnt-core. The CLI surface mirrors
//! the three actor flows that compose a release:
//!
//! 1. Blueprint owner publishes a new binary version, optionally pinning the
//!    artifact to IPFS, then bumps the active version pointer.
//! 2. An auditor attests against the version with a hashed report and a
//!    severity ladder entry.
//! 3. A service operator picks an upgrade policy (`AUTO`/`APPROVE`/`MANUAL`)
//!    and, under `APPROVE`, acks a specific version.
//!
//! The bindings here are inlined via `alloy_sol_types::sol!`. Once
//! `tnt-core-bindings` v0.18+ ships these facets the inlined ABI can be
//! deleted in favour of the published bindings — the function selectors are
//! stable across both paths so any callers that go through this module keep
//! working.
// TODO: switch to tnt-core-bindings v0.18+ once published.

use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use alloy_network::EthereumWallet;
use alloy_primitives::{Address, B256, Bytes, keccak256};
use alloy_provider::{Provider, ProviderBuilder};
use alloy_rpc_types_eth::TransactionRequest;
use alloy_sol_types::{SolCall, SolValue, sol};
use blueprint_crypto::k256::K256SigningKey;
use color_eyre::eyre::{Context, Result, bail, eyre};
use dialoguer::console::style;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use url::Url;

use crate::command::signer::{load_ecdsa_signing_key, load_keystore};

// ─────────────────────────────────────────────────────────────────────────────
// ABI bindings (inlined; replace with tnt-core-bindings >= 0.18 when available)
// ─────────────────────────────────────────────────────────────────────────────

sol! {
    #[allow(missing_docs)]
    #[derive(Debug)]
    enum UpgradePolicy {
        APPROVE,
        AUTO,
        MANUAL,
    }

    #[allow(missing_docs)]
    #[derive(Debug)]
    enum AttestationKind {
        AUDIT,
        FUZZ,
        FORMAL,
        BUG_BOUNTY,
        SELF_KIND,
    }

    #[allow(missing_docs)]
    #[derive(Debug)]
    struct BinaryVersion {
        uint64 versionId;
        bytes32 sha256Hash;
        string binaryUri;
        bytes32 attestationHash;
        uint64 publishedAt;
        bool deprecated;
    }

    #[allow(missing_docs)]
    #[derive(Debug)]
    struct AttestationRow {
        address attester;
        bytes32 reportHash;
        string reportUri;
        uint8 kind;
        uint8 severityFound;
        uint64 attestedAt;
        uint64 expiresAt;
        bool revoked;
    }

    #[allow(missing_docs)]
    #[derive(Debug)]
    struct AuditorRow {
        string name;
        string metadataUri;
        uint16 weight;
        uint8 tier;
        bool active;
        uint64 admittedAt;
    }

    #[allow(missing_docs)]
    function publishBinaryVersion(
        uint64 blueprintId,
        bytes32 sha256Hash,
        string binaryUri,
        bytes32 attestationHash
    ) external returns (uint64 versionId);

    #[allow(missing_docs)]
    function setActiveBinaryVersion(uint64 blueprintId, uint64 versionId) external;

    #[allow(missing_docs)]
    function deprecateBinaryVersion(uint64 blueprintId, uint64 versionId) external;

    #[allow(missing_docs)]
    function setServiceUpgradePolicy(uint64 serviceId, uint8 policy) external;

    #[allow(missing_docs)]
    function ackBinaryVersion(uint64 serviceId, uint64 versionId) external;

    #[allow(missing_docs)]
    function getBinaryVersion(uint64 blueprintId, uint64 versionId)
        external view returns (BinaryVersion memory);

    #[allow(missing_docs)]
    function getBinaryVersionCount(uint64 blueprintId) external view returns (uint64);

    #[allow(missing_docs)]
    function getActiveBinaryVersionId(uint64 blueprintId) external view returns (uint64);

    #[allow(missing_docs)]
    function getServiceUpgradePolicy(uint64 serviceId) external view returns (uint8);

    #[allow(missing_docs)]
    function getServiceAckedVersionId(uint64 serviceId) external view returns (uint64);

    #[allow(missing_docs)]
    function effectiveBinaryVersion(uint64 serviceId)
        external view returns (BinaryVersion memory);

    #[allow(missing_docs)]
    function attestBinaryVersion(
        uint64 blueprintId,
        uint64 versionId,
        bytes32 reportHash,
        string reportUri,
        uint8 kind,
        uint8 severityFound,
        uint64 expiresAt
    ) external returns (uint64 attestationId);

    #[allow(missing_docs)]
    function revokeAttestation(
        uint64 blueprintId,
        uint64 versionId,
        uint64 attestationId,
        string reasonUri
    ) external;

    #[allow(missing_docs)]
    function getAttestation(uint64 blueprintId, uint64 versionId, uint64 attestationId)
        external view returns (AttestationRow memory);

    #[allow(missing_docs)]
    function getAttestationCount(uint64 blueprintId, uint64 versionId)
        external view returns (uint64);

    #[allow(missing_docs)]
    function listAttestations(uint64 blueprintId, uint64 versionId)
        external view returns (AttestationRow[] memory);

    #[allow(missing_docs)]
    function getAuditor(address auditor) external view returns (AuditorRow memory);
}

// ─────────────────────────────────────────────────────────────────────────────
// Public CLI-facing enums (mirroring the on-chain enums in human form)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
#[clap(rename_all = "UPPER")]
pub enum UpgradePolicyArg {
    Auto,
    Approve,
    Manual,
}

impl UpgradePolicyArg {
    pub fn as_u8(self) -> u8 {
        // Mirror the on-chain Types.UpgradePolicy ordering: APPROVE=0, AUTO=1, MANUAL=2.
        match self {
            UpgradePolicyArg::Approve => 0,
            UpgradePolicyArg::Auto => 1,
            UpgradePolicyArg::Manual => 2,
        }
    }

    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(UpgradePolicyArg::Approve),
            1 => Some(UpgradePolicyArg::Auto),
            2 => Some(UpgradePolicyArg::Manual),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            UpgradePolicyArg::Auto => "AUTO",
            UpgradePolicyArg::Approve => "APPROVE",
            UpgradePolicyArg::Manual => "MANUAL",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
#[clap(rename_all = "UPPER")]
pub enum AttestationKindArg {
    Audit,
    Fuzz,
    Formal,
    BugBounty,
    Self_,
}

impl AttestationKindArg {
    pub fn as_u8(self) -> u8 {
        // Matches Types.AttestationKind: AUDIT=0, FUZZ=1, FORMAL=2, BUG_BOUNTY=3, SELF=4.
        match self {
            AttestationKindArg::Audit => 0,
            AttestationKindArg::Fuzz => 1,
            AttestationKindArg::Formal => 2,
            AttestationKindArg::BugBounty => 3,
            AttestationKindArg::Self_ => 4,
        }
    }

    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(AttestationKindArg::Audit),
            1 => Some(AttestationKindArg::Fuzz),
            2 => Some(AttestationKindArg::Formal),
            3 => Some(AttestationKindArg::BugBounty),
            4 => Some(AttestationKindArg::Self_),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            AttestationKindArg::Audit => "AUDIT",
            AttestationKindArg::Fuzz => "FUZZ",
            AttestationKindArg::Formal => "FORMAL",
            AttestationKindArg::BugBounty => "BUG_BOUNTY",
            AttestationKindArg::Self_ => "SELF",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
#[clap(rename_all = "lower")]
pub enum SeverityArg {
    None,
    Info,
    Low,
    Med,
    High,
    Critical,
}

impl SeverityArg {
    pub fn as_u8(self) -> u8 {
        match self {
            SeverityArg::None => 0,
            SeverityArg::Info => 1,
            SeverityArg::Low => 2,
            SeverityArg::Med => 3,
            SeverityArg::High => 4,
            SeverityArg::Critical => 5,
        }
    }

    pub fn from_u8(value: u8) -> &'static str {
        match value {
            0 => "none",
            1 => "info",
            2 => "low",
            3 => "med",
            4 => "high",
            5 => "critical",
            _ => "unknown",
        }
    }
}

/// Parameters shared by every mutation: where to send the tx, who signs it.
#[derive(Debug, Clone)]
pub struct TxContext {
    pub http_rpc_url: Url,
    pub tangle_contract: Address,
    pub keystore_path: PathBuf,
}

/// Parameters shared by every view call.
#[derive(Debug, Clone)]
pub struct ViewContext {
    pub http_rpc_url: Url,
    pub tangle_contract: Address,
    /// Optional `BlueprintAuditors` registry used for trust score weighting. If
    /// unset, attestations are still listed but the trust score treats all
    /// attesters as anonymous (weight 0).
    pub auditors_contract: Option<Address>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Publish flow
// ─────────────────────────────────────────────────────────────────────────────

/// Hash a file with sha256 and return the digest plus the file size.
pub fn hash_file(path: &Path) -> Result<(B256, u64)> {
    let bytes = fs::read(path).with_context(|| format!("reading {}", path.display()))?;
    let len = bytes.len() as u64;
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let digest = hasher.finalize();
    Ok((B256::from_slice(&digest), len))
}

/// IPFS pinning result.
#[derive(Debug, Clone)]
pub struct PinnedArtifact {
    pub cid: String,
    pub uri: String,
}

/// Pin a local file via Web3.Storage / Pinata-compatible JSON-RPC endpoint.
///
/// Selection priority:
///   1. `IPFS_API_URL` + `IPFS_API_TOKEN` — custom endpoint (e.g. self-hosted
///      Kubo, w3up, or any service that accepts `multipart/form-data` POSTs
///      with a `file` field and returns `{"cid":"..."}` or `{"Hash":"..."}`).
///   2. Fallback Pinata: `PINATA_JWT` set → POST to
///      `https://api.pinata.cloud/pinning/pinFileToIPFS` and parse `IpfsHash`.
///
/// Returns an `ipfs://<cid>` URI suitable for the on-chain `binaryUri` field.
pub async fn pin_file_to_ipfs(path: &Path) -> Result<PinnedArtifact> {
    let bytes = fs::read(path).with_context(|| format!("reading {}", path.display()))?;
    let file_name = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("artifact.bin")
        .to_string();

    if let (Ok(url), Ok(token)) = (
        std::env::var("IPFS_API_URL"),
        std::env::var("IPFS_API_TOKEN"),
    ) {
        return pin_via_generic(&url, &token, file_name, bytes).await;
    }
    if let Ok(jwt) = std::env::var("PINATA_JWT") {
        return pin_via_pinata(&jwt, file_name, bytes).await;
    }
    bail!(
        "no IPFS credentials found: set IPFS_API_URL + IPFS_API_TOKEN, or PINATA_JWT, before passing --pin-to-ipfs"
    );
}

async fn pin_via_generic(
    url: &str,
    token: &str,
    file_name: String,
    bytes: Vec<u8>,
) -> Result<PinnedArtifact> {
    let part = reqwest::multipart::Part::bytes(bytes).file_name(file_name);
    let form = reqwest::multipart::Form::new().part("file", part);
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(120))
        .build()
        .context("building reqwest client")?;
    let resp = client
        .post(url)
        .header("Authorization", format!("Bearer {token}"))
        .multipart(form)
        .send()
        .await
        .with_context(|| format!("POST {url}"))?;
    let status = resp.status();
    let body = resp.text().await.context("reading IPFS response body")?;
    if !status.is_success() {
        bail!("IPFS pin failed (HTTP {status}): {body}");
    }
    let value: Value =
        serde_json::from_str(&body).with_context(|| format!("parsing IPFS response: {body}"))?;
    let cid = value
        .get("cid")
        .and_then(Value::as_str)
        .or_else(|| value.get("Hash").and_then(Value::as_str))
        .or_else(|| value.get("IpfsHash").and_then(Value::as_str))
        .ok_or_else(|| eyre!("IPFS response missing cid/Hash/IpfsHash field: {body}"))?
        .to_string();
    Ok(PinnedArtifact {
        uri: format!("ipfs://{cid}"),
        cid,
    })
}

async fn pin_via_pinata(jwt: &str, file_name: String, bytes: Vec<u8>) -> Result<PinnedArtifact> {
    pin_via_generic(
        "https://api.pinata.cloud/pinning/pinFileToIPFS",
        jwt,
        file_name,
        bytes,
    )
    .await
}

#[derive(Debug, Clone)]
pub struct PublishVersionResult {
    pub blueprint_id: u64,
    pub version_id: u64,
    pub sha256_hash: B256,
    pub binary_uri: String,
    pub attestation_hash: B256,
    pub tx_hash: B256,
    pub block_number: Option<u64>,
}

/// Submit `publishBinaryVersion` and return the new version index plus tx
/// metadata. The version index is decoded from the receipt's `BinaryVersionPublished`
/// log (indexed `versionId`) so callers don't have to issue a follow-up RPC call.
#[allow(clippy::too_many_arguments)]
pub async fn publish_version(
    ctx: &TxContext,
    blueprint_id: u64,
    sha256_hash: B256,
    binary_uri: String,
    attestation_hash: B256,
) -> Result<PublishVersionResult> {
    let call = publishBinaryVersionCall {
        blueprintId: blueprint_id,
        sha256Hash: sha256_hash,
        binaryUri: binary_uri.clone(),
        attestationHash: attestation_hash,
    };
    let (receipt, _) = send_tx(ctx, call.abi_encode()).await?;

    // BinaryVersionPublished(uint64 indexed blueprintId, uint64 indexed versionId, bytes32 sha256Hash, string binaryUri)
    // Topic[0] is the event sig, topic[2] is the indexed versionId.
    let topic0 = keccak256("BinaryVersionPublished(uint64,uint64,bytes32,string)".as_bytes());
    let version_id = receipt
        .inner
        .logs()
        .iter()
        .find(|log| log.topics().first() == Some(&topic0) && log.address() == ctx.tangle_contract)
        .and_then(|log| log.topics().get(2).copied())
        .map(|topic| {
            let bytes = topic.0;
            let mut idx = [0u8; 8];
            idx.copy_from_slice(&bytes[24..32]);
            u64::from_be_bytes(idx)
        })
        .ok_or_else(|| {
            eyre!("publishBinaryVersion succeeded but emitted no BinaryVersionPublished log")
        })?;

    Ok(PublishVersionResult {
        blueprint_id,
        version_id,
        sha256_hash,
        binary_uri,
        attestation_hash,
        tx_hash: receipt.transaction_hash,
        block_number: receipt.block_number,
    })
}

#[derive(Debug, Clone)]
pub struct SimpleTxResult {
    pub tx_hash: B256,
    pub block_number: Option<u64>,
}

pub async fn set_active_version(
    ctx: &TxContext,
    blueprint_id: u64,
    version_id: u64,
) -> Result<SimpleTxResult> {
    let call = setActiveBinaryVersionCall {
        blueprintId: blueprint_id,
        versionId: version_id,
    };
    let (receipt, _) = send_tx(ctx, call.abi_encode()).await?;
    Ok(SimpleTxResult {
        tx_hash: receipt.transaction_hash,
        block_number: receipt.block_number,
    })
}

pub async fn deprecate_version(
    ctx: &TxContext,
    blueprint_id: u64,
    version_id: u64,
) -> Result<SimpleTxResult> {
    let call = deprecateBinaryVersionCall {
        blueprintId: blueprint_id,
        versionId: version_id,
    };
    let (receipt, _) = send_tx(ctx, call.abi_encode()).await?;
    Ok(SimpleTxResult {
        tx_hash: receipt.transaction_hash,
        block_number: receipt.block_number,
    })
}

pub async fn set_service_policy(
    ctx: &TxContext,
    service_id: u64,
    policy: UpgradePolicyArg,
) -> Result<SimpleTxResult> {
    let call = setServiceUpgradePolicyCall {
        serviceId: service_id,
        policy: policy.as_u8(),
    };
    let (receipt, _) = send_tx(ctx, call.abi_encode()).await?;
    Ok(SimpleTxResult {
        tx_hash: receipt.transaction_hash,
        block_number: receipt.block_number,
    })
}

pub async fn ack_version(
    ctx: &TxContext,
    service_id: u64,
    version_id: u64,
) -> Result<SimpleTxResult> {
    let call = ackBinaryVersionCall {
        serviceId: service_id,
        versionId: version_id,
    };
    let (receipt, _) = send_tx(ctx, call.abi_encode()).await?;
    Ok(SimpleTxResult {
        tx_hash: receipt.transaction_hash,
        block_number: receipt.block_number,
    })
}

#[derive(Debug, Clone)]
pub struct AttestResult {
    pub blueprint_id: u64,
    pub version_id: u64,
    pub attestation_id: u64,
    pub tx_hash: B256,
    pub block_number: Option<u64>,
}

#[allow(clippy::too_many_arguments)]
pub async fn attest_version(
    ctx: &TxContext,
    blueprint_id: u64,
    version_id: u64,
    report_hash: B256,
    report_uri: String,
    kind: AttestationKindArg,
    severity: SeverityArg,
    expires_at: u64,
) -> Result<AttestResult> {
    let call = attestBinaryVersionCall {
        blueprintId: blueprint_id,
        versionId: version_id,
        reportHash: report_hash,
        reportUri: report_uri,
        kind: kind.as_u8(),
        severityFound: severity.as_u8(),
        expiresAt: expires_at,
    };
    let (receipt, _) = send_tx(ctx, call.abi_encode()).await?;

    // BinaryVersionAttested(uint64 indexed blueprintId, uint64 indexed versionId, uint64 attestationId, address indexed attester, uint8 kind, uint8 severityFound, string reportUri)
    let topic0 = keccak256(
        "BinaryVersionAttested(uint64,uint64,uint64,address,uint8,uint8,string)".as_bytes(),
    );
    let attestation_id = receipt
        .inner
        .logs()
        .iter()
        .find(|log| log.topics().first() == Some(&topic0) && log.address() == ctx.tangle_contract)
        .and_then(|log| {
            // The non-indexed `attestationId` is the first 32 bytes of `data`.
            let data = log.data().data.as_ref();
            if data.len() < 32 {
                return None;
            }
            let mut idx = [0u8; 8];
            idx.copy_from_slice(&data[24..32]);
            Some(u64::from_be_bytes(idx))
        })
        .ok_or_else(|| {
            eyre!("attestBinaryVersion succeeded but emitted no BinaryVersionAttested log")
        })?;

    Ok(AttestResult {
        blueprint_id,
        version_id,
        attestation_id,
        tx_hash: receipt.transaction_hash,
        block_number: receipt.block_number,
    })
}

pub async fn revoke_attestation(
    ctx: &TxContext,
    blueprint_id: u64,
    version_id: u64,
    attestation_id: u64,
    reason_uri: String,
) -> Result<SimpleTxResult> {
    let call = revokeAttestationCall {
        blueprintId: blueprint_id,
        versionId: version_id,
        attestationId: attestation_id,
        reasonUri: reason_uri,
    };
    let (receipt, _) = send_tx(ctx, call.abi_encode()).await?;
    Ok(SimpleTxResult {
        tx_hash: receipt.transaction_hash,
        block_number: receipt.block_number,
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// Views
// ─────────────────────────────────────────────────────────────────────────────

pub async fn list_versions(ctx: &ViewContext, blueprint_id: u64) -> Result<Vec<BinaryVersion>> {
    let count = call_view::<getBinaryVersionCountCall>(
        ctx,
        getBinaryVersionCountCall {
            blueprintId: blueprint_id,
        },
    )
    .await?;
    let mut out = Vec::with_capacity(count as usize);
    for i in 0..count {
        let version = call_view::<getBinaryVersionCall>(
            ctx,
            getBinaryVersionCall {
                blueprintId: blueprint_id,
                versionId: i,
            },
        )
        .await?;
        out.push(version);
    }
    Ok(out)
}

pub async fn get_version(
    ctx: &ViewContext,
    blueprint_id: u64,
    version_id: u64,
) -> Result<BinaryVersion> {
    call_view::<getBinaryVersionCall>(
        ctx,
        getBinaryVersionCall {
            blueprintId: blueprint_id,
            versionId: version_id,
        },
    )
    .await
}

pub async fn get_active_version_id(ctx: &ViewContext, blueprint_id: u64) -> Result<u64> {
    call_view::<getActiveBinaryVersionIdCall>(
        ctx,
        getActiveBinaryVersionIdCall {
            blueprintId: blueprint_id,
        },
    )
    .await
}

pub async fn get_service_policy(ctx: &ViewContext, service_id: u64) -> Result<UpgradePolicyArg> {
    let raw = call_view::<getServiceUpgradePolicyCall>(
        ctx,
        getServiceUpgradePolicyCall {
            serviceId: service_id,
        },
    )
    .await?;
    UpgradePolicyArg::from_u8(raw)
        .ok_or_else(|| eyre!("contract returned unknown upgrade policy code: {raw}"))
}

pub async fn get_service_acked_version_id(ctx: &ViewContext, service_id: u64) -> Result<u64> {
    call_view::<getServiceAckedVersionIdCall>(
        ctx,
        getServiceAckedVersionIdCall {
            serviceId: service_id,
        },
    )
    .await
}

pub async fn get_effective_version(ctx: &ViewContext, service_id: u64) -> Result<BinaryVersion> {
    call_view::<effectiveBinaryVersionCall>(
        ctx,
        effectiveBinaryVersionCall {
            serviceId: service_id,
        },
    )
    .await
}

pub async fn list_attestations(
    ctx: &ViewContext,
    blueprint_id: u64,
    version_id: u64,
) -> Result<Vec<AttestationRow>> {
    call_view::<listAttestationsCall>(
        ctx,
        listAttestationsCall {
            blueprintId: blueprint_id,
            versionId: version_id,
        },
    )
    .await
}

pub async fn get_auditor(ctx: &ViewContext, who: Address) -> Result<Option<AuditorRow>> {
    let Some(registry) = ctx.auditors_contract else {
        return Ok(None);
    };
    let call = getAuditorCall { auditor: who };
    let provider = ProviderBuilder::new()
        .connect(ctx.http_rpc_url.as_str())
        .await
        .map_err(|e| eyre!("connecting to RPC: {e}"))?;
    let tx_req = TransactionRequest::default()
        .to(registry)
        .input(call.abi_encode().into());
    let output: Bytes = provider
        .call(tx_req)
        .await
        .map_err(|e| eyre!("getAuditor call failed: {e}"))?;
    if output.is_empty() {
        return Ok(None);
    }
    let decoded = <AuditorRow as SolValue>::abi_decode(output.as_ref())
        .map_err(|e| eyre!("decoding AuditorRow: {e}"))?;
    if decoded.admittedAt == 0 {
        return Ok(None);
    }
    Ok(Some(decoded))
}

// ─────────────────────────────────────────────────────────────────────────────
// Trust score
// ─────────────────────────────────────────────────────────────────────────────

/// Trust score breakdown surfaced to CI / the dapp.
#[derive(Debug, Clone)]
pub struct TrustScore {
    pub blueprint_id: u64,
    pub version_id: u64,
    /// Total raw attestations including revoked/expired (informational).
    pub total_attestations: usize,
    /// Attestations that still count toward the score.
    pub valid_attestations: usize,
    /// Sum of active auditor weights for valid attestations.
    pub weighted_sum: u32,
    /// Sum of severity penalties applied (one per valid attestation).
    pub severity_penalty: u32,
    /// 0..=100 trust score (clamped). Computed as
    ///   `(weighted_sum * 100 / MAX_AUDITOR_WEIGHT) - severity_penalty`
    /// then clamped to `[0, 100]`. `MAX_AUDITOR_WEIGHT == 1000` matches the
    /// on-chain constant; one max-weight auditor with no findings yields 100.
    pub score: u32,
    pub per_attestation: Vec<AttestationTrustEntry>,
}

#[derive(Debug, Clone)]
pub struct AttestationTrustEntry {
    pub attestation_id: u64,
    pub attester: Address,
    pub kind: AttestationKindArg,
    pub severity: u8,
    pub revoked: bool,
    pub expired: bool,
    pub auditor_name: Option<String>,
    pub auditor_active: bool,
    pub auditor_weight: u16,
    pub severity_penalty: u32,
}

/// Per-severity penalty applied against the weighted score. Tuned so that a
/// `critical` finding zeros out a max-weight auditor's contribution.
fn severity_penalty(severity: u8) -> u32 {
    match severity {
        0 => 0,
        1 => 2,
        2 => 10,
        3 => 25,
        4 => 50,
        5 => 100,
        _ => 100,
    }
}

const MAX_AUDITOR_WEIGHT: u32 = 1000;

pub async fn compute_trust_score(
    ctx: &ViewContext,
    blueprint_id: u64,
    version_id: u64,
) -> Result<TrustScore> {
    let attestations = list_attestations(ctx, blueprint_id, version_id).await?;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let mut weighted_sum: u32 = 0;
    let mut penalty_total: u32 = 0;
    let mut entries = Vec::with_capacity(attestations.len());
    let mut valid = 0usize;

    for (i, row) in attestations.iter().enumerate() {
        let expired = row.expiresAt != 0 && row.expiresAt <= now;
        let auditor = get_auditor(ctx, row.attester).await?;
        let (weight, active, name) = match &auditor {
            Some(a) => (a.weight, a.active, Some(a.name.clone())),
            None => (0, false, None),
        };
        let kind = AttestationKindArg::from_u8(row.kind).unwrap_or(AttestationKindArg::Self_);
        let counts = !row.revoked && !expired && active && weight > 0;
        let pen = if counts {
            severity_penalty(row.severityFound)
        } else {
            0
        };
        if counts {
            weighted_sum = weighted_sum.saturating_add(u32::from(weight));
            penalty_total = penalty_total.saturating_add(pen);
            valid += 1;
        }
        entries.push(AttestationTrustEntry {
            attestation_id: i as u64,
            attester: row.attester,
            kind,
            severity: row.severityFound,
            revoked: row.revoked,
            expired,
            auditor_name: name,
            auditor_active: active,
            auditor_weight: weight,
            severity_penalty: pen,
        });
    }

    let raw_score = weighted_sum
        .saturating_mul(100)
        .checked_div(MAX_AUDITOR_WEIGHT)
        .unwrap_or(0)
        .saturating_sub(penalty_total)
        .min(100);

    Ok(TrustScore {
        blueprint_id,
        version_id,
        total_attestations: attestations.len(),
        valid_attestations: valid,
        weighted_sum,
        severity_penalty: penalty_total,
        score: raw_score,
        per_attestation: entries,
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal — tx / view machinery
// ─────────────────────────────────────────────────────────────────────────────

async fn build_wallet_signing_key(ctx: &TxContext) -> Result<K256SigningKey> {
    let keystore = load_keystore(&ctx.keystore_path)?;
    load_ecdsa_signing_key(&keystore)
}

async fn send_tx(
    ctx: &TxContext,
    calldata: Vec<u8>,
) -> Result<(alloy_rpc_types_eth::TransactionReceipt, Address)> {
    let signing_key = build_wallet_signing_key(ctx).await?;
    let local_signer = signing_key
        .alloy_key()
        .map_err(|e| eyre!("preparing wallet signer: {e}"))?;
    let from = local_signer.address();
    let wallet = EthereumWallet::from(local_signer);

    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect(ctx.http_rpc_url.as_str())
        .await
        .map_err(|e| eyre!("connecting to RPC at {}: {e}", ctx.http_rpc_url))?;

    let tx_req = TransactionRequest::default()
        .from(from)
        .to(ctx.tangle_contract)
        .input(calldata.into());

    let pending = provider
        .send_transaction(tx_req)
        .await
        .map_err(|e| eyre!("submitting transaction: {e}"))?;
    let receipt = pending
        .get_receipt()
        .await
        .map_err(|e| eyre!("awaiting receipt: {e}"))?;

    if !receipt.status() {
        bail!(
            "transaction {:#x} reverted on-chain (block {:?}, gas_used={})",
            receipt.transaction_hash,
            receipt.block_number,
            receipt.gas_used,
        );
    }
    Ok((receipt, from))
}

async fn call_view<C: SolCall>(ctx: &ViewContext, call: C) -> Result<C::Return> {
    let provider = ProviderBuilder::new()
        .connect(ctx.http_rpc_url.as_str())
        .await
        .map_err(|e| eyre!("connecting to RPC at {}: {e}", ctx.http_rpc_url))?;
    let tx_req = TransactionRequest::default()
        .to(ctx.tangle_contract)
        .input(call.abi_encode().into());
    let output: Bytes = provider
        .call(tx_req)
        .await
        .map_err(|e| eyre!("eth_call failed: {e}"))?;
    let decoded =
        C::abi_decode_returns(output.as_ref()).map_err(|e| eyre!("decoding return value: {e}"))?;
    Ok(decoded)
}

// ─────────────────────────────────────────────────────────────────────────────
// Helpers — duration parsing, address parsing, printing
// ─────────────────────────────────────────────────────────────────────────────

/// Parse a human-readable duration like `6m`, `30d`, `2y`, `90d12h`.
/// Returns the equivalent number of seconds. `0`/empty/"never" → 0.
pub fn parse_duration_to_seconds(input: &str) -> Result<u64> {
    let input = input.trim();
    if input.is_empty() || input == "0" || input.eq_ignore_ascii_case("never") {
        return Ok(0);
    }
    let mut total: u64 = 0;
    let mut current = String::new();
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i] as char;
        if c.is_ascii_digit() {
            current.push(c);
            i += 1;
            continue;
        }
        if current.is_empty() {
            bail!("invalid duration `{input}`: missing number before `{c}`");
        }
        let n: u64 = current
            .parse()
            .with_context(|| format!("parsing number in `{input}`"))?;
        current.clear();
        let mul = match c {
            's' => 1,
            'm' => 60,
            'h' => 60 * 60,
            'd' => 24 * 60 * 60,
            'w' => 7 * 24 * 60 * 60,
            'y' => 365 * 24 * 60 * 60,
            other => bail!("invalid duration `{input}`: unknown unit `{other}`"),
        };
        total = total
            .checked_add(
                n.checked_mul(mul)
                    .ok_or_else(|| eyre!("duration overflow"))?,
            )
            .ok_or_else(|| eyre!("duration overflow"))?;
        i += 1;
    }
    if !current.is_empty() {
        // bare integer → treat as seconds (matches common UNIX convention).
        let n: u64 = current
            .parse()
            .with_context(|| format!("parsing number in `{input}`"))?;
        total = total
            .checked_add(n)
            .ok_or_else(|| eyre!("duration overflow"))?;
    }
    Ok(total)
}

/// Resolve an "expires-in" duration string into an absolute unix timestamp.
pub fn duration_to_expiry_timestamp(input: Option<&str>) -> Result<u64> {
    let Some(input) = input else { return Ok(0) };
    let seconds = parse_duration_to_seconds(input)?;
    if seconds == 0 {
        return Ok(0);
    }
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| eyre!("system clock error: {e}"))?
        .as_secs();
    Ok(now.saturating_add(seconds))
}

/// Parse a single optional `bytes32` from a hex string (with or without `0x`).
pub fn parse_b256(input: &str, label: &str) -> Result<B256> {
    let trimmed = input.trim().trim_start_matches("0x");
    if trimmed.len() != 64 {
        bail!(
            "invalid {label}: expected 32-byte hex, got {} chars",
            trimmed.len()
        );
    }
    let mut buf = [0u8; 32];
    hex::decode_to_slice(trimmed, &mut buf).with_context(|| format!("parsing {label}"))?;
    Ok(B256::from(buf))
}

pub fn parse_address(input: &str, label: &str) -> Result<Address> {
    Address::from_str(input.trim()).map_err(|e| eyre!("invalid {label} `{input}`: {e}"))
}

// ─────────────────────────────────────────────────────────────────────────────
// Pretty-printers
// ─────────────────────────────────────────────────────────────────────────────

pub fn print_publish_result(result: &PublishVersionResult, json_out: bool) {
    if json_out {
        println!(
            "{}",
            json!({
                "event": "binary_version_published",
                "blueprint_id": result.blueprint_id,
                "version_id": result.version_id,
                "sha256_hash": format!("{:#x}", result.sha256_hash),
                "binary_uri": result.binary_uri,
                "attestation_hash": format!("{:#x}", result.attestation_hash),
                "tx_hash": format!("{:#x}", result.tx_hash),
                "block_number": result.block_number,
            })
        );
        return;
    }
    println!(
        "{} version {} for blueprint {}",
        style("Published").green().bold(),
        style(result.version_id).green(),
        result.blueprint_id
    );
    println!("  sha256:           {:#x}", result.sha256_hash);
    println!("  binary_uri:       {}", result.binary_uri);
    println!("  attestation_hash: {:#x}", result.attestation_hash);
    println!("  tx_hash:          {:#x}", result.tx_hash);
    if let Some(block) = result.block_number {
        println!("  block:            {block}");
    }
}

pub fn print_simple_tx(action: &str, tx: &SimpleTxResult, json_out: bool) {
    if json_out {
        println!(
            "{}",
            json!({
                "event": "tx_confirmed",
                "action": action,
                "tx_hash": format!("{:#x}", tx.tx_hash),
                "block_number": tx.block_number,
            })
        );
        return;
    }
    println!(
        "{} {action} (tx_hash={:#x}{})",
        style("OK").green().bold(),
        tx.tx_hash,
        tx.block_number
            .map(|b| format!(", block={b}"))
            .unwrap_or_default()
    );
}

pub fn print_versions_table(
    blueprint_id: u64,
    versions: &[BinaryVersion],
    active: u64,
    json_out: bool,
) {
    if json_out {
        let rows: Vec<Value> = versions
            .iter()
            .map(|v| version_to_json(v, v.versionId == active))
            .collect();
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "blueprint_id": blueprint_id,
                "active_version_id": active,
                "versions": rows,
            }))
            .expect("serialize versions to json")
        );
        return;
    }
    if versions.is_empty() {
        println!("{}", style("No binary versions published").yellow());
        return;
    }
    println!(
        "{} {}  active_version_id={}",
        style("Blueprint").cyan().bold(),
        blueprint_id,
        active
    );
    println!(
        "  {:>3}  {:<18}  {:<7}  {:<12}  {}",
        "id", "sha256 (first 8B)", "deprec.", "published_at", "binary_uri"
    );
    for v in versions {
        let mark = if v.versionId == active { "*" } else { " " };
        let short = format!("{:#x}", v.sha256Hash);
        let short = &short[..18.min(short.len())];
        println!(
            "  {}{:>2}  {:<18}  {:<7}  {:<12}  {}",
            mark, v.versionId, short, v.deprecated, v.publishedAt, v.binaryUri
        );
    }
    println!("(* = currently active)");
}

pub fn print_version_detail(blueprint_id: u64, v: &BinaryVersion, json_out: bool) {
    if json_out {
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "blueprint_id": blueprint_id,
                "version": version_to_json(v, false),
            }))
            .expect("serialize version detail to json")
        );
        return;
    }
    println!(
        "{} {}/{}",
        style("Version").cyan().bold(),
        blueprint_id,
        v.versionId
    );
    println!("  sha256_hash:      {:#x}", v.sha256Hash);
    println!("  binary_uri:       {}", v.binaryUri);
    println!("  attestation_hash: {:#x}", v.attestationHash);
    println!("  published_at:     {}", v.publishedAt);
    println!("  deprecated:       {}", v.deprecated);
}

fn version_to_json(v: &BinaryVersion, active: bool) -> Value {
    json!({
        "version_id": v.versionId,
        "sha256_hash": format!("{:#x}", v.sha256Hash),
        "binary_uri": v.binaryUri,
        "attestation_hash": format!("{:#x}", v.attestationHash),
        "published_at": v.publishedAt,
        "deprecated": v.deprecated,
        "active": active,
    })
}

pub fn print_attest_result(result: &AttestResult, json_out: bool) {
    if json_out {
        println!(
            "{}",
            json!({
                "event": "binary_version_attested",
                "blueprint_id": result.blueprint_id,
                "version_id": result.version_id,
                "attestation_id": result.attestation_id,
                "tx_hash": format!("{:#x}", result.tx_hash),
                "block_number": result.block_number,
            })
        );
        return;
    }
    println!(
        "{} attestation #{} on {}/{}",
        style("Published").green().bold(),
        result.attestation_id,
        result.blueprint_id,
        result.version_id
    );
    println!("  tx_hash: {:#x}", result.tx_hash);
    if let Some(block) = result.block_number {
        println!("  block:   {block}");
    }
}

pub fn print_attestations(
    blueprint_id: u64,
    version_id: u64,
    rows: &[AttestationRow],
    json_out: bool,
) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    if json_out {
        let rows: Vec<Value> = rows
            .iter()
            .enumerate()
            .map(|(i, row)| attestation_to_json(i as u64, row, now))
            .collect();
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "blueprint_id": blueprint_id,
                "version_id": version_id,
                "attestations": rows,
            }))
            .expect("serialize attestations to json")
        );
        return;
    }
    if rows.is_empty() {
        println!("{}", style("No attestations recorded").yellow());
        return;
    }
    println!(
        "{} {}/{}",
        style("Attestations for").cyan().bold(),
        blueprint_id,
        version_id
    );
    for (i, row) in rows.iter().enumerate() {
        let expired = row.expiresAt != 0 && row.expiresAt <= now;
        let status = if row.revoked {
            "revoked"
        } else if expired {
            "expired"
        } else {
            "active"
        };
        let kind = AttestationKindArg::from_u8(row.kind)
            .map(|k| k.as_str())
            .unwrap_or("UNKNOWN");
        println!(
            "  #{i:<3} {:<8} {:<11} sev={:<8} {:#x} → {}",
            kind,
            status,
            SeverityArg::from_u8(row.severityFound),
            row.attester,
            row.reportUri,
        );
    }
}

fn attestation_to_json(id: u64, row: &AttestationRow, now: u64) -> Value {
    let expired = row.expiresAt != 0 && row.expiresAt <= now;
    json!({
        "attestation_id": id,
        "attester": format!("{:#x}", row.attester),
        "report_hash": format!("{:#x}", row.reportHash),
        "report_uri": row.reportUri,
        "kind": AttestationKindArg::from_u8(row.kind).map(|k| k.as_str()).unwrap_or("UNKNOWN"),
        "severity": SeverityArg::from_u8(row.severityFound),
        "attested_at": row.attestedAt,
        "expires_at": row.expiresAt,
        "expired": expired,
        "revoked": row.revoked,
    })
}

pub fn print_effective_version(service_id: u64, v: &BinaryVersion, json_out: bool) {
    if json_out {
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "service_id": service_id,
                "effective_version": version_to_json(v, true),
            }))
            .expect("serialize effective version to json")
        );
        return;
    }
    println!(
        "{} service={} version_id={} sha256={:#x} uri={}",
        style("Effective").cyan().bold(),
        service_id,
        v.versionId,
        v.sha256Hash,
        v.binaryUri
    );
    if v.deprecated {
        println!(
            "  {} this version is flagged deprecated",
            style("warning:").yellow()
        );
    }
}

pub fn print_upgrade_status(ctx: &UpgradeStatus, json_out: bool) {
    if json_out {
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "service_id": ctx.service_id,
                "policy": ctx.policy.as_str(),
                "acked_version_id": ctx.acked_version_id,
                "active_version_id": ctx.active_version_id,
                "effective_version_id": ctx.effective_version_id,
                "latest_version_id": ctx.latest_version_id,
                "up_to_date": ctx.up_to_date,
            }))
            .expect("serialize upgrade status to json")
        );
        return;
    }
    println!(
        "{} service={} policy={}",
        style("Upgrade status").cyan().bold(),
        ctx.service_id,
        ctx.policy.as_str()
    );
    println!("  acked_version_id:     {}", ctx.acked_version_id);
    println!("  active_version_id:    {}", ctx.active_version_id);
    println!("  effective_version_id: {}", ctx.effective_version_id);
    println!("  latest_version_id:    {}", ctx.latest_version_id);
    println!(
        "  up_to_date:           {}",
        if ctx.up_to_date {
            style("yes").green().to_string()
        } else {
            style("no").yellow().to_string()
        }
    );
}

#[derive(Debug, Clone)]
pub struct UpgradeStatus {
    pub service_id: u64,
    pub policy: UpgradePolicyArg,
    pub acked_version_id: u64,
    pub active_version_id: u64,
    pub effective_version_id: u64,
    pub latest_version_id: u64,
    pub up_to_date: bool,
}

pub fn print_trust_score(score: &TrustScore, json_out: bool) {
    if json_out {
        let entries: Vec<Value> = score
            .per_attestation
            .iter()
            .map(|e| {
                json!({
                    "attestation_id": e.attestation_id,
                    "attester": format!("{:#x}", e.attester),
                    "kind": e.kind.as_str(),
                    "severity": SeverityArg::from_u8(e.severity),
                    "revoked": e.revoked,
                    "expired": e.expired,
                    "auditor_name": e.auditor_name,
                    "auditor_active": e.auditor_active,
                    "auditor_weight": e.auditor_weight,
                    "severity_penalty": e.severity_penalty,
                })
            })
            .collect();
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "blueprint_id": score.blueprint_id,
                "version_id": score.version_id,
                "score": score.score,
                "weighted_sum": score.weighted_sum,
                "severity_penalty": score.severity_penalty,
                "valid_attestations": score.valid_attestations,
                "total_attestations": score.total_attestations,
                "attestations": entries,
            }))
            .expect("serialize trust score to json")
        );
        return;
    }
    let label = match score.score {
        90..=100 => style("HIGH").green().bold().to_string(),
        50..=89 => style("MEDIUM").yellow().bold().to_string(),
        _ => style("LOW").red().bold().to_string(),
    };
    println!(
        "{} {}/100 [{}]  blueprint={} version={}  (valid={}, total={})",
        style("Trust score:").cyan().bold(),
        score.score,
        label,
        score.blueprint_id,
        score.version_id,
        score.valid_attestations,
        score.total_attestations,
    );
    println!(
        "  weighted_sum={}  severity_penalty={}",
        score.weighted_sum, score.severity_penalty
    );
    for entry in &score.per_attestation {
        let status = if entry.revoked {
            "revoked"
        } else if entry.expired {
            "expired"
        } else if !entry.auditor_active || entry.auditor_weight == 0 {
            "unweighted"
        } else {
            "counted"
        };
        let name = entry
            .auditor_name
            .clone()
            .unwrap_or_else(|| "<anon>".into());
        println!(
            "    #{:<3} {:<10} weight={:<4} sev={:<8} kind={:<8} by {} ({:#x})",
            entry.attestation_id,
            status,
            entry.auditor_weight,
            SeverityArg::from_u8(entry.severity),
            entry.kind.as_str(),
            name,
            entry.attester
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests (parse-only / pure functions)
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_simple_durations() {
        assert_eq!(parse_duration_to_seconds("6m").unwrap(), 6 * 60);
        assert_eq!(parse_duration_to_seconds("30d").unwrap(), 30 * 24 * 3600);
        assert_eq!(parse_duration_to_seconds("1y").unwrap(), 365 * 24 * 3600);
        assert_eq!(
            parse_duration_to_seconds("90d12h").unwrap(),
            90 * 24 * 3600 + 12 * 3600
        );
        assert_eq!(parse_duration_to_seconds("0").unwrap(), 0);
        assert_eq!(parse_duration_to_seconds("never").unwrap(), 0);
        assert_eq!(parse_duration_to_seconds("").unwrap(), 0);
        // bare integers default to seconds
        assert_eq!(parse_duration_to_seconds("3600").unwrap(), 3600);
    }

    #[test]
    fn rejects_garbage_durations() {
        assert!(parse_duration_to_seconds("nope").is_err());
        assert!(parse_duration_to_seconds("5q").is_err());
    }

    #[test]
    fn parses_bytes32() {
        let h = parse_b256(
            "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
            "report-hash",
        )
        .unwrap();
        assert_eq!(format!("{h:#x}").len(), 66);
        assert!(parse_b256("0xabc", "report-hash").is_err());
    }

    #[test]
    fn upgrade_policy_encodes_correctly() {
        assert_eq!(UpgradePolicyArg::Approve.as_u8(), 0);
        assert_eq!(UpgradePolicyArg::Auto.as_u8(), 1);
        assert_eq!(UpgradePolicyArg::Manual.as_u8(), 2);
        assert_eq!(UpgradePolicyArg::from_u8(1), Some(UpgradePolicyArg::Auto));
        assert!(UpgradePolicyArg::from_u8(9).is_none());
    }

    #[test]
    fn attestation_kind_encodes_correctly() {
        assert_eq!(AttestationKindArg::Audit.as_u8(), 0);
        assert_eq!(AttestationKindArg::Self_.as_u8(), 4);
        assert_eq!(
            AttestationKindArg::from_u8(3),
            Some(AttestationKindArg::BugBounty)
        );
    }

    #[test]
    fn severity_ladder_matches_contract() {
        assert_eq!(SeverityArg::None.as_u8(), 0);
        assert_eq!(SeverityArg::Critical.as_u8(), 5);
        assert_eq!(SeverityArg::from_u8(3), "med");
    }

    #[test]
    fn hash_file_matches_known_digest() {
        let mut tmp = std::env::temp_dir();
        tmp.push("cargo-tangle-hash-test.bin");
        std::fs::write(&tmp, b"hello world").unwrap();
        let (digest, len) = hash_file(&tmp).unwrap();
        // Known sha256 of "hello world"
        let expected: B256 = parse_b256(
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9",
            "expected",
        )
        .unwrap();
        assert_eq!(digest, expected);
        assert_eq!(len, 11);
        let _ = std::fs::remove_file(&tmp);
    }
}
