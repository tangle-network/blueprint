//! On-chain interactions for the binary-version upgrade flow.
//!
//! Wraps the local `IBlueprintBinaryVersions` ABI stub against the Tangle
//! contract address so the watcher and the operator CLI share one source of
//! truth. Once `tnt-core-bindings v0.18` ships, the stub goes away and this
//! module just re-exports the bound interface.

use super::abi::IBlueprintBinaryVersions;
use super::error::{Result, UpgradeError};
use super::types::{BinaryVersionInfo, UpgradePolicy};
use alloy_primitives::{Address, B256};
use alloy_provider::{Provider, ProviderBuilder};
use alloy_rpc_types::{Filter, TransactionRequest};
use alloy_sol_types::{SolCall, SolEvent};
use blueprint_client_tangle::{TangleClient, client::TangleProvider};
use std::sync::Arc;

/// Conservative gas floor for the two single-state-write mutators
/// (`ackBinaryVersion`, `setServiceUpgradePolicy`). Both touch one storage
/// slot + one event; the contract estimator usually returns ~80k. We pad to
/// 200k so a brief node hiccup on `eth_estimateGas` doesn't strand the tx.
const MIN_GAS_LIMIT: u64 = 200_000;

/// Read-only view + write helpers backed by the Tangle contract address.
#[derive(Clone)]
pub struct ChainView {
    client: TangleClient,
    tangle_address: Address,
}

impl ChainView {
    #[must_use]
    pub fn new(client: TangleClient, tangle_address: Address) -> Self {
        Self {
            client,
            tangle_address,
        }
    }

    #[must_use]
    pub fn operator(&self) -> Address {
        self.client.account()
    }

    fn contract(
        &self,
    ) -> IBlueprintBinaryVersions::IBlueprintBinaryVersionsInstance<Arc<TangleProvider>> {
        // SAFETY note: we attach the binary-versions interface at the same
        // address the rest of the manager uses for `ITangle`. That is the
        // diamond entry point — every facet (including the audit's
        // `BlueprintsBinaryVersions` mixin) is delegated through it.
        let provider = Arc::clone(self.client.provider());
        IBlueprintBinaryVersions::IBlueprintBinaryVersionsInstance::new(
            self.tangle_address,
            provider,
        )
    }

    /// `getBinaryVersionCount(blueprintId)`
    pub async fn binary_version_count(&self, blueprint_id: u64) -> Result<u64> {
        self.contract()
            .getBinaryVersionCount(blueprint_id)
            .call()
            .await
            .map_err(|e| UpgradeError::ChainRead(format!("getBinaryVersionCount: {e}")))
    }

    /// `getActiveBinaryVersionId(blueprintId)`
    pub async fn active_binary_version_id(&self, blueprint_id: u64) -> Result<u64> {
        self.contract()
            .getActiveBinaryVersionId(blueprint_id)
            .call()
            .await
            .map_err(|e| UpgradeError::ChainRead(format!("getActiveBinaryVersionId: {e}")))
    }

    /// `getServiceUpgradePolicy(serviceId)`
    pub async fn service_upgrade_policy(&self, service_id: u64) -> Result<UpgradePolicy> {
        let raw = self
            .contract()
            .getServiceUpgradePolicy(service_id)
            .call()
            .await
            .map_err(|e| UpgradeError::ChainRead(format!("getServiceUpgradePolicy: {e}")))?;
        Ok(UpgradePolicy::from_u8(raw))
    }

    /// `getServiceAckedVersionId(serviceId)`
    pub async fn service_acked_version_id(&self, service_id: u64) -> Result<u64> {
        self.contract()
            .getServiceAckedVersionId(service_id)
            .call()
            .await
            .map_err(|e| UpgradeError::ChainRead(format!("getServiceAckedVersionId: {e}")))
    }

    /// `getBinaryVersion(blueprintId, versionId)`
    pub async fn get_binary_version(
        &self,
        blueprint_id: u64,
        version_id: u64,
    ) -> Result<BinaryVersionInfo> {
        let version = self
            .contract()
            .getBinaryVersion(blueprint_id, version_id)
            .call()
            .await
            .map_err(|e| UpgradeError::ChainRead(format!("getBinaryVersion: {e}")))?;
        // tnt-core 0.19: the view no longer returns the URI. Source it from the
        // `BinaryVersionPublished` event for this exact blueprint+version.
        let binary_uri = self
            .binary_uri_from_event(blueprint_id, version.versionId)
            .await?;
        Ok(BinaryVersionInfo {
            version_id: version.versionId,
            sha256: version.sha256Hash,
            binary_uri,
            attestation_hash: version.attestationHash,
            published_at: version.publishedAt,
            deprecated: version.deprecated,
        })
    }

