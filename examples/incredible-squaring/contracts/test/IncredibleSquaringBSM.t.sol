// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import { Test, console } from "forge-std/Test.sol";
import { IncredibleSquaringBSM } from "../src/IncredibleSquaringBSM.sol";
import { BlueprintServiceManagerBase } from "@tnt-core/BlueprintServiceManagerBase.sol";

/// @title IncredibleSquaringBSMTest
/// @notice Comprehensive tests for the IncredibleSquaringBSM hook tracking
contract IncredibleSquaringBSMTest is Test {
    IncredibleSquaringBSM public bsm;

    address public tangleCore;
    address public blueprintOwner;
    address public operator1;
    address public operator2;
    address public serviceRequester;

    uint64 public constant BLUEPRINT_ID = 1;
    uint64 public constant REQUEST_ID = 1;
    uint64 public constant SERVICE_ID = 1;
    uint64 public constant CALL_ID = 1;
    uint8 public constant JOB_INDEX = 0;

    function setUp() public {
        bsm = new IncredibleSquaringBSM();

        tangleCore = address(this); // Test contract acts as Tangle core
        blueprintOwner = makeAddr("blueprintOwner");
        operator1 = makeAddr("operator1");
        operator2 = makeAddr("operator2");
        serviceRequester = makeAddr("serviceRequester");
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // BLUEPRINT LIFECYCLE TESTS
    // ═══════════════════════════════════════════════════════════════════════════

    function test_OnBlueprintCreated() public {
        bsm.onBlueprintCreated(BLUEPRINT_ID, blueprintOwner, tangleCore);

        assertEq(bsm.blueprintId(), BLUEPRINT_ID);
        assertEq(bsm.blueprintOwner(), blueprintOwner);
        assertEq(bsm.tangleCore(), tangleCore);

        IncredibleSquaringBSM.HookCounters memory counters = bsm.getCounters();
        assertEq(counters.blueprintCreated, 1);
    }

    function test_OnBlueprintCreated_RevertIfAlreadyInitialized() public {
        bsm.onBlueprintCreated(BLUEPRINT_ID, blueprintOwner, tangleCore);

        vm.expectRevert("Already initialized");
        bsm.onBlueprintCreated(2, blueprintOwner, tangleCore);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // OPERATOR LIFECYCLE TESTS
    // ═══════════════════════════════════════════════════════════════════════════

    function test_OnRegister() public {
        _initializeBlueprint();

        bytes memory registrationData = abi.encode("operator1 preferences");
        bsm.onRegister(operator1, registrationData);

        assertTrue(bsm.isOperatorRegistered(operator1));
        assertEq(bsm.getOperatorCount(), 1);
        assertEq(bsm.operatorRegistrationCount(operator1), 1);
        assertEq(bsm.operatorRegistrationData(operator1), registrationData);

        IncredibleSquaringBSM.HookCounters memory counters = bsm.getCounters();
        assertEq(counters.register, 1);
    }

    function test_OnRegister_MultipleOperators() public {
        _initializeBlueprint();

        bsm.onRegister(operator1, "");
        bsm.onRegister(operator2, "");

        assertEq(bsm.getOperatorCount(), 2);
        assertTrue(bsm.isOperatorRegistered(operator1));
        assertTrue(bsm.isOperatorRegistered(operator2));

        IncredibleSquaringBSM.HookCounters memory counters = bsm.getCounters();
        assertEq(counters.register, 2);
    }

    function test_OnRegister_SameOperatorTwice() public {
        _initializeBlueprint();

        bsm.onRegister(operator1, "first");
        bsm.onRegister(operator1, "second");

        // Should not duplicate in operator list
        assertEq(bsm.getOperatorCount(), 1);
        // But should increment registration count
        assertEq(bsm.operatorRegistrationCount(operator1), 2);
        // And update registration data
        assertEq(string(bsm.operatorRegistrationData(operator1)), "second");
    }

    function test_OnUnregister() public {
        _initializeBlueprint();

        bsm.onRegister(operator1, "");
        assertTrue(bsm.isOperatorRegistered(operator1));

        bsm.onUnregister(operator1);
        assertFalse(bsm.isOperatorRegistered(operator1));

        IncredibleSquaringBSM.HookCounters memory counters = bsm.getCounters();
        assertEq(counters.unregister, 1);
    }

    function test_OnUpdatePreferences() public {
        _initializeBlueprint();

        bsm.onRegister(operator1, "initial");

        bytes memory newPrefs = abi.encode("updated preferences");
        bsm.onUpdatePreferences(operator1, newPrefs);

        assertEq(bsm.operatorPreferences(operator1), newPrefs);

        IncredibleSquaringBSM.HookCounters memory counters = bsm.getCounters();
        assertEq(counters.updatePreferences, 1);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // SERVICE LIFECYCLE TESTS
    // ═══════════════════════════════════════════════════════════════════════════

    function test_OnRequest() public {
        _initializeBlueprint();

        bytes memory requestData = abi.encode("service config");
        address[] memory operators = new address[](1);
        operators[0] = operator1;

        bsm.onRequest(REQUEST_ID, serviceRequester, operators, requestData, 1000, address(0), 1 ether);

        assertEq(bsm.requestInputs(REQUEST_ID), requestData);
        assertEq(bsm.requesters(REQUEST_ID), serviceRequester);

        IncredibleSquaringBSM.HookCounters memory counters = bsm.getCounters();
        assertEq(counters.request, 1);
    }

    function test_OnApprove() public {
        _initializeBlueprint();
        _createServiceRequest();

        bsm.onApprove(operator1, REQUEST_ID, 50);

        assertEq(bsm.requestApprovalCount(REQUEST_ID), 1);

        IncredibleSquaringBSM.HookCounters memory counters = bsm.getCounters();
        assertEq(counters.approve, 1);
    }

    function test_OnApprove_MultipleOperators() public {
        _initializeBlueprint();
        _createServiceRequest();

        bsm.onApprove(operator1, REQUEST_ID, 50);
        bsm.onApprove(operator2, REQUEST_ID, 30);

        assertEq(bsm.requestApprovalCount(REQUEST_ID), 2);

        IncredibleSquaringBSM.HookCounters memory counters = bsm.getCounters();
        assertEq(counters.approve, 2);
    }

    function test_OnReject() public {
        _initializeBlueprint();
        _createServiceRequest();

        bsm.onReject(operator1, REQUEST_ID);

        assertEq(bsm.requestRejectionCount(REQUEST_ID), 1);

        IncredibleSquaringBSM.HookCounters memory counters = bsm.getCounters();
        assertEq(counters.reject, 1);
    }

    function test_OnServiceInitialized() public {
        _initializeBlueprint();
        _createServiceRequest();
        _approveService();

        address[] memory callers = new address[](1);
        callers[0] = serviceRequester;

        bsm.onServiceInitialized(BLUEPRINT_ID, REQUEST_ID, SERVICE_ID, serviceRequester, callers, 1000);

        assertTrue(bsm.isServiceActive(SERVICE_ID));
        assertEq(bsm.serviceOwners(SERVICE_ID), serviceRequester);
        assertEq(bsm.serviceTTLs(SERVICE_ID), 1000);
        assertEq(bsm.getServiceCount(), 1);

        IncredibleSquaringBSM.HookCounters memory counters = bsm.getCounters();
        assertEq(counters.serviceInitialized, 1);
    }

    function test_OnServiceTermination() public {
        _initializeBlueprint();
        _setupActiveService();

        bsm.onServiceTermination(SERVICE_ID, serviceRequester);

        assertFalse(bsm.isServiceActive(SERVICE_ID));
        assertEq(bsm.getActiveServiceCount(), 0);

        IncredibleSquaringBSM.HookCounters memory counters = bsm.getCounters();
        assertEq(counters.serviceTermination, 1);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // DYNAMIC MEMBERSHIP TESTS
    // ═══════════════════════════════════════════════════════════════════════════

    function test_OnOperatorJoined() public {
        _initializeBlueprint();
        _setupActiveService();

        bsm.onOperatorJoined(SERVICE_ID, operator1, 5000); // 50%

        assertEq(bsm.serviceOperatorCount(SERVICE_ID), 1);

        address[] memory joined = bsm.getOperatorsJoined(SERVICE_ID);
        assertEq(joined.length, 1);
        assertEq(joined[0], operator1);

        IncredibleSquaringBSM.HookCounters memory counters = bsm.getCounters();
        assertEq(counters.operatorJoined, 1);
    }

    function test_OnOperatorLeft() public {
        _initializeBlueprint();
        _setupActiveService();

        bsm.onOperatorJoined(SERVICE_ID, operator1, 5000);
        bsm.onOperatorLeft(SERVICE_ID, operator1);

        assertEq(bsm.serviceOperatorCount(SERVICE_ID), 0);

        address[] memory left = bsm.getOperatorsLeft(SERVICE_ID);
        assertEq(left.length, 1);
        assertEq(left[0], operator1);

        IncredibleSquaringBSM.HookCounters memory counters = bsm.getCounters();
        assertEq(counters.operatorLeft, 1);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // JOB LIFECYCLE TESTS
    // ═══════════════════════════════════════════════════════════════════════════

    function test_OnJobCall() public {
        _initializeBlueprint();
        _setupActiveService();

        // ABI-encode a u64 value (5) for squaring
        bytes memory inputs = abi.encode(uint64(5));

        bsm.onJobCall(SERVICE_ID, JOB_INDEX, CALL_ID, inputs);

        assertEq(bsm.serviceJobCallCount(SERVICE_ID), 1);
        assertEq(bsm.jobInputs(SERVICE_ID, CALL_ID), inputs);
        assertEq(bsm.jobIndices(SERVICE_ID, CALL_ID), JOB_INDEX);
        assertEq(bsm.lastCallId(SERVICE_ID), CALL_ID);

        IncredibleSquaringBSM.HookCounters memory counters = bsm.getCounters();
        assertEq(counters.jobCall, 1);
    }

    function test_OnJobResult() public {
        _initializeBlueprint();
        _setupActiveService();

        bytes memory inputs = abi.encode(uint64(5));
        bsm.onJobCall(SERVICE_ID, JOB_INDEX, CALL_ID, inputs);

        // ABI-encode the result (25)
        bytes memory outputs = abi.encode(uint64(25));

        bsm.onJobResult(SERVICE_ID, JOB_INDEX, CALL_ID, operator1, inputs, outputs);

        assertEq(bsm.serviceJobResultCount(SERVICE_ID), 1);
        assertEq(bsm.jobOutputs(SERVICE_ID, CALL_ID), outputs);
        assertEq(bsm.jobResultCount(SERVICE_ID, CALL_ID), 1);
        assertTrue(bsm.operatorSubmittedResult(SERVICE_ID, CALL_ID, operator1));

        IncredibleSquaringBSM.HookCounters memory counters = bsm.getCounters();
        assertEq(counters.jobResult, 1);
    }

    function test_OnJobResult_MultipleOperators() public {
        _initializeBlueprint();
        _setupActiveService();

        bytes memory inputs = abi.encode(uint64(5));
        bytes memory outputs = abi.encode(uint64(25));

        bsm.onJobCall(SERVICE_ID, JOB_INDEX, CALL_ID, inputs);
        bsm.onJobResult(SERVICE_ID, JOB_INDEX, CALL_ID, operator1, inputs, outputs);
        bsm.onJobResult(SERVICE_ID, JOB_INDEX, CALL_ID, operator2, inputs, outputs);

        assertEq(bsm.jobResultCount(SERVICE_ID, CALL_ID), 2);
        assertTrue(bsm.operatorSubmittedResult(SERVICE_ID, CALL_ID, operator1));
        assertTrue(bsm.operatorSubmittedResult(SERVICE_ID, CALL_ID, operator2));

        IncredibleSquaringBSM.HookCounters memory counters = bsm.getCounters();
        assertEq(counters.jobResult, 2);
    }

    function test_GetJobDetails() public {
        _initializeBlueprint();
        _setupActiveService();

        bytes memory inputs = abi.encode(uint64(7));
        bytes memory outputs = abi.encode(uint64(49));

        bsm.onJobCall(SERVICE_ID, JOB_INDEX, CALL_ID, inputs);
        bsm.onJobResult(SERVICE_ID, JOB_INDEX, CALL_ID, operator1, inputs, outputs);

        (uint8 jobIndex, bytes memory gotInputs, bytes memory gotOutputs, uint256 resultCount) =
            bsm.getJobDetails(SERVICE_ID, CALL_ID);

        assertEq(jobIndex, JOB_INDEX);
        assertEq(gotInputs, inputs);
        assertEq(gotOutputs, outputs);
        assertEq(resultCount, 1);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // SLASHING TESTS
    // ═══════════════════════════════════════════════════════════════════════════

    function test_OnUnappliedSlash() public {
        _initializeBlueprint();
        _setupActiveService();

        bytes memory offender = abi.encodePacked(operator1);
        bsm.onUnappliedSlash(SERVICE_ID, offender, 10);

        assertEq(bsm.pendingSlashCount(SERVICE_ID), 1);

        IncredibleSquaringBSM.HookCounters memory counters = bsm.getCounters();
        assertEq(counters.unappliedSlash, 1);
    }

    function test_OnSlash() public {
        _initializeBlueprint();
        _setupActiveService();

        bytes memory offender = abi.encodePacked(operator1);
        bsm.onUnappliedSlash(SERVICE_ID, offender, 10);
        bsm.onSlash(SERVICE_ID, offender, 10);

        assertEq(bsm.pendingSlashCount(SERVICE_ID), 0);
        assertEq(bsm.appliedSlashCount(SERVICE_ID), 1);

        IncredibleSquaringBSM.HookCounters memory counters = bsm.getCounters();
        assertEq(counters.slash, 1);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // VERIFICATION TESTS
    // ═══════════════════════════════════════════════════════════════════════════

    function test_VerifyBasicLifecycle() public {
        // Initially should fail
        assertFalse(bsm.verifyBasicLifecycle());

        _initializeBlueprint();
        assertFalse(bsm.verifyBasicLifecycle());

        bsm.onRegister(operator1, "");
        assertFalse(bsm.verifyBasicLifecycle());

        _createServiceRequest();
        assertFalse(bsm.verifyBasicLifecycle());

        _approveService();
        assertFalse(bsm.verifyBasicLifecycle());

        _activateService();
        assertTrue(bsm.verifyBasicLifecycle());
    }

    function test_VerifyJobLifecycle() public {
        assertFalse(bsm.verifyJobLifecycle());

        _initializeBlueprint();
        _setupActiveService();

        bytes memory inputs = abi.encode(uint64(5));
        bsm.onJobCall(SERVICE_ID, JOB_INDEX, CALL_ID, inputs);
        assertFalse(bsm.verifyJobLifecycle());

        bytes memory outputs = abi.encode(uint64(25));
        bsm.onJobResult(SERVICE_ID, JOB_INDEX, CALL_ID, operator1, inputs, outputs);
        assertTrue(bsm.verifyJobLifecycle());
    }

    function test_VerifyCompleteLifecycle() public {
        assertFalse(bsm.verifyCompleteLifecycle());

        _initializeBlueprint();
        _setupActiveService();

        bytes memory inputs = abi.encode(uint64(5));
        bytes memory outputs = abi.encode(uint64(25));
        bsm.onJobCall(SERVICE_ID, JOB_INDEX, CALL_ID, inputs);
        bsm.onJobResult(SERVICE_ID, JOB_INDEX, CALL_ID, operator1, inputs, outputs);
        assertFalse(bsm.verifyCompleteLifecycle());

        bsm.onServiceTermination(SERVICE_ID, serviceRequester);
        assertTrue(bsm.verifyCompleteLifecycle());
    }

    function test_GetTotalHookCalls() public {
        _initializeBlueprint();
        assertEq(bsm.getTotalHookCalls(), 1);

        _setupActiveService();
        // blueprintCreated + register + request + approve + serviceInitialized
        assertEq(bsm.getTotalHookCalls(), 5);

        bytes memory inputs = abi.encode(uint64(5));
        bytes memory outputs = abi.encode(uint64(25));
        bsm.onJobCall(SERVICE_ID, JOB_INDEX, CALL_ID, inputs);
        bsm.onJobResult(SERVICE_ID, JOB_INDEX, CALL_ID, operator1, inputs, outputs);
        assertEq(bsm.getTotalHookCalls(), 7);

        bsm.onServiceTermination(SERVICE_ID, serviceRequester);
        assertEq(bsm.getTotalHookCalls(), 8);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // FULL E2E TEST - INCREDIBLE SQUARING
    // ═══════════════════════════════════════════════════════════════════════════

    function test_FullIncredibleSquaringFlow() public {
        // 1. Blueprint created
        bsm.onBlueprintCreated(BLUEPRINT_ID, blueprintOwner, tangleCore);

        // 2. Operators register
        bsm.onRegister(operator1, abi.encode("operator1 key"));
        bsm.onRegister(operator2, abi.encode("operator2 key"));

        // 3. Service requested
        address[] memory operators = new address[](2);
        operators[0] = operator1;
        operators[1] = operator2;
        bsm.onRequest(REQUEST_ID, serviceRequester, operators, "", 3600, address(0), 0);

        // 4. Operators approve
        bsm.onApprove(operator1, REQUEST_ID, 50);
        bsm.onApprove(operator2, REQUEST_ID, 50);

        // 5. Service initialized
        address[] memory callers = new address[](1);
        callers[0] = serviceRequester;
        bsm.onServiceInitialized(BLUEPRINT_ID, REQUEST_ID, SERVICE_ID, serviceRequester, callers, 3600);

        // Verify basic lifecycle
        assertTrue(bsm.verifyBasicLifecycle());

        // 6. Submit a job: square(5)
        bytes memory inputs1 = abi.encode(uint64(5));
        bsm.onJobCall(SERVICE_ID, JOB_INDEX, CALL_ID, inputs1);

        // 7. Operators submit results
        bytes memory outputs1 = abi.encode(uint64(25));
        bsm.onJobResult(SERVICE_ID, JOB_INDEX, CALL_ID, operator1, inputs1, outputs1);
        bsm.onJobResult(SERVICE_ID, JOB_INDEX, CALL_ID, operator2, inputs1, outputs1);

        // Verify job lifecycle
        assertTrue(bsm.verifyJobLifecycle());

        // 8. Submit another job: square(10)
        uint64 callId2 = 2;
        bytes memory inputs2 = abi.encode(uint64(10));
        bsm.onJobCall(SERVICE_ID, JOB_INDEX, callId2, inputs2);

        bytes memory outputs2 = abi.encode(uint64(100));
        bsm.onJobResult(SERVICE_ID, JOB_INDEX, callId2, operator1, inputs2, outputs2);

        // Verify job details
        (uint8 jobIndex, bytes memory gotInputs, bytes memory gotOutputs, uint256 resultCount) =
            bsm.getJobDetails(SERVICE_ID, callId2);

        assertEq(jobIndex, JOB_INDEX);
        assertEq(abi.decode(gotInputs, (uint64)), 10);
        assertEq(abi.decode(gotOutputs, (uint64)), 100);
        assertEq(resultCount, 1);

        // 9. Terminate service
        bsm.onServiceTermination(SERVICE_ID, serviceRequester);

        // Verify complete lifecycle
        assertTrue(bsm.verifyCompleteLifecycle());

        // Final stats
        IncredibleSquaringBSM.HookCounters memory counters = bsm.getCounters();
        assertEq(counters.blueprintCreated, 1);
        assertEq(counters.register, 2);
        assertEq(counters.request, 1);
        assertEq(counters.approve, 2);
        assertEq(counters.serviceInitialized, 1);
        assertEq(counters.jobCall, 2);
        assertEq(counters.jobResult, 3);
        assertEq(counters.serviceTermination, 1);

        console.log("Total hook calls:", bsm.getTotalHookCalls());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // AGGREGATION TESTS
    // ═══════════════════════════════════════════════════════════════════════════

    function test_GetRequiredResultCount_BasicSquare() public {
        _initializeBlueprint();

        // Job 0 (square) requires 1 result
        uint32 required = bsm.getRequiredResultCount(SERVICE_ID, 0);
        assertEq(required, 1, "Basic square should require 1 operator");
    }

    function test_GetRequiredResultCount_VerifiedSquare() public {
        _initializeBlueprint();

        // Job 1 (verified_square) requires 2 results
        uint32 required = bsm.getRequiredResultCount(SERVICE_ID, 1);
        assertEq(required, 2, "Verified square should require 2 operators");
    }

    function test_GetRequiredResultCount_ConsensusSquare() public {
        _initializeBlueprint();

        // Job 2 (consensus_square) requires 3 results
        uint32 required = bsm.getRequiredResultCount(SERVICE_ID, 2);
        assertEq(required, 3, "Consensus square should require 3 operators");
    }

    function test_GetRequiredResultCount_UnknownJob() public {
        _initializeBlueprint();

        // Unknown jobs default to 1 result
        uint32 required = bsm.getRequiredResultCount(SERVICE_ID, 99);
        assertEq(required, 1, "Unknown jobs should default to 1 operator");
    }

    function test_JobRequiresAggregation() public {
        assertFalse(bsm.jobRequiresAggregation(0), "Job 0 should not require aggregation");
        assertTrue(bsm.jobRequiresAggregation(1), "Job 1 should require aggregation");
        assertTrue(bsm.jobRequiresAggregation(2), "Job 2 should require aggregation");
    }

    function test_JobConstants() public {
        assertEq(bsm.JOB_SQUARE(), 0);
        assertEq(bsm.JOB_VERIFIED_SQUARE(), 1);
        assertEq(bsm.JOB_CONSENSUS_SQUARE(), 2);
    }

    function test_GetJobAggregationDescription() public {
        assertEq(
            bsm.getJobAggregationDescription(0),
            "Single operator (no aggregation)"
        );
        assertEq(
            bsm.getJobAggregationDescription(1),
            "Two operators (verification)"
        );
        assertEq(
            bsm.getJobAggregationDescription(2),
            "Three operators (consensus/quorum)"
        );
        assertEq(
            bsm.getJobAggregationDescription(99),
            "Unknown job"
        );
    }

    function test_VerifiedSquareJob_RequiresTwoResults() public {
        _initializeBlueprint();
        _setupActiveServiceWithMultipleOperators();

        // Submit a verified_square job (index 1)
        uint8 verifiedJobIndex = 1;
        uint64 callId = 100;
        bytes memory inputs = abi.encode(uint64(7));
        bytes memory outputs = abi.encode(uint64(49));

        bsm.onJobCall(SERVICE_ID, verifiedJobIndex, callId, inputs);

        // First operator submits
        bsm.onJobResult(SERVICE_ID, verifiedJobIndex, callId, operator1, inputs, outputs);
        assertEq(bsm.jobResultCount(SERVICE_ID, callId), 1);
        assertTrue(bsm.operatorSubmittedResult(SERVICE_ID, callId, operator1));
        assertFalse(bsm.operatorSubmittedResult(SERVICE_ID, callId, operator2));

        // Second operator submits
        bsm.onJobResult(SERVICE_ID, verifiedJobIndex, callId, operator2, inputs, outputs);
        assertEq(bsm.jobResultCount(SERVICE_ID, callId), 2);
        assertTrue(bsm.operatorSubmittedResult(SERVICE_ID, callId, operator2));

        // Verify both results tracked
        (uint8 jobIdx, bytes memory gotInputs, bytes memory gotOutputs, uint256 resultCount) =
            bsm.getJobDetails(SERVICE_ID, callId);

        assertEq(jobIdx, verifiedJobIndex);
        assertEq(abi.decode(gotInputs, (uint64)), 7);
        assertEq(abi.decode(gotOutputs, (uint64)), 49);
        assertEq(resultCount, 2);
    }

    function test_ConsensusSquareJob_RequiresThreeResults() public {
        _initializeBlueprint();
        _setupActiveServiceWithThreeOperators();

        // Submit a consensus_square job (index 2)
        uint8 consensusJobIndex = 2;
        uint64 callId = 200;
        bytes memory inputs = abi.encode(uint64(5));
        bytes memory outputs = abi.encode(uint64(25));

        bsm.onJobCall(SERVICE_ID, consensusJobIndex, callId, inputs);

        // All three operators submit
        address operator3 = makeAddr("operator3");
        bsm.onJobResult(SERVICE_ID, consensusJobIndex, callId, operator1, inputs, outputs);
        bsm.onJobResult(SERVICE_ID, consensusJobIndex, callId, operator2, inputs, outputs);
        bsm.onJobResult(SERVICE_ID, consensusJobIndex, callId, operator3, inputs, outputs);

        // Verify three results tracked
        (,, , uint256 resultCount) = bsm.getJobDetails(SERVICE_ID, callId);
        assertEq(resultCount, 3);
    }

    function test_AggregationRequirementsPerService() public {
        _initializeBlueprint();
        _setupActiveService();

        // Requirements are consistent regardless of service ID
        assertEq(bsm.getRequiredResultCount(1, 0), 1);
        assertEq(bsm.getRequiredResultCount(1, 1), 2);
        assertEq(bsm.getRequiredResultCount(1, 2), 3);

        assertEq(bsm.getRequiredResultCount(999, 0), 1);
        assertEq(bsm.getRequiredResultCount(999, 1), 2);
        assertEq(bsm.getRequiredResultCount(999, 2), 3);
    }

    function test_FullAggregationFlow() public {
        // This test demonstrates the complete flow for all three job types

        // 1. Setup
        bsm.onBlueprintCreated(BLUEPRINT_ID, blueprintOwner, tangleCore);
        address operator3 = makeAddr("operator3");

        // Register 3 operators
        bsm.onRegister(operator1, "");
        bsm.onRegister(operator2, "");
        bsm.onRegister(operator3, "");

        // Create and activate service
        address[] memory operators = new address[](3);
        operators[0] = operator1;
        operators[1] = operator2;
        operators[2] = operator3;
        bsm.onRequest(REQUEST_ID, serviceRequester, operators, "", 3600, address(0), 0);
        bsm.onApprove(operator1, REQUEST_ID, 50);
        bsm.onApprove(operator2, REQUEST_ID, 50);
        bsm.onApprove(operator3, REQUEST_ID, 50);
        address[] memory callers = new address[](1);
        callers[0] = serviceRequester;
        bsm.onServiceInitialized(BLUEPRINT_ID, REQUEST_ID, SERVICE_ID, serviceRequester, callers, 3600);

        // 2. Job 0: Basic square - completes with 1 result
        uint64 callId0 = 1;
        bytes memory inputs = abi.encode(uint64(4));
        bytes memory outputs = abi.encode(uint64(16));

        bsm.onJobCall(SERVICE_ID, 0, callId0, inputs);
        bsm.onJobResult(SERVICE_ID, 0, callId0, operator1, inputs, outputs);

        assertEq(bsm.getRequiredResultCount(SERVICE_ID, 0), 1);
        assertEq(bsm.jobResultCount(SERVICE_ID, callId0), 1);
        console.log("Job 0 (square): 1 result submitted, 1 required - COMPLETE");

        // 3. Job 1: Verified square - needs 2 results
        uint64 callId1 = 2;
        inputs = abi.encode(uint64(5));
        outputs = abi.encode(uint64(25));

        bsm.onJobCall(SERVICE_ID, 1, callId1, inputs);
        bsm.onJobResult(SERVICE_ID, 1, callId1, operator1, inputs, outputs);
        assertEq(bsm.jobResultCount(SERVICE_ID, callId1), 1);
        console.log("Job 1 (verified): 1 result submitted, 2 required - PENDING");

        bsm.onJobResult(SERVICE_ID, 1, callId1, operator2, inputs, outputs);
        assertEq(bsm.jobResultCount(SERVICE_ID, callId1), 2);
        console.log("Job 1 (verified): 2 results submitted, 2 required - COMPLETE");

        // 4. Job 2: Consensus square - needs 3 results
        uint64 callId2 = 3;
        inputs = abi.encode(uint64(6));
        outputs = abi.encode(uint64(36));

        bsm.onJobCall(SERVICE_ID, 2, callId2, inputs);
        bsm.onJobResult(SERVICE_ID, 2, callId2, operator1, inputs, outputs);
        console.log("Job 2 (consensus): 1 result submitted, 3 required - PENDING");

        bsm.onJobResult(SERVICE_ID, 2, callId2, operator2, inputs, outputs);
        console.log("Job 2 (consensus): 2 results submitted, 3 required - PENDING");

        bsm.onJobResult(SERVICE_ID, 2, callId2, operator3, inputs, outputs);
        assertEq(bsm.jobResultCount(SERVICE_ID, callId2), 3);
        console.log("Job 2 (consensus): 3 results submitted, 3 required - COMPLETE");

        // 5. Verify final stats
        IncredibleSquaringBSM.HookCounters memory counters = bsm.getCounters();
        assertEq(counters.jobCall, 3, "Should have 3 job calls");
        assertEq(counters.jobResult, 6, "Should have 6 job results (1+2+3)");

        console.log("Full aggregation flow completed successfully!");
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // HELPER FUNCTIONS
    // ═══════════════════════════════════════════════════════════════════════════

    function _initializeBlueprint() internal {
        bsm.onBlueprintCreated(BLUEPRINT_ID, blueprintOwner, tangleCore);
    }

    function _createServiceRequest() internal {
        address[] memory operators = new address[](1);
        operators[0] = operator1;
        bsm.onRequest(REQUEST_ID, serviceRequester, operators, "", 1000, address(0), 0);
    }

    function _approveService() internal {
        bsm.onApprove(operator1, REQUEST_ID, 50);
    }

    function _activateService() internal {
        address[] memory callers = new address[](1);
        callers[0] = serviceRequester;
        bsm.onServiceInitialized(BLUEPRINT_ID, REQUEST_ID, SERVICE_ID, serviceRequester, callers, 1000);
    }

    function _setupActiveService() internal {
        bsm.onRegister(operator1, "");
        _createServiceRequest();
        _approveService();
        _activateService();
    }

    function _setupActiveServiceWithMultipleOperators() internal {
        bsm.onRegister(operator1, "");
        bsm.onRegister(operator2, "");

        address[] memory operators = new address[](2);
        operators[0] = operator1;
        operators[1] = operator2;
        bsm.onRequest(REQUEST_ID, serviceRequester, operators, "", 1000, address(0), 0);

        bsm.onApprove(operator1, REQUEST_ID, 50);
        bsm.onApprove(operator2, REQUEST_ID, 50);

        address[] memory callers = new address[](1);
        callers[0] = serviceRequester;
        bsm.onServiceInitialized(BLUEPRINT_ID, REQUEST_ID, SERVICE_ID, serviceRequester, callers, 1000);
    }

    function _setupActiveServiceWithThreeOperators() internal {
        address operator3 = makeAddr("operator3");

        bsm.onRegister(operator1, "");
        bsm.onRegister(operator2, "");
        bsm.onRegister(operator3, "");

        address[] memory operators = new address[](3);
        operators[0] = operator1;
        operators[1] = operator2;
        operators[2] = operator3;
        bsm.onRequest(REQUEST_ID, serviceRequester, operators, "", 1000, address(0), 0);

        bsm.onApprove(operator1, REQUEST_ID, 50);
        bsm.onApprove(operator2, REQUEST_ID, 50);
        bsm.onApprove(operator3, REQUEST_ID, 50);

        address[] memory callers = new address[](1);
        callers[0] = serviceRequester;
        bsm.onServiceInitialized(BLUEPRINT_ID, REQUEST_ID, SERVICE_ID, serviceRequester, callers, 1000);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // ACCESS CONTROL TESTS - CRITICAL SECURITY
    // ═══════════════════════════════════════════════════════════════════════════

    function test_AccessControl_OnRegister_RejectsUnauthorized() public {
        _initializeBlueprint();

        address attacker = makeAddr("attacker");
        vm.prank(attacker);
        vm.expectRevert(abi.encodeWithSelector(
            BlueprintServiceManagerBase.OnlyTangleAllowed.selector,
            attacker,
            tangleCore
        ));
        bsm.onRegister(operator1, "");
    }

    function test_AccessControl_OnUnregister_RejectsUnauthorized() public {
        _initializeBlueprint();
        bsm.onRegister(operator1, "");

        address attacker = makeAddr("attacker");
        vm.prank(attacker);
        vm.expectRevert(abi.encodeWithSelector(
            BlueprintServiceManagerBase.OnlyTangleAllowed.selector,
            attacker,
            tangleCore
        ));
        bsm.onUnregister(operator1);
    }

    function test_AccessControl_OnRequest_RejectsUnauthorized() public {
        _initializeBlueprint();

        address attacker = makeAddr("attacker");
        address[] memory operators = new address[](1);
        operators[0] = operator1;

        vm.prank(attacker);
        vm.expectRevert(abi.encodeWithSelector(
            BlueprintServiceManagerBase.OnlyTangleAllowed.selector,
            attacker,
            tangleCore
        ));
        bsm.onRequest(REQUEST_ID, serviceRequester, operators, "", 1000, address(0), 0);
    }

    function test_AccessControl_OnApprove_RejectsUnauthorized() public {
        _initializeBlueprint();
        _createServiceRequest();

        address attacker = makeAddr("attacker");
        vm.prank(attacker);
        vm.expectRevert(abi.encodeWithSelector(
            BlueprintServiceManagerBase.OnlyTangleAllowed.selector,
            attacker,
            tangleCore
        ));
        bsm.onApprove(operator1, REQUEST_ID, 50);
    }

    function test_AccessControl_OnReject_RejectsUnauthorized() public {
        _initializeBlueprint();
        _createServiceRequest();

        address attacker = makeAddr("attacker");
        vm.prank(attacker);
        vm.expectRevert(abi.encodeWithSelector(
            BlueprintServiceManagerBase.OnlyTangleAllowed.selector,
            attacker,
            tangleCore
        ));
        bsm.onReject(operator1, REQUEST_ID);
    }

    function test_AccessControl_OnServiceInitialized_RejectsUnauthorized() public {
        _initializeBlueprint();
        _createServiceRequest();
        _approveService();

        address attacker = makeAddr("attacker");
        address[] memory callers = new address[](1);
        callers[0] = serviceRequester;

        vm.prank(attacker);
        vm.expectRevert(abi.encodeWithSelector(
            BlueprintServiceManagerBase.OnlyTangleAllowed.selector,
            attacker,
            tangleCore
        ));
        bsm.onServiceInitialized(BLUEPRINT_ID, REQUEST_ID, SERVICE_ID, serviceRequester, callers, 1000);
    }

    function test_AccessControl_OnJobCall_RejectsUnauthorized() public {
        _initializeBlueprint();
        _setupActiveService();

        address attacker = makeAddr("attacker");
        bytes memory inputs = abi.encode(uint64(5));

        vm.prank(attacker);
        vm.expectRevert(abi.encodeWithSelector(
            BlueprintServiceManagerBase.OnlyTangleAllowed.selector,
            attacker,
            tangleCore
        ));
        bsm.onJobCall(SERVICE_ID, JOB_INDEX, CALL_ID, inputs);
    }

    function test_AccessControl_OnJobResult_RejectsUnauthorized() public {
        _initializeBlueprint();
        _setupActiveService();
        bytes memory inputs = abi.encode(uint64(5));
        bsm.onJobCall(SERVICE_ID, JOB_INDEX, CALL_ID, inputs);

        address attacker = makeAddr("attacker");
        bytes memory outputs = abi.encode(uint64(25));

        vm.prank(attacker);
        vm.expectRevert(abi.encodeWithSelector(
            BlueprintServiceManagerBase.OnlyTangleAllowed.selector,
            attacker,
            tangleCore
        ));
        bsm.onJobResult(SERVICE_ID, JOB_INDEX, CALL_ID, operator1, inputs, outputs);
    }

    function test_AccessControl_OnOperatorJoined_RejectsUnauthorized() public {
        _initializeBlueprint();
        _setupActiveService();

        address attacker = makeAddr("attacker");
        vm.prank(attacker);
        vm.expectRevert(abi.encodeWithSelector(
            BlueprintServiceManagerBase.OnlyTangleAllowed.selector,
            attacker,
            tangleCore
        ));
        bsm.onOperatorJoined(SERVICE_ID, operator1, 5000);
    }

    function test_AccessControl_OnOperatorLeft_RejectsUnauthorized() public {
        _initializeBlueprint();
        _setupActiveService();
        bsm.onOperatorJoined(SERVICE_ID, operator1, 5000);

        address attacker = makeAddr("attacker");
        vm.prank(attacker);
        vm.expectRevert(abi.encodeWithSelector(
            BlueprintServiceManagerBase.OnlyTangleAllowed.selector,
            attacker,
            tangleCore
        ));
        bsm.onOperatorLeft(SERVICE_ID, operator1);
    }

    function test_AccessControl_OnServiceTermination_RejectsUnauthorized() public {
        _initializeBlueprint();
        _setupActiveService();

        address attacker = makeAddr("attacker");
        vm.prank(attacker);
        vm.expectRevert(abi.encodeWithSelector(
            BlueprintServiceManagerBase.OnlyTangleAllowed.selector,
            attacker,
            tangleCore
        ));
        bsm.onServiceTermination(SERVICE_ID, serviceRequester);
    }

    function test_AccessControl_OnUnappliedSlash_RejectsUnauthorized() public {
        _initializeBlueprint();
        _setupActiveService();

        address attacker = makeAddr("attacker");
        bytes memory offender = abi.encodePacked(operator1);

        vm.prank(attacker);
        vm.expectRevert(abi.encodeWithSelector(
            BlueprintServiceManagerBase.OnlyTangleAllowed.selector,
            attacker,
            tangleCore
        ));
        bsm.onUnappliedSlash(SERVICE_ID, offender, 10);
    }

    function test_AccessControl_OnSlash_RejectsUnauthorized() public {
        _initializeBlueprint();
        _setupActiveService();

        address attacker = makeAddr("attacker");
        bytes memory offender = abi.encodePacked(operator1);

        vm.prank(attacker);
        vm.expectRevert(abi.encodeWithSelector(
            BlueprintServiceManagerBase.OnlyTangleAllowed.selector,
            attacker,
            tangleCore
        ));
        bsm.onSlash(SERVICE_ID, offender, 10);
    }

    function test_AccessControl_OnUpdatePreferences_RejectsUnauthorized() public {
        _initializeBlueprint();
        bsm.onRegister(operator1, "initial");

        address attacker = makeAddr("attacker");
        vm.prank(attacker);
        vm.expectRevert(abi.encodeWithSelector(
            BlueprintServiceManagerBase.OnlyTangleAllowed.selector,
            attacker,
            tangleCore
        ));
        bsm.onUpdatePreferences(operator1, "malicious prefs");
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // DUPLICATE SUBMISSION TESTS
    // ═══════════════════════════════════════════════════════════════════════════

    function test_DuplicateSubmission_SameOperatorSubmitsTwice() public {
        _initializeBlueprint();
        _setupActiveService();

        bytes memory inputs = abi.encode(uint64(5));
        bytes memory outputs = abi.encode(uint64(25));

        bsm.onJobCall(SERVICE_ID, JOB_INDEX, CALL_ID, inputs);

        // Operator 1 submits first time
        bsm.onJobResult(SERVICE_ID, JOB_INDEX, CALL_ID, operator1, inputs, outputs);
        assertTrue(bsm.operatorSubmittedResult(SERVICE_ID, CALL_ID, operator1));
        assertEq(bsm.jobResultCount(SERVICE_ID, CALL_ID), 1);

        // Operator 1 tries to submit again (currently allowed - tracks count)
        // This documents current behavior - may want to add protection
        bsm.onJobResult(SERVICE_ID, JOB_INDEX, CALL_ID, operator1, inputs, outputs);
        assertEq(bsm.jobResultCount(SERVICE_ID, CALL_ID), 2);

        console.log("WARNING: Duplicate submissions currently allowed!");
        console.log("Job result count incremented to:", bsm.jobResultCount(SERVICE_ID, CALL_ID));
    }

    function test_DuplicateSubmission_TrackingStaysTrue() public {
        _initializeBlueprint();
        _setupActiveService();

        bytes memory inputs = abi.encode(uint64(5));
        bytes memory outputs = abi.encode(uint64(25));

        bsm.onJobCall(SERVICE_ID, JOB_INDEX, CALL_ID, inputs);
        bsm.onJobResult(SERVICE_ID, JOB_INDEX, CALL_ID, operator1, inputs, outputs);

        // After first submission, tracking is true
        assertTrue(bsm.operatorSubmittedResult(SERVICE_ID, CALL_ID, operator1));

        // Submit again
        bsm.onJobResult(SERVICE_ID, JOB_INDEX, CALL_ID, operator1, inputs, outputs);

        // Tracking should still be true (not reset or changed)
        assertTrue(bsm.operatorSubmittedResult(SERVICE_ID, CALL_ID, operator1));
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // THRESHOLD ENFORCEMENT TESTS
    // ═══════════════════════════════════════════════════════════════════════════

    function test_Threshold_JobCompletionCheck_BasicSquare() public {
        _initializeBlueprint();
        _setupActiveService();

        uint8 jobIndex = 0; // Basic square, requires 1
        uint64 callId = 100;
        bytes memory inputs = abi.encode(uint64(5));
        bytes memory outputs = abi.encode(uint64(25));

        bsm.onJobCall(SERVICE_ID, jobIndex, callId, inputs);

        // Check threshold requirement
        uint32 required = bsm.getRequiredResultCount(SERVICE_ID, jobIndex);
        assertEq(required, 1);

        // No results yet - not complete
        assertEq(bsm.jobResultCount(SERVICE_ID, callId), 0);
        assertTrue(bsm.jobResultCount(SERVICE_ID, callId) < required);

        // Submit 1 result - should be complete
        bsm.onJobResult(SERVICE_ID, jobIndex, callId, operator1, inputs, outputs);
        assertEq(bsm.jobResultCount(SERVICE_ID, callId), 1);
        assertTrue(bsm.jobResultCount(SERVICE_ID, callId) >= required);

        console.log("Basic square job: threshold met with", bsm.jobResultCount(SERVICE_ID, callId), "results");
    }

    function test_Threshold_JobCompletionCheck_VerifiedSquare() public {
        _initializeBlueprint();
        _setupActiveServiceWithMultipleOperators();

        uint8 jobIndex = 1; // Verified square, requires 2
        uint64 callId = 100;
        bytes memory inputs = abi.encode(uint64(7));
        bytes memory outputs = abi.encode(uint64(49));

        bsm.onJobCall(SERVICE_ID, jobIndex, callId, inputs);

        uint32 required = bsm.getRequiredResultCount(SERVICE_ID, jobIndex);
        assertEq(required, 2);

        // 1 result - not complete
        bsm.onJobResult(SERVICE_ID, jobIndex, callId, operator1, inputs, outputs);
        assertEq(bsm.jobResultCount(SERVICE_ID, callId), 1);
        assertTrue(bsm.jobResultCount(SERVICE_ID, callId) < required, "Should not be complete with 1/2");

        // 2 results - complete
        bsm.onJobResult(SERVICE_ID, jobIndex, callId, operator2, inputs, outputs);
        assertEq(bsm.jobResultCount(SERVICE_ID, callId), 2);
        assertTrue(bsm.jobResultCount(SERVICE_ID, callId) >= required, "Should be complete with 2/2");

        console.log("Verified square job: threshold met with", bsm.jobResultCount(SERVICE_ID, callId), "results");
    }

    function test_Threshold_JobCompletionCheck_ConsensusSquare() public {
        _initializeBlueprint();
        _setupActiveServiceWithThreeOperators();

        uint8 jobIndex = 2; // Consensus square, requires 3
        uint64 callId = 100;
        bytes memory inputs = abi.encode(uint64(6));
        bytes memory outputs = abi.encode(uint64(36));
        address operator3 = makeAddr("operator3");

        bsm.onJobCall(SERVICE_ID, jobIndex, callId, inputs);

        uint32 required = bsm.getRequiredResultCount(SERVICE_ID, jobIndex);
        assertEq(required, 3);

        // 1 result - not complete
        bsm.onJobResult(SERVICE_ID, jobIndex, callId, operator1, inputs, outputs);
        assertTrue(bsm.jobResultCount(SERVICE_ID, callId) < required, "1/3 - not complete");

        // 2 results - not complete
        bsm.onJobResult(SERVICE_ID, jobIndex, callId, operator2, inputs, outputs);
        assertTrue(bsm.jobResultCount(SERVICE_ID, callId) < required, "2/3 - not complete");

        // 3 results - complete
        bsm.onJobResult(SERVICE_ID, jobIndex, callId, operator3, inputs, outputs);
        assertTrue(bsm.jobResultCount(SERVICE_ID, callId) >= required, "3/3 - complete");

        console.log("Consensus square job: threshold met with", bsm.jobResultCount(SERVICE_ID, callId), "results");
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // EDGE CASE TESTS
    // ═══════════════════════════════════════════════════════════════════════════

    function test_EdgeCase_SubmitToNonExistentService() public {
        _initializeBlueprint();
        _setupActiveService();

        uint64 nonExistentServiceId = 999;
        bytes memory inputs = abi.encode(uint64(5));
        bytes memory outputs = abi.encode(uint64(25));

        // This currently doesn't revert - it just creates entries
        // Documenting current behavior
        bsm.onJobCall(nonExistentServiceId, JOB_INDEX, CALL_ID, inputs);
        bsm.onJobResult(nonExistentServiceId, JOB_INDEX, CALL_ID, operator1, inputs, outputs);

        // Verify data was stored (even for non-existent service)
        assertEq(bsm.jobResultCount(nonExistentServiceId, CALL_ID), 1);

        console.log("NOTE: Submitting to non-existent service succeeds - no validation");
    }

    function test_EdgeCase_SubmitWithoutJobCall() public {
        _initializeBlueprint();
        _setupActiveService();

        bytes memory inputs = abi.encode(uint64(5));
        bytes memory outputs = abi.encode(uint64(25));
        uint64 uncalledCallId = 999;

        // Submit result without prior job call
        bsm.onJobResult(SERVICE_ID, JOB_INDEX, uncalledCallId, operator1, inputs, outputs);

        // Currently allowed - documenting behavior
        assertEq(bsm.jobResultCount(SERVICE_ID, uncalledCallId), 1);

        console.log("NOTE: Submitting result without prior job call succeeds");
    }

    function test_EdgeCase_MultipleJobsPerService() public {
        _initializeBlueprint();
        _setupActiveServiceWithThreeOperators();

        // Submit multiple jobs concurrently
        for (uint64 callId = 1; callId <= 5; callId++) {
            bytes memory inputs = abi.encode(uint64(callId));
            bytes memory outputs = abi.encode(uint64(callId * callId));

            bsm.onJobCall(SERVICE_ID, JOB_INDEX, callId, inputs);
            bsm.onJobResult(SERVICE_ID, JOB_INDEX, callId, operator1, inputs, outputs);
        }

        // Verify all jobs tracked independently
        for (uint64 callId = 1; callId <= 5; callId++) {
            assertEq(bsm.jobResultCount(SERVICE_ID, callId), 1);
            assertTrue(bsm.operatorSubmittedResult(SERVICE_ID, callId, operator1));
        }

        assertEq(bsm.serviceJobCallCount(SERVICE_ID), 5);
        assertEq(bsm.serviceJobResultCount(SERVICE_ID), 5);
    }

    function test_EdgeCase_DifferentJobTypesInSequence() public {
        _initializeBlueprint();
        _setupActiveServiceWithThreeOperators();

        address operator3 = makeAddr("operator3");

        // Job 0 - Basic square (1 result needed)
        uint64 callId0 = 1;
        bsm.onJobCall(SERVICE_ID, 0, callId0, abi.encode(uint64(2)));
        bsm.onJobResult(SERVICE_ID, 0, callId0, operator1, "", abi.encode(uint64(4)));
        assertEq(bsm.jobIndices(SERVICE_ID, callId0), 0);

        // Job 1 - Verified square (2 results needed)
        uint64 callId1 = 2;
        bsm.onJobCall(SERVICE_ID, 1, callId1, abi.encode(uint64(3)));
        bsm.onJobResult(SERVICE_ID, 1, callId1, operator1, "", abi.encode(uint64(9)));
        bsm.onJobResult(SERVICE_ID, 1, callId1, operator2, "", abi.encode(uint64(9)));
        assertEq(bsm.jobIndices(SERVICE_ID, callId1), 1);

        // Job 2 - Consensus square (3 results needed)
        uint64 callId2 = 3;
        bsm.onJobCall(SERVICE_ID, 2, callId2, abi.encode(uint64(4)));
        bsm.onJobResult(SERVICE_ID, 2, callId2, operator1, "", abi.encode(uint64(16)));
        bsm.onJobResult(SERVICE_ID, 2, callId2, operator2, "", abi.encode(uint64(16)));
        bsm.onJobResult(SERVICE_ID, 2, callId2, operator3, "", abi.encode(uint64(16)));
        assertEq(bsm.jobIndices(SERVICE_ID, callId2), 2);

        // Verify correct job indices stored
        assertEq(bsm.getRequiredResultCount(SERVICE_ID, 0), 1);
        assertEq(bsm.getRequiredResultCount(SERVICE_ID, 1), 2);
        assertEq(bsm.getRequiredResultCount(SERVICE_ID, 2), 3);

        // Verify result counts match requirements
        assertTrue(bsm.jobResultCount(SERVICE_ID, callId0) >= bsm.getRequiredResultCount(SERVICE_ID, 0));
        assertTrue(bsm.jobResultCount(SERVICE_ID, callId1) >= bsm.getRequiredResultCount(SERVICE_ID, 1));
        assertTrue(bsm.jobResultCount(SERVICE_ID, callId2) >= bsm.getRequiredResultCount(SERVICE_ID, 2));
    }

    function test_EdgeCase_VerifyIsJobComplete() public view {
        // Helper function concept - check if job is complete
        // This shows how an external system would check completion

        uint64 serviceId = 1;
        uint8 jobIndex = 1; // verified_square
        uint32 required = bsm.getRequiredResultCount(serviceId, jobIndex);

        // In a real scenario, you'd check:
        // bool isComplete = bsm.jobResultCount(serviceId, callId) >= required;

        assertEq(required, 2);
        console.log("To check job completion: jobResultCount >= getRequiredResultCount");
    }
}
