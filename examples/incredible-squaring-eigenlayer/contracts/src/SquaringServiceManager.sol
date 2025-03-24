// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "@eigenlayer/contracts/libraries/BytesLib.sol";
import "./TaskManager.sol";
import "@eigenlayer-middleware/src/ServiceManagerBase.sol";

import {IRewardsCoordinator} from "@eigenlayer/contracts/interfaces/IRewardsCoordinator.sol";

/**
 * @title Primary entrypoint for procuring services from IncredibleSquaring.
 */
contract SquaringServiceManager is ServiceManagerBase {
    using BytesLib for bytes;

    TaskManager public immutable squaringTaskManager;

    /// @notice when applied to a function, ensures that the function is only callable by the `registryCoordinator`.
    modifier onlySquaringTaskManager() {
        require(
            msg.sender == address(squaringTaskManager),
            "onlySquaringTaskManager: not from credible squaring task manager"
        );
        _;
    }

    constructor(
        IAVSDirectory _avsDirectory,
        IRegistryCoordinator _registryCoordinator,
        IStakeRegistry _stakeRegistry,
        address rewardsCoordinator,
        TaskManager _squaringTaskManager
    )
        ServiceManagerBase(_avsDirectory, IRewardsCoordinator(rewardsCoordinator), _registryCoordinator, _stakeRegistry)
    {
        squaringTaskManager = _squaringTaskManager;
    }

    /// @notice This function initializes the contract's owner and the rewards initiator.
    /// @param initialOwner The address to be set as the initial owner of the contract.
    /// @param rewardsInitiator The address to be set as the rewards initiator for the contract.
    function initialize(address initialOwner, address rewardsInitiator) external initializer {
        __ServiceManagerBase_init(initialOwner, rewardsInitiator);
    }

    /// @notice Called in the event of challenge resolution, in order to forward a call to the Slasher, which 'freezes' the `operator`.
    /// @dev The Slasher contract is under active development and its interface expected to change.
    ///      We recommend writing slashing logic without integrating with the Slasher at this point in time.
    function freezeOperator(address operatorAddr) external onlySquaringTaskManager {
        // slasher.freezeOperator(operatorAddr);
    }
}