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
        address indexed user,
        string tier,
        uint256 amount,
        uint256 timestamp,
        uint64 indexed serviceId,
        uint64 indexed callId
    );

    event ResourceWritten(
        address indexed user,
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
    
    // User subscription data
    mapping(address => string) public userTiers;
    mapping(address => uint256) public subscriptionExpiry;
    
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
            // Decode subscription tier from inputs
            // inputs[0] should be the tier string
            require(inputs.length > 0, "Missing tier input");
            string memory tier = abi.decode(inputs, (string));
            
            require(msg.value > 0, "Payment required for API key purchase");
            require(tierPricing[tier] > 0, "Invalid subscription tier");
            require(msg.value >= tierPricing[tier], "Insufficient payment");

            // Update user subscription immediately on job call
            userTiers[tx.origin] = tier; // tx.origin is the original caller
            subscriptionExpiry[tx.origin] = block.timestamp + 30 days;

            emit ApiKeyPurchased(
                tx.origin,
                tier,
                msg.value,
                block.timestamp,
                serviceId,
                jobCallId
            );

            // Refund excess payment
            if (msg.value > tierPricing[tier]) {
                payable(tx.origin).transfer(msg.value - tierPricing[tier]);
            }
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
        bytes calldata outputs
    ) external payable override onlyFromMaster {
        if (job == WRITE_RESOURCE_JOB_ID) {
            // Decode resource data from inputs
            // inputs should contain (resource_id, data) tuple
            (string memory resourceId, string memory data) = abi.decode(inputs, (string, string));
            
            // Decode tenant hash from outputs (set by AuthContext)
            // outputs should contain the job result with tenant info
            bytes32 tenantHash = keccak256(abi.encodePacked("tenant_", operator.ecdsaKey));

            // Track resource creation per tenant
            if (!tenantResources[tenantHash][resourceId]) {
                tenantResources[tenantHash][resourceId] = true;
                tenantResourceCount[tenantHash]++;
            }

            emit ResourceWritten(
                tx.origin,
                resourceId,
                tenantHash,
                serviceId,
                jobCallId
            );
        }
        
        // For other jobs (WHOAMI, ECHO), we can log or track usage but no state changes needed
    }

    /**
     * @dev Check if user has active subscription
     */
    function hasActiveSubscription(address user) external view returns (bool) {
        return subscriptionExpiry[user] > block.timestamp;
    }

    /**
     * @dev Get user's current tier
     */
    function getUserTier(address user) external view returns (string memory) {
        require(this.hasActiveSubscription(user), "No active subscription");
        return userTiers[user];
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
}