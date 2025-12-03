// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import { BlueprintServiceManagerBase } from "@tnt-core/BlueprintServiceManagerBase.sol";

/// @title IncredibleSquaringBSM
/// @notice Blueprint Service Manager for the Incredible Squaring example
/// @dev This contract demonstrates comprehensive hook tracking for testing and verification.
///      Every hook increments a counter and stores relevant data for later verification.
///
/// Tracked hooks:
/// - Blueprint lifecycle: onBlueprintCreated
/// - Operator lifecycle: onRegister, onUnregister, onUpdatePreferences
/// - Service lifecycle: onRequest, onApprove, onReject, onServiceInitialized, onServiceTermination
/// - Dynamic membership: onOperatorJoined, onOperatorLeft
/// - Job lifecycle: onJobCall, onJobResult
/// - Slashing: onUnappliedSlash, onSlash
contract IncredibleSquaringBSM is BlueprintServiceManagerBase {
    // ═══════════════════════════════════════════════════════════════════════════
    // EVENTS
    // ═══════════════════════════════════════════════════════════════════════════

    event BlueprintCreatedHook(uint64 indexed blueprintId, address indexed owner);
    event OperatorRegisteredHook(address indexed operator, bytes data);
    event OperatorUnregisteredHook(address indexed operator);
    event OperatorPreferencesUpdatedHook(address indexed operator, bytes data);
    event ServiceRequestedHook(uint64 indexed requestId, address indexed requester);
    event ServiceApprovedHook(uint64 indexed requestId, address indexed operator, uint8 restakingPercent);
    event ServiceRejectedHook(uint64 indexed requestId, address indexed operator);
    event ServiceInitializedHook(uint64 indexed serviceId, uint64 indexed requestId, address indexed owner);
    event ServiceTerminatedHook(uint64 indexed serviceId, address indexed owner);
    event OperatorJoinedHook(uint64 indexed serviceId, address indexed operator, uint16 exposureBps);
    event OperatorLeftHook(uint64 indexed serviceId, address indexed operator);
    event JobCallHook(uint64 indexed serviceId, uint8 indexed jobIndex, uint64 callId, bytes inputs);
    event JobResultHook(uint64 indexed serviceId, uint64 indexed callId, address indexed operator, bytes outputs);
    event UnappliedSlashHook(uint64 indexed serviceId, bytes offender, uint8 slashPercent);
    event SlashAppliedHook(uint64 indexed serviceId, bytes offender, uint8 slashPercent);

    // ═══════════════════════════════════════════════════════════════════════════
    // HOOK CALL COUNTERS
    // ═══════════════════════════════════════════════════════════════════════════

    struct HookCounters {
        uint256 blueprintCreated;
        uint256 register;
        uint256 unregister;
        uint256 updatePreferences;
        uint256 request;
        uint256 approve;
        uint256 reject;
        uint256 serviceInitialized;
        uint256 serviceTermination;
        uint256 operatorJoined;
        uint256 operatorLeft;
        uint256 jobCall;
        uint256 jobResult;
        uint256 unappliedSlash;
        uint256 slash;
    }

    HookCounters public counters;

    // ═══════════════════════════════════════════════════════════════════════════
    // OPERATOR TRACKING
    // ═══════════════════════════════════════════════════════════════════════════

    /// @notice List of all operators who have ever registered
    address[] public operatorList;

    /// @notice Mapping of operator address to registration status
    mapping(address => bool) public isOperatorRegistered;

    /// @notice Mapping of operator address to their registration inputs
    mapping(address => bytes) public operatorRegistrationData;

    /// @notice Mapping of operator address to their current preferences
    mapping(address => bytes) public operatorPreferences;

    /// @notice Count of times each operator has registered
    mapping(address => uint256) public operatorRegistrationCount;

    // ═══════════════════════════════════════════════════════════════════════════
    // SERVICE TRACKING
    // ═══════════════════════════════════════════════════════════════════════════

    /// @notice List of all service IDs
    uint64[] public serviceList;

    /// @notice Mapping of service ID to active status
    mapping(uint64 => bool) public isServiceActive;

    /// @notice Mapping of request ID to request inputs
    mapping(uint64 => bytes) public requestInputs;

    /// @notice Mapping of request ID to requester address
    mapping(uint64 => address) public requesters;

    /// @notice Mapping of service ID to owner address
    mapping(uint64 => address) public serviceOwners;

    /// @notice Mapping of service ID to TTL
    mapping(uint64 => uint64) public serviceTTLs;

    /// @notice Count of approvals per request
    mapping(uint64 => uint256) public requestApprovalCount;

    /// @notice Count of rejections per request
    mapping(uint64 => uint256) public requestRejectionCount;

    // ═══════════════════════════════════════════════════════════════════════════
    // DYNAMIC MEMBERSHIP TRACKING
    // ═══════════════════════════════════════════════════════════════════════════

    /// @notice Operators who joined each service
    mapping(uint64 => address[]) public serviceOperatorsJoined;

    /// @notice Operators who left each service
    mapping(uint64 => address[]) public serviceOperatorsLeft;

    /// @notice Current operator count per service
    mapping(uint64 => uint256) public serviceOperatorCount;

    // ═══════════════════════════════════════════════════════════════════════════
    // JOB TRACKING
    // ═══════════════════════════════════════════════════════════════════════════

    /// @notice Total job call count per service
    mapping(uint64 => uint256) public serviceJobCallCount;

    /// @notice Total job result count per service
    mapping(uint64 => uint256) public serviceJobResultCount;

    /// @notice Mapping of (serviceId, callId) to job inputs
    mapping(uint64 => mapping(uint64 => bytes)) public jobInputs;

    /// @notice Mapping of (serviceId, callId) to last submitted outputs
    mapping(uint64 => mapping(uint64 => bytes)) public jobOutputs;

    /// @notice Mapping of (serviceId, callId) to job index
    mapping(uint64 => mapping(uint64 => uint8)) public jobIndices;

    /// @notice Mapping of (serviceId, callId) to result count
    mapping(uint64 => mapping(uint64 => uint256)) public jobResultCount;

    /// @notice Mapping of (serviceId, callId, operator) to whether they submitted
    mapping(uint64 => mapping(uint64 => mapping(address => bool))) public operatorSubmittedResult;

    /// @notice Last call ID per service
    mapping(uint64 => uint64) public lastCallId;

    // ═══════════════════════════════════════════════════════════════════════════
    // SLASHING TRACKING
    // ═══════════════════════════════════════════════════════════════════════════

    /// @notice Pending slash count per service
    mapping(uint64 => uint256) public pendingSlashCount;

    /// @notice Applied slash count per service
    mapping(uint64 => uint256) public appliedSlashCount;

    // ═══════════════════════════════════════════════════════════════════════════
    // BLUEPRINT LIFECYCLE
    // ═══════════════════════════════════════════════════════════════════════════

    /// @inheritdoc BlueprintServiceManagerBase
    function onBlueprintCreated(
        uint64 _blueprintId,
        address owner,
        address _tangleCore
    ) external virtual override {
        require(tangleCore == address(0), "Already initialized");

        blueprintId = _blueprintId;
        blueprintOwner = owner;
        tangleCore = _tangleCore;

        counters.blueprintCreated++;

        emit BlueprintCreatedHook(_blueprintId, owner);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // OPERATOR LIFECYCLE
    // ═══════════════════════════════════════════════════════════════════════════

    /// @inheritdoc BlueprintServiceManagerBase
    function onRegister(
        address operator,
        bytes calldata inputs
    ) external payable virtual override onlyFromTangle {
        counters.register++;

        if (!isOperatorRegistered[operator]) {
            operatorList.push(operator);
        }
        isOperatorRegistered[operator] = true;
        operatorRegistrationData[operator] = inputs;
        operatorRegistrationCount[operator]++;

        emit OperatorRegisteredHook(operator, inputs);
    }

    /// @inheritdoc BlueprintServiceManagerBase
    function onUnregister(address operator) external virtual override onlyFromTangle {
        counters.unregister++;

        isOperatorRegistered[operator] = false;

        emit OperatorUnregisteredHook(operator);
    }

    /// @inheritdoc BlueprintServiceManagerBase
    function onUpdatePreferences(
        address operator,
        bytes calldata newPreferences
    ) external payable virtual override onlyFromTangle {
        counters.updatePreferences++;

        operatorPreferences[operator] = newPreferences;

        emit OperatorPreferencesUpdatedHook(operator, newPreferences);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // SERVICE LIFECYCLE
    // ═══════════════════════════════════════════════════════════════════════════

    /// @inheritdoc BlueprintServiceManagerBase
    function onRequest(
        uint64 requestId,
        address requester,
        address[] calldata,
        bytes calldata _requestInputs,
        uint64,
        address,
        uint256
    ) external payable virtual override onlyFromTangle {
        counters.request++;

        requestInputs[requestId] = _requestInputs;
        requesters[requestId] = requester;

        emit ServiceRequestedHook(requestId, requester);
    }

    /// @inheritdoc BlueprintServiceManagerBase
    function onApprove(
        address operator,
        uint64 requestId,
        uint8 restakingPercent
    ) external payable virtual override onlyFromTangle {
        counters.approve++;

        requestApprovalCount[requestId]++;

        emit ServiceApprovedHook(requestId, operator, restakingPercent);
    }

    /// @inheritdoc BlueprintServiceManagerBase
    function onReject(
        address operator,
        uint64 requestId
    ) external virtual override onlyFromTangle {
        counters.reject++;

        requestRejectionCount[requestId]++;

        emit ServiceRejectedHook(requestId, operator);
    }

    /// @inheritdoc BlueprintServiceManagerBase
    function onServiceInitialized(
        uint64,
        uint64 requestId,
        uint64 serviceId,
        address owner,
        address[] calldata,
        uint64 ttl
    ) external virtual override onlyFromTangle {
        counters.serviceInitialized++;

        serviceList.push(serviceId);
        isServiceActive[serviceId] = true;
        serviceOwners[serviceId] = owner;
        serviceTTLs[serviceId] = ttl;

        emit ServiceInitializedHook(serviceId, requestId, owner);
    }

    /// @inheritdoc BlueprintServiceManagerBase
    function onServiceTermination(
        uint64 serviceId,
        address owner
    ) external virtual override onlyFromTangle {
        counters.serviceTermination++;

        isServiceActive[serviceId] = false;

        emit ServiceTerminatedHook(serviceId, owner);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // DYNAMIC MEMBERSHIP
    // ═══════════════════════════════════════════════════════════════════════════

    /// @inheritdoc BlueprintServiceManagerBase
    function onOperatorJoined(
        uint64 serviceId,
        address operator,
        uint16 exposureBps
    ) external virtual override onlyFromTangle {
        counters.operatorJoined++;

        serviceOperatorsJoined[serviceId].push(operator);
        serviceOperatorCount[serviceId]++;

        emit OperatorJoinedHook(serviceId, operator, exposureBps);
    }

    /// @inheritdoc BlueprintServiceManagerBase
    function onOperatorLeft(
        uint64 serviceId,
        address operator
    ) external virtual override onlyFromTangle {
        counters.operatorLeft++;

        serviceOperatorsLeft[serviceId].push(operator);
        if (serviceOperatorCount[serviceId] > 0) {
            serviceOperatorCount[serviceId]--;
        }

        emit OperatorLeftHook(serviceId, operator);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // JOB LIFECYCLE
    // ═══════════════════════════════════════════════════════════════════════════

    /// @inheritdoc BlueprintServiceManagerBase
    function onJobCall(
        uint64 serviceId,
        uint8 jobIndex,
        uint64 callId,
        bytes calldata inputs
    ) external payable virtual override onlyFromTangle {
        counters.jobCall++;

        serviceJobCallCount[serviceId]++;
        jobInputs[serviceId][callId] = inputs;
        jobIndices[serviceId][callId] = jobIndex;
        lastCallId[serviceId] = callId;

        emit JobCallHook(serviceId, jobIndex, callId, inputs);
    }

    /// @inheritdoc BlueprintServiceManagerBase
    function onJobResult(
        uint64 serviceId,
        uint8,
        uint64 callId,
        address operator,
        bytes calldata,
        bytes calldata outputs
    ) external payable virtual override onlyFromTangle {
        counters.jobResult++;

        serviceJobResultCount[serviceId]++;
        jobOutputs[serviceId][callId] = outputs;
        jobResultCount[serviceId][callId]++;
        operatorSubmittedResult[serviceId][callId][operator] = true;

        emit JobResultHook(serviceId, callId, operator, outputs);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // SLASHING
    // ═══════════════════════════════════════════════════════════════════════════

    /// @inheritdoc BlueprintServiceManagerBase
    function onUnappliedSlash(
        uint64 serviceId,
        bytes calldata offender,
        uint8 slashPercent
    ) external virtual override onlyFromTangle {
        counters.unappliedSlash++;

        pendingSlashCount[serviceId]++;

        emit UnappliedSlashHook(serviceId, offender, slashPercent);
    }

    /// @inheritdoc BlueprintServiceManagerBase
    function onSlash(
        uint64 serviceId,
        bytes calldata offender,
        uint8 slashPercent
    ) external virtual override onlyFromTangle {
        counters.slash++;

        appliedSlashCount[serviceId]++;
        if (pendingSlashCount[serviceId] > 0) {
            pendingSlashCount[serviceId]--;
        }

        emit SlashAppliedHook(serviceId, offender, slashPercent);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // VIEW FUNCTIONS - COUNTERS
    // ═══════════════════════════════════════════════════════════════════════════

    /// @notice Get all hook call counters
    /// @return The HookCounters struct with all counts
    function getCounters() external view returns (HookCounters memory) {
        return counters;
    }

    /// @notice Get the total number of hook calls across all hooks
    /// @return Total count
    function getTotalHookCalls() external view returns (uint256) {
        return counters.blueprintCreated
            + counters.register
            + counters.unregister
            + counters.updatePreferences
            + counters.request
            + counters.approve
            + counters.reject
            + counters.serviceInitialized
            + counters.serviceTermination
            + counters.operatorJoined
            + counters.operatorLeft
            + counters.jobCall
            + counters.jobResult
            + counters.unappliedSlash
            + counters.slash;
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // VIEW FUNCTIONS - OPERATORS
    // ═══════════════════════════════════════════════════════════════════════════

    /// @notice Get total number of unique operators who registered
    /// @return Count of operators
    function getOperatorCount() external view returns (uint256) {
        return operatorList.length;
    }

    /// @notice Get list of all operators
    /// @return Array of operator addresses
    function getOperators() external view returns (address[] memory) {
        return operatorList;
    }

    /// @notice Get count of currently registered operators
    /// @return Count of active operators
    function getActiveOperatorCount() external view returns (uint256) {
        uint256 count = 0;
        for (uint256 i = 0; i < operatorList.length; i++) {
            if (isOperatorRegistered[operatorList[i]]) {
                count++;
            }
        }
        return count;
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // VIEW FUNCTIONS - SERVICES
    // ═══════════════════════════════════════════════════════════════════════════

    /// @notice Get total number of services ever created
    /// @return Count of services
    function getServiceCount() external view returns (uint256) {
        return serviceList.length;
    }

    /// @notice Get list of all service IDs
    /// @return Array of service IDs
    function getServices() external view returns (uint64[] memory) {
        return serviceList;
    }

    /// @notice Get count of currently active services
    /// @return Count of active services
    function getActiveServiceCount() external view returns (uint256) {
        uint256 count = 0;
        for (uint256 i = 0; i < serviceList.length; i++) {
            if (isServiceActive[serviceList[i]]) {
                count++;
            }
        }
        return count;
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // VIEW FUNCTIONS - JOBS
    // ═══════════════════════════════════════════════════════════════════════════

    /// @notice Get job details for a specific call
    /// @param serviceId The service ID
    /// @param callId The call ID
    /// @return jobIndex The job index
    /// @return inputs The job inputs
    /// @return outputs The job outputs (last submitted)
    /// @return resultCount Number of results submitted
    function getJobDetails(
        uint64 serviceId,
        uint64 callId
    ) external view returns (
        uint8 jobIndex,
        bytes memory inputs,
        bytes memory outputs,
        uint256 resultCount
    ) {
        return (
            jobIndices[serviceId][callId],
            jobInputs[serviceId][callId],
            jobOutputs[serviceId][callId],
            jobResultCount[serviceId][callId]
        );
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // VIEW FUNCTIONS - MEMBERSHIP
    // ═══════════════════════════════════════════════════════════════════════════

    /// @notice Get operators who joined a service
    /// @param serviceId The service ID
    /// @return Array of operator addresses
    function getOperatorsJoined(uint64 serviceId) external view returns (address[] memory) {
        return serviceOperatorsJoined[serviceId];
    }

    /// @notice Get operators who left a service
    /// @param serviceId The service ID
    /// @return Array of operator addresses
    function getOperatorsLeft(uint64 serviceId) external view returns (address[] memory) {
        return serviceOperatorsLeft[serviceId];
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // JOB CONFIGURATION - AGGREGATION REQUIREMENTS
    // ═══════════════════════════════════════════════════════════════════════════

    /// @notice Job index constants (must match Rust lib)
    uint8 public constant JOB_SQUARE = 0;           // Basic square - 1 operator
    uint8 public constant JOB_VERIFIED_SQUARE = 1;  // Verified - 2 operators
    uint8 public constant JOB_CONSENSUS_SQUARE = 2; // Consensus - 3 operators

    /// @notice Get the number of results required to complete a job
    /// @dev Different jobs have different aggregation requirements:
    ///      - Job 0 (square): 1 result (single operator, immediate completion)
    ///      - Job 1 (verified_square): 2 results (redundancy, higher confidence)
    ///      - Job 2 (consensus_square): 3 results (Byzantine fault tolerance)
    /// @param serviceId The service ID (not used in this implementation)
    /// @param jobIndex The job index determining aggregation requirement
    /// @return required Number of operator results needed before job completes
    function getRequiredResultCount(
        uint64 serviceId,
        uint8 jobIndex
    ) external view virtual override returns (uint32) {
        // Silence unused variable warning
        serviceId;

        if (jobIndex == JOB_SQUARE) {
            return 1; // Single operator result sufficient
        } else if (jobIndex == JOB_VERIFIED_SQUARE) {
            return 2; // Two operators must submit results
        } else if (jobIndex == JOB_CONSENSUS_SQUARE) {
            return 3; // Three operators for consensus/quorum
        }

        // Default: require single result for unknown jobs
        return 1;
    }

    /// @notice Check if a job has aggregation requirements
    /// @param jobIndex The job index
    /// @return requiresMultiple True if job requires more than 1 operator result
    function jobRequiresAggregation(uint8 jobIndex) external pure returns (bool) {
        return jobIndex == JOB_VERIFIED_SQUARE || jobIndex == JOB_CONSENSUS_SQUARE;
    }

    /// @notice Get the aggregation requirement description for a job
    /// @param jobIndex The job index
    /// @return description Human-readable description of the requirement
    function getJobAggregationDescription(uint8 jobIndex) external pure returns (string memory) {
        if (jobIndex == JOB_SQUARE) {
            return "Single operator (no aggregation)";
        } else if (jobIndex == JOB_VERIFIED_SQUARE) {
            return "Two operators (verification)";
        } else if (jobIndex == JOB_CONSENSUS_SQUARE) {
            return "Three operators (consensus/quorum)";
        }
        return "Unknown job";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // VERIFICATION HELPERS
    // ═══════════════════════════════════════════════════════════════════════════

    /// @notice Verify that the basic lifecycle hooks were called in the expected order
    /// @dev Checks that blueprintCreated, register, request, approve, serviceInitialized were all called at least once
    /// @return passed True if all basic lifecycle hooks were called
    function verifyBasicLifecycle() public view returns (bool) {
        return counters.blueprintCreated >= 1
            && counters.register >= 1
            && counters.request >= 1
            && counters.approve >= 1
            && counters.serviceInitialized >= 1;
    }

    /// @notice Verify that job hooks were called
    /// @return passed True if at least one job was submitted and completed
    function verifyJobLifecycle() public view returns (bool) {
        return counters.jobCall >= 1 && counters.jobResult >= 1;
    }

    /// @notice Verify complete lifecycle including termination
    /// @return passed True if complete lifecycle was exercised
    function verifyCompleteLifecycle() public view returns (bool) {
        return verifyBasicLifecycle()
            && verifyJobLifecycle()
            && counters.serviceTermination >= 1;
    }
}
