// SPDX-License-Identifier: UNLICENSE
pragma solidity >=0.8.13;

import "tnt-core/BlueprintServiceManagerBase.sol";

/**
 * @title ExperimentalBlueprint
 * @dev OAuth Blueprint that manages scoped document operations
 * @dev Follows BlueprintServiceManagerBase spec for automatic job-to-hook wiring
 */
contract ExperimentalBlueprint is BlueprintServiceManagerBase {
    // Events for job execution tracking
    event DocumentWritten(
        bytes32 indexed tenantHash,
        string docId,
        uint256 timestamp,
        uint64 indexed serviceId,
        uint64 indexed callId
    );

    event DocumentRead(
        bytes32 indexed tenantHash,
        string docId,
        uint256 timestamp,
        uint64 indexed serviceId,
        uint64 indexed callId
    );

    event ScopeChecked(
        bytes32 indexed tenantHash,
        string scope,
        bool authorized,
        uint64 indexed serviceId,
        uint64 indexed callId
    );

    event AdminPurgeExecuted(
        bytes32 indexed targetTenant,
        bytes32 indexed adminTenant,
        uint256 timestamp,
        uint64 indexed serviceId,
        uint64 indexed callId
    );

    // Job IDs that correspond to Rust job constants
    uint8 public constant WHOAMI_JOB_ID = 0;
    uint8 public constant CHECK_SCOPE_JOB_ID = 1;
    uint8 public constant WRITE_DOC_JOB_ID = 2;
    uint8 public constant READ_DOC_JOB_ID = 3;
    uint8 public constant LIST_DOCS_JOB_ID = 4;
    uint8 public constant ADMIN_PURGE_JOB_ID = 5;
    uint8 public constant ECHO_JOB_ID = 6;

    // Document tracking per tenant
    mapping(bytes32 => mapping(string => bool)) public tenantDocuments;
    mapping(bytes32 => uint256) public tenantDocumentCount;
    mapping(bytes32 => string[]) public tenantDocumentIds;

    // Job call tracking
    mapping(uint64 => mapping(uint64 => bool)) public processedJobCalls;

    // Scope usage tracking
    mapping(bytes32 => mapping(string => uint256)) public scopeUsageCount;

    /**
     * @dev Hook called automatically when any job is submitted from Rust
     * @dev This is triggered before job execution on the Rust side
     */
    function onJobCall(
        uint64 serviceId,
        uint8 job,
        uint64 jobCallId,
        bytes calldata inputs
    ) external payable override onlyFromMaster {
        // Track that this job call was received
        processedJobCalls[serviceId][jobCallId] = true;

        // Pre-execution validation could happen here
        // For OAuth blueprint, we rely on Rust-side scope enforcement
    }

    /**
     * @dev Hook called automatically when job execution completes on Rust side
     * @dev This receives the job results and updates contract state accordingly
     */
    function onJobResult(
        uint64 serviceId,
        uint8 job,
        uint64 jobCallId,
        ServiceOperators.OperatorPreferences calldata operator,
        bytes calldata inputs,
        bytes calldata outputs
    ) external payable override onlyFromMaster {
        // Derive tenant hash from operator key for tracking
        bytes32 tenantHash = keccak256(abi.encodePacked("tenant_", operator.ecdsaKey));

        if (job == WRITE_DOC_JOB_ID) {
            // Decode document data from inputs
            (string memory docId, string memory content) = abi.decode(inputs, (string, string));
            
            // Track document creation per tenant
            if (!tenantDocuments[tenantHash][docId]) {
                tenantDocuments[tenantHash][docId] = true;
                tenantDocumentCount[tenantHash]++;
                tenantDocumentIds[tenantHash].push(docId);
            }

            emit DocumentWritten(
                tenantHash,
                docId,
                block.timestamp,
                serviceId,
                jobCallId
            );

        } else if (job == READ_DOC_JOB_ID) {
            // Decode document ID from inputs
            string memory docId = abi.decode(inputs, (string));

            emit DocumentRead(
                tenantHash,
                docId,
                block.timestamp,
                serviceId,
                jobCallId
            );

        } else if (job == CHECK_SCOPE_JOB_ID) {
            // Decode scope from inputs
            string memory scope = abi.decode(inputs, (string));
            
            // Decode authorization result from outputs
            // outputs should contain the job result with authorization status
            bool authorized = true; // Default assumption, could decode from outputs
            
            // Track scope usage
            scopeUsageCount[tenantHash][scope]++;

            emit ScopeChecked(
                tenantHash,
                scope,
                authorized,
                serviceId,
                jobCallId
            );

        } else if (job == ADMIN_PURGE_JOB_ID) {
            // Decode target tenant from inputs
            string memory targetTenantStr = abi.decode(inputs, (string));
            bytes32 targetTenant = keccak256(abi.encodePacked("tenant_", targetTenantStr));

            emit AdminPurgeExecuted(
                targetTenant,
                tenantHash, // admin who executed the purge
                block.timestamp,
                serviceId,
                jobCallId
            );
        }

        // WHOAMI_JOB_ID and ECHO_JOB_ID don't need contract state changes
        // but their execution is still tracked via processedJobCalls
    }

    /**
     * @dev Get tenant document count
     */
    function getTenantDocumentCount(bytes32 tenantHash) external view returns (uint256) {
        return tenantDocumentCount[tenantHash];
    }

    /**
     * @dev Check if tenant has specific document
     */
    function tenantHasDocument(bytes32 tenantHash, string memory docId) external view returns (bool) {
        return tenantDocuments[tenantHash][docId];
    }

    /**
     * @dev Get all document IDs for a tenant
     */
    function getTenantDocumentIds(bytes32 tenantHash) external view returns (string[] memory) {
        return tenantDocumentIds[tenantHash];
    }

    /**
     * @dev Get scope usage count for a tenant
     */
    function getScopeUsageCount(bytes32 tenantHash, string memory scope) external view returns (uint256) {
        return scopeUsageCount[tenantHash][scope];
    }

    /**
     * @dev Check if job call was processed
     */
    function wasJobCallProcessed(uint64 serviceId, uint64 jobCallId) external view returns (bool) {
        return processedJobCalls[serviceId][jobCallId];
    }
}