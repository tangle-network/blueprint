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
use alloy_rpc_types::TransactionRequest;
use alloy_sol_types::SolCall;
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
            Ok(version) => Ok(BinaryVersionInfo {
                version_id: version.versionId,
                sha256: version.sha256Hash,
                binary_uri: version.binaryUri,
                attestation_hash: version.attestationHash,
                published_at: version.publishedAt,
                deprecated: version.deprecated,
            }),
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
