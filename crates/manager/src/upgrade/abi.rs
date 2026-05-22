//! Local Solidity ABI stubs for `BlueprintsBinaryVersions`.
//!
//! These mirror the views/events/mutators introduced in tnt-core's
//! `src/core/BlueprintsBinaryVersions.sol`. They live here as a sidecar until
//! `tnt-core-bindings` is bumped to a version that ships this surface
//! (expected v0.18+); once that lands, this module is deleted and callers
//! re-import the types from `blueprint_client_tangle::contracts`.
//
// TODO: replace with tnt-core-bindings v0.18+ types once published.

use alloy_sol_types::sol;

sol! {
    /// Subset of the on-chain `BlueprintsBinaryVersions` interface that the
    /// manager needs to subscribe to and call. Selectors are derived from the
    /// canonical Solidity signatures, so the stub is wire-compatible with the
    /// audited contract — it only stops being authoritative on the day the
    /// real binding ships.
    #[allow(missing_docs)]
    #[sol(rpc)]
    interface IBlueprintBinaryVersions {
        // ----- structs ------------------------------------------------------
        struct BinaryVersion {
            uint64 versionId;
            bytes32 sha256Hash;
            string binaryUri;
            bytes32 attestationHash;
            uint64 publishedAt;
            bool deprecated;
        }

        // ----- enums --------------------------------------------------------
        // `UpgradePolicy` matches the on-chain enum ordering exactly:
        //   APPROVE = 0
        //   AUTO    = 1
        //   MANUAL  = 2
        // Reordering MUST NOT happen — the contract documents this invariant.

        // ----- read methods -------------------------------------------------
        function getBinaryVersion(uint64 blueprintId, uint64 versionId)
            external
            view
            returns (BinaryVersion memory);

        function getBinaryVersionCount(uint64 blueprintId)
            external
            view
            returns (uint64);

        function getActiveBinaryVersionId(uint64 blueprintId)
            external
            view
            returns (uint64);

        function getServiceUpgradePolicy(uint64 serviceId)
            external
            view
            returns (uint8);

        function getServiceAckedVersionId(uint64 serviceId)
            external
            view
            returns (uint64);

        function effectiveBinaryVersion(uint64 serviceId)
            external
            view
            returns (BinaryVersion memory);

        // ----- write methods ------------------------------------------------
        function ackBinaryVersion(uint64 serviceId, uint64 versionId) external;
        function setServiceUpgradePolicy(uint64 serviceId, uint8 policy) external;

        // ----- events -------------------------------------------------------
        event BinaryVersionPublished(
            uint64 indexed blueprintId,
            uint64 indexed versionId,
            bytes32 sha256Hash,
            string binaryUri
        );
        event BinaryVersionDeprecated(uint64 indexed blueprintId, uint64 indexed versionId);
        event BinaryActiveVersionChanged(uint64 indexed blueprintId, uint64 indexed versionId);
        event ServiceUpgradePolicySet(uint64 indexed serviceId, uint8 policy);
        event OperatorBinaryAcked(
            uint64 indexed serviceId,
            uint64 indexed versionId,
            address indexed operator
        );
    }
}
