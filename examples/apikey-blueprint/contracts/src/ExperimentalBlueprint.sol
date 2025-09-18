// SPDX-License-Identifier: UNLICENSE
pragma solidity >=0.8.13;

import "tnt-core/BlueprintServiceManagerBase.sol";

/**
 * @title ExperimentalBlueprint
 * @dev API Key Blueprint that manages subscription tiers and usage tracking
 * @dev Follows BlueprintServiceManagerBase spec for automatic job-to-hook wiring
 */
contract ExperimentalBlueprint is BlueprintServiceManagerBase {
    // Events for job execution tracking
    event ApiKeyPurchased(
        bytes32 indexed accountHash,
        string accountId,
        string tier,
        uint256 amount,
        uint256 timestamp,
        uint64 indexed serviceId,
        uint64 indexed callId
    );

    event ResourceWritten(
        bytes32 indexed accountHash,
        string accountId,
        string resourceId,
        bytes32 indexed tenantHash,
        uint64 indexed serviceId,
        uint64 indexed callId
    );

    // Job IDs that correspond to Rust job constants
    uint8 public constant WHOAMI_JOB_ID = 0;
    uint8 public constant WRITE_RESOURCE_JOB_ID = 1;
    uint8 public constant PURCHASE_API_KEY_JOB_ID = 2;
    uint8 public constant ECHO_JOB_ID = 3;

    // Subscription tier pricing (in wei)
    mapping(string => uint256) public tierPricing;

    // User subscription data keyed by a stable hash of the off-chain account identifier
    mapping(bytes32 => string) private userTiers;
    mapping(bytes32 => uint256) private subscriptionExpiry;
    mapping(bytes32 => string) private accountLabels;

    // Resource tracking per tenant
    mapping(bytes32 => mapping(string => bool)) public tenantResources;
    mapping(bytes32 => uint256) public tenantResourceCount;

    // Job call tracking
    mapping(uint64 => mapping(uint64 => bool)) public processedJobCalls;

    constructor() {
        // Initialize tier pricing
        tierPricing["basic"] = 0.01 ether;
        tierPricing["premium"] = 0.05 ether;
        tierPricing["enterprise"] = 0.1 ether;
    }

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

        if (job == PURCHASE_API_KEY_JOB_ID) {
            // Decode subscription request: (tier, account identifier)
            (string memory tier, string memory accountId) = abi.decode(inputs, (string, string));
            require(bytes(tier).length != 0, "Missing tier input");
            require(bytes(accountId).length != 0, "Missing account input");
            require(tierPricing[tier] > 0, "Invalid subscription tier");
            require(msg.value == tierPricing[tier], "Incorrect payment");

            bytes32 accountHash = _accountKey(accountId);

            // Update user subscription immediately on job call
            userTiers[accountHash] = tier;
            subscriptionExpiry[accountHash] = block.timestamp + 30 days;
            accountLabels[accountHash] = accountId;

            emit ApiKeyPurchased(
                accountHash,
                accountId,
                tier,
                msg.value,
                block.timestamp,
                serviceId,
                jobCallId
            );
        }
    }

    /**
     * @dev Hook called automatically when job execution completes on Rust side
     * @dev This receives the job results and can update contract state accordingly
     */
    function onJobResult(
        uint64 serviceId,
        uint8 job,
        uint64 jobCallId,
        ServiceOperators.OperatorPreferences calldata operator,
        bytes calldata inputs,
        bytes calldata /* outputs */
    ) external payable override onlyFromMaster {
        if (job == WRITE_RESOURCE_JOB_ID) {
            // Decode resource data from inputs
            (string memory resourceId, string memory data, string memory accountId) =
                abi.decode(inputs, (string, string, string));
            require(bytes(accountId).length != 0, "Missing account input");

            bytes32 tenantHash = keccak256(abi.encodePacked("tenant_", operator.ecdsaKey));
            bytes32 accountHash = _accountKey(accountId);

            // Track resource creation per tenant and account
            if (!tenantResources[tenantHash][resourceId]) {
                tenantResources[tenantHash][resourceId] = true;
                tenantResourceCount[tenantHash]++;
            }

            emit ResourceWritten(
                accountHash,
                accountId,
                resourceId,
                tenantHash,
                serviceId,
                jobCallId
            );

            // Silence unused variable warning in example
            data; // solhint-disable-line no-unused-vars
        }
        
        // For other jobs (WHOAMI, ECHO), we can log or track usage but no state changes needed
    }

    /**
     * @dev Check if user has active subscription
     */
    function hasActiveSubscription(bytes32 accountHash) external view returns (bool) {
        return subscriptionExpiry[accountHash] > block.timestamp;
    }

    /**
     * @dev Get user's current tier
     */
    function getUserTier(bytes32 accountHash) external view returns (string memory) {
        require(subscriptionExpiry[accountHash] > block.timestamp, "No active subscription");
        return userTiers[accountHash];
    }

    /**
     * @dev Return the canonical label stored for an account hash
     */
    function getAccountLabel(bytes32 accountHash) external view returns (string memory) {
        return accountLabels[accountHash];
    }

    /**
     * @dev Get tenant resource count
     */
    function getTenantResourceCount(bytes32 tenantHash) external view returns (uint256) {
        return tenantResourceCount[tenantHash];
    }

    /**
     * @dev Check if tenant has specific resource
     */
    function tenantHasResource(bytes32 tenantHash, string memory resourceId) external view returns (bool) {
        return tenantResources[tenantHash][resourceId];
    }

    /**
     * @dev Check if job call was processed
     */
    function wasJobCallProcessed(uint64 serviceId, uint64 jobCallId) external view returns (bool) {
        return processedJobCalls[serviceId][jobCallId];
    }

    /**
     * @dev Get tier pricing
     */
    function getTierPricing(string memory tier) external view returns (uint256) {
        return tierPricing[tier];
    }

    /**
     * @dev Emergency withdraw function (only for testing)
     */
    function withdraw() external {
        require(msg.sender == blueprintOwner, "Only owner can withdraw");
        payable(msg.sender).transfer(address(this).balance);
    }

    function _accountKey(string memory accountId) private pure returns (bytes32) {
        return keccak256(bytes(accountId));
    }
}
