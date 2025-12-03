//! Tangle v2 Contract ABIs and Bindings
//!
//! This module defines the Solidity interfaces for the Tangle v2 EVM contracts
//! using the `alloy-sol-types` `sol!` macro.

#![allow(missing_docs)]

use alloy_sol_types::sol;

// ═══════════════════════════════════════════════════════════════════════════════
// TANGLE CORE CONTRACT
// ═══════════════════════════════════════════════════════════════════════════════

sol! {
    /// Tangle v2 Core Contract Interface
    #[sol(rpc)]
    interface ITangle {
        // ═══════════════════════════════════════════════════════════════════════
        // TYPES
        // ═══════════════════════════════════════════════════════════════════════

        /// Blueprint selection mode for delegations
        enum BlueprintSelectionMode {
            All,
            Fixed
        }

        /// Membership model for services
        enum MembershipModel {
            Fixed,
            Dynamic
        }

        /// Pricing model for services
        enum PricingModel {
            PayOnce,
            Subscription,
            EventDriven
        }

        /// Service status
        enum ServiceStatus {
            Pending,
            Active,
            Terminated
        }

        // ═══════════════════════════════════════════════════════════════════════
        // EVENTS - Blueprints
        // ═══════════════════════════════════════════════════════════════════════

        event BlueprintCreated(uint64 indexed blueprintId, address indexed owner, address manager);
        event BlueprintUpdated(uint64 indexed blueprintId, string metadataUri);
        event BlueprintTransferred(uint64 indexed blueprintId, address indexed from, address indexed to);
        event BlueprintDeactivated(uint64 indexed blueprintId);

        // ═══════════════════════════════════════════════════════════════════════
        // EVENTS - Operators
        // ═══════════════════════════════════════════════════════════════════════

        event OperatorPreRegistered(uint64 indexed blueprintId, address indexed operator);
        event OperatorRegistered(uint64 indexed blueprintId, address indexed operator, bytes preferences);
        event OperatorUnregistered(uint64 indexed blueprintId, address indexed operator);
        event OperatorPreferencesUpdated(uint64 indexed blueprintId, address indexed operator, bytes preferences);
        event OperatorRpcAddressUpdated(uint64 indexed blueprintId, address indexed operator, string rpcAddress);

        // ═══════════════════════════════════════════════════════════════════════
        // EVENTS - Services
        // ═══════════════════════════════════════════════════════════════════════

        event ServiceRequested(uint64 indexed requestId, uint64 indexed blueprintId, address indexed requester);
        event ServiceApproved(uint64 indexed requestId, address indexed operator);
        event ServiceRejected(uint64 indexed requestId, address indexed operator);
        event ServiceActivated(uint64 indexed serviceId, uint64 indexed requestId, uint64 indexed blueprintId);
        event ServiceTerminated(uint64 indexed serviceId);
        event OperatorJoinedService(uint64 indexed serviceId, address indexed operator, uint16 exposureBps);
        event OperatorLeftService(uint64 indexed serviceId, address indexed operator);

        // ═══════════════════════════════════════════════════════════════════════
        // EVENTS - Jobs
        // ═══════════════════════════════════════════════════════════════════════

        event JobSubmitted(uint64 indexed serviceId, uint64 indexed callId, uint8 jobIndex, address caller, bytes inputs);
        event JobResultSubmitted(uint64 indexed serviceId, uint64 indexed callId, address indexed operator, bytes output);
        event JobCompleted(uint64 indexed serviceId, uint64 indexed callId);

        // ═══════════════════════════════════════════════════════════════════════
        // EVENTS - Payments
        // ═══════════════════════════════════════════════════════════════════════

        event EscrowFunded(uint64 indexed serviceId, address indexed token, uint256 amount);
        event SubscriptionBilled(uint64 indexed serviceId, uint256 amount, uint64 period);
        event RewardsClaimed(address indexed account, address indexed token, uint256 amount);

        // ═══════════════════════════════════════════════════════════════════════
        // EVENTS - Slashing
        // ═══════════════════════════════════════════════════════════════════════

        event SlashProposed(
            uint64 indexed slashId,
            uint64 indexed serviceId,
            address indexed operator,
            uint256 amount,
            bytes32 evidence
        );
        event SlashDisputed(uint64 indexed slashId, address indexed operator);
        event SlashExecuted(uint64 indexed slashId, uint256 actualAmount);
        event SlashCancelled(uint64 indexed slashId);

        // ═══════════════════════════════════════════════════════════════════════
        // READ FUNCTIONS - Blueprints
        // ═══════════════════════════════════════════════════════════════════════

        function blueprintCount() external view returns (uint64);
        function getBlueprint(uint64 blueprintId) external view returns (
            address owner,
            address manager,
            uint64 createdAt,
            uint32 operatorCount,
            MembershipModel membership,
            PricingModel pricing,
            bool active
        );
        function getBlueprintConfig(uint64 blueprintId) external view returns (
            MembershipModel membership,
            PricingModel pricing,
            uint32 minOperators,
            uint32 maxOperators,
            uint256 subscriptionRate,
            uint64 subscriptionInterval,
            uint256 eventRate
        );
        function isOperatorRegistered(uint64 blueprintId, address operator) external view returns (bool);
        function getBlueprintOperators(uint64 blueprintId) external view returns (address[] memory);

        // ═══════════════════════════════════════════════════════════════════════
        // READ FUNCTIONS - Services
        // ═══════════════════════════════════════════════════════════════════════

        function serviceCount() external view returns (uint64);
        function getService(uint64 serviceId) external view returns (
            uint64 blueprintId,
            address owner,
            uint64 createdAt,
            uint64 ttl,
            uint64 terminatedAt,
            uint64 lastPaymentAt,
            uint32 operatorCount,
            uint32 minOperators,
            uint32 maxOperators,
            MembershipModel membership,
            PricingModel pricing,
            ServiceStatus status
        );
        function getServiceOperators(uint64 serviceId) external view returns (address[] memory);
        function isServiceOperator(uint64 serviceId, address operator) external view returns (bool);
        function isPermittedCaller(uint64 serviceId, address caller) external view returns (bool);

        // ═══════════════════════════════════════════════════════════════════════
        // READ FUNCTIONS - Jobs
        // ═══════════════════════════════════════════════════════════════════════

        function getCallInfo(uint64 serviceId, uint64 callId) external view returns (
            uint8 jobIndex,
            uint64 submittedAt,
            uint32 resultCount,
            bool completed
        );

        // ═══════════════════════════════════════════════════════════════════════
        // WRITE FUNCTIONS - Blueprints
        // ═══════════════════════════════════════════════════════════════════════

        function createBlueprint(string calldata metadataUri, address manager) external returns (uint64);

        // ═══════════════════════════════════════════════════════════════════════
        // WRITE FUNCTIONS - Operators
        // ═══════════════════════════════════════════════════════════════════════

        function preRegister(uint64 blueprintId) external;
        function registerOperator(uint64 blueprintId, bytes calldata preferences) external;
        function unregisterOperator(uint64 blueprintId) external;
        function updateOperatorPreferences(uint64 blueprintId, bytes calldata preferences) external;
        function updateRpcAddress(uint64 blueprintId, string calldata rpcAddress) external;

        // ═══════════════════════════════════════════════════════════════════════
        // WRITE FUNCTIONS - Services
        // ═══════════════════════════════════════════════════════════════════════

        function requestService(
            uint64 blueprintId,
            address[] calldata operators,
            uint64 ttl,
            bytes calldata requestArgs
        ) external payable returns (uint64);

        function approveService(uint64 requestId) external;
        function rejectService(uint64 requestId) external;
        function terminateService(uint64 serviceId) external;
        function joinService(uint64 serviceId, uint16 exposureBps) external;
        function leaveService(uint64 serviceId) external;

        // ═══════════════════════════════════════════════════════════════════════
        // WRITE FUNCTIONS - Jobs
        // ═══════════════════════════════════════════════════════════════════════

        function submitJob(uint64 serviceId, uint8 jobIndex, bytes calldata inputs) external payable returns (uint64);
        function submitResult(uint64 serviceId, uint64 callId, bytes calldata output) external;
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// MULTI-ASSET DELEGATION CONTRACT
// ═══════════════════════════════════════════════════════════════════════════════

sol! {
    /// MultiAssetDelegation Contract Interface (Restaking)
    #[sol(rpc)]
    interface IMultiAssetDelegation {
        // ═══════════════════════════════════════════════════════════════════════
        // EVENTS
        // ═══════════════════════════════════════════════════════════════════════

        event OperatorRegistered(address indexed operator, uint256 stake);
        event OperatorStakeIncreased(address indexed operator, uint256 amount);
        event OperatorUnstakeScheduled(address indexed operator, uint256 amount, uint64 readyRound);
        event OperatorUnstakeExecuted(address indexed operator, uint256 amount);
        event OperatorLeavingScheduled(address indexed operator, uint64 readyRound);
        event OperatorLeft(address indexed operator);
        event OperatorBlueprintAdded(address indexed operator, uint64 indexed blueprintId);
        event OperatorBlueprintRemoved(address indexed operator, uint64 indexed blueprintId);

        event Delegated(
            address indexed delegator,
            address indexed operator,
            address indexed token,
            uint256 amount,
            uint256 shares,
            uint8 selectionMode
        );
        event DelegatorUnstakeScheduled(
            address indexed delegator,
            address indexed operator,
            address indexed token,
            uint256 shares,
            uint256 estimatedAmount,
            uint64 readyRound
        );
        event DelegatorUnstakeExecuted(
            address indexed delegator,
            address indexed operator,
            address indexed token,
            uint256 shares,
            uint256 amount
        );

        event RewardDistributed(address indexed operator, uint256 amount);
        event RewardClaimed(address indexed account, uint256 amount);

        event Slashed(
            address indexed operator,
            uint64 indexed serviceId,
            uint256 amount,
            bytes32 evidence
        );

        // ═══════════════════════════════════════════════════════════════════════
        // READ FUNCTIONS
        // ═══════════════════════════════════════════════════════════════════════

        function isOperator(address operator) external view returns (bool);
        function isOperatorActive(address operator) external view returns (bool);
        function getOperatorStake(address operator) external view returns (uint256);
        function getOperatorSelfStake(address operator) external view returns (uint256);
        function getOperatorDelegatedStake(address operator) external view returns (uint256);
        function getDelegation(address delegator, address operator) external view returns (uint256);
        function getTotalDelegation(address delegator) external view returns (uint256);
        function minOperatorStake() external view returns (uint256);
        function meetsStakeRequirement(address operator, uint256 required) external view returns (bool);
        function isSlasher(address account) external view returns (bool);
        function currentRound() external view returns (uint64);
        function operatorCount() external view returns (uint256);
        function operatorAt(uint256 index) external view returns (address);
        function getOperatorBlueprints(address operator) external view returns (uint256[] memory);

        // ═══════════════════════════════════════════════════════════════════════
        // WRITE FUNCTIONS
        // ═══════════════════════════════════════════════════════════════════════

        function registerOperator() external payable;
        function increaseStake() external payable;
        function scheduleOperatorUnstake(uint256 amount) external;
        function executeOperatorUnstake() external;
        function addBlueprint(uint64 blueprintId) external;
        function removeBlueprint(uint64 blueprintId) external;
        function startLeaving() external;
        function completeLeaving() external;

        function deposit() external payable;
        function depositAndDelegate(address operator) external payable;
        function delegate(address operator, uint256 amount) external;
        function scheduleDelegatorUnstake(address operator, address token, uint256 amount) external;
        function executeDelegatorUnstake() external;
        function scheduleWithdraw(address token, uint256 amount) external;
        function executeWithdraw() external;

        function claimDelegatorRewards() external;
        function claimOperatorRewards() external;

        // Slashing (only for authorized slashers)
        function slash(
            address operator,
            uint64 serviceId,
            uint256 amount,
            bytes32 evidence
        ) external returns (uint256 actualSlashed);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// OPERATOR STATUS REGISTRY
// ═══════════════════════════════════════════════════════════════════════════════

sol! {
    /// Operator Status Registry Interface
    #[sol(rpc)]
    interface IOperatorStatusRegistry {
        // Events
        event HeartbeatReceived(
            address indexed operator,
            uint64 indexed serviceId,
            uint64 indexed blueprintId,
            uint8 status,
            bytes metrics
        );
        event OperatorWentOffline(address indexed operator, uint64 indexed serviceId, uint64 missedCount);
        event OperatorCameOnline(address indexed operator, uint64 indexed serviceId);

        // Read functions
        function isOperatorOnline(address operator, uint64 serviceId) external view returns (bool);
        function getLastHeartbeat(address operator, uint64 serviceId) external view returns (uint64);
        function getMissedHeartbeats(address operator, uint64 serviceId) external view returns (uint64);
        function getOperatorStatus(address operator, uint64 serviceId) external view returns (uint8);

        // Write functions
        function submitHeartbeat(
            uint64 serviceId,
            uint64 blueprintId,
            uint8 status,
            bytes calldata metrics,
            bytes calldata signature
        ) external;
    }
}

// Re-export contract modules (not glob to avoid ambiguity)
// Use ITangle::*, IMultiAssetDelegation::*, IOperatorStatusRegistry::* in consuming code