    /// Resolve a binary's download URI from its `BinaryVersionPublished` event.
    ///
    /// tnt-core 0.19 dropped `binaryUri` from the `BinaryVersion` struct — the
    /// view returns no URI, so the manager reads it back from the event log the
    /// contract emitted at publish time:
    /// `BinaryVersionPublished(blueprintId indexed, versionId indexed, sha256Hash, binaryUri)`.
    ///
    /// The filter is scoped to the Tangle contract address (`self.tangle_address`)
    /// AND the indexed `blueprintId`/`versionId` topics. The address scope is the
    /// security boundary: event signatures and indexed topics are global, so
    /// without it any contract could emit a matching event and inject a rogue URI.
    /// With the address scope, at most one log matches (versions are append-only
    /// and immutable). We scan newest-first in bounded block windows so a
    /// provider's `eth_getLogs` span cap can't reject the query, and stop at the
    /// first match.
    async fn binary_uri_from_event(&self, blueprint_id: u64, version_id: u64) -> Result<String> {
        // Bounded per-request span. Kept well under common provider caps
        // (Infura/Alchemy ~10k, many self-hosted nodes 100k). The doubly-indexed
        // filter means the match is usually in the first (newest) window.
        const GETLOGS_WINDOW: u64 = 10_000;

        // Indexed topics are left-padded to 32 bytes, big-endian.
        let blueprint_topic = u64_topic(blueprint_id);
        let version_topic = u64_topic(version_id);

        let head = self
            .client
            .block_number()
            .await
            .map_err(|e| UpgradeError::ChainRead(format!("block_number: {e}")))?;

        let mut to_block = head;
        loop {
            let from_block = to_block.saturating_sub(GETLOGS_WINDOW.saturating_sub(1));
            let filter = Filter::new()
                .address(self.tangle_address)
                .event_signature(IBlueprintBinaryVersions::BinaryVersionPublished::SIGNATURE_HASH)
                .topic1(blueprint_topic)
                .topic2(version_topic)
                .from_block(from_block)
                .to_block(to_block);

            let logs = self
                .client
                .get_logs(&filter)
                .await
                .map_err(|e| UpgradeError::ChainRead(format!("BinaryVersionPublished logs: {e}")))?;

            // Newest wins if a chain ever re-emitted (it shouldn't: versions are
            // immutable). Event signatures and indexed topics are global and
            // attacker-choosable, so the `.address(self.tangle_address)` scope
            // above — NOT the blueprintId/versionId topics — is what stops another
            // contract from spoofing a matching `BinaryVersionPublished` and
            // feeding an attacker-controlled `binaryUri` into `download_and_verify`.
            // A decode failure must still be skipped rather than abort the scan.
            for log in logs.iter().rev() {
                if let Ok(decoded) =
                    IBlueprintBinaryVersions::BinaryVersionPublished::decode_log(&log.inner)
                {
                    // `decode_log` yields `Log<BinaryVersionPublished>`; the URI
                    // is reached via `Deref`, so clone it out of the borrow.
                    return Ok(decoded.binaryUri.clone());
                }
            }

            if from_block == 0 {
                break;
            }
            to_block = from_block.saturating_sub(1);
        }

        Err(UpgradeError::ChainRead(format!(
            "no BinaryVersionPublished event for blueprint {blueprint_id} version {version_id}"
        )))
    }

    /// `effectiveBinaryVersion(serviceId)` — the protocol's source of truth
    /// for "what binary should this service be running right now."
    ///
    /// Reverts on-chain (`VersionNotFound`) when no version has been
    /// published for the underlying blueprint; we translate that into
    /// `NoVersionsPublished` so the watcher can treat the service as "not
    /// yet provisioned" rather than a hard error.
    pub async fn effective_binary_version(
        &self,
        service_id: u64,
        blueprint_id: u64,
    ) -> Result<BinaryVersionInfo> {
        match self
            .contract()
            .effectiveBinaryVersion(service_id)
            .call()
            .await
        {
            Ok(version) => {
                // tnt-core 0.19: URI is event-only. Resolve it from the
                // `BinaryVersionPublished` log for the effective version id.
                let binary_uri = self
                    .binary_uri_from_event(blueprint_id, version.versionId)
                    .await?;
                Ok(BinaryVersionInfo {
                    version_id: version.versionId,
                    sha256: version.sha256Hash,
                    binary_uri,
                    attestation_hash: version.attestationHash,
                    published_at: version.publishedAt,
                    deprecated: version.deprecated,
                })
            }
            Err(err) => {
                if is_version_not_found(&err) {
                    Err(UpgradeError::NoVersionsPublished { blueprint_id })
                } else {
                    Err(UpgradeError::ChainRead(format!(
                        "effectiveBinaryVersion: {err}"
                    )))
                }
            }
        }
    }

