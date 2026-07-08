//! Local Solidity ABI stubs for `BlueprintsBinaryVersions`.
//!
//! These mirror the views/events/mutators introduced in tnt-core's
//! `src/core/BlueprintsBinaryVersions.sol` (struct shape tracks tnt-core 0.19's
//! `Types.BinaryVersion`). They live here as a sidecar until
//! `tnt-core-bindings` ships this facet's typed interface directly; once that
//! lands, this module is deleted and callers re-import the types from
//! `blueprint_client_tangle::contracts`.
//
// TODO: replace with tnt-core-bindings typed `IBlueprintBinaryVersions` once published.

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
        // Mirrors tnt-core 0.19 `Types.BinaryVersion` (src/libraries/Types.sol).
        // Field order is storage-packing sensitive on-chain and must match the
        // canonical struct exactly for ABI-decode of the view returns to line up.
        // NOTE: 0.19 removed `binaryUri` from this struct — the URI is now
        // event-only, carried on `BinaryVersionPublished` below. `getBinaryVersion`
        // / `effectiveBinaryVersion` no longer return a URI; callers must source it
        // from the event log (see chain.rs::binary_uri_from_event).
        struct BinaryVersion {
            uint64 versionId;
            uint64 publishedAt;
            bool deprecated;
            bytes32 sha256Hash;
            bytes32 attestationHash;
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
