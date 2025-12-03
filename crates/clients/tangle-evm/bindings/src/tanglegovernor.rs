///Module containing a contract's types and functions.
/**

```solidity
library IGovernor {
    type ProposalState is uint8;
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IGovernor {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ProposalState(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<ProposalState> for u8 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        #[automatically_derived]
        impl ProposalState {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from(value: u8) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into(self) -> u8 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for ProposalState {
            type RustType = u8;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ProposalState {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IGovernor`](self) contract instance.

See the [wrapper's documentation](`IGovernorInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IGovernorInstance<T, P, N> {
        IGovernorInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IGovernor`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IGovernor`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IGovernorInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IGovernorInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IGovernorInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IGovernorInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IGovernor`](self) contract instance.

See the [wrapper's documentation](`IGovernorInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> IGovernorInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IGovernorInstance<T, P, N> {
            IGovernorInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IGovernorInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IGovernorInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library IGovernor {
    type ProposalState is uint8;
}

interface TangleGovernor {
    error AddressEmptyCode(address target);
    error CheckpointUnorderedInsertion();
    error ERC1967InvalidImplementation(address implementation);
    error ERC1967NonPayable();
    error FailedCall();
    error GovernorAlreadyCastVote(address voter);
    error GovernorAlreadyQueuedProposal(uint256 proposalId);
    error GovernorDisabledDeposit();
    error GovernorInsufficientProposerVotes(address proposer, uint256 votes, uint256 threshold);
    error GovernorInvalidProposalLength(uint256 targets, uint256 calldatas, uint256 values);
    error GovernorInvalidQuorumFraction(uint256 quorumNumerator, uint256 quorumDenominator);
    error GovernorInvalidSignature(address voter);
    error GovernorInvalidVoteParams();
    error GovernorInvalidVoteType();
    error GovernorInvalidVotingPeriod(uint256 votingPeriod);
    error GovernorNonexistentProposal(uint256 proposalId);
    error GovernorNotQueuedProposal(uint256 proposalId);
    error GovernorOnlyExecutor(address account);
    error GovernorOnlyProposer(address account);
    error GovernorQueueNotImplemented();
    error GovernorRestrictedProposer(address proposer);
    error GovernorUnexpectedProposalState(uint256 proposalId, IGovernor.ProposalState current, bytes32 expectedStates);
    error InvalidAccountNonce(address account, uint256 currentNonce);
    error InvalidInitialization();
    error NotInitializing();
    error SafeCastOverflowedUintDowncast(uint8 bits, uint256 value);
    error UUPSUnauthorizedCallContext();
    error UUPSUnsupportedProxiableUUID(bytes32 slot);

    event EIP712DomainChanged();
    event Initialized(uint64 version);
    event ProposalCanceled(uint256 proposalId);
    event ProposalCreated(uint256 proposalId, address proposer, address[] targets, uint256[] values, string[] signatures, bytes[] calldatas, uint256 voteStart, uint256 voteEnd, string description);
    event ProposalExecuted(uint256 proposalId);
    event ProposalQueued(uint256 proposalId, uint256 etaSeconds);
    event ProposalThresholdSet(uint256 oldProposalThreshold, uint256 newProposalThreshold);
    event QuorumNumeratorUpdated(uint256 oldQuorumNumerator, uint256 newQuorumNumerator);
    event TimelockChange(address oldTimelock, address newTimelock);
    event Upgraded(address indexed implementation);
    event VoteCast(address indexed voter, uint256 proposalId, uint8 support, uint256 weight, string reason);
    event VoteCastWithParams(address indexed voter, uint256 proposalId, uint8 support, uint256 weight, string reason, bytes params);
    event VotingDelaySet(uint256 oldVotingDelay, uint256 newVotingDelay);
    event VotingPeriodSet(uint256 oldVotingPeriod, uint256 newVotingPeriod);

    constructor();

    receive() external payable;

    function BALLOT_TYPEHASH() external view returns (bytes32);
    function CLOCK_MODE() external view returns (string memory);
    function COUNTING_MODE() external pure returns (string memory);
    function EXTENDED_BALLOT_TYPEHASH() external view returns (bytes32);
    function UPGRADE_INTERFACE_VERSION() external view returns (string memory);
    function cancel(address[] memory targets, uint256[] memory values, bytes[] memory calldatas, bytes32 descriptionHash) external returns (uint256);
    function castVote(uint256 proposalId, uint8 support) external returns (uint256);
    function castVoteBySig(uint256 proposalId, uint8 support, address voter, bytes memory signature) external returns (uint256);
    function castVoteWithReason(uint256 proposalId, uint8 support, string memory reason) external returns (uint256);
    function castVoteWithReasonAndParams(uint256 proposalId, uint8 support, string memory reason, bytes memory params) external returns (uint256);
    function castVoteWithReasonAndParamsBySig(uint256 proposalId, uint8 support, address voter, string memory reason, bytes memory params, bytes memory signature) external returns (uint256);
    function clock() external view returns (uint48);
    function eip712Domain() external view returns (bytes1 fields, string memory name, string memory version, uint256 chainId, address verifyingContract, bytes32 salt, uint256[] memory extensions);
    function execute(address[] memory targets, uint256[] memory values, bytes[] memory calldatas, bytes32 descriptionHash) external payable returns (uint256);
    function getVotes(address account, uint256 timepoint) external view returns (uint256);
    function getVotesWithParams(address account, uint256 timepoint, bytes memory params) external view returns (uint256);
    function hasVoted(uint256 proposalId, address account) external view returns (bool);
    function hashProposal(address[] memory targets, uint256[] memory values, bytes[] memory calldatas, bytes32 descriptionHash) external pure returns (uint256);
    function initialize(address token, address timelock, uint48 initialVotingDelay, uint32 initialVotingPeriod, uint256 initialProposalThreshold, uint256 quorumPercent) external;
    function name() external view returns (string memory);
    function nonces(address owner) external view returns (uint256);
    function onERC1155BatchReceived(address, address, uint256[] memory, uint256[] memory, bytes memory) external returns (bytes4);
    function onERC1155Received(address, address, uint256, uint256, bytes memory) external returns (bytes4);
    function onERC721Received(address, address, uint256, bytes memory) external returns (bytes4);
    function proposalDeadline(uint256 proposalId) external view returns (uint256);
    function proposalEta(uint256 proposalId) external view returns (uint256);
    function proposalNeedsQueuing(uint256 proposalId) external view returns (bool);
    function proposalProposer(uint256 proposalId) external view returns (address);
    function proposalSnapshot(uint256 proposalId) external view returns (uint256);
    function proposalThreshold() external view returns (uint256);
    function proposalVotes(uint256 proposalId) external view returns (uint256 againstVotes, uint256 forVotes, uint256 abstainVotes);
    function propose(address[] memory targets, uint256[] memory values, bytes[] memory calldatas, string memory description) external returns (uint256);
    function proxiableUUID() external view returns (bytes32);
    function queue(address[] memory targets, uint256[] memory values, bytes[] memory calldatas, bytes32 descriptionHash) external returns (uint256);
    function quorum(uint256 blockNumber) external view returns (uint256);
    function quorumDenominator() external view returns (uint256);
    function quorumNumerator(uint256 timepoint) external view returns (uint256);
    function quorumNumerator() external view returns (uint256);
    function relay(address target, uint256 value, bytes memory data) external payable;
    function setProposalThreshold(uint256 newProposalThreshold) external;
    function setVotingDelay(uint48 newVotingDelay) external;
    function setVotingPeriod(uint32 newVotingPeriod) external;
    function state(uint256 proposalId) external view returns (IGovernor.ProposalState);
    function supportsInterface(bytes4 interfaceId) external view returns (bool);
    function timelock() external view returns (address);
    function token() external view returns (address);
    function updateQuorumNumerator(uint256 newQuorumNumerator) external;
    function updateTimelock(address newTimelock) external;
    function upgradeToAndCall(address newImplementation, bytes memory data) external payable;
    function version() external view returns (string memory);
    function votingDelay() external view returns (uint256);
    function votingPeriod() external view returns (uint256);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "receive",
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "BALLOT_TYPEHASH",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "CLOCK_MODE",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "COUNTING_MODE",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "EXTENDED_BALLOT_TYPEHASH",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "UPGRADE_INTERFACE_VERSION",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "cancel",
    "inputs": [
      {
        "name": "targets",
        "type": "address[]",
        "internalType": "address[]"
      },
      {
        "name": "values",
        "type": "uint256[]",
        "internalType": "uint256[]"
      },
      {
        "name": "calldatas",
        "type": "bytes[]",
        "internalType": "bytes[]"
      },
      {
        "name": "descriptionHash",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "castVote",
    "inputs": [
      {
        "name": "proposalId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "support",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "castVoteBySig",
    "inputs": [
      {
        "name": "proposalId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "support",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "voter",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "signature",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "castVoteWithReason",
    "inputs": [
      {
        "name": "proposalId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "support",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "reason",
        "type": "string",
        "internalType": "string"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "castVoteWithReasonAndParams",
    "inputs": [
      {
        "name": "proposalId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "support",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "reason",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "params",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "castVoteWithReasonAndParamsBySig",
    "inputs": [
      {
        "name": "proposalId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "support",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "voter",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "reason",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "params",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "signature",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "clock",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint48",
        "internalType": "uint48"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "eip712Domain",
    "inputs": [],
    "outputs": [
      {
        "name": "fields",
        "type": "bytes1",
        "internalType": "bytes1"
      },
      {
        "name": "name",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "version",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "chainId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "verifyingContract",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "salt",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "extensions",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "execute",
    "inputs": [
      {
        "name": "targets",
        "type": "address[]",
        "internalType": "address[]"
      },
      {
        "name": "values",
        "type": "uint256[]",
        "internalType": "uint256[]"
      },
      {
        "name": "calldatas",
        "type": "bytes[]",
        "internalType": "bytes[]"
      },
      {
        "name": "descriptionHash",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "getVotes",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "timepoint",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getVotesWithParams",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "timepoint",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "params",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "hasVoted",
    "inputs": [
      {
        "name": "proposalId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "hashProposal",
    "inputs": [
      {
        "name": "targets",
        "type": "address[]",
        "internalType": "address[]"
      },
      {
        "name": "values",
        "type": "uint256[]",
        "internalType": "uint256[]"
      },
      {
        "name": "calldatas",
        "type": "bytes[]",
        "internalType": "bytes[]"
      },
      {
        "name": "descriptionHash",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "initialize",
    "inputs": [
      {
        "name": "token",
        "type": "address",
        "internalType": "contract IVotes"
      },
      {
        "name": "timelock",
        "type": "address",
        "internalType": "contract TimelockControllerUpgradeable"
      },
      {
        "name": "initialVotingDelay",
        "type": "uint48",
        "internalType": "uint48"
      },
      {
        "name": "initialVotingPeriod",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "initialProposalThreshold",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "quorumPercent",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "name",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "nonces",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "onERC1155BatchReceived",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
        "type": "uint256[]",
        "internalType": "uint256[]"
      },
      {
        "name": "",
        "type": "uint256[]",
        "internalType": "uint256[]"
      },
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes4",
        "internalType": "bytes4"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "onERC1155Received",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes4",
        "internalType": "bytes4"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "onERC721Received",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes4",
        "internalType": "bytes4"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "proposalDeadline",
    "inputs": [
      {
        "name": "proposalId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "proposalEta",
    "inputs": [
      {
        "name": "proposalId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "proposalNeedsQueuing",
    "inputs": [
      {
        "name": "proposalId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "proposalProposer",
    "inputs": [
      {
        "name": "proposalId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "proposalSnapshot",
    "inputs": [
      {
        "name": "proposalId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "proposalThreshold",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "proposalVotes",
    "inputs": [
      {
        "name": "proposalId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "againstVotes",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "forVotes",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "abstainVotes",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "propose",
    "inputs": [
      {
        "name": "targets",
        "type": "address[]",
        "internalType": "address[]"
      },
      {
        "name": "values",
        "type": "uint256[]",
        "internalType": "uint256[]"
      },
      {
        "name": "calldatas",
        "type": "bytes[]",
        "internalType": "bytes[]"
      },
      {
        "name": "description",
        "type": "string",
        "internalType": "string"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "proxiableUUID",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "queue",
    "inputs": [
      {
        "name": "targets",
        "type": "address[]",
        "internalType": "address[]"
      },
      {
        "name": "values",
        "type": "uint256[]",
        "internalType": "uint256[]"
      },
      {
        "name": "calldatas",
        "type": "bytes[]",
        "internalType": "bytes[]"
      },
      {
        "name": "descriptionHash",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "quorum",
    "inputs": [
      {
        "name": "blockNumber",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "quorumDenominator",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "quorumNumerator",
    "inputs": [
      {
        "name": "timepoint",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "quorumNumerator",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "relay",
    "inputs": [
      {
        "name": "target",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "value",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "setProposalThreshold",
    "inputs": [
      {
        "name": "newProposalThreshold",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setVotingDelay",
    "inputs": [
      {
        "name": "newVotingDelay",
        "type": "uint48",
        "internalType": "uint48"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setVotingPeriod",
    "inputs": [
      {
        "name": "newVotingPeriod",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "state",
    "inputs": [
      {
        "name": "proposalId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "enum IGovernor.ProposalState"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "supportsInterface",
    "inputs": [
      {
        "name": "interfaceId",
        "type": "bytes4",
        "internalType": "bytes4"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "timelock",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "token",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IERC5805"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "updateQuorumNumerator",
    "inputs": [
      {
        "name": "newQuorumNumerator",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateTimelock",
    "inputs": [
      {
        "name": "newTimelock",
        "type": "address",
        "internalType": "contract TimelockControllerUpgradeable"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "upgradeToAndCall",
    "inputs": [
      {
        "name": "newImplementation",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "version",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "votingDelay",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "votingPeriod",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "event",
    "name": "EIP712DomainChanged",
    "inputs": [],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Initialized",
    "inputs": [
      {
        "name": "version",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ProposalCanceled",
    "inputs": [
      {
        "name": "proposalId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ProposalCreated",
    "inputs": [
      {
        "name": "proposalId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "proposer",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "targets",
        "type": "address[]",
        "indexed": false,
        "internalType": "address[]"
      },
      {
        "name": "values",
        "type": "uint256[]",
        "indexed": false,
        "internalType": "uint256[]"
      },
      {
        "name": "signatures",
        "type": "string[]",
        "indexed": false,
        "internalType": "string[]"
      },
      {
        "name": "calldatas",
        "type": "bytes[]",
        "indexed": false,
        "internalType": "bytes[]"
      },
      {
        "name": "voteStart",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "voteEnd",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "description",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ProposalExecuted",
    "inputs": [
      {
        "name": "proposalId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ProposalQueued",
    "inputs": [
      {
        "name": "proposalId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "etaSeconds",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ProposalThresholdSet",
    "inputs": [
      {
        "name": "oldProposalThreshold",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newProposalThreshold",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "QuorumNumeratorUpdated",
    "inputs": [
      {
        "name": "oldQuorumNumerator",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newQuorumNumerator",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TimelockChange",
    "inputs": [
      {
        "name": "oldTimelock",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "newTimelock",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Upgraded",
    "inputs": [
      {
        "name": "implementation",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "VoteCast",
    "inputs": [
      {
        "name": "voter",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "proposalId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "support",
        "type": "uint8",
        "indexed": false,
        "internalType": "uint8"
      },
      {
        "name": "weight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "reason",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "VoteCastWithParams",
    "inputs": [
      {
        "name": "voter",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "proposalId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "support",
        "type": "uint8",
        "indexed": false,
        "internalType": "uint8"
      },
      {
        "name": "weight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "reason",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "params",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "VotingDelaySet",
    "inputs": [
      {
        "name": "oldVotingDelay",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newVotingDelay",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "VotingPeriodSet",
    "inputs": [
      {
        "name": "oldVotingPeriod",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newVotingPeriod",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "AddressEmptyCode",
    "inputs": [
      {
        "name": "target",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "CheckpointUnorderedInsertion",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ERC1967InvalidImplementation",
    "inputs": [
      {
        "name": "implementation",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC1967NonPayable",
    "inputs": []
  },
  {
    "type": "error",
    "name": "FailedCall",
    "inputs": []
  },
  {
    "type": "error",
    "name": "GovernorAlreadyCastVote",
    "inputs": [
      {
        "name": "voter",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "GovernorAlreadyQueuedProposal",
    "inputs": [
      {
        "name": "proposalId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "GovernorDisabledDeposit",
    "inputs": []
  },
  {
    "type": "error",
    "name": "GovernorInsufficientProposerVotes",
    "inputs": [
      {
        "name": "proposer",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "votes",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "threshold",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "GovernorInvalidProposalLength",
    "inputs": [
      {
        "name": "targets",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "calldatas",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "values",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "GovernorInvalidQuorumFraction",
    "inputs": [
      {
        "name": "quorumNumerator",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "quorumDenominator",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "GovernorInvalidSignature",
    "inputs": [
      {
        "name": "voter",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "GovernorInvalidVoteParams",
    "inputs": []
  },
  {
    "type": "error",
    "name": "GovernorInvalidVoteType",
    "inputs": []
  },
  {
    "type": "error",
    "name": "GovernorInvalidVotingPeriod",
    "inputs": [
      {
        "name": "votingPeriod",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "GovernorNonexistentProposal",
    "inputs": [
      {
        "name": "proposalId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "GovernorNotQueuedProposal",
    "inputs": [
      {
        "name": "proposalId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "GovernorOnlyExecutor",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "GovernorOnlyProposer",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "GovernorQueueNotImplemented",
    "inputs": []
  },
  {
    "type": "error",
    "name": "GovernorRestrictedProposer",
    "inputs": [
      {
        "name": "proposer",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "GovernorUnexpectedProposalState",
    "inputs": [
      {
        "name": "proposalId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "current",
        "type": "uint8",
        "internalType": "enum IGovernor.ProposalState"
      },
      {
        "name": "expectedStates",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidAccountNonce",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "currentNonce",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidInitialization",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotInitializing",
    "inputs": []
  },
  {
    "type": "error",
    "name": "SafeCastOverflowedUintDowncast",
    "inputs": [
      {
        "name": "bits",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "value",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "UUPSUnauthorizedCallContext",
    "inputs": []
  },
  {
    "type": "error",
    "name": "UUPSUnsupportedProxiableUUID",
    "inputs": [
      {
        "name": "slot",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod TangleGovernor {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a080604052346100e857306080527ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a005460ff8160401c166100d9576002600160401b03196001600160401b03821601610073575b604051614a5990816100ed82396080518181816111fc01526113170152f35b6001600160401b0319166001600160401b039081177ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a005581527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d290602090a15f80610054565b63f92ee8a960e01b5f5260045ffd5b5f80fdfe60806040526004361015610022575b3615610018575f80fd5b610020612c44565b005b5f3560e01c806301ffc9a71461036157806302a251a31461035c57806306f3f9e61461035757806306fdde0314610352578063143489d01461034d578063150b7a0214610348578063160cbed71461034357806322f120de1461033e5780632656227d146103395780632d63f693146103345780632fe3e2611461032f5780633932abb11461032a5780633e4f49e6146103255780634385963214610320578063452115d61461031b5780634bf5d7e9146103165780634f1ef2861461031157806352d1902d1461030c578063544ffc9c1461030757806354fd4d501461030257806356781388146102fd5780635b8d0e0d146102f85780635f398a14146102f357806360c4247f146102ee57806379051887146102e95780637b3c71d3146102e45780637d5e81e2146102df5780637ecebe00146102da57806384b0196e146102d55780638ff262e3146102d057806391ddadf4146102cb57806397c3d334146102c65780639a802a6d146102c1578063a7713a70146102bc578063a890c910146102b7578063a9a95294146102b2578063ab58fb8e146102ad578063ad3cb1cc146102a8578063b58131b0146102a3578063bc197c811461029e578063c01f9e3714610299578063c28bc2fa14610294578063c59057e41461028f578063d33219b41461028a578063dd4e2ba514610285578063deaaa7cc14610280578063e540d01d1461027b578063eb9019d414610276578063ece40cc114610271578063f23a6e611461026c578063f8ce560a146102675763fc0c546a0361000e5761205a565b611fad565b611f3b565b611f17565b611e85565b611e53565b611e19565b611dba565b611d86565b611d6a565b611cff565b611ce1565b611c34565b611c0b565b611bc4565b611b71565b611b55565b611ac5565b611a9a565b6119d7565b6119bc565b611992565b61185f565b61178f565b6116a3565b6115f5565b6115a0565b611573565b611555565b6114e0565b61143a565b6113d4565b6113a9565b61135c565b611305565b6111ba565b61118b565b61102d565b610fcc565b610f9d565b610f3b565b610f01565b610ee3565b610d7e565b610bda565b61096a565b610721565b61060d565b61051b565b610415565b6103dd565b346103cf5760203660031901126103cf5760043563ffffffff60e01b81168091036103cf576020906332a2ad4360e11b81149081156103be575b81156103ad575b506040519015158152f35b6301ffc9a760e01b1490505f6103a2565b630271189760e51b8114915061039b565b5f80fd5b5f9103126103cf57565b346103cf575f3660031901126103cf57602061040d63ffffffff5f805160206148cd8339815191525460301c1690565b604051908152f35b346103cf5760203660031901126103cf57600435610431612c66565b606481116104cc576001600160d01b036104496138b5565b16906104536129ef565b916001600160d01b0382116104b4577f0553476bf02ef2726e8ce5ced78d63e26e602e4a2257b1f559418e24b463399792610498906001600160d01b0384169061455b565b505060408051918252602082019290925290819081015b0390a1005b506306dfcc6560e41b5f5260d060045260245260445ffd5b63243e544560e01b5f52600452606460245260445ffd5b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b9060206105189281815201906104e3565b90565b346103cf575f3660031901126103cf576040515f5f80516020614a0d833981519152546105478161208e565b80845290600181169081156105e9575060011461057f575b61057b8361056f8185038261067e565b60405191829182610507565b0390f35b5f80516020614a0d8339815191525f9081527fda13dda7583a39a3cd73e8830529c760837228fa4683752c823b17e10548aad5939250905b8082106105cf5750909150810160200161056f61055f565b9192600181602092548385880101520191019092916105b7565b60ff191660208086019190915291151560051b8401909101915061056f905061055f565b346103cf5760203660031901126103cf576004355f9081525f805160206148ad83398151915260209081526040909120546001600160a01b03166040516001600160a01b039091168152f35b6001600160a01b038116036103cf57565b634e487b7160e01b5f52604160045260245ffd5b90601f801991011681019081106001600160401b0382111761069f57604052565b61066a565b604051906106b360408361067e565b565b6001600160401b03811161069f57601f01601f191660200190565b9291926106dc826106b5565b916106ea604051938461067e565b8294818452818301116103cf578281602093845f960137010152565b9080601f830112156103cf57816020610518933591016106d0565b346103cf5760803660031901126103cf5761073d600435610659565b610748602435610659565b6064356001600160401b0381116103cf57610767903690600401610706565b50610770612c29565b306001600160a01b039091160361079357604051630a85bd0160e11b8152602090f35b637485328f60e11b5f5260045ffd5b6001600160401b03811161069f5760051b60200190565b9080601f830112156103cf5781356107d0816107a2565b926107de604051948561067e565b81845260208085019260051b8201019283116103cf57602001905b8282106108065750505090565b60208091833561081581610659565b8152019101906107f9565b9080601f830112156103cf578135610837816107a2565b92610845604051948561067e565b81845260208085019260051b8201019283116103cf57602001905b82821061086d5750505090565b8135815260209182019101610860565b9080601f830112156103cf578135610894816107a2565b926108a2604051948561067e565b81845260208085019260051b820101918383116103cf5760208201905b8382106108ce57505050505090565b81356001600160401b0381116103cf576020916108f087848094880101610706565b8152019101906108bf565b60806003198201126103cf576004356001600160401b0381116103cf5781610925916004016107b9565b916024356001600160401b0381116103cf578261094491600401610820565b91604435906001600160401b0382116103cf576109639160040161087d565b9060643590565b346103cf57610978366108fb565b909261098682858584612bc2565b9361099085612d19565b505f805160206149ed833981519152546109ba906001600160a01b03165b6001600160a01b031690565b936040519363793d064960e11b8552602085600481895afa948515610b62575f95610b96575b503060601b6bffffffffffffffffffffffff191618946020604051809263b1c5f42760e01b82528180610a198b89898c60048601613aed565b03915afa908115610b62575f91610b67575b50610a3587612240565b555f805160206149ed83398151915254610a57906001600160a01b03166109ae565b90813b156103cf575f8094610a8387604051998a97889687956308f2a0bb60e41b875260048701613b32565b03925af1908115610b6257610aa792610aa292610b48575b504261348d565b613450565b9065ffffffffffff821615610b39577f9a2e42fd6722813d69113e7d0079d3d940171428df7373df9c7f7617cfda2892610b2683610b0761057b956001610aed8761226d565b019065ffffffffffff1665ffffffffffff19825416179055565b6040805185815265ffffffffffff909216602083015290918291820190565b0390a16040519081529081906020820190565b634844252360e11b5f5260045ffd5b80610b565f610b5c9361067e565b806103d3565b5f610a9b565b6124e1565b610b89915060203d602011610b8f575b610b81818361067e565b81019061315e565b5f610a2b565b503d610b77565b610bb091955060203d602011610b8f57610b81818361067e565b935f6109e0565b65ffffffffffff8116036103cf57565b6064359063ffffffff821682036103cf57565b346103cf5760c03660031901126103cf57600435610bf781610659565b60243590610c0482610659565b604435610c1081610bb7565b610c18610bc7565b6084359060a435925f80516020614a2d83398151915254956001600160401b03610c5d610c50610c4c8a60ff9060401c1690565b1590565b986001600160401b031690565b1680159081610d76575b6001149081610d6c575b159081610d63575b50610d5457610cbc9587610cb360016001600160401b03195f80516020614a2d8339815191525416175f80516020614a2d83398151915255565b610d1f57612287565b610cc257005b610cec60ff60401b195f80516020614a2d83398151915254165f80516020614a2d83398151915255565b604051600181527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29080602081016104af565b610d4f600160401b60ff60401b195f80516020614a2d8339815191525416175f80516020614a2d83398151915255565b612287565b63f92ee8a960e01b5f5260045ffd5b9050155f610c79565b303b159150610c71565b889150610c67565b610d87366108fb565b91610d9483838387612bc2565b936020610da2603087612dd9565b50610dc2610daf8761226d565b805460ff60f01b1916600160f01b179055565b610dca612c29565b306001600160a01b0390911603610e7b575b5091849391610dee9361057b96613edd565b30610dfa6109ae612c29565b141580610e4e575b610e39575b6040518181527f712ae1383f79ac853f8d882153778e0260ef8f03b504e2866e0593e04d2b291f908060208101610b26565b5f5f805160206149cd83398151915255610e07565b50610e76610c4c5f805160206149cd833981519152546001600160801b0381169060801c1490565b610e02565b949091935f5b8351811015610ed55760019030610eab6109ae610e9e84896124a2565b516001600160a01b031690565b14610eb7575b01610e81565b610ed0610ec482886124a2565b51898151910120612fa7565b610eb1565b50909450929061057b610ddc565b346103cf5760203660031901126103cf57602061040d6004356124bb565b346103cf575f3660031901126103cf5760206040517f3e83946653575f9a39005e1545185629e92736b7528ab20ca3816f315424a8118152f35b346103cf575f3660031901126103cf57602065ffffffffffff5f805160206148cd8339815191525416604051908152f35b634e487b7160e01b5f52602160045260245ffd5b60081115610f8a57565b610f6c565b6008811015610f8a57602452565b346103cf5760203660031901126103cf57610fb960043561304e565b6040516008821015610f8a576020918152f35b346103cf5760403660031901126103cf57602060ff611021602435600435610ff382610659565b5f525f805160206149ad8339815191528452600360405f20019060018060a01b03165f5260205260405f2090565b54166040519015158152f35b346103cf5761103b366108fb565b9161104883838387612bc2565b61105181612d59565b505f9081525f805160206148ad83398151915260205260409020546001600160a01b031633036111785761108493612bc2565b61108f603b82612dd9565b506110b161109c8261226d565b80546001600160f81b0316600160f81b179055565b6040518181527f789cf55be980739dad1d0699b93b58e806b51c9d96619bfa8fe0a28abaa7b30c90602090a16110e681612240565b5490816110f9575b604051908152602090f35b5f805160206149ed8339815191525461111a906001600160a01b03166109ae565b803b156103cf5760405163c4d252f560e01b815260048101939093525f908390602490829084905af1918215610b625761057b92611164575b505f61115e82612240565b556110ee565b80610b565f6111729361067e565b5f611153565b63233d98e360e01b5f523360045260245ffd5b346103cf575f3660031901126103cf5761057b6111a6612527565b6040519182916020835260208301906104e3565b60403660031901126103cf576004356111d281610659565b6024356001600160401b0381116103cf576111f1903690600401610706565b906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000163081149081156112e3575b506112d457611234612c66565b6040516352d1902d60e01b8152916020836004816001600160a01b0386165afa5f93816112b3575b5061128057634c9c8ce360e01b5f526001600160a01b03821660045260245ffd5b5ffd5b905f8051602061498d833981519152830361129f57610020925061407e565b632a87526960e21b5f52600483905260245ffd5b6112cd91945060203d602011610b8f57610b81818361067e565b925f61125c565b63703e46dd60e11b5f5260045ffd5b5f8051602061498d833981519152546001600160a01b0316141590505f611227565b346103cf575f3660031901126103cf577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031630036112d45760206040515f8051602061498d8339815191528152f35b346103cf5760203660031901126103cf576004355f525f805160206149ad833981519152602052606060405f20805490600260018201549101549060405192835260208301526040820152f35b346103cf575f3660031901126103cf5761057b6111a66125e9565b6024359060ff821682036103cf57565b346103cf5760403660031901126103cf57602061040d6004356113f56113c4565b60405191611403858461067e565b5f8352339061316d565b9181601f840112156103cf578235916001600160401b0383116103cf57602083818601950101116103cf57565b346103cf5760c03660031901126103cf576004356114566113c4565b906044359061146482610659565b6064356001600160401b0381116103cf5761148390369060040161140d565b6084356001600160401b0381116103cf576114a2903690600401610706565b9160a435946001600160401b0386116103cf5761057b966114ca6114d0973690600401610706565b95612608565b6040519081529081906020820190565b346103cf5760803660031901126103cf576004356114fc6113c4565b906044356001600160401b0381116103cf5761151c90369060040161140d565b9190926064356001600160401b0381116103cf576110ee9461154561154d923690600401610706565b9436916106d0565b91339061333f565b346103cf5760203660031901126103cf57602061040d60043561274a565b346103cf5760203660031901126103cf5761002060043561159381610bb7565b61159b612c66565b61349a565b346103cf5760603660031901126103cf576004356115bc6113c4565b90604435906001600160401b0382116103cf576020926115ed6115e661040d94369060040161140d565b36916106d0565b91339061316d565b346103cf5760803660031901126103cf576004356001600160401b0381116103cf576116259036906004016107b9565b6024356001600160401b0381116103cf57611644903690600401610820565b906044356001600160401b0381116103cf5761166490369060040161087d565b90606435916001600160401b0383116103cf57366023840112156103cf5761057b9361169d6114d09436906024816004013591016106d0565b926128ba565b346103cf5760203660031901126103cf576004356116c081610659565b60018060a01b03165f527f5ab42ced628888259c08ac98db1eb0cf702fc1501344311d8b100cd1bfe4bb00602052602060405f2054604051908152f35b90602080835192838152019201905f5b81811061171a5750505090565b825184526020938401939092019160010161170d565b916117659061175761051897959693600f60f81b865260e0602087015260e08601906104e3565b9084820360408601526104e3565b60608301949094526001600160a01b031660808201525f60a082015280830360c0909101526116fd565b346103cf575f3660031901126103cf577fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d100541580611836575b156117f9576117d56120c6565b6117dd612193565b9061057b6117e96129d4565b6040519384933091469186611730565b60405162461bcd60e51b81526020600482015260156024820152741152540dcc4c8e88155b9a5b9a5d1a585b1a5e9959605a1b6044820152606490fd5b507fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d10154156117c8565b346103cf5760803660031901126103cf5760043561187b6113c4565b906044359161188983610659565b6064356001600160401b0381116103cf57610c4c6118ae61195b923690600401610706565b6001600160a01b0386165f9081527f5ab42ced628888259c08ac98db1eb0cf702fc1501344311d8b100cd1bfe4bb00602052604090208054600181019091556119559060405160208101917ff2aad550cf55f045cb27e9c559f9889fdfb6e6cdaa032301d6ea397784ae51d7835288604083015260ff8816606083015260018060a01b038a16608083015260a082015260a0815261194d60c08261067e565b51902061318a565b86613216565b61197657906114d09161057b93611970611bb0565b9261316d565b6394ab6c0760e01b5f526001600160a01b03831660045260245ffd5b346103cf575f3660031901126103cf5760206119ac6129ef565b65ffffffffffff60405191168152f35b346103cf575f3660031901126103cf57602060405160648152f35b346103cf5760603660031901126103cf576004356119f481610659565b6044356024356001600160401b0382116103cf57611a186020923690600401610706565b505f8051602061494d83398151915254604051630748d63560e31b81526001600160a01b0394851660048201526024810192909252909283916044918391165afa8015610b625761057b915f91611a7b575b506040519081529081906020820190565b611a94915060203d602011610b8f57610b81818361067e565b5f611a6a565b346103cf575f3660031901126103cf5760206001600160d01b03611abc6138b5565b16604051908152f35b346103cf5760203660031901126103cf57600435611ae281610659565b611aea612c66565b5f805160206149ed83398151915254604080516001600160a01b03808416825290931660208401819052927f08f74ea46ef7894f65eabfb5e6e695de773a000b47c529ab559178069b2264019190a16001600160a01b031916175f805160206149ed83398151915255005b346103cf5760203660031901126103cf57602060405160018152f35b346103cf5760203660031901126103cf57602061040d6004355f525f805160206148ad83398151915260205265ffffffffffff600160405f2001541690565b60405190611bbf60208361067e565b5f8252565b346103cf575f3660031901126103cf5761057b604051611be560408261067e565b60058152640352e302e360dc1b60208201526040519182916020835260208301906104e3565b346103cf575f3660031901126103cf5760205f8051602061496d83398151915254604051908152f35b346103cf5760a03660031901126103cf57611c50600435610659565b611c5b602435610659565b6044356001600160401b0381116103cf57611c7a903690600401610820565b506064356001600160401b0381116103cf57611c9a903690600401610820565b506084356001600160401b0381116103cf57611cba903690600401610706565b5061057b611cc6612a75565b6040516001600160e01b031990911681529081906020820190565b346103cf5760203660031901126103cf57602061040d600435612aa0565b60603660031901126103cf57600435611d1781610659565b6024356044356001600160401b0381116103cf57610020925f92611d408493369060040161140d565b9190611d4a612c66565b826040519384928337810185815203925af1611d64612aff565b90613911565b346103cf57602061040d611d7d366108fb565b92919091612bc2565b346103cf575f3660031901126103cf575f805160206149ed833981519152546040516001600160a01b039091168152602090f35b346103cf575f3660031901126103cf5761057b604051611ddb60408261067e565b602081527f737570706f72743d627261766f2671756f72756d3d666f722c6162737461696e60208201526040519182916020835260208301906104e3565b346103cf575f3660031901126103cf5760206040517ff2aad550cf55f045cb27e9c559f9889fdfb6e6cdaa032301d6ea397784ae51d78152f35b346103cf5760203660031901126103cf5760043563ffffffff811681036103cf5761002090611e80612c66565b61391e565b346103cf5760403660031901126103cf57600435611ea281610659565b60206024355f604051611eb5848261067e565b525f8051602061494d83398151915254604051630748d63560e31b81526001600160a01b0394851660048201526024810192909252909283916044918391165afa8015610b625761057b915f91611a7b57506040519081529081906020820190565b346103cf5760203660031901126103cf57610020600435611f36612c66565b6139b7565b346103cf5760a03660031901126103cf57611f57600435610659565b611f62602435610659565b6084356001600160401b0381116103cf57611f81903690600401610706565b50611f8a612c29565b306001600160a01b03909116036107935760405163f23a6e6160e01b8152602090f35b346103cf5760203660031901126103cf5760246004355f8051602061494d83398151915254604051632394e7a360e21b8152600481018390529260209184919082906001600160a01b03165afa918215610b62575f92612035575b506120129061274a565b908181029181830414901517156120305761057b90606490046114d0565b612719565b6120129192506120539060203d602011610b8f57610b81818361067e565b9190612008565b346103cf575f3660031901126103cf575f8051602061494d833981519152546040516001600160a01b039091168152602090f35b90600182811c921680156120bc575b60208310146120a857565b634e487b7160e01b5f52602260045260245ffd5b91607f169161209d565b604051905f825f8051602061490d83398151915254916120e58361208e565b80835292600181169081156121745750600114612109575b6106b39250038361067e565b505f8051602061490d8339815191525f90815290917f42ad5d3e1f2e6e70edcf6d991b8a3023d3fca8047a131592f9edb9fd9b89d57d5b8183106121585750509060206106b3928201016120fd565b6020919350806001915483858901015201910190918492612140565b602092506106b394915060ff191682840152151560051b8201016120fd565b604051905f825f8051602061492d83398151915254916121b28361208e565b808352926001811690811561217457506001146121d5576106b39250038361067e565b505f8051602061492d8339815191525f90815290917f5f9ce34815f8e11431c7bb75a8e6886a91478f7ffc1dbb0a98dc240fddd76b755b8183106122245750509060206106b3928201016120fd565b602091935080600191548385890101520191019091849261220c565b5f527f0d5829787b8befdbc6044ef7457d8a95c2a04bc99235349f1a212c063e59d40160205260405f2090565b5f525f805160206148ad83398151915260205260405f2090565b929094939160405161229a60408261067e565b600e81526d2a30b733b632a3b7bb32b93737b960911b60208201526122bd613b7f565b6122c56125e9565b6122cd613b7f565b81516001600160401b03811161069f576122fd816122f85f8051602061490d8339815191525461208e565b613baa565b6020601f82116001146123dc57936123ad6123b2946123586123c99c9b9995612344866123bf9b976123c49e9b5f916123d1575b508160011b915f199060031b1c19161790565b5f8051602061490d83398151915255613cb1565b6123805f7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d10055565b6123a85f7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d10155565b613dd0565b612e17565b6123ba613b7f565b612e33565b612e7d565b612f2c565b6106b3613b7f565b90508501515f612331565b5f8051602061490d8339815191525f52601f198216907f42ad5d3e1f2e6e70edcf6d991b8a3023d3fca8047a131592f9edb9fd9b89d57d915f5b8181106124765750946123586123c99c9b99956001866123c49d9a966123ad966123bf9d996123b29c1061245e575b5050811b015f8051602061490d83398151915255613cb1565b8601515f1960f88460031b161c191690555f80612445565b9192602060018192868a015181550194019201612416565b634e487b7160e01b5f52603260045260245ffd5b80518210156124b65760209160051b010190565b61248e565b5f525f805160206148ad83398151915260205265ffffffffffff60405f205460a01c1690565b6040513d5f823e3d90fd5b604051906124fb60408361067e565b601d82527f6d6f64653d626c6f636b6e756d6265722666726f6d3d64656661756c740000006020830152565b5f8051602061494d83398151915254604051634bf5d7e960e01b8152905f90829060049082906001600160a01b03165afa5f918161256e575b5061051857506105186124ec565b9091503d805f833e612580818361067e565b8101906020818303126103cf578051906001600160401b0382116103cf570181601f820112156103cf578051906125b6826106b5565b926125c4604051948561067e565b828452602083830101116103cf57815f9260208093018386015e83010152905f612560565b604051906125f860408361067e565b60018252603160f81b6020830152565b939092919695610c4c6126e2916126dc8a61265b8160018060a01b03165f527f5ab42ced628888259c08ac98db1eb0cf702fc1501344311d8b100cd1bfe4bb0060205260405f2080549060018201905590565b61266636888a6106d0565b602081519101208b5160208d0120906040519260208401947f3e83946653575f9a39005e1545185629e92736b7528ab20ca3816f315424a81186528d604086015260ff8d16606086015260018060a01b0316608085015260a084015260c083015260e082015260e0815261194d6101008261067e565b8a613216565b6126fd576105189596916126f79136916106d0565b9261333f565b6394ab6c0760e01b5f526001600160a01b03871660045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b5f1981019190821161203057565b60271981019190821161203057565b5f805160206148ed83398151915254905f198201828111612030578211156124b6575f805160206148ed8339815191525f527f293b0181c8ec34cd3252e741689bdc21b70ee7a0ec76216439035a5c3718909a8201548165ffffffffffff821611156128b157506127ba90613450565b5f829160058411612834575b6127d0935061439e565b806127da57505f90565b6128286128216127ec6105189361272d565b5f805160206148ed8339815191525f527f293b0181c8ec34cd3252e741689bdc21b70ee7a0ec76216439035a5c3718909b0190565b5460301c90565b6001600160d01b031690565b919261283f81614240565b8103908111612030576127d0935f805160206148ed8339815191525f5265ffffffffffff827f293b0181c8ec34cd3252e741689bdc21b70ee7a0ec76216439035a5c3718909b01541665ffffffffffff8516105f1461289f5750916127c6565b9291506128ab9061347f565b906127c6565b91505060301c90565b91939290936128c9823361351f565b156129c1575f8051602061496d8339815191525494856128f1575b61051894955033936136eb565b5f1965ffffffffffff6129026129ef565b160165ffffffffffff81116120305765ffffffffffff16955f60405161292960208261067e565b525f8051602061494d83398151915254604051630748d63560e31b81523360048201526024810198909852602090889060449082906001600160a01b03165afa968715610b62575f976129a0575b5080871061298557506128e4565b636121770b60e11b5f5233600452602487905260445260645ffd5b6129ba91975060203d602011610b8f57610b81818361067e565b955f612977565b63d9b3955760e01b5f523360045260245ffd5b604051906129e360208361067e565b5f808352366020840137565b5f8051602061494d833981519152546040516324776b7d60e21b815290602090829060049082906001600160a01b03165afa5f9181612a38575b50610518575061051843613450565b9091506020813d602011612a6d575b81612a546020938361067e565b810103126103cf5751612a6681610bb7565b905f612a29565b3d9150612a47565b5f805160206149ed83398151915254306001600160a01b03909116036107935763bc197c8160e01b90565b805f525f805160206148ad83398151915260205265ffffffffffff60405f205460a01c16905f525f805160206148ad83398151915260205263ffffffff60405f205460d01c160165ffffffffffff81116120305765ffffffffffff1690565b3d15612b29573d90612b10826106b5565b91612b1e604051938461067e565b82523d5f602084013e565b606090565b90602080835192838152019201905f5b818110612b4b5750505090565b82516001600160a01b0316845260209384019390920191600101612b3e565b9080602083519182815201916020808360051b8301019401925f915b838310612b9557505050505090565b9091929394602080612bb3600193601f1986820301875289516104e3565b97019301930191939290612b86565b9290612c2391612c0f612bfd94604051958694612beb602087019960808b5260a0880190612b2e565b868103601f19016040880152906116fd565b848103601f1901606086015290612b6a565b90608083015203601f19810183528261067e565b51902090565b5f805160206149ed833981519152546001600160a01b031690565b5f805160206149ed83398151915254306001600160a01b039091160361079357565b612c6e612c29565b336001600160a01b0390911603612cda57612c87612c29565b306001600160a01b0390911603612c9a57565b612ca3366106b5565b612cb0604051918261067e565b3681526020810190365f83375f602036830101525190205b80612cd1613a26565b03612cc8575b50565b6347096e4760e01b5f523360045260245ffd5b6008811015610f8a5760ff600191161b90565b600452606491906008811015610f8a576024525f604452565b612d228161304e565b906010612d2e83612ced565b1615612d38575090565b6331b75e4d60e01b5f52600452612d4f9150610f8f565b601060445260645ffd5b612d628161304e565b906001612d6e83612ced565b1615612d78575090565b6331b75e4d60e01b5f52600452612d8f9150610f8f565b600160445260645ffd5b612da28161304e565b906002612dae83612ced565b1615612db8575090565b6331b75e4d60e01b5f52600452612dcf9150610f8f565b600260445260645ffd5b90612de38261304e565b9181612dee84612ced565b1615612df957505090565b6331b75e4d60e01b5f52600452612e0f82610f8f565b60445260645ffd5b6106b39291611e80611f3692612e2b613b7f565b61159b613b7f565b612e3b613b7f565b612e43613b7f565b60018060a01b03166bffffffffffffffffffffffff60a01b5f8051602061494d8339815191525416175f8051602061494d83398151915255565b90612e86613b7f565b612e8e613b7f565b60648211612f14576001600160d01b03612ea66138b5565b1691612eb06129ef565b926001600160d01b0382116104b45791927f0553476bf02ef2726e8ce5ced78d63e26e602e4a2257b1f559418e24b46339979290612ef8906001600160d01b0384169061455b565b505060408051918252602082019290925290819081015b0390a1565b5063243e544560e01b5f52600452606460245260445ffd5b612f34613b7f565b612f3c613b7f565b5f805160206149ed83398151915254604080516001600160a01b03808416825290931660208401819052927f08f74ea46ef7894f65eabfb5e6e695de773a000b47c529ab559178069b2264019190a16001600160a01b031916175f805160206149ed83398151915255565b5f805160206149cd83398151915254908160801c6001600160801b0380600183011693168314613024575f527f7c712897014dbe49c045ef1299aa2d5f9e67e48eea4403efa21f1e0f3ac0cb0360205260405f20555f805160206149cd833981519152906001600160801b0382549181199060801b169116179055565b634e487b715f5260416020526024601cfd5b908160209103126103cf575180151581036103cf5790565b61305781613f73565b9061306182610f80565b6005820361315a576130739150612240565b545f805160206149ed83398151915254613095906001600160a01b03166109ae565b604051632c258a9f60e11b815260048101839052602081602481855afa908115610b62575f9161313b575b50156130cd575050600590565b604051632ab0f52960e01b81526004810192909252602090829060249082905afa908115610b62575f9161310c575b501561310757600790565b600290565b61312e915060203d602011613134575b613126818361067e565b810190613036565b5f6130fc565b503d61311c565b613154915060203d60201161313457613126818361067e565b5f6130c0565b5090565b908160209103126103cf575190565b9161051893916040519361318260208661067e565b5f855261333f565b6042906131956147fd565b61319d614867565b6040519060208201927f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f8452604083015260608201524660808201523060a082015260a081526131ee60c08261067e565b519020906040519161190160f01b8352600283015260228201522090565b60041115610f8a57565b9190823b61325157906132289161411d565b506132328161320c565b15918261323e57505090565b6001600160a01b03918216911614919050565b915f9261328761329585946040519283916020830195630b135d3f60e11b875260248401526040604484015260648301906104e3565b03601f19810183528261067e565b51915afa6132a1612aff565b816132d1575b816132b0575090565b90506132cd630b135d3f60e11b916020808251830101910161315e565b1490565b9050602081511015906132a7565b93909260ff61330b9361051897958752166020860152604085015260a0606085015260a08401906104e3565b9160808184039101526104e3565b909260ff60809361051896958452166020830152604082015281606082015201906104e3565b9291909261334c81612d99565b50613356816124bb565b5f8051602061494d83398151915254604051630748d63560e31b81526001600160a01b03808816600483018190526024830194909452929692909160209183916044918391165afa908115610b62576133b99285915f9361342f575b5084614157565b948051155f146133fc57506133f67fb8e138887d0aa13bab447e82de9d5c1777041ecd21ca36ba824ff1e6c07ddda4938660405194859485613319565b0390a290565b6133f6907fe2babfbac5889a709b63bb7f598b324e08bc5a4fb9ec647fb3cbc9ec07eb87129487604051958695866132df565b61344991935060203d602011610b8f57610b81818361067e565b915f6133b2565b65ffffffffffff81116134685765ffffffffffff1690565b6306dfcc6560e41b5f52603060045260245260445ffd5b906001820180921161203057565b9190820180921161203057565b7fc565b045403dc03c2eea82b81a0465edad9e2e7fc4d97e11421c209da93d7a93604065ffffffffffff805f805160206148cd83398151915254169382519485521692836020820152a165ffffffffffff195f805160206148cd8339815191525416175f805160206148cd83398151915255565b9081518110156124b6570160200190565b8151603481106135ce5760131981840101516001600160a01b0319166b1b91f1b211f2119351b859f160a31b016135ce57915f9261355c8161273b565b915b818310613579575050506001600160a01b0391821691161490565b90919361359f61359a61358c878561350e565b516001600160f81b03191690565b614423565b90156135c35760019160ff9060041b6010600160a01b03169116179401919061355e565b505050505050600190565b505050600190565b906135e0826107a2565b6135ed604051918261067e565b82815280926135fe601f19916107a2565b01905f5b82811061360e57505050565b806060602080938501015201613602565b9599989697949391926136609361365292885260018060a01b031660208801526101206040880152610120870190612b2e565b9085820360608701526116fd565b968388036080850152815180895260208901906020808260051b8c01019401915f905b8282106136bf5750505050610518969750906136a69184820360a0860152612b6a565b9360c083015260e08201526101008184039101526104e3565b909192946020806136dd6001938f601f1990820301865289516104e3565b970192019201909291613683565b92909493919461370382516020840120878387612bc2565b9584518251908181148015906138aa575b80156138a2575b61388357505065ffffffffffff6137436137348961226d565b5460a01c65ffffffffffff1690565b166138665791612f0f917f7d84a6263ae0d98d3329bd7b46bb4e8d6f98cd35a7adb45c274c8b7fd5ebd5e09594936137a761377c6129ef565b65ffffffffffff6137a065ffffffffffff5f805160206148cd833981519152541690565b911661348d565b906137c663ffffffff5f805160206148cd8339815191525460301c1690565b6138446137d28c61226d565b80546001600160a01b0319166001600160a01b038a1617815561381b6137f786613450565b825465ffffffffffff60a01b191660a09190911b65ffffffffffff60a01b16178255565b613824836144a0565b815463ffffffff60d01b191660d09190911b63ffffffff60d01b16179055565b61385861385189516135d6565b918461348d565b936040519889988d8a61361f565b61127d876138738161304e565b6331b75e4d60e01b5f5290612d00565b9151630447b05d60e41b5f908152600493909352602452604452606490fd5b50801561371b565b508251811415613714565b5f805160206148ed83398151915254806138ce57505f90565b805f19810111612030575f805160206148ed8339815191525f527f293b0181c8ec34cd3252e741689bdc21b70ee7a0ec76216439035a5c3718909a015460301c90565b9091906106b357506144cb565b63ffffffff81169081156139a45769ffffffff000000000000907f7e3f7f0708a84de9203036abaa450dccc85ad5ff52f78c170f3edb55cf5e882860405f805160206148cd833981519152549481519063ffffffff8760301c1682526020820152a160301b169069ffffffff0000000000001916175f805160206148cd83398151915255565b63f1cfbf0560e01b5f525f60045260245ffd5b5f8051602061496d8339815191525460408051918252602082018390527fccb45da8d5717e6c4544694297c4ba5cf151d455c9bb0ed4fc7a38411bc0546191a15f8051602061496d83398151915255565b8115613a12570490565b634e487b7160e01b5f52601260045260245ffd5b5f805160206149cd83398151915254906001600160801b0382169160801c8214613adb57815f527f7c712897014dbe49c045ef1299aa2d5f9e67e48eea4403efa21f1e0f3ac0cb0360205260405f2054916001600160801b0381165f527f7c712897014dbe49c045ef1299aa2d5f9e67e48eea4403efa21f1e0f3ac0cb036020525f60408120556001600160801b038060015f805160206149cd833981519152930116166001600160801b0319825416179055565b634e487b715f5260316020526024601cfd5b949392613b19608093613b0b613b279460a08a5260a08a0190612b2e565b9088820360208a01526116fd565b908682036040880152612b6a565b935f60608201520152565b9192613b6160a094613b53613b6f949998979960c0875260c0870190612b2e565b9085820360208701526116fd565b908382036040850152612b6a565b945f606083015260808201520152565b60ff5f80516020614a2d8339815191525460401c1615613b9b57565b631afcd79f60e31b5f5260045ffd5b601f8111613bb6575050565b5f8051602061490d8339815191525f5260205f20906020601f840160051c83019310613bfc575b601f0160051c01905b818110613bf1575050565b5f8155600101613be6565b9091508190613bdd565b601f8111613c12575050565b5f80516020614a0d8339815191525f5260205f20906020601f840160051c83019310613c58575b601f0160051c01905b818110613c4d575050565b5f8155600101613c42565b9091508190613c39565b601f8211613c6f57505050565b5f5260205f20906020601f840160051c83019310613ca7575b601f0160051c01905b818110613c9c575050565b5f8155600101613c91565b9091508190613c88565b9081516001600160401b03811161069f57613cf081613cdd5f8051602061492d8339815191525461208e565b5f8051602061492d833981519152613c62565b602092601f8211600114613d3c57613d20929382915f92613d31575b50508160011b915f199060031b1c19161790565b5f8051602061492d83398151915255565b015190505f80613d0c565b5f8051602061492d8339815191525f52601f198216937f5f9ce34815f8e11431c7bb75a8e6886a91478f7ffc1dbb0a98dc240fddd76b75915f5b868110613db85750836001959610613da0575b505050811b015f8051602061492d83398151915255565b01515f1960f88460031b161c191690555f8080613d89565b91926020600181928685015181550194019201613d76565b90613dd9613b7f565b81516001600160401b03811161069f57613e0981613e045f80516020614a0d8339815191525461208e565b613c06565b602092601f8211600114613e4957613e38929382915f92613d315750508160011b915f199060031b1c19161790565b5f80516020614a0d83398151915255565b5f80516020614a0d8339815191525f52601f198216937fda13dda7583a39a3cd73e8830529c760837228fa4683752c823b17e10548aad5915f5b868110613ec55750836001959610613ead575b505050811b015f80516020614a0d83398151915255565b01515f1960f88460031b161c191690555f8080613e96565b91926020600181928685015181550194019201613e83565b5f805160206149ed8339815191525490949192916001600160a01b03909116906bffffffffffffffffffffffff193060601b16823b156103cf57613f3b5f956040519788968795869563e38335e560e01b8752189260048601613aed565b039134905af18015610b6257613f5a575b50613f575f91612240565b55565b80613f665f809361067e565b8003126103cf575f613f4c565b613f7c8161226d565b5460f881901c9060f01c60ff166140775761407157613f9a816124bb565b801561405d57613fb6613fab6129ef565b65ffffffffffff1690565b8091101561405757613fc782612aa0565b10613fd25750600190565b613fde610c4c82614653565b8015614028575b15613ff05750600390565b61401a905f525f805160206148ad83398151915260205265ffffffffffff600160405f2001541690565b61402357600490565b600590565b50614052610c4c825f525f805160206149ad83398151915260205260405f20600181015490541090565b613fe5565b50505f90565b636ad0607560e01b5f52600482905260245ffd5b50600290565b5050600790565b90813b156140fc575f8051602061498d83398151915280546001600160a01b0319166001600160a01b0384169081179091557fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b5f80a28051156140e457612cd791614719565b5050346140ed57565b63b398979f60e01b5f5260045ffd5b50634c9c8ce360e01b5f9081526001600160a01b0391909116600452602490fd5b815191906041830361414d576141469250602082015190606060408401519301515f1a90614736565b9192909190565b50505f9160029190565b614178909291925f525f805160206149ad83398151915260205260405f2090565b91600383016141a161419a83839060018060a01b03165f5260205260405f2090565b5460ff1690565b614224576141c560ff93926141d2929060018060a01b03165f5260205260405f2090565b805460ff19166001179055565b16806141e957506141e482825461348d565b905590565b6001810361420057506001016141e482825461348d565b600203614215576002016141e482825461348d565b6303599be160e11b5f5260045ffd5b6371c6af4960e01b5f526001600160a01b03821660045260245ffd5b600181111561051857806001600160801b821015614361575b6143076142fd6142f36142e96142df6142d56142c461430e9760048a600160401b6143139c1015614354575b640100000000811015614347575b6201000081101561433a575b61010081101561432d575b6010811015614320575b1015614318575b60030260011c90565b6142ce818b613a08565b0160011c90565b6142ce818a613a08565b6142ce8189613a08565b6142ce8188613a08565b6142ce8187613a08565b6142ce8186613a08565b8093613a08565b821190565b900390565b60011b6142bb565b60041c9160021b916142b4565b60081c9160041b916142aa565b60101c9160081b9161429f565b60201c9160101b91614293565b60401c9160201b91614285565b505061431361430e6143076142fd6142f36142e96142df6142d56142c46143888a60801c90565b9850600160401b97506142599650505050505050565b905b8281106143ac57505090565b90918082169080831860011c8201809211612030575f805160206148ed8339815191525f527f293b0181c8ec34cd3252e741689bdc21b70ee7a0ec76216439035a5c3718909b82015465ffffffffffff90811690851610156144115750915b906143a0565b92915061441d9061347f565b9061440b565b60f81c9081602f1080614496575b1561444357600191602f190160ff1690565b816040108061448c575b1561445f576001916036190160ff1690565b8160601080614482575b1561447b576001916056190160ff1690565b5f91508190565b5060678210614469565b506047821061444d565b50603a8210614431565b63ffffffff81116144b45763ffffffff1690565b6306dfcc6560e41b5f52602060045260245260445ffd5b8051156144da57805190602001fd5b63d6bda27560e01b5f5260045ffd5b908154600160401b81101561069f57600181018084558110156124b6576106b3925f5260205f20019061453965ffffffffffff825116839065ffffffffffff1665ffffffffffff19825416179055565b60200151815465ffffffffffff1660309190911b65ffffffffffff1916179055565b5f805160206148ed83398151915254919291801561462a576127ec61457f9161272d565b90815461459b6145948265ffffffffffff1690565b9160301c90565b9265ffffffffffff80841692169180831161461b578692036145d9576145d592509065ffffffffffff82549181199060301b169116179055565b9190565b50506145d5906145f86145ea6106a4565b65ffffffffffff9092168252565b6001600160d01b03851660208201525b5f805160206148ed8339815191526144e9565b632520601d60e01b5f5260045ffd5b5061464e9061463a6145ea6106a4565b6001600160d01b0384166020820152614608565b5f9190565b805f525f805160206149ad833981519152602052602461467660405f20926124bb565b5f8051602061494d83398151915254604051632394e7a360e21b8152600481018390529260209184919082906001600160a01b03165afa918215610b62575f926146f4575b506146c59061274a565b90818102918183041490151715612030576146ef906064900491600260018201549101549061348d565b101590565b6146c59192506147129060203d602011610b8f57610b81818361067e565b91906146bb565b5f8061051893602081519101845af4614730612aff565b916147b8565b91907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a084116147ad579160209360809260ff5f9560405194855216868401526040830152606082015282805260015afa15610b62575f516001600160a01b038116156147a357905f905f90565b505f906001905f90565b5050505f9160039190565b906147c357506144cb565b815115806147f4575b6147d4575090565b639996b31560e01b5f9081526001600160a01b0391909116600452602490fd5b50803b156147cc565b6148056120c6565b8051908115614815576020012090565b50507fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d1005480156148425790565b507fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47090565b61486f612193565b805190811561487f576020012090565b50507fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d101548015614842579056fe7c712897014dbe49c045ef1299aa2d5f9e67e48eea4403efa21f1e0f3ac0cb0100d7616c8fe29c6c2fbe1d0c5bc8f2faa4c35b43746e70b24b4d532752affd01e770710421fd2cad75ad828c61aa98f2d77d423a440b67872d0f65554148e000a16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d102a16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d1033ba4977254e415696610a40ebf2258dbfa0ec6a2ff64e84bfe715ff16977cc0000d7616c8fe29c6c2fbe1d0c5bc8f2faa4c35b43746e70b24b4d532752affd00360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbca1cefa0f43667ef127a258e673c94202a79b656e62899531c4376d87a7f398007c712897014dbe49c045ef1299aa2d5f9e67e48eea4403efa21f1e0f3ac0cb020d5829787b8befdbc6044ef7457d8a95c2a04bc99235349f1a212c063e59d4007c712897014dbe49c045ef1299aa2d5f9e67e48eea4403efa21f1e0f3ac0cb00f0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00a164736f6c634300081a000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0\x80`@R4a\0\xE8W0`\x80R\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`\xFF\x81`@\x1C\x16a\0\xD9W`\x02`\x01`@\x1B\x03\x19`\x01`\x01`@\x1B\x03\x82\x16\x01a\0sW[`@QaJY\x90\x81a\0\xED\x829`\x80Q\x81\x81\x81a\x11\xFC\x01Ra\x13\x17\x01R\xF3[`\x01`\x01`@\x1B\x03\x19\x16`\x01`\x01`@\x1B\x03\x90\x81\x17\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0U\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x90\xA1_\x80a\0TV[c\xF9.\xE8\xA9`\xE0\x1B_R`\x04_\xFD[_\x80\xFD\xFE`\x80`@R`\x046\x10\x15a\0\"W[6\x15a\0\x18W_\x80\xFD[a\0 a,DV[\0[_5`\xE0\x1C\x80c\x01\xFF\xC9\xA7\x14a\x03aW\x80c\x02\xA2Q\xA3\x14a\x03\\W\x80c\x06\xF3\xF9\xE6\x14a\x03WW\x80c\x06\xFD\xDE\x03\x14a\x03RW\x80c\x144\x89\xD0\x14a\x03MW\x80c\x15\x0Bz\x02\x14a\x03HW\x80c\x16\x0C\xBE\xD7\x14a\x03CW\x80c\"\xF1 \xDE\x14a\x03>W\x80c&V\"}\x14a\x039W\x80c-c\xF6\x93\x14a\x034W\x80c/\xE3\xE2a\x14a\x03/W\x80c92\xAB\xB1\x14a\x03*W\x80c>OI\xE6\x14a\x03%W\x80cC\x85\x962\x14a\x03 W\x80cE!\x15\xD6\x14a\x03\x1BW\x80cK\xF5\xD7\xE9\x14a\x03\x16W\x80cO\x1E\xF2\x86\x14a\x03\x11W\x80cR\xD1\x90-\x14a\x03\x0CW\x80cTO\xFC\x9C\x14a\x03\x07W\x80cT\xFDMP\x14a\x03\x02W\x80cVx\x13\x88\x14a\x02\xFDW\x80c[\x8D\x0E\r\x14a\x02\xF8W\x80c_9\x8A\x14\x14a\x02\xF3W\x80c`\xC4$\x7F\x14a\x02\xEEW\x80cy\x05\x18\x87\x14a\x02\xE9W\x80c{<q\xD3\x14a\x02\xE4W\x80c}^\x81\xE2\x14a\x02\xDFW\x80c~\xCE\xBE\0\x14a\x02\xDAW\x80c\x84\xB0\x19n\x14a\x02\xD5W\x80c\x8F\xF2b\xE3\x14a\x02\xD0W\x80c\x91\xDD\xAD\xF4\x14a\x02\xCBW\x80c\x97\xC3\xD34\x14a\x02\xC6W\x80c\x9A\x80*m\x14a\x02\xC1W\x80c\xA7q:p\x14a\x02\xBCW\x80c\xA8\x90\xC9\x10\x14a\x02\xB7W\x80c\xA9\xA9R\x94\x14a\x02\xB2W\x80c\xABX\xFB\x8E\x14a\x02\xADW\x80c\xAD<\xB1\xCC\x14a\x02\xA8W\x80c\xB5\x811\xB0\x14a\x02\xA3W\x80c\xBC\x19|\x81\x14a\x02\x9EW\x80c\xC0\x1F\x9E7\x14a\x02\x99W\x80c\xC2\x8B\xC2\xFA\x14a\x02\x94W\x80c\xC5\x90W\xE4\x14a\x02\x8FW\x80c\xD32\x19\xB4\x14a\x02\x8AW\x80c\xDDN+\xA5\x14a\x02\x85W\x80c\xDE\xAA\xA7\xCC\x14a\x02\x80W\x80c\xE5@\xD0\x1D\x14a\x02{W\x80c\xEB\x90\x19\xD4\x14a\x02vW\x80c\xEC\xE4\x0C\xC1\x14a\x02qW\x80c\xF2:na\x14a\x02lW\x80c\xF8\xCEV\n\x14a\x02gWc\xFC\x0CTj\x03a\0\x0EWa ZV[a\x1F\xADV[a\x1F;V[a\x1F\x17V[a\x1E\x85V[a\x1ESV[a\x1E\x19V[a\x1D\xBAV[a\x1D\x86V[a\x1DjV[a\x1C\xFFV[a\x1C\xE1V[a\x1C4V[a\x1C\x0BV[a\x1B\xC4V[a\x1BqV[a\x1BUV[a\x1A\xC5V[a\x1A\x9AV[a\x19\xD7V[a\x19\xBCV[a\x19\x92V[a\x18_V[a\x17\x8FV[a\x16\xA3V[a\x15\xF5V[a\x15\xA0V[a\x15sV[a\x15UV[a\x14\xE0V[a\x14:V[a\x13\xD4V[a\x13\xA9V[a\x13\\V[a\x13\x05V[a\x11\xBAV[a\x11\x8BV[a\x10-V[a\x0F\xCCV[a\x0F\x9DV[a\x0F;V[a\x0F\x01V[a\x0E\xE3V[a\r~V[a\x0B\xDAV[a\tjV[a\x07!V[a\x06\rV[a\x05\x1BV[a\x04\x15V[a\x03\xDDV[4a\x03\xCFW` 6`\x03\x19\x01\x12a\x03\xCFW`\x045c\xFF\xFF\xFF\xFF`\xE0\x1B\x81\x16\x80\x91\x03a\x03\xCFW` \x90c2\xA2\xADC`\xE1\x1B\x81\x14\x90\x81\x15a\x03\xBEW[\x81\x15a\x03\xADW[P`@Q\x90\x15\x15\x81R\xF3[c\x01\xFF\xC9\xA7`\xE0\x1B\x14\x90P_a\x03\xA2V[c\x02q\x18\x97`\xE5\x1B\x81\x14\x91Pa\x03\x9BV[_\x80\xFD[_\x91\x03\x12a\x03\xCFWV[4a\x03\xCFW_6`\x03\x19\x01\x12a\x03\xCFW` a\x04\rc\xFF\xFF\xFF\xFF_\x80Q` aH\xCD\x839\x81Q\x91RT`0\x1C\x16\x90V[`@Q\x90\x81R\xF3[4a\x03\xCFW` 6`\x03\x19\x01\x12a\x03\xCFW`\x045a\x041a,fV[`d\x81\x11a\x04\xCCW`\x01`\x01`\xD0\x1B\x03a\x04Ia8\xB5V[\x16\x90a\x04Sa)\xEFV[\x91`\x01`\x01`\xD0\x1B\x03\x82\x11a\x04\xB4W\x7F\x05SGk\xF0.\xF2rn\x8C\xE5\xCE\xD7\x8Dc\xE2n`.J\"W\xB1\xF5YA\x8E$\xB4c9\x97\x92a\x04\x98\x90`\x01`\x01`\xD0\x1B\x03\x84\x16\x90aE[V[PP`@\x80Q\x91\x82R` \x82\x01\x92\x90\x92R\x90\x81\x90\x81\x01[\x03\x90\xA1\0[Pc\x06\xDF\xCCe`\xE4\x1B_R`\xD0`\x04R`$R`D_\xFD[c$>TE`\xE0\x1B_R`\x04R`d`$R`D_\xFD[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90` a\x05\x18\x92\x81\x81R\x01\x90a\x04\xE3V[\x90V[4a\x03\xCFW_6`\x03\x19\x01\x12a\x03\xCFW`@Q__\x80Q` aJ\r\x839\x81Q\x91RTa\x05G\x81a \x8EV[\x80\x84R\x90`\x01\x81\x16\x90\x81\x15a\x05\xE9WP`\x01\x14a\x05\x7FW[a\x05{\x83a\x05o\x81\x85\x03\x82a\x06~V[`@Q\x91\x82\x91\x82a\x05\x07V[\x03\x90\xF3[_\x80Q` aJ\r\x839\x81Q\x91R_\x90\x81R\x7F\xDA\x13\xDD\xA7X:9\xA3\xCDs\xE8\x83\x05)\xC7`\x83r(\xFAF\x83u,\x82;\x17\xE1\x05H\xAA\xD5\x93\x92P\x90[\x80\x82\x10a\x05\xCFWP\x90\x91P\x81\x01` \x01a\x05oa\x05_V[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a\x05\xB7V[`\xFF\x19\x16` \x80\x86\x01\x91\x90\x91R\x91\x15\x15`\x05\x1B\x84\x01\x90\x91\x01\x91Pa\x05o\x90Pa\x05_V[4a\x03\xCFW` 6`\x03\x19\x01\x12a\x03\xCFW`\x045_\x90\x81R_\x80Q` aH\xAD\x839\x81Q\x91R` \x90\x81R`@\x90\x91 T`\x01`\x01`\xA0\x1B\x03\x16`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\xF3[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x03\xCFWV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x06\x9FW`@RV[a\x06jV[`@Q\x90a\x06\xB3`@\x83a\x06~V[V[`\x01`\x01`@\x1B\x03\x81\x11a\x06\x9FW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\x06\xDC\x82a\x06\xB5V[\x91a\x06\xEA`@Q\x93\x84a\x06~V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x03\xCFW\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x03\xCFW\x81` a\x05\x18\x935\x91\x01a\x06\xD0V[4a\x03\xCFW`\x806`\x03\x19\x01\x12a\x03\xCFWa\x07=`\x045a\x06YV[a\x07H`$5a\x06YV[`d5`\x01`\x01`@\x1B\x03\x81\x11a\x03\xCFWa\x07g\x906\x90`\x04\x01a\x07\x06V[Pa\x07pa,)V[0`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x03a\x07\x93W`@Qc\n\x85\xBD\x01`\xE1\x1B\x81R` \x90\xF3[ct\x852\x8F`\xE1\x1B_R`\x04_\xFD[`\x01`\x01`@\x1B\x03\x81\x11a\x06\x9FW`\x05\x1B` \x01\x90V[\x90\x80`\x1F\x83\x01\x12\x15a\x03\xCFW\x815a\x07\xD0\x81a\x07\xA2V[\x92a\x07\xDE`@Q\x94\x85a\x06~V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03\xCFW` \x01\x90[\x82\x82\x10a\x08\x06WPPP\x90V[` \x80\x91\x835a\x08\x15\x81a\x06YV[\x81R\x01\x91\x01\x90a\x07\xF9V[\x90\x80`\x1F\x83\x01\x12\x15a\x03\xCFW\x815a\x087\x81a\x07\xA2V[\x92a\x08E`@Q\x94\x85a\x06~V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03\xCFW` \x01\x90[\x82\x82\x10a\x08mWPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\x08`V[\x90\x80`\x1F\x83\x01\x12\x15a\x03\xCFW\x815a\x08\x94\x81a\x07\xA2V[\x92a\x08\xA2`@Q\x94\x85a\x06~V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x91\x83\x83\x11a\x03\xCFW` \x82\x01\x90[\x83\x82\x10a\x08\xCEWPPPPP\x90V[\x815`\x01`\x01`@\x1B\x03\x81\x11a\x03\xCFW` \x91a\x08\xF0\x87\x84\x80\x94\x88\x01\x01a\x07\x06V[\x81R\x01\x91\x01\x90a\x08\xBFV[`\x80`\x03\x19\x82\x01\x12a\x03\xCFW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03\xCFW\x81a\t%\x91`\x04\x01a\x07\xB9V[\x91`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03\xCFW\x82a\tD\x91`\x04\x01a\x08 V[\x91`D5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\xCFWa\tc\x91`\x04\x01a\x08}V[\x90`d5\x90V[4a\x03\xCFWa\tx6a\x08\xFBV[\x90\x92a\t\x86\x82\x85\x85\x84a+\xC2V[\x93a\t\x90\x85a-\x19V[P_\x80Q` aI\xED\x839\x81Q\x91RTa\t\xBA\x90`\x01`\x01`\xA0\x1B\x03\x16[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x93`@Q\x93cy=\x06I`\xE1\x1B\x85R` \x85`\x04\x81\x89Z\xFA\x94\x85\x15a\x0BbW_\x95a\x0B\x96W[P0``\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x18\x94` `@Q\x80\x92c\xB1\xC5\xF4'`\xE0\x1B\x82R\x81\x80a\n\x19\x8B\x89\x89\x8C`\x04\x86\x01a:\xEDV[\x03\x91Z\xFA\x90\x81\x15a\x0BbW_\x91a\x0BgW[Pa\n5\x87a\"@V[U_\x80Q` aI\xED\x839\x81Q\x91RTa\nW\x90`\x01`\x01`\xA0\x1B\x03\x16a\t\xAEV[\x90\x81;\x15a\x03\xCFW_\x80\x94a\n\x83\x87`@Q\x99\x8A\x97\x88\x96\x87\x95c\x08\xF2\xA0\xBB`\xE4\x1B\x87R`\x04\x87\x01a;2V[\x03\x92Z\xF1\x90\x81\x15a\x0BbWa\n\xA7\x92a\n\xA2\x92a\x0BHW[PBa4\x8DV[a4PV[\x90e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x15a\x0B9W\x7F\x9A.B\xFDg\"\x81=i\x11>}\0y\xD3\xD9@\x17\x14(\xDFss\xDF\x9C\x7Fv\x17\xCF\xDA(\x92a\x0B&\x83a\x0B\x07a\x05{\x95`\x01a\n\xED\x87a\"mV[\x01\x90e\xFF\xFF\xFF\xFF\xFF\xFF\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82T\x16\x17\x90UV[`@\x80Q\x85\x81Re\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16` \x83\x01R\x90\x91\x82\x91\x82\x01\x90V[\x03\x90\xA1`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[cHD%#`\xE1\x1B_R`\x04_\xFD[\x80a\x0BV_a\x0B\\\x93a\x06~V[\x80a\x03\xD3V[_a\n\x9BV[a$\xE1V[a\x0B\x89\x91P` =` \x11a\x0B\x8FW[a\x0B\x81\x81\x83a\x06~V[\x81\x01\x90a1^V[_a\n+V[P=a\x0BwV[a\x0B\xB0\x91\x95P` =` \x11a\x0B\x8FWa\x0B\x81\x81\x83a\x06~V[\x93_a\t\xE0V[e\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x03a\x03\xCFWV[`d5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x03\xCFWV[4a\x03\xCFW`\xC06`\x03\x19\x01\x12a\x03\xCFW`\x045a\x0B\xF7\x81a\x06YV[`$5\x90a\x0C\x04\x82a\x06YV[`D5a\x0C\x10\x81a\x0B\xB7V[a\x0C\x18a\x0B\xC7V[`\x845\x90`\xA45\x92_\x80Q` aJ-\x839\x81Q\x91RT\x95`\x01`\x01`@\x1B\x03a\x0C]a\x0CPa\x0CL\x8A`\xFF\x90`@\x1C\x16\x90V[\x15\x90V[\x98`\x01`\x01`@\x1B\x03\x16\x90V[\x16\x80\x15\x90\x81a\rvW[`\x01\x14\x90\x81a\rlW[\x15\x90\x81a\rcW[Pa\rTWa\x0C\xBC\x95\x87a\x0C\xB3`\x01`\x01`\x01`@\x1B\x03\x19_\x80Q` aJ-\x839\x81Q\x91RT\x16\x17_\x80Q` aJ-\x839\x81Q\x91RUV[a\r\x1FWa\"\x87V[a\x0C\xC2W\0[a\x0C\xEC`\xFF`@\x1B\x19_\x80Q` aJ-\x839\x81Q\x91RT\x16_\x80Q` aJ-\x839\x81Q\x91RUV[`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90\x80` \x81\x01a\x04\xAFV[a\rO`\x01`@\x1B`\xFF`@\x1B\x19_\x80Q` aJ-\x839\x81Q\x91RT\x16\x17_\x80Q` aJ-\x839\x81Q\x91RUV[a\"\x87V[c\xF9.\xE8\xA9`\xE0\x1B_R`\x04_\xFD[\x90P\x15_a\x0CyV[0;\x15\x91Pa\x0CqV[\x88\x91Pa\x0CgV[a\r\x876a\x08\xFBV[\x91a\r\x94\x83\x83\x83\x87a+\xC2V[\x93` a\r\xA2`0\x87a-\xD9V[Pa\r\xC2a\r\xAF\x87a\"mV[\x80T`\xFF`\xF0\x1B\x19\x16`\x01`\xF0\x1B\x17\x90UV[a\r\xCAa,)V[0`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x03a\x0E{W[P\x91\x84\x93\x91a\r\xEE\x93a\x05{\x96a>\xDDV[0a\r\xFAa\t\xAEa,)V[\x14\x15\x80a\x0ENW[a\x0E9W[`@Q\x81\x81R\x7Fq*\xE18?y\xAC\x85?\x8D\x88!Sw\x8E\x02`\xEF\x8F\x03\xB5\x04\xE2\x86n\x05\x93\xE0M+)\x1F\x90\x80` \x81\x01a\x0B&V[__\x80Q` aI\xCD\x839\x81Q\x91RUa\x0E\x07V[Pa\x0Eva\x0CL_\x80Q` aI\xCD\x839\x81Q\x91RT`\x01`\x01`\x80\x1B\x03\x81\x16\x90`\x80\x1C\x14\x90V[a\x0E\x02V[\x94\x90\x91\x93_[\x83Q\x81\x10\x15a\x0E\xD5W`\x01\x900a\x0E\xABa\t\xAEa\x0E\x9E\x84\x89a$\xA2V[Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x14a\x0E\xB7W[\x01a\x0E\x81V[a\x0E\xD0a\x0E\xC4\x82\x88a$\xA2V[Q\x89\x81Q\x91\x01 a/\xA7V[a\x0E\xB1V[P\x90\x94P\x92\x90a\x05{a\r\xDCV[4a\x03\xCFW` 6`\x03\x19\x01\x12a\x03\xCFW` a\x04\r`\x045a$\xBBV[4a\x03\xCFW_6`\x03\x19\x01\x12a\x03\xCFW` `@Q\x7F>\x83\x94fSW_\x9A9\0^\x15E\x18V)\xE9'6\xB7R\x8A\xB2\x0C\xA3\x81o1T$\xA8\x11\x81R\xF3[4a\x03\xCFW_6`\x03\x19\x01\x12a\x03\xCFW` e\xFF\xFF\xFF\xFF\xFF\xFF_\x80Q` aH\xCD\x839\x81Q\x91RT\x16`@Q\x90\x81R\xF3[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x08\x11\x15a\x0F\x8AWV[a\x0FlV[`\x08\x81\x10\x15a\x0F\x8AW`$RV[4a\x03\xCFW` 6`\x03\x19\x01\x12a\x03\xCFWa\x0F\xB9`\x045a0NV[`@Q`\x08\x82\x10\x15a\x0F\x8AW` \x91\x81R\xF3[4a\x03\xCFW`@6`\x03\x19\x01\x12a\x03\xCFW` `\xFFa\x10!`$5`\x045a\x0F\xF3\x82a\x06YV[_R_\x80Q` aI\xAD\x839\x81Q\x91R\x84R`\x03`@_ \x01\x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03\xCFWa\x10;6a\x08\xFBV[\x91a\x10H\x83\x83\x83\x87a+\xC2V[a\x10Q\x81a-YV[P_\x90\x81R_\x80Q` aH\xAD\x839\x81Q\x91R` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x163\x03a\x11xWa\x10\x84\x93a+\xC2V[a\x10\x8F`;\x82a-\xD9V[Pa\x10\xB1a\x10\x9C\x82a\"mV[\x80T`\x01`\x01`\xF8\x1B\x03\x16`\x01`\xF8\x1B\x17\x90UV[`@Q\x81\x81R\x7Fx\x9C\xF5[\xE9\x80s\x9D\xAD\x1D\x06\x99\xB9;X\xE8\x06\xB5\x1C\x9D\x96a\x9B\xFA\x8F\xE0\xA2\x8A\xBA\xA7\xB3\x0C\x90` \x90\xA1a\x10\xE6\x81a\"@V[T\x90\x81a\x10\xF9W[`@Q\x90\x81R` \x90\xF3[_\x80Q` aI\xED\x839\x81Q\x91RTa\x11\x1A\x90`\x01`\x01`\xA0\x1B\x03\x16a\t\xAEV[\x80;\x15a\x03\xCFW`@Qc\xC4\xD2R\xF5`\xE0\x1B\x81R`\x04\x81\x01\x93\x90\x93R_\x90\x83\x90`$\x90\x82\x90\x84\x90Z\xF1\x91\x82\x15a\x0BbWa\x05{\x92a\x11dW[P_a\x11^\x82a\"@V[Ua\x10\xEEV[\x80a\x0BV_a\x11r\x93a\x06~V[_a\x11SV[c#=\x98\xE3`\xE0\x1B_R3`\x04R`$_\xFD[4a\x03\xCFW_6`\x03\x19\x01\x12a\x03\xCFWa\x05{a\x11\xA6a%'V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x04\xE3V[`@6`\x03\x19\x01\x12a\x03\xCFW`\x045a\x11\xD2\x81a\x06YV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03\xCFWa\x11\xF1\x906\x90`\x04\x01a\x07\x06V[\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x81\x14\x90\x81\x15a\x12\xE3W[Pa\x12\xD4Wa\x124a,fV[`@QcR\xD1\x90-`\xE0\x1B\x81R\x91` \x83`\x04\x81`\x01`\x01`\xA0\x1B\x03\x86\x16Z\xFA_\x93\x81a\x12\xB3W[Pa\x12\x80WcL\x9C\x8C\xE3`\xE0\x1B_R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04R`$_\xFD[_\xFD[\x90_\x80Q` aI\x8D\x839\x81Q\x91R\x83\x03a\x12\x9FWa\0 \x92Pa@~V[c*\x87Ri`\xE2\x1B_R`\x04\x83\x90R`$_\xFD[a\x12\xCD\x91\x94P` =` \x11a\x0B\x8FWa\x0B\x81\x81\x83a\x06~V[\x92_a\x12\\V[cp>F\xDD`\xE1\x1B_R`\x04_\xFD[_\x80Q` aI\x8D\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x90P_a\x12'V[4a\x03\xCFW_6`\x03\x19\x01\x12a\x03\xCFW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x160\x03a\x12\xD4W` `@Q_\x80Q` aI\x8D\x839\x81Q\x91R\x81R\xF3[4a\x03\xCFW` 6`\x03\x19\x01\x12a\x03\xCFW`\x045_R_\x80Q` aI\xAD\x839\x81Q\x91R` R```@_ \x80T\x90`\x02`\x01\x82\x01T\x91\x01T\x90`@Q\x92\x83R` \x83\x01R`@\x82\x01R\xF3[4a\x03\xCFW_6`\x03\x19\x01\x12a\x03\xCFWa\x05{a\x11\xA6a%\xE9V[`$5\x90`\xFF\x82\x16\x82\x03a\x03\xCFWV[4a\x03\xCFW`@6`\x03\x19\x01\x12a\x03\xCFW` a\x04\r`\x045a\x13\xF5a\x13\xC4V[`@Q\x91a\x14\x03\x85\x84a\x06~V[_\x83R3\x90a1mV[\x91\x81`\x1F\x84\x01\x12\x15a\x03\xCFW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x03\xCFW` \x83\x81\x86\x01\x95\x01\x01\x11a\x03\xCFWV[4a\x03\xCFW`\xC06`\x03\x19\x01\x12a\x03\xCFW`\x045a\x14Va\x13\xC4V[\x90`D5\x90a\x14d\x82a\x06YV[`d5`\x01`\x01`@\x1B\x03\x81\x11a\x03\xCFWa\x14\x83\x906\x90`\x04\x01a\x14\rV[`\x845`\x01`\x01`@\x1B\x03\x81\x11a\x03\xCFWa\x14\xA2\x906\x90`\x04\x01a\x07\x06V[\x91`\xA45\x94`\x01`\x01`@\x1B\x03\x86\x11a\x03\xCFWa\x05{\x96a\x14\xCAa\x14\xD0\x976\x90`\x04\x01a\x07\x06V[\x95a&\x08V[`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[4a\x03\xCFW`\x806`\x03\x19\x01\x12a\x03\xCFW`\x045a\x14\xFCa\x13\xC4V[\x90`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03\xCFWa\x15\x1C\x906\x90`\x04\x01a\x14\rV[\x91\x90\x92`d5`\x01`\x01`@\x1B\x03\x81\x11a\x03\xCFWa\x10\xEE\x94a\x15Ea\x15M\x926\x90`\x04\x01a\x07\x06V[\x946\x91a\x06\xD0V[\x913\x90a3?V[4a\x03\xCFW` 6`\x03\x19\x01\x12a\x03\xCFW` a\x04\r`\x045a'JV[4a\x03\xCFW` 6`\x03\x19\x01\x12a\x03\xCFWa\0 `\x045a\x15\x93\x81a\x0B\xB7V[a\x15\x9Ba,fV[a4\x9AV[4a\x03\xCFW``6`\x03\x19\x01\x12a\x03\xCFW`\x045a\x15\xBCa\x13\xC4V[\x90`D5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\xCFW` \x92a\x15\xEDa\x15\xE6a\x04\r\x946\x90`\x04\x01a\x14\rV[6\x91a\x06\xD0V[\x913\x90a1mV[4a\x03\xCFW`\x806`\x03\x19\x01\x12a\x03\xCFW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03\xCFWa\x16%\x906\x90`\x04\x01a\x07\xB9V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03\xCFWa\x16D\x906\x90`\x04\x01a\x08 V[\x90`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03\xCFWa\x16d\x906\x90`\x04\x01a\x08}V[\x90`d5\x91`\x01`\x01`@\x1B\x03\x83\x11a\x03\xCFW6`#\x84\x01\x12\x15a\x03\xCFWa\x05{\x93a\x16\x9Da\x14\xD0\x946\x90`$\x81`\x04\x015\x91\x01a\x06\xD0V[\x92a(\xBAV[4a\x03\xCFW` 6`\x03\x19\x01\x12a\x03\xCFW`\x045a\x16\xC0\x81a\x06YV[`\x01\x80`\xA0\x1B\x03\x16_R\x7FZ\xB4,\xEDb\x88\x88%\x9C\x08\xAC\x98\xDB\x1E\xB0\xCFp/\xC1P\x13D1\x1D\x8B\x10\x0C\xD1\xBF\xE4\xBB\0` R` `@_ T`@Q\x90\x81R\xF3[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x17\x1AWPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x17\rV[\x91a\x17e\x90a\x17Wa\x05\x18\x97\x95\x96\x93`\x0F`\xF8\x1B\x86R`\xE0` \x87\x01R`\xE0\x86\x01\x90a\x04\xE3V[\x90\x84\x82\x03`@\x86\x01Ra\x04\xE3V[``\x83\x01\x94\x90\x94R`\x01`\x01`\xA0\x1B\x03\x16`\x80\x82\x01R_`\xA0\x82\x01R\x80\x83\x03`\xC0\x90\x91\x01Ra\x16\xFDV[4a\x03\xCFW_6`\x03\x19\x01\x12a\x03\xCFW\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0T\x15\x80a\x186W[\x15a\x17\xF9Wa\x17\xD5a \xC6V[a\x17\xDDa!\x93V[\x90a\x05{a\x17\xE9a)\xD4V[`@Q\x93\x84\x930\x91F\x91\x86a\x170V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x11RT\r\xCCL\x8E\x88\x15[\x9A[\x9A]\x1AX[\x1A^\x99Y`Z\x1B`D\x82\x01R`d\x90\xFD[P\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\x01T\x15a\x17\xC8V[4a\x03\xCFW`\x806`\x03\x19\x01\x12a\x03\xCFW`\x045a\x18{a\x13\xC4V[\x90`D5\x91a\x18\x89\x83a\x06YV[`d5`\x01`\x01`@\x1B\x03\x81\x11a\x03\xCFWa\x0CLa\x18\xAEa\x19[\x926\x90`\x04\x01a\x07\x06V[`\x01`\x01`\xA0\x1B\x03\x86\x16_\x90\x81R\x7FZ\xB4,\xEDb\x88\x88%\x9C\x08\xAC\x98\xDB\x1E\xB0\xCFp/\xC1P\x13D1\x1D\x8B\x10\x0C\xD1\xBF\xE4\xBB\0` R`@\x90 \x80T`\x01\x81\x01\x90\x91Ua\x19U\x90`@Q` \x81\x01\x91\x7F\xF2\xAA\xD5P\xCFU\xF0E\xCB'\xE9\xC5Y\xF9\x88\x9F\xDF\xB6\xE6\xCD\xAA\x03#\x01\xD6\xEA9w\x84\xAEQ\xD7\x83R\x88`@\x83\x01R`\xFF\x88\x16``\x83\x01R`\x01\x80`\xA0\x1B\x03\x8A\x16`\x80\x83\x01R`\xA0\x82\x01R`\xA0\x81Ra\x19M`\xC0\x82a\x06~V[Q\x90 a1\x8AV[\x86a2\x16V[a\x19vW\x90a\x14\xD0\x91a\x05{\x93a\x19pa\x1B\xB0V[\x92a1mV[c\x94\xABl\x07`\xE0\x1B_R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04R`$_\xFD[4a\x03\xCFW_6`\x03\x19\x01\x12a\x03\xCFW` a\x19\xACa)\xEFV[e\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x91\x16\x81R\xF3[4a\x03\xCFW_6`\x03\x19\x01\x12a\x03\xCFW` `@Q`d\x81R\xF3[4a\x03\xCFW``6`\x03\x19\x01\x12a\x03\xCFW`\x045a\x19\xF4\x81a\x06YV[`D5`$5`\x01`\x01`@\x1B\x03\x82\x11a\x03\xCFWa\x1A\x18` \x926\x90`\x04\x01a\x07\x06V[P_\x80Q` aIM\x839\x81Q\x91RT`@Qc\x07H\xD65`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`\x04\x82\x01R`$\x81\x01\x92\x90\x92R\x90\x92\x83\x91`D\x91\x83\x91\x16Z\xFA\x80\x15a\x0BbWa\x05{\x91_\x91a\x1A{W[P`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[a\x1A\x94\x91P` =` \x11a\x0B\x8FWa\x0B\x81\x81\x83a\x06~V[_a\x1AjV[4a\x03\xCFW_6`\x03\x19\x01\x12a\x03\xCFW` `\x01`\x01`\xD0\x1B\x03a\x1A\xBCa8\xB5V[\x16`@Q\x90\x81R\xF3[4a\x03\xCFW` 6`\x03\x19\x01\x12a\x03\xCFW`\x045a\x1A\xE2\x81a\x06YV[a\x1A\xEAa,fV[_\x80Q` aI\xED\x839\x81Q\x91RT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82R\x90\x93\x16` \x84\x01\x81\x90R\x92\x7F\x08\xF7N\xA4n\xF7\x89Oe\xEA\xBF\xB5\xE6\xE6\x95\xDEw:\0\x0BG\xC5)\xABU\x91x\x06\x9B\"d\x01\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x19\x16\x17_\x80Q` aI\xED\x839\x81Q\x91RU\0[4a\x03\xCFW` 6`\x03\x19\x01\x12a\x03\xCFW` `@Q`\x01\x81R\xF3[4a\x03\xCFW` 6`\x03\x19\x01\x12a\x03\xCFW` a\x04\r`\x045_R_\x80Q` aH\xAD\x839\x81Q\x91R` Re\xFF\xFF\xFF\xFF\xFF\xFF`\x01`@_ \x01T\x16\x90V[`@Q\x90a\x1B\xBF` \x83a\x06~V[_\x82RV[4a\x03\xCFW_6`\x03\x19\x01\x12a\x03\xCFWa\x05{`@Qa\x1B\xE5`@\x82a\x06~V[`\x05\x81Rd\x03R\xE3\x02\xE3`\xDC\x1B` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x04\xE3V[4a\x03\xCFW_6`\x03\x19\x01\x12a\x03\xCFW` _\x80Q` aIm\x839\x81Q\x91RT`@Q\x90\x81R\xF3[4a\x03\xCFW`\xA06`\x03\x19\x01\x12a\x03\xCFWa\x1CP`\x045a\x06YV[a\x1C[`$5a\x06YV[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03\xCFWa\x1Cz\x906\x90`\x04\x01a\x08 V[P`d5`\x01`\x01`@\x1B\x03\x81\x11a\x03\xCFWa\x1C\x9A\x906\x90`\x04\x01a\x08 V[P`\x845`\x01`\x01`@\x1B\x03\x81\x11a\x03\xCFWa\x1C\xBA\x906\x90`\x04\x01a\x07\x06V[Pa\x05{a\x1C\xC6a*uV[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[4a\x03\xCFW` 6`\x03\x19\x01\x12a\x03\xCFW` a\x04\r`\x045a*\xA0V[``6`\x03\x19\x01\x12a\x03\xCFW`\x045a\x1D\x17\x81a\x06YV[`$5`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03\xCFWa\0 \x92_\x92a\x1D@\x84\x936\x90`\x04\x01a\x14\rV[\x91\x90a\x1DJa,fV[\x82`@Q\x93\x84\x92\x837\x81\x01\x85\x81R\x03\x92Z\xF1a\x1Dda*\xFFV[\x90a9\x11V[4a\x03\xCFW` a\x04\ra\x1D}6a\x08\xFBV[\x92\x91\x90\x91a+\xC2V[4a\x03\xCFW_6`\x03\x19\x01\x12a\x03\xCFW_\x80Q` aI\xED\x839\x81Q\x91RT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03\xCFW_6`\x03\x19\x01\x12a\x03\xCFWa\x05{`@Qa\x1D\xDB`@\x82a\x06~V[` \x81R\x7Fsupport=bravo&quorum=for,abstain` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x04\xE3V[4a\x03\xCFW_6`\x03\x19\x01\x12a\x03\xCFW` `@Q\x7F\xF2\xAA\xD5P\xCFU\xF0E\xCB'\xE9\xC5Y\xF9\x88\x9F\xDF\xB6\xE6\xCD\xAA\x03#\x01\xD6\xEA9w\x84\xAEQ\xD7\x81R\xF3[4a\x03\xCFW` 6`\x03\x19\x01\x12a\x03\xCFW`\x045c\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x03\xCFWa\0 \x90a\x1E\x80a,fV[a9\x1EV[4a\x03\xCFW`@6`\x03\x19\x01\x12a\x03\xCFW`\x045a\x1E\xA2\x81a\x06YV[` `$5_`@Qa\x1E\xB5\x84\x82a\x06~V[R_\x80Q` aIM\x839\x81Q\x91RT`@Qc\x07H\xD65`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`\x04\x82\x01R`$\x81\x01\x92\x90\x92R\x90\x92\x83\x91`D\x91\x83\x91\x16Z\xFA\x80\x15a\x0BbWa\x05{\x91_\x91a\x1A{WP`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[4a\x03\xCFW` 6`\x03\x19\x01\x12a\x03\xCFWa\0 `\x045a\x1F6a,fV[a9\xB7V[4a\x03\xCFW`\xA06`\x03\x19\x01\x12a\x03\xCFWa\x1FW`\x045a\x06YV[a\x1Fb`$5a\x06YV[`\x845`\x01`\x01`@\x1B\x03\x81\x11a\x03\xCFWa\x1F\x81\x906\x90`\x04\x01a\x07\x06V[Pa\x1F\x8Aa,)V[0`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x03a\x07\x93W`@Qc\xF2:na`\xE0\x1B\x81R` \x90\xF3[4a\x03\xCFW` 6`\x03\x19\x01\x12a\x03\xCFW`$`\x045_\x80Q` aIM\x839\x81Q\x91RT`@Qc#\x94\xE7\xA3`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R\x92` \x91\x84\x91\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x0BbW_\x92a 5W[Pa \x12\x90a'JV[\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x15a 0Wa\x05{\x90`d\x90\x04a\x14\xD0V[a'\x19V[a \x12\x91\x92Pa S\x90` =` \x11a\x0B\x8FWa\x0B\x81\x81\x83a\x06~V[\x91\x90a \x08V[4a\x03\xCFW_6`\x03\x19\x01\x12a\x03\xCFW_\x80Q` aIM\x839\x81Q\x91RT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a \xBCW[` \x83\x10\x14a \xA8WV[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a \x9DV[`@Q\x90_\x82_\x80Q` aI\r\x839\x81Q\x91RT\x91a \xE5\x83a \x8EV[\x80\x83R\x92`\x01\x81\x16\x90\x81\x15a!tWP`\x01\x14a!\tW[a\x06\xB3\x92P\x03\x83a\x06~V[P_\x80Q` aI\r\x839\x81Q\x91R_\x90\x81R\x90\x91\x7FB\xAD]>\x1F.np\xED\xCFm\x99\x1B\x8A0#\xD3\xFC\xA8\x04z\x13\x15\x92\xF9\xED\xB9\xFD\x9B\x89\xD5}[\x81\x83\x10a!XWPP\x90` a\x06\xB3\x92\x82\x01\x01a \xFDV[` \x91\x93P\x80`\x01\x91T\x83\x85\x89\x01\x01R\x01\x91\x01\x90\x91\x84\x92a!@V[` \x92Pa\x06\xB3\x94\x91P`\xFF\x19\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01a \xFDV[`@Q\x90_\x82_\x80Q` aI-\x839\x81Q\x91RT\x91a!\xB2\x83a \x8EV[\x80\x83R\x92`\x01\x81\x16\x90\x81\x15a!tWP`\x01\x14a!\xD5Wa\x06\xB3\x92P\x03\x83a\x06~V[P_\x80Q` aI-\x839\x81Q\x91R_\x90\x81R\x90\x91\x7F_\x9C\xE3H\x15\xF8\xE1\x141\xC7\xBBu\xA8\xE6\x88j\x91G\x8F\x7F\xFC\x1D\xBB\n\x98\xDC$\x0F\xDD\xD7ku[\x81\x83\x10a\"$WPP\x90` a\x06\xB3\x92\x82\x01\x01a \xFDV[` \x91\x93P\x80`\x01\x91T\x83\x85\x89\x01\x01R\x01\x91\x01\x90\x91\x84\x92a\"\x0CV[_R\x7F\rX)x{\x8B\xEF\xDB\xC6\x04N\xF7E}\x8A\x95\xC2\xA0K\xC9\x9254\x9F\x1A!,\x06>Y\xD4\x01` R`@_ \x90V[_R_\x80Q` aH\xAD\x839\x81Q\x91R` R`@_ \x90V[\x92\x90\x94\x93\x91`@Qa\"\x9A`@\x82a\x06~V[`\x0E\x81Rm*0\xB73\xB62\xA3\xB7\xBB2\xB977\xB9`\x91\x1B` \x82\x01Ra\"\xBDa;\x7FV[a\"\xC5a%\xE9V[a\"\xCDa;\x7FV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11a\x06\x9FWa\"\xFD\x81a\"\xF8_\x80Q` aI\r\x839\x81Q\x91RTa \x8EV[a;\xAAV[` `\x1F\x82\x11`\x01\x14a#\xDCW\x93a#\xADa#\xB2\x94a#Xa#\xC9\x9C\x9B\x99\x95a#D\x86a#\xBF\x9B\x97a#\xC4\x9E\x9B_\x91a#\xD1W[P\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[_\x80Q` aI\r\x839\x81Q\x91RUa<\xB1V[a#\x80_\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0UV[a#\xA8_\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\x01UV[a=\xD0V[a.\x17V[a#\xBAa;\x7FV[a.3V[a.}V[a/,V[a\x06\xB3a;\x7FV[\x90P\x85\x01Q_a#1V[_\x80Q` aI\r\x839\x81Q\x91R_R`\x1F\x19\x82\x16\x90\x7FB\xAD]>\x1F.np\xED\xCFm\x99\x1B\x8A0#\xD3\xFC\xA8\x04z\x13\x15\x92\xF9\xED\xB9\xFD\x9B\x89\xD5}\x91_[\x81\x81\x10a$vWP\x94a#Xa#\xC9\x9C\x9B\x99\x95`\x01\x86a#\xC4\x9D\x9A\x96a#\xAD\x96a#\xBF\x9D\x99a#\xB2\x9C\x10a$^W[PP\x81\x1B\x01_\x80Q` aI\r\x839\x81Q\x91RUa<\xB1V[\x86\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80a$EV[\x91\x92` `\x01\x81\x92\x86\x8A\x01Q\x81U\x01\x94\x01\x92\x01a$\x16V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80Q\x82\x10\x15a$\xB6W` \x91`\x05\x1B\x01\x01\x90V[a$\x8EV[_R_\x80Q` aH\xAD\x839\x81Q\x91R` Re\xFF\xFF\xFF\xFF\xFF\xFF`@_ T`\xA0\x1C\x16\x90V[`@Q=_\x82>=\x90\xFD[`@Q\x90a$\xFB`@\x83a\x06~V[`\x1D\x82R\x7Fmode=blocknumber&from=default\0\0\0` \x83\x01RV[_\x80Q` aIM\x839\x81Q\x91RT`@QcK\xF5\xD7\xE9`\xE0\x1B\x81R\x90_\x90\x82\x90`\x04\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA_\x91\x81a%nW[Pa\x05\x18WPa\x05\x18a$\xECV[\x90\x91P=\x80_\x83>a%\x80\x81\x83a\x06~V[\x81\x01\x90` \x81\x83\x03\x12a\x03\xCFW\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\xCFW\x01\x81`\x1F\x82\x01\x12\x15a\x03\xCFW\x80Q\x90a%\xB6\x82a\x06\xB5V[\x92a%\xC4`@Q\x94\x85a\x06~V[\x82\x84R` \x83\x83\x01\x01\x11a\x03\xCFW\x81_\x92` \x80\x93\x01\x83\x86\x01^\x83\x01\x01R\x90_a%`V[`@Q\x90a%\xF8`@\x83a\x06~V[`\x01\x82R`1`\xF8\x1B` \x83\x01RV[\x93\x90\x92\x91\x96\x95a\x0CLa&\xE2\x91a&\xDC\x8Aa&[\x81`\x01\x80`\xA0\x1B\x03\x16_R\x7FZ\xB4,\xEDb\x88\x88%\x9C\x08\xAC\x98\xDB\x1E\xB0\xCFp/\xC1P\x13D1\x1D\x8B\x10\x0C\xD1\xBF\xE4\xBB\0` R`@_ \x80T\x90`\x01\x82\x01\x90U\x90V[a&f6\x88\x8Aa\x06\xD0V[` \x81Q\x91\x01 \x8BQ` \x8D\x01 \x90`@Q\x92` \x84\x01\x94\x7F>\x83\x94fSW_\x9A9\0^\x15E\x18V)\xE9'6\xB7R\x8A\xB2\x0C\xA3\x81o1T$\xA8\x11\x86R\x8D`@\x86\x01R`\xFF\x8D\x16``\x86\x01R`\x01\x80`\xA0\x1B\x03\x16`\x80\x85\x01R`\xA0\x84\x01R`\xC0\x83\x01R`\xE0\x82\x01R`\xE0\x81Ra\x19Ma\x01\0\x82a\x06~V[\x8Aa2\x16V[a&\xFDWa\x05\x18\x95\x96\x91a&\xF7\x916\x91a\x06\xD0V[\x92a3?V[c\x94\xABl\x07`\xE0\x1B_R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_\x19\x81\x01\x91\x90\x82\x11a 0WV[`'\x19\x81\x01\x91\x90\x82\x11a 0WV[_\x80Q` aH\xED\x839\x81Q\x91RT\x90_\x19\x82\x01\x82\x81\x11a 0W\x82\x11\x15a$\xB6W_\x80Q` aH\xED\x839\x81Q\x91R_R\x7F);\x01\x81\xC8\xEC4\xCD2R\xE7Ah\x9B\xDC!\xB7\x0E\xE7\xA0\xECv!d9\x03Z\\7\x18\x90\x9A\x82\x01T\x81e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x11\x15a(\xB1WPa'\xBA\x90a4PV[_\x82\x91`\x05\x84\x11a(4W[a'\xD0\x93PaC\x9EV[\x80a'\xDAWP_\x90V[a((a(!a'\xECa\x05\x18\x93a'-V[_\x80Q` aH\xED\x839\x81Q\x91R_R\x7F);\x01\x81\xC8\xEC4\xCD2R\xE7Ah\x9B\xDC!\xB7\x0E\xE7\xA0\xECv!d9\x03Z\\7\x18\x90\x9B\x01\x90V[T`0\x1C\x90V[`\x01`\x01`\xD0\x1B\x03\x16\x90V[\x91\x92a(?\x81aB@V[\x81\x03\x90\x81\x11a 0Wa'\xD0\x93_\x80Q` aH\xED\x839\x81Q\x91R_Re\xFF\xFF\xFF\xFF\xFF\xFF\x82\x7F);\x01\x81\xC8\xEC4\xCD2R\xE7Ah\x9B\xDC!\xB7\x0E\xE7\xA0\xECv!d9\x03Z\\7\x18\x90\x9B\x01T\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x10_\x14a(\x9FWP\x91a'\xC6V[\x92\x91Pa(\xAB\x90a4\x7FV[\x90a'\xC6V[\x91PP`0\x1C\x90V[\x91\x93\x92\x90\x93a(\xC9\x823a5\x1FV[\x15a)\xC1W_\x80Q` aIm\x839\x81Q\x91RT\x94\x85a(\xF1W[a\x05\x18\x94\x95P3\x93a6\xEBV[_\x19e\xFF\xFF\xFF\xFF\xFF\xFFa)\x02a)\xEFV[\x16\x01e\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a 0We\xFF\xFF\xFF\xFF\xFF\xFF\x16\x95_`@Qa))` \x82a\x06~V[R_\x80Q` aIM\x839\x81Q\x91RT`@Qc\x07H\xD65`\xE3\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x98\x90\x98R` \x90\x88\x90`D\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x96\x87\x15a\x0BbW_\x97a)\xA0W[P\x80\x87\x10a)\x85WPa(\xE4V[ca!w\x0B`\xE1\x1B_R3`\x04R`$\x87\x90R`DR`d_\xFD[a)\xBA\x91\x97P` =` \x11a\x0B\x8FWa\x0B\x81\x81\x83a\x06~V[\x95_a)wV[c\xD9\xB3\x95W`\xE0\x1B_R3`\x04R`$_\xFD[`@Q\x90a)\xE3` \x83a\x06~V[_\x80\x83R6` \x84\x017V[_\x80Q` aIM\x839\x81Q\x91RT`@Qc$wk}`\xE2\x1B\x81R\x90` \x90\x82\x90`\x04\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA_\x91\x81a*8W[Pa\x05\x18WPa\x05\x18Ca4PV[\x90\x91P` \x81=` \x11a*mW[\x81a*T` \x93\x83a\x06~V[\x81\x01\x03\x12a\x03\xCFWQa*f\x81a\x0B\xB7V[\x90_a*)V[=\x91Pa*GV[_\x80Q` aI\xED\x839\x81Q\x91RT0`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x03a\x07\x93Wc\xBC\x19|\x81`\xE0\x1B\x90V[\x80_R_\x80Q` aH\xAD\x839\x81Q\x91R` Re\xFF\xFF\xFF\xFF\xFF\xFF`@_ T`\xA0\x1C\x16\x90_R_\x80Q` aH\xAD\x839\x81Q\x91R` Rc\xFF\xFF\xFF\xFF`@_ T`\xD0\x1C\x16\x01e\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a 0We\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[=\x15a+)W=\x90a+\x10\x82a\x06\xB5V[\x91a+\x1E`@Q\x93\x84a\x06~V[\x82R=_` \x84\x01>V[``\x90V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a+KWPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a+>V[\x90\x80` \x83Q\x91\x82\x81R\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a+\x95WPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a+\xB3`\x01\x93`\x1F\x19\x86\x82\x03\x01\x87R\x89Qa\x04\xE3V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a+\x86V[\x92\x90a,#\x91a,\x0Fa+\xFD\x94`@Q\x95\x86\x94a+\xEB` \x87\x01\x99`\x80\x8BR`\xA0\x88\x01\x90a+.V[\x86\x81\x03`\x1F\x19\x01`@\x88\x01R\x90a\x16\xFDV[\x84\x81\x03`\x1F\x19\x01``\x86\x01R\x90a+jV[\x90`\x80\x83\x01R\x03`\x1F\x19\x81\x01\x83R\x82a\x06~V[Q\x90 \x90V[_\x80Q` aI\xED\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[_\x80Q` aI\xED\x839\x81Q\x91RT0`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x03a\x07\x93WV[a,na,)V[3`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x03a,\xDAWa,\x87a,)V[0`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x03a,\x9AWV[a,\xA36a\x06\xB5V[a,\xB0`@Q\x91\x82a\x06~V[6\x81R` \x81\x01\x906_\x837_` 6\x83\x01\x01RQ\x90 [\x80a,\xD1a:&V[\x03a,\xC8W[PV[cG\tnG`\xE0\x1B_R3`\x04R`$_\xFD[`\x08\x81\x10\x15a\x0F\x8AW`\xFF`\x01\x91\x16\x1B\x90V[`\x04R`d\x91\x90`\x08\x81\x10\x15a\x0F\x8AW`$R_`DRV[a-\"\x81a0NV[\x90`\x10a-.\x83a,\xEDV[\x16\x15a-8WP\x90V[c1\xB7^M`\xE0\x1B_R`\x04Ra-O\x91Pa\x0F\x8FV[`\x10`DR`d_\xFD[a-b\x81a0NV[\x90`\x01a-n\x83a,\xEDV[\x16\x15a-xWP\x90V[c1\xB7^M`\xE0\x1B_R`\x04Ra-\x8F\x91Pa\x0F\x8FV[`\x01`DR`d_\xFD[a-\xA2\x81a0NV[\x90`\x02a-\xAE\x83a,\xEDV[\x16\x15a-\xB8WP\x90V[c1\xB7^M`\xE0\x1B_R`\x04Ra-\xCF\x91Pa\x0F\x8FV[`\x02`DR`d_\xFD[\x90a-\xE3\x82a0NV[\x91\x81a-\xEE\x84a,\xEDV[\x16\x15a-\xF9WPP\x90V[c1\xB7^M`\xE0\x1B_R`\x04Ra.\x0F\x82a\x0F\x8FV[`DR`d_\xFD[a\x06\xB3\x92\x91a\x1E\x80a\x1F6\x92a.+a;\x7FV[a\x15\x9Ba;\x7FV[a.;a;\x7FV[a.Ca;\x7FV[`\x01\x80`\xA0\x1B\x03\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B_\x80Q` aIM\x839\x81Q\x91RT\x16\x17_\x80Q` aIM\x839\x81Q\x91RUV[\x90a.\x86a;\x7FV[a.\x8Ea;\x7FV[`d\x82\x11a/\x14W`\x01`\x01`\xD0\x1B\x03a.\xA6a8\xB5V[\x16\x91a.\xB0a)\xEFV[\x92`\x01`\x01`\xD0\x1B\x03\x82\x11a\x04\xB4W\x91\x92\x7F\x05SGk\xF0.\xF2rn\x8C\xE5\xCE\xD7\x8Dc\xE2n`.J\"W\xB1\xF5YA\x8E$\xB4c9\x97\x92\x90a.\xF8\x90`\x01`\x01`\xD0\x1B\x03\x84\x16\x90aE[V[PP`@\x80Q\x91\x82R` \x82\x01\x92\x90\x92R\x90\x81\x90\x81\x01[\x03\x90\xA1V[Pc$>TE`\xE0\x1B_R`\x04R`d`$R`D_\xFD[a/4a;\x7FV[a/<a;\x7FV[_\x80Q` aI\xED\x839\x81Q\x91RT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82R\x90\x93\x16` \x84\x01\x81\x90R\x92\x7F\x08\xF7N\xA4n\xF7\x89Oe\xEA\xBF\xB5\xE6\xE6\x95\xDEw:\0\x0BG\xC5)\xABU\x91x\x06\x9B\"d\x01\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x19\x16\x17_\x80Q` aI\xED\x839\x81Q\x91RUV[_\x80Q` aI\xCD\x839\x81Q\x91RT\x90\x81`\x80\x1C`\x01`\x01`\x80\x1B\x03\x80`\x01\x83\x01\x16\x93\x16\x83\x14a0$W_R\x7F|q(\x97\x01M\xBEI\xC0E\xEF\x12\x99\xAA-_\x9Eg\xE4\x8E\xEAD\x03\xEF\xA2\x1F\x1E\x0F:\xC0\xCB\x03` R`@_ U_\x80Q` aI\xCD\x839\x81Q\x91R\x90`\x01`\x01`\x80\x1B\x03\x82T\x91\x81\x19\x90`\x80\x1B\x16\x91\x16\x17\x90UV[cNH{q_R`A` R`$`\x1C\xFD[\x90\x81` \x91\x03\x12a\x03\xCFWQ\x80\x15\x15\x81\x03a\x03\xCFW\x90V[a0W\x81a?sV[\x90a0a\x82a\x0F\x80V[`\x05\x82\x03a1ZWa0s\x91Pa\"@V[T_\x80Q` aI\xED\x839\x81Q\x91RTa0\x95\x90`\x01`\x01`\xA0\x1B\x03\x16a\t\xAEV[`@Qc,%\x8A\x9F`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R` \x81`$\x81\x85Z\xFA\x90\x81\x15a\x0BbW_\x91a1;W[P\x15a0\xCDWPP`\x05\x90V[`@Qc*\xB0\xF5)`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R` \x90\x82\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x0BbW_\x91a1\x0CW[P\x15a1\x07W`\x07\x90V[`\x02\x90V[a1.\x91P` =` \x11a14W[a1&\x81\x83a\x06~V[\x81\x01\x90a06V[_a0\xFCV[P=a1\x1CV[a1T\x91P` =` \x11a14Wa1&\x81\x83a\x06~V[_a0\xC0V[P\x90V[\x90\x81` \x91\x03\x12a\x03\xCFWQ\x90V[\x91a\x05\x18\x93\x91`@Q\x93a1\x82` \x86a\x06~V[_\x85Ra3?V[`B\x90a1\x95aG\xFDV[a1\x9DaHgV[`@Q\x90` \x82\x01\x92\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x84R`@\x83\x01R``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xA0\x81Ra1\xEE`\xC0\x82a\x06~V[Q\x90 \x90`@Q\x91a\x19\x01`\xF0\x1B\x83R`\x02\x83\x01R`\"\x82\x01R \x90V[`\x04\x11\x15a\x0F\x8AWV[\x91\x90\x82;a2QW\x90a2(\x91aA\x1DV[Pa22\x81a2\x0CV[\x15\x91\x82a2>WPP\x90V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x16\x14\x91\x90PV[\x91_\x92a2\x87a2\x95\x85\x94`@Q\x92\x83\x91` \x83\x01\x95c\x0B\x13]?`\xE1\x1B\x87R`$\x84\x01R`@`D\x84\x01R`d\x83\x01\x90a\x04\xE3V[\x03`\x1F\x19\x81\x01\x83R\x82a\x06~V[Q\x91Z\xFAa2\xA1a*\xFFV[\x81a2\xD1W[\x81a2\xB0WP\x90V[\x90Pa2\xCDc\x0B\x13]?`\xE1\x1B\x91` \x80\x82Q\x83\x01\x01\x91\x01a1^V[\x14\x90V[\x90P` \x81Q\x10\x15\x90a2\xA7V[\x93\x90\x92`\xFFa3\x0B\x93a\x05\x18\x97\x95\x87R\x16` \x86\x01R`@\x85\x01R`\xA0``\x85\x01R`\xA0\x84\x01\x90a\x04\xE3V[\x91`\x80\x81\x84\x03\x91\x01Ra\x04\xE3V[\x90\x92`\xFF`\x80\x93a\x05\x18\x96\x95\x84R\x16` \x83\x01R`@\x82\x01R\x81``\x82\x01R\x01\x90a\x04\xE3V[\x92\x91\x90\x92a3L\x81a-\x99V[Pa3V\x81a$\xBBV[_\x80Q` aIM\x839\x81Q\x91RT`@Qc\x07H\xD65`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x88\x16`\x04\x83\x01\x81\x90R`$\x83\x01\x94\x90\x94R\x92\x96\x92\x90\x91` \x91\x83\x91`D\x91\x83\x91\x16Z\xFA\x90\x81\x15a\x0BbWa3\xB9\x92\x85\x91_\x93a4/W[P\x84aAWV[\x94\x80Q\x15_\x14a3\xFCWPa3\xF6\x7F\xB8\xE18\x88}\n\xA1;\xABD~\x82\xDE\x9D\\\x17w\x04\x1E\xCD!\xCA6\xBA\x82O\xF1\xE6\xC0}\xDD\xA4\x93\x86`@Q\x94\x85\x94\x85a3\x19V[\x03\x90\xA2\x90V[a3\xF6\x90\x7F\xE2\xBA\xBF\xBA\xC5\x88\x9Ap\x9Bc\xBB\x7FY\x8B2N\x08\xBCZO\xB9\xECd\x7F\xB3\xCB\xC9\xEC\x07\xEB\x87\x12\x94\x87`@Q\x95\x86\x95\x86a2\xDFV[a4I\x91\x93P` =` \x11a\x0B\x8FWa\x0B\x81\x81\x83a\x06~V[\x91_a3\xB2V[e\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a4hWe\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[c\x06\xDF\xCCe`\xE4\x1B_R`0`\x04R`$R`D_\xFD[\x90`\x01\x82\x01\x80\x92\x11a 0WV[\x91\x90\x82\x01\x80\x92\x11a 0WV[\x7F\xC5e\xB0E@=\xC0<.\xEA\x82\xB8\x1A\x04e\xED\xAD\x9E.\x7F\xC4\xD9~\x11B\x1C \x9D\xA9=z\x93`@e\xFF\xFF\xFF\xFF\xFF\xFF\x80_\x80Q` aH\xCD\x839\x81Q\x91RT\x16\x93\x82Q\x94\x85R\x16\x92\x83` \x82\x01R\xA1e\xFF\xFF\xFF\xFF\xFF\xFF\x19_\x80Q` aH\xCD\x839\x81Q\x91RT\x16\x17_\x80Q` aH\xCD\x839\x81Q\x91RUV[\x90\x81Q\x81\x10\x15a$\xB6W\x01` \x01\x90V[\x81Q`4\x81\x10a5\xCEW`\x13\x19\x81\x84\x01\x01Q`\x01`\x01`\xA0\x1B\x03\x19\x16k\x1B\x91\xF1\xB2\x11\xF2\x11\x93Q\xB8Y\xF1`\xA3\x1B\x01a5\xCEW\x91_\x92a5\\\x81a';V[\x91[\x81\x83\x10a5yWPPP`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x16\x14\x90V[\x90\x91\x93a5\x9Fa5\x9Aa5\x8C\x87\x85a5\x0EV[Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[aD#V[\x90\x15a5\xC3W`\x01\x91`\xFF\x90`\x04\x1B`\x10`\x01`\xA0\x1B\x03\x16\x91\x16\x17\x94\x01\x91\x90a5^V[PPPPPP`\x01\x90V[PPP`\x01\x90V[\x90a5\xE0\x82a\x07\xA2V[a5\xED`@Q\x91\x82a\x06~V[\x82\x81R\x80\x92a5\xFE`\x1F\x19\x91a\x07\xA2V[\x01\x90_[\x82\x81\x10a6\x0EWPPPV[\x80``` \x80\x93\x85\x01\x01R\x01a6\x02V[\x95\x99\x98\x96\x97\x94\x93\x91\x92a6`\x93a6R\x92\x88R`\x01\x80`\xA0\x1B\x03\x16` \x88\x01Ra\x01 `@\x88\x01Ra\x01 \x87\x01\x90a+.V[\x90\x85\x82\x03``\x87\x01Ra\x16\xFDV[\x96\x83\x88\x03`\x80\x85\x01R\x81Q\x80\x89R` \x89\x01\x90` \x80\x82`\x05\x1B\x8C\x01\x01\x94\x01\x91_\x90[\x82\x82\x10a6\xBFWPPPPa\x05\x18\x96\x97P\x90a6\xA6\x91\x84\x82\x03`\xA0\x86\x01Ra+jV[\x93`\xC0\x83\x01R`\xE0\x82\x01Ra\x01\0\x81\x84\x03\x91\x01Ra\x04\xE3V[\x90\x91\x92\x94` \x80a6\xDD`\x01\x93\x8F`\x1F\x19\x90\x82\x03\x01\x86R\x89Qa\x04\xE3V[\x97\x01\x92\x01\x92\x01\x90\x92\x91a6\x83V[\x92\x90\x94\x93\x91\x94a7\x03\x82Q` \x84\x01 \x87\x83\x87a+\xC2V[\x95\x84Q\x82Q\x90\x81\x81\x14\x80\x15\x90a8\xAAW[\x80\x15a8\xA2W[a8\x83WPPe\xFF\xFF\xFF\xFF\xFF\xFFa7Ca74\x89a\"mV[T`\xA0\x1Ce\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x16a8fW\x91a/\x0F\x91\x7F}\x84\xA6&:\xE0\xD9\x8D3)\xBD{F\xBBN\x8Do\x98\xCD5\xA7\xAD\xB4\\'L\x8B\x7F\xD5\xEB\xD5\xE0\x95\x94\x93a7\xA7a7|a)\xEFV[e\xFF\xFF\xFF\xFF\xFF\xFFa7\xA0e\xFF\xFF\xFF\xFF\xFF\xFF_\x80Q` aH\xCD\x839\x81Q\x91RT\x16\x90V[\x91\x16a4\x8DV[\x90a7\xC6c\xFF\xFF\xFF\xFF_\x80Q` aH\xCD\x839\x81Q\x91RT`0\x1C\x16\x90V[a8Da7\xD2\x8Ca\"mV[\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x8A\x16\x17\x81Ua8\x1Ba7\xF7\x86a4PV[\x82Te\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16`\xA0\x91\x90\x91\x1Be\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x16\x17\x82UV[a8$\x83aD\xA0V[\x81Tc\xFF\xFF\xFF\xFF`\xD0\x1B\x19\x16`\xD0\x91\x90\x91\x1Bc\xFF\xFF\xFF\xFF`\xD0\x1B\x16\x17\x90UV[a8Xa8Q\x89Qa5\xD6V[\x91\x84a4\x8DV[\x93`@Q\x98\x89\x98\x8D\x8Aa6\x1FV[a\x12}\x87a8s\x81a0NV[c1\xB7^M`\xE0\x1B_R\x90a-\0V[\x91Qc\x04G\xB0]`\xE4\x1B_\x90\x81R`\x04\x93\x90\x93R`$R`DR`d\x90\xFD[P\x80\x15a7\x1BV[P\x82Q\x81\x14\x15a7\x14V[_\x80Q` aH\xED\x839\x81Q\x91RT\x80a8\xCEWP_\x90V[\x80_\x19\x81\x01\x11a 0W_\x80Q` aH\xED\x839\x81Q\x91R_R\x7F);\x01\x81\xC8\xEC4\xCD2R\xE7Ah\x9B\xDC!\xB7\x0E\xE7\xA0\xECv!d9\x03Z\\7\x18\x90\x9A\x01T`0\x1C\x90V[\x90\x91\x90a\x06\xB3WPaD\xCBV[c\xFF\xFF\xFF\xFF\x81\x16\x90\x81\x15a9\xA4Wi\xFF\xFF\xFF\xFF\0\0\0\0\0\0\x90\x7F~?\x7F\x07\x08\xA8M\xE9 06\xAB\xAAE\r\xCC\xC8Z\xD5\xFFR\xF7\x8C\x17\x0F>\xDBU\xCF^\x88(`@_\x80Q` aH\xCD\x839\x81Q\x91RT\x94\x81Q\x90c\xFF\xFF\xFF\xFF\x87`0\x1C\x16\x82R` \x82\x01R\xA1`0\x1B\x16\x90i\xFF\xFF\xFF\xFF\0\0\0\0\0\0\x19\x16\x17_\x80Q` aH\xCD\x839\x81Q\x91RUV[c\xF1\xCF\xBF\x05`\xE0\x1B_R_`\x04R`$_\xFD[_\x80Q` aIm\x839\x81Q\x91RT`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F\xCC\xB4]\xA8\xD5q~lEDiB\x97\xC4\xBA\\\xF1Q\xD4U\xC9\xBB\x0E\xD4\xFCz8A\x1B\xC0Ta\x91\xA1_\x80Q` aIm\x839\x81Q\x91RUV[\x81\x15a:\x12W\x04\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x80Q` aI\xCD\x839\x81Q\x91RT\x90`\x01`\x01`\x80\x1B\x03\x82\x16\x91`\x80\x1C\x82\x14a:\xDBW\x81_R\x7F|q(\x97\x01M\xBEI\xC0E\xEF\x12\x99\xAA-_\x9Eg\xE4\x8E\xEAD\x03\xEF\xA2\x1F\x1E\x0F:\xC0\xCB\x03` R`@_ T\x91`\x01`\x01`\x80\x1B\x03\x81\x16_R\x7F|q(\x97\x01M\xBEI\xC0E\xEF\x12\x99\xAA-_\x9Eg\xE4\x8E\xEAD\x03\xEF\xA2\x1F\x1E\x0F:\xC0\xCB\x03` R_`@\x81 U`\x01`\x01`\x80\x1B\x03\x80`\x01_\x80Q` aI\xCD\x839\x81Q\x91R\x93\x01\x16\x16`\x01`\x01`\x80\x1B\x03\x19\x82T\x16\x17\x90UV[cNH{q_R`1` R`$`\x1C\xFD[\x94\x93\x92a;\x19`\x80\x93a;\x0Ba;'\x94`\xA0\x8AR`\xA0\x8A\x01\x90a+.V[\x90\x88\x82\x03` \x8A\x01Ra\x16\xFDV[\x90\x86\x82\x03`@\x88\x01Ra+jV[\x93_``\x82\x01R\x01RV[\x91\x92a;a`\xA0\x94a;Sa;o\x94\x99\x98\x97\x99`\xC0\x87R`\xC0\x87\x01\x90a+.V[\x90\x85\x82\x03` \x87\x01Ra\x16\xFDV[\x90\x83\x82\x03`@\x85\x01Ra+jV[\x94_``\x83\x01R`\x80\x82\x01R\x01RV[`\xFF_\x80Q` aJ-\x839\x81Q\x91RT`@\x1C\x16\x15a;\x9BWV[c\x1A\xFC\xD7\x9F`\xE3\x1B_R`\x04_\xFD[`\x1F\x81\x11a;\xB6WPPV[_\x80Q` aI\r\x839\x81Q\x91R_R` _ \x90` `\x1F\x84\x01`\x05\x1C\x83\x01\x93\x10a;\xFCW[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a;\xF1WPPV[_\x81U`\x01\x01a;\xE6V[\x90\x91P\x81\x90a;\xDDV[`\x1F\x81\x11a<\x12WPPV[_\x80Q` aJ\r\x839\x81Q\x91R_R` _ \x90` `\x1F\x84\x01`\x05\x1C\x83\x01\x93\x10a<XW[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a<MWPPV[_\x81U`\x01\x01a<BV[\x90\x91P\x81\x90a<9V[`\x1F\x82\x11a<oWPPPV[_R` _ \x90` `\x1F\x84\x01`\x05\x1C\x83\x01\x93\x10a<\xA7W[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a<\x9CWPPV[_\x81U`\x01\x01a<\x91V[\x90\x91P\x81\x90a<\x88V[\x90\x81Q`\x01`\x01`@\x1B\x03\x81\x11a\x06\x9FWa<\xF0\x81a<\xDD_\x80Q` aI-\x839\x81Q\x91RTa \x8EV[_\x80Q` aI-\x839\x81Q\x91Ra<bV[` \x92`\x1F\x82\x11`\x01\x14a=<Wa= \x92\x93\x82\x91_\x92a=1W[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[_\x80Q` aI-\x839\x81Q\x91RUV[\x01Q\x90P_\x80a=\x0CV[_\x80Q` aI-\x839\x81Q\x91R_R`\x1F\x19\x82\x16\x93\x7F_\x9C\xE3H\x15\xF8\xE1\x141\xC7\xBBu\xA8\xE6\x88j\x91G\x8F\x7F\xFC\x1D\xBB\n\x98\xDC$\x0F\xDD\xD7ku\x91_[\x86\x81\x10a=\xB8WP\x83`\x01\x95\x96\x10a=\xA0W[PPP\x81\x1B\x01_\x80Q` aI-\x839\x81Q\x91RUV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a=\x89V[\x91\x92` `\x01\x81\x92\x86\x85\x01Q\x81U\x01\x94\x01\x92\x01a=vV[\x90a=\xD9a;\x7FV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11a\x06\x9FWa>\t\x81a>\x04_\x80Q` aJ\r\x839\x81Q\x91RTa \x8EV[a<\x06V[` \x92`\x1F\x82\x11`\x01\x14a>IWa>8\x92\x93\x82\x91_\x92a=1WPP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[_\x80Q` aJ\r\x839\x81Q\x91RUV[_\x80Q` aJ\r\x839\x81Q\x91R_R`\x1F\x19\x82\x16\x93\x7F\xDA\x13\xDD\xA7X:9\xA3\xCDs\xE8\x83\x05)\xC7`\x83r(\xFAF\x83u,\x82;\x17\xE1\x05H\xAA\xD5\x91_[\x86\x81\x10a>\xC5WP\x83`\x01\x95\x96\x10a>\xADW[PPP\x81\x1B\x01_\x80Q` aJ\r\x839\x81Q\x91RUV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a>\x96V[\x91\x92` `\x01\x81\x92\x86\x85\x01Q\x81U\x01\x94\x01\x92\x01a>\x83V[_\x80Q` aI\xED\x839\x81Q\x91RT\x90\x94\x91\x92\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x190``\x1B\x16\x82;\x15a\x03\xCFWa?;_\x95`@Q\x97\x88\x96\x87\x95\x86\x95c\xE3\x835\xE5`\xE0\x1B\x87R\x18\x92`\x04\x86\x01a:\xEDV[\x03\x914\x90Z\xF1\x80\x15a\x0BbWa?ZW[Pa?W_\x91a\"@V[UV[\x80a?f_\x80\x93a\x06~V[\x80\x03\x12a\x03\xCFW_a?LV[a?|\x81a\"mV[T`\xF8\x81\x90\x1C\x90`\xF0\x1C`\xFF\x16a@wWa@qWa?\x9A\x81a$\xBBV[\x80\x15a@]Wa?\xB6a?\xABa)\xEFV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x80\x91\x10\x15a@WWa?\xC7\x82a*\xA0V[\x10a?\xD2WP`\x01\x90V[a?\xDEa\x0CL\x82aFSV[\x80\x15a@(W[\x15a?\xF0WP`\x03\x90V[a@\x1A\x90_R_\x80Q` aH\xAD\x839\x81Q\x91R` Re\xFF\xFF\xFF\xFF\xFF\xFF`\x01`@_ \x01T\x16\x90V[a@#W`\x04\x90V[`\x05\x90V[Pa@Ra\x0CL\x82_R_\x80Q` aI\xAD\x839\x81Q\x91R` R`@_ `\x01\x81\x01T\x90T\x10\x90V[a?\xE5V[PP_\x90V[cj\xD0`u`\xE0\x1B_R`\x04\x82\x90R`$_\xFD[P`\x02\x90V[PP`\x07\x90V[\x90\x81;\x15a@\xFCW_\x80Q` aI\x8D\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x90\x91U\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;_\x80\xA2\x80Q\x15a@\xE4Wa,\xD7\x91aG\x19V[PP4a@\xEDWV[c\xB3\x98\x97\x9F`\xE0\x1B_R`\x04_\xFD[PcL\x9C\x8C\xE3`\xE0\x1B_\x90\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x04R`$\x90\xFD[\x81Q\x91\x90`A\x83\x03aAMWaAF\x92P` \x82\x01Q\x90```@\x84\x01Q\x93\x01Q_\x1A\x90aG6V[\x91\x92\x90\x91\x90V[PP_\x91`\x02\x91\x90V[aAx\x90\x92\x91\x92_R_\x80Q` aI\xAD\x839\x81Q\x91R` R`@_ \x90V[\x91`\x03\x83\x01aA\xA1aA\x9A\x83\x83\x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[T`\xFF\x16\x90V[aB$WaA\xC5`\xFF\x93\x92aA\xD2\x92\x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[\x80T`\xFF\x19\x16`\x01\x17\x90UV[\x16\x80aA\xE9WPaA\xE4\x82\x82Ta4\x8DV[\x90U\x90V[`\x01\x81\x03aB\0WP`\x01\x01aA\xE4\x82\x82Ta4\x8DV[`\x02\x03aB\x15W`\x02\x01aA\xE4\x82\x82Ta4\x8DV[c\x03Y\x9B\xE1`\xE1\x1B_R`\x04_\xFD[cq\xC6\xAFI`\xE0\x1B_R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04R`$_\xFD[`\x01\x81\x11\x15a\x05\x18W\x80`\x01`\x01`\x80\x1B\x82\x10\x15aCaW[aC\x07aB\xFDaB\xF3aB\xE9aB\xDFaB\xD5aB\xC4aC\x0E\x97`\x04\x8A`\x01`@\x1BaC\x13\x9C\x10\x15aCTW[d\x01\0\0\0\0\x81\x10\x15aCGW[b\x01\0\0\x81\x10\x15aC:W[a\x01\0\x81\x10\x15aC-W[`\x10\x81\x10\x15aC W[\x10\x15aC\x18W[`\x03\x02`\x01\x1C\x90V[aB\xCE\x81\x8Ba:\x08V[\x01`\x01\x1C\x90V[aB\xCE\x81\x8Aa:\x08V[aB\xCE\x81\x89a:\x08V[aB\xCE\x81\x88a:\x08V[aB\xCE\x81\x87a:\x08V[aB\xCE\x81\x86a:\x08V[\x80\x93a:\x08V[\x82\x11\x90V[\x90\x03\x90V[`\x01\x1BaB\xBBV[`\x04\x1C\x91`\x02\x1B\x91aB\xB4V[`\x08\x1C\x91`\x04\x1B\x91aB\xAAV[`\x10\x1C\x91`\x08\x1B\x91aB\x9FV[` \x1C\x91`\x10\x1B\x91aB\x93V[`@\x1C\x91` \x1B\x91aB\x85V[PPaC\x13aC\x0EaC\x07aB\xFDaB\xF3aB\xE9aB\xDFaB\xD5aB\xC4aC\x88\x8A`\x80\x1C\x90V[\x98P`\x01`@\x1B\x97PaBY\x96PPPPPPPV[\x90[\x82\x81\x10aC\xACWPP\x90V[\x90\x91\x80\x82\x16\x90\x80\x83\x18`\x01\x1C\x82\x01\x80\x92\x11a 0W_\x80Q` aH\xED\x839\x81Q\x91R_R\x7F);\x01\x81\xC8\xEC4\xCD2R\xE7Ah\x9B\xDC!\xB7\x0E\xE7\xA0\xECv!d9\x03Z\\7\x18\x90\x9B\x82\x01Te\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x85\x16\x10\x15aD\x11WP\x91[\x90aC\xA0V[\x92\x91PaD\x1D\x90a4\x7FV[\x90aD\x0BV[`\xF8\x1C\x90\x81`/\x10\x80aD\x96W[\x15aDCW`\x01\x91`/\x19\x01`\xFF\x16\x90V[\x81`@\x10\x80aD\x8CW[\x15aD_W`\x01\x91`6\x19\x01`\xFF\x16\x90V[\x81``\x10\x80aD\x82W[\x15aD{W`\x01\x91`V\x19\x01`\xFF\x16\x90V[_\x91P\x81\x90V[P`g\x82\x10aDiV[P`G\x82\x10aDMV[P`:\x82\x10aD1V[c\xFF\xFF\xFF\xFF\x81\x11aD\xB4Wc\xFF\xFF\xFF\xFF\x16\x90V[c\x06\xDF\xCCe`\xE4\x1B_R` `\x04R`$R`D_\xFD[\x80Q\x15aD\xDAW\x80Q\x90` \x01\xFD[c\xD6\xBD\xA2u`\xE0\x1B_R`\x04_\xFD[\x90\x81T`\x01`@\x1B\x81\x10\x15a\x06\x9FW`\x01\x81\x01\x80\x84U\x81\x10\x15a$\xB6Wa\x06\xB3\x92_R` _ \x01\x90aE9e\xFF\xFF\xFF\xFF\xFF\xFF\x82Q\x16\x83\x90e\xFF\xFF\xFF\xFF\xFF\xFF\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82T\x16\x17\x90UV[` \x01Q\x81Te\xFF\xFF\xFF\xFF\xFF\xFF\x16`0\x91\x90\x91\x1Be\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x17\x90UV[_\x80Q` aH\xED\x839\x81Q\x91RT\x91\x92\x91\x80\x15aF*Wa'\xECaE\x7F\x91a'-V[\x90\x81TaE\x9BaE\x94\x82e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x91`0\x1C\x90V[\x92e\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x16\x92\x16\x91\x80\x83\x11aF\x1BW\x86\x92\x03aE\xD9WaE\xD5\x92P\x90e\xFF\xFF\xFF\xFF\xFF\xFF\x82T\x91\x81\x19\x90`0\x1B\x16\x91\x16\x17\x90UV[\x91\x90V[PPaE\xD5\x90aE\xF8aE\xEAa\x06\xA4V[e\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x82RV[`\x01`\x01`\xD0\x1B\x03\x85\x16` \x82\x01R[_\x80Q` aH\xED\x839\x81Q\x91RaD\xE9V[c% `\x1D`\xE0\x1B_R`\x04_\xFD[PaFN\x90aF:aE\xEAa\x06\xA4V[`\x01`\x01`\xD0\x1B\x03\x84\x16` \x82\x01RaF\x08V[_\x91\x90V[\x80_R_\x80Q` aI\xAD\x839\x81Q\x91R` R`$aFv`@_ \x92a$\xBBV[_\x80Q` aIM\x839\x81Q\x91RT`@Qc#\x94\xE7\xA3`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R\x92` \x91\x84\x91\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x0BbW_\x92aF\xF4W[PaF\xC5\x90a'JV[\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x15a 0WaF\xEF\x90`d\x90\x04\x91`\x02`\x01\x82\x01T\x91\x01T\x90a4\x8DV[\x10\x15\x90V[aF\xC5\x91\x92PaG\x12\x90` =` \x11a\x0B\x8FWa\x0B\x81\x81\x83a\x06~V[\x91\x90aF\xBBV[_\x80a\x05\x18\x93` \x81Q\x91\x01\x84Z\xF4aG0a*\xFFV[\x91aG\xB8V[\x91\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11aG\xADW\x91` \x93`\x80\x92`\xFF_\x95`@Q\x94\x85R\x16\x86\x84\x01R`@\x83\x01R``\x82\x01R\x82\x80R`\x01Z\xFA\x15a\x0BbW_Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x15aG\xA3W\x90_\x90_\x90V[P_\x90`\x01\x90_\x90V[PPP_\x91`\x03\x91\x90V[\x90aG\xC3WPaD\xCBV[\x81Q\x15\x80aG\xF4W[aG\xD4WP\x90V[c\x99\x96\xB3\x15`\xE0\x1B_\x90\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x04R`$\x90\xFD[P\x80;\x15aG\xCCV[aH\x05a \xC6V[\x80Q\x90\x81\x15aH\x15W` \x01 \x90V[PP\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0T\x80\x15aHBW\x90V[P\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x90V[aHoa!\x93V[\x80Q\x90\x81\x15aH\x7FW` \x01 \x90V[PP\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\x01T\x80\x15aHBW\x90V\xFE|q(\x97\x01M\xBEI\xC0E\xEF\x12\x99\xAA-_\x9Eg\xE4\x8E\xEAD\x03\xEF\xA2\x1F\x1E\x0F:\xC0\xCB\x01\0\xD7al\x8F\xE2\x9Cl/\xBE\x1D\x0C[\xC8\xF2\xFA\xA4\xC3[Ctnp\xB2KMS'R\xAF\xFD\x01\xE7pq\x04!\xFD,\xADu\xAD\x82\x8Ca\xAA\x98\xF2\xD7}B:D\x0Bg\x87-\x0FeUAH\xE0\0\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\x02\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\x03;\xA4\x97rT\xE4\x15if\x10\xA4\x0E\xBF\"X\xDB\xFA\x0E\xC6\xA2\xFFd\xE8K\xFEq_\xF1iw\xCC\0\0\xD7al\x8F\xE2\x9Cl/\xBE\x1D\x0C[\xC8\xF2\xFA\xA4\xC3[Ctnp\xB2KMS'R\xAF\xFD\x006\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\xA1\xCE\xFA\x0FCf~\xF1'\xA2X\xE6s\xC9B\x02\xA7\x9Benb\x89\x951\xC47m\x87\xA7\xF3\x98\0|q(\x97\x01M\xBEI\xC0E\xEF\x12\x99\xAA-_\x9Eg\xE4\x8E\xEAD\x03\xEF\xA2\x1F\x1E\x0F:\xC0\xCB\x02\rX)x{\x8B\xEF\xDB\xC6\x04N\xF7E}\x8A\x95\xC2\xA0K\xC9\x9254\x9F\x1A!,\x06>Y\xD4\0|q(\x97\x01M\xBEI\xC0E\xEF\x12\x99\xAA-_\x9Eg\xE4\x8E\xEAD\x03\xEF\xA2\x1F\x1E\x0F:\xC0\xCB\0\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\xA1dsolcC\0\x08\x1A\0\n",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610022575b3615610018575f80fd5b610020612c44565b005b5f3560e01c806301ffc9a71461036157806302a251a31461035c57806306f3f9e61461035757806306fdde0314610352578063143489d01461034d578063150b7a0214610348578063160cbed71461034357806322f120de1461033e5780632656227d146103395780632d63f693146103345780632fe3e2611461032f5780633932abb11461032a5780633e4f49e6146103255780634385963214610320578063452115d61461031b5780634bf5d7e9146103165780634f1ef2861461031157806352d1902d1461030c578063544ffc9c1461030757806354fd4d501461030257806356781388146102fd5780635b8d0e0d146102f85780635f398a14146102f357806360c4247f146102ee57806379051887146102e95780637b3c71d3146102e45780637d5e81e2146102df5780637ecebe00146102da57806384b0196e146102d55780638ff262e3146102d057806391ddadf4146102cb57806397c3d334146102c65780639a802a6d146102c1578063a7713a70146102bc578063a890c910146102b7578063a9a95294146102b2578063ab58fb8e146102ad578063ad3cb1cc146102a8578063b58131b0146102a3578063bc197c811461029e578063c01f9e3714610299578063c28bc2fa14610294578063c59057e41461028f578063d33219b41461028a578063dd4e2ba514610285578063deaaa7cc14610280578063e540d01d1461027b578063eb9019d414610276578063ece40cc114610271578063f23a6e611461026c578063f8ce560a146102675763fc0c546a0361000e5761205a565b611fad565b611f3b565b611f17565b611e85565b611e53565b611e19565b611dba565b611d86565b611d6a565b611cff565b611ce1565b611c34565b611c0b565b611bc4565b611b71565b611b55565b611ac5565b611a9a565b6119d7565b6119bc565b611992565b61185f565b61178f565b6116a3565b6115f5565b6115a0565b611573565b611555565b6114e0565b61143a565b6113d4565b6113a9565b61135c565b611305565b6111ba565b61118b565b61102d565b610fcc565b610f9d565b610f3b565b610f01565b610ee3565b610d7e565b610bda565b61096a565b610721565b61060d565b61051b565b610415565b6103dd565b346103cf5760203660031901126103cf5760043563ffffffff60e01b81168091036103cf576020906332a2ad4360e11b81149081156103be575b81156103ad575b506040519015158152f35b6301ffc9a760e01b1490505f6103a2565b630271189760e51b8114915061039b565b5f80fd5b5f9103126103cf57565b346103cf575f3660031901126103cf57602061040d63ffffffff5f805160206148cd8339815191525460301c1690565b604051908152f35b346103cf5760203660031901126103cf57600435610431612c66565b606481116104cc576001600160d01b036104496138b5565b16906104536129ef565b916001600160d01b0382116104b4577f0553476bf02ef2726e8ce5ced78d63e26e602e4a2257b1f559418e24b463399792610498906001600160d01b0384169061455b565b505060408051918252602082019290925290819081015b0390a1005b506306dfcc6560e41b5f5260d060045260245260445ffd5b63243e544560e01b5f52600452606460245260445ffd5b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b9060206105189281815201906104e3565b90565b346103cf575f3660031901126103cf576040515f5f80516020614a0d833981519152546105478161208e565b80845290600181169081156105e9575060011461057f575b61057b8361056f8185038261067e565b60405191829182610507565b0390f35b5f80516020614a0d8339815191525f9081527fda13dda7583a39a3cd73e8830529c760837228fa4683752c823b17e10548aad5939250905b8082106105cf5750909150810160200161056f61055f565b9192600181602092548385880101520191019092916105b7565b60ff191660208086019190915291151560051b8401909101915061056f905061055f565b346103cf5760203660031901126103cf576004355f9081525f805160206148ad83398151915260209081526040909120546001600160a01b03166040516001600160a01b039091168152f35b6001600160a01b038116036103cf57565b634e487b7160e01b5f52604160045260245ffd5b90601f801991011681019081106001600160401b0382111761069f57604052565b61066a565b604051906106b360408361067e565b565b6001600160401b03811161069f57601f01601f191660200190565b9291926106dc826106b5565b916106ea604051938461067e565b8294818452818301116103cf578281602093845f960137010152565b9080601f830112156103cf57816020610518933591016106d0565b346103cf5760803660031901126103cf5761073d600435610659565b610748602435610659565b6064356001600160401b0381116103cf57610767903690600401610706565b50610770612c29565b306001600160a01b039091160361079357604051630a85bd0160e11b8152602090f35b637485328f60e11b5f5260045ffd5b6001600160401b03811161069f5760051b60200190565b9080601f830112156103cf5781356107d0816107a2565b926107de604051948561067e565b81845260208085019260051b8201019283116103cf57602001905b8282106108065750505090565b60208091833561081581610659565b8152019101906107f9565b9080601f830112156103cf578135610837816107a2565b92610845604051948561067e565b81845260208085019260051b8201019283116103cf57602001905b82821061086d5750505090565b8135815260209182019101610860565b9080601f830112156103cf578135610894816107a2565b926108a2604051948561067e565b81845260208085019260051b820101918383116103cf5760208201905b8382106108ce57505050505090565b81356001600160401b0381116103cf576020916108f087848094880101610706565b8152019101906108bf565b60806003198201126103cf576004356001600160401b0381116103cf5781610925916004016107b9565b916024356001600160401b0381116103cf578261094491600401610820565b91604435906001600160401b0382116103cf576109639160040161087d565b9060643590565b346103cf57610978366108fb565b909261098682858584612bc2565b9361099085612d19565b505f805160206149ed833981519152546109ba906001600160a01b03165b6001600160a01b031690565b936040519363793d064960e11b8552602085600481895afa948515610b62575f95610b96575b503060601b6bffffffffffffffffffffffff191618946020604051809263b1c5f42760e01b82528180610a198b89898c60048601613aed565b03915afa908115610b62575f91610b67575b50610a3587612240565b555f805160206149ed83398151915254610a57906001600160a01b03166109ae565b90813b156103cf575f8094610a8387604051998a97889687956308f2a0bb60e41b875260048701613b32565b03925af1908115610b6257610aa792610aa292610b48575b504261348d565b613450565b9065ffffffffffff821615610b39577f9a2e42fd6722813d69113e7d0079d3d940171428df7373df9c7f7617cfda2892610b2683610b0761057b956001610aed8761226d565b019065ffffffffffff1665ffffffffffff19825416179055565b6040805185815265ffffffffffff909216602083015290918291820190565b0390a16040519081529081906020820190565b634844252360e11b5f5260045ffd5b80610b565f610b5c9361067e565b806103d3565b5f610a9b565b6124e1565b610b89915060203d602011610b8f575b610b81818361067e565b81019061315e565b5f610a2b565b503d610b77565b610bb091955060203d602011610b8f57610b81818361067e565b935f6109e0565b65ffffffffffff8116036103cf57565b6064359063ffffffff821682036103cf57565b346103cf5760c03660031901126103cf57600435610bf781610659565b60243590610c0482610659565b604435610c1081610bb7565b610c18610bc7565b6084359060a435925f80516020614a2d83398151915254956001600160401b03610c5d610c50610c4c8a60ff9060401c1690565b1590565b986001600160401b031690565b1680159081610d76575b6001149081610d6c575b159081610d63575b50610d5457610cbc9587610cb360016001600160401b03195f80516020614a2d8339815191525416175f80516020614a2d83398151915255565b610d1f57612287565b610cc257005b610cec60ff60401b195f80516020614a2d83398151915254165f80516020614a2d83398151915255565b604051600181527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29080602081016104af565b610d4f600160401b60ff60401b195f80516020614a2d8339815191525416175f80516020614a2d83398151915255565b612287565b63f92ee8a960e01b5f5260045ffd5b9050155f610c79565b303b159150610c71565b889150610c67565b610d87366108fb565b91610d9483838387612bc2565b936020610da2603087612dd9565b50610dc2610daf8761226d565b805460ff60f01b1916600160f01b179055565b610dca612c29565b306001600160a01b0390911603610e7b575b5091849391610dee9361057b96613edd565b30610dfa6109ae612c29565b141580610e4e575b610e39575b6040518181527f712ae1383f79ac853f8d882153778e0260ef8f03b504e2866e0593e04d2b291f908060208101610b26565b5f5f805160206149cd83398151915255610e07565b50610e76610c4c5f805160206149cd833981519152546001600160801b0381169060801c1490565b610e02565b949091935f5b8351811015610ed55760019030610eab6109ae610e9e84896124a2565b516001600160a01b031690565b14610eb7575b01610e81565b610ed0610ec482886124a2565b51898151910120612fa7565b610eb1565b50909450929061057b610ddc565b346103cf5760203660031901126103cf57602061040d6004356124bb565b346103cf575f3660031901126103cf5760206040517f3e83946653575f9a39005e1545185629e92736b7528ab20ca3816f315424a8118152f35b346103cf575f3660031901126103cf57602065ffffffffffff5f805160206148cd8339815191525416604051908152f35b634e487b7160e01b5f52602160045260245ffd5b60081115610f8a57565b610f6c565b6008811015610f8a57602452565b346103cf5760203660031901126103cf57610fb960043561304e565b6040516008821015610f8a576020918152f35b346103cf5760403660031901126103cf57602060ff611021602435600435610ff382610659565b5f525f805160206149ad8339815191528452600360405f20019060018060a01b03165f5260205260405f2090565b54166040519015158152f35b346103cf5761103b366108fb565b9161104883838387612bc2565b61105181612d59565b505f9081525f805160206148ad83398151915260205260409020546001600160a01b031633036111785761108493612bc2565b61108f603b82612dd9565b506110b161109c8261226d565b80546001600160f81b0316600160f81b179055565b6040518181527f789cf55be980739dad1d0699b93b58e806b51c9d96619bfa8fe0a28abaa7b30c90602090a16110e681612240565b5490816110f9575b604051908152602090f35b5f805160206149ed8339815191525461111a906001600160a01b03166109ae565b803b156103cf5760405163c4d252f560e01b815260048101939093525f908390602490829084905af1918215610b625761057b92611164575b505f61115e82612240565b556110ee565b80610b565f6111729361067e565b5f611153565b63233d98e360e01b5f523360045260245ffd5b346103cf575f3660031901126103cf5761057b6111a6612527565b6040519182916020835260208301906104e3565b60403660031901126103cf576004356111d281610659565b6024356001600160401b0381116103cf576111f1903690600401610706565b906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000163081149081156112e3575b506112d457611234612c66565b6040516352d1902d60e01b8152916020836004816001600160a01b0386165afa5f93816112b3575b5061128057634c9c8ce360e01b5f526001600160a01b03821660045260245ffd5b5ffd5b905f8051602061498d833981519152830361129f57610020925061407e565b632a87526960e21b5f52600483905260245ffd5b6112cd91945060203d602011610b8f57610b81818361067e565b925f61125c565b63703e46dd60e11b5f5260045ffd5b5f8051602061498d833981519152546001600160a01b0316141590505f611227565b346103cf575f3660031901126103cf577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031630036112d45760206040515f8051602061498d8339815191528152f35b346103cf5760203660031901126103cf576004355f525f805160206149ad833981519152602052606060405f20805490600260018201549101549060405192835260208301526040820152f35b346103cf575f3660031901126103cf5761057b6111a66125e9565b6024359060ff821682036103cf57565b346103cf5760403660031901126103cf57602061040d6004356113f56113c4565b60405191611403858461067e565b5f8352339061316d565b9181601f840112156103cf578235916001600160401b0383116103cf57602083818601950101116103cf57565b346103cf5760c03660031901126103cf576004356114566113c4565b906044359061146482610659565b6064356001600160401b0381116103cf5761148390369060040161140d565b6084356001600160401b0381116103cf576114a2903690600401610706565b9160a435946001600160401b0386116103cf5761057b966114ca6114d0973690600401610706565b95612608565b6040519081529081906020820190565b346103cf5760803660031901126103cf576004356114fc6113c4565b906044356001600160401b0381116103cf5761151c90369060040161140d565b9190926064356001600160401b0381116103cf576110ee9461154561154d923690600401610706565b9436916106d0565b91339061333f565b346103cf5760203660031901126103cf57602061040d60043561274a565b346103cf5760203660031901126103cf5761002060043561159381610bb7565b61159b612c66565b61349a565b346103cf5760603660031901126103cf576004356115bc6113c4565b90604435906001600160401b0382116103cf576020926115ed6115e661040d94369060040161140d565b36916106d0565b91339061316d565b346103cf5760803660031901126103cf576004356001600160401b0381116103cf576116259036906004016107b9565b6024356001600160401b0381116103cf57611644903690600401610820565b906044356001600160401b0381116103cf5761166490369060040161087d565b90606435916001600160401b0383116103cf57366023840112156103cf5761057b9361169d6114d09436906024816004013591016106d0565b926128ba565b346103cf5760203660031901126103cf576004356116c081610659565b60018060a01b03165f527f5ab42ced628888259c08ac98db1eb0cf702fc1501344311d8b100cd1bfe4bb00602052602060405f2054604051908152f35b90602080835192838152019201905f5b81811061171a5750505090565b825184526020938401939092019160010161170d565b916117659061175761051897959693600f60f81b865260e0602087015260e08601906104e3565b9084820360408601526104e3565b60608301949094526001600160a01b031660808201525f60a082015280830360c0909101526116fd565b346103cf575f3660031901126103cf577fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d100541580611836575b156117f9576117d56120c6565b6117dd612193565b9061057b6117e96129d4565b6040519384933091469186611730565b60405162461bcd60e51b81526020600482015260156024820152741152540dcc4c8e88155b9a5b9a5d1a585b1a5e9959605a1b6044820152606490fd5b507fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d10154156117c8565b346103cf5760803660031901126103cf5760043561187b6113c4565b906044359161188983610659565b6064356001600160401b0381116103cf57610c4c6118ae61195b923690600401610706565b6001600160a01b0386165f9081527f5ab42ced628888259c08ac98db1eb0cf702fc1501344311d8b100cd1bfe4bb00602052604090208054600181019091556119559060405160208101917ff2aad550cf55f045cb27e9c559f9889fdfb6e6cdaa032301d6ea397784ae51d7835288604083015260ff8816606083015260018060a01b038a16608083015260a082015260a0815261194d60c08261067e565b51902061318a565b86613216565b61197657906114d09161057b93611970611bb0565b9261316d565b6394ab6c0760e01b5f526001600160a01b03831660045260245ffd5b346103cf575f3660031901126103cf5760206119ac6129ef565b65ffffffffffff60405191168152f35b346103cf575f3660031901126103cf57602060405160648152f35b346103cf5760603660031901126103cf576004356119f481610659565b6044356024356001600160401b0382116103cf57611a186020923690600401610706565b505f8051602061494d83398151915254604051630748d63560e31b81526001600160a01b0394851660048201526024810192909252909283916044918391165afa8015610b625761057b915f91611a7b575b506040519081529081906020820190565b611a94915060203d602011610b8f57610b81818361067e565b5f611a6a565b346103cf575f3660031901126103cf5760206001600160d01b03611abc6138b5565b16604051908152f35b346103cf5760203660031901126103cf57600435611ae281610659565b611aea612c66565b5f805160206149ed83398151915254604080516001600160a01b03808416825290931660208401819052927f08f74ea46ef7894f65eabfb5e6e695de773a000b47c529ab559178069b2264019190a16001600160a01b031916175f805160206149ed83398151915255005b346103cf5760203660031901126103cf57602060405160018152f35b346103cf5760203660031901126103cf57602061040d6004355f525f805160206148ad83398151915260205265ffffffffffff600160405f2001541690565b60405190611bbf60208361067e565b5f8252565b346103cf575f3660031901126103cf5761057b604051611be560408261067e565b60058152640352e302e360dc1b60208201526040519182916020835260208301906104e3565b346103cf575f3660031901126103cf5760205f8051602061496d83398151915254604051908152f35b346103cf5760a03660031901126103cf57611c50600435610659565b611c5b602435610659565b6044356001600160401b0381116103cf57611c7a903690600401610820565b506064356001600160401b0381116103cf57611c9a903690600401610820565b506084356001600160401b0381116103cf57611cba903690600401610706565b5061057b611cc6612a75565b6040516001600160e01b031990911681529081906020820190565b346103cf5760203660031901126103cf57602061040d600435612aa0565b60603660031901126103cf57600435611d1781610659565b6024356044356001600160401b0381116103cf57610020925f92611d408493369060040161140d565b9190611d4a612c66565b826040519384928337810185815203925af1611d64612aff565b90613911565b346103cf57602061040d611d7d366108fb565b92919091612bc2565b346103cf575f3660031901126103cf575f805160206149ed833981519152546040516001600160a01b039091168152602090f35b346103cf575f3660031901126103cf5761057b604051611ddb60408261067e565b602081527f737570706f72743d627261766f2671756f72756d3d666f722c6162737461696e60208201526040519182916020835260208301906104e3565b346103cf575f3660031901126103cf5760206040517ff2aad550cf55f045cb27e9c559f9889fdfb6e6cdaa032301d6ea397784ae51d78152f35b346103cf5760203660031901126103cf5760043563ffffffff811681036103cf5761002090611e80612c66565b61391e565b346103cf5760403660031901126103cf57600435611ea281610659565b60206024355f604051611eb5848261067e565b525f8051602061494d83398151915254604051630748d63560e31b81526001600160a01b0394851660048201526024810192909252909283916044918391165afa8015610b625761057b915f91611a7b57506040519081529081906020820190565b346103cf5760203660031901126103cf57610020600435611f36612c66565b6139b7565b346103cf5760a03660031901126103cf57611f57600435610659565b611f62602435610659565b6084356001600160401b0381116103cf57611f81903690600401610706565b50611f8a612c29565b306001600160a01b03909116036107935760405163f23a6e6160e01b8152602090f35b346103cf5760203660031901126103cf5760246004355f8051602061494d83398151915254604051632394e7a360e21b8152600481018390529260209184919082906001600160a01b03165afa918215610b62575f92612035575b506120129061274a565b908181029181830414901517156120305761057b90606490046114d0565b612719565b6120129192506120539060203d602011610b8f57610b81818361067e565b9190612008565b346103cf575f3660031901126103cf575f8051602061494d833981519152546040516001600160a01b039091168152602090f35b90600182811c921680156120bc575b60208310146120a857565b634e487b7160e01b5f52602260045260245ffd5b91607f169161209d565b604051905f825f8051602061490d83398151915254916120e58361208e565b80835292600181169081156121745750600114612109575b6106b39250038361067e565b505f8051602061490d8339815191525f90815290917f42ad5d3e1f2e6e70edcf6d991b8a3023d3fca8047a131592f9edb9fd9b89d57d5b8183106121585750509060206106b3928201016120fd565b6020919350806001915483858901015201910190918492612140565b602092506106b394915060ff191682840152151560051b8201016120fd565b604051905f825f8051602061492d83398151915254916121b28361208e565b808352926001811690811561217457506001146121d5576106b39250038361067e565b505f8051602061492d8339815191525f90815290917f5f9ce34815f8e11431c7bb75a8e6886a91478f7ffc1dbb0a98dc240fddd76b755b8183106122245750509060206106b3928201016120fd565b602091935080600191548385890101520191019091849261220c565b5f527f0d5829787b8befdbc6044ef7457d8a95c2a04bc99235349f1a212c063e59d40160205260405f2090565b5f525f805160206148ad83398151915260205260405f2090565b929094939160405161229a60408261067e565b600e81526d2a30b733b632a3b7bb32b93737b960911b60208201526122bd613b7f565b6122c56125e9565b6122cd613b7f565b81516001600160401b03811161069f576122fd816122f85f8051602061490d8339815191525461208e565b613baa565b6020601f82116001146123dc57936123ad6123b2946123586123c99c9b9995612344866123bf9b976123c49e9b5f916123d1575b508160011b915f199060031b1c19161790565b5f8051602061490d83398151915255613cb1565b6123805f7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d10055565b6123a85f7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d10155565b613dd0565b612e17565b6123ba613b7f565b612e33565b612e7d565b612f2c565b6106b3613b7f565b90508501515f612331565b5f8051602061490d8339815191525f52601f198216907f42ad5d3e1f2e6e70edcf6d991b8a3023d3fca8047a131592f9edb9fd9b89d57d915f5b8181106124765750946123586123c99c9b99956001866123c49d9a966123ad966123bf9d996123b29c1061245e575b5050811b015f8051602061490d83398151915255613cb1565b8601515f1960f88460031b161c191690555f80612445565b9192602060018192868a015181550194019201612416565b634e487b7160e01b5f52603260045260245ffd5b80518210156124b65760209160051b010190565b61248e565b5f525f805160206148ad83398151915260205265ffffffffffff60405f205460a01c1690565b6040513d5f823e3d90fd5b604051906124fb60408361067e565b601d82527f6d6f64653d626c6f636b6e756d6265722666726f6d3d64656661756c740000006020830152565b5f8051602061494d83398151915254604051634bf5d7e960e01b8152905f90829060049082906001600160a01b03165afa5f918161256e575b5061051857506105186124ec565b9091503d805f833e612580818361067e565b8101906020818303126103cf578051906001600160401b0382116103cf570181601f820112156103cf578051906125b6826106b5565b926125c4604051948561067e565b828452602083830101116103cf57815f9260208093018386015e83010152905f612560565b604051906125f860408361067e565b60018252603160f81b6020830152565b939092919695610c4c6126e2916126dc8a61265b8160018060a01b03165f527f5ab42ced628888259c08ac98db1eb0cf702fc1501344311d8b100cd1bfe4bb0060205260405f2080549060018201905590565b61266636888a6106d0565b602081519101208b5160208d0120906040519260208401947f3e83946653575f9a39005e1545185629e92736b7528ab20ca3816f315424a81186528d604086015260ff8d16606086015260018060a01b0316608085015260a084015260c083015260e082015260e0815261194d6101008261067e565b8a613216565b6126fd576105189596916126f79136916106d0565b9261333f565b6394ab6c0760e01b5f526001600160a01b03871660045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b5f1981019190821161203057565b60271981019190821161203057565b5f805160206148ed83398151915254905f198201828111612030578211156124b6575f805160206148ed8339815191525f527f293b0181c8ec34cd3252e741689bdc21b70ee7a0ec76216439035a5c3718909a8201548165ffffffffffff821611156128b157506127ba90613450565b5f829160058411612834575b6127d0935061439e565b806127da57505f90565b6128286128216127ec6105189361272d565b5f805160206148ed8339815191525f527f293b0181c8ec34cd3252e741689bdc21b70ee7a0ec76216439035a5c3718909b0190565b5460301c90565b6001600160d01b031690565b919261283f81614240565b8103908111612030576127d0935f805160206148ed8339815191525f5265ffffffffffff827f293b0181c8ec34cd3252e741689bdc21b70ee7a0ec76216439035a5c3718909b01541665ffffffffffff8516105f1461289f5750916127c6565b9291506128ab9061347f565b906127c6565b91505060301c90565b91939290936128c9823361351f565b156129c1575f8051602061496d8339815191525494856128f1575b61051894955033936136eb565b5f1965ffffffffffff6129026129ef565b160165ffffffffffff81116120305765ffffffffffff16955f60405161292960208261067e565b525f8051602061494d83398151915254604051630748d63560e31b81523360048201526024810198909852602090889060449082906001600160a01b03165afa968715610b62575f976129a0575b5080871061298557506128e4565b636121770b60e11b5f5233600452602487905260445260645ffd5b6129ba91975060203d602011610b8f57610b81818361067e565b955f612977565b63d9b3955760e01b5f523360045260245ffd5b604051906129e360208361067e565b5f808352366020840137565b5f8051602061494d833981519152546040516324776b7d60e21b815290602090829060049082906001600160a01b03165afa5f9181612a38575b50610518575061051843613450565b9091506020813d602011612a6d575b81612a546020938361067e565b810103126103cf5751612a6681610bb7565b905f612a29565b3d9150612a47565b5f805160206149ed83398151915254306001600160a01b03909116036107935763bc197c8160e01b90565b805f525f805160206148ad83398151915260205265ffffffffffff60405f205460a01c16905f525f805160206148ad83398151915260205263ffffffff60405f205460d01c160165ffffffffffff81116120305765ffffffffffff1690565b3d15612b29573d90612b10826106b5565b91612b1e604051938461067e565b82523d5f602084013e565b606090565b90602080835192838152019201905f5b818110612b4b5750505090565b82516001600160a01b0316845260209384019390920191600101612b3e565b9080602083519182815201916020808360051b8301019401925f915b838310612b9557505050505090565b9091929394602080612bb3600193601f1986820301875289516104e3565b97019301930191939290612b86565b9290612c2391612c0f612bfd94604051958694612beb602087019960808b5260a0880190612b2e565b868103601f19016040880152906116fd565b848103601f1901606086015290612b6a565b90608083015203601f19810183528261067e565b51902090565b5f805160206149ed833981519152546001600160a01b031690565b5f805160206149ed83398151915254306001600160a01b039091160361079357565b612c6e612c29565b336001600160a01b0390911603612cda57612c87612c29565b306001600160a01b0390911603612c9a57565b612ca3366106b5565b612cb0604051918261067e565b3681526020810190365f83375f602036830101525190205b80612cd1613a26565b03612cc8575b50565b6347096e4760e01b5f523360045260245ffd5b6008811015610f8a5760ff600191161b90565b600452606491906008811015610f8a576024525f604452565b612d228161304e565b906010612d2e83612ced565b1615612d38575090565b6331b75e4d60e01b5f52600452612d4f9150610f8f565b601060445260645ffd5b612d628161304e565b906001612d6e83612ced565b1615612d78575090565b6331b75e4d60e01b5f52600452612d8f9150610f8f565b600160445260645ffd5b612da28161304e565b906002612dae83612ced565b1615612db8575090565b6331b75e4d60e01b5f52600452612dcf9150610f8f565b600260445260645ffd5b90612de38261304e565b9181612dee84612ced565b1615612df957505090565b6331b75e4d60e01b5f52600452612e0f82610f8f565b60445260645ffd5b6106b39291611e80611f3692612e2b613b7f565b61159b613b7f565b612e3b613b7f565b612e43613b7f565b60018060a01b03166bffffffffffffffffffffffff60a01b5f8051602061494d8339815191525416175f8051602061494d83398151915255565b90612e86613b7f565b612e8e613b7f565b60648211612f14576001600160d01b03612ea66138b5565b1691612eb06129ef565b926001600160d01b0382116104b45791927f0553476bf02ef2726e8ce5ced78d63e26e602e4a2257b1f559418e24b46339979290612ef8906001600160d01b0384169061455b565b505060408051918252602082019290925290819081015b0390a1565b5063243e544560e01b5f52600452606460245260445ffd5b612f34613b7f565b612f3c613b7f565b5f805160206149ed83398151915254604080516001600160a01b03808416825290931660208401819052927f08f74ea46ef7894f65eabfb5e6e695de773a000b47c529ab559178069b2264019190a16001600160a01b031916175f805160206149ed83398151915255565b5f805160206149cd83398151915254908160801c6001600160801b0380600183011693168314613024575f527f7c712897014dbe49c045ef1299aa2d5f9e67e48eea4403efa21f1e0f3ac0cb0360205260405f20555f805160206149cd833981519152906001600160801b0382549181199060801b169116179055565b634e487b715f5260416020526024601cfd5b908160209103126103cf575180151581036103cf5790565b61305781613f73565b9061306182610f80565b6005820361315a576130739150612240565b545f805160206149ed83398151915254613095906001600160a01b03166109ae565b604051632c258a9f60e11b815260048101839052602081602481855afa908115610b62575f9161313b575b50156130cd575050600590565b604051632ab0f52960e01b81526004810192909252602090829060249082905afa908115610b62575f9161310c575b501561310757600790565b600290565b61312e915060203d602011613134575b613126818361067e565b810190613036565b5f6130fc565b503d61311c565b613154915060203d60201161313457613126818361067e565b5f6130c0565b5090565b908160209103126103cf575190565b9161051893916040519361318260208661067e565b5f855261333f565b6042906131956147fd565b61319d614867565b6040519060208201927f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f8452604083015260608201524660808201523060a082015260a081526131ee60c08261067e565b519020906040519161190160f01b8352600283015260228201522090565b60041115610f8a57565b9190823b61325157906132289161411d565b506132328161320c565b15918261323e57505090565b6001600160a01b03918216911614919050565b915f9261328761329585946040519283916020830195630b135d3f60e11b875260248401526040604484015260648301906104e3565b03601f19810183528261067e565b51915afa6132a1612aff565b816132d1575b816132b0575090565b90506132cd630b135d3f60e11b916020808251830101910161315e565b1490565b9050602081511015906132a7565b93909260ff61330b9361051897958752166020860152604085015260a0606085015260a08401906104e3565b9160808184039101526104e3565b909260ff60809361051896958452166020830152604082015281606082015201906104e3565b9291909261334c81612d99565b50613356816124bb565b5f8051602061494d83398151915254604051630748d63560e31b81526001600160a01b03808816600483018190526024830194909452929692909160209183916044918391165afa908115610b62576133b99285915f9361342f575b5084614157565b948051155f146133fc57506133f67fb8e138887d0aa13bab447e82de9d5c1777041ecd21ca36ba824ff1e6c07ddda4938660405194859485613319565b0390a290565b6133f6907fe2babfbac5889a709b63bb7f598b324e08bc5a4fb9ec647fb3cbc9ec07eb87129487604051958695866132df565b61344991935060203d602011610b8f57610b81818361067e565b915f6133b2565b65ffffffffffff81116134685765ffffffffffff1690565b6306dfcc6560e41b5f52603060045260245260445ffd5b906001820180921161203057565b9190820180921161203057565b7fc565b045403dc03c2eea82b81a0465edad9e2e7fc4d97e11421c209da93d7a93604065ffffffffffff805f805160206148cd83398151915254169382519485521692836020820152a165ffffffffffff195f805160206148cd8339815191525416175f805160206148cd83398151915255565b9081518110156124b6570160200190565b8151603481106135ce5760131981840101516001600160a01b0319166b1b91f1b211f2119351b859f160a31b016135ce57915f9261355c8161273b565b915b818310613579575050506001600160a01b0391821691161490565b90919361359f61359a61358c878561350e565b516001600160f81b03191690565b614423565b90156135c35760019160ff9060041b6010600160a01b03169116179401919061355e565b505050505050600190565b505050600190565b906135e0826107a2565b6135ed604051918261067e565b82815280926135fe601f19916107a2565b01905f5b82811061360e57505050565b806060602080938501015201613602565b9599989697949391926136609361365292885260018060a01b031660208801526101206040880152610120870190612b2e565b9085820360608701526116fd565b968388036080850152815180895260208901906020808260051b8c01019401915f905b8282106136bf5750505050610518969750906136a69184820360a0860152612b6a565b9360c083015260e08201526101008184039101526104e3565b909192946020806136dd6001938f601f1990820301865289516104e3565b970192019201909291613683565b92909493919461370382516020840120878387612bc2565b9584518251908181148015906138aa575b80156138a2575b61388357505065ffffffffffff6137436137348961226d565b5460a01c65ffffffffffff1690565b166138665791612f0f917f7d84a6263ae0d98d3329bd7b46bb4e8d6f98cd35a7adb45c274c8b7fd5ebd5e09594936137a761377c6129ef565b65ffffffffffff6137a065ffffffffffff5f805160206148cd833981519152541690565b911661348d565b906137c663ffffffff5f805160206148cd8339815191525460301c1690565b6138446137d28c61226d565b80546001600160a01b0319166001600160a01b038a1617815561381b6137f786613450565b825465ffffffffffff60a01b191660a09190911b65ffffffffffff60a01b16178255565b613824836144a0565b815463ffffffff60d01b191660d09190911b63ffffffff60d01b16179055565b61385861385189516135d6565b918461348d565b936040519889988d8a61361f565b61127d876138738161304e565b6331b75e4d60e01b5f5290612d00565b9151630447b05d60e41b5f908152600493909352602452604452606490fd5b50801561371b565b508251811415613714565b5f805160206148ed83398151915254806138ce57505f90565b805f19810111612030575f805160206148ed8339815191525f527f293b0181c8ec34cd3252e741689bdc21b70ee7a0ec76216439035a5c3718909a015460301c90565b9091906106b357506144cb565b63ffffffff81169081156139a45769ffffffff000000000000907f7e3f7f0708a84de9203036abaa450dccc85ad5ff52f78c170f3edb55cf5e882860405f805160206148cd833981519152549481519063ffffffff8760301c1682526020820152a160301b169069ffffffff0000000000001916175f805160206148cd83398151915255565b63f1cfbf0560e01b5f525f60045260245ffd5b5f8051602061496d8339815191525460408051918252602082018390527fccb45da8d5717e6c4544694297c4ba5cf151d455c9bb0ed4fc7a38411bc0546191a15f8051602061496d83398151915255565b8115613a12570490565b634e487b7160e01b5f52601260045260245ffd5b5f805160206149cd83398151915254906001600160801b0382169160801c8214613adb57815f527f7c712897014dbe49c045ef1299aa2d5f9e67e48eea4403efa21f1e0f3ac0cb0360205260405f2054916001600160801b0381165f527f7c712897014dbe49c045ef1299aa2d5f9e67e48eea4403efa21f1e0f3ac0cb036020525f60408120556001600160801b038060015f805160206149cd833981519152930116166001600160801b0319825416179055565b634e487b715f5260316020526024601cfd5b949392613b19608093613b0b613b279460a08a5260a08a0190612b2e565b9088820360208a01526116fd565b908682036040880152612b6a565b935f60608201520152565b9192613b6160a094613b53613b6f949998979960c0875260c0870190612b2e565b9085820360208701526116fd565b908382036040850152612b6a565b945f606083015260808201520152565b60ff5f80516020614a2d8339815191525460401c1615613b9b57565b631afcd79f60e31b5f5260045ffd5b601f8111613bb6575050565b5f8051602061490d8339815191525f5260205f20906020601f840160051c83019310613bfc575b601f0160051c01905b818110613bf1575050565b5f8155600101613be6565b9091508190613bdd565b601f8111613c12575050565b5f80516020614a0d8339815191525f5260205f20906020601f840160051c83019310613c58575b601f0160051c01905b818110613c4d575050565b5f8155600101613c42565b9091508190613c39565b601f8211613c6f57505050565b5f5260205f20906020601f840160051c83019310613ca7575b601f0160051c01905b818110613c9c575050565b5f8155600101613c91565b9091508190613c88565b9081516001600160401b03811161069f57613cf081613cdd5f8051602061492d8339815191525461208e565b5f8051602061492d833981519152613c62565b602092601f8211600114613d3c57613d20929382915f92613d31575b50508160011b915f199060031b1c19161790565b5f8051602061492d83398151915255565b015190505f80613d0c565b5f8051602061492d8339815191525f52601f198216937f5f9ce34815f8e11431c7bb75a8e6886a91478f7ffc1dbb0a98dc240fddd76b75915f5b868110613db85750836001959610613da0575b505050811b015f8051602061492d83398151915255565b01515f1960f88460031b161c191690555f8080613d89565b91926020600181928685015181550194019201613d76565b90613dd9613b7f565b81516001600160401b03811161069f57613e0981613e045f80516020614a0d8339815191525461208e565b613c06565b602092601f8211600114613e4957613e38929382915f92613d315750508160011b915f199060031b1c19161790565b5f80516020614a0d83398151915255565b5f80516020614a0d8339815191525f52601f198216937fda13dda7583a39a3cd73e8830529c760837228fa4683752c823b17e10548aad5915f5b868110613ec55750836001959610613ead575b505050811b015f80516020614a0d83398151915255565b01515f1960f88460031b161c191690555f8080613e96565b91926020600181928685015181550194019201613e83565b5f805160206149ed8339815191525490949192916001600160a01b03909116906bffffffffffffffffffffffff193060601b16823b156103cf57613f3b5f956040519788968795869563e38335e560e01b8752189260048601613aed565b039134905af18015610b6257613f5a575b50613f575f91612240565b55565b80613f665f809361067e565b8003126103cf575f613f4c565b613f7c8161226d565b5460f881901c9060f01c60ff166140775761407157613f9a816124bb565b801561405d57613fb6613fab6129ef565b65ffffffffffff1690565b8091101561405757613fc782612aa0565b10613fd25750600190565b613fde610c4c82614653565b8015614028575b15613ff05750600390565b61401a905f525f805160206148ad83398151915260205265ffffffffffff600160405f2001541690565b61402357600490565b600590565b50614052610c4c825f525f805160206149ad83398151915260205260405f20600181015490541090565b613fe5565b50505f90565b636ad0607560e01b5f52600482905260245ffd5b50600290565b5050600790565b90813b156140fc575f8051602061498d83398151915280546001600160a01b0319166001600160a01b0384169081179091557fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b5f80a28051156140e457612cd791614719565b5050346140ed57565b63b398979f60e01b5f5260045ffd5b50634c9c8ce360e01b5f9081526001600160a01b0391909116600452602490fd5b815191906041830361414d576141469250602082015190606060408401519301515f1a90614736565b9192909190565b50505f9160029190565b614178909291925f525f805160206149ad83398151915260205260405f2090565b91600383016141a161419a83839060018060a01b03165f5260205260405f2090565b5460ff1690565b614224576141c560ff93926141d2929060018060a01b03165f5260205260405f2090565b805460ff19166001179055565b16806141e957506141e482825461348d565b905590565b6001810361420057506001016141e482825461348d565b600203614215576002016141e482825461348d565b6303599be160e11b5f5260045ffd5b6371c6af4960e01b5f526001600160a01b03821660045260245ffd5b600181111561051857806001600160801b821015614361575b6143076142fd6142f36142e96142df6142d56142c461430e9760048a600160401b6143139c1015614354575b640100000000811015614347575b6201000081101561433a575b61010081101561432d575b6010811015614320575b1015614318575b60030260011c90565b6142ce818b613a08565b0160011c90565b6142ce818a613a08565b6142ce8189613a08565b6142ce8188613a08565b6142ce8187613a08565b6142ce8186613a08565b8093613a08565b821190565b900390565b60011b6142bb565b60041c9160021b916142b4565b60081c9160041b916142aa565b60101c9160081b9161429f565b60201c9160101b91614293565b60401c9160201b91614285565b505061431361430e6143076142fd6142f36142e96142df6142d56142c46143888a60801c90565b9850600160401b97506142599650505050505050565b905b8281106143ac57505090565b90918082169080831860011c8201809211612030575f805160206148ed8339815191525f527f293b0181c8ec34cd3252e741689bdc21b70ee7a0ec76216439035a5c3718909b82015465ffffffffffff90811690851610156144115750915b906143a0565b92915061441d9061347f565b9061440b565b60f81c9081602f1080614496575b1561444357600191602f190160ff1690565b816040108061448c575b1561445f576001916036190160ff1690565b8160601080614482575b1561447b576001916056190160ff1690565b5f91508190565b5060678210614469565b506047821061444d565b50603a8210614431565b63ffffffff81116144b45763ffffffff1690565b6306dfcc6560e41b5f52602060045260245260445ffd5b8051156144da57805190602001fd5b63d6bda27560e01b5f5260045ffd5b908154600160401b81101561069f57600181018084558110156124b6576106b3925f5260205f20019061453965ffffffffffff825116839065ffffffffffff1665ffffffffffff19825416179055565b60200151815465ffffffffffff1660309190911b65ffffffffffff1916179055565b5f805160206148ed83398151915254919291801561462a576127ec61457f9161272d565b90815461459b6145948265ffffffffffff1690565b9160301c90565b9265ffffffffffff80841692169180831161461b578692036145d9576145d592509065ffffffffffff82549181199060301b169116179055565b9190565b50506145d5906145f86145ea6106a4565b65ffffffffffff9092168252565b6001600160d01b03851660208201525b5f805160206148ed8339815191526144e9565b632520601d60e01b5f5260045ffd5b5061464e9061463a6145ea6106a4565b6001600160d01b0384166020820152614608565b5f9190565b805f525f805160206149ad833981519152602052602461467660405f20926124bb565b5f8051602061494d83398151915254604051632394e7a360e21b8152600481018390529260209184919082906001600160a01b03165afa918215610b62575f926146f4575b506146c59061274a565b90818102918183041490151715612030576146ef906064900491600260018201549101549061348d565b101590565b6146c59192506147129060203d602011610b8f57610b81818361067e565b91906146bb565b5f8061051893602081519101845af4614730612aff565b916147b8565b91907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a084116147ad579160209360809260ff5f9560405194855216868401526040830152606082015282805260015afa15610b62575f516001600160a01b038116156147a357905f905f90565b505f906001905f90565b5050505f9160039190565b906147c357506144cb565b815115806147f4575b6147d4575090565b639996b31560e01b5f9081526001600160a01b0391909116600452602490fd5b50803b156147cc565b6148056120c6565b8051908115614815576020012090565b50507fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d1005480156148425790565b507fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47090565b61486f612193565b805190811561487f576020012090565b50507fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d101548015614842579056fe7c712897014dbe49c045ef1299aa2d5f9e67e48eea4403efa21f1e0f3ac0cb0100d7616c8fe29c6c2fbe1d0c5bc8f2faa4c35b43746e70b24b4d532752affd01e770710421fd2cad75ad828c61aa98f2d77d423a440b67872d0f65554148e000a16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d102a16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d1033ba4977254e415696610a40ebf2258dbfa0ec6a2ff64e84bfe715ff16977cc0000d7616c8fe29c6c2fbe1d0c5bc8f2faa4c35b43746e70b24b4d532752affd00360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbca1cefa0f43667ef127a258e673c94202a79b656e62899531c4376d87a7f398007c712897014dbe49c045ef1299aa2d5f9e67e48eea4403efa21f1e0f3ac0cb020d5829787b8befdbc6044ef7457d8a95c2a04bc99235349f1a212c063e59d4007c712897014dbe49c045ef1299aa2d5f9e67e48eea4403efa21f1e0f3ac0cb00f0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00a164736f6c634300081a000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\"W[6\x15a\0\x18W_\x80\xFD[a\0 a,DV[\0[_5`\xE0\x1C\x80c\x01\xFF\xC9\xA7\x14a\x03aW\x80c\x02\xA2Q\xA3\x14a\x03\\W\x80c\x06\xF3\xF9\xE6\x14a\x03WW\x80c\x06\xFD\xDE\x03\x14a\x03RW\x80c\x144\x89\xD0\x14a\x03MW\x80c\x15\x0Bz\x02\x14a\x03HW\x80c\x16\x0C\xBE\xD7\x14a\x03CW\x80c\"\xF1 \xDE\x14a\x03>W\x80c&V\"}\x14a\x039W\x80c-c\xF6\x93\x14a\x034W\x80c/\xE3\xE2a\x14a\x03/W\x80c92\xAB\xB1\x14a\x03*W\x80c>OI\xE6\x14a\x03%W\x80cC\x85\x962\x14a\x03 W\x80cE!\x15\xD6\x14a\x03\x1BW\x80cK\xF5\xD7\xE9\x14a\x03\x16W\x80cO\x1E\xF2\x86\x14a\x03\x11W\x80cR\xD1\x90-\x14a\x03\x0CW\x80cTO\xFC\x9C\x14a\x03\x07W\x80cT\xFDMP\x14a\x03\x02W\x80cVx\x13\x88\x14a\x02\xFDW\x80c[\x8D\x0E\r\x14a\x02\xF8W\x80c_9\x8A\x14\x14a\x02\xF3W\x80c`\xC4$\x7F\x14a\x02\xEEW\x80cy\x05\x18\x87\x14a\x02\xE9W\x80c{<q\xD3\x14a\x02\xE4W\x80c}^\x81\xE2\x14a\x02\xDFW\x80c~\xCE\xBE\0\x14a\x02\xDAW\x80c\x84\xB0\x19n\x14a\x02\xD5W\x80c\x8F\xF2b\xE3\x14a\x02\xD0W\x80c\x91\xDD\xAD\xF4\x14a\x02\xCBW\x80c\x97\xC3\xD34\x14a\x02\xC6W\x80c\x9A\x80*m\x14a\x02\xC1W\x80c\xA7q:p\x14a\x02\xBCW\x80c\xA8\x90\xC9\x10\x14a\x02\xB7W\x80c\xA9\xA9R\x94\x14a\x02\xB2W\x80c\xABX\xFB\x8E\x14a\x02\xADW\x80c\xAD<\xB1\xCC\x14a\x02\xA8W\x80c\xB5\x811\xB0\x14a\x02\xA3W\x80c\xBC\x19|\x81\x14a\x02\x9EW\x80c\xC0\x1F\x9E7\x14a\x02\x99W\x80c\xC2\x8B\xC2\xFA\x14a\x02\x94W\x80c\xC5\x90W\xE4\x14a\x02\x8FW\x80c\xD32\x19\xB4\x14a\x02\x8AW\x80c\xDDN+\xA5\x14a\x02\x85W\x80c\xDE\xAA\xA7\xCC\x14a\x02\x80W\x80c\xE5@\xD0\x1D\x14a\x02{W\x80c\xEB\x90\x19\xD4\x14a\x02vW\x80c\xEC\xE4\x0C\xC1\x14a\x02qW\x80c\xF2:na\x14a\x02lW\x80c\xF8\xCEV\n\x14a\x02gWc\xFC\x0CTj\x03a\0\x0EWa ZV[a\x1F\xADV[a\x1F;V[a\x1F\x17V[a\x1E\x85V[a\x1ESV[a\x1E\x19V[a\x1D\xBAV[a\x1D\x86V[a\x1DjV[a\x1C\xFFV[a\x1C\xE1V[a\x1C4V[a\x1C\x0BV[a\x1B\xC4V[a\x1BqV[a\x1BUV[a\x1A\xC5V[a\x1A\x9AV[a\x19\xD7V[a\x19\xBCV[a\x19\x92V[a\x18_V[a\x17\x8FV[a\x16\xA3V[a\x15\xF5V[a\x15\xA0V[a\x15sV[a\x15UV[a\x14\xE0V[a\x14:V[a\x13\xD4V[a\x13\xA9V[a\x13\\V[a\x13\x05V[a\x11\xBAV[a\x11\x8BV[a\x10-V[a\x0F\xCCV[a\x0F\x9DV[a\x0F;V[a\x0F\x01V[a\x0E\xE3V[a\r~V[a\x0B\xDAV[a\tjV[a\x07!V[a\x06\rV[a\x05\x1BV[a\x04\x15V[a\x03\xDDV[4a\x03\xCFW` 6`\x03\x19\x01\x12a\x03\xCFW`\x045c\xFF\xFF\xFF\xFF`\xE0\x1B\x81\x16\x80\x91\x03a\x03\xCFW` \x90c2\xA2\xADC`\xE1\x1B\x81\x14\x90\x81\x15a\x03\xBEW[\x81\x15a\x03\xADW[P`@Q\x90\x15\x15\x81R\xF3[c\x01\xFF\xC9\xA7`\xE0\x1B\x14\x90P_a\x03\xA2V[c\x02q\x18\x97`\xE5\x1B\x81\x14\x91Pa\x03\x9BV[_\x80\xFD[_\x91\x03\x12a\x03\xCFWV[4a\x03\xCFW_6`\x03\x19\x01\x12a\x03\xCFW` a\x04\rc\xFF\xFF\xFF\xFF_\x80Q` aH\xCD\x839\x81Q\x91RT`0\x1C\x16\x90V[`@Q\x90\x81R\xF3[4a\x03\xCFW` 6`\x03\x19\x01\x12a\x03\xCFW`\x045a\x041a,fV[`d\x81\x11a\x04\xCCW`\x01`\x01`\xD0\x1B\x03a\x04Ia8\xB5V[\x16\x90a\x04Sa)\xEFV[\x91`\x01`\x01`\xD0\x1B\x03\x82\x11a\x04\xB4W\x7F\x05SGk\xF0.\xF2rn\x8C\xE5\xCE\xD7\x8Dc\xE2n`.J\"W\xB1\xF5YA\x8E$\xB4c9\x97\x92a\x04\x98\x90`\x01`\x01`\xD0\x1B\x03\x84\x16\x90aE[V[PP`@\x80Q\x91\x82R` \x82\x01\x92\x90\x92R\x90\x81\x90\x81\x01[\x03\x90\xA1\0[Pc\x06\xDF\xCCe`\xE4\x1B_R`\xD0`\x04R`$R`D_\xFD[c$>TE`\xE0\x1B_R`\x04R`d`$R`D_\xFD[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90` a\x05\x18\x92\x81\x81R\x01\x90a\x04\xE3V[\x90V[4a\x03\xCFW_6`\x03\x19\x01\x12a\x03\xCFW`@Q__\x80Q` aJ\r\x839\x81Q\x91RTa\x05G\x81a \x8EV[\x80\x84R\x90`\x01\x81\x16\x90\x81\x15a\x05\xE9WP`\x01\x14a\x05\x7FW[a\x05{\x83a\x05o\x81\x85\x03\x82a\x06~V[`@Q\x91\x82\x91\x82a\x05\x07V[\x03\x90\xF3[_\x80Q` aJ\r\x839\x81Q\x91R_\x90\x81R\x7F\xDA\x13\xDD\xA7X:9\xA3\xCDs\xE8\x83\x05)\xC7`\x83r(\xFAF\x83u,\x82;\x17\xE1\x05H\xAA\xD5\x93\x92P\x90[\x80\x82\x10a\x05\xCFWP\x90\x91P\x81\x01` \x01a\x05oa\x05_V[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a\x05\xB7V[`\xFF\x19\x16` \x80\x86\x01\x91\x90\x91R\x91\x15\x15`\x05\x1B\x84\x01\x90\x91\x01\x91Pa\x05o\x90Pa\x05_V[4a\x03\xCFW` 6`\x03\x19\x01\x12a\x03\xCFW`\x045_\x90\x81R_\x80Q` aH\xAD\x839\x81Q\x91R` \x90\x81R`@\x90\x91 T`\x01`\x01`\xA0\x1B\x03\x16`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\xF3[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x03\xCFWV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x06\x9FW`@RV[a\x06jV[`@Q\x90a\x06\xB3`@\x83a\x06~V[V[`\x01`\x01`@\x1B\x03\x81\x11a\x06\x9FW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\x06\xDC\x82a\x06\xB5V[\x91a\x06\xEA`@Q\x93\x84a\x06~V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x03\xCFW\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x03\xCFW\x81` a\x05\x18\x935\x91\x01a\x06\xD0V[4a\x03\xCFW`\x806`\x03\x19\x01\x12a\x03\xCFWa\x07=`\x045a\x06YV[a\x07H`$5a\x06YV[`d5`\x01`\x01`@\x1B\x03\x81\x11a\x03\xCFWa\x07g\x906\x90`\x04\x01a\x07\x06V[Pa\x07pa,)V[0`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x03a\x07\x93W`@Qc\n\x85\xBD\x01`\xE1\x1B\x81R` \x90\xF3[ct\x852\x8F`\xE1\x1B_R`\x04_\xFD[`\x01`\x01`@\x1B\x03\x81\x11a\x06\x9FW`\x05\x1B` \x01\x90V[\x90\x80`\x1F\x83\x01\x12\x15a\x03\xCFW\x815a\x07\xD0\x81a\x07\xA2V[\x92a\x07\xDE`@Q\x94\x85a\x06~V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03\xCFW` \x01\x90[\x82\x82\x10a\x08\x06WPPP\x90V[` \x80\x91\x835a\x08\x15\x81a\x06YV[\x81R\x01\x91\x01\x90a\x07\xF9V[\x90\x80`\x1F\x83\x01\x12\x15a\x03\xCFW\x815a\x087\x81a\x07\xA2V[\x92a\x08E`@Q\x94\x85a\x06~V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03\xCFW` \x01\x90[\x82\x82\x10a\x08mWPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\x08`V[\x90\x80`\x1F\x83\x01\x12\x15a\x03\xCFW\x815a\x08\x94\x81a\x07\xA2V[\x92a\x08\xA2`@Q\x94\x85a\x06~V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x91\x83\x83\x11a\x03\xCFW` \x82\x01\x90[\x83\x82\x10a\x08\xCEWPPPPP\x90V[\x815`\x01`\x01`@\x1B\x03\x81\x11a\x03\xCFW` \x91a\x08\xF0\x87\x84\x80\x94\x88\x01\x01a\x07\x06V[\x81R\x01\x91\x01\x90a\x08\xBFV[`\x80`\x03\x19\x82\x01\x12a\x03\xCFW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03\xCFW\x81a\t%\x91`\x04\x01a\x07\xB9V[\x91`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03\xCFW\x82a\tD\x91`\x04\x01a\x08 V[\x91`D5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\xCFWa\tc\x91`\x04\x01a\x08}V[\x90`d5\x90V[4a\x03\xCFWa\tx6a\x08\xFBV[\x90\x92a\t\x86\x82\x85\x85\x84a+\xC2V[\x93a\t\x90\x85a-\x19V[P_\x80Q` aI\xED\x839\x81Q\x91RTa\t\xBA\x90`\x01`\x01`\xA0\x1B\x03\x16[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x93`@Q\x93cy=\x06I`\xE1\x1B\x85R` \x85`\x04\x81\x89Z\xFA\x94\x85\x15a\x0BbW_\x95a\x0B\x96W[P0``\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x18\x94` `@Q\x80\x92c\xB1\xC5\xF4'`\xE0\x1B\x82R\x81\x80a\n\x19\x8B\x89\x89\x8C`\x04\x86\x01a:\xEDV[\x03\x91Z\xFA\x90\x81\x15a\x0BbW_\x91a\x0BgW[Pa\n5\x87a\"@V[U_\x80Q` aI\xED\x839\x81Q\x91RTa\nW\x90`\x01`\x01`\xA0\x1B\x03\x16a\t\xAEV[\x90\x81;\x15a\x03\xCFW_\x80\x94a\n\x83\x87`@Q\x99\x8A\x97\x88\x96\x87\x95c\x08\xF2\xA0\xBB`\xE4\x1B\x87R`\x04\x87\x01a;2V[\x03\x92Z\xF1\x90\x81\x15a\x0BbWa\n\xA7\x92a\n\xA2\x92a\x0BHW[PBa4\x8DV[a4PV[\x90e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x15a\x0B9W\x7F\x9A.B\xFDg\"\x81=i\x11>}\0y\xD3\xD9@\x17\x14(\xDFss\xDF\x9C\x7Fv\x17\xCF\xDA(\x92a\x0B&\x83a\x0B\x07a\x05{\x95`\x01a\n\xED\x87a\"mV[\x01\x90e\xFF\xFF\xFF\xFF\xFF\xFF\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82T\x16\x17\x90UV[`@\x80Q\x85\x81Re\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16` \x83\x01R\x90\x91\x82\x91\x82\x01\x90V[\x03\x90\xA1`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[cHD%#`\xE1\x1B_R`\x04_\xFD[\x80a\x0BV_a\x0B\\\x93a\x06~V[\x80a\x03\xD3V[_a\n\x9BV[a$\xE1V[a\x0B\x89\x91P` =` \x11a\x0B\x8FW[a\x0B\x81\x81\x83a\x06~V[\x81\x01\x90a1^V[_a\n+V[P=a\x0BwV[a\x0B\xB0\x91\x95P` =` \x11a\x0B\x8FWa\x0B\x81\x81\x83a\x06~V[\x93_a\t\xE0V[e\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x03a\x03\xCFWV[`d5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x03\xCFWV[4a\x03\xCFW`\xC06`\x03\x19\x01\x12a\x03\xCFW`\x045a\x0B\xF7\x81a\x06YV[`$5\x90a\x0C\x04\x82a\x06YV[`D5a\x0C\x10\x81a\x0B\xB7V[a\x0C\x18a\x0B\xC7V[`\x845\x90`\xA45\x92_\x80Q` aJ-\x839\x81Q\x91RT\x95`\x01`\x01`@\x1B\x03a\x0C]a\x0CPa\x0CL\x8A`\xFF\x90`@\x1C\x16\x90V[\x15\x90V[\x98`\x01`\x01`@\x1B\x03\x16\x90V[\x16\x80\x15\x90\x81a\rvW[`\x01\x14\x90\x81a\rlW[\x15\x90\x81a\rcW[Pa\rTWa\x0C\xBC\x95\x87a\x0C\xB3`\x01`\x01`\x01`@\x1B\x03\x19_\x80Q` aJ-\x839\x81Q\x91RT\x16\x17_\x80Q` aJ-\x839\x81Q\x91RUV[a\r\x1FWa\"\x87V[a\x0C\xC2W\0[a\x0C\xEC`\xFF`@\x1B\x19_\x80Q` aJ-\x839\x81Q\x91RT\x16_\x80Q` aJ-\x839\x81Q\x91RUV[`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90\x80` \x81\x01a\x04\xAFV[a\rO`\x01`@\x1B`\xFF`@\x1B\x19_\x80Q` aJ-\x839\x81Q\x91RT\x16\x17_\x80Q` aJ-\x839\x81Q\x91RUV[a\"\x87V[c\xF9.\xE8\xA9`\xE0\x1B_R`\x04_\xFD[\x90P\x15_a\x0CyV[0;\x15\x91Pa\x0CqV[\x88\x91Pa\x0CgV[a\r\x876a\x08\xFBV[\x91a\r\x94\x83\x83\x83\x87a+\xC2V[\x93` a\r\xA2`0\x87a-\xD9V[Pa\r\xC2a\r\xAF\x87a\"mV[\x80T`\xFF`\xF0\x1B\x19\x16`\x01`\xF0\x1B\x17\x90UV[a\r\xCAa,)V[0`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x03a\x0E{W[P\x91\x84\x93\x91a\r\xEE\x93a\x05{\x96a>\xDDV[0a\r\xFAa\t\xAEa,)V[\x14\x15\x80a\x0ENW[a\x0E9W[`@Q\x81\x81R\x7Fq*\xE18?y\xAC\x85?\x8D\x88!Sw\x8E\x02`\xEF\x8F\x03\xB5\x04\xE2\x86n\x05\x93\xE0M+)\x1F\x90\x80` \x81\x01a\x0B&V[__\x80Q` aI\xCD\x839\x81Q\x91RUa\x0E\x07V[Pa\x0Eva\x0CL_\x80Q` aI\xCD\x839\x81Q\x91RT`\x01`\x01`\x80\x1B\x03\x81\x16\x90`\x80\x1C\x14\x90V[a\x0E\x02V[\x94\x90\x91\x93_[\x83Q\x81\x10\x15a\x0E\xD5W`\x01\x900a\x0E\xABa\t\xAEa\x0E\x9E\x84\x89a$\xA2V[Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x14a\x0E\xB7W[\x01a\x0E\x81V[a\x0E\xD0a\x0E\xC4\x82\x88a$\xA2V[Q\x89\x81Q\x91\x01 a/\xA7V[a\x0E\xB1V[P\x90\x94P\x92\x90a\x05{a\r\xDCV[4a\x03\xCFW` 6`\x03\x19\x01\x12a\x03\xCFW` a\x04\r`\x045a$\xBBV[4a\x03\xCFW_6`\x03\x19\x01\x12a\x03\xCFW` `@Q\x7F>\x83\x94fSW_\x9A9\0^\x15E\x18V)\xE9'6\xB7R\x8A\xB2\x0C\xA3\x81o1T$\xA8\x11\x81R\xF3[4a\x03\xCFW_6`\x03\x19\x01\x12a\x03\xCFW` e\xFF\xFF\xFF\xFF\xFF\xFF_\x80Q` aH\xCD\x839\x81Q\x91RT\x16`@Q\x90\x81R\xF3[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x08\x11\x15a\x0F\x8AWV[a\x0FlV[`\x08\x81\x10\x15a\x0F\x8AW`$RV[4a\x03\xCFW` 6`\x03\x19\x01\x12a\x03\xCFWa\x0F\xB9`\x045a0NV[`@Q`\x08\x82\x10\x15a\x0F\x8AW` \x91\x81R\xF3[4a\x03\xCFW`@6`\x03\x19\x01\x12a\x03\xCFW` `\xFFa\x10!`$5`\x045a\x0F\xF3\x82a\x06YV[_R_\x80Q` aI\xAD\x839\x81Q\x91R\x84R`\x03`@_ \x01\x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03\xCFWa\x10;6a\x08\xFBV[\x91a\x10H\x83\x83\x83\x87a+\xC2V[a\x10Q\x81a-YV[P_\x90\x81R_\x80Q` aH\xAD\x839\x81Q\x91R` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x163\x03a\x11xWa\x10\x84\x93a+\xC2V[a\x10\x8F`;\x82a-\xD9V[Pa\x10\xB1a\x10\x9C\x82a\"mV[\x80T`\x01`\x01`\xF8\x1B\x03\x16`\x01`\xF8\x1B\x17\x90UV[`@Q\x81\x81R\x7Fx\x9C\xF5[\xE9\x80s\x9D\xAD\x1D\x06\x99\xB9;X\xE8\x06\xB5\x1C\x9D\x96a\x9B\xFA\x8F\xE0\xA2\x8A\xBA\xA7\xB3\x0C\x90` \x90\xA1a\x10\xE6\x81a\"@V[T\x90\x81a\x10\xF9W[`@Q\x90\x81R` \x90\xF3[_\x80Q` aI\xED\x839\x81Q\x91RTa\x11\x1A\x90`\x01`\x01`\xA0\x1B\x03\x16a\t\xAEV[\x80;\x15a\x03\xCFW`@Qc\xC4\xD2R\xF5`\xE0\x1B\x81R`\x04\x81\x01\x93\x90\x93R_\x90\x83\x90`$\x90\x82\x90\x84\x90Z\xF1\x91\x82\x15a\x0BbWa\x05{\x92a\x11dW[P_a\x11^\x82a\"@V[Ua\x10\xEEV[\x80a\x0BV_a\x11r\x93a\x06~V[_a\x11SV[c#=\x98\xE3`\xE0\x1B_R3`\x04R`$_\xFD[4a\x03\xCFW_6`\x03\x19\x01\x12a\x03\xCFWa\x05{a\x11\xA6a%'V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x04\xE3V[`@6`\x03\x19\x01\x12a\x03\xCFW`\x045a\x11\xD2\x81a\x06YV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03\xCFWa\x11\xF1\x906\x90`\x04\x01a\x07\x06V[\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x81\x14\x90\x81\x15a\x12\xE3W[Pa\x12\xD4Wa\x124a,fV[`@QcR\xD1\x90-`\xE0\x1B\x81R\x91` \x83`\x04\x81`\x01`\x01`\xA0\x1B\x03\x86\x16Z\xFA_\x93\x81a\x12\xB3W[Pa\x12\x80WcL\x9C\x8C\xE3`\xE0\x1B_R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04R`$_\xFD[_\xFD[\x90_\x80Q` aI\x8D\x839\x81Q\x91R\x83\x03a\x12\x9FWa\0 \x92Pa@~V[c*\x87Ri`\xE2\x1B_R`\x04\x83\x90R`$_\xFD[a\x12\xCD\x91\x94P` =` \x11a\x0B\x8FWa\x0B\x81\x81\x83a\x06~V[\x92_a\x12\\V[cp>F\xDD`\xE1\x1B_R`\x04_\xFD[_\x80Q` aI\x8D\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x90P_a\x12'V[4a\x03\xCFW_6`\x03\x19\x01\x12a\x03\xCFW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x160\x03a\x12\xD4W` `@Q_\x80Q` aI\x8D\x839\x81Q\x91R\x81R\xF3[4a\x03\xCFW` 6`\x03\x19\x01\x12a\x03\xCFW`\x045_R_\x80Q` aI\xAD\x839\x81Q\x91R` R```@_ \x80T\x90`\x02`\x01\x82\x01T\x91\x01T\x90`@Q\x92\x83R` \x83\x01R`@\x82\x01R\xF3[4a\x03\xCFW_6`\x03\x19\x01\x12a\x03\xCFWa\x05{a\x11\xA6a%\xE9V[`$5\x90`\xFF\x82\x16\x82\x03a\x03\xCFWV[4a\x03\xCFW`@6`\x03\x19\x01\x12a\x03\xCFW` a\x04\r`\x045a\x13\xF5a\x13\xC4V[`@Q\x91a\x14\x03\x85\x84a\x06~V[_\x83R3\x90a1mV[\x91\x81`\x1F\x84\x01\x12\x15a\x03\xCFW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x03\xCFW` \x83\x81\x86\x01\x95\x01\x01\x11a\x03\xCFWV[4a\x03\xCFW`\xC06`\x03\x19\x01\x12a\x03\xCFW`\x045a\x14Va\x13\xC4V[\x90`D5\x90a\x14d\x82a\x06YV[`d5`\x01`\x01`@\x1B\x03\x81\x11a\x03\xCFWa\x14\x83\x906\x90`\x04\x01a\x14\rV[`\x845`\x01`\x01`@\x1B\x03\x81\x11a\x03\xCFWa\x14\xA2\x906\x90`\x04\x01a\x07\x06V[\x91`\xA45\x94`\x01`\x01`@\x1B\x03\x86\x11a\x03\xCFWa\x05{\x96a\x14\xCAa\x14\xD0\x976\x90`\x04\x01a\x07\x06V[\x95a&\x08V[`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[4a\x03\xCFW`\x806`\x03\x19\x01\x12a\x03\xCFW`\x045a\x14\xFCa\x13\xC4V[\x90`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03\xCFWa\x15\x1C\x906\x90`\x04\x01a\x14\rV[\x91\x90\x92`d5`\x01`\x01`@\x1B\x03\x81\x11a\x03\xCFWa\x10\xEE\x94a\x15Ea\x15M\x926\x90`\x04\x01a\x07\x06V[\x946\x91a\x06\xD0V[\x913\x90a3?V[4a\x03\xCFW` 6`\x03\x19\x01\x12a\x03\xCFW` a\x04\r`\x045a'JV[4a\x03\xCFW` 6`\x03\x19\x01\x12a\x03\xCFWa\0 `\x045a\x15\x93\x81a\x0B\xB7V[a\x15\x9Ba,fV[a4\x9AV[4a\x03\xCFW``6`\x03\x19\x01\x12a\x03\xCFW`\x045a\x15\xBCa\x13\xC4V[\x90`D5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\xCFW` \x92a\x15\xEDa\x15\xE6a\x04\r\x946\x90`\x04\x01a\x14\rV[6\x91a\x06\xD0V[\x913\x90a1mV[4a\x03\xCFW`\x806`\x03\x19\x01\x12a\x03\xCFW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03\xCFWa\x16%\x906\x90`\x04\x01a\x07\xB9V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03\xCFWa\x16D\x906\x90`\x04\x01a\x08 V[\x90`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03\xCFWa\x16d\x906\x90`\x04\x01a\x08}V[\x90`d5\x91`\x01`\x01`@\x1B\x03\x83\x11a\x03\xCFW6`#\x84\x01\x12\x15a\x03\xCFWa\x05{\x93a\x16\x9Da\x14\xD0\x946\x90`$\x81`\x04\x015\x91\x01a\x06\xD0V[\x92a(\xBAV[4a\x03\xCFW` 6`\x03\x19\x01\x12a\x03\xCFW`\x045a\x16\xC0\x81a\x06YV[`\x01\x80`\xA0\x1B\x03\x16_R\x7FZ\xB4,\xEDb\x88\x88%\x9C\x08\xAC\x98\xDB\x1E\xB0\xCFp/\xC1P\x13D1\x1D\x8B\x10\x0C\xD1\xBF\xE4\xBB\0` R` `@_ T`@Q\x90\x81R\xF3[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x17\x1AWPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x17\rV[\x91a\x17e\x90a\x17Wa\x05\x18\x97\x95\x96\x93`\x0F`\xF8\x1B\x86R`\xE0` \x87\x01R`\xE0\x86\x01\x90a\x04\xE3V[\x90\x84\x82\x03`@\x86\x01Ra\x04\xE3V[``\x83\x01\x94\x90\x94R`\x01`\x01`\xA0\x1B\x03\x16`\x80\x82\x01R_`\xA0\x82\x01R\x80\x83\x03`\xC0\x90\x91\x01Ra\x16\xFDV[4a\x03\xCFW_6`\x03\x19\x01\x12a\x03\xCFW\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0T\x15\x80a\x186W[\x15a\x17\xF9Wa\x17\xD5a \xC6V[a\x17\xDDa!\x93V[\x90a\x05{a\x17\xE9a)\xD4V[`@Q\x93\x84\x930\x91F\x91\x86a\x170V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x11RT\r\xCCL\x8E\x88\x15[\x9A[\x9A]\x1AX[\x1A^\x99Y`Z\x1B`D\x82\x01R`d\x90\xFD[P\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\x01T\x15a\x17\xC8V[4a\x03\xCFW`\x806`\x03\x19\x01\x12a\x03\xCFW`\x045a\x18{a\x13\xC4V[\x90`D5\x91a\x18\x89\x83a\x06YV[`d5`\x01`\x01`@\x1B\x03\x81\x11a\x03\xCFWa\x0CLa\x18\xAEa\x19[\x926\x90`\x04\x01a\x07\x06V[`\x01`\x01`\xA0\x1B\x03\x86\x16_\x90\x81R\x7FZ\xB4,\xEDb\x88\x88%\x9C\x08\xAC\x98\xDB\x1E\xB0\xCFp/\xC1P\x13D1\x1D\x8B\x10\x0C\xD1\xBF\xE4\xBB\0` R`@\x90 \x80T`\x01\x81\x01\x90\x91Ua\x19U\x90`@Q` \x81\x01\x91\x7F\xF2\xAA\xD5P\xCFU\xF0E\xCB'\xE9\xC5Y\xF9\x88\x9F\xDF\xB6\xE6\xCD\xAA\x03#\x01\xD6\xEA9w\x84\xAEQ\xD7\x83R\x88`@\x83\x01R`\xFF\x88\x16``\x83\x01R`\x01\x80`\xA0\x1B\x03\x8A\x16`\x80\x83\x01R`\xA0\x82\x01R`\xA0\x81Ra\x19M`\xC0\x82a\x06~V[Q\x90 a1\x8AV[\x86a2\x16V[a\x19vW\x90a\x14\xD0\x91a\x05{\x93a\x19pa\x1B\xB0V[\x92a1mV[c\x94\xABl\x07`\xE0\x1B_R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04R`$_\xFD[4a\x03\xCFW_6`\x03\x19\x01\x12a\x03\xCFW` a\x19\xACa)\xEFV[e\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x91\x16\x81R\xF3[4a\x03\xCFW_6`\x03\x19\x01\x12a\x03\xCFW` `@Q`d\x81R\xF3[4a\x03\xCFW``6`\x03\x19\x01\x12a\x03\xCFW`\x045a\x19\xF4\x81a\x06YV[`D5`$5`\x01`\x01`@\x1B\x03\x82\x11a\x03\xCFWa\x1A\x18` \x926\x90`\x04\x01a\x07\x06V[P_\x80Q` aIM\x839\x81Q\x91RT`@Qc\x07H\xD65`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`\x04\x82\x01R`$\x81\x01\x92\x90\x92R\x90\x92\x83\x91`D\x91\x83\x91\x16Z\xFA\x80\x15a\x0BbWa\x05{\x91_\x91a\x1A{W[P`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[a\x1A\x94\x91P` =` \x11a\x0B\x8FWa\x0B\x81\x81\x83a\x06~V[_a\x1AjV[4a\x03\xCFW_6`\x03\x19\x01\x12a\x03\xCFW` `\x01`\x01`\xD0\x1B\x03a\x1A\xBCa8\xB5V[\x16`@Q\x90\x81R\xF3[4a\x03\xCFW` 6`\x03\x19\x01\x12a\x03\xCFW`\x045a\x1A\xE2\x81a\x06YV[a\x1A\xEAa,fV[_\x80Q` aI\xED\x839\x81Q\x91RT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82R\x90\x93\x16` \x84\x01\x81\x90R\x92\x7F\x08\xF7N\xA4n\xF7\x89Oe\xEA\xBF\xB5\xE6\xE6\x95\xDEw:\0\x0BG\xC5)\xABU\x91x\x06\x9B\"d\x01\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x19\x16\x17_\x80Q` aI\xED\x839\x81Q\x91RU\0[4a\x03\xCFW` 6`\x03\x19\x01\x12a\x03\xCFW` `@Q`\x01\x81R\xF3[4a\x03\xCFW` 6`\x03\x19\x01\x12a\x03\xCFW` a\x04\r`\x045_R_\x80Q` aH\xAD\x839\x81Q\x91R` Re\xFF\xFF\xFF\xFF\xFF\xFF`\x01`@_ \x01T\x16\x90V[`@Q\x90a\x1B\xBF` \x83a\x06~V[_\x82RV[4a\x03\xCFW_6`\x03\x19\x01\x12a\x03\xCFWa\x05{`@Qa\x1B\xE5`@\x82a\x06~V[`\x05\x81Rd\x03R\xE3\x02\xE3`\xDC\x1B` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x04\xE3V[4a\x03\xCFW_6`\x03\x19\x01\x12a\x03\xCFW` _\x80Q` aIm\x839\x81Q\x91RT`@Q\x90\x81R\xF3[4a\x03\xCFW`\xA06`\x03\x19\x01\x12a\x03\xCFWa\x1CP`\x045a\x06YV[a\x1C[`$5a\x06YV[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03\xCFWa\x1Cz\x906\x90`\x04\x01a\x08 V[P`d5`\x01`\x01`@\x1B\x03\x81\x11a\x03\xCFWa\x1C\x9A\x906\x90`\x04\x01a\x08 V[P`\x845`\x01`\x01`@\x1B\x03\x81\x11a\x03\xCFWa\x1C\xBA\x906\x90`\x04\x01a\x07\x06V[Pa\x05{a\x1C\xC6a*uV[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[4a\x03\xCFW` 6`\x03\x19\x01\x12a\x03\xCFW` a\x04\r`\x045a*\xA0V[``6`\x03\x19\x01\x12a\x03\xCFW`\x045a\x1D\x17\x81a\x06YV[`$5`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03\xCFWa\0 \x92_\x92a\x1D@\x84\x936\x90`\x04\x01a\x14\rV[\x91\x90a\x1DJa,fV[\x82`@Q\x93\x84\x92\x837\x81\x01\x85\x81R\x03\x92Z\xF1a\x1Dda*\xFFV[\x90a9\x11V[4a\x03\xCFW` a\x04\ra\x1D}6a\x08\xFBV[\x92\x91\x90\x91a+\xC2V[4a\x03\xCFW_6`\x03\x19\x01\x12a\x03\xCFW_\x80Q` aI\xED\x839\x81Q\x91RT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03\xCFW_6`\x03\x19\x01\x12a\x03\xCFWa\x05{`@Qa\x1D\xDB`@\x82a\x06~V[` \x81R\x7Fsupport=bravo&quorum=for,abstain` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x04\xE3V[4a\x03\xCFW_6`\x03\x19\x01\x12a\x03\xCFW` `@Q\x7F\xF2\xAA\xD5P\xCFU\xF0E\xCB'\xE9\xC5Y\xF9\x88\x9F\xDF\xB6\xE6\xCD\xAA\x03#\x01\xD6\xEA9w\x84\xAEQ\xD7\x81R\xF3[4a\x03\xCFW` 6`\x03\x19\x01\x12a\x03\xCFW`\x045c\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x03\xCFWa\0 \x90a\x1E\x80a,fV[a9\x1EV[4a\x03\xCFW`@6`\x03\x19\x01\x12a\x03\xCFW`\x045a\x1E\xA2\x81a\x06YV[` `$5_`@Qa\x1E\xB5\x84\x82a\x06~V[R_\x80Q` aIM\x839\x81Q\x91RT`@Qc\x07H\xD65`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`\x04\x82\x01R`$\x81\x01\x92\x90\x92R\x90\x92\x83\x91`D\x91\x83\x91\x16Z\xFA\x80\x15a\x0BbWa\x05{\x91_\x91a\x1A{WP`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[4a\x03\xCFW` 6`\x03\x19\x01\x12a\x03\xCFWa\0 `\x045a\x1F6a,fV[a9\xB7V[4a\x03\xCFW`\xA06`\x03\x19\x01\x12a\x03\xCFWa\x1FW`\x045a\x06YV[a\x1Fb`$5a\x06YV[`\x845`\x01`\x01`@\x1B\x03\x81\x11a\x03\xCFWa\x1F\x81\x906\x90`\x04\x01a\x07\x06V[Pa\x1F\x8Aa,)V[0`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x03a\x07\x93W`@Qc\xF2:na`\xE0\x1B\x81R` \x90\xF3[4a\x03\xCFW` 6`\x03\x19\x01\x12a\x03\xCFW`$`\x045_\x80Q` aIM\x839\x81Q\x91RT`@Qc#\x94\xE7\xA3`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R\x92` \x91\x84\x91\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x0BbW_\x92a 5W[Pa \x12\x90a'JV[\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x15a 0Wa\x05{\x90`d\x90\x04a\x14\xD0V[a'\x19V[a \x12\x91\x92Pa S\x90` =` \x11a\x0B\x8FWa\x0B\x81\x81\x83a\x06~V[\x91\x90a \x08V[4a\x03\xCFW_6`\x03\x19\x01\x12a\x03\xCFW_\x80Q` aIM\x839\x81Q\x91RT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a \xBCW[` \x83\x10\x14a \xA8WV[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a \x9DV[`@Q\x90_\x82_\x80Q` aI\r\x839\x81Q\x91RT\x91a \xE5\x83a \x8EV[\x80\x83R\x92`\x01\x81\x16\x90\x81\x15a!tWP`\x01\x14a!\tW[a\x06\xB3\x92P\x03\x83a\x06~V[P_\x80Q` aI\r\x839\x81Q\x91R_\x90\x81R\x90\x91\x7FB\xAD]>\x1F.np\xED\xCFm\x99\x1B\x8A0#\xD3\xFC\xA8\x04z\x13\x15\x92\xF9\xED\xB9\xFD\x9B\x89\xD5}[\x81\x83\x10a!XWPP\x90` a\x06\xB3\x92\x82\x01\x01a \xFDV[` \x91\x93P\x80`\x01\x91T\x83\x85\x89\x01\x01R\x01\x91\x01\x90\x91\x84\x92a!@V[` \x92Pa\x06\xB3\x94\x91P`\xFF\x19\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01a \xFDV[`@Q\x90_\x82_\x80Q` aI-\x839\x81Q\x91RT\x91a!\xB2\x83a \x8EV[\x80\x83R\x92`\x01\x81\x16\x90\x81\x15a!tWP`\x01\x14a!\xD5Wa\x06\xB3\x92P\x03\x83a\x06~V[P_\x80Q` aI-\x839\x81Q\x91R_\x90\x81R\x90\x91\x7F_\x9C\xE3H\x15\xF8\xE1\x141\xC7\xBBu\xA8\xE6\x88j\x91G\x8F\x7F\xFC\x1D\xBB\n\x98\xDC$\x0F\xDD\xD7ku[\x81\x83\x10a\"$WPP\x90` a\x06\xB3\x92\x82\x01\x01a \xFDV[` \x91\x93P\x80`\x01\x91T\x83\x85\x89\x01\x01R\x01\x91\x01\x90\x91\x84\x92a\"\x0CV[_R\x7F\rX)x{\x8B\xEF\xDB\xC6\x04N\xF7E}\x8A\x95\xC2\xA0K\xC9\x9254\x9F\x1A!,\x06>Y\xD4\x01` R`@_ \x90V[_R_\x80Q` aH\xAD\x839\x81Q\x91R` R`@_ \x90V[\x92\x90\x94\x93\x91`@Qa\"\x9A`@\x82a\x06~V[`\x0E\x81Rm*0\xB73\xB62\xA3\xB7\xBB2\xB977\xB9`\x91\x1B` \x82\x01Ra\"\xBDa;\x7FV[a\"\xC5a%\xE9V[a\"\xCDa;\x7FV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11a\x06\x9FWa\"\xFD\x81a\"\xF8_\x80Q` aI\r\x839\x81Q\x91RTa \x8EV[a;\xAAV[` `\x1F\x82\x11`\x01\x14a#\xDCW\x93a#\xADa#\xB2\x94a#Xa#\xC9\x9C\x9B\x99\x95a#D\x86a#\xBF\x9B\x97a#\xC4\x9E\x9B_\x91a#\xD1W[P\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[_\x80Q` aI\r\x839\x81Q\x91RUa<\xB1V[a#\x80_\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0UV[a#\xA8_\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\x01UV[a=\xD0V[a.\x17V[a#\xBAa;\x7FV[a.3V[a.}V[a/,V[a\x06\xB3a;\x7FV[\x90P\x85\x01Q_a#1V[_\x80Q` aI\r\x839\x81Q\x91R_R`\x1F\x19\x82\x16\x90\x7FB\xAD]>\x1F.np\xED\xCFm\x99\x1B\x8A0#\xD3\xFC\xA8\x04z\x13\x15\x92\xF9\xED\xB9\xFD\x9B\x89\xD5}\x91_[\x81\x81\x10a$vWP\x94a#Xa#\xC9\x9C\x9B\x99\x95`\x01\x86a#\xC4\x9D\x9A\x96a#\xAD\x96a#\xBF\x9D\x99a#\xB2\x9C\x10a$^W[PP\x81\x1B\x01_\x80Q` aI\r\x839\x81Q\x91RUa<\xB1V[\x86\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80a$EV[\x91\x92` `\x01\x81\x92\x86\x8A\x01Q\x81U\x01\x94\x01\x92\x01a$\x16V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80Q\x82\x10\x15a$\xB6W` \x91`\x05\x1B\x01\x01\x90V[a$\x8EV[_R_\x80Q` aH\xAD\x839\x81Q\x91R` Re\xFF\xFF\xFF\xFF\xFF\xFF`@_ T`\xA0\x1C\x16\x90V[`@Q=_\x82>=\x90\xFD[`@Q\x90a$\xFB`@\x83a\x06~V[`\x1D\x82R\x7Fmode=blocknumber&from=default\0\0\0` \x83\x01RV[_\x80Q` aIM\x839\x81Q\x91RT`@QcK\xF5\xD7\xE9`\xE0\x1B\x81R\x90_\x90\x82\x90`\x04\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA_\x91\x81a%nW[Pa\x05\x18WPa\x05\x18a$\xECV[\x90\x91P=\x80_\x83>a%\x80\x81\x83a\x06~V[\x81\x01\x90` \x81\x83\x03\x12a\x03\xCFW\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\xCFW\x01\x81`\x1F\x82\x01\x12\x15a\x03\xCFW\x80Q\x90a%\xB6\x82a\x06\xB5V[\x92a%\xC4`@Q\x94\x85a\x06~V[\x82\x84R` \x83\x83\x01\x01\x11a\x03\xCFW\x81_\x92` \x80\x93\x01\x83\x86\x01^\x83\x01\x01R\x90_a%`V[`@Q\x90a%\xF8`@\x83a\x06~V[`\x01\x82R`1`\xF8\x1B` \x83\x01RV[\x93\x90\x92\x91\x96\x95a\x0CLa&\xE2\x91a&\xDC\x8Aa&[\x81`\x01\x80`\xA0\x1B\x03\x16_R\x7FZ\xB4,\xEDb\x88\x88%\x9C\x08\xAC\x98\xDB\x1E\xB0\xCFp/\xC1P\x13D1\x1D\x8B\x10\x0C\xD1\xBF\xE4\xBB\0` R`@_ \x80T\x90`\x01\x82\x01\x90U\x90V[a&f6\x88\x8Aa\x06\xD0V[` \x81Q\x91\x01 \x8BQ` \x8D\x01 \x90`@Q\x92` \x84\x01\x94\x7F>\x83\x94fSW_\x9A9\0^\x15E\x18V)\xE9'6\xB7R\x8A\xB2\x0C\xA3\x81o1T$\xA8\x11\x86R\x8D`@\x86\x01R`\xFF\x8D\x16``\x86\x01R`\x01\x80`\xA0\x1B\x03\x16`\x80\x85\x01R`\xA0\x84\x01R`\xC0\x83\x01R`\xE0\x82\x01R`\xE0\x81Ra\x19Ma\x01\0\x82a\x06~V[\x8Aa2\x16V[a&\xFDWa\x05\x18\x95\x96\x91a&\xF7\x916\x91a\x06\xD0V[\x92a3?V[c\x94\xABl\x07`\xE0\x1B_R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_\x19\x81\x01\x91\x90\x82\x11a 0WV[`'\x19\x81\x01\x91\x90\x82\x11a 0WV[_\x80Q` aH\xED\x839\x81Q\x91RT\x90_\x19\x82\x01\x82\x81\x11a 0W\x82\x11\x15a$\xB6W_\x80Q` aH\xED\x839\x81Q\x91R_R\x7F);\x01\x81\xC8\xEC4\xCD2R\xE7Ah\x9B\xDC!\xB7\x0E\xE7\xA0\xECv!d9\x03Z\\7\x18\x90\x9A\x82\x01T\x81e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x11\x15a(\xB1WPa'\xBA\x90a4PV[_\x82\x91`\x05\x84\x11a(4W[a'\xD0\x93PaC\x9EV[\x80a'\xDAWP_\x90V[a((a(!a'\xECa\x05\x18\x93a'-V[_\x80Q` aH\xED\x839\x81Q\x91R_R\x7F);\x01\x81\xC8\xEC4\xCD2R\xE7Ah\x9B\xDC!\xB7\x0E\xE7\xA0\xECv!d9\x03Z\\7\x18\x90\x9B\x01\x90V[T`0\x1C\x90V[`\x01`\x01`\xD0\x1B\x03\x16\x90V[\x91\x92a(?\x81aB@V[\x81\x03\x90\x81\x11a 0Wa'\xD0\x93_\x80Q` aH\xED\x839\x81Q\x91R_Re\xFF\xFF\xFF\xFF\xFF\xFF\x82\x7F);\x01\x81\xC8\xEC4\xCD2R\xE7Ah\x9B\xDC!\xB7\x0E\xE7\xA0\xECv!d9\x03Z\\7\x18\x90\x9B\x01T\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x10_\x14a(\x9FWP\x91a'\xC6V[\x92\x91Pa(\xAB\x90a4\x7FV[\x90a'\xC6V[\x91PP`0\x1C\x90V[\x91\x93\x92\x90\x93a(\xC9\x823a5\x1FV[\x15a)\xC1W_\x80Q` aIm\x839\x81Q\x91RT\x94\x85a(\xF1W[a\x05\x18\x94\x95P3\x93a6\xEBV[_\x19e\xFF\xFF\xFF\xFF\xFF\xFFa)\x02a)\xEFV[\x16\x01e\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a 0We\xFF\xFF\xFF\xFF\xFF\xFF\x16\x95_`@Qa))` \x82a\x06~V[R_\x80Q` aIM\x839\x81Q\x91RT`@Qc\x07H\xD65`\xE3\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x98\x90\x98R` \x90\x88\x90`D\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x96\x87\x15a\x0BbW_\x97a)\xA0W[P\x80\x87\x10a)\x85WPa(\xE4V[ca!w\x0B`\xE1\x1B_R3`\x04R`$\x87\x90R`DR`d_\xFD[a)\xBA\x91\x97P` =` \x11a\x0B\x8FWa\x0B\x81\x81\x83a\x06~V[\x95_a)wV[c\xD9\xB3\x95W`\xE0\x1B_R3`\x04R`$_\xFD[`@Q\x90a)\xE3` \x83a\x06~V[_\x80\x83R6` \x84\x017V[_\x80Q` aIM\x839\x81Q\x91RT`@Qc$wk}`\xE2\x1B\x81R\x90` \x90\x82\x90`\x04\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA_\x91\x81a*8W[Pa\x05\x18WPa\x05\x18Ca4PV[\x90\x91P` \x81=` \x11a*mW[\x81a*T` \x93\x83a\x06~V[\x81\x01\x03\x12a\x03\xCFWQa*f\x81a\x0B\xB7V[\x90_a*)V[=\x91Pa*GV[_\x80Q` aI\xED\x839\x81Q\x91RT0`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x03a\x07\x93Wc\xBC\x19|\x81`\xE0\x1B\x90V[\x80_R_\x80Q` aH\xAD\x839\x81Q\x91R` Re\xFF\xFF\xFF\xFF\xFF\xFF`@_ T`\xA0\x1C\x16\x90_R_\x80Q` aH\xAD\x839\x81Q\x91R` Rc\xFF\xFF\xFF\xFF`@_ T`\xD0\x1C\x16\x01e\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a 0We\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[=\x15a+)W=\x90a+\x10\x82a\x06\xB5V[\x91a+\x1E`@Q\x93\x84a\x06~V[\x82R=_` \x84\x01>V[``\x90V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a+KWPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a+>V[\x90\x80` \x83Q\x91\x82\x81R\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a+\x95WPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a+\xB3`\x01\x93`\x1F\x19\x86\x82\x03\x01\x87R\x89Qa\x04\xE3V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a+\x86V[\x92\x90a,#\x91a,\x0Fa+\xFD\x94`@Q\x95\x86\x94a+\xEB` \x87\x01\x99`\x80\x8BR`\xA0\x88\x01\x90a+.V[\x86\x81\x03`\x1F\x19\x01`@\x88\x01R\x90a\x16\xFDV[\x84\x81\x03`\x1F\x19\x01``\x86\x01R\x90a+jV[\x90`\x80\x83\x01R\x03`\x1F\x19\x81\x01\x83R\x82a\x06~V[Q\x90 \x90V[_\x80Q` aI\xED\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[_\x80Q` aI\xED\x839\x81Q\x91RT0`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x03a\x07\x93WV[a,na,)V[3`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x03a,\xDAWa,\x87a,)V[0`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x03a,\x9AWV[a,\xA36a\x06\xB5V[a,\xB0`@Q\x91\x82a\x06~V[6\x81R` \x81\x01\x906_\x837_` 6\x83\x01\x01RQ\x90 [\x80a,\xD1a:&V[\x03a,\xC8W[PV[cG\tnG`\xE0\x1B_R3`\x04R`$_\xFD[`\x08\x81\x10\x15a\x0F\x8AW`\xFF`\x01\x91\x16\x1B\x90V[`\x04R`d\x91\x90`\x08\x81\x10\x15a\x0F\x8AW`$R_`DRV[a-\"\x81a0NV[\x90`\x10a-.\x83a,\xEDV[\x16\x15a-8WP\x90V[c1\xB7^M`\xE0\x1B_R`\x04Ra-O\x91Pa\x0F\x8FV[`\x10`DR`d_\xFD[a-b\x81a0NV[\x90`\x01a-n\x83a,\xEDV[\x16\x15a-xWP\x90V[c1\xB7^M`\xE0\x1B_R`\x04Ra-\x8F\x91Pa\x0F\x8FV[`\x01`DR`d_\xFD[a-\xA2\x81a0NV[\x90`\x02a-\xAE\x83a,\xEDV[\x16\x15a-\xB8WP\x90V[c1\xB7^M`\xE0\x1B_R`\x04Ra-\xCF\x91Pa\x0F\x8FV[`\x02`DR`d_\xFD[\x90a-\xE3\x82a0NV[\x91\x81a-\xEE\x84a,\xEDV[\x16\x15a-\xF9WPP\x90V[c1\xB7^M`\xE0\x1B_R`\x04Ra.\x0F\x82a\x0F\x8FV[`DR`d_\xFD[a\x06\xB3\x92\x91a\x1E\x80a\x1F6\x92a.+a;\x7FV[a\x15\x9Ba;\x7FV[a.;a;\x7FV[a.Ca;\x7FV[`\x01\x80`\xA0\x1B\x03\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B_\x80Q` aIM\x839\x81Q\x91RT\x16\x17_\x80Q` aIM\x839\x81Q\x91RUV[\x90a.\x86a;\x7FV[a.\x8Ea;\x7FV[`d\x82\x11a/\x14W`\x01`\x01`\xD0\x1B\x03a.\xA6a8\xB5V[\x16\x91a.\xB0a)\xEFV[\x92`\x01`\x01`\xD0\x1B\x03\x82\x11a\x04\xB4W\x91\x92\x7F\x05SGk\xF0.\xF2rn\x8C\xE5\xCE\xD7\x8Dc\xE2n`.J\"W\xB1\xF5YA\x8E$\xB4c9\x97\x92\x90a.\xF8\x90`\x01`\x01`\xD0\x1B\x03\x84\x16\x90aE[V[PP`@\x80Q\x91\x82R` \x82\x01\x92\x90\x92R\x90\x81\x90\x81\x01[\x03\x90\xA1V[Pc$>TE`\xE0\x1B_R`\x04R`d`$R`D_\xFD[a/4a;\x7FV[a/<a;\x7FV[_\x80Q` aI\xED\x839\x81Q\x91RT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82R\x90\x93\x16` \x84\x01\x81\x90R\x92\x7F\x08\xF7N\xA4n\xF7\x89Oe\xEA\xBF\xB5\xE6\xE6\x95\xDEw:\0\x0BG\xC5)\xABU\x91x\x06\x9B\"d\x01\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x19\x16\x17_\x80Q` aI\xED\x839\x81Q\x91RUV[_\x80Q` aI\xCD\x839\x81Q\x91RT\x90\x81`\x80\x1C`\x01`\x01`\x80\x1B\x03\x80`\x01\x83\x01\x16\x93\x16\x83\x14a0$W_R\x7F|q(\x97\x01M\xBEI\xC0E\xEF\x12\x99\xAA-_\x9Eg\xE4\x8E\xEAD\x03\xEF\xA2\x1F\x1E\x0F:\xC0\xCB\x03` R`@_ U_\x80Q` aI\xCD\x839\x81Q\x91R\x90`\x01`\x01`\x80\x1B\x03\x82T\x91\x81\x19\x90`\x80\x1B\x16\x91\x16\x17\x90UV[cNH{q_R`A` R`$`\x1C\xFD[\x90\x81` \x91\x03\x12a\x03\xCFWQ\x80\x15\x15\x81\x03a\x03\xCFW\x90V[a0W\x81a?sV[\x90a0a\x82a\x0F\x80V[`\x05\x82\x03a1ZWa0s\x91Pa\"@V[T_\x80Q` aI\xED\x839\x81Q\x91RTa0\x95\x90`\x01`\x01`\xA0\x1B\x03\x16a\t\xAEV[`@Qc,%\x8A\x9F`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R` \x81`$\x81\x85Z\xFA\x90\x81\x15a\x0BbW_\x91a1;W[P\x15a0\xCDWPP`\x05\x90V[`@Qc*\xB0\xF5)`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R` \x90\x82\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x0BbW_\x91a1\x0CW[P\x15a1\x07W`\x07\x90V[`\x02\x90V[a1.\x91P` =` \x11a14W[a1&\x81\x83a\x06~V[\x81\x01\x90a06V[_a0\xFCV[P=a1\x1CV[a1T\x91P` =` \x11a14Wa1&\x81\x83a\x06~V[_a0\xC0V[P\x90V[\x90\x81` \x91\x03\x12a\x03\xCFWQ\x90V[\x91a\x05\x18\x93\x91`@Q\x93a1\x82` \x86a\x06~V[_\x85Ra3?V[`B\x90a1\x95aG\xFDV[a1\x9DaHgV[`@Q\x90` \x82\x01\x92\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x84R`@\x83\x01R``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xA0\x81Ra1\xEE`\xC0\x82a\x06~V[Q\x90 \x90`@Q\x91a\x19\x01`\xF0\x1B\x83R`\x02\x83\x01R`\"\x82\x01R \x90V[`\x04\x11\x15a\x0F\x8AWV[\x91\x90\x82;a2QW\x90a2(\x91aA\x1DV[Pa22\x81a2\x0CV[\x15\x91\x82a2>WPP\x90V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x16\x14\x91\x90PV[\x91_\x92a2\x87a2\x95\x85\x94`@Q\x92\x83\x91` \x83\x01\x95c\x0B\x13]?`\xE1\x1B\x87R`$\x84\x01R`@`D\x84\x01R`d\x83\x01\x90a\x04\xE3V[\x03`\x1F\x19\x81\x01\x83R\x82a\x06~V[Q\x91Z\xFAa2\xA1a*\xFFV[\x81a2\xD1W[\x81a2\xB0WP\x90V[\x90Pa2\xCDc\x0B\x13]?`\xE1\x1B\x91` \x80\x82Q\x83\x01\x01\x91\x01a1^V[\x14\x90V[\x90P` \x81Q\x10\x15\x90a2\xA7V[\x93\x90\x92`\xFFa3\x0B\x93a\x05\x18\x97\x95\x87R\x16` \x86\x01R`@\x85\x01R`\xA0``\x85\x01R`\xA0\x84\x01\x90a\x04\xE3V[\x91`\x80\x81\x84\x03\x91\x01Ra\x04\xE3V[\x90\x92`\xFF`\x80\x93a\x05\x18\x96\x95\x84R\x16` \x83\x01R`@\x82\x01R\x81``\x82\x01R\x01\x90a\x04\xE3V[\x92\x91\x90\x92a3L\x81a-\x99V[Pa3V\x81a$\xBBV[_\x80Q` aIM\x839\x81Q\x91RT`@Qc\x07H\xD65`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x88\x16`\x04\x83\x01\x81\x90R`$\x83\x01\x94\x90\x94R\x92\x96\x92\x90\x91` \x91\x83\x91`D\x91\x83\x91\x16Z\xFA\x90\x81\x15a\x0BbWa3\xB9\x92\x85\x91_\x93a4/W[P\x84aAWV[\x94\x80Q\x15_\x14a3\xFCWPa3\xF6\x7F\xB8\xE18\x88}\n\xA1;\xABD~\x82\xDE\x9D\\\x17w\x04\x1E\xCD!\xCA6\xBA\x82O\xF1\xE6\xC0}\xDD\xA4\x93\x86`@Q\x94\x85\x94\x85a3\x19V[\x03\x90\xA2\x90V[a3\xF6\x90\x7F\xE2\xBA\xBF\xBA\xC5\x88\x9Ap\x9Bc\xBB\x7FY\x8B2N\x08\xBCZO\xB9\xECd\x7F\xB3\xCB\xC9\xEC\x07\xEB\x87\x12\x94\x87`@Q\x95\x86\x95\x86a2\xDFV[a4I\x91\x93P` =` \x11a\x0B\x8FWa\x0B\x81\x81\x83a\x06~V[\x91_a3\xB2V[e\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a4hWe\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[c\x06\xDF\xCCe`\xE4\x1B_R`0`\x04R`$R`D_\xFD[\x90`\x01\x82\x01\x80\x92\x11a 0WV[\x91\x90\x82\x01\x80\x92\x11a 0WV[\x7F\xC5e\xB0E@=\xC0<.\xEA\x82\xB8\x1A\x04e\xED\xAD\x9E.\x7F\xC4\xD9~\x11B\x1C \x9D\xA9=z\x93`@e\xFF\xFF\xFF\xFF\xFF\xFF\x80_\x80Q` aH\xCD\x839\x81Q\x91RT\x16\x93\x82Q\x94\x85R\x16\x92\x83` \x82\x01R\xA1e\xFF\xFF\xFF\xFF\xFF\xFF\x19_\x80Q` aH\xCD\x839\x81Q\x91RT\x16\x17_\x80Q` aH\xCD\x839\x81Q\x91RUV[\x90\x81Q\x81\x10\x15a$\xB6W\x01` \x01\x90V[\x81Q`4\x81\x10a5\xCEW`\x13\x19\x81\x84\x01\x01Q`\x01`\x01`\xA0\x1B\x03\x19\x16k\x1B\x91\xF1\xB2\x11\xF2\x11\x93Q\xB8Y\xF1`\xA3\x1B\x01a5\xCEW\x91_\x92a5\\\x81a';V[\x91[\x81\x83\x10a5yWPPP`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x16\x14\x90V[\x90\x91\x93a5\x9Fa5\x9Aa5\x8C\x87\x85a5\x0EV[Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[aD#V[\x90\x15a5\xC3W`\x01\x91`\xFF\x90`\x04\x1B`\x10`\x01`\xA0\x1B\x03\x16\x91\x16\x17\x94\x01\x91\x90a5^V[PPPPPP`\x01\x90V[PPP`\x01\x90V[\x90a5\xE0\x82a\x07\xA2V[a5\xED`@Q\x91\x82a\x06~V[\x82\x81R\x80\x92a5\xFE`\x1F\x19\x91a\x07\xA2V[\x01\x90_[\x82\x81\x10a6\x0EWPPPV[\x80``` \x80\x93\x85\x01\x01R\x01a6\x02V[\x95\x99\x98\x96\x97\x94\x93\x91\x92a6`\x93a6R\x92\x88R`\x01\x80`\xA0\x1B\x03\x16` \x88\x01Ra\x01 `@\x88\x01Ra\x01 \x87\x01\x90a+.V[\x90\x85\x82\x03``\x87\x01Ra\x16\xFDV[\x96\x83\x88\x03`\x80\x85\x01R\x81Q\x80\x89R` \x89\x01\x90` \x80\x82`\x05\x1B\x8C\x01\x01\x94\x01\x91_\x90[\x82\x82\x10a6\xBFWPPPPa\x05\x18\x96\x97P\x90a6\xA6\x91\x84\x82\x03`\xA0\x86\x01Ra+jV[\x93`\xC0\x83\x01R`\xE0\x82\x01Ra\x01\0\x81\x84\x03\x91\x01Ra\x04\xE3V[\x90\x91\x92\x94` \x80a6\xDD`\x01\x93\x8F`\x1F\x19\x90\x82\x03\x01\x86R\x89Qa\x04\xE3V[\x97\x01\x92\x01\x92\x01\x90\x92\x91a6\x83V[\x92\x90\x94\x93\x91\x94a7\x03\x82Q` \x84\x01 \x87\x83\x87a+\xC2V[\x95\x84Q\x82Q\x90\x81\x81\x14\x80\x15\x90a8\xAAW[\x80\x15a8\xA2W[a8\x83WPPe\xFF\xFF\xFF\xFF\xFF\xFFa7Ca74\x89a\"mV[T`\xA0\x1Ce\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x16a8fW\x91a/\x0F\x91\x7F}\x84\xA6&:\xE0\xD9\x8D3)\xBD{F\xBBN\x8Do\x98\xCD5\xA7\xAD\xB4\\'L\x8B\x7F\xD5\xEB\xD5\xE0\x95\x94\x93a7\xA7a7|a)\xEFV[e\xFF\xFF\xFF\xFF\xFF\xFFa7\xA0e\xFF\xFF\xFF\xFF\xFF\xFF_\x80Q` aH\xCD\x839\x81Q\x91RT\x16\x90V[\x91\x16a4\x8DV[\x90a7\xC6c\xFF\xFF\xFF\xFF_\x80Q` aH\xCD\x839\x81Q\x91RT`0\x1C\x16\x90V[a8Da7\xD2\x8Ca\"mV[\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x8A\x16\x17\x81Ua8\x1Ba7\xF7\x86a4PV[\x82Te\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16`\xA0\x91\x90\x91\x1Be\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x16\x17\x82UV[a8$\x83aD\xA0V[\x81Tc\xFF\xFF\xFF\xFF`\xD0\x1B\x19\x16`\xD0\x91\x90\x91\x1Bc\xFF\xFF\xFF\xFF`\xD0\x1B\x16\x17\x90UV[a8Xa8Q\x89Qa5\xD6V[\x91\x84a4\x8DV[\x93`@Q\x98\x89\x98\x8D\x8Aa6\x1FV[a\x12}\x87a8s\x81a0NV[c1\xB7^M`\xE0\x1B_R\x90a-\0V[\x91Qc\x04G\xB0]`\xE4\x1B_\x90\x81R`\x04\x93\x90\x93R`$R`DR`d\x90\xFD[P\x80\x15a7\x1BV[P\x82Q\x81\x14\x15a7\x14V[_\x80Q` aH\xED\x839\x81Q\x91RT\x80a8\xCEWP_\x90V[\x80_\x19\x81\x01\x11a 0W_\x80Q` aH\xED\x839\x81Q\x91R_R\x7F);\x01\x81\xC8\xEC4\xCD2R\xE7Ah\x9B\xDC!\xB7\x0E\xE7\xA0\xECv!d9\x03Z\\7\x18\x90\x9A\x01T`0\x1C\x90V[\x90\x91\x90a\x06\xB3WPaD\xCBV[c\xFF\xFF\xFF\xFF\x81\x16\x90\x81\x15a9\xA4Wi\xFF\xFF\xFF\xFF\0\0\0\0\0\0\x90\x7F~?\x7F\x07\x08\xA8M\xE9 06\xAB\xAAE\r\xCC\xC8Z\xD5\xFFR\xF7\x8C\x17\x0F>\xDBU\xCF^\x88(`@_\x80Q` aH\xCD\x839\x81Q\x91RT\x94\x81Q\x90c\xFF\xFF\xFF\xFF\x87`0\x1C\x16\x82R` \x82\x01R\xA1`0\x1B\x16\x90i\xFF\xFF\xFF\xFF\0\0\0\0\0\0\x19\x16\x17_\x80Q` aH\xCD\x839\x81Q\x91RUV[c\xF1\xCF\xBF\x05`\xE0\x1B_R_`\x04R`$_\xFD[_\x80Q` aIm\x839\x81Q\x91RT`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F\xCC\xB4]\xA8\xD5q~lEDiB\x97\xC4\xBA\\\xF1Q\xD4U\xC9\xBB\x0E\xD4\xFCz8A\x1B\xC0Ta\x91\xA1_\x80Q` aIm\x839\x81Q\x91RUV[\x81\x15a:\x12W\x04\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x80Q` aI\xCD\x839\x81Q\x91RT\x90`\x01`\x01`\x80\x1B\x03\x82\x16\x91`\x80\x1C\x82\x14a:\xDBW\x81_R\x7F|q(\x97\x01M\xBEI\xC0E\xEF\x12\x99\xAA-_\x9Eg\xE4\x8E\xEAD\x03\xEF\xA2\x1F\x1E\x0F:\xC0\xCB\x03` R`@_ T\x91`\x01`\x01`\x80\x1B\x03\x81\x16_R\x7F|q(\x97\x01M\xBEI\xC0E\xEF\x12\x99\xAA-_\x9Eg\xE4\x8E\xEAD\x03\xEF\xA2\x1F\x1E\x0F:\xC0\xCB\x03` R_`@\x81 U`\x01`\x01`\x80\x1B\x03\x80`\x01_\x80Q` aI\xCD\x839\x81Q\x91R\x93\x01\x16\x16`\x01`\x01`\x80\x1B\x03\x19\x82T\x16\x17\x90UV[cNH{q_R`1` R`$`\x1C\xFD[\x94\x93\x92a;\x19`\x80\x93a;\x0Ba;'\x94`\xA0\x8AR`\xA0\x8A\x01\x90a+.V[\x90\x88\x82\x03` \x8A\x01Ra\x16\xFDV[\x90\x86\x82\x03`@\x88\x01Ra+jV[\x93_``\x82\x01R\x01RV[\x91\x92a;a`\xA0\x94a;Sa;o\x94\x99\x98\x97\x99`\xC0\x87R`\xC0\x87\x01\x90a+.V[\x90\x85\x82\x03` \x87\x01Ra\x16\xFDV[\x90\x83\x82\x03`@\x85\x01Ra+jV[\x94_``\x83\x01R`\x80\x82\x01R\x01RV[`\xFF_\x80Q` aJ-\x839\x81Q\x91RT`@\x1C\x16\x15a;\x9BWV[c\x1A\xFC\xD7\x9F`\xE3\x1B_R`\x04_\xFD[`\x1F\x81\x11a;\xB6WPPV[_\x80Q` aI\r\x839\x81Q\x91R_R` _ \x90` `\x1F\x84\x01`\x05\x1C\x83\x01\x93\x10a;\xFCW[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a;\xF1WPPV[_\x81U`\x01\x01a;\xE6V[\x90\x91P\x81\x90a;\xDDV[`\x1F\x81\x11a<\x12WPPV[_\x80Q` aJ\r\x839\x81Q\x91R_R` _ \x90` `\x1F\x84\x01`\x05\x1C\x83\x01\x93\x10a<XW[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a<MWPPV[_\x81U`\x01\x01a<BV[\x90\x91P\x81\x90a<9V[`\x1F\x82\x11a<oWPPPV[_R` _ \x90` `\x1F\x84\x01`\x05\x1C\x83\x01\x93\x10a<\xA7W[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a<\x9CWPPV[_\x81U`\x01\x01a<\x91V[\x90\x91P\x81\x90a<\x88V[\x90\x81Q`\x01`\x01`@\x1B\x03\x81\x11a\x06\x9FWa<\xF0\x81a<\xDD_\x80Q` aI-\x839\x81Q\x91RTa \x8EV[_\x80Q` aI-\x839\x81Q\x91Ra<bV[` \x92`\x1F\x82\x11`\x01\x14a=<Wa= \x92\x93\x82\x91_\x92a=1W[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[_\x80Q` aI-\x839\x81Q\x91RUV[\x01Q\x90P_\x80a=\x0CV[_\x80Q` aI-\x839\x81Q\x91R_R`\x1F\x19\x82\x16\x93\x7F_\x9C\xE3H\x15\xF8\xE1\x141\xC7\xBBu\xA8\xE6\x88j\x91G\x8F\x7F\xFC\x1D\xBB\n\x98\xDC$\x0F\xDD\xD7ku\x91_[\x86\x81\x10a=\xB8WP\x83`\x01\x95\x96\x10a=\xA0W[PPP\x81\x1B\x01_\x80Q` aI-\x839\x81Q\x91RUV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a=\x89V[\x91\x92` `\x01\x81\x92\x86\x85\x01Q\x81U\x01\x94\x01\x92\x01a=vV[\x90a=\xD9a;\x7FV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11a\x06\x9FWa>\t\x81a>\x04_\x80Q` aJ\r\x839\x81Q\x91RTa \x8EV[a<\x06V[` \x92`\x1F\x82\x11`\x01\x14a>IWa>8\x92\x93\x82\x91_\x92a=1WPP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[_\x80Q` aJ\r\x839\x81Q\x91RUV[_\x80Q` aJ\r\x839\x81Q\x91R_R`\x1F\x19\x82\x16\x93\x7F\xDA\x13\xDD\xA7X:9\xA3\xCDs\xE8\x83\x05)\xC7`\x83r(\xFAF\x83u,\x82;\x17\xE1\x05H\xAA\xD5\x91_[\x86\x81\x10a>\xC5WP\x83`\x01\x95\x96\x10a>\xADW[PPP\x81\x1B\x01_\x80Q` aJ\r\x839\x81Q\x91RUV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a>\x96V[\x91\x92` `\x01\x81\x92\x86\x85\x01Q\x81U\x01\x94\x01\x92\x01a>\x83V[_\x80Q` aI\xED\x839\x81Q\x91RT\x90\x94\x91\x92\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x190``\x1B\x16\x82;\x15a\x03\xCFWa?;_\x95`@Q\x97\x88\x96\x87\x95\x86\x95c\xE3\x835\xE5`\xE0\x1B\x87R\x18\x92`\x04\x86\x01a:\xEDV[\x03\x914\x90Z\xF1\x80\x15a\x0BbWa?ZW[Pa?W_\x91a\"@V[UV[\x80a?f_\x80\x93a\x06~V[\x80\x03\x12a\x03\xCFW_a?LV[a?|\x81a\"mV[T`\xF8\x81\x90\x1C\x90`\xF0\x1C`\xFF\x16a@wWa@qWa?\x9A\x81a$\xBBV[\x80\x15a@]Wa?\xB6a?\xABa)\xEFV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x80\x91\x10\x15a@WWa?\xC7\x82a*\xA0V[\x10a?\xD2WP`\x01\x90V[a?\xDEa\x0CL\x82aFSV[\x80\x15a@(W[\x15a?\xF0WP`\x03\x90V[a@\x1A\x90_R_\x80Q` aH\xAD\x839\x81Q\x91R` Re\xFF\xFF\xFF\xFF\xFF\xFF`\x01`@_ \x01T\x16\x90V[a@#W`\x04\x90V[`\x05\x90V[Pa@Ra\x0CL\x82_R_\x80Q` aI\xAD\x839\x81Q\x91R` R`@_ `\x01\x81\x01T\x90T\x10\x90V[a?\xE5V[PP_\x90V[cj\xD0`u`\xE0\x1B_R`\x04\x82\x90R`$_\xFD[P`\x02\x90V[PP`\x07\x90V[\x90\x81;\x15a@\xFCW_\x80Q` aI\x8D\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x90\x91U\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;_\x80\xA2\x80Q\x15a@\xE4Wa,\xD7\x91aG\x19V[PP4a@\xEDWV[c\xB3\x98\x97\x9F`\xE0\x1B_R`\x04_\xFD[PcL\x9C\x8C\xE3`\xE0\x1B_\x90\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x04R`$\x90\xFD[\x81Q\x91\x90`A\x83\x03aAMWaAF\x92P` \x82\x01Q\x90```@\x84\x01Q\x93\x01Q_\x1A\x90aG6V[\x91\x92\x90\x91\x90V[PP_\x91`\x02\x91\x90V[aAx\x90\x92\x91\x92_R_\x80Q` aI\xAD\x839\x81Q\x91R` R`@_ \x90V[\x91`\x03\x83\x01aA\xA1aA\x9A\x83\x83\x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[T`\xFF\x16\x90V[aB$WaA\xC5`\xFF\x93\x92aA\xD2\x92\x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[\x80T`\xFF\x19\x16`\x01\x17\x90UV[\x16\x80aA\xE9WPaA\xE4\x82\x82Ta4\x8DV[\x90U\x90V[`\x01\x81\x03aB\0WP`\x01\x01aA\xE4\x82\x82Ta4\x8DV[`\x02\x03aB\x15W`\x02\x01aA\xE4\x82\x82Ta4\x8DV[c\x03Y\x9B\xE1`\xE1\x1B_R`\x04_\xFD[cq\xC6\xAFI`\xE0\x1B_R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04R`$_\xFD[`\x01\x81\x11\x15a\x05\x18W\x80`\x01`\x01`\x80\x1B\x82\x10\x15aCaW[aC\x07aB\xFDaB\xF3aB\xE9aB\xDFaB\xD5aB\xC4aC\x0E\x97`\x04\x8A`\x01`@\x1BaC\x13\x9C\x10\x15aCTW[d\x01\0\0\0\0\x81\x10\x15aCGW[b\x01\0\0\x81\x10\x15aC:W[a\x01\0\x81\x10\x15aC-W[`\x10\x81\x10\x15aC W[\x10\x15aC\x18W[`\x03\x02`\x01\x1C\x90V[aB\xCE\x81\x8Ba:\x08V[\x01`\x01\x1C\x90V[aB\xCE\x81\x8Aa:\x08V[aB\xCE\x81\x89a:\x08V[aB\xCE\x81\x88a:\x08V[aB\xCE\x81\x87a:\x08V[aB\xCE\x81\x86a:\x08V[\x80\x93a:\x08V[\x82\x11\x90V[\x90\x03\x90V[`\x01\x1BaB\xBBV[`\x04\x1C\x91`\x02\x1B\x91aB\xB4V[`\x08\x1C\x91`\x04\x1B\x91aB\xAAV[`\x10\x1C\x91`\x08\x1B\x91aB\x9FV[` \x1C\x91`\x10\x1B\x91aB\x93V[`@\x1C\x91` \x1B\x91aB\x85V[PPaC\x13aC\x0EaC\x07aB\xFDaB\xF3aB\xE9aB\xDFaB\xD5aB\xC4aC\x88\x8A`\x80\x1C\x90V[\x98P`\x01`@\x1B\x97PaBY\x96PPPPPPPV[\x90[\x82\x81\x10aC\xACWPP\x90V[\x90\x91\x80\x82\x16\x90\x80\x83\x18`\x01\x1C\x82\x01\x80\x92\x11a 0W_\x80Q` aH\xED\x839\x81Q\x91R_R\x7F);\x01\x81\xC8\xEC4\xCD2R\xE7Ah\x9B\xDC!\xB7\x0E\xE7\xA0\xECv!d9\x03Z\\7\x18\x90\x9B\x82\x01Te\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x85\x16\x10\x15aD\x11WP\x91[\x90aC\xA0V[\x92\x91PaD\x1D\x90a4\x7FV[\x90aD\x0BV[`\xF8\x1C\x90\x81`/\x10\x80aD\x96W[\x15aDCW`\x01\x91`/\x19\x01`\xFF\x16\x90V[\x81`@\x10\x80aD\x8CW[\x15aD_W`\x01\x91`6\x19\x01`\xFF\x16\x90V[\x81``\x10\x80aD\x82W[\x15aD{W`\x01\x91`V\x19\x01`\xFF\x16\x90V[_\x91P\x81\x90V[P`g\x82\x10aDiV[P`G\x82\x10aDMV[P`:\x82\x10aD1V[c\xFF\xFF\xFF\xFF\x81\x11aD\xB4Wc\xFF\xFF\xFF\xFF\x16\x90V[c\x06\xDF\xCCe`\xE4\x1B_R` `\x04R`$R`D_\xFD[\x80Q\x15aD\xDAW\x80Q\x90` \x01\xFD[c\xD6\xBD\xA2u`\xE0\x1B_R`\x04_\xFD[\x90\x81T`\x01`@\x1B\x81\x10\x15a\x06\x9FW`\x01\x81\x01\x80\x84U\x81\x10\x15a$\xB6Wa\x06\xB3\x92_R` _ \x01\x90aE9e\xFF\xFF\xFF\xFF\xFF\xFF\x82Q\x16\x83\x90e\xFF\xFF\xFF\xFF\xFF\xFF\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82T\x16\x17\x90UV[` \x01Q\x81Te\xFF\xFF\xFF\xFF\xFF\xFF\x16`0\x91\x90\x91\x1Be\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x17\x90UV[_\x80Q` aH\xED\x839\x81Q\x91RT\x91\x92\x91\x80\x15aF*Wa'\xECaE\x7F\x91a'-V[\x90\x81TaE\x9BaE\x94\x82e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x91`0\x1C\x90V[\x92e\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x16\x92\x16\x91\x80\x83\x11aF\x1BW\x86\x92\x03aE\xD9WaE\xD5\x92P\x90e\xFF\xFF\xFF\xFF\xFF\xFF\x82T\x91\x81\x19\x90`0\x1B\x16\x91\x16\x17\x90UV[\x91\x90V[PPaE\xD5\x90aE\xF8aE\xEAa\x06\xA4V[e\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x82RV[`\x01`\x01`\xD0\x1B\x03\x85\x16` \x82\x01R[_\x80Q` aH\xED\x839\x81Q\x91RaD\xE9V[c% `\x1D`\xE0\x1B_R`\x04_\xFD[PaFN\x90aF:aE\xEAa\x06\xA4V[`\x01`\x01`\xD0\x1B\x03\x84\x16` \x82\x01RaF\x08V[_\x91\x90V[\x80_R_\x80Q` aI\xAD\x839\x81Q\x91R` R`$aFv`@_ \x92a$\xBBV[_\x80Q` aIM\x839\x81Q\x91RT`@Qc#\x94\xE7\xA3`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R\x92` \x91\x84\x91\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x0BbW_\x92aF\xF4W[PaF\xC5\x90a'JV[\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x15a 0WaF\xEF\x90`d\x90\x04\x91`\x02`\x01\x82\x01T\x91\x01T\x90a4\x8DV[\x10\x15\x90V[aF\xC5\x91\x92PaG\x12\x90` =` \x11a\x0B\x8FWa\x0B\x81\x81\x83a\x06~V[\x91\x90aF\xBBV[_\x80a\x05\x18\x93` \x81Q\x91\x01\x84Z\xF4aG0a*\xFFV[\x91aG\xB8V[\x91\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11aG\xADW\x91` \x93`\x80\x92`\xFF_\x95`@Q\x94\x85R\x16\x86\x84\x01R`@\x83\x01R``\x82\x01R\x82\x80R`\x01Z\xFA\x15a\x0BbW_Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x15aG\xA3W\x90_\x90_\x90V[P_\x90`\x01\x90_\x90V[PPP_\x91`\x03\x91\x90V[\x90aG\xC3WPaD\xCBV[\x81Q\x15\x80aG\xF4W[aG\xD4WP\x90V[c\x99\x96\xB3\x15`\xE0\x1B_\x90\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x04R`$\x90\xFD[P\x80;\x15aG\xCCV[aH\x05a \xC6V[\x80Q\x90\x81\x15aH\x15W` \x01 \x90V[PP\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0T\x80\x15aHBW\x90V[P\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x90V[aHoa!\x93V[\x80Q\x90\x81\x15aH\x7FW` \x01 \x90V[PP\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\x01T\x80\x15aHBW\x90V\xFE|q(\x97\x01M\xBEI\xC0E\xEF\x12\x99\xAA-_\x9Eg\xE4\x8E\xEAD\x03\xEF\xA2\x1F\x1E\x0F:\xC0\xCB\x01\0\xD7al\x8F\xE2\x9Cl/\xBE\x1D\x0C[\xC8\xF2\xFA\xA4\xC3[Ctnp\xB2KMS'R\xAF\xFD\x01\xE7pq\x04!\xFD,\xADu\xAD\x82\x8Ca\xAA\x98\xF2\xD7}B:D\x0Bg\x87-\x0FeUAH\xE0\0\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\x02\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\x03;\xA4\x97rT\xE4\x15if\x10\xA4\x0E\xBF\"X\xDB\xFA\x0E\xC6\xA2\xFFd\xE8K\xFEq_\xF1iw\xCC\0\0\xD7al\x8F\xE2\x9Cl/\xBE\x1D\x0C[\xC8\xF2\xFA\xA4\xC3[Ctnp\xB2KMS'R\xAF\xFD\x006\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\xA1\xCE\xFA\x0FCf~\xF1'\xA2X\xE6s\xC9B\x02\xA7\x9Benb\x89\x951\xC47m\x87\xA7\xF3\x98\0|q(\x97\x01M\xBEI\xC0E\xEF\x12\x99\xAA-_\x9Eg\xE4\x8E\xEAD\x03\xEF\xA2\x1F\x1E\x0F:\xC0\xCB\x02\rX)x{\x8B\xEF\xDB\xC6\x04N\xF7E}\x8A\x95\xC2\xA0K\xC9\x9254\x9F\x1A!,\x06>Y\xD4\0|q(\x97\x01M\xBEI\xC0E\xEF\x12\x99\xAA-_\x9Eg\xE4\x8E\xEAD\x03\xEF\xA2\x1F\x1E\x0F:\xC0\xCB\0\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\xA1dsolcC\0\x08\x1A\0\n",
    );
    /**Custom error with signature `AddressEmptyCode(address)` and selector `0x9996b315`.
```solidity
error AddressEmptyCode(address target);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AddressEmptyCode {
        #[allow(missing_docs)]
        pub target: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<AddressEmptyCode> for UnderlyingRustTuple<'_> {
            fn from(value: AddressEmptyCode) -> Self {
                (value.target,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AddressEmptyCode {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { target: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AddressEmptyCode {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AddressEmptyCode(address)";
            const SELECTOR: [u8; 4] = [153u8, 150u8, 179u8, 21u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.target,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `CheckpointUnorderedInsertion()` and selector `0x2520601d`.
```solidity
error CheckpointUnorderedInsertion();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CheckpointUnorderedInsertion {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<CheckpointUnorderedInsertion>
        for UnderlyingRustTuple<'_> {
            fn from(value: CheckpointUnorderedInsertion) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for CheckpointUnorderedInsertion {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CheckpointUnorderedInsertion {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CheckpointUnorderedInsertion()";
            const SELECTOR: [u8; 4] = [37u8, 32u8, 96u8, 29u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `ERC1967InvalidImplementation(address)` and selector `0x4c9c8ce3`.
```solidity
error ERC1967InvalidImplementation(address implementation);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC1967InvalidImplementation {
        #[allow(missing_docs)]
        pub implementation: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ERC1967InvalidImplementation>
        for UnderlyingRustTuple<'_> {
            fn from(value: ERC1967InvalidImplementation) -> Self {
                (value.implementation,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ERC1967InvalidImplementation {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { implementation: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC1967InvalidImplementation {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC1967InvalidImplementation(address)";
            const SELECTOR: [u8; 4] = [76u8, 156u8, 140u8, 227u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.implementation,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `ERC1967NonPayable()` and selector `0xb398979f`.
```solidity
error ERC1967NonPayable();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC1967NonPayable {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ERC1967NonPayable> for UnderlyingRustTuple<'_> {
            fn from(value: ERC1967NonPayable) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC1967NonPayable {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC1967NonPayable {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC1967NonPayable()";
            const SELECTOR: [u8; 4] = [179u8, 152u8, 151u8, 159u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `FailedCall()` and selector `0xd6bda275`.
```solidity
error FailedCall();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FailedCall {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<FailedCall> for UnderlyingRustTuple<'_> {
            fn from(value: FailedCall) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FailedCall {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for FailedCall {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FailedCall()";
            const SELECTOR: [u8; 4] = [214u8, 189u8, 162u8, 117u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `GovernorAlreadyCastVote(address)` and selector `0x71c6af49`.
```solidity
error GovernorAlreadyCastVote(address voter);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GovernorAlreadyCastVote {
        #[allow(missing_docs)]
        pub voter: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<GovernorAlreadyCastVote> for UnderlyingRustTuple<'_> {
            fn from(value: GovernorAlreadyCastVote) -> Self {
                (value.voter,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for GovernorAlreadyCastVote {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { voter: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for GovernorAlreadyCastVote {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GovernorAlreadyCastVote(address)";
            const SELECTOR: [u8; 4] = [113u8, 198u8, 175u8, 73u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.voter,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `GovernorAlreadyQueuedProposal(uint256)` and selector `0xf20e7d37`.
```solidity
error GovernorAlreadyQueuedProposal(uint256 proposalId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GovernorAlreadyQueuedProposal {
        #[allow(missing_docs)]
        pub proposalId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<GovernorAlreadyQueuedProposal>
        for UnderlyingRustTuple<'_> {
            fn from(value: GovernorAlreadyQueuedProposal) -> Self {
                (value.proposalId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for GovernorAlreadyQueuedProposal {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { proposalId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for GovernorAlreadyQueuedProposal {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GovernorAlreadyQueuedProposal(uint256)";
            const SELECTOR: [u8; 4] = [242u8, 14u8, 125u8, 55u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.proposalId),
                )
            }
        }
    };
    /**Custom error with signature `GovernorDisabledDeposit()` and selector `0xe90a651e`.
```solidity
error GovernorDisabledDeposit();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GovernorDisabledDeposit {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<GovernorDisabledDeposit> for UnderlyingRustTuple<'_> {
            fn from(value: GovernorDisabledDeposit) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for GovernorDisabledDeposit {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for GovernorDisabledDeposit {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GovernorDisabledDeposit()";
            const SELECTOR: [u8; 4] = [233u8, 10u8, 101u8, 30u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `GovernorInsufficientProposerVotes(address,uint256,uint256)` and selector `0xc242ee16`.
```solidity
error GovernorInsufficientProposerVotes(address proposer, uint256 votes, uint256 threshold);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GovernorInsufficientProposerVotes {
        #[allow(missing_docs)]
        pub proposer: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub votes: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub threshold: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<GovernorInsufficientProposerVotes>
        for UnderlyingRustTuple<'_> {
            fn from(value: GovernorInsufficientProposerVotes) -> Self {
                (value.proposer, value.votes, value.threshold)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for GovernorInsufficientProposerVotes {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    proposer: tuple.0,
                    votes: tuple.1,
                    threshold: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for GovernorInsufficientProposerVotes {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GovernorInsufficientProposerVotes(address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [194u8, 66u8, 238u8, 22u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.proposer,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.votes),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.threshold),
                )
            }
        }
    };
    /**Custom error with signature `GovernorInvalidProposalLength(uint256,uint256,uint256)` and selector `0x447b05d0`.
```solidity
error GovernorInvalidProposalLength(uint256 targets, uint256 calldatas, uint256 values);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GovernorInvalidProposalLength {
        #[allow(missing_docs)]
        pub targets: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub calldatas: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub values: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<GovernorInvalidProposalLength>
        for UnderlyingRustTuple<'_> {
            fn from(value: GovernorInvalidProposalLength) -> Self {
                (value.targets, value.calldatas, value.values)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for GovernorInvalidProposalLength {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    targets: tuple.0,
                    calldatas: tuple.1,
                    values: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for GovernorInvalidProposalLength {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GovernorInvalidProposalLength(uint256,uint256,uint256)";
            const SELECTOR: [u8; 4] = [68u8, 123u8, 5u8, 208u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.targets),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.calldatas),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.values),
                )
            }
        }
    };
    /**Custom error with signature `GovernorInvalidQuorumFraction(uint256,uint256)` and selector `0x243e5445`.
```solidity
error GovernorInvalidQuorumFraction(uint256 quorumNumerator, uint256 quorumDenominator);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GovernorInvalidQuorumFraction {
        #[allow(missing_docs)]
        pub quorumNumerator: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub quorumDenominator: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<GovernorInvalidQuorumFraction>
        for UnderlyingRustTuple<'_> {
            fn from(value: GovernorInvalidQuorumFraction) -> Self {
                (value.quorumNumerator, value.quorumDenominator)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for GovernorInvalidQuorumFraction {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    quorumNumerator: tuple.0,
                    quorumDenominator: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for GovernorInvalidQuorumFraction {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GovernorInvalidQuorumFraction(uint256,uint256)";
            const SELECTOR: [u8; 4] = [36u8, 62u8, 84u8, 69u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumerator),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumDenominator),
                )
            }
        }
    };
    /**Custom error with signature `GovernorInvalidSignature(address)` and selector `0x94ab6c07`.
```solidity
error GovernorInvalidSignature(address voter);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GovernorInvalidSignature {
        #[allow(missing_docs)]
        pub voter: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<GovernorInvalidSignature>
        for UnderlyingRustTuple<'_> {
            fn from(value: GovernorInvalidSignature) -> Self {
                (value.voter,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for GovernorInvalidSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { voter: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for GovernorInvalidSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GovernorInvalidSignature(address)";
            const SELECTOR: [u8; 4] = [148u8, 171u8, 108u8, 7u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.voter,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `GovernorInvalidVoteParams()` and selector `0x867db771`.
```solidity
error GovernorInvalidVoteParams();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GovernorInvalidVoteParams {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<GovernorInvalidVoteParams>
        for UnderlyingRustTuple<'_> {
            fn from(value: GovernorInvalidVoteParams) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for GovernorInvalidVoteParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for GovernorInvalidVoteParams {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GovernorInvalidVoteParams()";
            const SELECTOR: [u8; 4] = [134u8, 125u8, 183u8, 113u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `GovernorInvalidVoteType()` and selector `0x06b337c2`.
```solidity
error GovernorInvalidVoteType();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GovernorInvalidVoteType {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<GovernorInvalidVoteType> for UnderlyingRustTuple<'_> {
            fn from(value: GovernorInvalidVoteType) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for GovernorInvalidVoteType {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for GovernorInvalidVoteType {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GovernorInvalidVoteType()";
            const SELECTOR: [u8; 4] = [6u8, 179u8, 55u8, 194u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `GovernorInvalidVotingPeriod(uint256)` and selector `0xf1cfbf05`.
```solidity
error GovernorInvalidVotingPeriod(uint256 votingPeriod);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GovernorInvalidVotingPeriod {
        #[allow(missing_docs)]
        pub votingPeriod: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<GovernorInvalidVotingPeriod>
        for UnderlyingRustTuple<'_> {
            fn from(value: GovernorInvalidVotingPeriod) -> Self {
                (value.votingPeriod,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for GovernorInvalidVotingPeriod {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { votingPeriod: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for GovernorInvalidVotingPeriod {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GovernorInvalidVotingPeriod(uint256)";
            const SELECTOR: [u8; 4] = [241u8, 207u8, 191u8, 5u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.votingPeriod),
                )
            }
        }
    };
    /**Custom error with signature `GovernorNonexistentProposal(uint256)` and selector `0x6ad06075`.
```solidity
error GovernorNonexistentProposal(uint256 proposalId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GovernorNonexistentProposal {
        #[allow(missing_docs)]
        pub proposalId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<GovernorNonexistentProposal>
        for UnderlyingRustTuple<'_> {
            fn from(value: GovernorNonexistentProposal) -> Self {
                (value.proposalId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for GovernorNonexistentProposal {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { proposalId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for GovernorNonexistentProposal {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GovernorNonexistentProposal(uint256)";
            const SELECTOR: [u8; 4] = [106u8, 208u8, 96u8, 117u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.proposalId),
                )
            }
        }
    };
    /**Custom error with signature `GovernorNotQueuedProposal(uint256)` and selector `0xd5ddb825`.
```solidity
error GovernorNotQueuedProposal(uint256 proposalId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GovernorNotQueuedProposal {
        #[allow(missing_docs)]
        pub proposalId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<GovernorNotQueuedProposal>
        for UnderlyingRustTuple<'_> {
            fn from(value: GovernorNotQueuedProposal) -> Self {
                (value.proposalId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for GovernorNotQueuedProposal {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { proposalId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for GovernorNotQueuedProposal {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GovernorNotQueuedProposal(uint256)";
            const SELECTOR: [u8; 4] = [213u8, 221u8, 184u8, 37u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.proposalId),
                )
            }
        }
    };
    /**Custom error with signature `GovernorOnlyExecutor(address)` and selector `0x47096e47`.
```solidity
error GovernorOnlyExecutor(address account);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GovernorOnlyExecutor {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<GovernorOnlyExecutor> for UnderlyingRustTuple<'_> {
            fn from(value: GovernorOnlyExecutor) -> Self {
                (value.account,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for GovernorOnlyExecutor {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { account: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for GovernorOnlyExecutor {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GovernorOnlyExecutor(address)";
            const SELECTOR: [u8; 4] = [71u8, 9u8, 110u8, 71u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `GovernorOnlyProposer(address)` and selector `0x233d98e3`.
```solidity
error GovernorOnlyProposer(address account);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GovernorOnlyProposer {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<GovernorOnlyProposer> for UnderlyingRustTuple<'_> {
            fn from(value: GovernorOnlyProposer) -> Self {
                (value.account,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for GovernorOnlyProposer {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { account: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for GovernorOnlyProposer {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GovernorOnlyProposer(address)";
            const SELECTOR: [u8; 4] = [35u8, 61u8, 152u8, 227u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `GovernorQueueNotImplemented()` and selector `0x90884a46`.
```solidity
error GovernorQueueNotImplemented();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GovernorQueueNotImplemented {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<GovernorQueueNotImplemented>
        for UnderlyingRustTuple<'_> {
            fn from(value: GovernorQueueNotImplemented) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for GovernorQueueNotImplemented {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for GovernorQueueNotImplemented {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GovernorQueueNotImplemented()";
            const SELECTOR: [u8; 4] = [144u8, 136u8, 74u8, 70u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `GovernorRestrictedProposer(address)` and selector `0xd9b39557`.
```solidity
error GovernorRestrictedProposer(address proposer);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GovernorRestrictedProposer {
        #[allow(missing_docs)]
        pub proposer: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<GovernorRestrictedProposer>
        for UnderlyingRustTuple<'_> {
            fn from(value: GovernorRestrictedProposer) -> Self {
                (value.proposer,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for GovernorRestrictedProposer {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { proposer: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for GovernorRestrictedProposer {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GovernorRestrictedProposer(address)";
            const SELECTOR: [u8; 4] = [217u8, 179u8, 149u8, 87u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.proposer,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `GovernorUnexpectedProposalState(uint256,uint8,bytes32)` and selector `0x31b75e4d`.
```solidity
error GovernorUnexpectedProposalState(uint256 proposalId, IGovernor.ProposalState current, bytes32 expectedStates);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GovernorUnexpectedProposalState {
        #[allow(missing_docs)]
        pub proposalId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub current: <IGovernor::ProposalState as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub expectedStates: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            IGovernor::ProposalState,
            alloy::sol_types::sol_data::FixedBytes<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            <IGovernor::ProposalState as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::FixedBytes<32>,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<GovernorUnexpectedProposalState>
        for UnderlyingRustTuple<'_> {
            fn from(value: GovernorUnexpectedProposalState) -> Self {
                (value.proposalId, value.current, value.expectedStates)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for GovernorUnexpectedProposalState {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    proposalId: tuple.0,
                    current: tuple.1,
                    expectedStates: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for GovernorUnexpectedProposalState {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GovernorUnexpectedProposalState(uint256,uint8,bytes32)";
            const SELECTOR: [u8; 4] = [49u8, 183u8, 94u8, 77u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.proposalId),
                    <IGovernor::ProposalState as alloy_sol_types::SolType>::tokenize(
                        &self.current,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.expectedStates),
                )
            }
        }
    };
    /**Custom error with signature `InvalidAccountNonce(address,uint256)` and selector `0x752d88c0`.
```solidity
error InvalidAccountNonce(address account, uint256 currentNonce);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidAccountNonce {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub currentNonce: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidAccountNonce> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidAccountNonce) -> Self {
                (value.account, value.currentNonce)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidAccountNonce {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    account: tuple.0,
                    currentNonce: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidAccountNonce {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidAccountNonce(address,uint256)";
            const SELECTOR: [u8; 4] = [117u8, 45u8, 136u8, 192u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.currentNonce),
                )
            }
        }
    };
    /**Custom error with signature `InvalidInitialization()` and selector `0xf92ee8a9`.
```solidity
error InvalidInitialization();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidInitialization {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidInitialization> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidInitialization) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidInitialization {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidInitialization {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidInitialization()";
            const SELECTOR: [u8; 4] = [249u8, 46u8, 232u8, 169u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `NotInitializing()` and selector `0xd7e6bcf8`.
```solidity
error NotInitializing();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotInitializing {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NotInitializing> for UnderlyingRustTuple<'_> {
            fn from(value: NotInitializing) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotInitializing {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotInitializing {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotInitializing()";
            const SELECTOR: [u8; 4] = [215u8, 230u8, 188u8, 248u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `SafeCastOverflowedUintDowncast(uint8,uint256)` and selector `0x6dfcc650`.
```solidity
error SafeCastOverflowedUintDowncast(uint8 bits, uint256 value);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SafeCastOverflowedUintDowncast {
        #[allow(missing_docs)]
        pub bits: u8,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<8>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u8,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<SafeCastOverflowedUintDowncast>
        for UnderlyingRustTuple<'_> {
            fn from(value: SafeCastOverflowedUintDowncast) -> Self {
                (value.bits, value.value)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for SafeCastOverflowedUintDowncast {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    bits: tuple.0,
                    value: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SafeCastOverflowedUintDowncast {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SafeCastOverflowedUintDowncast(uint8,uint256)";
            const SELECTOR: [u8; 4] = [109u8, 252u8, 198u8, 80u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.bits),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
                )
            }
        }
    };
    /**Custom error with signature `UUPSUnauthorizedCallContext()` and selector `0xe07c8dba`.
```solidity
error UUPSUnauthorizedCallContext();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UUPSUnauthorizedCallContext {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UUPSUnauthorizedCallContext>
        for UnderlyingRustTuple<'_> {
            fn from(value: UUPSUnauthorizedCallContext) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for UUPSUnauthorizedCallContext {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UUPSUnauthorizedCallContext {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UUPSUnauthorizedCallContext()";
            const SELECTOR: [u8; 4] = [224u8, 124u8, 141u8, 186u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `UUPSUnsupportedProxiableUUID(bytes32)` and selector `0xaa1d49a4`.
```solidity
error UUPSUnsupportedProxiableUUID(bytes32 slot);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UUPSUnsupportedProxiableUUID {
        #[allow(missing_docs)]
        pub slot: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UUPSUnsupportedProxiableUUID>
        for UnderlyingRustTuple<'_> {
            fn from(value: UUPSUnsupportedProxiableUUID) -> Self {
                (value.slot,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for UUPSUnsupportedProxiableUUID {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { slot: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UUPSUnsupportedProxiableUUID {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UUPSUnsupportedProxiableUUID(bytes32)";
            const SELECTOR: [u8; 4] = [170u8, 29u8, 73u8, 164u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.slot),
                )
            }
        }
    };
    /**Event with signature `EIP712DomainChanged()` and selector `0x0a6387c9ea3628b88a633bb4f3b151770f70085117a15f9bf3787cda53f13d31`.
```solidity
event EIP712DomainChanged();
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct EIP712DomainChanged {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for EIP712DomainChanged {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "EIP712DomainChanged()";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                10u8,
                99u8,
                135u8,
                201u8,
                234u8,
                54u8,
                40u8,
                184u8,
                138u8,
                99u8,
                59u8,
                180u8,
                243u8,
                177u8,
                81u8,
                119u8,
                15u8,
                112u8,
                8u8,
                81u8,
                23u8,
                161u8,
                95u8,
                155u8,
                243u8,
                120u8,
                124u8,
                218u8,
                83u8,
                241u8,
                61u8,
                49u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {}
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for EIP712DomainChanged {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&EIP712DomainChanged> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &EIP712DomainChanged) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Initialized(uint64)` and selector `0xc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2`.
```solidity
event Initialized(uint64 version);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Initialized {
        #[allow(missing_docs)]
        pub version: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Initialized {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Initialized(uint64)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                199u8,
                245u8,
                5u8,
                178u8,
                243u8,
                113u8,
                174u8,
                33u8,
                117u8,
                238u8,
                73u8,
                19u8,
                244u8,
                73u8,
                158u8,
                31u8,
                38u8,
                51u8,
                167u8,
                181u8,
                147u8,
                99u8,
                33u8,
                238u8,
                209u8,
                205u8,
                174u8,
                182u8,
                17u8,
                81u8,
                129u8,
                210u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { version: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.version),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Initialized {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Initialized> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Initialized) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ProposalCanceled(uint256)` and selector `0x789cf55be980739dad1d0699b93b58e806b51c9d96619bfa8fe0a28abaa7b30c`.
```solidity
event ProposalCanceled(uint256 proposalId);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ProposalCanceled {
        #[allow(missing_docs)]
        pub proposalId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ProposalCanceled {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ProposalCanceled(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                120u8,
                156u8,
                245u8,
                91u8,
                233u8,
                128u8,
                115u8,
                157u8,
                173u8,
                29u8,
                6u8,
                153u8,
                185u8,
                59u8,
                88u8,
                232u8,
                6u8,
                181u8,
                28u8,
                157u8,
                150u8,
                97u8,
                155u8,
                250u8,
                143u8,
                224u8,
                162u8,
                138u8,
                186u8,
                167u8,
                179u8,
                12u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { proposalId: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.proposalId),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ProposalCanceled {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ProposalCanceled> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ProposalCanceled) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ProposalCreated(uint256,address,address[],uint256[],string[],bytes[],uint256,uint256,string)` and selector `0x7d84a6263ae0d98d3329bd7b46bb4e8d6f98cd35a7adb45c274c8b7fd5ebd5e0`.
```solidity
event ProposalCreated(uint256 proposalId, address proposer, address[] targets, uint256[] values, string[] signatures, bytes[] calldatas, uint256 voteStart, uint256 voteEnd, string description);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ProposalCreated {
        #[allow(missing_docs)]
        pub proposalId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub proposer: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub targets: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        #[allow(missing_docs)]
        pub values: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        #[allow(missing_docs)]
        pub signatures: alloy::sol_types::private::Vec<
            alloy::sol_types::private::String,
        >,
        #[allow(missing_docs)]
        pub calldatas: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
        #[allow(missing_docs)]
        pub voteStart: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub voteEnd: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub description: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ProposalCreated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::String,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ProposalCreated(uint256,address,address[],uint256[],string[],bytes[],uint256,uint256,string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                125u8,
                132u8,
                166u8,
                38u8,
                58u8,
                224u8,
                217u8,
                141u8,
                51u8,
                41u8,
                189u8,
                123u8,
                70u8,
                187u8,
                78u8,
                141u8,
                111u8,
                152u8,
                205u8,
                53u8,
                167u8,
                173u8,
                180u8,
                92u8,
                39u8,
                76u8,
                139u8,
                127u8,
                213u8,
                235u8,
                213u8,
                224u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    proposalId: data.0,
                    proposer: data.1,
                    targets: data.2,
                    values: data.3,
                    signatures: data.4,
                    calldatas: data.5,
                    voteStart: data.6,
                    voteEnd: data.7,
                    description: data.8,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.proposalId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.proposer,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.targets),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.values),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::tokenize(&self.signatures),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::SolType>::tokenize(&self.calldatas),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.voteStart),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.voteEnd),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.description,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ProposalCreated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ProposalCreated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ProposalCreated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ProposalExecuted(uint256)` and selector `0x712ae1383f79ac853f8d882153778e0260ef8f03b504e2866e0593e04d2b291f`.
```solidity
event ProposalExecuted(uint256 proposalId);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ProposalExecuted {
        #[allow(missing_docs)]
        pub proposalId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ProposalExecuted {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ProposalExecuted(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                113u8,
                42u8,
                225u8,
                56u8,
                63u8,
                121u8,
                172u8,
                133u8,
                63u8,
                141u8,
                136u8,
                33u8,
                83u8,
                119u8,
                142u8,
                2u8,
                96u8,
                239u8,
                143u8,
                3u8,
                181u8,
                4u8,
                226u8,
                134u8,
                110u8,
                5u8,
                147u8,
                224u8,
                77u8,
                43u8,
                41u8,
                31u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { proposalId: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.proposalId),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ProposalExecuted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ProposalExecuted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ProposalExecuted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ProposalQueued(uint256,uint256)` and selector `0x9a2e42fd6722813d69113e7d0079d3d940171428df7373df9c7f7617cfda2892`.
```solidity
event ProposalQueued(uint256 proposalId, uint256 etaSeconds);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ProposalQueued {
        #[allow(missing_docs)]
        pub proposalId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub etaSeconds: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ProposalQueued {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ProposalQueued(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                154u8,
                46u8,
                66u8,
                253u8,
                103u8,
                34u8,
                129u8,
                61u8,
                105u8,
                17u8,
                62u8,
                125u8,
                0u8,
                121u8,
                211u8,
                217u8,
                64u8,
                23u8,
                20u8,
                40u8,
                223u8,
                115u8,
                115u8,
                223u8,
                156u8,
                127u8,
                118u8,
                23u8,
                207u8,
                218u8,
                40u8,
                146u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    proposalId: data.0,
                    etaSeconds: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.proposalId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.etaSeconds),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ProposalQueued {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ProposalQueued> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ProposalQueued) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ProposalThresholdSet(uint256,uint256)` and selector `0xccb45da8d5717e6c4544694297c4ba5cf151d455c9bb0ed4fc7a38411bc05461`.
```solidity
event ProposalThresholdSet(uint256 oldProposalThreshold, uint256 newProposalThreshold);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ProposalThresholdSet {
        #[allow(missing_docs)]
        pub oldProposalThreshold: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newProposalThreshold: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ProposalThresholdSet {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ProposalThresholdSet(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                204u8,
                180u8,
                93u8,
                168u8,
                213u8,
                113u8,
                126u8,
                108u8,
                69u8,
                68u8,
                105u8,
                66u8,
                151u8,
                196u8,
                186u8,
                92u8,
                241u8,
                81u8,
                212u8,
                85u8,
                201u8,
                187u8,
                14u8,
                212u8,
                252u8,
                122u8,
                56u8,
                65u8,
                27u8,
                192u8,
                84u8,
                97u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    oldProposalThreshold: data.0,
                    newProposalThreshold: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.oldProposalThreshold),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newProposalThreshold),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ProposalThresholdSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ProposalThresholdSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ProposalThresholdSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `QuorumNumeratorUpdated(uint256,uint256)` and selector `0x0553476bf02ef2726e8ce5ced78d63e26e602e4a2257b1f559418e24b4633997`.
```solidity
event QuorumNumeratorUpdated(uint256 oldQuorumNumerator, uint256 newQuorumNumerator);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct QuorumNumeratorUpdated {
        #[allow(missing_docs)]
        pub oldQuorumNumerator: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newQuorumNumerator: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for QuorumNumeratorUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "QuorumNumeratorUpdated(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                5u8,
                83u8,
                71u8,
                107u8,
                240u8,
                46u8,
                242u8,
                114u8,
                110u8,
                140u8,
                229u8,
                206u8,
                215u8,
                141u8,
                99u8,
                226u8,
                110u8,
                96u8,
                46u8,
                74u8,
                34u8,
                87u8,
                177u8,
                245u8,
                89u8,
                65u8,
                142u8,
                36u8,
                180u8,
                99u8,
                57u8,
                151u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    oldQuorumNumerator: data.0,
                    newQuorumNumerator: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.oldQuorumNumerator),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newQuorumNumerator),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for QuorumNumeratorUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&QuorumNumeratorUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &QuorumNumeratorUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `TimelockChange(address,address)` and selector `0x08f74ea46ef7894f65eabfb5e6e695de773a000b47c529ab559178069b226401`.
```solidity
event TimelockChange(address oldTimelock, address newTimelock);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TimelockChange {
        #[allow(missing_docs)]
        pub oldTimelock: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newTimelock: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for TimelockChange {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "TimelockChange(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                8u8,
                247u8,
                78u8,
                164u8,
                110u8,
                247u8,
                137u8,
                79u8,
                101u8,
                234u8,
                191u8,
                181u8,
                230u8,
                230u8,
                149u8,
                222u8,
                119u8,
                58u8,
                0u8,
                11u8,
                71u8,
                197u8,
                41u8,
                171u8,
                85u8,
                145u8,
                120u8,
                6u8,
                155u8,
                34u8,
                100u8,
                1u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    oldTimelock: data.0,
                    newTimelock: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.oldTimelock,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newTimelock,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for TimelockChange {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TimelockChange> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TimelockChange) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Upgraded(address)` and selector `0xbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b`.
```solidity
event Upgraded(address indexed implementation);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Upgraded {
        #[allow(missing_docs)]
        pub implementation: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Upgraded {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Upgraded(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                188u8,
                124u8,
                215u8,
                90u8,
                32u8,
                238u8,
                39u8,
                253u8,
                154u8,
                222u8,
                186u8,
                179u8,
                32u8,
                65u8,
                247u8,
                85u8,
                33u8,
                77u8,
                188u8,
                107u8,
                255u8,
                169u8,
                12u8,
                192u8,
                34u8,
                91u8,
                57u8,
                218u8,
                46u8,
                92u8,
                45u8,
                59u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { implementation: topics.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.implementation.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.implementation,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Upgraded {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Upgraded> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Upgraded) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `VoteCast(address,uint256,uint8,uint256,string)` and selector `0xb8e138887d0aa13bab447e82de9d5c1777041ecd21ca36ba824ff1e6c07ddda4`.
```solidity
event VoteCast(address indexed voter, uint256 proposalId, uint8 support, uint256 weight, string reason);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct VoteCast {
        #[allow(missing_docs)]
        pub voter: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub proposalId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub support: u8,
        #[allow(missing_docs)]
        pub weight: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub reason: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for VoteCast {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::String,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "VoteCast(address,uint256,uint8,uint256,string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                184u8,
                225u8,
                56u8,
                136u8,
                125u8,
                10u8,
                161u8,
                59u8,
                171u8,
                68u8,
                126u8,
                130u8,
                222u8,
                157u8,
                92u8,
                23u8,
                119u8,
                4u8,
                30u8,
                205u8,
                33u8,
                202u8,
                54u8,
                186u8,
                130u8,
                79u8,
                241u8,
                230u8,
                192u8,
                125u8,
                221u8,
                164u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    voter: topics.1,
                    proposalId: data.0,
                    support: data.1,
                    weight: data.2,
                    reason: data.3,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.proposalId),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.support),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.weight),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.reason,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.voter.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.voter,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for VoteCast {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&VoteCast> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &VoteCast) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `VoteCastWithParams(address,uint256,uint8,uint256,string,bytes)` and selector `0xe2babfbac5889a709b63bb7f598b324e08bc5a4fb9ec647fb3cbc9ec07eb8712`.
```solidity
event VoteCastWithParams(address indexed voter, uint256 proposalId, uint8 support, uint256 weight, string reason, bytes params);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct VoteCastWithParams {
        #[allow(missing_docs)]
        pub voter: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub proposalId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub support: u8,
        #[allow(missing_docs)]
        pub weight: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub reason: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub params: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for VoteCastWithParams {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "VoteCastWithParams(address,uint256,uint8,uint256,string,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                226u8,
                186u8,
                191u8,
                186u8,
                197u8,
                136u8,
                154u8,
                112u8,
                155u8,
                99u8,
                187u8,
                127u8,
                89u8,
                139u8,
                50u8,
                78u8,
                8u8,
                188u8,
                90u8,
                79u8,
                185u8,
                236u8,
                100u8,
                127u8,
                179u8,
                203u8,
                201u8,
                236u8,
                7u8,
                235u8,
                135u8,
                18u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    voter: topics.1,
                    proposalId: data.0,
                    support: data.1,
                    weight: data.2,
                    reason: data.3,
                    params: data.4,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.proposalId),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.support),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.weight),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.reason,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.params,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.voter.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.voter,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for VoteCastWithParams {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&VoteCastWithParams> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &VoteCastWithParams) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `VotingDelaySet(uint256,uint256)` and selector `0xc565b045403dc03c2eea82b81a0465edad9e2e7fc4d97e11421c209da93d7a93`.
```solidity
event VotingDelaySet(uint256 oldVotingDelay, uint256 newVotingDelay);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct VotingDelaySet {
        #[allow(missing_docs)]
        pub oldVotingDelay: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newVotingDelay: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for VotingDelaySet {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "VotingDelaySet(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                197u8,
                101u8,
                176u8,
                69u8,
                64u8,
                61u8,
                192u8,
                60u8,
                46u8,
                234u8,
                130u8,
                184u8,
                26u8,
                4u8,
                101u8,
                237u8,
                173u8,
                158u8,
                46u8,
                127u8,
                196u8,
                217u8,
                126u8,
                17u8,
                66u8,
                28u8,
                32u8,
                157u8,
                169u8,
                61u8,
                122u8,
                147u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    oldVotingDelay: data.0,
                    newVotingDelay: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.oldVotingDelay),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newVotingDelay),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for VotingDelaySet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&VotingDelaySet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &VotingDelaySet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `VotingPeriodSet(uint256,uint256)` and selector `0x7e3f7f0708a84de9203036abaa450dccc85ad5ff52f78c170f3edb55cf5e8828`.
```solidity
event VotingPeriodSet(uint256 oldVotingPeriod, uint256 newVotingPeriod);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct VotingPeriodSet {
        #[allow(missing_docs)]
        pub oldVotingPeriod: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newVotingPeriod: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for VotingPeriodSet {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "VotingPeriodSet(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                126u8,
                63u8,
                127u8,
                7u8,
                8u8,
                168u8,
                77u8,
                233u8,
                32u8,
                48u8,
                54u8,
                171u8,
                170u8,
                69u8,
                13u8,
                204u8,
                200u8,
                90u8,
                213u8,
                255u8,
                82u8,
                247u8,
                140u8,
                23u8,
                15u8,
                62u8,
                219u8,
                85u8,
                207u8,
                94u8,
                136u8,
                40u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    oldVotingPeriod: data.0,
                    newVotingPeriod: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.oldVotingPeriod),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newVotingPeriod),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for VotingPeriodSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&VotingPeriodSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &VotingPeriodSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {}
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Function with signature `BALLOT_TYPEHASH()` and selector `0xdeaaa7cc`.
```solidity
function BALLOT_TYPEHASH() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BALLOT_TYPEHASHCall {}
    ///Container type for the return parameters of the [`BALLOT_TYPEHASH()`](BALLOT_TYPEHASHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BALLOT_TYPEHASHReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<BALLOT_TYPEHASHCall> for UnderlyingRustTuple<'_> {
                fn from(value: BALLOT_TYPEHASHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for BALLOT_TYPEHASHCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<BALLOT_TYPEHASHReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: BALLOT_TYPEHASHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for BALLOT_TYPEHASHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for BALLOT_TYPEHASHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = BALLOT_TYPEHASHReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BALLOT_TYPEHASH()";
            const SELECTOR: [u8; 4] = [222u8, 170u8, 167u8, 204u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `CLOCK_MODE()` and selector `0x4bf5d7e9`.
```solidity
function CLOCK_MODE() external view returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CLOCK_MODECall {}
    ///Container type for the return parameters of the [`CLOCK_MODE()`](CLOCK_MODECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CLOCK_MODEReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<CLOCK_MODECall> for UnderlyingRustTuple<'_> {
                fn from(value: CLOCK_MODECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for CLOCK_MODECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<CLOCK_MODEReturn> for UnderlyingRustTuple<'_> {
                fn from(value: CLOCK_MODEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for CLOCK_MODEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for CLOCK_MODECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = CLOCK_MODEReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CLOCK_MODE()";
            const SELECTOR: [u8; 4] = [75u8, 245u8, 215u8, 233u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `COUNTING_MODE()` and selector `0xdd4e2ba5`.
```solidity
function COUNTING_MODE() external pure returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct COUNTING_MODECall {}
    ///Container type for the return parameters of the [`COUNTING_MODE()`](COUNTING_MODECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct COUNTING_MODEReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<COUNTING_MODECall> for UnderlyingRustTuple<'_> {
                fn from(value: COUNTING_MODECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for COUNTING_MODECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<COUNTING_MODEReturn> for UnderlyingRustTuple<'_> {
                fn from(value: COUNTING_MODEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for COUNTING_MODEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for COUNTING_MODECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = COUNTING_MODEReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "COUNTING_MODE()";
            const SELECTOR: [u8; 4] = [221u8, 78u8, 43u8, 165u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `EXTENDED_BALLOT_TYPEHASH()` and selector `0x2fe3e261`.
```solidity
function EXTENDED_BALLOT_TYPEHASH() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EXTENDED_BALLOT_TYPEHASHCall {}
    ///Container type for the return parameters of the [`EXTENDED_BALLOT_TYPEHASH()`](EXTENDED_BALLOT_TYPEHASHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EXTENDED_BALLOT_TYPEHASHReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<EXTENDED_BALLOT_TYPEHASHCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: EXTENDED_BALLOT_TYPEHASHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for EXTENDED_BALLOT_TYPEHASHCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<EXTENDED_BALLOT_TYPEHASHReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: EXTENDED_BALLOT_TYPEHASHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for EXTENDED_BALLOT_TYPEHASHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for EXTENDED_BALLOT_TYPEHASHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = EXTENDED_BALLOT_TYPEHASHReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EXTENDED_BALLOT_TYPEHASH()";
            const SELECTOR: [u8; 4] = [47u8, 227u8, 226u8, 97u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `UPGRADE_INTERFACE_VERSION()` and selector `0xad3cb1cc`.
```solidity
function UPGRADE_INTERFACE_VERSION() external view returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UPGRADE_INTERFACE_VERSIONCall {}
    ///Container type for the return parameters of the [`UPGRADE_INTERFACE_VERSION()`](UPGRADE_INTERFACE_VERSIONCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UPGRADE_INTERFACE_VERSIONReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UPGRADE_INTERFACE_VERSIONCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: UPGRADE_INTERFACE_VERSIONCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for UPGRADE_INTERFACE_VERSIONCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UPGRADE_INTERFACE_VERSIONReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: UPGRADE_INTERFACE_VERSIONReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for UPGRADE_INTERFACE_VERSIONReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for UPGRADE_INTERFACE_VERSIONCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = UPGRADE_INTERFACE_VERSIONReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UPGRADE_INTERFACE_VERSION()";
            const SELECTOR: [u8; 4] = [173u8, 60u8, 177u8, 204u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `cancel(address[],uint256[],bytes[],bytes32)` and selector `0x452115d6`.
```solidity
function cancel(address[] memory targets, uint256[] memory values, bytes[] memory calldatas, bytes32 descriptionHash) external returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cancelCall {
        #[allow(missing_docs)]
        pub targets: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        #[allow(missing_docs)]
        pub values: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        #[allow(missing_docs)]
        pub calldatas: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
        #[allow(missing_docs)]
        pub descriptionHash: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`cancel(address[],uint256[],bytes[],bytes32)`](cancelCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cancelReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
                alloy::sol_types::private::FixedBytes<32>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<cancelCall> for UnderlyingRustTuple<'_> {
                fn from(value: cancelCall) -> Self {
                    (value.targets, value.values, value.calldatas, value.descriptionHash)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for cancelCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targets: tuple.0,
                        values: tuple.1,
                        calldatas: tuple.2,
                        descriptionHash: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<cancelReturn> for UnderlyingRustTuple<'_> {
                fn from(value: cancelReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for cancelReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for cancelCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = cancelReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "cancel(address[],uint256[],bytes[],bytes32)";
            const SELECTOR: [u8; 4] = [69u8, 33u8, 21u8, 214u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.targets),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.values),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::SolType>::tokenize(&self.calldatas),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.descriptionHash),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `castVote(uint256,uint8)` and selector `0x56781388`.
```solidity
function castVote(uint256 proposalId, uint8 support) external returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct castVoteCall {
        #[allow(missing_docs)]
        pub proposalId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub support: u8,
    }
    ///Container type for the return parameters of the [`castVote(uint256,uint8)`](castVoteCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct castVoteReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                u8,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<castVoteCall> for UnderlyingRustTuple<'_> {
                fn from(value: castVoteCall) -> Self {
                    (value.proposalId, value.support)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for castVoteCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        proposalId: tuple.0,
                        support: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<castVoteReturn> for UnderlyingRustTuple<'_> {
                fn from(value: castVoteReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for castVoteReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for castVoteCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = castVoteReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "castVote(uint256,uint8)";
            const SELECTOR: [u8; 4] = [86u8, 120u8, 19u8, 136u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.proposalId),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.support),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `castVoteBySig(uint256,uint8,address,bytes)` and selector `0x8ff262e3`.
```solidity
function castVoteBySig(uint256 proposalId, uint8 support, address voter, bytes memory signature) external returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct castVoteBySigCall {
        #[allow(missing_docs)]
        pub proposalId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub support: u8,
        #[allow(missing_docs)]
        pub voter: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`castVoteBySig(uint256,uint8,address,bytes)`](castVoteBySigCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct castVoteBySigReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                u8,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<castVoteBySigCall> for UnderlyingRustTuple<'_> {
                fn from(value: castVoteBySigCall) -> Self {
                    (value.proposalId, value.support, value.voter, value.signature)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for castVoteBySigCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        proposalId: tuple.0,
                        support: tuple.1,
                        voter: tuple.2,
                        signature: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<castVoteBySigReturn> for UnderlyingRustTuple<'_> {
                fn from(value: castVoteBySigReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for castVoteBySigReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for castVoteBySigCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = castVoteBySigReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "castVoteBySig(uint256,uint8,address,bytes)";
            const SELECTOR: [u8; 4] = [143u8, 242u8, 98u8, 227u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.proposalId),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.support),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.voter,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `castVoteWithReason(uint256,uint8,string)` and selector `0x7b3c71d3`.
```solidity
function castVoteWithReason(uint256 proposalId, uint8 support, string memory reason) external returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct castVoteWithReasonCall {
        #[allow(missing_docs)]
        pub proposalId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub support: u8,
        #[allow(missing_docs)]
        pub reason: alloy::sol_types::private::String,
    }
    ///Container type for the return parameters of the [`castVoteWithReason(uint256,uint8,string)`](castVoteWithReasonCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct castVoteWithReasonReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::String,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                u8,
                alloy::sol_types::private::String,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<castVoteWithReasonCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: castVoteWithReasonCall) -> Self {
                    (value.proposalId, value.support, value.reason)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for castVoteWithReasonCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        proposalId: tuple.0,
                        support: tuple.1,
                        reason: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<castVoteWithReasonReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: castVoteWithReasonReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for castVoteWithReasonReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for castVoteWithReasonCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::String,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = castVoteWithReasonReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "castVoteWithReason(uint256,uint8,string)";
            const SELECTOR: [u8; 4] = [123u8, 60u8, 113u8, 211u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.proposalId),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.support),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.reason,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `castVoteWithReasonAndParams(uint256,uint8,string,bytes)` and selector `0x5f398a14`.
```solidity
function castVoteWithReasonAndParams(uint256 proposalId, uint8 support, string memory reason, bytes memory params) external returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct castVoteWithReasonAndParamsCall {
        #[allow(missing_docs)]
        pub proposalId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub support: u8,
        #[allow(missing_docs)]
        pub reason: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub params: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`castVoteWithReasonAndParams(uint256,uint8,string,bytes)`](castVoteWithReasonAndParamsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct castVoteWithReasonAndParamsReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                u8,
                alloy::sol_types::private::String,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<castVoteWithReasonAndParamsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: castVoteWithReasonAndParamsCall) -> Self {
                    (value.proposalId, value.support, value.reason, value.params)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for castVoteWithReasonAndParamsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        proposalId: tuple.0,
                        support: tuple.1,
                        reason: tuple.2,
                        params: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<castVoteWithReasonAndParamsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: castVoteWithReasonAndParamsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for castVoteWithReasonAndParamsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for castVoteWithReasonAndParamsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = castVoteWithReasonAndParamsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "castVoteWithReasonAndParams(uint256,uint8,string,bytes)";
            const SELECTOR: [u8; 4] = [95u8, 57u8, 138u8, 20u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.proposalId),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.support),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.reason,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.params,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `castVoteWithReasonAndParamsBySig(uint256,uint8,address,string,bytes,bytes)` and selector `0x5b8d0e0d`.
```solidity
function castVoteWithReasonAndParamsBySig(uint256 proposalId, uint8 support, address voter, string memory reason, bytes memory params, bytes memory signature) external returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct castVoteWithReasonAndParamsBySigCall {
        #[allow(missing_docs)]
        pub proposalId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub support: u8,
        #[allow(missing_docs)]
        pub voter: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub reason: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub params: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`castVoteWithReasonAndParamsBySig(uint256,uint8,address,string,bytes,bytes)`](castVoteWithReasonAndParamsBySigCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct castVoteWithReasonAndParamsBySigReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                u8,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::String,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<castVoteWithReasonAndParamsBySigCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: castVoteWithReasonAndParamsBySigCall) -> Self {
                    (
                        value.proposalId,
                        value.support,
                        value.voter,
                        value.reason,
                        value.params,
                        value.signature,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for castVoteWithReasonAndParamsBySigCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        proposalId: tuple.0,
                        support: tuple.1,
                        voter: tuple.2,
                        reason: tuple.3,
                        params: tuple.4,
                        signature: tuple.5,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<castVoteWithReasonAndParamsBySigReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: castVoteWithReasonAndParamsBySigReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for castVoteWithReasonAndParamsBySigReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for castVoteWithReasonAndParamsBySigCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = castVoteWithReasonAndParamsBySigReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "castVoteWithReasonAndParamsBySig(uint256,uint8,address,string,bytes,bytes)";
            const SELECTOR: [u8; 4] = [91u8, 141u8, 14u8, 13u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.proposalId),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.support),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.voter,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.reason,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.params,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `clock()` and selector `0x91ddadf4`.
```solidity
function clock() external view returns (uint48);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct clockCall {}
    ///Container type for the return parameters of the [`clock()`](clockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct clockReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U48,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<clockCall> for UnderlyingRustTuple<'_> {
                fn from(value: clockCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for clockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<48>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U48,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<clockReturn> for UnderlyingRustTuple<'_> {
                fn from(value: clockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for clockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for clockCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = clockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<48>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "clock()";
            const SELECTOR: [u8; 4] = [145u8, 221u8, 173u8, 244u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `eip712Domain()` and selector `0x84b0196e`.
```solidity
function eip712Domain() external view returns (bytes1 fields, string memory name, string memory version, uint256 chainId, address verifyingContract, bytes32 salt, uint256[] memory extensions);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eip712DomainCall {}
    ///Container type for the return parameters of the [`eip712Domain()`](eip712DomainCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eip712DomainReturn {
        #[allow(missing_docs)]
        pub fields: alloy::sol_types::private::FixedBytes<1>,
        #[allow(missing_docs)]
        pub name: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub version: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub chainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub verifyingContract: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub salt: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub extensions: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<eip712DomainCall> for UnderlyingRustTuple<'_> {
                fn from(value: eip712DomainCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eip712DomainCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<1>,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<1>,
                alloy::sol_types::private::String,
                alloy::sol_types::private::String,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<eip712DomainReturn> for UnderlyingRustTuple<'_> {
                fn from(value: eip712DomainReturn) -> Self {
                    (
                        value.fields,
                        value.name,
                        value.version,
                        value.chainId,
                        value.verifyingContract,
                        value.salt,
                        value.extensions,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eip712DomainReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        fields: tuple.0,
                        name: tuple.1,
                        version: tuple.2,
                        chainId: tuple.3,
                        verifyingContract: tuple.4,
                        salt: tuple.5,
                        extensions: tuple.6,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for eip712DomainCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = eip712DomainReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<1>,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "eip712Domain()";
            const SELECTOR: [u8; 4] = [132u8, 176u8, 25u8, 110u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `execute(address[],uint256[],bytes[],bytes32)` and selector `0x2656227d`.
```solidity
function execute(address[] memory targets, uint256[] memory values, bytes[] memory calldatas, bytes32 descriptionHash) external payable returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeCall {
        #[allow(missing_docs)]
        pub targets: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        #[allow(missing_docs)]
        pub values: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        #[allow(missing_docs)]
        pub calldatas: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
        #[allow(missing_docs)]
        pub descriptionHash: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`execute(address[],uint256[],bytes[],bytes32)`](executeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
                alloy::sol_types::private::FixedBytes<32>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<executeCall> for UnderlyingRustTuple<'_> {
                fn from(value: executeCall) -> Self {
                    (value.targets, value.values, value.calldatas, value.descriptionHash)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targets: tuple.0,
                        values: tuple.1,
                        calldatas: tuple.2,
                        descriptionHash: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<executeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: executeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for executeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = executeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "execute(address[],uint256[],bytes[],bytes32)";
            const SELECTOR: [u8; 4] = [38u8, 86u8, 34u8, 125u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.targets),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.values),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::SolType>::tokenize(&self.calldatas),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.descriptionHash),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getVotes(address,uint256)` and selector `0xeb9019d4`.
```solidity
function getVotes(address account, uint256 timepoint) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getVotesCall {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub timepoint: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getVotes(address,uint256)`](getVotesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getVotesReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getVotesCall> for UnderlyingRustTuple<'_> {
                fn from(value: getVotesCall) -> Self {
                    (value.account, value.timepoint)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getVotesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        account: tuple.0,
                        timepoint: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getVotesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getVotesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getVotesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getVotesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getVotesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getVotes(address,uint256)";
            const SELECTOR: [u8; 4] = [235u8, 144u8, 25u8, 212u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.timepoint),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getVotesWithParams(address,uint256,bytes)` and selector `0x9a802a6d`.
```solidity
function getVotesWithParams(address account, uint256 timepoint, bytes memory params) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getVotesWithParamsCall {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub timepoint: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub params: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`getVotesWithParams(address,uint256,bytes)`](getVotesWithParamsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getVotesWithParamsReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getVotesWithParamsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getVotesWithParamsCall) -> Self {
                    (value.account, value.timepoint, value.params)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getVotesWithParamsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        account: tuple.0,
                        timepoint: tuple.1,
                        params: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getVotesWithParamsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getVotesWithParamsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getVotesWithParamsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getVotesWithParamsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getVotesWithParamsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getVotesWithParams(address,uint256,bytes)";
            const SELECTOR: [u8; 4] = [154u8, 128u8, 42u8, 109u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.timepoint),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.params,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `hasVoted(uint256,address)` and selector `0x43859632`.
```solidity
function hasVoted(uint256 proposalId, address account) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hasVotedCall {
        #[allow(missing_docs)]
        pub proposalId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`hasVoted(uint256,address)`](hasVotedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hasVotedReturn {
        #[allow(missing_docs)]
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<hasVotedCall> for UnderlyingRustTuple<'_> {
                fn from(value: hasVotedCall) -> Self {
                    (value.proposalId, value.account)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hasVotedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        proposalId: tuple.0,
                        account: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<hasVotedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: hasVotedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hasVotedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for hasVotedCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = hasVotedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "hasVoted(uint256,address)";
            const SELECTOR: [u8; 4] = [67u8, 133u8, 150u8, 50u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.proposalId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `hashProposal(address[],uint256[],bytes[],bytes32)` and selector `0xc59057e4`.
```solidity
function hashProposal(address[] memory targets, uint256[] memory values, bytes[] memory calldatas, bytes32 descriptionHash) external pure returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hashProposalCall {
        #[allow(missing_docs)]
        pub targets: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        #[allow(missing_docs)]
        pub values: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        #[allow(missing_docs)]
        pub calldatas: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
        #[allow(missing_docs)]
        pub descriptionHash: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`hashProposal(address[],uint256[],bytes[],bytes32)`](hashProposalCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hashProposalReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
                alloy::sol_types::private::FixedBytes<32>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<hashProposalCall> for UnderlyingRustTuple<'_> {
                fn from(value: hashProposalCall) -> Self {
                    (value.targets, value.values, value.calldatas, value.descriptionHash)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hashProposalCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targets: tuple.0,
                        values: tuple.1,
                        calldatas: tuple.2,
                        descriptionHash: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<hashProposalReturn> for UnderlyingRustTuple<'_> {
                fn from(value: hashProposalReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hashProposalReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for hashProposalCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = hashProposalReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "hashProposal(address[],uint256[],bytes[],bytes32)";
            const SELECTOR: [u8; 4] = [197u8, 144u8, 87u8, 228u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.targets),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.values),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::SolType>::tokenize(&self.calldatas),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.descriptionHash),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `initialize(address,address,uint48,uint32,uint256,uint256)` and selector `0x22f120de`.
```solidity
function initialize(address token, address timelock, uint48 initialVotingDelay, uint32 initialVotingPeriod, uint256 initialProposalThreshold, uint256 quorumPercent) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub timelock: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub initialVotingDelay: alloy::sol_types::private::primitives::aliases::U48,
        #[allow(missing_docs)]
        pub initialVotingPeriod: u32,
        #[allow(missing_docs)]
        pub initialProposalThreshold: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub quorumPercent: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`initialize(address,address,uint48,uint32,uint256,uint256)`](initializeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<48>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U48,
                u32,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (
                        value.token,
                        value.timelock,
                        value.initialVotingDelay,
                        value.initialVotingPeriod,
                        value.initialProposalThreshold,
                        value.quorumPercent,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        token: tuple.0,
                        timelock: tuple.1,
                        initialVotingDelay: tuple.2,
                        initialVotingPeriod: tuple.3,
                        initialProposalThreshold: tuple.4,
                        quorumPercent: tuple.5,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: initializeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<48>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize(address,address,uint48,uint32,uint256,uint256)";
            const SELECTOR: [u8; 4] = [34u8, 241u8, 32u8, 222u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.timelock,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        48,
                    > as alloy_sol_types::SolType>::tokenize(&self.initialVotingDelay),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.initialVotingPeriod),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.initialProposalThreshold,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumPercent),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `name()` and selector `0x06fdde03`.
```solidity
function name() external view returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nameCall {}
    ///Container type for the return parameters of the [`name()`](nameCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nameReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<nameCall> for UnderlyingRustTuple<'_> {
                fn from(value: nameCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nameCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<nameReturn> for UnderlyingRustTuple<'_> {
                fn from(value: nameReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nameReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for nameCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = nameReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "name()";
            const SELECTOR: [u8; 4] = [6u8, 253u8, 222u8, 3u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `nonces(address)` and selector `0x7ecebe00`.
```solidity
function nonces(address owner) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct noncesCall {
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`nonces(address)`](noncesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct noncesReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<noncesCall> for UnderlyingRustTuple<'_> {
                fn from(value: noncesCall) -> Self {
                    (value.owner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for noncesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { owner: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<noncesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: noncesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for noncesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for noncesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = noncesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "nonces(address)";
            const SELECTOR: [u8; 4] = [126u8, 206u8, 190u8, 0u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)` and selector `0xbc197c81`.
```solidity
function onERC1155BatchReceived(address, address, uint256[] memory, uint256[] memory, bytes memory) external returns (bytes4);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onERC1155BatchReceivedCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _2: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        #[allow(missing_docs)]
        pub _3: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        #[allow(missing_docs)]
        pub _4: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)`](onERC1155BatchReceivedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onERC1155BatchReceivedReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<4>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<onERC1155BatchReceivedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: onERC1155BatchReceivedCall) -> Self {
                    (value._0, value._1, value._2, value._3, value._4)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for onERC1155BatchReceivedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                        _2: tuple.2,
                        _3: tuple.3,
                        _4: tuple.4,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<4>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<onERC1155BatchReceivedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: onERC1155BatchReceivedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for onERC1155BatchReceivedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for onERC1155BatchReceivedCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = onERC1155BatchReceivedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)";
            const SELECTOR: [u8; 4] = [188u8, 25u8, 124u8, 129u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._1,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self._2),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self._3),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._4,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `onERC1155Received(address,address,uint256,uint256,bytes)` and selector `0xf23a6e61`.
```solidity
function onERC1155Received(address, address, uint256, uint256, bytes memory) external returns (bytes4);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onERC1155ReceivedCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _2: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _3: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _4: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`onERC1155Received(address,address,uint256,uint256,bytes)`](onERC1155ReceivedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onERC1155ReceivedReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<4>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<onERC1155ReceivedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: onERC1155ReceivedCall) -> Self {
                    (value._0, value._1, value._2, value._3, value._4)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for onERC1155ReceivedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                        _2: tuple.2,
                        _3: tuple.3,
                        _4: tuple.4,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<4>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<onERC1155ReceivedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: onERC1155ReceivedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for onERC1155ReceivedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for onERC1155ReceivedCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = onERC1155ReceivedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "onERC1155Received(address,address,uint256,uint256,bytes)";
            const SELECTOR: [u8; 4] = [242u8, 58u8, 110u8, 97u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._1,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._2),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._3),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._4,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `onERC721Received(address,address,uint256,bytes)` and selector `0x150b7a02`.
```solidity
function onERC721Received(address, address, uint256, bytes memory) external returns (bytes4);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onERC721ReceivedCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _2: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _3: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`onERC721Received(address,address,uint256,bytes)`](onERC721ReceivedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onERC721ReceivedReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<4>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<onERC721ReceivedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: onERC721ReceivedCall) -> Self {
                    (value._0, value._1, value._2, value._3)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for onERC721ReceivedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                        _2: tuple.2,
                        _3: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<4>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<onERC721ReceivedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: onERC721ReceivedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for onERC721ReceivedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for onERC721ReceivedCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = onERC721ReceivedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "onERC721Received(address,address,uint256,bytes)";
            const SELECTOR: [u8; 4] = [21u8, 11u8, 122u8, 2u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._1,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._2),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._3,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `proposalDeadline(uint256)` and selector `0xc01f9e37`.
```solidity
function proposalDeadline(uint256 proposalId) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposalDeadlineCall {
        #[allow(missing_docs)]
        pub proposalId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`proposalDeadline(uint256)`](proposalDeadlineCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposalDeadlineReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proposalDeadlineCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: proposalDeadlineCall) -> Self {
                    (value.proposalId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proposalDeadlineCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { proposalId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proposalDeadlineReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: proposalDeadlineReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proposalDeadlineReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proposalDeadlineCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = proposalDeadlineReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proposalDeadline(uint256)";
            const SELECTOR: [u8; 4] = [192u8, 31u8, 158u8, 55u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.proposalId),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `proposalEta(uint256)` and selector `0xab58fb8e`.
```solidity
function proposalEta(uint256 proposalId) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposalEtaCall {
        #[allow(missing_docs)]
        pub proposalId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`proposalEta(uint256)`](proposalEtaCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposalEtaReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proposalEtaCall> for UnderlyingRustTuple<'_> {
                fn from(value: proposalEtaCall) -> Self {
                    (value.proposalId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proposalEtaCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { proposalId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proposalEtaReturn> for UnderlyingRustTuple<'_> {
                fn from(value: proposalEtaReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proposalEtaReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proposalEtaCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = proposalEtaReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proposalEta(uint256)";
            const SELECTOR: [u8; 4] = [171u8, 88u8, 251u8, 142u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.proposalId),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `proposalNeedsQueuing(uint256)` and selector `0xa9a95294`.
```solidity
function proposalNeedsQueuing(uint256 proposalId) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposalNeedsQueuingCall {
        #[allow(missing_docs)]
        pub proposalId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`proposalNeedsQueuing(uint256)`](proposalNeedsQueuingCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposalNeedsQueuingReturn {
        #[allow(missing_docs)]
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proposalNeedsQueuingCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: proposalNeedsQueuingCall) -> Self {
                    (value.proposalId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proposalNeedsQueuingCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { proposalId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proposalNeedsQueuingReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: proposalNeedsQueuingReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proposalNeedsQueuingReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proposalNeedsQueuingCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = proposalNeedsQueuingReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proposalNeedsQueuing(uint256)";
            const SELECTOR: [u8; 4] = [169u8, 169u8, 82u8, 148u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.proposalId),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `proposalProposer(uint256)` and selector `0x143489d0`.
```solidity
function proposalProposer(uint256 proposalId) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposalProposerCall {
        #[allow(missing_docs)]
        pub proposalId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`proposalProposer(uint256)`](proposalProposerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposalProposerReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proposalProposerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: proposalProposerCall) -> Self {
                    (value.proposalId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proposalProposerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { proposalId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proposalProposerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: proposalProposerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proposalProposerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proposalProposerCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = proposalProposerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proposalProposer(uint256)";
            const SELECTOR: [u8; 4] = [20u8, 52u8, 137u8, 208u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.proposalId),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `proposalSnapshot(uint256)` and selector `0x2d63f693`.
```solidity
function proposalSnapshot(uint256 proposalId) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposalSnapshotCall {
        #[allow(missing_docs)]
        pub proposalId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`proposalSnapshot(uint256)`](proposalSnapshotCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposalSnapshotReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proposalSnapshotCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: proposalSnapshotCall) -> Self {
                    (value.proposalId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proposalSnapshotCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { proposalId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proposalSnapshotReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: proposalSnapshotReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proposalSnapshotReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proposalSnapshotCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = proposalSnapshotReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proposalSnapshot(uint256)";
            const SELECTOR: [u8; 4] = [45u8, 99u8, 246u8, 147u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.proposalId),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `proposalThreshold()` and selector `0xb58131b0`.
```solidity
function proposalThreshold() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposalThresholdCall {}
    ///Container type for the return parameters of the [`proposalThreshold()`](proposalThresholdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposalThresholdReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proposalThresholdCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: proposalThresholdCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proposalThresholdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proposalThresholdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: proposalThresholdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proposalThresholdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proposalThresholdCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = proposalThresholdReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proposalThreshold()";
            const SELECTOR: [u8; 4] = [181u8, 129u8, 49u8, 176u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `proposalVotes(uint256)` and selector `0x544ffc9c`.
```solidity
function proposalVotes(uint256 proposalId) external view returns (uint256 againstVotes, uint256 forVotes, uint256 abstainVotes);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposalVotesCall {
        #[allow(missing_docs)]
        pub proposalId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`proposalVotes(uint256)`](proposalVotesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposalVotesReturn {
        #[allow(missing_docs)]
        pub againstVotes: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub forVotes: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub abstainVotes: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proposalVotesCall> for UnderlyingRustTuple<'_> {
                fn from(value: proposalVotesCall) -> Self {
                    (value.proposalId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proposalVotesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { proposalId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proposalVotesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: proposalVotesReturn) -> Self {
                    (value.againstVotes, value.forVotes, value.abstainVotes)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proposalVotesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        againstVotes: tuple.0,
                        forVotes: tuple.1,
                        abstainVotes: tuple.2,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proposalVotesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = proposalVotesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proposalVotes(uint256)";
            const SELECTOR: [u8; 4] = [84u8, 79u8, 252u8, 156u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.proposalId),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `propose(address[],uint256[],bytes[],string)` and selector `0x7d5e81e2`.
```solidity
function propose(address[] memory targets, uint256[] memory values, bytes[] memory calldatas, string memory description) external returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposeCall {
        #[allow(missing_docs)]
        pub targets: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        #[allow(missing_docs)]
        pub values: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        #[allow(missing_docs)]
        pub calldatas: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
        #[allow(missing_docs)]
        pub description: alloy::sol_types::private::String,
    }
    ///Container type for the return parameters of the [`propose(address[],uint256[],bytes[],string)`](proposeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proposeReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
                alloy::sol_types::sol_data::String,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
                alloy::sol_types::private::String,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proposeCall> for UnderlyingRustTuple<'_> {
                fn from(value: proposeCall) -> Self {
                    (value.targets, value.values, value.calldatas, value.description)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proposeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targets: tuple.0,
                        values: tuple.1,
                        calldatas: tuple.2,
                        description: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proposeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: proposeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proposeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proposeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
                alloy::sol_types::sol_data::String,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = proposeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "propose(address[],uint256[],bytes[],string)";
            const SELECTOR: [u8; 4] = [125u8, 94u8, 129u8, 226u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.targets),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.values),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::SolType>::tokenize(&self.calldatas),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.description,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `proxiableUUID()` and selector `0x52d1902d`.
```solidity
function proxiableUUID() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proxiableUUIDCall {}
    ///Container type for the return parameters of the [`proxiableUUID()`](proxiableUUIDCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proxiableUUIDReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proxiableUUIDCall> for UnderlyingRustTuple<'_> {
                fn from(value: proxiableUUIDCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proxiableUUIDCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proxiableUUIDReturn> for UnderlyingRustTuple<'_> {
                fn from(value: proxiableUUIDReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proxiableUUIDReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proxiableUUIDCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = proxiableUUIDReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proxiableUUID()";
            const SELECTOR: [u8; 4] = [82u8, 209u8, 144u8, 45u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `queue(address[],uint256[],bytes[],bytes32)` and selector `0x160cbed7`.
```solidity
function queue(address[] memory targets, uint256[] memory values, bytes[] memory calldatas, bytes32 descriptionHash) external returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct queueCall {
        #[allow(missing_docs)]
        pub targets: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        #[allow(missing_docs)]
        pub values: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        #[allow(missing_docs)]
        pub calldatas: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
        #[allow(missing_docs)]
        pub descriptionHash: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`queue(address[],uint256[],bytes[],bytes32)`](queueCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct queueReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
                alloy::sol_types::private::FixedBytes<32>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<queueCall> for UnderlyingRustTuple<'_> {
                fn from(value: queueCall) -> Self {
                    (value.targets, value.values, value.calldatas, value.descriptionHash)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for queueCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targets: tuple.0,
                        values: tuple.1,
                        calldatas: tuple.2,
                        descriptionHash: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<queueReturn> for UnderlyingRustTuple<'_> {
                fn from(value: queueReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for queueReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for queueCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = queueReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "queue(address[],uint256[],bytes[],bytes32)";
            const SELECTOR: [u8; 4] = [22u8, 12u8, 190u8, 215u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.targets),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.values),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::SolType>::tokenize(&self.calldatas),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.descriptionHash),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `quorum(uint256)` and selector `0xf8ce560a`.
```solidity
function quorum(uint256 blockNumber) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct quorumCall {
        #[allow(missing_docs)]
        pub blockNumber: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`quorum(uint256)`](quorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct quorumReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<quorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: quorumCall) -> Self {
                    (value.blockNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for quorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { blockNumber: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<quorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: quorumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for quorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for quorumCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = quorumReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "quorum(uint256)";
            const SELECTOR: [u8; 4] = [248u8, 206u8, 86u8, 10u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.blockNumber),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `quorumDenominator()` and selector `0x97c3d334`.
```solidity
function quorumDenominator() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct quorumDenominatorCall {}
    ///Container type for the return parameters of the [`quorumDenominator()`](quorumDenominatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct quorumDenominatorReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<quorumDenominatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: quorumDenominatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for quorumDenominatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<quorumDenominatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: quorumDenominatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for quorumDenominatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for quorumDenominatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = quorumDenominatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "quorumDenominator()";
            const SELECTOR: [u8; 4] = [151u8, 195u8, 211u8, 52u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `quorumNumerator(uint256)` and selector `0x60c4247f`.
```solidity
function quorumNumerator(uint256 timepoint) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct quorumNumerator_0Call {
        #[allow(missing_docs)]
        pub timepoint: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`quorumNumerator(uint256)`](quorumNumerator_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct quorumNumerator_0Return {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<quorumNumerator_0Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: quorumNumerator_0Call) -> Self {
                    (value.timepoint,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for quorumNumerator_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { timepoint: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<quorumNumerator_0Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: quorumNumerator_0Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for quorumNumerator_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for quorumNumerator_0Call {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = quorumNumerator_0Return;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "quorumNumerator(uint256)";
            const SELECTOR: [u8; 4] = [96u8, 196u8, 36u8, 127u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.timepoint),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `quorumNumerator()` and selector `0xa7713a70`.
```solidity
function quorumNumerator() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct quorumNumerator_1Call {}
    ///Container type for the return parameters of the [`quorumNumerator()`](quorumNumerator_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct quorumNumerator_1Return {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<quorumNumerator_1Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: quorumNumerator_1Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for quorumNumerator_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<quorumNumerator_1Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: quorumNumerator_1Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for quorumNumerator_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for quorumNumerator_1Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = quorumNumerator_1Return;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "quorumNumerator()";
            const SELECTOR: [u8; 4] = [167u8, 113u8, 58u8, 112u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `relay(address,uint256,bytes)` and selector `0xc28bc2fa`.
```solidity
function relay(address target, uint256 value, bytes memory data) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct relayCall {
        #[allow(missing_docs)]
        pub target: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`relay(address,uint256,bytes)`](relayCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct relayReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<relayCall> for UnderlyingRustTuple<'_> {
                fn from(value: relayCall) -> Self {
                    (value.target, value.value, value.data)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for relayCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        target: tuple.0,
                        value: tuple.1,
                        data: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<relayReturn> for UnderlyingRustTuple<'_> {
                fn from(value: relayReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for relayReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for relayCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = relayReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "relay(address,uint256,bytes)";
            const SELECTOR: [u8; 4] = [194u8, 139u8, 194u8, 250u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.target,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `setProposalThreshold(uint256)` and selector `0xece40cc1`.
```solidity
function setProposalThreshold(uint256 newProposalThreshold) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setProposalThresholdCall {
        #[allow(missing_docs)]
        pub newProposalThreshold: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`setProposalThreshold(uint256)`](setProposalThresholdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setProposalThresholdReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setProposalThresholdCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setProposalThresholdCall) -> Self {
                    (value.newProposalThreshold,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setProposalThresholdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newProposalThreshold: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setProposalThresholdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setProposalThresholdReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setProposalThresholdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setProposalThresholdCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setProposalThresholdReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setProposalThreshold(uint256)";
            const SELECTOR: [u8; 4] = [236u8, 228u8, 12u8, 193u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newProposalThreshold),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `setVotingDelay(uint48)` and selector `0x79051887`.
```solidity
function setVotingDelay(uint48 newVotingDelay) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setVotingDelayCall {
        #[allow(missing_docs)]
        pub newVotingDelay: alloy::sol_types::private::primitives::aliases::U48,
    }
    ///Container type for the return parameters of the [`setVotingDelay(uint48)`](setVotingDelayCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setVotingDelayReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<48>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U48,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setVotingDelayCall> for UnderlyingRustTuple<'_> {
                fn from(value: setVotingDelayCall) -> Self {
                    (value.newVotingDelay,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setVotingDelayCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newVotingDelay: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setVotingDelayReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setVotingDelayReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setVotingDelayReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setVotingDelayCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<48>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setVotingDelayReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setVotingDelay(uint48)";
            const SELECTOR: [u8; 4] = [121u8, 5u8, 24u8, 135u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        48,
                    > as alloy_sol_types::SolType>::tokenize(&self.newVotingDelay),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `setVotingPeriod(uint32)` and selector `0xe540d01d`.
```solidity
function setVotingPeriod(uint32 newVotingPeriod) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setVotingPeriodCall {
        #[allow(missing_docs)]
        pub newVotingPeriod: u32,
    }
    ///Container type for the return parameters of the [`setVotingPeriod(uint32)`](setVotingPeriodCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setVotingPeriodReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setVotingPeriodCall> for UnderlyingRustTuple<'_> {
                fn from(value: setVotingPeriodCall) -> Self {
                    (value.newVotingPeriod,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setVotingPeriodCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newVotingPeriod: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setVotingPeriodReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setVotingPeriodReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setVotingPeriodReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setVotingPeriodCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setVotingPeriodReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setVotingPeriod(uint32)";
            const SELECTOR: [u8; 4] = [229u8, 64u8, 208u8, 29u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.newVotingPeriod),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `state(uint256)` and selector `0x3e4f49e6`.
```solidity
function state(uint256 proposalId) external view returns (IGovernor.ProposalState);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stateCall {
        #[allow(missing_docs)]
        pub proposalId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`state(uint256)`](stateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stateReturn {
        #[allow(missing_docs)]
        pub _0: <IGovernor::ProposalState as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<stateCall> for UnderlyingRustTuple<'_> {
                fn from(value: stateCall) -> Self {
                    (value.proposalId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { proposalId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IGovernor::ProposalState,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IGovernor::ProposalState as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<stateReturn> for UnderlyingRustTuple<'_> {
                fn from(value: stateReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for stateCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = stateReturn;
            type ReturnTuple<'a> = (IGovernor::ProposalState,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "state(uint256)";
            const SELECTOR: [u8; 4] = [62u8, 79u8, 73u8, 230u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.proposalId),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`.
```solidity
function supportsInterface(bytes4 interfaceId) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct supportsInterfaceCall {
        #[allow(missing_docs)]
        pub interfaceId: alloy::sol_types::private::FixedBytes<4>,
    }
    ///Container type for the return parameters of the [`supportsInterface(bytes4)`](supportsInterfaceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct supportsInterfaceReturn {
        #[allow(missing_docs)]
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<4>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<supportsInterfaceCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: supportsInterfaceCall) -> Self {
                    (value.interfaceId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for supportsInterfaceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { interfaceId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<supportsInterfaceReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: supportsInterfaceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for supportsInterfaceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for supportsInterfaceCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = supportsInterfaceReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "supportsInterface(bytes4)";
            const SELECTOR: [u8; 4] = [1u8, 255u8, 201u8, 167u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        4,
                    > as alloy_sol_types::SolType>::tokenize(&self.interfaceId),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `timelock()` and selector `0xd33219b4`.
```solidity
function timelock() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct timelockCall {}
    ///Container type for the return parameters of the [`timelock()`](timelockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct timelockReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<timelockCall> for UnderlyingRustTuple<'_> {
                fn from(value: timelockCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for timelockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<timelockReturn> for UnderlyingRustTuple<'_> {
                fn from(value: timelockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for timelockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for timelockCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = timelockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "timelock()";
            const SELECTOR: [u8; 4] = [211u8, 50u8, 25u8, 180u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `token()` and selector `0xfc0c546a`.
```solidity
function token() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct tokenCall {}
    ///Container type for the return parameters of the [`token()`](tokenCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct tokenReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<tokenCall> for UnderlyingRustTuple<'_> {
                fn from(value: tokenCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for tokenCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<tokenReturn> for UnderlyingRustTuple<'_> {
                fn from(value: tokenReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for tokenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for tokenCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = tokenReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "token()";
            const SELECTOR: [u8; 4] = [252u8, 12u8, 84u8, 106u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `updateQuorumNumerator(uint256)` and selector `0x06f3f9e6`.
```solidity
function updateQuorumNumerator(uint256 newQuorumNumerator) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateQuorumNumeratorCall {
        #[allow(missing_docs)]
        pub newQuorumNumerator: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`updateQuorumNumerator(uint256)`](updateQuorumNumeratorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateQuorumNumeratorReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateQuorumNumeratorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateQuorumNumeratorCall) -> Self {
                    (value.newQuorumNumerator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateQuorumNumeratorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newQuorumNumerator: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateQuorumNumeratorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateQuorumNumeratorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateQuorumNumeratorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateQuorumNumeratorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateQuorumNumeratorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateQuorumNumerator(uint256)";
            const SELECTOR: [u8; 4] = [6u8, 243u8, 249u8, 230u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newQuorumNumerator),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `updateTimelock(address)` and selector `0xa890c910`.
```solidity
function updateTimelock(address newTimelock) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateTimelockCall {
        #[allow(missing_docs)]
        pub newTimelock: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`updateTimelock(address)`](updateTimelockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateTimelockReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateTimelockCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateTimelockCall) -> Self {
                    (value.newTimelock,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateTimelockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newTimelock: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateTimelockReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateTimelockReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateTimelockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateTimelockCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateTimelockReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateTimelock(address)";
            const SELECTOR: [u8; 4] = [168u8, 144u8, 201u8, 16u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newTimelock,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `upgradeToAndCall(address,bytes)` and selector `0x4f1ef286`.
```solidity
function upgradeToAndCall(address newImplementation, bytes memory data) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct upgradeToAndCallCall {
        #[allow(missing_docs)]
        pub newImplementation: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`upgradeToAndCall(address,bytes)`](upgradeToAndCallCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct upgradeToAndCallReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<upgradeToAndCallCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: upgradeToAndCallCall) -> Self {
                    (value.newImplementation, value.data)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for upgradeToAndCallCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newImplementation: tuple.0,
                        data: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<upgradeToAndCallReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: upgradeToAndCallReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for upgradeToAndCallReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for upgradeToAndCallCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = upgradeToAndCallReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "upgradeToAndCall(address,bytes)";
            const SELECTOR: [u8; 4] = [79u8, 30u8, 242u8, 134u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newImplementation,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `version()` and selector `0x54fd4d50`.
```solidity
function version() external view returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct versionCall {}
    ///Container type for the return parameters of the [`version()`](versionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct versionReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<versionCall> for UnderlyingRustTuple<'_> {
                fn from(value: versionCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for versionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<versionReturn> for UnderlyingRustTuple<'_> {
                fn from(value: versionReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for versionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for versionCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = versionReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "version()";
            const SELECTOR: [u8; 4] = [84u8, 253u8, 77u8, 80u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `votingDelay()` and selector `0x3932abb1`.
```solidity
function votingDelay() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct votingDelayCall {}
    ///Container type for the return parameters of the [`votingDelay()`](votingDelayCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct votingDelayReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<votingDelayCall> for UnderlyingRustTuple<'_> {
                fn from(value: votingDelayCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for votingDelayCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<votingDelayReturn> for UnderlyingRustTuple<'_> {
                fn from(value: votingDelayReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for votingDelayReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for votingDelayCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = votingDelayReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "votingDelay()";
            const SELECTOR: [u8; 4] = [57u8, 50u8, 171u8, 177u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `votingPeriod()` and selector `0x02a251a3`.
```solidity
function votingPeriod() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct votingPeriodCall {}
    ///Container type for the return parameters of the [`votingPeriod()`](votingPeriodCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct votingPeriodReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<votingPeriodCall> for UnderlyingRustTuple<'_> {
                fn from(value: votingPeriodCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for votingPeriodCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<votingPeriodReturn> for UnderlyingRustTuple<'_> {
                fn from(value: votingPeriodReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for votingPeriodReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for votingPeriodCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = votingPeriodReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "votingPeriod()";
            const SELECTOR: [u8; 4] = [2u8, 162u8, 81u8, 163u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`TangleGovernor`](self) function calls.
    pub enum TangleGovernorCalls {
        #[allow(missing_docs)]
        BALLOT_TYPEHASH(BALLOT_TYPEHASHCall),
        #[allow(missing_docs)]
        CLOCK_MODE(CLOCK_MODECall),
        #[allow(missing_docs)]
        COUNTING_MODE(COUNTING_MODECall),
        #[allow(missing_docs)]
        EXTENDED_BALLOT_TYPEHASH(EXTENDED_BALLOT_TYPEHASHCall),
        #[allow(missing_docs)]
        UPGRADE_INTERFACE_VERSION(UPGRADE_INTERFACE_VERSIONCall),
        #[allow(missing_docs)]
        cancel(cancelCall),
        #[allow(missing_docs)]
        castVote(castVoteCall),
        #[allow(missing_docs)]
        castVoteBySig(castVoteBySigCall),
        #[allow(missing_docs)]
        castVoteWithReason(castVoteWithReasonCall),
        #[allow(missing_docs)]
        castVoteWithReasonAndParams(castVoteWithReasonAndParamsCall),
        #[allow(missing_docs)]
        castVoteWithReasonAndParamsBySig(castVoteWithReasonAndParamsBySigCall),
        #[allow(missing_docs)]
        clock(clockCall),
        #[allow(missing_docs)]
        eip712Domain(eip712DomainCall),
        #[allow(missing_docs)]
        execute(executeCall),
        #[allow(missing_docs)]
        getVotes(getVotesCall),
        #[allow(missing_docs)]
        getVotesWithParams(getVotesWithParamsCall),
        #[allow(missing_docs)]
        hasVoted(hasVotedCall),
        #[allow(missing_docs)]
        hashProposal(hashProposalCall),
        #[allow(missing_docs)]
        initialize(initializeCall),
        #[allow(missing_docs)]
        name(nameCall),
        #[allow(missing_docs)]
        nonces(noncesCall),
        #[allow(missing_docs)]
        onERC1155BatchReceived(onERC1155BatchReceivedCall),
        #[allow(missing_docs)]
        onERC1155Received(onERC1155ReceivedCall),
        #[allow(missing_docs)]
        onERC721Received(onERC721ReceivedCall),
        #[allow(missing_docs)]
        proposalDeadline(proposalDeadlineCall),
        #[allow(missing_docs)]
        proposalEta(proposalEtaCall),
        #[allow(missing_docs)]
        proposalNeedsQueuing(proposalNeedsQueuingCall),
        #[allow(missing_docs)]
        proposalProposer(proposalProposerCall),
        #[allow(missing_docs)]
        proposalSnapshot(proposalSnapshotCall),
        #[allow(missing_docs)]
        proposalThreshold(proposalThresholdCall),
        #[allow(missing_docs)]
        proposalVotes(proposalVotesCall),
        #[allow(missing_docs)]
        propose(proposeCall),
        #[allow(missing_docs)]
        proxiableUUID(proxiableUUIDCall),
        #[allow(missing_docs)]
        queue(queueCall),
        #[allow(missing_docs)]
        quorum(quorumCall),
        #[allow(missing_docs)]
        quorumDenominator(quorumDenominatorCall),
        #[allow(missing_docs)]
        quorumNumerator_0(quorumNumerator_0Call),
        #[allow(missing_docs)]
        quorumNumerator_1(quorumNumerator_1Call),
        #[allow(missing_docs)]
        relay(relayCall),
        #[allow(missing_docs)]
        setProposalThreshold(setProposalThresholdCall),
        #[allow(missing_docs)]
        setVotingDelay(setVotingDelayCall),
        #[allow(missing_docs)]
        setVotingPeriod(setVotingPeriodCall),
        #[allow(missing_docs)]
        state(stateCall),
        #[allow(missing_docs)]
        supportsInterface(supportsInterfaceCall),
        #[allow(missing_docs)]
        timelock(timelockCall),
        #[allow(missing_docs)]
        token(tokenCall),
        #[allow(missing_docs)]
        updateQuorumNumerator(updateQuorumNumeratorCall),
        #[allow(missing_docs)]
        updateTimelock(updateTimelockCall),
        #[allow(missing_docs)]
        upgradeToAndCall(upgradeToAndCallCall),
        #[allow(missing_docs)]
        version(versionCall),
        #[allow(missing_docs)]
        votingDelay(votingDelayCall),
        #[allow(missing_docs)]
        votingPeriod(votingPeriodCall),
    }
    #[automatically_derived]
    impl TangleGovernorCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [1u8, 255u8, 201u8, 167u8],
            [2u8, 162u8, 81u8, 163u8],
            [6u8, 243u8, 249u8, 230u8],
            [6u8, 253u8, 222u8, 3u8],
            [20u8, 52u8, 137u8, 208u8],
            [21u8, 11u8, 122u8, 2u8],
            [22u8, 12u8, 190u8, 215u8],
            [34u8, 241u8, 32u8, 222u8],
            [38u8, 86u8, 34u8, 125u8],
            [45u8, 99u8, 246u8, 147u8],
            [47u8, 227u8, 226u8, 97u8],
            [57u8, 50u8, 171u8, 177u8],
            [62u8, 79u8, 73u8, 230u8],
            [67u8, 133u8, 150u8, 50u8],
            [69u8, 33u8, 21u8, 214u8],
            [75u8, 245u8, 215u8, 233u8],
            [79u8, 30u8, 242u8, 134u8],
            [82u8, 209u8, 144u8, 45u8],
            [84u8, 79u8, 252u8, 156u8],
            [84u8, 253u8, 77u8, 80u8],
            [86u8, 120u8, 19u8, 136u8],
            [91u8, 141u8, 14u8, 13u8],
            [95u8, 57u8, 138u8, 20u8],
            [96u8, 196u8, 36u8, 127u8],
            [121u8, 5u8, 24u8, 135u8],
            [123u8, 60u8, 113u8, 211u8],
            [125u8, 94u8, 129u8, 226u8],
            [126u8, 206u8, 190u8, 0u8],
            [132u8, 176u8, 25u8, 110u8],
            [143u8, 242u8, 98u8, 227u8],
            [145u8, 221u8, 173u8, 244u8],
            [151u8, 195u8, 211u8, 52u8],
            [154u8, 128u8, 42u8, 109u8],
            [167u8, 113u8, 58u8, 112u8],
            [168u8, 144u8, 201u8, 16u8],
            [169u8, 169u8, 82u8, 148u8],
            [171u8, 88u8, 251u8, 142u8],
            [173u8, 60u8, 177u8, 204u8],
            [181u8, 129u8, 49u8, 176u8],
            [188u8, 25u8, 124u8, 129u8],
            [192u8, 31u8, 158u8, 55u8],
            [194u8, 139u8, 194u8, 250u8],
            [197u8, 144u8, 87u8, 228u8],
            [211u8, 50u8, 25u8, 180u8],
            [221u8, 78u8, 43u8, 165u8],
            [222u8, 170u8, 167u8, 204u8],
            [229u8, 64u8, 208u8, 29u8],
            [235u8, 144u8, 25u8, 212u8],
            [236u8, 228u8, 12u8, 193u8],
            [242u8, 58u8, 110u8, 97u8],
            [248u8, 206u8, 86u8, 10u8],
            [252u8, 12u8, 84u8, 106u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for TangleGovernorCalls {
        const NAME: &'static str = "TangleGovernorCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 52usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::BALLOT_TYPEHASH(_) => {
                    <BALLOT_TYPEHASHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::CLOCK_MODE(_) => {
                    <CLOCK_MODECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::COUNTING_MODE(_) => {
                    <COUNTING_MODECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::EXTENDED_BALLOT_TYPEHASH(_) => {
                    <EXTENDED_BALLOT_TYPEHASHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::UPGRADE_INTERFACE_VERSION(_) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::cancel(_) => <cancelCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::castVote(_) => <castVoteCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::castVoteBySig(_) => {
                    <castVoteBySigCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::castVoteWithReason(_) => {
                    <castVoteWithReasonCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::castVoteWithReasonAndParams(_) => {
                    <castVoteWithReasonAndParamsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::castVoteWithReasonAndParamsBySig(_) => {
                    <castVoteWithReasonAndParamsBySigCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::clock(_) => <clockCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::eip712Domain(_) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::execute(_) => <executeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getVotes(_) => <getVotesCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getVotesWithParams(_) => {
                    <getVotesWithParamsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::hasVoted(_) => <hasVotedCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::hashProposal(_) => {
                    <hashProposalCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::name(_) => <nameCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::nonces(_) => <noncesCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::onERC1155BatchReceived(_) => {
                    <onERC1155BatchReceivedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::onERC1155Received(_) => {
                    <onERC1155ReceivedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::onERC721Received(_) => {
                    <onERC721ReceivedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proposalDeadline(_) => {
                    <proposalDeadlineCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proposalEta(_) => {
                    <proposalEtaCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proposalNeedsQueuing(_) => {
                    <proposalNeedsQueuingCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proposalProposer(_) => {
                    <proposalProposerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proposalSnapshot(_) => {
                    <proposalSnapshotCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proposalThreshold(_) => {
                    <proposalThresholdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proposalVotes(_) => {
                    <proposalVotesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::propose(_) => <proposeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::proxiableUUID(_) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::queue(_) => <queueCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::quorum(_) => <quorumCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::quorumDenominator(_) => {
                    <quorumDenominatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::quorumNumerator_0(_) => {
                    <quorumNumerator_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::quorumNumerator_1(_) => {
                    <quorumNumerator_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::relay(_) => <relayCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::setProposalThreshold(_) => {
                    <setProposalThresholdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setVotingDelay(_) => {
                    <setVotingDelayCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setVotingPeriod(_) => {
                    <setVotingPeriodCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::state(_) => <stateCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::supportsInterface(_) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::timelock(_) => <timelockCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::token(_) => <tokenCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::updateQuorumNumerator(_) => {
                    <updateQuorumNumeratorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateTimelock(_) => {
                    <updateTimelockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::upgradeToAndCall(_) => {
                    <upgradeToAndCallCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::version(_) => <versionCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::votingDelay(_) => {
                    <votingDelayCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::votingPeriod(_) => {
                    <votingPeriodCall as alloy_sol_types::SolCall>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<TangleGovernorCalls>] = &[
                {
                    fn supportsInterface(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::supportsInterface)
                    }
                    supportsInterface
                },
                {
                    fn votingPeriod(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <votingPeriodCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::votingPeriod)
                    }
                    votingPeriod
                },
                {
                    fn updateQuorumNumerator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <updateQuorumNumeratorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::updateQuorumNumerator)
                    }
                    updateQuorumNumerator
                },
                {
                    fn name(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <nameCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::name)
                    }
                    name
                },
                {
                    fn proposalProposer(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <proposalProposerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::proposalProposer)
                    }
                    proposalProposer
                },
                {
                    fn onERC721Received(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <onERC721ReceivedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::onERC721Received)
                    }
                    onERC721Received
                },
                {
                    fn queue(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <queueCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::queue)
                    }
                    queue
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::initialize)
                    }
                    initialize
                },
                {
                    fn execute(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <executeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::execute)
                    }
                    execute
                },
                {
                    fn proposalSnapshot(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <proposalSnapshotCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::proposalSnapshot)
                    }
                    proposalSnapshot
                },
                {
                    fn EXTENDED_BALLOT_TYPEHASH(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <EXTENDED_BALLOT_TYPEHASHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::EXTENDED_BALLOT_TYPEHASH)
                    }
                    EXTENDED_BALLOT_TYPEHASH
                },
                {
                    fn votingDelay(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <votingDelayCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::votingDelay)
                    }
                    votingDelay
                },
                {
                    fn state(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <stateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::state)
                    }
                    state
                },
                {
                    fn hasVoted(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <hasVotedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::hasVoted)
                    }
                    hasVoted
                },
                {
                    fn cancel(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <cancelCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::cancel)
                    }
                    cancel
                },
                {
                    fn CLOCK_MODE(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <CLOCK_MODECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::CLOCK_MODE)
                    }
                    CLOCK_MODE
                },
                {
                    fn upgradeToAndCall(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::upgradeToAndCall)
                    }
                    upgradeToAndCall
                },
                {
                    fn proxiableUUID(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::proxiableUUID)
                    }
                    proxiableUUID
                },
                {
                    fn proposalVotes(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <proposalVotesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::proposalVotes)
                    }
                    proposalVotes
                },
                {
                    fn version(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::version)
                    }
                    version
                },
                {
                    fn castVote(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <castVoteCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::castVote)
                    }
                    castVote
                },
                {
                    fn castVoteWithReasonAndParamsBySig(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <castVoteWithReasonAndParamsBySigCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::castVoteWithReasonAndParamsBySig)
                    }
                    castVoteWithReasonAndParamsBySig
                },
                {
                    fn castVoteWithReasonAndParams(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <castVoteWithReasonAndParamsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::castVoteWithReasonAndParams)
                    }
                    castVoteWithReasonAndParams
                },
                {
                    fn quorumNumerator_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <quorumNumerator_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::quorumNumerator_0)
                    }
                    quorumNumerator_0
                },
                {
                    fn setVotingDelay(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <setVotingDelayCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::setVotingDelay)
                    }
                    setVotingDelay
                },
                {
                    fn castVoteWithReason(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <castVoteWithReasonCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::castVoteWithReason)
                    }
                    castVoteWithReason
                },
                {
                    fn propose(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <proposeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::propose)
                    }
                    propose
                },
                {
                    fn nonces(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <noncesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::nonces)
                    }
                    nonces
                },
                {
                    fn eip712Domain(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <eip712DomainCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::eip712Domain)
                    }
                    eip712Domain
                },
                {
                    fn castVoteBySig(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <castVoteBySigCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::castVoteBySig)
                    }
                    castVoteBySig
                },
                {
                    fn clock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <clockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::clock)
                    }
                    clock
                },
                {
                    fn quorumDenominator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <quorumDenominatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::quorumDenominator)
                    }
                    quorumDenominator
                },
                {
                    fn getVotesWithParams(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <getVotesWithParamsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::getVotesWithParams)
                    }
                    getVotesWithParams
                },
                {
                    fn quorumNumerator_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <quorumNumerator_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::quorumNumerator_1)
                    }
                    quorumNumerator_1
                },
                {
                    fn updateTimelock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <updateTimelockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::updateTimelock)
                    }
                    updateTimelock
                },
                {
                    fn proposalNeedsQueuing(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <proposalNeedsQueuingCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::proposalNeedsQueuing)
                    }
                    proposalNeedsQueuing
                },
                {
                    fn proposalEta(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <proposalEtaCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::proposalEta)
                    }
                    proposalEta
                },
                {
                    fn UPGRADE_INTERFACE_VERSION(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::UPGRADE_INTERFACE_VERSION)
                    }
                    UPGRADE_INTERFACE_VERSION
                },
                {
                    fn proposalThreshold(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <proposalThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::proposalThreshold)
                    }
                    proposalThreshold
                },
                {
                    fn onERC1155BatchReceived(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <onERC1155BatchReceivedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::onERC1155BatchReceived)
                    }
                    onERC1155BatchReceived
                },
                {
                    fn proposalDeadline(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <proposalDeadlineCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::proposalDeadline)
                    }
                    proposalDeadline
                },
                {
                    fn relay(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <relayCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::relay)
                    }
                    relay
                },
                {
                    fn hashProposal(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <hashProposalCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::hashProposal)
                    }
                    hashProposal
                },
                {
                    fn timelock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <timelockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::timelock)
                    }
                    timelock
                },
                {
                    fn COUNTING_MODE(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <COUNTING_MODECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::COUNTING_MODE)
                    }
                    COUNTING_MODE
                },
                {
                    fn BALLOT_TYPEHASH(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <BALLOT_TYPEHASHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::BALLOT_TYPEHASH)
                    }
                    BALLOT_TYPEHASH
                },
                {
                    fn setVotingPeriod(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <setVotingPeriodCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::setVotingPeriod)
                    }
                    setVotingPeriod
                },
                {
                    fn getVotes(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <getVotesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::getVotes)
                    }
                    getVotes
                },
                {
                    fn setProposalThreshold(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <setProposalThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::setProposalThreshold)
                    }
                    setProposalThreshold
                },
                {
                    fn onERC1155Received(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <onERC1155ReceivedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::onERC1155Received)
                    }
                    onERC1155Received
                },
                {
                    fn quorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <quorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::quorum)
                    }
                    quorum
                },
                {
                    fn token(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorCalls> {
                        <tokenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorCalls::token)
                    }
                    token
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::BALLOT_TYPEHASH(inner) => {
                    <BALLOT_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CLOCK_MODE(inner) => {
                    <CLOCK_MODECall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::COUNTING_MODE(inner) => {
                    <COUNTING_MODECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EXTENDED_BALLOT_TYPEHASH(inner) => {
                    <EXTENDED_BALLOT_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UPGRADE_INTERFACE_VERSION(inner) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::cancel(inner) => {
                    <cancelCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::castVote(inner) => {
                    <castVoteCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::castVoteBySig(inner) => {
                    <castVoteBySigCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::castVoteWithReason(inner) => {
                    <castVoteWithReasonCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::castVoteWithReasonAndParams(inner) => {
                    <castVoteWithReasonAndParamsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::castVoteWithReasonAndParamsBySig(inner) => {
                    <castVoteWithReasonAndParamsBySigCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::clock(inner) => {
                    <clockCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::eip712Domain(inner) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::execute(inner) => {
                    <executeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getVotes(inner) => {
                    <getVotesCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getVotesWithParams(inner) => {
                    <getVotesWithParamsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::hasVoted(inner) => {
                    <hasVotedCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::hashProposal(inner) => {
                    <hashProposalCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::name(inner) => {
                    <nameCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::nonces(inner) => {
                    <noncesCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::onERC1155BatchReceived(inner) => {
                    <onERC1155BatchReceivedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::onERC1155Received(inner) => {
                    <onERC1155ReceivedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::onERC721Received(inner) => {
                    <onERC721ReceivedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::proposalDeadline(inner) => {
                    <proposalDeadlineCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::proposalEta(inner) => {
                    <proposalEtaCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::proposalNeedsQueuing(inner) => {
                    <proposalNeedsQueuingCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::proposalProposer(inner) => {
                    <proposalProposerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::proposalSnapshot(inner) => {
                    <proposalSnapshotCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::proposalThreshold(inner) => {
                    <proposalThresholdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::proposalVotes(inner) => {
                    <proposalVotesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::propose(inner) => {
                    <proposeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::proxiableUUID(inner) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::queue(inner) => {
                    <queueCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::quorum(inner) => {
                    <quorumCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::quorumDenominator(inner) => {
                    <quorumDenominatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::quorumNumerator_0(inner) => {
                    <quorumNumerator_0Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::quorumNumerator_1(inner) => {
                    <quorumNumerator_1Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::relay(inner) => {
                    <relayCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::setProposalThreshold(inner) => {
                    <setProposalThresholdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setVotingDelay(inner) => {
                    <setVotingDelayCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setVotingPeriod(inner) => {
                    <setVotingPeriodCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::state(inner) => {
                    <stateCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::supportsInterface(inner) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::timelock(inner) => {
                    <timelockCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::token(inner) => {
                    <tokenCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::updateQuorumNumerator(inner) => {
                    <updateQuorumNumeratorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateTimelock(inner) => {
                    <updateTimelockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::upgradeToAndCall(inner) => {
                    <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::version(inner) => {
                    <versionCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::votingDelay(inner) => {
                    <votingDelayCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::votingPeriod(inner) => {
                    <votingPeriodCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::BALLOT_TYPEHASH(inner) => {
                    <BALLOT_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CLOCK_MODE(inner) => {
                    <CLOCK_MODECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::COUNTING_MODE(inner) => {
                    <COUNTING_MODECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EXTENDED_BALLOT_TYPEHASH(inner) => {
                    <EXTENDED_BALLOT_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UPGRADE_INTERFACE_VERSION(inner) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::cancel(inner) => {
                    <cancelCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::castVote(inner) => {
                    <castVoteCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::castVoteBySig(inner) => {
                    <castVoteBySigCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::castVoteWithReason(inner) => {
                    <castVoteWithReasonCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::castVoteWithReasonAndParams(inner) => {
                    <castVoteWithReasonAndParamsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::castVoteWithReasonAndParamsBySig(inner) => {
                    <castVoteWithReasonAndParamsBySigCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::clock(inner) => {
                    <clockCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::eip712Domain(inner) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::execute(inner) => {
                    <executeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getVotes(inner) => {
                    <getVotesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getVotesWithParams(inner) => {
                    <getVotesWithParamsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::hasVoted(inner) => {
                    <hasVotedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::hashProposal(inner) => {
                    <hashProposalCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::name(inner) => {
                    <nameCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::nonces(inner) => {
                    <noncesCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::onERC1155BatchReceived(inner) => {
                    <onERC1155BatchReceivedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::onERC1155Received(inner) => {
                    <onERC1155ReceivedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::onERC721Received(inner) => {
                    <onERC721ReceivedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::proposalDeadline(inner) => {
                    <proposalDeadlineCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::proposalEta(inner) => {
                    <proposalEtaCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::proposalNeedsQueuing(inner) => {
                    <proposalNeedsQueuingCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::proposalProposer(inner) => {
                    <proposalProposerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::proposalSnapshot(inner) => {
                    <proposalSnapshotCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::proposalThreshold(inner) => {
                    <proposalThresholdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::proposalVotes(inner) => {
                    <proposalVotesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::propose(inner) => {
                    <proposeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::proxiableUUID(inner) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::queue(inner) => {
                    <queueCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::quorum(inner) => {
                    <quorumCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::quorumDenominator(inner) => {
                    <quorumDenominatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::quorumNumerator_0(inner) => {
                    <quorumNumerator_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::quorumNumerator_1(inner) => {
                    <quorumNumerator_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::relay(inner) => {
                    <relayCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::setProposalThreshold(inner) => {
                    <setProposalThresholdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setVotingDelay(inner) => {
                    <setVotingDelayCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setVotingPeriod(inner) => {
                    <setVotingPeriodCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::state(inner) => {
                    <stateCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::supportsInterface(inner) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::timelock(inner) => {
                    <timelockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::token(inner) => {
                    <tokenCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::updateQuorumNumerator(inner) => {
                    <updateQuorumNumeratorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateTimelock(inner) => {
                    <updateTimelockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::upgradeToAndCall(inner) => {
                    <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::version(inner) => {
                    <versionCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::votingDelay(inner) => {
                    <votingDelayCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::votingPeriod(inner) => {
                    <votingPeriodCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`TangleGovernor`](self) custom errors.
    pub enum TangleGovernorErrors {
        #[allow(missing_docs)]
        AddressEmptyCode(AddressEmptyCode),
        #[allow(missing_docs)]
        CheckpointUnorderedInsertion(CheckpointUnorderedInsertion),
        #[allow(missing_docs)]
        ERC1967InvalidImplementation(ERC1967InvalidImplementation),
        #[allow(missing_docs)]
        ERC1967NonPayable(ERC1967NonPayable),
        #[allow(missing_docs)]
        FailedCall(FailedCall),
        #[allow(missing_docs)]
        GovernorAlreadyCastVote(GovernorAlreadyCastVote),
        #[allow(missing_docs)]
        GovernorAlreadyQueuedProposal(GovernorAlreadyQueuedProposal),
        #[allow(missing_docs)]
        GovernorDisabledDeposit(GovernorDisabledDeposit),
        #[allow(missing_docs)]
        GovernorInsufficientProposerVotes(GovernorInsufficientProposerVotes),
        #[allow(missing_docs)]
        GovernorInvalidProposalLength(GovernorInvalidProposalLength),
        #[allow(missing_docs)]
        GovernorInvalidQuorumFraction(GovernorInvalidQuorumFraction),
        #[allow(missing_docs)]
        GovernorInvalidSignature(GovernorInvalidSignature),
        #[allow(missing_docs)]
        GovernorInvalidVoteParams(GovernorInvalidVoteParams),
        #[allow(missing_docs)]
        GovernorInvalidVoteType(GovernorInvalidVoteType),
        #[allow(missing_docs)]
        GovernorInvalidVotingPeriod(GovernorInvalidVotingPeriod),
        #[allow(missing_docs)]
        GovernorNonexistentProposal(GovernorNonexistentProposal),
        #[allow(missing_docs)]
        GovernorNotQueuedProposal(GovernorNotQueuedProposal),
        #[allow(missing_docs)]
        GovernorOnlyExecutor(GovernorOnlyExecutor),
        #[allow(missing_docs)]
        GovernorOnlyProposer(GovernorOnlyProposer),
        #[allow(missing_docs)]
        GovernorQueueNotImplemented(GovernorQueueNotImplemented),
        #[allow(missing_docs)]
        GovernorRestrictedProposer(GovernorRestrictedProposer),
        #[allow(missing_docs)]
        GovernorUnexpectedProposalState(GovernorUnexpectedProposalState),
        #[allow(missing_docs)]
        InvalidAccountNonce(InvalidAccountNonce),
        #[allow(missing_docs)]
        InvalidInitialization(InvalidInitialization),
        #[allow(missing_docs)]
        NotInitializing(NotInitializing),
        #[allow(missing_docs)]
        SafeCastOverflowedUintDowncast(SafeCastOverflowedUintDowncast),
        #[allow(missing_docs)]
        UUPSUnauthorizedCallContext(UUPSUnauthorizedCallContext),
        #[allow(missing_docs)]
        UUPSUnsupportedProxiableUUID(UUPSUnsupportedProxiableUUID),
    }
    #[automatically_derived]
    impl TangleGovernorErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [6u8, 179u8, 55u8, 194u8],
            [35u8, 61u8, 152u8, 227u8],
            [36u8, 62u8, 84u8, 69u8],
            [37u8, 32u8, 96u8, 29u8],
            [49u8, 183u8, 94u8, 77u8],
            [68u8, 123u8, 5u8, 208u8],
            [71u8, 9u8, 110u8, 71u8],
            [76u8, 156u8, 140u8, 227u8],
            [106u8, 208u8, 96u8, 117u8],
            [109u8, 252u8, 198u8, 80u8],
            [113u8, 198u8, 175u8, 73u8],
            [117u8, 45u8, 136u8, 192u8],
            [134u8, 125u8, 183u8, 113u8],
            [144u8, 136u8, 74u8, 70u8],
            [148u8, 171u8, 108u8, 7u8],
            [153u8, 150u8, 179u8, 21u8],
            [170u8, 29u8, 73u8, 164u8],
            [179u8, 152u8, 151u8, 159u8],
            [194u8, 66u8, 238u8, 22u8],
            [213u8, 221u8, 184u8, 37u8],
            [214u8, 189u8, 162u8, 117u8],
            [215u8, 230u8, 188u8, 248u8],
            [217u8, 179u8, 149u8, 87u8],
            [224u8, 124u8, 141u8, 186u8],
            [233u8, 10u8, 101u8, 30u8],
            [241u8, 207u8, 191u8, 5u8],
            [242u8, 14u8, 125u8, 55u8],
            [249u8, 46u8, 232u8, 169u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for TangleGovernorErrors {
        const NAME: &'static str = "TangleGovernorErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 28usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AddressEmptyCode(_) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CheckpointUnorderedInsertion(_) => {
                    <CheckpointUnorderedInsertion as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC1967InvalidImplementation(_) => {
                    <ERC1967InvalidImplementation as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC1967NonPayable(_) => {
                    <ERC1967NonPayable as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FailedCall(_) => {
                    <FailedCall as alloy_sol_types::SolError>::SELECTOR
                }
                Self::GovernorAlreadyCastVote(_) => {
                    <GovernorAlreadyCastVote as alloy_sol_types::SolError>::SELECTOR
                }
                Self::GovernorAlreadyQueuedProposal(_) => {
                    <GovernorAlreadyQueuedProposal as alloy_sol_types::SolError>::SELECTOR
                }
                Self::GovernorDisabledDeposit(_) => {
                    <GovernorDisabledDeposit as alloy_sol_types::SolError>::SELECTOR
                }
                Self::GovernorInsufficientProposerVotes(_) => {
                    <GovernorInsufficientProposerVotes as alloy_sol_types::SolError>::SELECTOR
                }
                Self::GovernorInvalidProposalLength(_) => {
                    <GovernorInvalidProposalLength as alloy_sol_types::SolError>::SELECTOR
                }
                Self::GovernorInvalidQuorumFraction(_) => {
                    <GovernorInvalidQuorumFraction as alloy_sol_types::SolError>::SELECTOR
                }
                Self::GovernorInvalidSignature(_) => {
                    <GovernorInvalidSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::GovernorInvalidVoteParams(_) => {
                    <GovernorInvalidVoteParams as alloy_sol_types::SolError>::SELECTOR
                }
                Self::GovernorInvalidVoteType(_) => {
                    <GovernorInvalidVoteType as alloy_sol_types::SolError>::SELECTOR
                }
                Self::GovernorInvalidVotingPeriod(_) => {
                    <GovernorInvalidVotingPeriod as alloy_sol_types::SolError>::SELECTOR
                }
                Self::GovernorNonexistentProposal(_) => {
                    <GovernorNonexistentProposal as alloy_sol_types::SolError>::SELECTOR
                }
                Self::GovernorNotQueuedProposal(_) => {
                    <GovernorNotQueuedProposal as alloy_sol_types::SolError>::SELECTOR
                }
                Self::GovernorOnlyExecutor(_) => {
                    <GovernorOnlyExecutor as alloy_sol_types::SolError>::SELECTOR
                }
                Self::GovernorOnlyProposer(_) => {
                    <GovernorOnlyProposer as alloy_sol_types::SolError>::SELECTOR
                }
                Self::GovernorQueueNotImplemented(_) => {
                    <GovernorQueueNotImplemented as alloy_sol_types::SolError>::SELECTOR
                }
                Self::GovernorRestrictedProposer(_) => {
                    <GovernorRestrictedProposer as alloy_sol_types::SolError>::SELECTOR
                }
                Self::GovernorUnexpectedProposalState(_) => {
                    <GovernorUnexpectedProposalState as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidAccountNonce(_) => {
                    <InvalidAccountNonce as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidInitialization(_) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotInitializing(_) => {
                    <NotInitializing as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SafeCastOverflowedUintDowncast(_) => {
                    <SafeCastOverflowedUintDowncast as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UUPSUnauthorizedCallContext(_) => {
                    <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UUPSUnsupportedProxiableUUID(_) => {
                    <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<TangleGovernorErrors>] = &[
                {
                    fn GovernorInvalidVoteType(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorErrors> {
                        <GovernorInvalidVoteType as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorErrors::GovernorInvalidVoteType)
                    }
                    GovernorInvalidVoteType
                },
                {
                    fn GovernorOnlyProposer(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorErrors> {
                        <GovernorOnlyProposer as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorErrors::GovernorOnlyProposer)
                    }
                    GovernorOnlyProposer
                },
                {
                    fn GovernorInvalidQuorumFraction(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorErrors> {
                        <GovernorInvalidQuorumFraction as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorErrors::GovernorInvalidQuorumFraction)
                    }
                    GovernorInvalidQuorumFraction
                },
                {
                    fn CheckpointUnorderedInsertion(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorErrors> {
                        <CheckpointUnorderedInsertion as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorErrors::CheckpointUnorderedInsertion)
                    }
                    CheckpointUnorderedInsertion
                },
                {
                    fn GovernorUnexpectedProposalState(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorErrors> {
                        <GovernorUnexpectedProposalState as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorErrors::GovernorUnexpectedProposalState)
                    }
                    GovernorUnexpectedProposalState
                },
                {
                    fn GovernorInvalidProposalLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorErrors> {
                        <GovernorInvalidProposalLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorErrors::GovernorInvalidProposalLength)
                    }
                    GovernorInvalidProposalLength
                },
                {
                    fn GovernorOnlyExecutor(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorErrors> {
                        <GovernorOnlyExecutor as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorErrors::GovernorOnlyExecutor)
                    }
                    GovernorOnlyExecutor
                },
                {
                    fn ERC1967InvalidImplementation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorErrors> {
                        <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorErrors::ERC1967InvalidImplementation)
                    }
                    ERC1967InvalidImplementation
                },
                {
                    fn GovernorNonexistentProposal(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorErrors> {
                        <GovernorNonexistentProposal as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorErrors::GovernorNonexistentProposal)
                    }
                    GovernorNonexistentProposal
                },
                {
                    fn SafeCastOverflowedUintDowncast(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorErrors> {
                        <SafeCastOverflowedUintDowncast as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorErrors::SafeCastOverflowedUintDowncast)
                    }
                    SafeCastOverflowedUintDowncast
                },
                {
                    fn GovernorAlreadyCastVote(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorErrors> {
                        <GovernorAlreadyCastVote as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorErrors::GovernorAlreadyCastVote)
                    }
                    GovernorAlreadyCastVote
                },
                {
                    fn InvalidAccountNonce(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorErrors> {
                        <InvalidAccountNonce as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorErrors::InvalidAccountNonce)
                    }
                    InvalidAccountNonce
                },
                {
                    fn GovernorInvalidVoteParams(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorErrors> {
                        <GovernorInvalidVoteParams as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorErrors::GovernorInvalidVoteParams)
                    }
                    GovernorInvalidVoteParams
                },
                {
                    fn GovernorQueueNotImplemented(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorErrors> {
                        <GovernorQueueNotImplemented as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorErrors::GovernorQueueNotImplemented)
                    }
                    GovernorQueueNotImplemented
                },
                {
                    fn GovernorInvalidSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorErrors> {
                        <GovernorInvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorErrors::GovernorInvalidSignature)
                    }
                    GovernorInvalidSignature
                },
                {
                    fn AddressEmptyCode(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorErrors> {
                        <AddressEmptyCode as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorErrors::AddressEmptyCode)
                    }
                    AddressEmptyCode
                },
                {
                    fn UUPSUnsupportedProxiableUUID(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorErrors> {
                        <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorErrors::UUPSUnsupportedProxiableUUID)
                    }
                    UUPSUnsupportedProxiableUUID
                },
                {
                    fn ERC1967NonPayable(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorErrors> {
                        <ERC1967NonPayable as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorErrors::ERC1967NonPayable)
                    }
                    ERC1967NonPayable
                },
                {
                    fn GovernorInsufficientProposerVotes(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorErrors> {
                        <GovernorInsufficientProposerVotes as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorErrors::GovernorInsufficientProposerVotes)
                    }
                    GovernorInsufficientProposerVotes
                },
                {
                    fn GovernorNotQueuedProposal(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorErrors> {
                        <GovernorNotQueuedProposal as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorErrors::GovernorNotQueuedProposal)
                    }
                    GovernorNotQueuedProposal
                },
                {
                    fn FailedCall(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorErrors> {
                        <FailedCall as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorErrors::FailedCall)
                    }
                    FailedCall
                },
                {
                    fn NotInitializing(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorErrors> {
                        <NotInitializing as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorErrors::NotInitializing)
                    }
                    NotInitializing
                },
                {
                    fn GovernorRestrictedProposer(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorErrors> {
                        <GovernorRestrictedProposer as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorErrors::GovernorRestrictedProposer)
                    }
                    GovernorRestrictedProposer
                },
                {
                    fn UUPSUnauthorizedCallContext(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorErrors> {
                        <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorErrors::UUPSUnauthorizedCallContext)
                    }
                    UUPSUnauthorizedCallContext
                },
                {
                    fn GovernorDisabledDeposit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorErrors> {
                        <GovernorDisabledDeposit as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorErrors::GovernorDisabledDeposit)
                    }
                    GovernorDisabledDeposit
                },
                {
                    fn GovernorInvalidVotingPeriod(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorErrors> {
                        <GovernorInvalidVotingPeriod as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorErrors::GovernorInvalidVotingPeriod)
                    }
                    GovernorInvalidVotingPeriod
                },
                {
                    fn GovernorAlreadyQueuedProposal(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorErrors> {
                        <GovernorAlreadyQueuedProposal as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorErrors::GovernorAlreadyQueuedProposal)
                    }
                    GovernorAlreadyQueuedProposal
                },
                {
                    fn InvalidInitialization(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleGovernorErrors> {
                        <InvalidInitialization as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleGovernorErrors::InvalidInitialization)
                    }
                    InvalidInitialization
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::AddressEmptyCode(inner) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CheckpointUnorderedInsertion(inner) => {
                    <CheckpointUnorderedInsertion as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC1967InvalidImplementation(inner) => {
                    <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC1967NonPayable(inner) => {
                    <ERC1967NonPayable as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FailedCall(inner) => {
                    <FailedCall as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::GovernorAlreadyCastVote(inner) => {
                    <GovernorAlreadyCastVote as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::GovernorAlreadyQueuedProposal(inner) => {
                    <GovernorAlreadyQueuedProposal as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::GovernorDisabledDeposit(inner) => {
                    <GovernorDisabledDeposit as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::GovernorInsufficientProposerVotes(inner) => {
                    <GovernorInsufficientProposerVotes as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::GovernorInvalidProposalLength(inner) => {
                    <GovernorInvalidProposalLength as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::GovernorInvalidQuorumFraction(inner) => {
                    <GovernorInvalidQuorumFraction as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::GovernorInvalidSignature(inner) => {
                    <GovernorInvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::GovernorInvalidVoteParams(inner) => {
                    <GovernorInvalidVoteParams as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::GovernorInvalidVoteType(inner) => {
                    <GovernorInvalidVoteType as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::GovernorInvalidVotingPeriod(inner) => {
                    <GovernorInvalidVotingPeriod as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::GovernorNonexistentProposal(inner) => {
                    <GovernorNonexistentProposal as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::GovernorNotQueuedProposal(inner) => {
                    <GovernorNotQueuedProposal as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::GovernorOnlyExecutor(inner) => {
                    <GovernorOnlyExecutor as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::GovernorOnlyProposer(inner) => {
                    <GovernorOnlyProposer as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::GovernorQueueNotImplemented(inner) => {
                    <GovernorQueueNotImplemented as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::GovernorRestrictedProposer(inner) => {
                    <GovernorRestrictedProposer as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::GovernorUnexpectedProposalState(inner) => {
                    <GovernorUnexpectedProposalState as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidAccountNonce(inner) => {
                    <InvalidAccountNonce as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidInitialization(inner) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotInitializing(inner) => {
                    <NotInitializing as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SafeCastOverflowedUintDowncast(inner) => {
                    <SafeCastOverflowedUintDowncast as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UUPSUnauthorizedCallContext(inner) => {
                    <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UUPSUnsupportedProxiableUUID(inner) => {
                    <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::AddressEmptyCode(inner) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CheckpointUnorderedInsertion(inner) => {
                    <CheckpointUnorderedInsertion as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC1967InvalidImplementation(inner) => {
                    <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC1967NonPayable(inner) => {
                    <ERC1967NonPayable as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FailedCall(inner) => {
                    <FailedCall as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::GovernorAlreadyCastVote(inner) => {
                    <GovernorAlreadyCastVote as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::GovernorAlreadyQueuedProposal(inner) => {
                    <GovernorAlreadyQueuedProposal as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::GovernorDisabledDeposit(inner) => {
                    <GovernorDisabledDeposit as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::GovernorInsufficientProposerVotes(inner) => {
                    <GovernorInsufficientProposerVotes as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::GovernorInvalidProposalLength(inner) => {
                    <GovernorInvalidProposalLength as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::GovernorInvalidQuorumFraction(inner) => {
                    <GovernorInvalidQuorumFraction as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::GovernorInvalidSignature(inner) => {
                    <GovernorInvalidSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::GovernorInvalidVoteParams(inner) => {
                    <GovernorInvalidVoteParams as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::GovernorInvalidVoteType(inner) => {
                    <GovernorInvalidVoteType as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::GovernorInvalidVotingPeriod(inner) => {
                    <GovernorInvalidVotingPeriod as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::GovernorNonexistentProposal(inner) => {
                    <GovernorNonexistentProposal as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::GovernorNotQueuedProposal(inner) => {
                    <GovernorNotQueuedProposal as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::GovernorOnlyExecutor(inner) => {
                    <GovernorOnlyExecutor as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::GovernorOnlyProposer(inner) => {
                    <GovernorOnlyProposer as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::GovernorQueueNotImplemented(inner) => {
                    <GovernorQueueNotImplemented as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::GovernorRestrictedProposer(inner) => {
                    <GovernorRestrictedProposer as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::GovernorUnexpectedProposalState(inner) => {
                    <GovernorUnexpectedProposalState as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidAccountNonce(inner) => {
                    <InvalidAccountNonce as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidInitialization(inner) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotInitializing(inner) => {
                    <NotInitializing as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SafeCastOverflowedUintDowncast(inner) => {
                    <SafeCastOverflowedUintDowncast as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UUPSUnauthorizedCallContext(inner) => {
                    <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UUPSUnsupportedProxiableUUID(inner) => {
                    <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`TangleGovernor`](self) events.
    pub enum TangleGovernorEvents {
        #[allow(missing_docs)]
        EIP712DomainChanged(EIP712DomainChanged),
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        ProposalCanceled(ProposalCanceled),
        #[allow(missing_docs)]
        ProposalCreated(ProposalCreated),
        #[allow(missing_docs)]
        ProposalExecuted(ProposalExecuted),
        #[allow(missing_docs)]
        ProposalQueued(ProposalQueued),
        #[allow(missing_docs)]
        ProposalThresholdSet(ProposalThresholdSet),
        #[allow(missing_docs)]
        QuorumNumeratorUpdated(QuorumNumeratorUpdated),
        #[allow(missing_docs)]
        TimelockChange(TimelockChange),
        #[allow(missing_docs)]
        Upgraded(Upgraded),
        #[allow(missing_docs)]
        VoteCast(VoteCast),
        #[allow(missing_docs)]
        VoteCastWithParams(VoteCastWithParams),
        #[allow(missing_docs)]
        VotingDelaySet(VotingDelaySet),
        #[allow(missing_docs)]
        VotingPeriodSet(VotingPeriodSet),
    }
    #[automatically_derived]
    impl TangleGovernorEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                5u8,
                83u8,
                71u8,
                107u8,
                240u8,
                46u8,
                242u8,
                114u8,
                110u8,
                140u8,
                229u8,
                206u8,
                215u8,
                141u8,
                99u8,
                226u8,
                110u8,
                96u8,
                46u8,
                74u8,
                34u8,
                87u8,
                177u8,
                245u8,
                89u8,
                65u8,
                142u8,
                36u8,
                180u8,
                99u8,
                57u8,
                151u8,
            ],
            [
                8u8,
                247u8,
                78u8,
                164u8,
                110u8,
                247u8,
                137u8,
                79u8,
                101u8,
                234u8,
                191u8,
                181u8,
                230u8,
                230u8,
                149u8,
                222u8,
                119u8,
                58u8,
                0u8,
                11u8,
                71u8,
                197u8,
                41u8,
                171u8,
                85u8,
                145u8,
                120u8,
                6u8,
                155u8,
                34u8,
                100u8,
                1u8,
            ],
            [
                10u8,
                99u8,
                135u8,
                201u8,
                234u8,
                54u8,
                40u8,
                184u8,
                138u8,
                99u8,
                59u8,
                180u8,
                243u8,
                177u8,
                81u8,
                119u8,
                15u8,
                112u8,
                8u8,
                81u8,
                23u8,
                161u8,
                95u8,
                155u8,
                243u8,
                120u8,
                124u8,
                218u8,
                83u8,
                241u8,
                61u8,
                49u8,
            ],
            [
                113u8,
                42u8,
                225u8,
                56u8,
                63u8,
                121u8,
                172u8,
                133u8,
                63u8,
                141u8,
                136u8,
                33u8,
                83u8,
                119u8,
                142u8,
                2u8,
                96u8,
                239u8,
                143u8,
                3u8,
                181u8,
                4u8,
                226u8,
                134u8,
                110u8,
                5u8,
                147u8,
                224u8,
                77u8,
                43u8,
                41u8,
                31u8,
            ],
            [
                120u8,
                156u8,
                245u8,
                91u8,
                233u8,
                128u8,
                115u8,
                157u8,
                173u8,
                29u8,
                6u8,
                153u8,
                185u8,
                59u8,
                88u8,
                232u8,
                6u8,
                181u8,
                28u8,
                157u8,
                150u8,
                97u8,
                155u8,
                250u8,
                143u8,
                224u8,
                162u8,
                138u8,
                186u8,
                167u8,
                179u8,
                12u8,
            ],
            [
                125u8,
                132u8,
                166u8,
                38u8,
                58u8,
                224u8,
                217u8,
                141u8,
                51u8,
                41u8,
                189u8,
                123u8,
                70u8,
                187u8,
                78u8,
                141u8,
                111u8,
                152u8,
                205u8,
                53u8,
                167u8,
                173u8,
                180u8,
                92u8,
                39u8,
                76u8,
                139u8,
                127u8,
                213u8,
                235u8,
                213u8,
                224u8,
            ],
            [
                126u8,
                63u8,
                127u8,
                7u8,
                8u8,
                168u8,
                77u8,
                233u8,
                32u8,
                48u8,
                54u8,
                171u8,
                170u8,
                69u8,
                13u8,
                204u8,
                200u8,
                90u8,
                213u8,
                255u8,
                82u8,
                247u8,
                140u8,
                23u8,
                15u8,
                62u8,
                219u8,
                85u8,
                207u8,
                94u8,
                136u8,
                40u8,
            ],
            [
                154u8,
                46u8,
                66u8,
                253u8,
                103u8,
                34u8,
                129u8,
                61u8,
                105u8,
                17u8,
                62u8,
                125u8,
                0u8,
                121u8,
                211u8,
                217u8,
                64u8,
                23u8,
                20u8,
                40u8,
                223u8,
                115u8,
                115u8,
                223u8,
                156u8,
                127u8,
                118u8,
                23u8,
                207u8,
                218u8,
                40u8,
                146u8,
            ],
            [
                184u8,
                225u8,
                56u8,
                136u8,
                125u8,
                10u8,
                161u8,
                59u8,
                171u8,
                68u8,
                126u8,
                130u8,
                222u8,
                157u8,
                92u8,
                23u8,
                119u8,
                4u8,
                30u8,
                205u8,
                33u8,
                202u8,
                54u8,
                186u8,
                130u8,
                79u8,
                241u8,
                230u8,
                192u8,
                125u8,
                221u8,
                164u8,
            ],
            [
                188u8,
                124u8,
                215u8,
                90u8,
                32u8,
                238u8,
                39u8,
                253u8,
                154u8,
                222u8,
                186u8,
                179u8,
                32u8,
                65u8,
                247u8,
                85u8,
                33u8,
                77u8,
                188u8,
                107u8,
                255u8,
                169u8,
                12u8,
                192u8,
                34u8,
                91u8,
                57u8,
                218u8,
                46u8,
                92u8,
                45u8,
                59u8,
            ],
            [
                197u8,
                101u8,
                176u8,
                69u8,
                64u8,
                61u8,
                192u8,
                60u8,
                46u8,
                234u8,
                130u8,
                184u8,
                26u8,
                4u8,
                101u8,
                237u8,
                173u8,
                158u8,
                46u8,
                127u8,
                196u8,
                217u8,
                126u8,
                17u8,
                66u8,
                28u8,
                32u8,
                157u8,
                169u8,
                61u8,
                122u8,
                147u8,
            ],
            [
                199u8,
                245u8,
                5u8,
                178u8,
                243u8,
                113u8,
                174u8,
                33u8,
                117u8,
                238u8,
                73u8,
                19u8,
                244u8,
                73u8,
                158u8,
                31u8,
                38u8,
                51u8,
                167u8,
                181u8,
                147u8,
                99u8,
                33u8,
                238u8,
                209u8,
                205u8,
                174u8,
                182u8,
                17u8,
                81u8,
                129u8,
                210u8,
            ],
            [
                204u8,
                180u8,
                93u8,
                168u8,
                213u8,
                113u8,
                126u8,
                108u8,
                69u8,
                68u8,
                105u8,
                66u8,
                151u8,
                196u8,
                186u8,
                92u8,
                241u8,
                81u8,
                212u8,
                85u8,
                201u8,
                187u8,
                14u8,
                212u8,
                252u8,
                122u8,
                56u8,
                65u8,
                27u8,
                192u8,
                84u8,
                97u8,
            ],
            [
                226u8,
                186u8,
                191u8,
                186u8,
                197u8,
                136u8,
                154u8,
                112u8,
                155u8,
                99u8,
                187u8,
                127u8,
                89u8,
                139u8,
                50u8,
                78u8,
                8u8,
                188u8,
                90u8,
                79u8,
                185u8,
                236u8,
                100u8,
                127u8,
                179u8,
                203u8,
                201u8,
                236u8,
                7u8,
                235u8,
                135u8,
                18u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for TangleGovernorEvents {
        const NAME: &'static str = "TangleGovernorEvents";
        const COUNT: usize = 14usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <EIP712DomainChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <EIP712DomainChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::EIP712DomainChanged)
                }
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Initialized)
                }
                Some(<ProposalCanceled as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ProposalCanceled as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ProposalCanceled)
                }
                Some(<ProposalCreated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ProposalCreated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ProposalCreated)
                }
                Some(<ProposalExecuted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ProposalExecuted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ProposalExecuted)
                }
                Some(<ProposalQueued as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ProposalQueued as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ProposalQueued)
                }
                Some(
                    <ProposalThresholdSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ProposalThresholdSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ProposalThresholdSet)
                }
                Some(
                    <QuorumNumeratorUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <QuorumNumeratorUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::QuorumNumeratorUpdated)
                }
                Some(<TimelockChange as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <TimelockChange as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::TimelockChange)
                }
                Some(<Upgraded as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Upgraded as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Upgraded)
                }
                Some(<VoteCast as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <VoteCast as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::VoteCast)
                }
                Some(
                    <VoteCastWithParams as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <VoteCastWithParams as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::VoteCastWithParams)
                }
                Some(<VotingDelaySet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <VotingDelaySet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::VotingDelaySet)
                }
                Some(<VotingPeriodSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <VotingPeriodSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::VotingPeriodSet)
                }
                _ => {
                    alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                        name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                        log: alloy_sol_types::private::Box::new(
                            alloy_sol_types::private::LogData::new_unchecked(
                                topics.to_vec(),
                                data.to_vec().into(),
                            ),
                        ),
                    })
                }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for TangleGovernorEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::EIP712DomainChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ProposalCanceled(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ProposalCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ProposalExecuted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ProposalQueued(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ProposalThresholdSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::QuorumNumeratorUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TimelockChange(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::VoteCast(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::VoteCastWithParams(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::VotingDelaySet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::VotingPeriodSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::EIP712DomainChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ProposalCanceled(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ProposalCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ProposalExecuted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ProposalQueued(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ProposalThresholdSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::QuorumNumeratorUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TimelockChange(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::VoteCast(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::VoteCastWithParams(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::VotingDelaySet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::VotingPeriodSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`TangleGovernor`](self) contract instance.

See the [wrapper's documentation](`TangleGovernorInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> TangleGovernorInstance<T, P, N> {
        TangleGovernorInstance::<T, P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<TangleGovernorInstance<T, P, N>>,
    > {
        TangleGovernorInstance::<T, P, N>::deploy(provider)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
        TangleGovernorInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`TangleGovernor`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`TangleGovernor`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct TangleGovernorInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for TangleGovernorInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("TangleGovernorInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > TangleGovernorInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`TangleGovernor`](self) contract instance.

See the [wrapper's documentation](`TangleGovernorInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
        ) -> alloy_contract::Result<TangleGovernorInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                ::core::clone::Clone::clone(&BYTECODE),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> TangleGovernorInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> TangleGovernorInstance<T, P, N> {
            TangleGovernorInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > TangleGovernorInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`BALLOT_TYPEHASH`] function.
        pub fn BALLOT_TYPEHASH(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, BALLOT_TYPEHASHCall, N> {
            self.call_builder(&BALLOT_TYPEHASHCall {})
        }
        ///Creates a new call builder for the [`CLOCK_MODE`] function.
        pub fn CLOCK_MODE(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, CLOCK_MODECall, N> {
            self.call_builder(&CLOCK_MODECall {})
        }
        ///Creates a new call builder for the [`COUNTING_MODE`] function.
        pub fn COUNTING_MODE(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, COUNTING_MODECall, N> {
            self.call_builder(&COUNTING_MODECall {})
        }
        ///Creates a new call builder for the [`EXTENDED_BALLOT_TYPEHASH`] function.
        pub fn EXTENDED_BALLOT_TYPEHASH(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, EXTENDED_BALLOT_TYPEHASHCall, N> {
            self.call_builder(&EXTENDED_BALLOT_TYPEHASHCall {})
        }
        ///Creates a new call builder for the [`UPGRADE_INTERFACE_VERSION`] function.
        pub fn UPGRADE_INTERFACE_VERSION(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, UPGRADE_INTERFACE_VERSIONCall, N> {
            self.call_builder(&UPGRADE_INTERFACE_VERSIONCall {})
        }
        ///Creates a new call builder for the [`cancel`] function.
        pub fn cancel(
            &self,
            targets: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            values: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
            calldatas: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
            descriptionHash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, cancelCall, N> {
            self.call_builder(
                &cancelCall {
                    targets,
                    values,
                    calldatas,
                    descriptionHash,
                },
            )
        }
        ///Creates a new call builder for the [`castVote`] function.
        pub fn castVote(
            &self,
            proposalId: alloy::sol_types::private::primitives::aliases::U256,
            support: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, castVoteCall, N> {
            self.call_builder(
                &castVoteCall {
                    proposalId,
                    support,
                },
            )
        }
        ///Creates a new call builder for the [`castVoteBySig`] function.
        pub fn castVoteBySig(
            &self,
            proposalId: alloy::sol_types::private::primitives::aliases::U256,
            support: u8,
            voter: alloy::sol_types::private::Address,
            signature: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, castVoteBySigCall, N> {
            self.call_builder(
                &castVoteBySigCall {
                    proposalId,
                    support,
                    voter,
                    signature,
                },
            )
        }
        ///Creates a new call builder for the [`castVoteWithReason`] function.
        pub fn castVoteWithReason(
            &self,
            proposalId: alloy::sol_types::private::primitives::aliases::U256,
            support: u8,
            reason: alloy::sol_types::private::String,
        ) -> alloy_contract::SolCallBuilder<T, &P, castVoteWithReasonCall, N> {
            self.call_builder(
                &castVoteWithReasonCall {
                    proposalId,
                    support,
                    reason,
                },
            )
        }
        ///Creates a new call builder for the [`castVoteWithReasonAndParams`] function.
        pub fn castVoteWithReasonAndParams(
            &self,
            proposalId: alloy::sol_types::private::primitives::aliases::U256,
            support: u8,
            reason: alloy::sol_types::private::String,
            params: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, castVoteWithReasonAndParamsCall, N> {
            self.call_builder(
                &castVoteWithReasonAndParamsCall {
                    proposalId,
                    support,
                    reason,
                    params,
                },
            )
        }
        ///Creates a new call builder for the [`castVoteWithReasonAndParamsBySig`] function.
        pub fn castVoteWithReasonAndParamsBySig(
            &self,
            proposalId: alloy::sol_types::private::primitives::aliases::U256,
            support: u8,
            voter: alloy::sol_types::private::Address,
            reason: alloy::sol_types::private::String,
            params: alloy::sol_types::private::Bytes,
            signature: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            castVoteWithReasonAndParamsBySigCall,
            N,
        > {
            self.call_builder(
                &castVoteWithReasonAndParamsBySigCall {
                    proposalId,
                    support,
                    voter,
                    reason,
                    params,
                    signature,
                },
            )
        }
        ///Creates a new call builder for the [`clock`] function.
        pub fn clock(&self) -> alloy_contract::SolCallBuilder<T, &P, clockCall, N> {
            self.call_builder(&clockCall {})
        }
        ///Creates a new call builder for the [`eip712Domain`] function.
        pub fn eip712Domain(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, eip712DomainCall, N> {
            self.call_builder(&eip712DomainCall {})
        }
        ///Creates a new call builder for the [`execute`] function.
        pub fn execute(
            &self,
            targets: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            values: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
            calldatas: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
            descriptionHash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, executeCall, N> {
            self.call_builder(
                &executeCall {
                    targets,
                    values,
                    calldatas,
                    descriptionHash,
                },
            )
        }
        ///Creates a new call builder for the [`getVotes`] function.
        pub fn getVotes(
            &self,
            account: alloy::sol_types::private::Address,
            timepoint: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getVotesCall, N> {
            self.call_builder(&getVotesCall { account, timepoint })
        }
        ///Creates a new call builder for the [`getVotesWithParams`] function.
        pub fn getVotesWithParams(
            &self,
            account: alloy::sol_types::private::Address,
            timepoint: alloy::sol_types::private::primitives::aliases::U256,
            params: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, getVotesWithParamsCall, N> {
            self.call_builder(
                &getVotesWithParamsCall {
                    account,
                    timepoint,
                    params,
                },
            )
        }
        ///Creates a new call builder for the [`hasVoted`] function.
        pub fn hasVoted(
            &self,
            proposalId: alloy::sol_types::private::primitives::aliases::U256,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, hasVotedCall, N> {
            self.call_builder(
                &hasVotedCall {
                    proposalId,
                    account,
                },
            )
        }
        ///Creates a new call builder for the [`hashProposal`] function.
        pub fn hashProposal(
            &self,
            targets: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            values: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
            calldatas: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
            descriptionHash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, hashProposalCall, N> {
            self.call_builder(
                &hashProposalCall {
                    targets,
                    values,
                    calldatas,
                    descriptionHash,
                },
            )
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            token: alloy::sol_types::private::Address,
            timelock: alloy::sol_types::private::Address,
            initialVotingDelay: alloy::sol_types::private::primitives::aliases::U48,
            initialVotingPeriod: u32,
            initialProposalThreshold: alloy::sol_types::private::primitives::aliases::U256,
            quorumPercent: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(
                &initializeCall {
                    token,
                    timelock,
                    initialVotingDelay,
                    initialVotingPeriod,
                    initialProposalThreshold,
                    quorumPercent,
                },
            )
        }
        ///Creates a new call builder for the [`name`] function.
        pub fn name(&self) -> alloy_contract::SolCallBuilder<T, &P, nameCall, N> {
            self.call_builder(&nameCall {})
        }
        ///Creates a new call builder for the [`nonces`] function.
        pub fn nonces(
            &self,
            owner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, noncesCall, N> {
            self.call_builder(&noncesCall { owner })
        }
        ///Creates a new call builder for the [`onERC1155BatchReceived`] function.
        pub fn onERC1155BatchReceived(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::Address,
            _2: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
            _3: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
            _4: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, onERC1155BatchReceivedCall, N> {
            self.call_builder(
                &onERC1155BatchReceivedCall {
                    _0,
                    _1,
                    _2,
                    _3,
                    _4,
                },
            )
        }
        ///Creates a new call builder for the [`onERC1155Received`] function.
        pub fn onERC1155Received(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::Address,
            _2: alloy::sol_types::private::primitives::aliases::U256,
            _3: alloy::sol_types::private::primitives::aliases::U256,
            _4: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, onERC1155ReceivedCall, N> {
            self.call_builder(
                &onERC1155ReceivedCall {
                    _0,
                    _1,
                    _2,
                    _3,
                    _4,
                },
            )
        }
        ///Creates a new call builder for the [`onERC721Received`] function.
        pub fn onERC721Received(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::Address,
            _2: alloy::sol_types::private::primitives::aliases::U256,
            _3: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, onERC721ReceivedCall, N> {
            self.call_builder(
                &onERC721ReceivedCall {
                    _0,
                    _1,
                    _2,
                    _3,
                },
            )
        }
        ///Creates a new call builder for the [`proposalDeadline`] function.
        pub fn proposalDeadline(
            &self,
            proposalId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, proposalDeadlineCall, N> {
            self.call_builder(&proposalDeadlineCall { proposalId })
        }
        ///Creates a new call builder for the [`proposalEta`] function.
        pub fn proposalEta(
            &self,
            proposalId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, proposalEtaCall, N> {
            self.call_builder(&proposalEtaCall { proposalId })
        }
        ///Creates a new call builder for the [`proposalNeedsQueuing`] function.
        pub fn proposalNeedsQueuing(
            &self,
            proposalId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, proposalNeedsQueuingCall, N> {
            self.call_builder(
                &proposalNeedsQueuingCall {
                    proposalId,
                },
            )
        }
        ///Creates a new call builder for the [`proposalProposer`] function.
        pub fn proposalProposer(
            &self,
            proposalId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, proposalProposerCall, N> {
            self.call_builder(&proposalProposerCall { proposalId })
        }
        ///Creates a new call builder for the [`proposalSnapshot`] function.
        pub fn proposalSnapshot(
            &self,
            proposalId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, proposalSnapshotCall, N> {
            self.call_builder(&proposalSnapshotCall { proposalId })
        }
        ///Creates a new call builder for the [`proposalThreshold`] function.
        pub fn proposalThreshold(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, proposalThresholdCall, N> {
            self.call_builder(&proposalThresholdCall {})
        }
        ///Creates a new call builder for the [`proposalVotes`] function.
        pub fn proposalVotes(
            &self,
            proposalId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, proposalVotesCall, N> {
            self.call_builder(&proposalVotesCall { proposalId })
        }
        ///Creates a new call builder for the [`propose`] function.
        pub fn propose(
            &self,
            targets: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            values: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
            calldatas: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
            description: alloy::sol_types::private::String,
        ) -> alloy_contract::SolCallBuilder<T, &P, proposeCall, N> {
            self.call_builder(
                &proposeCall {
                    targets,
                    values,
                    calldatas,
                    description,
                },
            )
        }
        ///Creates a new call builder for the [`proxiableUUID`] function.
        pub fn proxiableUUID(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, proxiableUUIDCall, N> {
            self.call_builder(&proxiableUUIDCall {})
        }
        ///Creates a new call builder for the [`queue`] function.
        pub fn queue(
            &self,
            targets: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            values: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
            calldatas: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
            descriptionHash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, queueCall, N> {
            self.call_builder(
                &queueCall {
                    targets,
                    values,
                    calldatas,
                    descriptionHash,
                },
            )
        }
        ///Creates a new call builder for the [`quorum`] function.
        pub fn quorum(
            &self,
            blockNumber: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, quorumCall, N> {
            self.call_builder(&quorumCall { blockNumber })
        }
        ///Creates a new call builder for the [`quorumDenominator`] function.
        pub fn quorumDenominator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, quorumDenominatorCall, N> {
            self.call_builder(&quorumDenominatorCall {})
        }
        ///Creates a new call builder for the [`quorumNumerator_0`] function.
        pub fn quorumNumerator_0(
            &self,
            timepoint: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, quorumNumerator_0Call, N> {
            self.call_builder(&quorumNumerator_0Call { timepoint })
        }
        ///Creates a new call builder for the [`quorumNumerator_1`] function.
        pub fn quorumNumerator_1(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, quorumNumerator_1Call, N> {
            self.call_builder(&quorumNumerator_1Call {})
        }
        ///Creates a new call builder for the [`relay`] function.
        pub fn relay(
            &self,
            target: alloy::sol_types::private::Address,
            value: alloy::sol_types::private::primitives::aliases::U256,
            data: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, relayCall, N> {
            self.call_builder(&relayCall { target, value, data })
        }
        ///Creates a new call builder for the [`setProposalThreshold`] function.
        pub fn setProposalThreshold(
            &self,
            newProposalThreshold: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, setProposalThresholdCall, N> {
            self.call_builder(
                &setProposalThresholdCall {
                    newProposalThreshold,
                },
            )
        }
        ///Creates a new call builder for the [`setVotingDelay`] function.
        pub fn setVotingDelay(
            &self,
            newVotingDelay: alloy::sol_types::private::primitives::aliases::U48,
        ) -> alloy_contract::SolCallBuilder<T, &P, setVotingDelayCall, N> {
            self.call_builder(
                &setVotingDelayCall {
                    newVotingDelay,
                },
            )
        }
        ///Creates a new call builder for the [`setVotingPeriod`] function.
        pub fn setVotingPeriod(
            &self,
            newVotingPeriod: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, setVotingPeriodCall, N> {
            self.call_builder(
                &setVotingPeriodCall {
                    newVotingPeriod,
                },
            )
        }
        ///Creates a new call builder for the [`state`] function.
        pub fn state(
            &self,
            proposalId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, stateCall, N> {
            self.call_builder(&stateCall { proposalId })
        }
        ///Creates a new call builder for the [`supportsInterface`] function.
        pub fn supportsInterface(
            &self,
            interfaceId: alloy::sol_types::private::FixedBytes<4>,
        ) -> alloy_contract::SolCallBuilder<T, &P, supportsInterfaceCall, N> {
            self.call_builder(
                &supportsInterfaceCall {
                    interfaceId,
                },
            )
        }
        ///Creates a new call builder for the [`timelock`] function.
        pub fn timelock(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, timelockCall, N> {
            self.call_builder(&timelockCall {})
        }
        ///Creates a new call builder for the [`token`] function.
        pub fn token(&self) -> alloy_contract::SolCallBuilder<T, &P, tokenCall, N> {
            self.call_builder(&tokenCall {})
        }
        ///Creates a new call builder for the [`updateQuorumNumerator`] function.
        pub fn updateQuorumNumerator(
            &self,
            newQuorumNumerator: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateQuorumNumeratorCall, N> {
            self.call_builder(
                &updateQuorumNumeratorCall {
                    newQuorumNumerator,
                },
            )
        }
        ///Creates a new call builder for the [`updateTimelock`] function.
        pub fn updateTimelock(
            &self,
            newTimelock: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateTimelockCall, N> {
            self.call_builder(&updateTimelockCall { newTimelock })
        }
        ///Creates a new call builder for the [`upgradeToAndCall`] function.
        pub fn upgradeToAndCall(
            &self,
            newImplementation: alloy::sol_types::private::Address,
            data: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, upgradeToAndCallCall, N> {
            self.call_builder(
                &upgradeToAndCallCall {
                    newImplementation,
                    data,
                },
            )
        }
        ///Creates a new call builder for the [`version`] function.
        pub fn version(&self) -> alloy_contract::SolCallBuilder<T, &P, versionCall, N> {
            self.call_builder(&versionCall {})
        }
        ///Creates a new call builder for the [`votingDelay`] function.
        pub fn votingDelay(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, votingDelayCall, N> {
            self.call_builder(&votingDelayCall {})
        }
        ///Creates a new call builder for the [`votingPeriod`] function.
        pub fn votingPeriod(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, votingPeriodCall, N> {
            self.call_builder(&votingPeriodCall {})
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > TangleGovernorInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`EIP712DomainChanged`] event.
        pub fn EIP712DomainChanged_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, EIP712DomainChanged, N> {
            self.event_filter::<EIP712DomainChanged>()
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`ProposalCanceled`] event.
        pub fn ProposalCanceled_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ProposalCanceled, N> {
            self.event_filter::<ProposalCanceled>()
        }
        ///Creates a new event filter for the [`ProposalCreated`] event.
        pub fn ProposalCreated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ProposalCreated, N> {
            self.event_filter::<ProposalCreated>()
        }
        ///Creates a new event filter for the [`ProposalExecuted`] event.
        pub fn ProposalExecuted_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ProposalExecuted, N> {
            self.event_filter::<ProposalExecuted>()
        }
        ///Creates a new event filter for the [`ProposalQueued`] event.
        pub fn ProposalQueued_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ProposalQueued, N> {
            self.event_filter::<ProposalQueued>()
        }
        ///Creates a new event filter for the [`ProposalThresholdSet`] event.
        pub fn ProposalThresholdSet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ProposalThresholdSet, N> {
            self.event_filter::<ProposalThresholdSet>()
        }
        ///Creates a new event filter for the [`QuorumNumeratorUpdated`] event.
        pub fn QuorumNumeratorUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, QuorumNumeratorUpdated, N> {
            self.event_filter::<QuorumNumeratorUpdated>()
        }
        ///Creates a new event filter for the [`TimelockChange`] event.
        pub fn TimelockChange_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, TimelockChange, N> {
            self.event_filter::<TimelockChange>()
        }
        ///Creates a new event filter for the [`Upgraded`] event.
        pub fn Upgraded_filter(&self) -> alloy_contract::Event<T, &P, Upgraded, N> {
            self.event_filter::<Upgraded>()
        }
        ///Creates a new event filter for the [`VoteCast`] event.
        pub fn VoteCast_filter(&self) -> alloy_contract::Event<T, &P, VoteCast, N> {
            self.event_filter::<VoteCast>()
        }
        ///Creates a new event filter for the [`VoteCastWithParams`] event.
        pub fn VoteCastWithParams_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, VoteCastWithParams, N> {
            self.event_filter::<VoteCastWithParams>()
        }
        ///Creates a new event filter for the [`VotingDelaySet`] event.
        pub fn VotingDelaySet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, VotingDelaySet, N> {
            self.event_filter::<VotingDelaySet>()
        }
        ///Creates a new event filter for the [`VotingPeriodSet`] event.
        pub fn VotingPeriodSet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, VotingPeriodSet, N> {
            self.event_filter::<VotingPeriodSet>()
        }
    }
}