    /// Submit `ackBinaryVersion(serviceId, versionId)` from the manager's
    /// signing key. Returns the transaction hash.
    pub async fn ack_binary_version(&self, service_id: u64, version_id: u64) -> Result<B256> {
        let calldata = IBlueprintBinaryVersions::ackBinaryVersionCall {
            serviceId: service_id,
            versionId: version_id,
        }
        .abi_encode();
        self.send_call(calldata).await
    }

    /// Submit `setServiceUpgradePolicy(serviceId, policy)` from the manager's
    /// signing key. Returns the transaction hash.
    pub async fn set_service_upgrade_policy(
        &self,
        service_id: u64,
        policy: UpgradePolicy,
    ) -> Result<B256> {
        let calldata = IBlueprintBinaryVersions::setServiceUpgradePolicyCall {
            serviceId: service_id,
            policy: policy.as_u8(),
        }
        .abi_encode();
        self.send_call(calldata).await
    }

    /// Common path for both mutators: build a wallet-bound provider, attach
    /// `from`, estimate-with-fallback, and surface revert reasons as
    /// `ChainWrite` errors rather than swallowing them as silent failures.
    async fn send_call(&self, calldata: Vec<u8>) -> Result<B256> {
        let wallet = self
            .client
            .wallet()
            .map_err(|e| UpgradeError::ChainWrite(format!("wallet: {e}")))?;
        let from = wallet.default_signer().address();
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect(self.client.config.http_rpc_endpoint.as_str())
            .await
            .map_err(|e| UpgradeError::ChainWrite(format!("provider: {e}")))?;

        let tx = TransactionRequest::default()
            .from(from)
            .to(self.tangle_address)
            .input(alloy_primitives::Bytes::from(calldata).into());

        let gas_limit = match provider.estimate_gas(tx.clone()).await {
            Ok(estimated) => buffered_gas_limit(estimated, MIN_GAS_LIMIT),
            Err(_) => MIN_GAS_LIMIT,
        };

        let pending = provider
            .send_transaction(tx.gas_limit(gas_limit))
            .await
            .map_err(|e| UpgradeError::ChainWrite(format!("send: {e}")))?;

        let receipt = pending
            .get_receipt()
            .await
            .map_err(|e| UpgradeError::ChainWrite(format!("receipt: {e}")))?;

        if !receipt.status() {
            return Err(UpgradeError::ChainWrite(format!(
                "tx {} reverted (block={:?}, gas_used={})",
                receipt.transaction_hash, receipt.block_number, receipt.gas_used
            )));
        }
        Ok(receipt.transaction_hash)
    }
}

/// Match ITangle's `VersionNotFound()` custom error in the RPC error payload.
///
/// We can't import the binding for the selector until tnt-core-bindings v0.18
/// lands (the `BlueprintsBinaryVersions` mixin is not in 0.17.1), so for now
/// we rely on the string label. Both alloy and most RPC providers surface the
/// custom-error name in the human-readable message even when they ship the
/// raw selector hex.
//
// TODO: switch to `ITangleErrors::VersionNotFound::SELECTOR` once the
// regenerated bindings land.
fn is_version_not_found(err: &impl ToString) -> bool {
    err.to_string()
        .to_ascii_lowercase()
        .contains("versionnotfound")
}

fn buffered_gas_limit(estimated: u64, min_gas_limit: u64) -> u64 {
    let buffered = estimated.saturating_add(estimated / 10);
    buffered.max(min_gas_limit)
}

/// Encode a `uint64` indexed event argument as a 32-byte log topic
/// (left-padded, big-endian) for `eth_getLogs` topic filtering.
fn u64_topic(value: u64) -> B256 {
    let mut topic = [0u8; 32];
    topic[24..].copy_from_slice(&value.to_be_bytes());
    B256::from(topic)
}
