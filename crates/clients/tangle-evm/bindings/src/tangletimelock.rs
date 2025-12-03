///Module containing a contract's types and functions.
/**

```solidity
library TimelockControllerUpgradeable {
    type OperationState is uint8;
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod TimelockControllerUpgradeable {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperationState(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<OperationState> for u8 {
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
        impl OperationState {
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
        impl alloy_sol_types::SolType for OperationState {
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
        impl alloy_sol_types::EventTopic for OperationState {
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
    /**Creates a new wrapper around an on-chain [`TimelockControllerUpgradeable`](self) contract instance.

See the [wrapper's documentation](`TimelockControllerUpgradeableInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> TimelockControllerUpgradeableInstance<T, P, N> {
        TimelockControllerUpgradeableInstance::<T, P, N>::new(address, provider)
    }
    /**A [`TimelockControllerUpgradeable`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`TimelockControllerUpgradeable`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct TimelockControllerUpgradeableInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for TimelockControllerUpgradeableInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("TimelockControllerUpgradeableInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > TimelockControllerUpgradeableInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`TimelockControllerUpgradeable`](self) contract instance.

See the [wrapper's documentation](`TimelockControllerUpgradeableInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> TimelockControllerUpgradeableInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(
            self,
        ) -> TimelockControllerUpgradeableInstance<T, P, N> {
            TimelockControllerUpgradeableInstance {
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
    > TimelockControllerUpgradeableInstance<T, P, N> {
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
    > TimelockControllerUpgradeableInstance<T, P, N> {
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
library TimelockControllerUpgradeable {
    type OperationState is uint8;
}

interface TangleTimelock {
    error AccessControlBadConfirmation();
    error AccessControlUnauthorizedAccount(address account, bytes32 neededRole);
    error AddressEmptyCode(address target);
    error ERC1967InvalidImplementation(address implementation);
    error ERC1967NonPayable();
    error FailedCall();
    error InvalidInitialization();
    error NotInitializing();
    error TimelockInsufficientDelay(uint256 delay, uint256 minDelay);
    error TimelockInvalidOperationLength(uint256 targets, uint256 payloads, uint256 values);
    error TimelockUnauthorizedCaller(address caller);
    error TimelockUnexecutedPredecessor(bytes32 predecessorId);
    error TimelockUnexpectedOperationState(bytes32 operationId, bytes32 expectedStates);
    error UUPSUnauthorizedCallContext();
    error UUPSUnsupportedProxiableUUID(bytes32 slot);

    event CallExecuted(bytes32 indexed id, uint256 indexed index, address target, uint256 value, bytes data);
    event CallSalt(bytes32 indexed id, bytes32 salt);
    event CallScheduled(bytes32 indexed id, uint256 indexed index, address target, uint256 value, bytes data, bytes32 predecessor, uint256 delay);
    event Cancelled(bytes32 indexed id);
    event Initialized(uint64 version);
    event MinDelayChange(uint256 oldDuration, uint256 newDuration);
    event RoleAdminChanged(bytes32 indexed role, bytes32 indexed previousAdminRole, bytes32 indexed newAdminRole);
    event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
    event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);
    event Upgraded(address indexed implementation);

    constructor();

    receive() external payable;

    function CANCELLER_ROLE() external view returns (bytes32);
    function DEFAULT_ADMIN_ROLE() external view returns (bytes32);
    function EXECUTOR_ROLE() external view returns (bytes32);
    function MAX_DELAY() external view returns (uint256);
    function MIN_DELAY() external view returns (uint256);
    function PROPOSER_ROLE() external view returns (bytes32);
    function UPGRADE_INTERFACE_VERSION() external view returns (string memory);
    function cancel(bytes32 id) external;
    function execute(address target, uint256 value, bytes memory payload, bytes32 predecessor, bytes32 salt) external payable;
    function executeBatch(address[] memory targets, uint256[] memory values, bytes[] memory payloads, bytes32 predecessor, bytes32 salt) external payable;
    function getMinDelay() external view returns (uint256);
    function getOperationState(bytes32 id) external view returns (TimelockControllerUpgradeable.OperationState);
    function getRoleAdmin(bytes32 role) external view returns (bytes32);
    function getTimestamp(bytes32 id) external view returns (uint256);
    function grantRole(bytes32 role, address account) external;
    function hasRole(bytes32 role, address account) external view returns (bool);
    function hashOperation(address target, uint256 value, bytes memory data, bytes32 predecessor, bytes32 salt) external pure returns (bytes32);
    function hashOperationBatch(address[] memory targets, uint256[] memory values, bytes[] memory payloads, bytes32 predecessor, bytes32 salt) external pure returns (bytes32);
    function initialize(uint256 minDelay, address[] memory proposers, address[] memory executors, address admin) external;
    function isCanceller(address account) external view returns (bool);
    function isExecutor(address account) external view returns (bool);
    function isOperation(bytes32 id) external view returns (bool);
    function isOperationDone(bytes32 id) external view returns (bool);
    function isOperationPending(bytes32 id) external view returns (bool);
    function isOperationReady(bytes32 id) external view returns (bool);
    function isProposer(address account) external view returns (bool);
    function onERC1155BatchReceived(address, address, uint256[] memory, uint256[] memory, bytes memory) external returns (bytes4);
    function onERC1155Received(address, address, uint256, uint256, bytes memory) external returns (bytes4);
    function onERC721Received(address, address, uint256, bytes memory) external returns (bytes4);
    function proxiableUUID() external view returns (bytes32);
    function renounceRole(bytes32 role, address callerConfirmation) external;
    function revokeRole(bytes32 role, address account) external;
    function schedule(address target, uint256 value, bytes memory data, bytes32 predecessor, bytes32 salt, uint256 delay) external;
    function scheduleBatch(address[] memory targets, uint256[] memory values, bytes[] memory payloads, bytes32 predecessor, bytes32 salt, uint256 delay) external;
    function supportsInterface(bytes4 interfaceId) external view returns (bool);
    function updateDelay(uint256 newDelay) external;
    function upgradeToAndCall(address newImplementation, bytes memory data) external payable;
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
    "name": "CANCELLER_ROLE",
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
    "name": "DEFAULT_ADMIN_ROLE",
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
    "name": "EXECUTOR_ROLE",
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
    "name": "MAX_DELAY",
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
    "name": "MIN_DELAY",
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
    "name": "PROPOSER_ROLE",
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
        "name": "id",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "execute",
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
        "name": "payload",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "predecessor",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "salt",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "executeBatch",
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
        "name": "payloads",
        "type": "bytes[]",
        "internalType": "bytes[]"
      },
      {
        "name": "predecessor",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "salt",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "getMinDelay",
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
    "name": "getOperationState",
    "inputs": [
      {
        "name": "id",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "enum TimelockControllerUpgradeable.OperationState"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getRoleAdmin",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
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
    "name": "getTimestamp",
    "inputs": [
      {
        "name": "id",
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
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "grantRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "hasRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
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
    "name": "hashOperation",
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
      },
      {
        "name": "predecessor",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "salt",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "hashOperationBatch",
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
        "name": "payloads",
        "type": "bytes[]",
        "internalType": "bytes[]"
      },
      {
        "name": "predecessor",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "salt",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "initialize",
    "inputs": [
      {
        "name": "minDelay",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "proposers",
        "type": "address[]",
        "internalType": "address[]"
      },
      {
        "name": "executors",
        "type": "address[]",
        "internalType": "address[]"
      },
      {
        "name": "admin",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "isCanceller",
    "inputs": [
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
    "name": "isExecutor",
    "inputs": [
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
    "name": "isOperation",
    "inputs": [
      {
        "name": "id",
        "type": "bytes32",
        "internalType": "bytes32"
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
    "name": "isOperationDone",
    "inputs": [
      {
        "name": "id",
        "type": "bytes32",
        "internalType": "bytes32"
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
    "name": "isOperationPending",
    "inputs": [
      {
        "name": "id",
        "type": "bytes32",
        "internalType": "bytes32"
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
    "name": "isOperationReady",
    "inputs": [
      {
        "name": "id",
        "type": "bytes32",
        "internalType": "bytes32"
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
    "name": "isProposer",
    "inputs": [
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
    "name": "renounceRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "callerConfirmation",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "revokeRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "schedule",
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
      },
      {
        "name": "predecessor",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "salt",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "delay",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "scheduleBatch",
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
        "name": "payloads",
        "type": "bytes[]",
        "internalType": "bytes[]"
      },
      {
        "name": "predecessor",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "salt",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "delay",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
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
    "name": "updateDelay",
    "inputs": [
      {
        "name": "newDelay",
        "type": "uint256",
        "internalType": "uint256"
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
    "type": "event",
    "name": "CallExecuted",
    "inputs": [
      {
        "name": "id",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "index",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "target",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "value",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "data",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "CallSalt",
    "inputs": [
      {
        "name": "id",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "salt",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "CallScheduled",
    "inputs": [
      {
        "name": "id",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "index",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "target",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "value",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "data",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      },
      {
        "name": "predecessor",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      },
      {
        "name": "delay",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Cancelled",
    "inputs": [
      {
        "name": "id",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      }
    ],
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
    "name": "MinDelayChange",
    "inputs": [
      {
        "name": "oldDuration",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newDuration",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RoleAdminChanged",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "previousAdminRole",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "newAdminRole",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RoleGranted",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RoleRevoked",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
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
    "type": "error",
    "name": "AccessControlBadConfirmation",
    "inputs": []
  },
  {
    "type": "error",
    "name": "AccessControlUnauthorizedAccount",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "neededRole",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
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
    "name": "TimelockInsufficientDelay",
    "inputs": [
      {
        "name": "delay",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "minDelay",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "TimelockInvalidOperationLength",
    "inputs": [
      {
        "name": "targets",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "payloads",
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
    "name": "TimelockUnauthorizedCaller",
    "inputs": [
      {
        "name": "caller",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "TimelockUnexecutedPredecessor",
    "inputs": [
      {
        "name": "predecessorId",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "TimelockUnexpectedOperationState",
    "inputs": [
      {
        "name": "operationId",
        "type": "bytes32",
        "internalType": "bytes32"
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
pub mod TangleTimelock {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a080604052346100e857306080527ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a005460ff8160401c166100d9576002600160401b03196001600160401b03821601610073575b60405161212e90816100ed8239608051818181610d570152610de80152f35b6001600160401b0319166001600160401b039081177ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a005581527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d290602090a15f80610054565b63f92ee8a960e01b5f5260045ffd5b5f80fdfe6080604052600436101561001a575b3615610018575f80fd5b005b5f3560e01c806301d5062a1461125e57806301ffc9a7146111ee57806307bd0265146111c7578063134008d31461111a57806313bc9f20146110fc578063150b7a02146110a7578063248a9ca3146110895780632ab0f5291461106b5780632f2ff15d1461103a57806331d507501461101c57806336568abe14610fd85780634125ff9014610fbb5780634f1ef28614610dab57806352d1902d14610d45578063584b153e14610d1d57806364d6235314610ca057806374ec29a014610c525780637958004c14610c0f5780638065657f14610bf05780638f2a0bb014610a6b5780638f61f4f514610a3157806391d14854146109dc5780639f81aed7146109bf578063a217fddf146109a5578063ad3cb1cc14610947578063b08e51c01461090d578063b1c5f427146108e3578063b426475e14610895578063bc197c8114610800578063c4c4c7b314610503578063c4d252f51461042c578063d45c4435146103f6578063d547741f146103be578063debfda3014610370578063e38335e514610237578063f23a6e61146101e25763f27a0c920361000e57346101de575f3660031901126101de5760205f805160206120a283398151915254604051908152f35b5f80fd5b346101de5760a03660031901126101de576101fb611309565b5061020461131f565b506084356001600160401b0381116101de57610224903690600401611414565b5060405163f23a6e6160e01b8152602090f35b6102403661148a565b5f80525f805160206120e28339815191526020527fdd1864c1ab258d549957f6a4d7e3a5005661df58241dbbc1700fb170ef0615465492979196919593949260ff1615610362575b828214801590610358575b61033d576102aa6102b191888a888789888d6117f0565b9687611ae4565b5f5b8181106102c35761001887611b97565b8080887fc2617efa69bab66782fa219543714338489c4e9e178271560a91b82c3f612b58888861033461031b8f986001998f828e61030e8f836103099161031496611777565b61179b565b97611777565b35956117af565b9061032882828787611b6b565b60405194859485611629565b0390a3016102b3565b50869063ffb0321160e01b5f5260045260245260445260645ffd5b5087821415610293565b61036b33611a2f565b610288565b346101de5760203660031901126101de57610389611309565b6001600160a01b03165f9081525f805160206120e2833981519152602090815260409182902054915160ff9092161515825290f35b346101de5760403660031901126101de576100186004356103dd61131f565b906103f16103ea82611668565b3390611a8f565b611e94565b346101de5760203660031901126101de576004355f525f80516020612022833981519152602052602060405f2054604051908152f35b346101de5760203660031901126101de57335f9081525f8051602061200283398151915260205260409020546004359060ff16156104cc5761046d816116b5565b156104b257805f525f805160206120228339815191526020525f60408120557fbaa1eb22f2a492ba1a5fea61b8df4d27c6c8b5f3971e63bb58fa14ff72eedb705f80a2005b635ead8eb560e01b5f52600452600460021760245260445ffd5b63e2517d3f60e01b5f52336004527ffd643c72710c63c0180259aba6b2d05451e3591a24e58b62239378085726f78360245260445ffd5b346101de5760803660031901126101de576004356024356001600160401b0381116101de5761053690369060040161156c565b906044356001600160401b0381116101de5761055690369060040161156c565b6064356001600160a01b03811692908381036101de575f80516020612102833981519152549360ff8560401c1615946001600160401b038116801590816107f8575b60011490816107ee575b1590816107e5575b506107d65767ffffffffffffffff1981166001175f8051602061210283398151915255856107aa575b506201518083106107735762278d00831161073d576105f0611f3f565b6105f8611f3f565b61060130611bc1565b5061072d575b505f5b8451811015610654576001906106326001600160a01b0361062b8389611f6a565b5116611c5d565b5061064d828060a01b036106468389611f6a565b5116611cf0565b500161060a565b50825f5b8351811015610687576001906106806001600160a01b036106798388611f6a565b5116611d83565b5001610658565b507f11c24f4ead16507c69ac467fbd5e4eed5fb5c699626d2cc6d66421df253886d5604083805f805160206120a2833981519152558151905f82526020820152a16106d0611f3f565b6106d657005b68ff0000000000000000195f8051602061210283398151915254165f80516020612102833981519152557fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2602060405160018152a1005b61073690611bc1565b5084610607565b60405162461bcd60e51b815260206004820152600e60248201526d44656c617920746f6f206c6f6e6760901b6044820152606490fd5b60405162461bcd60e51b815260206004820152600f60248201526e11195b185e481d1bdbc81cda1bdc9d608a1b6044820152606490fd5b68ffffffffffffffffff191668010000000000000001175f8051602061210283398151915255866105d3565b63f92ee8a960e01b5f5260045ffd5b905015886105aa565b303b1591506105a2565b879150610598565b346101de5760a03660031901126101de57610819611309565b5061082261131f565b506044356001600160401b0381116101de5761084290369060040161150f565b506064356001600160401b0381116101de5761086290369060040161150f565b506084356001600160401b0381116101de57610882903690600401611414565b5060405163bc197c8160e01b8152602090f35b346101de5760203660031901126101de576108ae611309565b6001600160a01b03165f9081525f80516020612002833981519152602090815260409182902054915160ff9092161515825290f35b346101de5760206109056108f63661148a565b969590959491949392936117f0565b604051908152f35b346101de575f3660031901126101de5760206040517ffd643c72710c63c0180259aba6b2d05451e3591a24e58b62239378085726f7838152f35b346101de575f3660031901126101de57604080519061096681836113c4565b600582526020820191640352e302e360dc1b83528151928391602083525180918160208501528484015e5f828201840152601f01601f19168101030190f35b346101de575f3660031901126101de5760206040515f8152f35b346101de575f3660031901126101de576020604051620151808152f35b346101de5760403660031901126101de576109f561131f565b6004355f525f8051602061208283398151915260205260405f209060018060a01b03165f52602052602060ff60405f2054166040519015158152f35b346101de575f3660031901126101de5760206040517fb09aa5aeb3702cfd50b6b62bc4532604938f21248a27a1d5ca736082b6819cc18152f35b346101de5760c03660031901126101de576004356001600160401b0381116101de57610a9b90369060040161145a565b906024356001600160401b0381116101de57610abb90369060040161145a565b6044929192356001600160401b0381116101de57610add90369060040161145a565b9390916064356084359560a43592610af4336119bc565b808914801590610be6575b610bcc57610b1388848489858a8f8e6117f0565b98610b1e858b611931565b895f5b828110610b5e57508980610b3157005b60207f20fda5fd27a1ea7bf5b9567f143ac5470bb059374a27e8f67cb44f946f6d038791604051908152a2005b806001927f4cf4410cc57040e44862ef0f45f3dd5a5e02db8eb8add648d4b0e236f1d07dca8b8b610bc18f8c610bb48f928e610bad8f8f90610ba76103098f8097948195611777565b99611777565b35976117af565b90604051968796876115f1565b0390a3018a90610b21565b908863ffb0321160e01b5f5260045260245260445260645ffd5b5081891415610aff565b346101de576020610905610c0336611376565b94939093929192611722565b346101de5760203660031901126101de57610c2b6004356116de565b6040516004821015610c3e576020918152f35b634e487b7160e01b5f52602160045260245ffd5b346101de5760203660031901126101de57610c6b611309565b6001600160a01b03165f9081525f80516020612042833981519152602090815260409182902054915160ff9092161515825290f35b346101de5760203660031901126101de57600435303303610d0a577f11c24f4ead16507c69ac467fbd5e4eed5fb5c699626d2cc6d66421df253886d560405f805160206120a2833981519152548151908152836020820152a15f805160206120a283398151915255005b63e2850c5960e01b5f523360045260245ffd5b346101de5760203660031901126101de576020610d3b6004356116b5565b6040519015158152f35b346101de575f3660031901126101de577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03163003610d9c5760206040515f805160206120628339815191528152f35b63703e46dd60e11b5f5260045ffd5b60403660031901126101de57610dbf611309565b6024356001600160401b0381116101de57610dde903690600401611414565b6001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016308114908115610f99575b50610d9c57303303610f55576040516352d1902d60e01b81526001600160a01b0383169290602081600481875afa5f9181610f21575b50610e615783634c9c8ce360e01b5f5260045260245ffd5b805f80516020612062833981519152859203610f0f5750813b15610efd575f8051602061206283398151915280546001600160a01b031916821790557fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b5f80a2815115610ee5575f8083602061001895519101845af4610edf611b3c565b91611f9c565b505034610eee57005b63b398979f60e01b5f5260045ffd5b634c9c8ce360e01b5f5260045260245ffd5b632a87526960e21b5f5260045260245ffd5b9091506020813d602011610f4d575b81610f3d602093836113c4565b810103126101de57519085610e49565b3d9150610f30565b606460405162461bcd60e51b815260206004820152602060248201527f4f6e6c792073656c662d757067726164652076696120676f7665726e616e63656044820152fd5b5f80516020612062833981519152546001600160a01b03161415905083610e13565b346101de575f3660031901126101de57602060405162278d008152f35b346101de5760403660031901126101de57610ff161131f565b336001600160a01b0382160361100d5761001890600435611e94565b63334bd91960e11b5f5260045ffd5b346101de5760203660031901126101de576020610d3b60043561169e565b346101de5760403660031901126101de5761001860043561105961131f565b906110666103ea82611668565b611e03565b346101de5760203660031901126101de576020610d3b600435611686565b346101de5760203660031901126101de576020610905600435611668565b346101de5760803660031901126101de576110c0611309565b506110c961131f565b506064356001600160401b0381116101de576110e9903690600401611414565b50604051630a85bd0160e11b8152602090f35b346101de5760203660031901126101de576020610d3b600435611650565b6100186111a55f6111b17fc2617efa69bab66782fa219543714338489c4e9e178271560a91b82c3f612b5861119c61115136611376565b5f805160206120c28339815191528a9995979299949394525f8051602061208283398151915260205260408a208a805260205260ff60408b205416156111b9575b8884848989611722565b98899788611ae4565b61032882828787611b6b565b0390a3611b97565b6111c233611a2f565b611192565b346101de575f3660031901126101de5760206040515f805160206120c28339815191528152f35b346101de5760203660031901126101de5760043563ffffffff60e01b81168091036101de57602090630271189760e51b8114908115611233575b506040519015158152f35b637965db0b60e01b81149150811561124d575b5082611228565b6301ffc9a760e01b14905082611246565b346101de5760c03660031901126101de57611277611309565b602435906044356001600160401b0381116101de577f4cf4410cc57040e44862ef0f45f3dd5a5e02db8eb8add648d4b0e236f1d07dca926112bd5f923690600401611349565b949091606435946112ff6084359660a435906112d8336119bc565b6112e689828c8a8989611722565b998a976112f3848a611931565b604051968796876115f1565b0390a380610b3157005b600435906001600160a01b03821682036101de57565b602435906001600160a01b03821682036101de57565b35906001600160a01b03821682036101de57565b9181601f840112156101de578235916001600160401b0383116101de57602083818601950101116101de57565b60a06003198201126101de576004356001600160a01b03811681036101de579160243591604435906001600160401b0382116101de576113b891600401611349565b90916064359060843590565b90601f801991011681019081106001600160401b038211176113e557604052565b634e487b7160e01b5f52604160045260245ffd5b6001600160401b0381116113e557601f01601f191660200190565b81601f820112156101de5780359061142b826113f9565b9261143960405194856113c4565b828452602083830101116101de57815f926020809301838601378301015290565b9181601f840112156101de578235916001600160401b0383116101de576020808501948460051b0101116101de57565b60a06003198201126101de576004356001600160401b0381116101de57816114b49160040161145a565b929092916024356001600160401b0381116101de57816114d69160040161145a565b92909291604435906001600160401b0382116101de576113b89160040161145a565b6001600160401b0381116113e55760051b60200190565b9080601f830112156101de578135611526816114f8565b9261153460405194856113c4565b81845260208085019260051b8201019283116101de57602001905b82821061155c5750505090565b813581526020918201910161154f565b9080601f830112156101de578135611583816114f8565b9261159160405194856113c4565b81845260208085019260051b8201019283116101de57602001905b8282106115b95750505090565b602080916115c684611335565b8152019101906115ac565b908060209392818452848401375f828201840152601f01601f1916010190565b92909361161f926080959897969860018060a01b03168552602085015260a0604085015260a08401916115d1565b9460608201520152565b61164d949260609260018060a01b03168252602082015281604082015201916115d1565b90565b611659906116de565b6004811015610c3e5760021490565b5f525f80516020612082833981519152602052600160405f20015490565b61168f906116de565b6004811015610c3e5760031490565b6116a7906116de565b6004811015610c3e57151590565b6116be906116de565b6004811015610c3e57600181149081156116d6575090565b600291501490565b5f525f8051602061202283398151915260205260405f205480155f1461170357505f90565b600181036117115750600390565b42101561171d57600190565b600290565b9461175861177194959293604051968795602087019960018060a01b03168a52604087015260a0606087015260c08601916115d1565b91608084015260a083015203601f1981018352826113c4565b51902090565b91908110156117875760051b0190565b634e487b7160e01b5f52603260045260245ffd5b356001600160a01b03811681036101de5790565b91908110156117875760051b81013590601e19813603018212156101de5701908135916001600160401b0383116101de5760200182360381136101de579190565b9693949190969592956040519660208801988060c08a0160a08c525260e0890192905f5b81811061190b57505050878203601f190160408901528082526001600160fb1b0381116101de579087959394929160051b8092602083013701848103606086015260208101849052600584901b8101604090810194908201915f90889036829003601e1901905b8484106118a5575050505050506117719450608084015260a083015203601f1981018352826113c4565b91939597909294969850601f19601f19838303010187528935838112156101de57840190602082359201916001600160401b0381116101de5780360383136101de576118f760209283926001956115d1565b9b0197019401918a9896999795939161187b565b909193602080600192838060a01b0361192389611335565b168152019501929101611814565b9061193b8261169e565b6119a4575f805160206120a28339815191525480821061198e575042019081421161197a575f525f8051602061202283398151915260205260405f2055565b634e487b7160e01b5f52601160045260245ffd5b90635433660960e01b5f5260045260245260445ffd5b50635ead8eb560e01b5f52600452600160245260445ffd5b6001600160a01b0381165f9081525f80516020612042833981519152602052604090205460ff16156119eb5750565b63e2517d3f60e01b5f9081526001600160a01b03919091166004527fb09aa5aeb3702cfd50b6b62bc4532604938f21248a27a1d5ca736082b6819cc1602452604490fd5b6001600160a01b0381165f9081525f805160206120e2833981519152602052604090205460ff1615611a5e5750565b63e2517d3f60e01b5f9081526001600160a01b03919091166004525f805160206120c2833981519152602452604490fd5b90815f525f8051602061208283398151915260205260405f2060018060a01b0382165f5260205260ff60405f20541615611ac7575050565b63e2517d3f60e01b5f5260018060a01b031660045260245260445ffd5b611aed81611650565b15611b25575080151580611b15575b611b035750565b63121534c360e31b5f5260045260245ffd5b50611b1f81611686565b15611afc565b635ead8eb560e01b5f52600452600460245260445ffd5b3d15611b66573d90611b4d826113f9565b91611b5b60405193846113c4565b82523d5f602084013e565b606090565b611b94935f93928493826040519384928337810185815203925af1611b8e611b3c565b90611f30565b50565b611ba081611650565b15611b25575f525f80516020612022833981519152602052600160405f2055565b6001600160a01b0381165f9081527fb7db2dd08fcb62d0c9e08c51941cae53c267786a0b75803fb7960902fc8ef97d602052604090205460ff16611c58576001600160a01b03165f8181527fb7db2dd08fcb62d0c9e08c51941cae53c267786a0b75803fb7960902fc8ef97d60205260408120805460ff191660011790553391905f80516020611fe28339815191528180a4600190565b505f90565b6001600160a01b0381165f9081525f80516020612042833981519152602052604090205460ff16611c58576001600160a01b03165f8181525f8051602061204283398151915260205260408120805460ff191660011790553391907fb09aa5aeb3702cfd50b6b62bc4532604938f21248a27a1d5ca736082b6819cc1905f80516020611fe28339815191529080a4600190565b6001600160a01b0381165f9081525f80516020612002833981519152602052604090205460ff16611c58576001600160a01b03165f8181525f8051602061200283398151915260205260408120805460ff191660011790553391907ffd643c72710c63c0180259aba6b2d05451e3591a24e58b62239378085726f783905f80516020611fe28339815191529080a4600190565b6001600160a01b0381165f9081525f805160206120e2833981519152602052604090205460ff16611c58576001600160a01b03165f8181525f805160206120e283398151915260205260408120805460ff191660011790553391905f805160206120c2833981519152905f80516020611fe28339815191529080a4600190565b5f8181525f80516020612082833981519152602090815260408083206001600160a01b038616845290915290205460ff16611e8e575f8181525f80516020612082833981519152602090815260408083206001600160a01b0395909516808452949091528120805460ff19166001179055339291905f80516020611fe28339815191529080a4600190565b50505f90565b5f8181525f80516020612082833981519152602090815260408083206001600160a01b038616845290915290205460ff1615611e8e575f8181525f80516020612082833981519152602090815260408083206001600160a01b0395909516808452949091528120805460ff19169055339291907ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b9080a4600190565b909190611f3d5750611f7e565b565b60ff5f805160206121028339815191525460401c1615611f5b57565b631afcd79f60e31b5f5260045ffd5b80518210156117875760209160051b010190565b805115611f8d57805190602001fd5b63d6bda27560e01b5f5260045ffd5b90611fa75750611f7e565b81511580611fd8575b611fb8575090565b639996b31560e01b5f9081526001600160a01b0391909116600452602490fd5b50803b15611fb056fe2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0dfa71e07f24c4701ef65a970775979de1292cfe909335cd18a32d2b7b739879149a37c2aa9d186a0969ff8a8267bf4e07e864c2f2768f5040949e28a624fb36005a8734c34b98d7c96eb2ea25f298989407e1f25da116ec139bcce0887bcb7cf7360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc02dd7bc7dec4dceedda775e58dd541e08a116c6c53815c0bd028192f7b6268009a37c2aa9d186a0969ff8a8267bf4e07e864c2f2768f5040949e28a624fb3601d8aa0f3194971a2a116679f7c2090f6939c8d4e01a2a8d7e41d55e5351469e6352fce5e8a5d0d9e8d1ea29f4525e512e9c27bf92cae50374d497f918ab48f382f0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00a164736f6c634300081a000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0\x80`@R4a\0\xE8W0`\x80R\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`\xFF\x81`@\x1C\x16a\0\xD9W`\x02`\x01`@\x1B\x03\x19`\x01`\x01`@\x1B\x03\x82\x16\x01a\0sW[`@Qa!.\x90\x81a\0\xED\x829`\x80Q\x81\x81\x81a\rW\x01Ra\r\xE8\x01R\xF3[`\x01`\x01`@\x1B\x03\x19\x16`\x01`\x01`@\x1B\x03\x90\x81\x17\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0U\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x90\xA1_\x80a\0TV[c\xF9.\xE8\xA9`\xE0\x1B_R`\x04_\xFD[_\x80\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x1AW[6\x15a\0\x18W_\x80\xFD[\0[_5`\xE0\x1C\x80c\x01\xD5\x06*\x14a\x12^W\x80c\x01\xFF\xC9\xA7\x14a\x11\xEEW\x80c\x07\xBD\x02e\x14a\x11\xC7W\x80c\x13@\x08\xD3\x14a\x11\x1AW\x80c\x13\xBC\x9F \x14a\x10\xFCW\x80c\x15\x0Bz\x02\x14a\x10\xA7W\x80c$\x8A\x9C\xA3\x14a\x10\x89W\x80c*\xB0\xF5)\x14a\x10kW\x80c//\xF1]\x14a\x10:W\x80c1\xD5\x07P\x14a\x10\x1CW\x80c6V\x8A\xBE\x14a\x0F\xD8W\x80cA%\xFF\x90\x14a\x0F\xBBW\x80cO\x1E\xF2\x86\x14a\r\xABW\x80cR\xD1\x90-\x14a\rEW\x80cXK\x15>\x14a\r\x1DW\x80cd\xD6#S\x14a\x0C\xA0W\x80ct\xEC)\xA0\x14a\x0CRW\x80cyX\0L\x14a\x0C\x0FW\x80c\x80ee\x7F\x14a\x0B\xF0W\x80c\x8F*\x0B\xB0\x14a\nkW\x80c\x8Fa\xF4\xF5\x14a\n1W\x80c\x91\xD1HT\x14a\t\xDCW\x80c\x9F\x81\xAE\xD7\x14a\t\xBFW\x80c\xA2\x17\xFD\xDF\x14a\t\xA5W\x80c\xAD<\xB1\xCC\x14a\tGW\x80c\xB0\x8EQ\xC0\x14a\t\rW\x80c\xB1\xC5\xF4'\x14a\x08\xE3W\x80c\xB4&G^\x14a\x08\x95W\x80c\xBC\x19|\x81\x14a\x08\0W\x80c\xC4\xC4\xC7\xB3\x14a\x05\x03W\x80c\xC4\xD2R\xF5\x14a\x04,W\x80c\xD4\\D5\x14a\x03\xF6W\x80c\xD5Gt\x1F\x14a\x03\xBEW\x80c\xDE\xBF\xDA0\x14a\x03pW\x80c\xE3\x835\xE5\x14a\x027W\x80c\xF2:na\x14a\x01\xE2Wc\xF2z\x0C\x92\x03a\0\x0EW4a\x01\xDEW_6`\x03\x19\x01\x12a\x01\xDEW` _\x80Q` a \xA2\x839\x81Q\x91RT`@Q\x90\x81R\xF3[_\x80\xFD[4a\x01\xDEW`\xA06`\x03\x19\x01\x12a\x01\xDEWa\x01\xFBa\x13\tV[Pa\x02\x04a\x13\x1FV[P`\x845`\x01`\x01`@\x1B\x03\x81\x11a\x01\xDEWa\x02$\x906\x90`\x04\x01a\x14\x14V[P`@Qc\xF2:na`\xE0\x1B\x81R` \x90\xF3[a\x02@6a\x14\x8AV[_\x80R_\x80Q` a \xE2\x839\x81Q\x91R` R\x7F\xDD\x18d\xC1\xAB%\x8DT\x99W\xF6\xA4\xD7\xE3\xA5\0Va\xDFX$\x1D\xBB\xC1p\x0F\xB1p\xEF\x06\x15FT\x92\x97\x91\x96\x91\x95\x93\x94\x92`\xFF\x16\x15a\x03bW[\x82\x82\x14\x80\x15\x90a\x03XW[a\x03=Wa\x02\xAAa\x02\xB1\x91\x88\x8A\x88\x87\x89\x88\x8Da\x17\xF0V[\x96\x87a\x1A\xE4V[_[\x81\x81\x10a\x02\xC3Wa\0\x18\x87a\x1B\x97V[\x80\x80\x88\x7F\xC2a~\xFAi\xBA\xB6g\x82\xFA!\x95CqC8H\x9CN\x9E\x17\x82qV\n\x91\xB8,?a+X\x88\x88a\x034a\x03\x1B\x8F\x98`\x01\x99\x8F\x82\x8Ea\x03\x0E\x8F\x83a\x03\t\x91a\x03\x14\x96a\x17wV[a\x17\x9BV[\x97a\x17wV[5\x95a\x17\xAFV[\x90a\x03(\x82\x82\x87\x87a\x1BkV[`@Q\x94\x85\x94\x85a\x16)V[\x03\x90\xA3\x01a\x02\xB3V[P\x86\x90c\xFF\xB02\x11`\xE0\x1B_R`\x04R`$R`DR`d_\xFD[P\x87\x82\x14\x15a\x02\x93V[a\x03k3a\x1A/V[a\x02\x88V[4a\x01\xDEW` 6`\x03\x19\x01\x12a\x01\xDEWa\x03\x89a\x13\tV[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R_\x80Q` a \xE2\x839\x81Q\x91R` \x90\x81R`@\x91\x82\x90 T\x91Q`\xFF\x90\x92\x16\x15\x15\x82R\x90\xF3[4a\x01\xDEW`@6`\x03\x19\x01\x12a\x01\xDEWa\0\x18`\x045a\x03\xDDa\x13\x1FV[\x90a\x03\xF1a\x03\xEA\x82a\x16hV[3\x90a\x1A\x8FV[a\x1E\x94V[4a\x01\xDEW` 6`\x03\x19\x01\x12a\x01\xDEW`\x045_R_\x80Q` a \"\x839\x81Q\x91R` R` `@_ T`@Q\x90\x81R\xF3[4a\x01\xDEW` 6`\x03\x19\x01\x12a\x01\xDEW3_\x90\x81R_\x80Q` a \x02\x839\x81Q\x91R` R`@\x90 T`\x045\x90`\xFF\x16\x15a\x04\xCCWa\x04m\x81a\x16\xB5V[\x15a\x04\xB2W\x80_R_\x80Q` a \"\x839\x81Q\x91R` R_`@\x81 U\x7F\xBA\xA1\xEB\"\xF2\xA4\x92\xBA\x1A_\xEAa\xB8\xDFM'\xC6\xC8\xB5\xF3\x97\x1Ec\xBBX\xFA\x14\xFFr\xEE\xDBp_\x80\xA2\0[c^\xAD\x8E\xB5`\xE0\x1B_R`\x04R`\x04`\x02\x17`$R`D_\xFD[c\xE2Q}?`\xE0\x1B_R3`\x04R\x7F\xFDd<rq\x0Cc\xC0\x18\x02Y\xAB\xA6\xB2\xD0TQ\xE3Y\x1A$\xE5\x8Bb#\x93x\x08W&\xF7\x83`$R`D_\xFD[4a\x01\xDEW`\x806`\x03\x19\x01\x12a\x01\xDEW`\x045`$5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xDEWa\x056\x906\x90`\x04\x01a\x15lV[\x90`D5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xDEWa\x05V\x906\x90`\x04\x01a\x15lV[`d5`\x01`\x01`\xA0\x1B\x03\x81\x16\x92\x90\x83\x81\x03a\x01\xDEW_\x80Q` a!\x02\x839\x81Q\x91RT\x93`\xFF\x85`@\x1C\x16\x15\x94`\x01`\x01`@\x1B\x03\x81\x16\x80\x15\x90\x81a\x07\xF8W[`\x01\x14\x90\x81a\x07\xEEW[\x15\x90\x81a\x07\xE5W[Pa\x07\xD6Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16`\x01\x17_\x80Q` a!\x02\x839\x81Q\x91RU\x85a\x07\xAAW[Pb\x01Q\x80\x83\x10a\x07sWb'\x8D\0\x83\x11a\x07=Wa\x05\xF0a\x1F?V[a\x05\xF8a\x1F?V[a\x06\x010a\x1B\xC1V[Pa\x07-W[P_[\x84Q\x81\x10\x15a\x06TW`\x01\x90a\x062`\x01`\x01`\xA0\x1B\x03a\x06+\x83\x89a\x1FjV[Q\x16a\x1C]V[Pa\x06M\x82\x80`\xA0\x1B\x03a\x06F\x83\x89a\x1FjV[Q\x16a\x1C\xF0V[P\x01a\x06\nV[P\x82_[\x83Q\x81\x10\x15a\x06\x87W`\x01\x90a\x06\x80`\x01`\x01`\xA0\x1B\x03a\x06y\x83\x88a\x1FjV[Q\x16a\x1D\x83V[P\x01a\x06XV[P\x7F\x11\xC2ON\xAD\x16P|i\xACF\x7F\xBD^N\xED_\xB5\xC6\x99bm,\xC6\xD6d!\xDF%8\x86\xD5`@\x83\x80_\x80Q` a \xA2\x839\x81Q\x91RU\x81Q\x90_\x82R` \x82\x01R\xA1a\x06\xD0a\x1F?V[a\x06\xD6W\0[h\xFF\0\0\0\0\0\0\0\0\x19_\x80Q` a!\x02\x839\x81Q\x91RT\x16_\x80Q` a!\x02\x839\x81Q\x91RU\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2` `@Q`\x01\x81R\xA1\0[a\x076\x90a\x1B\xC1V[P\x84a\x06\x07V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01RmDelay too long`\x90\x1B`D\x82\x01R`d\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x11\x19[\x18^H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`\x8A\x1B`D\x82\x01R`d\x90\xFD[h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16h\x01\0\0\0\0\0\0\0\x01\x17_\x80Q` a!\x02\x839\x81Q\x91RU\x86a\x05\xD3V[c\xF9.\xE8\xA9`\xE0\x1B_R`\x04_\xFD[\x90P\x15\x88a\x05\xAAV[0;\x15\x91Pa\x05\xA2V[\x87\x91Pa\x05\x98V[4a\x01\xDEW`\xA06`\x03\x19\x01\x12a\x01\xDEWa\x08\x19a\x13\tV[Pa\x08\"a\x13\x1FV[P`D5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xDEWa\x08B\x906\x90`\x04\x01a\x15\x0FV[P`d5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xDEWa\x08b\x906\x90`\x04\x01a\x15\x0FV[P`\x845`\x01`\x01`@\x1B\x03\x81\x11a\x01\xDEWa\x08\x82\x906\x90`\x04\x01a\x14\x14V[P`@Qc\xBC\x19|\x81`\xE0\x1B\x81R` \x90\xF3[4a\x01\xDEW` 6`\x03\x19\x01\x12a\x01\xDEWa\x08\xAEa\x13\tV[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R_\x80Q` a \x02\x839\x81Q\x91R` \x90\x81R`@\x91\x82\x90 T\x91Q`\xFF\x90\x92\x16\x15\x15\x82R\x90\xF3[4a\x01\xDEW` a\t\x05a\x08\xF66a\x14\x8AV[\x96\x95\x90\x95\x94\x91\x94\x93\x92\x93a\x17\xF0V[`@Q\x90\x81R\xF3[4a\x01\xDEW_6`\x03\x19\x01\x12a\x01\xDEW` `@Q\x7F\xFDd<rq\x0Cc\xC0\x18\x02Y\xAB\xA6\xB2\xD0TQ\xE3Y\x1A$\xE5\x8Bb#\x93x\x08W&\xF7\x83\x81R\xF3[4a\x01\xDEW_6`\x03\x19\x01\x12a\x01\xDEW`@\x80Q\x90a\tf\x81\x83a\x13\xC4V[`\x05\x82R` \x82\x01\x91d\x03R\xE3\x02\xE3`\xDC\x1B\x83R\x81Q\x92\x83\x91` \x83RQ\x80\x91\x81` \x85\x01R\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x81\x01\x03\x01\x90\xF3[4a\x01\xDEW_6`\x03\x19\x01\x12a\x01\xDEW` `@Q_\x81R\xF3[4a\x01\xDEW_6`\x03\x19\x01\x12a\x01\xDEW` `@Qb\x01Q\x80\x81R\xF3[4a\x01\xDEW`@6`\x03\x19\x01\x12a\x01\xDEWa\t\xF5a\x13\x1FV[`\x045_R_\x80Q` a \x82\x839\x81Q\x91R` R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x01\xDEW_6`\x03\x19\x01\x12a\x01\xDEW` `@Q\x7F\xB0\x9A\xA5\xAE\xB3p,\xFDP\xB6\xB6+\xC4S&\x04\x93\x8F!$\x8A'\xA1\xD5\xCAs`\x82\xB6\x81\x9C\xC1\x81R\xF3[4a\x01\xDEW`\xC06`\x03\x19\x01\x12a\x01\xDEW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x01\xDEWa\n\x9B\x906\x90`\x04\x01a\x14ZV[\x90`$5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xDEWa\n\xBB\x906\x90`\x04\x01a\x14ZV[`D\x92\x91\x925`\x01`\x01`@\x1B\x03\x81\x11a\x01\xDEWa\n\xDD\x906\x90`\x04\x01a\x14ZV[\x93\x90\x91`d5`\x845\x95`\xA45\x92a\n\xF43a\x19\xBCV[\x80\x89\x14\x80\x15\x90a\x0B\xE6W[a\x0B\xCCWa\x0B\x13\x88\x84\x84\x89\x85\x8A\x8F\x8Ea\x17\xF0V[\x98a\x0B\x1E\x85\x8Ba\x191V[\x89_[\x82\x81\x10a\x0B^WP\x89\x80a\x0B1W\0[` \x7F \xFD\xA5\xFD'\xA1\xEA{\xF5\xB9V\x7F\x14:\xC5G\x0B\xB0Y7J'\xE8\xF6|\xB4O\x94om\x03\x87\x91`@Q\x90\x81R\xA2\0[\x80`\x01\x92\x7FL\xF4A\x0C\xC5p@\xE4Hb\xEF\x0FE\xF3\xDDZ^\x02\xDB\x8E\xB8\xAD\xD6H\xD4\xB0\xE26\xF1\xD0}\xCA\x8B\x8Ba\x0B\xC1\x8F\x8Ca\x0B\xB4\x8F\x92\x8Ea\x0B\xAD\x8F\x8F\x90a\x0B\xA7a\x03\t\x8F\x80\x97\x94\x81\x95a\x17wV[\x99a\x17wV[5\x97a\x17\xAFV[\x90`@Q\x96\x87\x96\x87a\x15\xF1V[\x03\x90\xA3\x01\x8A\x90a\x0B!V[\x90\x88c\xFF\xB02\x11`\xE0\x1B_R`\x04R`$R`DR`d_\xFD[P\x81\x89\x14\x15a\n\xFFV[4a\x01\xDEW` a\t\x05a\x0C\x036a\x13vV[\x94\x93\x90\x93\x92\x91\x92a\x17\"V[4a\x01\xDEW` 6`\x03\x19\x01\x12a\x01\xDEWa\x0C+`\x045a\x16\xDEV[`@Q`\x04\x82\x10\x15a\x0C>W` \x91\x81R\xF3[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[4a\x01\xDEW` 6`\x03\x19\x01\x12a\x01\xDEWa\x0Cka\x13\tV[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R_\x80Q` a B\x839\x81Q\x91R` \x90\x81R`@\x91\x82\x90 T\x91Q`\xFF\x90\x92\x16\x15\x15\x82R\x90\xF3[4a\x01\xDEW` 6`\x03\x19\x01\x12a\x01\xDEW`\x04503\x03a\r\nW\x7F\x11\xC2ON\xAD\x16P|i\xACF\x7F\xBD^N\xED_\xB5\xC6\x99bm,\xC6\xD6d!\xDF%8\x86\xD5`@_\x80Q` a \xA2\x839\x81Q\x91RT\x81Q\x90\x81R\x83` \x82\x01R\xA1_\x80Q` a \xA2\x839\x81Q\x91RU\0[c\xE2\x85\x0CY`\xE0\x1B_R3`\x04R`$_\xFD[4a\x01\xDEW` 6`\x03\x19\x01\x12a\x01\xDEW` a\r;`\x045a\x16\xB5V[`@Q\x90\x15\x15\x81R\xF3[4a\x01\xDEW_6`\x03\x19\x01\x12a\x01\xDEW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x160\x03a\r\x9CW` `@Q_\x80Q` a b\x839\x81Q\x91R\x81R\xF3[cp>F\xDD`\xE1\x1B_R`\x04_\xFD[`@6`\x03\x19\x01\x12a\x01\xDEWa\r\xBFa\x13\tV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xDEWa\r\xDE\x906\x90`\x04\x01a\x14\x14V[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x81\x14\x90\x81\x15a\x0F\x99W[Pa\r\x9CW03\x03a\x0FUW`@QcR\xD1\x90-`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x92\x90` \x81`\x04\x81\x87Z\xFA_\x91\x81a\x0F!W[Pa\x0EaW\x83cL\x9C\x8C\xE3`\xE0\x1B_R`\x04R`$_\xFD[\x80_\x80Q` a b\x839\x81Q\x91R\x85\x92\x03a\x0F\x0FWP\x81;\x15a\x0E\xFDW_\x80Q` a b\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x82\x17\x90U\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;_\x80\xA2\x81Q\x15a\x0E\xE5W_\x80\x83` a\0\x18\x95Q\x91\x01\x84Z\xF4a\x0E\xDFa\x1B<V[\x91a\x1F\x9CV[PP4a\x0E\xEEW\0[c\xB3\x98\x97\x9F`\xE0\x1B_R`\x04_\xFD[cL\x9C\x8C\xE3`\xE0\x1B_R`\x04R`$_\xFD[c*\x87Ri`\xE2\x1B_R`\x04R`$_\xFD[\x90\x91P` \x81=` \x11a\x0FMW[\x81a\x0F=` \x93\x83a\x13\xC4V[\x81\x01\x03\x12a\x01\xDEWQ\x90\x85a\x0EIV[=\x91Pa\x0F0V[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOnly self-upgrade via governance`D\x82\x01R\xFD[_\x80Q` a b\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x90P\x83a\x0E\x13V[4a\x01\xDEW_6`\x03\x19\x01\x12a\x01\xDEW` `@Qb'\x8D\0\x81R\xF3[4a\x01\xDEW`@6`\x03\x19\x01\x12a\x01\xDEWa\x0F\xF1a\x13\x1FV[3`\x01`\x01`\xA0\x1B\x03\x82\x16\x03a\x10\rWa\0\x18\x90`\x045a\x1E\x94V[c3K\xD9\x19`\xE1\x1B_R`\x04_\xFD[4a\x01\xDEW` 6`\x03\x19\x01\x12a\x01\xDEW` a\r;`\x045a\x16\x9EV[4a\x01\xDEW`@6`\x03\x19\x01\x12a\x01\xDEWa\0\x18`\x045a\x10Ya\x13\x1FV[\x90a\x10fa\x03\xEA\x82a\x16hV[a\x1E\x03V[4a\x01\xDEW` 6`\x03\x19\x01\x12a\x01\xDEW` a\r;`\x045a\x16\x86V[4a\x01\xDEW` 6`\x03\x19\x01\x12a\x01\xDEW` a\t\x05`\x045a\x16hV[4a\x01\xDEW`\x806`\x03\x19\x01\x12a\x01\xDEWa\x10\xC0a\x13\tV[Pa\x10\xC9a\x13\x1FV[P`d5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xDEWa\x10\xE9\x906\x90`\x04\x01a\x14\x14V[P`@Qc\n\x85\xBD\x01`\xE1\x1B\x81R` \x90\xF3[4a\x01\xDEW` 6`\x03\x19\x01\x12a\x01\xDEW` a\r;`\x045a\x16PV[a\0\x18a\x11\xA5_a\x11\xB1\x7F\xC2a~\xFAi\xBA\xB6g\x82\xFA!\x95CqC8H\x9CN\x9E\x17\x82qV\n\x91\xB8,?a+Xa\x11\x9Ca\x11Q6a\x13vV[_\x80Q` a \xC2\x839\x81Q\x91R\x8A\x99\x95\x97\x92\x99\x94\x93\x94R_\x80Q` a \x82\x839\x81Q\x91R` R`@\x8A \x8A\x80R` R`\xFF`@\x8B T\x16\x15a\x11\xB9W[\x88\x84\x84\x89\x89a\x17\"V[\x98\x89\x97\x88a\x1A\xE4V[a\x03(\x82\x82\x87\x87a\x1BkV[\x03\x90\xA3a\x1B\x97V[a\x11\xC23a\x1A/V[a\x11\x92V[4a\x01\xDEW_6`\x03\x19\x01\x12a\x01\xDEW` `@Q_\x80Q` a \xC2\x839\x81Q\x91R\x81R\xF3[4a\x01\xDEW` 6`\x03\x19\x01\x12a\x01\xDEW`\x045c\xFF\xFF\xFF\xFF`\xE0\x1B\x81\x16\x80\x91\x03a\x01\xDEW` \x90c\x02q\x18\x97`\xE5\x1B\x81\x14\x90\x81\x15a\x123W[P`@Q\x90\x15\x15\x81R\xF3[cye\xDB\x0B`\xE0\x1B\x81\x14\x91P\x81\x15a\x12MW[P\x82a\x12(V[c\x01\xFF\xC9\xA7`\xE0\x1B\x14\x90P\x82a\x12FV[4a\x01\xDEW`\xC06`\x03\x19\x01\x12a\x01\xDEWa\x12wa\x13\tV[`$5\x90`D5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xDEW\x7FL\xF4A\x0C\xC5p@\xE4Hb\xEF\x0FE\xF3\xDDZ^\x02\xDB\x8E\xB8\xAD\xD6H\xD4\xB0\xE26\xF1\xD0}\xCA\x92a\x12\xBD_\x926\x90`\x04\x01a\x13IV[\x94\x90\x91`d5\x94a\x12\xFF`\x845\x96`\xA45\x90a\x12\xD83a\x19\xBCV[a\x12\xE6\x89\x82\x8C\x8A\x89\x89a\x17\"V[\x99\x8A\x97a\x12\xF3\x84\x8Aa\x191V[`@Q\x96\x87\x96\x87a\x15\xF1V[\x03\x90\xA3\x80a\x0B1W\0[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01\xDEWV[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01\xDEWV[5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01\xDEWV[\x91\x81`\x1F\x84\x01\x12\x15a\x01\xDEW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x01\xDEW` \x83\x81\x86\x01\x95\x01\x01\x11a\x01\xDEWV[`\xA0`\x03\x19\x82\x01\x12a\x01\xDEW`\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x01\xDEW\x91`$5\x91`D5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x01\xDEWa\x13\xB8\x91`\x04\x01a\x13IV[\x90\x91`d5\x90`\x845\x90V[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x13\xE5W`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`\x01`\x01`@\x1B\x03\x81\x11a\x13\xE5W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x81`\x1F\x82\x01\x12\x15a\x01\xDEW\x805\x90a\x14+\x82a\x13\xF9V[\x92a\x149`@Q\x94\x85a\x13\xC4V[\x82\x84R` \x83\x83\x01\x01\x11a\x01\xDEW\x81_\x92` \x80\x93\x01\x83\x86\x017\x83\x01\x01R\x90V[\x91\x81`\x1F\x84\x01\x12\x15a\x01\xDEW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x01\xDEW` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\x01\xDEWV[`\xA0`\x03\x19\x82\x01\x12a\x01\xDEW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x01\xDEW\x81a\x14\xB4\x91`\x04\x01a\x14ZV[\x92\x90\x92\x91`$5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xDEW\x81a\x14\xD6\x91`\x04\x01a\x14ZV[\x92\x90\x92\x91`D5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x01\xDEWa\x13\xB8\x91`\x04\x01a\x14ZV[`\x01`\x01`@\x1B\x03\x81\x11a\x13\xE5W`\x05\x1B` \x01\x90V[\x90\x80`\x1F\x83\x01\x12\x15a\x01\xDEW\x815a\x15&\x81a\x14\xF8V[\x92a\x154`@Q\x94\x85a\x13\xC4V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x01\xDEW` \x01\x90[\x82\x82\x10a\x15\\WPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\x15OV[\x90\x80`\x1F\x83\x01\x12\x15a\x01\xDEW\x815a\x15\x83\x81a\x14\xF8V[\x92a\x15\x91`@Q\x94\x85a\x13\xC4V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x01\xDEW` \x01\x90[\x82\x82\x10a\x15\xB9WPPP\x90V[` \x80\x91a\x15\xC6\x84a\x135V[\x81R\x01\x91\x01\x90a\x15\xACV[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x92\x90\x93a\x16\x1F\x92`\x80\x95\x98\x97\x96\x98`\x01\x80`\xA0\x1B\x03\x16\x85R` \x85\x01R`\xA0`@\x85\x01R`\xA0\x84\x01\x91a\x15\xD1V[\x94``\x82\x01R\x01RV[a\x16M\x94\x92``\x92`\x01\x80`\xA0\x1B\x03\x16\x82R` \x82\x01R\x81`@\x82\x01R\x01\x91a\x15\xD1V[\x90V[a\x16Y\x90a\x16\xDEV[`\x04\x81\x10\x15a\x0C>W`\x02\x14\x90V[_R_\x80Q` a \x82\x839\x81Q\x91R` R`\x01`@_ \x01T\x90V[a\x16\x8F\x90a\x16\xDEV[`\x04\x81\x10\x15a\x0C>W`\x03\x14\x90V[a\x16\xA7\x90a\x16\xDEV[`\x04\x81\x10\x15a\x0C>W\x15\x15\x90V[a\x16\xBE\x90a\x16\xDEV[`\x04\x81\x10\x15a\x0C>W`\x01\x81\x14\x90\x81\x15a\x16\xD6WP\x90V[`\x02\x91P\x14\x90V[_R_\x80Q` a \"\x839\x81Q\x91R` R`@_ T\x80\x15_\x14a\x17\x03WP_\x90V[`\x01\x81\x03a\x17\x11WP`\x03\x90V[B\x10\x15a\x17\x1DW`\x01\x90V[`\x02\x90V[\x94a\x17Xa\x17q\x94\x95\x92\x93`@Q\x96\x87\x95` \x87\x01\x99`\x01\x80`\xA0\x1B\x03\x16\x8AR`@\x87\x01R`\xA0``\x87\x01R`\xC0\x86\x01\x91a\x15\xD1V[\x91`\x80\x84\x01R`\xA0\x83\x01R\x03`\x1F\x19\x81\x01\x83R\x82a\x13\xC4V[Q\x90 \x90V[\x91\x90\x81\x10\x15a\x17\x87W`\x05\x1B\x01\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x01\xDEW\x90V[\x91\x90\x81\x10\x15a\x17\x87W`\x05\x1B\x81\x015\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x01\xDEW\x01\x90\x815\x91`\x01`\x01`@\x1B\x03\x83\x11a\x01\xDEW` \x01\x826\x03\x81\x13a\x01\xDEW\x91\x90V[\x96\x93\x94\x91\x90\x96\x95\x92\x95`@Q\x96` \x88\x01\x98\x80`\xC0\x8A\x01`\xA0\x8CRR`\xE0\x89\x01\x92\x90_[\x81\x81\x10a\x19\x0BWPPP\x87\x82\x03`\x1F\x19\x01`@\x89\x01R\x80\x82R`\x01`\x01`\xFB\x1B\x03\x81\x11a\x01\xDEW\x90\x87\x95\x93\x94\x92\x91`\x05\x1B\x80\x92` \x83\x017\x01\x84\x81\x03``\x86\x01R` \x81\x01\x84\x90R`\x05\x84\x90\x1B\x81\x01`@\x90\x81\x01\x94\x90\x82\x01\x91_\x90\x88\x906\x82\x90\x03`\x1E\x19\x01\x90[\x84\x84\x10a\x18\xA5WPPPPPPa\x17q\x94P`\x80\x84\x01R`\xA0\x83\x01R\x03`\x1F\x19\x81\x01\x83R\x82a\x13\xC4V[\x91\x93\x95\x97\x90\x92\x94\x96\x98P`\x1F\x19`\x1F\x19\x83\x83\x03\x01\x01\x87R\x895\x83\x81\x12\x15a\x01\xDEW\x84\x01\x90` \x825\x92\x01\x91`\x01`\x01`@\x1B\x03\x81\x11a\x01\xDEW\x806\x03\x83\x13a\x01\xDEWa\x18\xF7` \x92\x83\x92`\x01\x95a\x15\xD1V[\x9B\x01\x97\x01\x94\x01\x91\x8A\x98\x96\x99\x97\x95\x93\x91a\x18{V[\x90\x91\x93` \x80`\x01\x92\x83\x80`\xA0\x1B\x03a\x19#\x89a\x135V[\x16\x81R\x01\x95\x01\x92\x91\x01a\x18\x14V[\x90a\x19;\x82a\x16\x9EV[a\x19\xA4W_\x80Q` a \xA2\x839\x81Q\x91RT\x80\x82\x10a\x19\x8EWPB\x01\x90\x81B\x11a\x19zW_R_\x80Q` a \"\x839\x81Q\x91R` R`@_ UV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x90cT3f\t`\xE0\x1B_R`\x04R`$R`D_\xFD[Pc^\xAD\x8E\xB5`\xE0\x1B_R`\x04R`\x01`$R`D_\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_\x80Q` a B\x839\x81Q\x91R` R`@\x90 T`\xFF\x16\x15a\x19\xEBWPV[c\xE2Q}?`\xE0\x1B_\x90\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x04R\x7F\xB0\x9A\xA5\xAE\xB3p,\xFDP\xB6\xB6+\xC4S&\x04\x93\x8F!$\x8A'\xA1\xD5\xCAs`\x82\xB6\x81\x9C\xC1`$R`D\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_\x80Q` a \xE2\x839\x81Q\x91R` R`@\x90 T`\xFF\x16\x15a\x1A^WPV[c\xE2Q}?`\xE0\x1B_\x90\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x04R_\x80Q` a \xC2\x839\x81Q\x91R`$R`D\x90\xFD[\x90\x81_R_\x80Q` a \x82\x839\x81Q\x91R` R`@_ `\x01\x80`\xA0\x1B\x03\x82\x16_R` R`\xFF`@_ T\x16\x15a\x1A\xC7WPPV[c\xE2Q}?`\xE0\x1B_R`\x01\x80`\xA0\x1B\x03\x16`\x04R`$R`D_\xFD[a\x1A\xED\x81a\x16PV[\x15a\x1B%WP\x80\x15\x15\x80a\x1B\x15W[a\x1B\x03WPV[c\x12\x154\xC3`\xE3\x1B_R`\x04R`$_\xFD[Pa\x1B\x1F\x81a\x16\x86V[\x15a\x1A\xFCV[c^\xAD\x8E\xB5`\xE0\x1B_R`\x04R`\x04`$R`D_\xFD[=\x15a\x1BfW=\x90a\x1BM\x82a\x13\xF9V[\x91a\x1B[`@Q\x93\x84a\x13\xC4V[\x82R=_` \x84\x01>V[``\x90V[a\x1B\x94\x93_\x93\x92\x84\x93\x82`@Q\x93\x84\x92\x837\x81\x01\x85\x81R\x03\x92Z\xF1a\x1B\x8Ea\x1B<V[\x90a\x1F0V[PV[a\x1B\xA0\x81a\x16PV[\x15a\x1B%W_R_\x80Q` a \"\x839\x81Q\x91R` R`\x01`@_ UV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R\x7F\xB7\xDB-\xD0\x8F\xCBb\xD0\xC9\xE0\x8CQ\x94\x1C\xAES\xC2gxj\x0Bu\x80?\xB7\x96\t\x02\xFC\x8E\xF9}` R`@\x90 T`\xFF\x16a\x1CXW`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R\x7F\xB7\xDB-\xD0\x8F\xCBb\xD0\xC9\xE0\x8CQ\x94\x1C\xAES\xC2gxj\x0Bu\x80?\xB7\x96\t\x02\xFC\x8E\xF9}` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90_\x80Q` a\x1F\xE2\x839\x81Q\x91R\x81\x80\xA4`\x01\x90V[P_\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_\x80Q` a B\x839\x81Q\x91R` R`@\x90 T`\xFF\x16a\x1CXW`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_\x80Q` a B\x839\x81Q\x91R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90\x7F\xB0\x9A\xA5\xAE\xB3p,\xFDP\xB6\xB6+\xC4S&\x04\x93\x8F!$\x8A'\xA1\xD5\xCAs`\x82\xB6\x81\x9C\xC1\x90_\x80Q` a\x1F\xE2\x839\x81Q\x91R\x90\x80\xA4`\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_\x80Q` a \x02\x839\x81Q\x91R` R`@\x90 T`\xFF\x16a\x1CXW`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_\x80Q` a \x02\x839\x81Q\x91R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90\x7F\xFDd<rq\x0Cc\xC0\x18\x02Y\xAB\xA6\xB2\xD0TQ\xE3Y\x1A$\xE5\x8Bb#\x93x\x08W&\xF7\x83\x90_\x80Q` a\x1F\xE2\x839\x81Q\x91R\x90\x80\xA4`\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_\x80Q` a \xE2\x839\x81Q\x91R` R`@\x90 T`\xFF\x16a\x1CXW`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_\x80Q` a \xE2\x839\x81Q\x91R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90_\x80Q` a \xC2\x839\x81Q\x91R\x90_\x80Q` a\x1F\xE2\x839\x81Q\x91R\x90\x80\xA4`\x01\x90V[_\x81\x81R_\x80Q` a \x82\x839\x81Q\x91R` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x84R\x90\x91R\x90 T`\xFF\x16a\x1E\x8EW_\x81\x81R_\x80Q` a \x82\x839\x81Q\x91R` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x95\x90\x95\x16\x80\x84R\x94\x90\x91R\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x92\x91\x90_\x80Q` a\x1F\xE2\x839\x81Q\x91R\x90\x80\xA4`\x01\x90V[PP_\x90V[_\x81\x81R_\x80Q` a \x82\x839\x81Q\x91R` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x84R\x90\x91R\x90 T`\xFF\x16\x15a\x1E\x8EW_\x81\x81R_\x80Q` a \x82\x839\x81Q\x91R` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x95\x90\x95\x16\x80\x84R\x94\x90\x91R\x81 \x80T`\xFF\x19\x16\x90U3\x92\x91\x90\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x90\x80\xA4`\x01\x90V[\x90\x91\x90a\x1F=WPa\x1F~V[V[`\xFF_\x80Q` a!\x02\x839\x81Q\x91RT`@\x1C\x16\x15a\x1F[WV[c\x1A\xFC\xD7\x9F`\xE3\x1B_R`\x04_\xFD[\x80Q\x82\x10\x15a\x17\x87W` \x91`\x05\x1B\x01\x01\x90V[\x80Q\x15a\x1F\x8DW\x80Q\x90` \x01\xFD[c\xD6\xBD\xA2u`\xE0\x1B_R`\x04_\xFD[\x90a\x1F\xA7WPa\x1F~V[\x81Q\x15\x80a\x1F\xD8W[a\x1F\xB8WP\x90V[c\x99\x96\xB3\x15`\xE0\x1B_\x90\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x04R`$\x90\xFD[P\x80;\x15a\x1F\xB0V\xFE/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r\xFAq\xE0\x7F$\xC4p\x1E\xF6Z\x97\x07u\x97\x9D\xE1),\xFE\x90\x935\xCD\x18\xA3-+{s\x98y\x14\x9A7\xC2\xAA\x9D\x18j\ti\xFF\x8A\x82g\xBFN\x07\xE8d\xC2\xF2v\x8FP@\x94\x9E(\xA6$\xFB6\0Z\x874\xC3K\x98\xD7\xC9n\xB2\xEA%\xF2\x98\x98\x94\x07\xE1\xF2]\xA1\x16\xEC\x13\x9B\xCC\xE0\x88{\xCB|\xF76\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x02\xDD{\xC7\xDE\xC4\xDC\xEE\xDD\xA7u\xE5\x8D\xD5A\xE0\x8A\x11llS\x81\\\x0B\xD0(\x19/{bh\0\x9A7\xC2\xAA\x9D\x18j\ti\xFF\x8A\x82g\xBFN\x07\xE8d\xC2\xF2v\x8FP@\x94\x9E(\xA6$\xFB6\x01\xD8\xAA\x0F1\x94\x97\x1A*\x11fy\xF7\xC2\t\x0Fi9\xC8\xD4\xE0\x1A*\x8D~A\xD5^SQF\x9EcR\xFC\xE5\xE8\xA5\xD0\xD9\xE8\xD1\xEA)\xF4R^Q.\x9C'\xBF\x92\xCA\xE5\x03t\xD4\x97\xF9\x18\xABH\xF3\x82\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\xA1dsolcC\0\x08\x1A\0\n",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080604052600436101561001a575b3615610018575f80fd5b005b5f3560e01c806301d5062a1461125e57806301ffc9a7146111ee57806307bd0265146111c7578063134008d31461111a57806313bc9f20146110fc578063150b7a02146110a7578063248a9ca3146110895780632ab0f5291461106b5780632f2ff15d1461103a57806331d507501461101c57806336568abe14610fd85780634125ff9014610fbb5780634f1ef28614610dab57806352d1902d14610d45578063584b153e14610d1d57806364d6235314610ca057806374ec29a014610c525780637958004c14610c0f5780638065657f14610bf05780638f2a0bb014610a6b5780638f61f4f514610a3157806391d14854146109dc5780639f81aed7146109bf578063a217fddf146109a5578063ad3cb1cc14610947578063b08e51c01461090d578063b1c5f427146108e3578063b426475e14610895578063bc197c8114610800578063c4c4c7b314610503578063c4d252f51461042c578063d45c4435146103f6578063d547741f146103be578063debfda3014610370578063e38335e514610237578063f23a6e61146101e25763f27a0c920361000e57346101de575f3660031901126101de5760205f805160206120a283398151915254604051908152f35b5f80fd5b346101de5760a03660031901126101de576101fb611309565b5061020461131f565b506084356001600160401b0381116101de57610224903690600401611414565b5060405163f23a6e6160e01b8152602090f35b6102403661148a565b5f80525f805160206120e28339815191526020527fdd1864c1ab258d549957f6a4d7e3a5005661df58241dbbc1700fb170ef0615465492979196919593949260ff1615610362575b828214801590610358575b61033d576102aa6102b191888a888789888d6117f0565b9687611ae4565b5f5b8181106102c35761001887611b97565b8080887fc2617efa69bab66782fa219543714338489c4e9e178271560a91b82c3f612b58888861033461031b8f986001998f828e61030e8f836103099161031496611777565b61179b565b97611777565b35956117af565b9061032882828787611b6b565b60405194859485611629565b0390a3016102b3565b50869063ffb0321160e01b5f5260045260245260445260645ffd5b5087821415610293565b61036b33611a2f565b610288565b346101de5760203660031901126101de57610389611309565b6001600160a01b03165f9081525f805160206120e2833981519152602090815260409182902054915160ff9092161515825290f35b346101de5760403660031901126101de576100186004356103dd61131f565b906103f16103ea82611668565b3390611a8f565b611e94565b346101de5760203660031901126101de576004355f525f80516020612022833981519152602052602060405f2054604051908152f35b346101de5760203660031901126101de57335f9081525f8051602061200283398151915260205260409020546004359060ff16156104cc5761046d816116b5565b156104b257805f525f805160206120228339815191526020525f60408120557fbaa1eb22f2a492ba1a5fea61b8df4d27c6c8b5f3971e63bb58fa14ff72eedb705f80a2005b635ead8eb560e01b5f52600452600460021760245260445ffd5b63e2517d3f60e01b5f52336004527ffd643c72710c63c0180259aba6b2d05451e3591a24e58b62239378085726f78360245260445ffd5b346101de5760803660031901126101de576004356024356001600160401b0381116101de5761053690369060040161156c565b906044356001600160401b0381116101de5761055690369060040161156c565b6064356001600160a01b03811692908381036101de575f80516020612102833981519152549360ff8560401c1615946001600160401b038116801590816107f8575b60011490816107ee575b1590816107e5575b506107d65767ffffffffffffffff1981166001175f8051602061210283398151915255856107aa575b506201518083106107735762278d00831161073d576105f0611f3f565b6105f8611f3f565b61060130611bc1565b5061072d575b505f5b8451811015610654576001906106326001600160a01b0361062b8389611f6a565b5116611c5d565b5061064d828060a01b036106468389611f6a565b5116611cf0565b500161060a565b50825f5b8351811015610687576001906106806001600160a01b036106798388611f6a565b5116611d83565b5001610658565b507f11c24f4ead16507c69ac467fbd5e4eed5fb5c699626d2cc6d66421df253886d5604083805f805160206120a2833981519152558151905f82526020820152a16106d0611f3f565b6106d657005b68ff0000000000000000195f8051602061210283398151915254165f80516020612102833981519152557fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2602060405160018152a1005b61073690611bc1565b5084610607565b60405162461bcd60e51b815260206004820152600e60248201526d44656c617920746f6f206c6f6e6760901b6044820152606490fd5b60405162461bcd60e51b815260206004820152600f60248201526e11195b185e481d1bdbc81cda1bdc9d608a1b6044820152606490fd5b68ffffffffffffffffff191668010000000000000001175f8051602061210283398151915255866105d3565b63f92ee8a960e01b5f5260045ffd5b905015886105aa565b303b1591506105a2565b879150610598565b346101de5760a03660031901126101de57610819611309565b5061082261131f565b506044356001600160401b0381116101de5761084290369060040161150f565b506064356001600160401b0381116101de5761086290369060040161150f565b506084356001600160401b0381116101de57610882903690600401611414565b5060405163bc197c8160e01b8152602090f35b346101de5760203660031901126101de576108ae611309565b6001600160a01b03165f9081525f80516020612002833981519152602090815260409182902054915160ff9092161515825290f35b346101de5760206109056108f63661148a565b969590959491949392936117f0565b604051908152f35b346101de575f3660031901126101de5760206040517ffd643c72710c63c0180259aba6b2d05451e3591a24e58b62239378085726f7838152f35b346101de575f3660031901126101de57604080519061096681836113c4565b600582526020820191640352e302e360dc1b83528151928391602083525180918160208501528484015e5f828201840152601f01601f19168101030190f35b346101de575f3660031901126101de5760206040515f8152f35b346101de575f3660031901126101de576020604051620151808152f35b346101de5760403660031901126101de576109f561131f565b6004355f525f8051602061208283398151915260205260405f209060018060a01b03165f52602052602060ff60405f2054166040519015158152f35b346101de575f3660031901126101de5760206040517fb09aa5aeb3702cfd50b6b62bc4532604938f21248a27a1d5ca736082b6819cc18152f35b346101de5760c03660031901126101de576004356001600160401b0381116101de57610a9b90369060040161145a565b906024356001600160401b0381116101de57610abb90369060040161145a565b6044929192356001600160401b0381116101de57610add90369060040161145a565b9390916064356084359560a43592610af4336119bc565b808914801590610be6575b610bcc57610b1388848489858a8f8e6117f0565b98610b1e858b611931565b895f5b828110610b5e57508980610b3157005b60207f20fda5fd27a1ea7bf5b9567f143ac5470bb059374a27e8f67cb44f946f6d038791604051908152a2005b806001927f4cf4410cc57040e44862ef0f45f3dd5a5e02db8eb8add648d4b0e236f1d07dca8b8b610bc18f8c610bb48f928e610bad8f8f90610ba76103098f8097948195611777565b99611777565b35976117af565b90604051968796876115f1565b0390a3018a90610b21565b908863ffb0321160e01b5f5260045260245260445260645ffd5b5081891415610aff565b346101de576020610905610c0336611376565b94939093929192611722565b346101de5760203660031901126101de57610c2b6004356116de565b6040516004821015610c3e576020918152f35b634e487b7160e01b5f52602160045260245ffd5b346101de5760203660031901126101de57610c6b611309565b6001600160a01b03165f9081525f80516020612042833981519152602090815260409182902054915160ff9092161515825290f35b346101de5760203660031901126101de57600435303303610d0a577f11c24f4ead16507c69ac467fbd5e4eed5fb5c699626d2cc6d66421df253886d560405f805160206120a2833981519152548151908152836020820152a15f805160206120a283398151915255005b63e2850c5960e01b5f523360045260245ffd5b346101de5760203660031901126101de576020610d3b6004356116b5565b6040519015158152f35b346101de575f3660031901126101de577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03163003610d9c5760206040515f805160206120628339815191528152f35b63703e46dd60e11b5f5260045ffd5b60403660031901126101de57610dbf611309565b6024356001600160401b0381116101de57610dde903690600401611414565b6001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016308114908115610f99575b50610d9c57303303610f55576040516352d1902d60e01b81526001600160a01b0383169290602081600481875afa5f9181610f21575b50610e615783634c9c8ce360e01b5f5260045260245ffd5b805f80516020612062833981519152859203610f0f5750813b15610efd575f8051602061206283398151915280546001600160a01b031916821790557fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b5f80a2815115610ee5575f8083602061001895519101845af4610edf611b3c565b91611f9c565b505034610eee57005b63b398979f60e01b5f5260045ffd5b634c9c8ce360e01b5f5260045260245ffd5b632a87526960e21b5f5260045260245ffd5b9091506020813d602011610f4d575b81610f3d602093836113c4565b810103126101de57519085610e49565b3d9150610f30565b606460405162461bcd60e51b815260206004820152602060248201527f4f6e6c792073656c662d757067726164652076696120676f7665726e616e63656044820152fd5b5f80516020612062833981519152546001600160a01b03161415905083610e13565b346101de575f3660031901126101de57602060405162278d008152f35b346101de5760403660031901126101de57610ff161131f565b336001600160a01b0382160361100d5761001890600435611e94565b63334bd91960e11b5f5260045ffd5b346101de5760203660031901126101de576020610d3b60043561169e565b346101de5760403660031901126101de5761001860043561105961131f565b906110666103ea82611668565b611e03565b346101de5760203660031901126101de576020610d3b600435611686565b346101de5760203660031901126101de576020610905600435611668565b346101de5760803660031901126101de576110c0611309565b506110c961131f565b506064356001600160401b0381116101de576110e9903690600401611414565b50604051630a85bd0160e11b8152602090f35b346101de5760203660031901126101de576020610d3b600435611650565b6100186111a55f6111b17fc2617efa69bab66782fa219543714338489c4e9e178271560a91b82c3f612b5861119c61115136611376565b5f805160206120c28339815191528a9995979299949394525f8051602061208283398151915260205260408a208a805260205260ff60408b205416156111b9575b8884848989611722565b98899788611ae4565b61032882828787611b6b565b0390a3611b97565b6111c233611a2f565b611192565b346101de575f3660031901126101de5760206040515f805160206120c28339815191528152f35b346101de5760203660031901126101de5760043563ffffffff60e01b81168091036101de57602090630271189760e51b8114908115611233575b506040519015158152f35b637965db0b60e01b81149150811561124d575b5082611228565b6301ffc9a760e01b14905082611246565b346101de5760c03660031901126101de57611277611309565b602435906044356001600160401b0381116101de577f4cf4410cc57040e44862ef0f45f3dd5a5e02db8eb8add648d4b0e236f1d07dca926112bd5f923690600401611349565b949091606435946112ff6084359660a435906112d8336119bc565b6112e689828c8a8989611722565b998a976112f3848a611931565b604051968796876115f1565b0390a380610b3157005b600435906001600160a01b03821682036101de57565b602435906001600160a01b03821682036101de57565b35906001600160a01b03821682036101de57565b9181601f840112156101de578235916001600160401b0383116101de57602083818601950101116101de57565b60a06003198201126101de576004356001600160a01b03811681036101de579160243591604435906001600160401b0382116101de576113b891600401611349565b90916064359060843590565b90601f801991011681019081106001600160401b038211176113e557604052565b634e487b7160e01b5f52604160045260245ffd5b6001600160401b0381116113e557601f01601f191660200190565b81601f820112156101de5780359061142b826113f9565b9261143960405194856113c4565b828452602083830101116101de57815f926020809301838601378301015290565b9181601f840112156101de578235916001600160401b0383116101de576020808501948460051b0101116101de57565b60a06003198201126101de576004356001600160401b0381116101de57816114b49160040161145a565b929092916024356001600160401b0381116101de57816114d69160040161145a565b92909291604435906001600160401b0382116101de576113b89160040161145a565b6001600160401b0381116113e55760051b60200190565b9080601f830112156101de578135611526816114f8565b9261153460405194856113c4565b81845260208085019260051b8201019283116101de57602001905b82821061155c5750505090565b813581526020918201910161154f565b9080601f830112156101de578135611583816114f8565b9261159160405194856113c4565b81845260208085019260051b8201019283116101de57602001905b8282106115b95750505090565b602080916115c684611335565b8152019101906115ac565b908060209392818452848401375f828201840152601f01601f1916010190565b92909361161f926080959897969860018060a01b03168552602085015260a0604085015260a08401916115d1565b9460608201520152565b61164d949260609260018060a01b03168252602082015281604082015201916115d1565b90565b611659906116de565b6004811015610c3e5760021490565b5f525f80516020612082833981519152602052600160405f20015490565b61168f906116de565b6004811015610c3e5760031490565b6116a7906116de565b6004811015610c3e57151590565b6116be906116de565b6004811015610c3e57600181149081156116d6575090565b600291501490565b5f525f8051602061202283398151915260205260405f205480155f1461170357505f90565b600181036117115750600390565b42101561171d57600190565b600290565b9461175861177194959293604051968795602087019960018060a01b03168a52604087015260a0606087015260c08601916115d1565b91608084015260a083015203601f1981018352826113c4565b51902090565b91908110156117875760051b0190565b634e487b7160e01b5f52603260045260245ffd5b356001600160a01b03811681036101de5790565b91908110156117875760051b81013590601e19813603018212156101de5701908135916001600160401b0383116101de5760200182360381136101de579190565b9693949190969592956040519660208801988060c08a0160a08c525260e0890192905f5b81811061190b57505050878203601f190160408901528082526001600160fb1b0381116101de579087959394929160051b8092602083013701848103606086015260208101849052600584901b8101604090810194908201915f90889036829003601e1901905b8484106118a5575050505050506117719450608084015260a083015203601f1981018352826113c4565b91939597909294969850601f19601f19838303010187528935838112156101de57840190602082359201916001600160401b0381116101de5780360383136101de576118f760209283926001956115d1565b9b0197019401918a9896999795939161187b565b909193602080600192838060a01b0361192389611335565b168152019501929101611814565b9061193b8261169e565b6119a4575f805160206120a28339815191525480821061198e575042019081421161197a575f525f8051602061202283398151915260205260405f2055565b634e487b7160e01b5f52601160045260245ffd5b90635433660960e01b5f5260045260245260445ffd5b50635ead8eb560e01b5f52600452600160245260445ffd5b6001600160a01b0381165f9081525f80516020612042833981519152602052604090205460ff16156119eb5750565b63e2517d3f60e01b5f9081526001600160a01b03919091166004527fb09aa5aeb3702cfd50b6b62bc4532604938f21248a27a1d5ca736082b6819cc1602452604490fd5b6001600160a01b0381165f9081525f805160206120e2833981519152602052604090205460ff1615611a5e5750565b63e2517d3f60e01b5f9081526001600160a01b03919091166004525f805160206120c2833981519152602452604490fd5b90815f525f8051602061208283398151915260205260405f2060018060a01b0382165f5260205260ff60405f20541615611ac7575050565b63e2517d3f60e01b5f5260018060a01b031660045260245260445ffd5b611aed81611650565b15611b25575080151580611b15575b611b035750565b63121534c360e31b5f5260045260245ffd5b50611b1f81611686565b15611afc565b635ead8eb560e01b5f52600452600460245260445ffd5b3d15611b66573d90611b4d826113f9565b91611b5b60405193846113c4565b82523d5f602084013e565b606090565b611b94935f93928493826040519384928337810185815203925af1611b8e611b3c565b90611f30565b50565b611ba081611650565b15611b25575f525f80516020612022833981519152602052600160405f2055565b6001600160a01b0381165f9081527fb7db2dd08fcb62d0c9e08c51941cae53c267786a0b75803fb7960902fc8ef97d602052604090205460ff16611c58576001600160a01b03165f8181527fb7db2dd08fcb62d0c9e08c51941cae53c267786a0b75803fb7960902fc8ef97d60205260408120805460ff191660011790553391905f80516020611fe28339815191528180a4600190565b505f90565b6001600160a01b0381165f9081525f80516020612042833981519152602052604090205460ff16611c58576001600160a01b03165f8181525f8051602061204283398151915260205260408120805460ff191660011790553391907fb09aa5aeb3702cfd50b6b62bc4532604938f21248a27a1d5ca736082b6819cc1905f80516020611fe28339815191529080a4600190565b6001600160a01b0381165f9081525f80516020612002833981519152602052604090205460ff16611c58576001600160a01b03165f8181525f8051602061200283398151915260205260408120805460ff191660011790553391907ffd643c72710c63c0180259aba6b2d05451e3591a24e58b62239378085726f783905f80516020611fe28339815191529080a4600190565b6001600160a01b0381165f9081525f805160206120e2833981519152602052604090205460ff16611c58576001600160a01b03165f8181525f805160206120e283398151915260205260408120805460ff191660011790553391905f805160206120c2833981519152905f80516020611fe28339815191529080a4600190565b5f8181525f80516020612082833981519152602090815260408083206001600160a01b038616845290915290205460ff16611e8e575f8181525f80516020612082833981519152602090815260408083206001600160a01b0395909516808452949091528120805460ff19166001179055339291905f80516020611fe28339815191529080a4600190565b50505f90565b5f8181525f80516020612082833981519152602090815260408083206001600160a01b038616845290915290205460ff1615611e8e575f8181525f80516020612082833981519152602090815260408083206001600160a01b0395909516808452949091528120805460ff19169055339291907ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b9080a4600190565b909190611f3d5750611f7e565b565b60ff5f805160206121028339815191525460401c1615611f5b57565b631afcd79f60e31b5f5260045ffd5b80518210156117875760209160051b010190565b805115611f8d57805190602001fd5b63d6bda27560e01b5f5260045ffd5b90611fa75750611f7e565b81511580611fd8575b611fb8575090565b639996b31560e01b5f9081526001600160a01b0391909116600452602490fd5b50803b15611fb056fe2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0dfa71e07f24c4701ef65a970775979de1292cfe909335cd18a32d2b7b739879149a37c2aa9d186a0969ff8a8267bf4e07e864c2f2768f5040949e28a624fb36005a8734c34b98d7c96eb2ea25f298989407e1f25da116ec139bcce0887bcb7cf7360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc02dd7bc7dec4dceedda775e58dd541e08a116c6c53815c0bd028192f7b6268009a37c2aa9d186a0969ff8a8267bf4e07e864c2f2768f5040949e28a624fb3601d8aa0f3194971a2a116679f7c2090f6939c8d4e01a2a8d7e41d55e5351469e6352fce5e8a5d0d9e8d1ea29f4525e512e9c27bf92cae50374d497f918ab48f382f0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00a164736f6c634300081a000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x1AW[6\x15a\0\x18W_\x80\xFD[\0[_5`\xE0\x1C\x80c\x01\xD5\x06*\x14a\x12^W\x80c\x01\xFF\xC9\xA7\x14a\x11\xEEW\x80c\x07\xBD\x02e\x14a\x11\xC7W\x80c\x13@\x08\xD3\x14a\x11\x1AW\x80c\x13\xBC\x9F \x14a\x10\xFCW\x80c\x15\x0Bz\x02\x14a\x10\xA7W\x80c$\x8A\x9C\xA3\x14a\x10\x89W\x80c*\xB0\xF5)\x14a\x10kW\x80c//\xF1]\x14a\x10:W\x80c1\xD5\x07P\x14a\x10\x1CW\x80c6V\x8A\xBE\x14a\x0F\xD8W\x80cA%\xFF\x90\x14a\x0F\xBBW\x80cO\x1E\xF2\x86\x14a\r\xABW\x80cR\xD1\x90-\x14a\rEW\x80cXK\x15>\x14a\r\x1DW\x80cd\xD6#S\x14a\x0C\xA0W\x80ct\xEC)\xA0\x14a\x0CRW\x80cyX\0L\x14a\x0C\x0FW\x80c\x80ee\x7F\x14a\x0B\xF0W\x80c\x8F*\x0B\xB0\x14a\nkW\x80c\x8Fa\xF4\xF5\x14a\n1W\x80c\x91\xD1HT\x14a\t\xDCW\x80c\x9F\x81\xAE\xD7\x14a\t\xBFW\x80c\xA2\x17\xFD\xDF\x14a\t\xA5W\x80c\xAD<\xB1\xCC\x14a\tGW\x80c\xB0\x8EQ\xC0\x14a\t\rW\x80c\xB1\xC5\xF4'\x14a\x08\xE3W\x80c\xB4&G^\x14a\x08\x95W\x80c\xBC\x19|\x81\x14a\x08\0W\x80c\xC4\xC4\xC7\xB3\x14a\x05\x03W\x80c\xC4\xD2R\xF5\x14a\x04,W\x80c\xD4\\D5\x14a\x03\xF6W\x80c\xD5Gt\x1F\x14a\x03\xBEW\x80c\xDE\xBF\xDA0\x14a\x03pW\x80c\xE3\x835\xE5\x14a\x027W\x80c\xF2:na\x14a\x01\xE2Wc\xF2z\x0C\x92\x03a\0\x0EW4a\x01\xDEW_6`\x03\x19\x01\x12a\x01\xDEW` _\x80Q` a \xA2\x839\x81Q\x91RT`@Q\x90\x81R\xF3[_\x80\xFD[4a\x01\xDEW`\xA06`\x03\x19\x01\x12a\x01\xDEWa\x01\xFBa\x13\tV[Pa\x02\x04a\x13\x1FV[P`\x845`\x01`\x01`@\x1B\x03\x81\x11a\x01\xDEWa\x02$\x906\x90`\x04\x01a\x14\x14V[P`@Qc\xF2:na`\xE0\x1B\x81R` \x90\xF3[a\x02@6a\x14\x8AV[_\x80R_\x80Q` a \xE2\x839\x81Q\x91R` R\x7F\xDD\x18d\xC1\xAB%\x8DT\x99W\xF6\xA4\xD7\xE3\xA5\0Va\xDFX$\x1D\xBB\xC1p\x0F\xB1p\xEF\x06\x15FT\x92\x97\x91\x96\x91\x95\x93\x94\x92`\xFF\x16\x15a\x03bW[\x82\x82\x14\x80\x15\x90a\x03XW[a\x03=Wa\x02\xAAa\x02\xB1\x91\x88\x8A\x88\x87\x89\x88\x8Da\x17\xF0V[\x96\x87a\x1A\xE4V[_[\x81\x81\x10a\x02\xC3Wa\0\x18\x87a\x1B\x97V[\x80\x80\x88\x7F\xC2a~\xFAi\xBA\xB6g\x82\xFA!\x95CqC8H\x9CN\x9E\x17\x82qV\n\x91\xB8,?a+X\x88\x88a\x034a\x03\x1B\x8F\x98`\x01\x99\x8F\x82\x8Ea\x03\x0E\x8F\x83a\x03\t\x91a\x03\x14\x96a\x17wV[a\x17\x9BV[\x97a\x17wV[5\x95a\x17\xAFV[\x90a\x03(\x82\x82\x87\x87a\x1BkV[`@Q\x94\x85\x94\x85a\x16)V[\x03\x90\xA3\x01a\x02\xB3V[P\x86\x90c\xFF\xB02\x11`\xE0\x1B_R`\x04R`$R`DR`d_\xFD[P\x87\x82\x14\x15a\x02\x93V[a\x03k3a\x1A/V[a\x02\x88V[4a\x01\xDEW` 6`\x03\x19\x01\x12a\x01\xDEWa\x03\x89a\x13\tV[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R_\x80Q` a \xE2\x839\x81Q\x91R` \x90\x81R`@\x91\x82\x90 T\x91Q`\xFF\x90\x92\x16\x15\x15\x82R\x90\xF3[4a\x01\xDEW`@6`\x03\x19\x01\x12a\x01\xDEWa\0\x18`\x045a\x03\xDDa\x13\x1FV[\x90a\x03\xF1a\x03\xEA\x82a\x16hV[3\x90a\x1A\x8FV[a\x1E\x94V[4a\x01\xDEW` 6`\x03\x19\x01\x12a\x01\xDEW`\x045_R_\x80Q` a \"\x839\x81Q\x91R` R` `@_ T`@Q\x90\x81R\xF3[4a\x01\xDEW` 6`\x03\x19\x01\x12a\x01\xDEW3_\x90\x81R_\x80Q` a \x02\x839\x81Q\x91R` R`@\x90 T`\x045\x90`\xFF\x16\x15a\x04\xCCWa\x04m\x81a\x16\xB5V[\x15a\x04\xB2W\x80_R_\x80Q` a \"\x839\x81Q\x91R` R_`@\x81 U\x7F\xBA\xA1\xEB\"\xF2\xA4\x92\xBA\x1A_\xEAa\xB8\xDFM'\xC6\xC8\xB5\xF3\x97\x1Ec\xBBX\xFA\x14\xFFr\xEE\xDBp_\x80\xA2\0[c^\xAD\x8E\xB5`\xE0\x1B_R`\x04R`\x04`\x02\x17`$R`D_\xFD[c\xE2Q}?`\xE0\x1B_R3`\x04R\x7F\xFDd<rq\x0Cc\xC0\x18\x02Y\xAB\xA6\xB2\xD0TQ\xE3Y\x1A$\xE5\x8Bb#\x93x\x08W&\xF7\x83`$R`D_\xFD[4a\x01\xDEW`\x806`\x03\x19\x01\x12a\x01\xDEW`\x045`$5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xDEWa\x056\x906\x90`\x04\x01a\x15lV[\x90`D5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xDEWa\x05V\x906\x90`\x04\x01a\x15lV[`d5`\x01`\x01`\xA0\x1B\x03\x81\x16\x92\x90\x83\x81\x03a\x01\xDEW_\x80Q` a!\x02\x839\x81Q\x91RT\x93`\xFF\x85`@\x1C\x16\x15\x94`\x01`\x01`@\x1B\x03\x81\x16\x80\x15\x90\x81a\x07\xF8W[`\x01\x14\x90\x81a\x07\xEEW[\x15\x90\x81a\x07\xE5W[Pa\x07\xD6Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16`\x01\x17_\x80Q` a!\x02\x839\x81Q\x91RU\x85a\x07\xAAW[Pb\x01Q\x80\x83\x10a\x07sWb'\x8D\0\x83\x11a\x07=Wa\x05\xF0a\x1F?V[a\x05\xF8a\x1F?V[a\x06\x010a\x1B\xC1V[Pa\x07-W[P_[\x84Q\x81\x10\x15a\x06TW`\x01\x90a\x062`\x01`\x01`\xA0\x1B\x03a\x06+\x83\x89a\x1FjV[Q\x16a\x1C]V[Pa\x06M\x82\x80`\xA0\x1B\x03a\x06F\x83\x89a\x1FjV[Q\x16a\x1C\xF0V[P\x01a\x06\nV[P\x82_[\x83Q\x81\x10\x15a\x06\x87W`\x01\x90a\x06\x80`\x01`\x01`\xA0\x1B\x03a\x06y\x83\x88a\x1FjV[Q\x16a\x1D\x83V[P\x01a\x06XV[P\x7F\x11\xC2ON\xAD\x16P|i\xACF\x7F\xBD^N\xED_\xB5\xC6\x99bm,\xC6\xD6d!\xDF%8\x86\xD5`@\x83\x80_\x80Q` a \xA2\x839\x81Q\x91RU\x81Q\x90_\x82R` \x82\x01R\xA1a\x06\xD0a\x1F?V[a\x06\xD6W\0[h\xFF\0\0\0\0\0\0\0\0\x19_\x80Q` a!\x02\x839\x81Q\x91RT\x16_\x80Q` a!\x02\x839\x81Q\x91RU\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2` `@Q`\x01\x81R\xA1\0[a\x076\x90a\x1B\xC1V[P\x84a\x06\x07V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01RmDelay too long`\x90\x1B`D\x82\x01R`d\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x11\x19[\x18^H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`\x8A\x1B`D\x82\x01R`d\x90\xFD[h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16h\x01\0\0\0\0\0\0\0\x01\x17_\x80Q` a!\x02\x839\x81Q\x91RU\x86a\x05\xD3V[c\xF9.\xE8\xA9`\xE0\x1B_R`\x04_\xFD[\x90P\x15\x88a\x05\xAAV[0;\x15\x91Pa\x05\xA2V[\x87\x91Pa\x05\x98V[4a\x01\xDEW`\xA06`\x03\x19\x01\x12a\x01\xDEWa\x08\x19a\x13\tV[Pa\x08\"a\x13\x1FV[P`D5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xDEWa\x08B\x906\x90`\x04\x01a\x15\x0FV[P`d5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xDEWa\x08b\x906\x90`\x04\x01a\x15\x0FV[P`\x845`\x01`\x01`@\x1B\x03\x81\x11a\x01\xDEWa\x08\x82\x906\x90`\x04\x01a\x14\x14V[P`@Qc\xBC\x19|\x81`\xE0\x1B\x81R` \x90\xF3[4a\x01\xDEW` 6`\x03\x19\x01\x12a\x01\xDEWa\x08\xAEa\x13\tV[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R_\x80Q` a \x02\x839\x81Q\x91R` \x90\x81R`@\x91\x82\x90 T\x91Q`\xFF\x90\x92\x16\x15\x15\x82R\x90\xF3[4a\x01\xDEW` a\t\x05a\x08\xF66a\x14\x8AV[\x96\x95\x90\x95\x94\x91\x94\x93\x92\x93a\x17\xF0V[`@Q\x90\x81R\xF3[4a\x01\xDEW_6`\x03\x19\x01\x12a\x01\xDEW` `@Q\x7F\xFDd<rq\x0Cc\xC0\x18\x02Y\xAB\xA6\xB2\xD0TQ\xE3Y\x1A$\xE5\x8Bb#\x93x\x08W&\xF7\x83\x81R\xF3[4a\x01\xDEW_6`\x03\x19\x01\x12a\x01\xDEW`@\x80Q\x90a\tf\x81\x83a\x13\xC4V[`\x05\x82R` \x82\x01\x91d\x03R\xE3\x02\xE3`\xDC\x1B\x83R\x81Q\x92\x83\x91` \x83RQ\x80\x91\x81` \x85\x01R\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x81\x01\x03\x01\x90\xF3[4a\x01\xDEW_6`\x03\x19\x01\x12a\x01\xDEW` `@Q_\x81R\xF3[4a\x01\xDEW_6`\x03\x19\x01\x12a\x01\xDEW` `@Qb\x01Q\x80\x81R\xF3[4a\x01\xDEW`@6`\x03\x19\x01\x12a\x01\xDEWa\t\xF5a\x13\x1FV[`\x045_R_\x80Q` a \x82\x839\x81Q\x91R` R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x01\xDEW_6`\x03\x19\x01\x12a\x01\xDEW` `@Q\x7F\xB0\x9A\xA5\xAE\xB3p,\xFDP\xB6\xB6+\xC4S&\x04\x93\x8F!$\x8A'\xA1\xD5\xCAs`\x82\xB6\x81\x9C\xC1\x81R\xF3[4a\x01\xDEW`\xC06`\x03\x19\x01\x12a\x01\xDEW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x01\xDEWa\n\x9B\x906\x90`\x04\x01a\x14ZV[\x90`$5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xDEWa\n\xBB\x906\x90`\x04\x01a\x14ZV[`D\x92\x91\x925`\x01`\x01`@\x1B\x03\x81\x11a\x01\xDEWa\n\xDD\x906\x90`\x04\x01a\x14ZV[\x93\x90\x91`d5`\x845\x95`\xA45\x92a\n\xF43a\x19\xBCV[\x80\x89\x14\x80\x15\x90a\x0B\xE6W[a\x0B\xCCWa\x0B\x13\x88\x84\x84\x89\x85\x8A\x8F\x8Ea\x17\xF0V[\x98a\x0B\x1E\x85\x8Ba\x191V[\x89_[\x82\x81\x10a\x0B^WP\x89\x80a\x0B1W\0[` \x7F \xFD\xA5\xFD'\xA1\xEA{\xF5\xB9V\x7F\x14:\xC5G\x0B\xB0Y7J'\xE8\xF6|\xB4O\x94om\x03\x87\x91`@Q\x90\x81R\xA2\0[\x80`\x01\x92\x7FL\xF4A\x0C\xC5p@\xE4Hb\xEF\x0FE\xF3\xDDZ^\x02\xDB\x8E\xB8\xAD\xD6H\xD4\xB0\xE26\xF1\xD0}\xCA\x8B\x8Ba\x0B\xC1\x8F\x8Ca\x0B\xB4\x8F\x92\x8Ea\x0B\xAD\x8F\x8F\x90a\x0B\xA7a\x03\t\x8F\x80\x97\x94\x81\x95a\x17wV[\x99a\x17wV[5\x97a\x17\xAFV[\x90`@Q\x96\x87\x96\x87a\x15\xF1V[\x03\x90\xA3\x01\x8A\x90a\x0B!V[\x90\x88c\xFF\xB02\x11`\xE0\x1B_R`\x04R`$R`DR`d_\xFD[P\x81\x89\x14\x15a\n\xFFV[4a\x01\xDEW` a\t\x05a\x0C\x036a\x13vV[\x94\x93\x90\x93\x92\x91\x92a\x17\"V[4a\x01\xDEW` 6`\x03\x19\x01\x12a\x01\xDEWa\x0C+`\x045a\x16\xDEV[`@Q`\x04\x82\x10\x15a\x0C>W` \x91\x81R\xF3[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[4a\x01\xDEW` 6`\x03\x19\x01\x12a\x01\xDEWa\x0Cka\x13\tV[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R_\x80Q` a B\x839\x81Q\x91R` \x90\x81R`@\x91\x82\x90 T\x91Q`\xFF\x90\x92\x16\x15\x15\x82R\x90\xF3[4a\x01\xDEW` 6`\x03\x19\x01\x12a\x01\xDEW`\x04503\x03a\r\nW\x7F\x11\xC2ON\xAD\x16P|i\xACF\x7F\xBD^N\xED_\xB5\xC6\x99bm,\xC6\xD6d!\xDF%8\x86\xD5`@_\x80Q` a \xA2\x839\x81Q\x91RT\x81Q\x90\x81R\x83` \x82\x01R\xA1_\x80Q` a \xA2\x839\x81Q\x91RU\0[c\xE2\x85\x0CY`\xE0\x1B_R3`\x04R`$_\xFD[4a\x01\xDEW` 6`\x03\x19\x01\x12a\x01\xDEW` a\r;`\x045a\x16\xB5V[`@Q\x90\x15\x15\x81R\xF3[4a\x01\xDEW_6`\x03\x19\x01\x12a\x01\xDEW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x160\x03a\r\x9CW` `@Q_\x80Q` a b\x839\x81Q\x91R\x81R\xF3[cp>F\xDD`\xE1\x1B_R`\x04_\xFD[`@6`\x03\x19\x01\x12a\x01\xDEWa\r\xBFa\x13\tV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xDEWa\r\xDE\x906\x90`\x04\x01a\x14\x14V[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x81\x14\x90\x81\x15a\x0F\x99W[Pa\r\x9CW03\x03a\x0FUW`@QcR\xD1\x90-`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x92\x90` \x81`\x04\x81\x87Z\xFA_\x91\x81a\x0F!W[Pa\x0EaW\x83cL\x9C\x8C\xE3`\xE0\x1B_R`\x04R`$_\xFD[\x80_\x80Q` a b\x839\x81Q\x91R\x85\x92\x03a\x0F\x0FWP\x81;\x15a\x0E\xFDW_\x80Q` a b\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x82\x17\x90U\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;_\x80\xA2\x81Q\x15a\x0E\xE5W_\x80\x83` a\0\x18\x95Q\x91\x01\x84Z\xF4a\x0E\xDFa\x1B<V[\x91a\x1F\x9CV[PP4a\x0E\xEEW\0[c\xB3\x98\x97\x9F`\xE0\x1B_R`\x04_\xFD[cL\x9C\x8C\xE3`\xE0\x1B_R`\x04R`$_\xFD[c*\x87Ri`\xE2\x1B_R`\x04R`$_\xFD[\x90\x91P` \x81=` \x11a\x0FMW[\x81a\x0F=` \x93\x83a\x13\xC4V[\x81\x01\x03\x12a\x01\xDEWQ\x90\x85a\x0EIV[=\x91Pa\x0F0V[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOnly self-upgrade via governance`D\x82\x01R\xFD[_\x80Q` a b\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x90P\x83a\x0E\x13V[4a\x01\xDEW_6`\x03\x19\x01\x12a\x01\xDEW` `@Qb'\x8D\0\x81R\xF3[4a\x01\xDEW`@6`\x03\x19\x01\x12a\x01\xDEWa\x0F\xF1a\x13\x1FV[3`\x01`\x01`\xA0\x1B\x03\x82\x16\x03a\x10\rWa\0\x18\x90`\x045a\x1E\x94V[c3K\xD9\x19`\xE1\x1B_R`\x04_\xFD[4a\x01\xDEW` 6`\x03\x19\x01\x12a\x01\xDEW` a\r;`\x045a\x16\x9EV[4a\x01\xDEW`@6`\x03\x19\x01\x12a\x01\xDEWa\0\x18`\x045a\x10Ya\x13\x1FV[\x90a\x10fa\x03\xEA\x82a\x16hV[a\x1E\x03V[4a\x01\xDEW` 6`\x03\x19\x01\x12a\x01\xDEW` a\r;`\x045a\x16\x86V[4a\x01\xDEW` 6`\x03\x19\x01\x12a\x01\xDEW` a\t\x05`\x045a\x16hV[4a\x01\xDEW`\x806`\x03\x19\x01\x12a\x01\xDEWa\x10\xC0a\x13\tV[Pa\x10\xC9a\x13\x1FV[P`d5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xDEWa\x10\xE9\x906\x90`\x04\x01a\x14\x14V[P`@Qc\n\x85\xBD\x01`\xE1\x1B\x81R` \x90\xF3[4a\x01\xDEW` 6`\x03\x19\x01\x12a\x01\xDEW` a\r;`\x045a\x16PV[a\0\x18a\x11\xA5_a\x11\xB1\x7F\xC2a~\xFAi\xBA\xB6g\x82\xFA!\x95CqC8H\x9CN\x9E\x17\x82qV\n\x91\xB8,?a+Xa\x11\x9Ca\x11Q6a\x13vV[_\x80Q` a \xC2\x839\x81Q\x91R\x8A\x99\x95\x97\x92\x99\x94\x93\x94R_\x80Q` a \x82\x839\x81Q\x91R` R`@\x8A \x8A\x80R` R`\xFF`@\x8B T\x16\x15a\x11\xB9W[\x88\x84\x84\x89\x89a\x17\"V[\x98\x89\x97\x88a\x1A\xE4V[a\x03(\x82\x82\x87\x87a\x1BkV[\x03\x90\xA3a\x1B\x97V[a\x11\xC23a\x1A/V[a\x11\x92V[4a\x01\xDEW_6`\x03\x19\x01\x12a\x01\xDEW` `@Q_\x80Q` a \xC2\x839\x81Q\x91R\x81R\xF3[4a\x01\xDEW` 6`\x03\x19\x01\x12a\x01\xDEW`\x045c\xFF\xFF\xFF\xFF`\xE0\x1B\x81\x16\x80\x91\x03a\x01\xDEW` \x90c\x02q\x18\x97`\xE5\x1B\x81\x14\x90\x81\x15a\x123W[P`@Q\x90\x15\x15\x81R\xF3[cye\xDB\x0B`\xE0\x1B\x81\x14\x91P\x81\x15a\x12MW[P\x82a\x12(V[c\x01\xFF\xC9\xA7`\xE0\x1B\x14\x90P\x82a\x12FV[4a\x01\xDEW`\xC06`\x03\x19\x01\x12a\x01\xDEWa\x12wa\x13\tV[`$5\x90`D5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xDEW\x7FL\xF4A\x0C\xC5p@\xE4Hb\xEF\x0FE\xF3\xDDZ^\x02\xDB\x8E\xB8\xAD\xD6H\xD4\xB0\xE26\xF1\xD0}\xCA\x92a\x12\xBD_\x926\x90`\x04\x01a\x13IV[\x94\x90\x91`d5\x94a\x12\xFF`\x845\x96`\xA45\x90a\x12\xD83a\x19\xBCV[a\x12\xE6\x89\x82\x8C\x8A\x89\x89a\x17\"V[\x99\x8A\x97a\x12\xF3\x84\x8Aa\x191V[`@Q\x96\x87\x96\x87a\x15\xF1V[\x03\x90\xA3\x80a\x0B1W\0[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01\xDEWV[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01\xDEWV[5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01\xDEWV[\x91\x81`\x1F\x84\x01\x12\x15a\x01\xDEW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x01\xDEW` \x83\x81\x86\x01\x95\x01\x01\x11a\x01\xDEWV[`\xA0`\x03\x19\x82\x01\x12a\x01\xDEW`\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x01\xDEW\x91`$5\x91`D5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x01\xDEWa\x13\xB8\x91`\x04\x01a\x13IV[\x90\x91`d5\x90`\x845\x90V[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x13\xE5W`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`\x01`\x01`@\x1B\x03\x81\x11a\x13\xE5W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x81`\x1F\x82\x01\x12\x15a\x01\xDEW\x805\x90a\x14+\x82a\x13\xF9V[\x92a\x149`@Q\x94\x85a\x13\xC4V[\x82\x84R` \x83\x83\x01\x01\x11a\x01\xDEW\x81_\x92` \x80\x93\x01\x83\x86\x017\x83\x01\x01R\x90V[\x91\x81`\x1F\x84\x01\x12\x15a\x01\xDEW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x01\xDEW` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\x01\xDEWV[`\xA0`\x03\x19\x82\x01\x12a\x01\xDEW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x01\xDEW\x81a\x14\xB4\x91`\x04\x01a\x14ZV[\x92\x90\x92\x91`$5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xDEW\x81a\x14\xD6\x91`\x04\x01a\x14ZV[\x92\x90\x92\x91`D5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x01\xDEWa\x13\xB8\x91`\x04\x01a\x14ZV[`\x01`\x01`@\x1B\x03\x81\x11a\x13\xE5W`\x05\x1B` \x01\x90V[\x90\x80`\x1F\x83\x01\x12\x15a\x01\xDEW\x815a\x15&\x81a\x14\xF8V[\x92a\x154`@Q\x94\x85a\x13\xC4V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x01\xDEW` \x01\x90[\x82\x82\x10a\x15\\WPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\x15OV[\x90\x80`\x1F\x83\x01\x12\x15a\x01\xDEW\x815a\x15\x83\x81a\x14\xF8V[\x92a\x15\x91`@Q\x94\x85a\x13\xC4V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x01\xDEW` \x01\x90[\x82\x82\x10a\x15\xB9WPPP\x90V[` \x80\x91a\x15\xC6\x84a\x135V[\x81R\x01\x91\x01\x90a\x15\xACV[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x92\x90\x93a\x16\x1F\x92`\x80\x95\x98\x97\x96\x98`\x01\x80`\xA0\x1B\x03\x16\x85R` \x85\x01R`\xA0`@\x85\x01R`\xA0\x84\x01\x91a\x15\xD1V[\x94``\x82\x01R\x01RV[a\x16M\x94\x92``\x92`\x01\x80`\xA0\x1B\x03\x16\x82R` \x82\x01R\x81`@\x82\x01R\x01\x91a\x15\xD1V[\x90V[a\x16Y\x90a\x16\xDEV[`\x04\x81\x10\x15a\x0C>W`\x02\x14\x90V[_R_\x80Q` a \x82\x839\x81Q\x91R` R`\x01`@_ \x01T\x90V[a\x16\x8F\x90a\x16\xDEV[`\x04\x81\x10\x15a\x0C>W`\x03\x14\x90V[a\x16\xA7\x90a\x16\xDEV[`\x04\x81\x10\x15a\x0C>W\x15\x15\x90V[a\x16\xBE\x90a\x16\xDEV[`\x04\x81\x10\x15a\x0C>W`\x01\x81\x14\x90\x81\x15a\x16\xD6WP\x90V[`\x02\x91P\x14\x90V[_R_\x80Q` a \"\x839\x81Q\x91R` R`@_ T\x80\x15_\x14a\x17\x03WP_\x90V[`\x01\x81\x03a\x17\x11WP`\x03\x90V[B\x10\x15a\x17\x1DW`\x01\x90V[`\x02\x90V[\x94a\x17Xa\x17q\x94\x95\x92\x93`@Q\x96\x87\x95` \x87\x01\x99`\x01\x80`\xA0\x1B\x03\x16\x8AR`@\x87\x01R`\xA0``\x87\x01R`\xC0\x86\x01\x91a\x15\xD1V[\x91`\x80\x84\x01R`\xA0\x83\x01R\x03`\x1F\x19\x81\x01\x83R\x82a\x13\xC4V[Q\x90 \x90V[\x91\x90\x81\x10\x15a\x17\x87W`\x05\x1B\x01\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x01\xDEW\x90V[\x91\x90\x81\x10\x15a\x17\x87W`\x05\x1B\x81\x015\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x01\xDEW\x01\x90\x815\x91`\x01`\x01`@\x1B\x03\x83\x11a\x01\xDEW` \x01\x826\x03\x81\x13a\x01\xDEW\x91\x90V[\x96\x93\x94\x91\x90\x96\x95\x92\x95`@Q\x96` \x88\x01\x98\x80`\xC0\x8A\x01`\xA0\x8CRR`\xE0\x89\x01\x92\x90_[\x81\x81\x10a\x19\x0BWPPP\x87\x82\x03`\x1F\x19\x01`@\x89\x01R\x80\x82R`\x01`\x01`\xFB\x1B\x03\x81\x11a\x01\xDEW\x90\x87\x95\x93\x94\x92\x91`\x05\x1B\x80\x92` \x83\x017\x01\x84\x81\x03``\x86\x01R` \x81\x01\x84\x90R`\x05\x84\x90\x1B\x81\x01`@\x90\x81\x01\x94\x90\x82\x01\x91_\x90\x88\x906\x82\x90\x03`\x1E\x19\x01\x90[\x84\x84\x10a\x18\xA5WPPPPPPa\x17q\x94P`\x80\x84\x01R`\xA0\x83\x01R\x03`\x1F\x19\x81\x01\x83R\x82a\x13\xC4V[\x91\x93\x95\x97\x90\x92\x94\x96\x98P`\x1F\x19`\x1F\x19\x83\x83\x03\x01\x01\x87R\x895\x83\x81\x12\x15a\x01\xDEW\x84\x01\x90` \x825\x92\x01\x91`\x01`\x01`@\x1B\x03\x81\x11a\x01\xDEW\x806\x03\x83\x13a\x01\xDEWa\x18\xF7` \x92\x83\x92`\x01\x95a\x15\xD1V[\x9B\x01\x97\x01\x94\x01\x91\x8A\x98\x96\x99\x97\x95\x93\x91a\x18{V[\x90\x91\x93` \x80`\x01\x92\x83\x80`\xA0\x1B\x03a\x19#\x89a\x135V[\x16\x81R\x01\x95\x01\x92\x91\x01a\x18\x14V[\x90a\x19;\x82a\x16\x9EV[a\x19\xA4W_\x80Q` a \xA2\x839\x81Q\x91RT\x80\x82\x10a\x19\x8EWPB\x01\x90\x81B\x11a\x19zW_R_\x80Q` a \"\x839\x81Q\x91R` R`@_ UV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x90cT3f\t`\xE0\x1B_R`\x04R`$R`D_\xFD[Pc^\xAD\x8E\xB5`\xE0\x1B_R`\x04R`\x01`$R`D_\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_\x80Q` a B\x839\x81Q\x91R` R`@\x90 T`\xFF\x16\x15a\x19\xEBWPV[c\xE2Q}?`\xE0\x1B_\x90\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x04R\x7F\xB0\x9A\xA5\xAE\xB3p,\xFDP\xB6\xB6+\xC4S&\x04\x93\x8F!$\x8A'\xA1\xD5\xCAs`\x82\xB6\x81\x9C\xC1`$R`D\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_\x80Q` a \xE2\x839\x81Q\x91R` R`@\x90 T`\xFF\x16\x15a\x1A^WPV[c\xE2Q}?`\xE0\x1B_\x90\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x04R_\x80Q` a \xC2\x839\x81Q\x91R`$R`D\x90\xFD[\x90\x81_R_\x80Q` a \x82\x839\x81Q\x91R` R`@_ `\x01\x80`\xA0\x1B\x03\x82\x16_R` R`\xFF`@_ T\x16\x15a\x1A\xC7WPPV[c\xE2Q}?`\xE0\x1B_R`\x01\x80`\xA0\x1B\x03\x16`\x04R`$R`D_\xFD[a\x1A\xED\x81a\x16PV[\x15a\x1B%WP\x80\x15\x15\x80a\x1B\x15W[a\x1B\x03WPV[c\x12\x154\xC3`\xE3\x1B_R`\x04R`$_\xFD[Pa\x1B\x1F\x81a\x16\x86V[\x15a\x1A\xFCV[c^\xAD\x8E\xB5`\xE0\x1B_R`\x04R`\x04`$R`D_\xFD[=\x15a\x1BfW=\x90a\x1BM\x82a\x13\xF9V[\x91a\x1B[`@Q\x93\x84a\x13\xC4V[\x82R=_` \x84\x01>V[``\x90V[a\x1B\x94\x93_\x93\x92\x84\x93\x82`@Q\x93\x84\x92\x837\x81\x01\x85\x81R\x03\x92Z\xF1a\x1B\x8Ea\x1B<V[\x90a\x1F0V[PV[a\x1B\xA0\x81a\x16PV[\x15a\x1B%W_R_\x80Q` a \"\x839\x81Q\x91R` R`\x01`@_ UV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R\x7F\xB7\xDB-\xD0\x8F\xCBb\xD0\xC9\xE0\x8CQ\x94\x1C\xAES\xC2gxj\x0Bu\x80?\xB7\x96\t\x02\xFC\x8E\xF9}` R`@\x90 T`\xFF\x16a\x1CXW`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R\x7F\xB7\xDB-\xD0\x8F\xCBb\xD0\xC9\xE0\x8CQ\x94\x1C\xAES\xC2gxj\x0Bu\x80?\xB7\x96\t\x02\xFC\x8E\xF9}` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90_\x80Q` a\x1F\xE2\x839\x81Q\x91R\x81\x80\xA4`\x01\x90V[P_\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_\x80Q` a B\x839\x81Q\x91R` R`@\x90 T`\xFF\x16a\x1CXW`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_\x80Q` a B\x839\x81Q\x91R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90\x7F\xB0\x9A\xA5\xAE\xB3p,\xFDP\xB6\xB6+\xC4S&\x04\x93\x8F!$\x8A'\xA1\xD5\xCAs`\x82\xB6\x81\x9C\xC1\x90_\x80Q` a\x1F\xE2\x839\x81Q\x91R\x90\x80\xA4`\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_\x80Q` a \x02\x839\x81Q\x91R` R`@\x90 T`\xFF\x16a\x1CXW`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_\x80Q` a \x02\x839\x81Q\x91R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90\x7F\xFDd<rq\x0Cc\xC0\x18\x02Y\xAB\xA6\xB2\xD0TQ\xE3Y\x1A$\xE5\x8Bb#\x93x\x08W&\xF7\x83\x90_\x80Q` a\x1F\xE2\x839\x81Q\x91R\x90\x80\xA4`\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R_\x80Q` a \xE2\x839\x81Q\x91R` R`@\x90 T`\xFF\x16a\x1CXW`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R_\x80Q` a \xE2\x839\x81Q\x91R` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x91\x90_\x80Q` a \xC2\x839\x81Q\x91R\x90_\x80Q` a\x1F\xE2\x839\x81Q\x91R\x90\x80\xA4`\x01\x90V[_\x81\x81R_\x80Q` a \x82\x839\x81Q\x91R` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x84R\x90\x91R\x90 T`\xFF\x16a\x1E\x8EW_\x81\x81R_\x80Q` a \x82\x839\x81Q\x91R` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x95\x90\x95\x16\x80\x84R\x94\x90\x91R\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U3\x92\x91\x90_\x80Q` a\x1F\xE2\x839\x81Q\x91R\x90\x80\xA4`\x01\x90V[PP_\x90V[_\x81\x81R_\x80Q` a \x82\x839\x81Q\x91R` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x84R\x90\x91R\x90 T`\xFF\x16\x15a\x1E\x8EW_\x81\x81R_\x80Q` a \x82\x839\x81Q\x91R` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x95\x90\x95\x16\x80\x84R\x94\x90\x91R\x81 \x80T`\xFF\x19\x16\x90U3\x92\x91\x90\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x90\x80\xA4`\x01\x90V[\x90\x91\x90a\x1F=WPa\x1F~V[V[`\xFF_\x80Q` a!\x02\x839\x81Q\x91RT`@\x1C\x16\x15a\x1F[WV[c\x1A\xFC\xD7\x9F`\xE3\x1B_R`\x04_\xFD[\x80Q\x82\x10\x15a\x17\x87W` \x91`\x05\x1B\x01\x01\x90V[\x80Q\x15a\x1F\x8DW\x80Q\x90` \x01\xFD[c\xD6\xBD\xA2u`\xE0\x1B_R`\x04_\xFD[\x90a\x1F\xA7WPa\x1F~V[\x81Q\x15\x80a\x1F\xD8W[a\x1F\xB8WP\x90V[c\x99\x96\xB3\x15`\xE0\x1B_\x90\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x04R`$\x90\xFD[P\x80;\x15a\x1F\xB0V\xFE/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r\xFAq\xE0\x7F$\xC4p\x1E\xF6Z\x97\x07u\x97\x9D\xE1),\xFE\x90\x935\xCD\x18\xA3-+{s\x98y\x14\x9A7\xC2\xAA\x9D\x18j\ti\xFF\x8A\x82g\xBFN\x07\xE8d\xC2\xF2v\x8FP@\x94\x9E(\xA6$\xFB6\0Z\x874\xC3K\x98\xD7\xC9n\xB2\xEA%\xF2\x98\x98\x94\x07\xE1\xF2]\xA1\x16\xEC\x13\x9B\xCC\xE0\x88{\xCB|\xF76\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x02\xDD{\xC7\xDE\xC4\xDC\xEE\xDD\xA7u\xE5\x8D\xD5A\xE0\x8A\x11llS\x81\\\x0B\xD0(\x19/{bh\0\x9A7\xC2\xAA\x9D\x18j\ti\xFF\x8A\x82g\xBFN\x07\xE8d\xC2\xF2v\x8FP@\x94\x9E(\xA6$\xFB6\x01\xD8\xAA\x0F1\x94\x97\x1A*\x11fy\xF7\xC2\t\x0Fi9\xC8\xD4\xE0\x1A*\x8D~A\xD5^SQF\x9EcR\xFC\xE5\xE8\xA5\xD0\xD9\xE8\xD1\xEA)\xF4R^Q.\x9C'\xBF\x92\xCA\xE5\x03t\xD4\x97\xF9\x18\xABH\xF3\x82\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\xA1dsolcC\0\x08\x1A\0\n",
    );
    /**Custom error with signature `AccessControlBadConfirmation()` and selector `0x6697b232`.
```solidity
error AccessControlBadConfirmation();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AccessControlBadConfirmation {}
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
        impl ::core::convert::From<AccessControlBadConfirmation>
        for UnderlyingRustTuple<'_> {
            fn from(value: AccessControlBadConfirmation) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for AccessControlBadConfirmation {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AccessControlBadConfirmation {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AccessControlBadConfirmation()";
            const SELECTOR: [u8; 4] = [102u8, 151u8, 178u8, 50u8];
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
    /**Custom error with signature `AccessControlUnauthorizedAccount(address,bytes32)` and selector `0xe2517d3f`.
```solidity
error AccessControlUnauthorizedAccount(address account, bytes32 neededRole);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AccessControlUnauthorizedAccount {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub neededRole: alloy::sol_types::private::FixedBytes<32>,
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
            alloy::sol_types::sol_data::FixedBytes<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<AccessControlUnauthorizedAccount>
        for UnderlyingRustTuple<'_> {
            fn from(value: AccessControlUnauthorizedAccount) -> Self {
                (value.account, value.neededRole)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for AccessControlUnauthorizedAccount {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    account: tuple.0,
                    neededRole: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AccessControlUnauthorizedAccount {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AccessControlUnauthorizedAccount(address,bytes32)";
            const SELECTOR: [u8; 4] = [226u8, 81u8, 125u8, 63u8];
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.neededRole),
                )
            }
        }
    };
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
    /**Custom error with signature `TimelockInsufficientDelay(uint256,uint256)` and selector `0x54336609`.
```solidity
error TimelockInsufficientDelay(uint256 delay, uint256 minDelay);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TimelockInsufficientDelay {
        #[allow(missing_docs)]
        pub delay: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub minDelay: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<TimelockInsufficientDelay>
        for UnderlyingRustTuple<'_> {
            fn from(value: TimelockInsufficientDelay) -> Self {
                (value.delay, value.minDelay)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for TimelockInsufficientDelay {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    delay: tuple.0,
                    minDelay: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TimelockInsufficientDelay {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TimelockInsufficientDelay(uint256,uint256)";
            const SELECTOR: [u8; 4] = [84u8, 51u8, 102u8, 9u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.delay),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.minDelay),
                )
            }
        }
    };
    /**Custom error with signature `TimelockInvalidOperationLength(uint256,uint256,uint256)` and selector `0xffb03211`.
```solidity
error TimelockInvalidOperationLength(uint256 targets, uint256 payloads, uint256 values);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TimelockInvalidOperationLength {
        #[allow(missing_docs)]
        pub targets: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub payloads: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<TimelockInvalidOperationLength>
        for UnderlyingRustTuple<'_> {
            fn from(value: TimelockInvalidOperationLength) -> Self {
                (value.targets, value.payloads, value.values)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for TimelockInvalidOperationLength {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    targets: tuple.0,
                    payloads: tuple.1,
                    values: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TimelockInvalidOperationLength {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TimelockInvalidOperationLength(uint256,uint256,uint256)";
            const SELECTOR: [u8; 4] = [255u8, 176u8, 50u8, 17u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.payloads),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.values),
                )
            }
        }
    };
    /**Custom error with signature `TimelockUnauthorizedCaller(address)` and selector `0xe2850c59`.
```solidity
error TimelockUnauthorizedCaller(address caller);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TimelockUnauthorizedCaller {
        #[allow(missing_docs)]
        pub caller: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<TimelockUnauthorizedCaller>
        for UnderlyingRustTuple<'_> {
            fn from(value: TimelockUnauthorizedCaller) -> Self {
                (value.caller,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for TimelockUnauthorizedCaller {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { caller: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TimelockUnauthorizedCaller {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TimelockUnauthorizedCaller(address)";
            const SELECTOR: [u8; 4] = [226u8, 133u8, 12u8, 89u8];
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
                        &self.caller,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `TimelockUnexecutedPredecessor(bytes32)` and selector `0x90a9a618`.
```solidity
error TimelockUnexecutedPredecessor(bytes32 predecessorId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TimelockUnexecutedPredecessor {
        #[allow(missing_docs)]
        pub predecessorId: alloy::sol_types::private::FixedBytes<32>,
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
        impl ::core::convert::From<TimelockUnexecutedPredecessor>
        for UnderlyingRustTuple<'_> {
            fn from(value: TimelockUnexecutedPredecessor) -> Self {
                (value.predecessorId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for TimelockUnexecutedPredecessor {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { predecessorId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TimelockUnexecutedPredecessor {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TimelockUnexecutedPredecessor(bytes32)";
            const SELECTOR: [u8; 4] = [144u8, 169u8, 166u8, 24u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.predecessorId),
                )
            }
        }
    };
    /**Custom error with signature `TimelockUnexpectedOperationState(bytes32,bytes32)` and selector `0x5ead8eb5`.
```solidity
error TimelockUnexpectedOperationState(bytes32 operationId, bytes32 expectedStates);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TimelockUnexpectedOperationState {
        #[allow(missing_docs)]
        pub operationId: alloy::sol_types::private::FixedBytes<32>,
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
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
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
        impl ::core::convert::From<TimelockUnexpectedOperationState>
        for UnderlyingRustTuple<'_> {
            fn from(value: TimelockUnexpectedOperationState) -> Self {
                (value.operationId, value.expectedStates)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for TimelockUnexpectedOperationState {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    operationId: tuple.0,
                    expectedStates: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TimelockUnexpectedOperationState {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TimelockUnexpectedOperationState(bytes32,bytes32)";
            const SELECTOR: [u8; 4] = [94u8, 173u8, 142u8, 181u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.operationId),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.expectedStates),
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
    /**Event with signature `CallExecuted(bytes32,uint256,address,uint256,bytes)` and selector `0xc2617efa69bab66782fa219543714338489c4e9e178271560a91b82c3f612b58`.
```solidity
event CallExecuted(bytes32 indexed id, uint256 indexed index, address target, uint256 value, bytes data);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct CallExecuted {
        #[allow(missing_docs)]
        pub id: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub index: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub target: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
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
        impl alloy_sol_types::SolEvent for CallExecuted {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "CallExecuted(bytes32,uint256,address,uint256,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                194u8,
                97u8,
                126u8,
                250u8,
                105u8,
                186u8,
                182u8,
                103u8,
                130u8,
                250u8,
                33u8,
                149u8,
                67u8,
                113u8,
                67u8,
                56u8,
                72u8,
                156u8,
                78u8,
                158u8,
                23u8,
                130u8,
                113u8,
                86u8,
                10u8,
                145u8,
                184u8,
                44u8,
                63u8,
                97u8,
                43u8,
                88u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    id: topics.1,
                    index: topics.2,
                    target: data.0,
                    value: data.1,
                    data: data.2,
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
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.id.clone(), self.index.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.id);
                out[2usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.index);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for CallExecuted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&CallExecuted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &CallExecuted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `CallSalt(bytes32,bytes32)` and selector `0x20fda5fd27a1ea7bf5b9567f143ac5470bb059374a27e8f67cb44f946f6d0387`.
```solidity
event CallSalt(bytes32 indexed id, bytes32 salt);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct CallSalt {
        #[allow(missing_docs)]
        pub id: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub salt: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for CallSalt {
            type DataTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "CallSalt(bytes32,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                32u8,
                253u8,
                165u8,
                253u8,
                39u8,
                161u8,
                234u8,
                123u8,
                245u8,
                185u8,
                86u8,
                127u8,
                20u8,
                58u8,
                197u8,
                71u8,
                11u8,
                176u8,
                89u8,
                55u8,
                74u8,
                39u8,
                232u8,
                246u8,
                124u8,
                180u8,
                79u8,
                148u8,
                111u8,
                109u8,
                3u8,
                135u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { id: topics.1, salt: data.0 }
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.id.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.id);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for CallSalt {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&CallSalt> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &CallSalt) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `CallScheduled(bytes32,uint256,address,uint256,bytes,bytes32,uint256)` and selector `0x4cf4410cc57040e44862ef0f45f3dd5a5e02db8eb8add648d4b0e236f1d07dca`.
```solidity
event CallScheduled(bytes32 indexed id, uint256 indexed index, address target, uint256 value, bytes data, bytes32 predecessor, uint256 delay);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct CallScheduled {
        #[allow(missing_docs)]
        pub id: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub index: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub target: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub predecessor: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub delay: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for CallScheduled {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "CallScheduled(bytes32,uint256,address,uint256,bytes,bytes32,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                76u8,
                244u8,
                65u8,
                12u8,
                197u8,
                112u8,
                64u8,
                228u8,
                72u8,
                98u8,
                239u8,
                15u8,
                69u8,
                243u8,
                221u8,
                90u8,
                94u8,
                2u8,
                219u8,
                142u8,
                184u8,
                173u8,
                214u8,
                72u8,
                212u8,
                176u8,
                226u8,
                54u8,
                241u8,
                208u8,
                125u8,
                202u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    id: topics.1,
                    index: topics.2,
                    target: data.0,
                    value: data.1,
                    data: data.2,
                    predecessor: data.3,
                    delay: data.4,
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
                        &self.target,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.predecessor),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.delay),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.id.clone(), self.index.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.id);
                out[2usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.index);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for CallScheduled {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&CallScheduled> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &CallScheduled) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Cancelled(bytes32)` and selector `0xbaa1eb22f2a492ba1a5fea61b8df4d27c6c8b5f3971e63bb58fa14ff72eedb70`.
```solidity
event Cancelled(bytes32 indexed id);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Cancelled {
        #[allow(missing_docs)]
        pub id: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for Cancelled {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "Cancelled(bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                186u8,
                161u8,
                235u8,
                34u8,
                242u8,
                164u8,
                146u8,
                186u8,
                26u8,
                95u8,
                234u8,
                97u8,
                184u8,
                223u8,
                77u8,
                39u8,
                198u8,
                200u8,
                181u8,
                243u8,
                151u8,
                30u8,
                99u8,
                187u8,
                88u8,
                250u8,
                20u8,
                255u8,
                114u8,
                238u8,
                219u8,
                112u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { id: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.id.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.id);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Cancelled {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Cancelled> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Cancelled) -> alloy_sol_types::private::LogData {
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
    /**Event with signature `MinDelayChange(uint256,uint256)` and selector `0x11c24f4ead16507c69ac467fbd5e4eed5fb5c699626d2cc6d66421df253886d5`.
```solidity
event MinDelayChange(uint256 oldDuration, uint256 newDuration);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct MinDelayChange {
        #[allow(missing_docs)]
        pub oldDuration: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newDuration: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for MinDelayChange {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "MinDelayChange(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                17u8,
                194u8,
                79u8,
                78u8,
                173u8,
                22u8,
                80u8,
                124u8,
                105u8,
                172u8,
                70u8,
                127u8,
                189u8,
                94u8,
                78u8,
                237u8,
                95u8,
                181u8,
                198u8,
                153u8,
                98u8,
                109u8,
                44u8,
                198u8,
                214u8,
                100u8,
                33u8,
                223u8,
                37u8,
                56u8,
                134u8,
                213u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    oldDuration: data.0,
                    newDuration: data.1,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.oldDuration),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newDuration),
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
        impl alloy_sol_types::private::IntoLogData for MinDelayChange {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&MinDelayChange> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &MinDelayChange) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `RoleAdminChanged(bytes32,bytes32,bytes32)` and selector `0xbd79b86ffe0ab8e8776151514217cd7cacd52c909f66475c3af44e129f0b00ff`.
```solidity
event RoleAdminChanged(bytes32 indexed role, bytes32 indexed previousAdminRole, bytes32 indexed newAdminRole);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RoleAdminChanged {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub previousAdminRole: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub newAdminRole: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for RoleAdminChanged {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "RoleAdminChanged(bytes32,bytes32,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                189u8,
                121u8,
                184u8,
                111u8,
                254u8,
                10u8,
                184u8,
                232u8,
                119u8,
                97u8,
                81u8,
                81u8,
                66u8,
                23u8,
                205u8,
                124u8,
                172u8,
                213u8,
                44u8,
                144u8,
                159u8,
                102u8,
                71u8,
                92u8,
                58u8,
                244u8,
                78u8,
                18u8,
                159u8,
                11u8,
                0u8,
                255u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    role: topics.1,
                    previousAdminRole: topics.2,
                    newAdminRole: topics.3,
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
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.role.clone(),
                    self.previousAdminRole.clone(),
                    self.newAdminRole.clone(),
                )
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
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.role);
                out[2usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.previousAdminRole);
                out[3usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.newAdminRole);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RoleAdminChanged {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RoleAdminChanged> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RoleAdminChanged) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `RoleGranted(bytes32,address,address)` and selector `0x2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d`.
```solidity
event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RoleGranted {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for RoleGranted {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "RoleGranted(bytes32,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                47u8,
                135u8,
                136u8,
                17u8,
                126u8,
                126u8,
                255u8,
                29u8,
                130u8,
                233u8,
                38u8,
                236u8,
                121u8,
                73u8,
                1u8,
                209u8,
                124u8,
                120u8,
                2u8,
                74u8,
                80u8,
                39u8,
                9u8,
                64u8,
                48u8,
                69u8,
                64u8,
                167u8,
                51u8,
                101u8,
                111u8,
                13u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    role: topics.1,
                    account: topics.2,
                    sender: topics.3,
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
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.role.clone(),
                    self.account.clone(),
                    self.sender.clone(),
                )
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
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.role);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.account,
                );
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.sender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RoleGranted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RoleGranted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RoleGranted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `RoleRevoked(bytes32,address,address)` and selector `0xf6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b`.
```solidity
event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RoleRevoked {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for RoleRevoked {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "RoleRevoked(bytes32,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                246u8,
                57u8,
                31u8,
                92u8,
                50u8,
                217u8,
                198u8,
                157u8,
                42u8,
                71u8,
                234u8,
                103u8,
                11u8,
                68u8,
                41u8,
                116u8,
                181u8,
                57u8,
                53u8,
                209u8,
                237u8,
                199u8,
                253u8,
                100u8,
                235u8,
                33u8,
                224u8,
                71u8,
                168u8,
                57u8,
                23u8,
                27u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    role: topics.1,
                    account: topics.2,
                    sender: topics.3,
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
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.role.clone(),
                    self.account.clone(),
                    self.sender.clone(),
                )
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
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.role);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.account,
                );
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.sender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RoleRevoked {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RoleRevoked> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RoleRevoked) -> alloy_sol_types::private::LogData {
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
    /**Function with signature `CANCELLER_ROLE()` and selector `0xb08e51c0`.
```solidity
function CANCELLER_ROLE() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CANCELLER_ROLECall {}
    ///Container type for the return parameters of the [`CANCELLER_ROLE()`](CANCELLER_ROLECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CANCELLER_ROLEReturn {
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
            impl ::core::convert::From<CANCELLER_ROLECall> for UnderlyingRustTuple<'_> {
                fn from(value: CANCELLER_ROLECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for CANCELLER_ROLECall {
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
            impl ::core::convert::From<CANCELLER_ROLEReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: CANCELLER_ROLEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for CANCELLER_ROLEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for CANCELLER_ROLECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = CANCELLER_ROLEReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CANCELLER_ROLE()";
            const SELECTOR: [u8; 4] = [176u8, 142u8, 81u8, 192u8];
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
    /**Function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`.
```solidity
function DEFAULT_ADMIN_ROLE() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DEFAULT_ADMIN_ROLECall {}
    ///Container type for the return parameters of the [`DEFAULT_ADMIN_ROLE()`](DEFAULT_ADMIN_ROLECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DEFAULT_ADMIN_ROLEReturn {
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
            impl ::core::convert::From<DEFAULT_ADMIN_ROLECall>
            for UnderlyingRustTuple<'_> {
                fn from(value: DEFAULT_ADMIN_ROLECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DEFAULT_ADMIN_ROLECall {
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
            impl ::core::convert::From<DEFAULT_ADMIN_ROLEReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: DEFAULT_ADMIN_ROLEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DEFAULT_ADMIN_ROLEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for DEFAULT_ADMIN_ROLECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = DEFAULT_ADMIN_ROLEReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DEFAULT_ADMIN_ROLE()";
            const SELECTOR: [u8; 4] = [162u8, 23u8, 253u8, 223u8];
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
    /**Function with signature `EXECUTOR_ROLE()` and selector `0x07bd0265`.
```solidity
function EXECUTOR_ROLE() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EXECUTOR_ROLECall {}
    ///Container type for the return parameters of the [`EXECUTOR_ROLE()`](EXECUTOR_ROLECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EXECUTOR_ROLEReturn {
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
            impl ::core::convert::From<EXECUTOR_ROLECall> for UnderlyingRustTuple<'_> {
                fn from(value: EXECUTOR_ROLECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for EXECUTOR_ROLECall {
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
            impl ::core::convert::From<EXECUTOR_ROLEReturn> for UnderlyingRustTuple<'_> {
                fn from(value: EXECUTOR_ROLEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for EXECUTOR_ROLEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for EXECUTOR_ROLECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = EXECUTOR_ROLEReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EXECUTOR_ROLE()";
            const SELECTOR: [u8; 4] = [7u8, 189u8, 2u8, 101u8];
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
    /**Function with signature `MAX_DELAY()` and selector `0x4125ff90`.
```solidity
function MAX_DELAY() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_DELAYCall {}
    ///Container type for the return parameters of the [`MAX_DELAY()`](MAX_DELAYCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_DELAYReturn {
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
            impl ::core::convert::From<MAX_DELAYCall> for UnderlyingRustTuple<'_> {
                fn from(value: MAX_DELAYCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for MAX_DELAYCall {
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
            impl ::core::convert::From<MAX_DELAYReturn> for UnderlyingRustTuple<'_> {
                fn from(value: MAX_DELAYReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for MAX_DELAYReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for MAX_DELAYCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = MAX_DELAYReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MAX_DELAY()";
            const SELECTOR: [u8; 4] = [65u8, 37u8, 255u8, 144u8];
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
    /**Function with signature `MIN_DELAY()` and selector `0x9f81aed7`.
```solidity
function MIN_DELAY() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MIN_DELAYCall {}
    ///Container type for the return parameters of the [`MIN_DELAY()`](MIN_DELAYCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MIN_DELAYReturn {
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
            impl ::core::convert::From<MIN_DELAYCall> for UnderlyingRustTuple<'_> {
                fn from(value: MIN_DELAYCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for MIN_DELAYCall {
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
            impl ::core::convert::From<MIN_DELAYReturn> for UnderlyingRustTuple<'_> {
                fn from(value: MIN_DELAYReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for MIN_DELAYReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for MIN_DELAYCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = MIN_DELAYReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MIN_DELAY()";
            const SELECTOR: [u8; 4] = [159u8, 129u8, 174u8, 215u8];
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
    /**Function with signature `PROPOSER_ROLE()` and selector `0x8f61f4f5`.
```solidity
function PROPOSER_ROLE() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PROPOSER_ROLECall {}
    ///Container type for the return parameters of the [`PROPOSER_ROLE()`](PROPOSER_ROLECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PROPOSER_ROLEReturn {
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
            impl ::core::convert::From<PROPOSER_ROLECall> for UnderlyingRustTuple<'_> {
                fn from(value: PROPOSER_ROLECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for PROPOSER_ROLECall {
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
            impl ::core::convert::From<PROPOSER_ROLEReturn> for UnderlyingRustTuple<'_> {
                fn from(value: PROPOSER_ROLEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for PROPOSER_ROLEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for PROPOSER_ROLECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = PROPOSER_ROLEReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PROPOSER_ROLE()";
            const SELECTOR: [u8; 4] = [143u8, 97u8, 244u8, 245u8];
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
    /**Function with signature `cancel(bytes32)` and selector `0xc4d252f5`.
```solidity
function cancel(bytes32 id) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cancelCall {
        #[allow(missing_docs)]
        pub id: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`cancel(bytes32)`](cancelCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cancelReturn {}
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
            impl ::core::convert::From<cancelCall> for UnderlyingRustTuple<'_> {
                fn from(value: cancelCall) -> Self {
                    (value.id,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for cancelCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { id: tuple.0 }
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
            impl ::core::convert::From<cancelReturn> for UnderlyingRustTuple<'_> {
                fn from(value: cancelReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for cancelReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for cancelCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = cancelReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "cancel(bytes32)";
            const SELECTOR: [u8; 4] = [196u8, 210u8, 82u8, 245u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
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
    /**Function with signature `execute(address,uint256,bytes,bytes32,bytes32)` and selector `0x134008d3`.
```solidity
function execute(address target, uint256 value, bytes memory payload, bytes32 predecessor, bytes32 salt) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeCall {
        #[allow(missing_docs)]
        pub target: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub payload: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub predecessor: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub salt: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`execute(address,uint256,bytes,bytes32,bytes32)`](executeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeReturn {}
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
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::FixedBytes<32>,
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
                    (
                        value.target,
                        value.value,
                        value.payload,
                        value.predecessor,
                        value.salt,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        target: tuple.0,
                        value: tuple.1,
                        payload: tuple.2,
                        predecessor: tuple.3,
                        salt: tuple.4,
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
            impl ::core::convert::From<executeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: executeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for executeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = executeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "execute(address,uint256,bytes,bytes32,bytes32)";
            const SELECTOR: [u8; 4] = [19u8, 64u8, 8u8, 211u8];
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
                        &self.payload,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.predecessor),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
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
    /**Function with signature `executeBatch(address[],uint256[],bytes[],bytes32,bytes32)` and selector `0xe38335e5`.
```solidity
function executeBatch(address[] memory targets, uint256[] memory values, bytes[] memory payloads, bytes32 predecessor, bytes32 salt) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeBatchCall {
        #[allow(missing_docs)]
        pub targets: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        #[allow(missing_docs)]
        pub values: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        #[allow(missing_docs)]
        pub payloads: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
        #[allow(missing_docs)]
        pub predecessor: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub salt: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`executeBatch(address[],uint256[],bytes[],bytes32,bytes32)`](executeBatchCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeBatchReturn {}
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
            impl ::core::convert::From<executeBatchCall> for UnderlyingRustTuple<'_> {
                fn from(value: executeBatchCall) -> Self {
                    (
                        value.targets,
                        value.values,
                        value.payloads,
                        value.predecessor,
                        value.salt,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeBatchCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targets: tuple.0,
                        values: tuple.1,
                        payloads: tuple.2,
                        predecessor: tuple.3,
                        salt: tuple.4,
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
            impl ::core::convert::From<executeBatchReturn> for UnderlyingRustTuple<'_> {
                fn from(value: executeBatchReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeBatchReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for executeBatchCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = executeBatchReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "executeBatch(address[],uint256[],bytes[],bytes32,bytes32)";
            const SELECTOR: [u8; 4] = [227u8, 131u8, 53u8, 229u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.payloads),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.predecessor),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
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
    /**Function with signature `getMinDelay()` and selector `0xf27a0c92`.
```solidity
function getMinDelay() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMinDelayCall {}
    ///Container type for the return parameters of the [`getMinDelay()`](getMinDelayCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMinDelayReturn {
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
            impl ::core::convert::From<getMinDelayCall> for UnderlyingRustTuple<'_> {
                fn from(value: getMinDelayCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getMinDelayCall {
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
            impl ::core::convert::From<getMinDelayReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getMinDelayReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getMinDelayReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getMinDelayCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getMinDelayReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getMinDelay()";
            const SELECTOR: [u8; 4] = [242u8, 122u8, 12u8, 146u8];
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
    /**Function with signature `getOperationState(bytes32)` and selector `0x7958004c`.
```solidity
function getOperationState(bytes32 id) external view returns (TimelockControllerUpgradeable.OperationState);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperationStateCall {
        #[allow(missing_docs)]
        pub id: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`getOperationState(bytes32)`](getOperationStateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperationStateReturn {
        #[allow(missing_docs)]
        pub _0: <TimelockControllerUpgradeable::OperationState as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getOperationStateCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperationStateCall) -> Self {
                    (value.id,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperationStateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { id: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                TimelockControllerUpgradeable::OperationState,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <TimelockControllerUpgradeable::OperationState as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getOperationStateReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperationStateReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperationStateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperationStateCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperationStateReturn;
            type ReturnTuple<'a> = (TimelockControllerUpgradeable::OperationState,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperationState(bytes32)";
            const SELECTOR: [u8; 4] = [121u8, 88u8, 0u8, 76u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
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
    /**Function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`.
```solidity
function getRoleAdmin(bytes32 role) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRoleAdminCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`getRoleAdmin(bytes32)`](getRoleAdminCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRoleAdminReturn {
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
            impl ::core::convert::From<getRoleAdminCall> for UnderlyingRustTuple<'_> {
                fn from(value: getRoleAdminCall) -> Self {
                    (value.role,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRoleAdminCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { role: tuple.0 }
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
            impl ::core::convert::From<getRoleAdminReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getRoleAdminReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRoleAdminReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRoleAdminCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getRoleAdminReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getRoleAdmin(bytes32)";
            const SELECTOR: [u8; 4] = [36u8, 138u8, 156u8, 163u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
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
    /**Function with signature `getTimestamp(bytes32)` and selector `0xd45c4435`.
```solidity
function getTimestamp(bytes32 id) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTimestampCall {
        #[allow(missing_docs)]
        pub id: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`getTimestamp(bytes32)`](getTimestampCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTimestampReturn {
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
            impl ::core::convert::From<getTimestampCall> for UnderlyingRustTuple<'_> {
                fn from(value: getTimestampCall) -> Self {
                    (value.id,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTimestampCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { id: tuple.0 }
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
            impl ::core::convert::From<getTimestampReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getTimestampReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTimestampReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTimestampCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTimestampReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTimestamp(bytes32)";
            const SELECTOR: [u8; 4] = [212u8, 92u8, 68u8, 53u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
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
    /**Function with signature `grantRole(bytes32,address)` and selector `0x2f2ff15d`.
```solidity
function grantRole(bytes32 role, address account) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct grantRoleCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`grantRole(bytes32,address)`](grantRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct grantRoleReturn {}
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
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<grantRoleCall> for UnderlyingRustTuple<'_> {
                fn from(value: grantRoleCall) -> Self {
                    (value.role, value.account)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for grantRoleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        role: tuple.0,
                        account: tuple.1,
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
            impl ::core::convert::From<grantRoleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: grantRoleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for grantRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for grantRoleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = grantRoleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "grantRole(bytes32,address)";
            const SELECTOR: [u8; 4] = [47u8, 47u8, 241u8, 93u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
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
    /**Function with signature `hasRole(bytes32,address)` and selector `0x91d14854`.
```solidity
function hasRole(bytes32 role, address account) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hasRoleCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`hasRole(bytes32,address)`](hasRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hasRoleReturn {
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
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<hasRoleCall> for UnderlyingRustTuple<'_> {
                fn from(value: hasRoleCall) -> Self {
                    (value.role, value.account)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hasRoleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        role: tuple.0,
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
            impl ::core::convert::From<hasRoleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: hasRoleReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hasRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for hasRoleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = hasRoleReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "hasRole(bytes32,address)";
            const SELECTOR: [u8; 4] = [145u8, 209u8, 72u8, 84u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
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
    /**Function with signature `hashOperation(address,uint256,bytes,bytes32,bytes32)` and selector `0x8065657f`.
```solidity
function hashOperation(address target, uint256 value, bytes memory data, bytes32 predecessor, bytes32 salt) external pure returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hashOperationCall {
        #[allow(missing_docs)]
        pub target: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub predecessor: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub salt: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`hashOperation(address,uint256,bytes,bytes32,bytes32)`](hashOperationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hashOperationReturn {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<hashOperationCall> for UnderlyingRustTuple<'_> {
                fn from(value: hashOperationCall) -> Self {
                    (
                        value.target,
                        value.value,
                        value.data,
                        value.predecessor,
                        value.salt,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hashOperationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        target: tuple.0,
                        value: tuple.1,
                        data: tuple.2,
                        predecessor: tuple.3,
                        salt: tuple.4,
                    }
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
            impl ::core::convert::From<hashOperationReturn> for UnderlyingRustTuple<'_> {
                fn from(value: hashOperationReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hashOperationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for hashOperationCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = hashOperationReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "hashOperation(address,uint256,bytes,bytes32,bytes32)";
            const SELECTOR: [u8; 4] = [128u8, 101u8, 101u8, 127u8];
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.predecessor),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
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
    /**Function with signature `hashOperationBatch(address[],uint256[],bytes[],bytes32,bytes32)` and selector `0xb1c5f427`.
```solidity
function hashOperationBatch(address[] memory targets, uint256[] memory values, bytes[] memory payloads, bytes32 predecessor, bytes32 salt) external pure returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hashOperationBatchCall {
        #[allow(missing_docs)]
        pub targets: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        #[allow(missing_docs)]
        pub values: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        #[allow(missing_docs)]
        pub payloads: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
        #[allow(missing_docs)]
        pub predecessor: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub salt: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`hashOperationBatch(address[],uint256[],bytes[],bytes32,bytes32)`](hashOperationBatchCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hashOperationBatchReturn {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
                alloy::sol_types::sol_data::FixedBytes<32>,
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
            impl ::core::convert::From<hashOperationBatchCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: hashOperationBatchCall) -> Self {
                    (
                        value.targets,
                        value.values,
                        value.payloads,
                        value.predecessor,
                        value.salt,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for hashOperationBatchCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targets: tuple.0,
                        values: tuple.1,
                        payloads: tuple.2,
                        predecessor: tuple.3,
                        salt: tuple.4,
                    }
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
            impl ::core::convert::From<hashOperationBatchReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: hashOperationBatchReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for hashOperationBatchReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for hashOperationBatchCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = hashOperationBatchReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "hashOperationBatch(address[],uint256[],bytes[],bytes32,bytes32)";
            const SELECTOR: [u8; 4] = [177u8, 197u8, 244u8, 39u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.payloads),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.predecessor),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
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
    /**Function with signature `initialize(uint256,address[],address[],address)` and selector `0xc4c4c7b3`.
```solidity
function initialize(uint256 minDelay, address[] memory proposers, address[] memory executors, address admin) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        #[allow(missing_docs)]
        pub minDelay: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub proposers: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub executors: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub admin: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`initialize(uint256,address[],address[],address)`](initializeCall) function.
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (value.minDelay, value.proposers, value.executors, value.admin)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        minDelay: tuple.0,
                        proposers: tuple.1,
                        executors: tuple.2,
                        admin: tuple.3,
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize(uint256,address[],address[],address)";
            const SELECTOR: [u8; 4] = [196u8, 196u8, 199u8, 179u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.minDelay),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.proposers),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.executors),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.admin,
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
    /**Function with signature `isCanceller(address)` and selector `0xb426475e`.
```solidity
function isCanceller(address account) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isCancellerCall {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`isCanceller(address)`](isCancellerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isCancellerReturn {
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
            impl ::core::convert::From<isCancellerCall> for UnderlyingRustTuple<'_> {
                fn from(value: isCancellerCall) -> Self {
                    (value.account,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isCancellerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { account: tuple.0 }
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
            impl ::core::convert::From<isCancellerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isCancellerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isCancellerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isCancellerCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isCancellerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isCanceller(address)";
            const SELECTOR: [u8; 4] = [180u8, 38u8, 71u8, 94u8];
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
    /**Function with signature `isExecutor(address)` and selector `0xdebfda30`.
```solidity
function isExecutor(address account) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isExecutorCall {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`isExecutor(address)`](isExecutorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isExecutorReturn {
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
            impl ::core::convert::From<isExecutorCall> for UnderlyingRustTuple<'_> {
                fn from(value: isExecutorCall) -> Self {
                    (value.account,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isExecutorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { account: tuple.0 }
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
            impl ::core::convert::From<isExecutorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isExecutorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isExecutorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isExecutorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isExecutorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isExecutor(address)";
            const SELECTOR: [u8; 4] = [222u8, 191u8, 218u8, 48u8];
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
    /**Function with signature `isOperation(bytes32)` and selector `0x31d50750`.
```solidity
function isOperation(bytes32 id) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperationCall {
        #[allow(missing_docs)]
        pub id: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`isOperation(bytes32)`](isOperationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperationReturn {
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
            impl ::core::convert::From<isOperationCall> for UnderlyingRustTuple<'_> {
                fn from(value: isOperationCall) -> Self {
                    (value.id,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isOperationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { id: tuple.0 }
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
            impl ::core::convert::From<isOperationReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isOperationReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isOperationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isOperationCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isOperationReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isOperation(bytes32)";
            const SELECTOR: [u8; 4] = [49u8, 213u8, 7u8, 80u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
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
    /**Function with signature `isOperationDone(bytes32)` and selector `0x2ab0f529`.
```solidity
function isOperationDone(bytes32 id) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperationDoneCall {
        #[allow(missing_docs)]
        pub id: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`isOperationDone(bytes32)`](isOperationDoneCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperationDoneReturn {
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
            impl ::core::convert::From<isOperationDoneCall> for UnderlyingRustTuple<'_> {
                fn from(value: isOperationDoneCall) -> Self {
                    (value.id,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isOperationDoneCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { id: tuple.0 }
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
            impl ::core::convert::From<isOperationDoneReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isOperationDoneReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isOperationDoneReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isOperationDoneCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isOperationDoneReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isOperationDone(bytes32)";
            const SELECTOR: [u8; 4] = [42u8, 176u8, 245u8, 41u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
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
    /**Function with signature `isOperationPending(bytes32)` and selector `0x584b153e`.
```solidity
function isOperationPending(bytes32 id) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperationPendingCall {
        #[allow(missing_docs)]
        pub id: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`isOperationPending(bytes32)`](isOperationPendingCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperationPendingReturn {
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
            impl ::core::convert::From<isOperationPendingCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isOperationPendingCall) -> Self {
                    (value.id,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isOperationPendingCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { id: tuple.0 }
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
            impl ::core::convert::From<isOperationPendingReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isOperationPendingReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isOperationPendingReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isOperationPendingCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isOperationPendingReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isOperationPending(bytes32)";
            const SELECTOR: [u8; 4] = [88u8, 75u8, 21u8, 62u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
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
    /**Function with signature `isOperationReady(bytes32)` and selector `0x13bc9f20`.
```solidity
function isOperationReady(bytes32 id) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperationReadyCall {
        #[allow(missing_docs)]
        pub id: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`isOperationReady(bytes32)`](isOperationReadyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperationReadyReturn {
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
            impl ::core::convert::From<isOperationReadyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isOperationReadyCall) -> Self {
                    (value.id,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isOperationReadyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { id: tuple.0 }
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
            impl ::core::convert::From<isOperationReadyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isOperationReadyReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isOperationReadyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isOperationReadyCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isOperationReadyReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isOperationReady(bytes32)";
            const SELECTOR: [u8; 4] = [19u8, 188u8, 159u8, 32u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
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
    /**Function with signature `isProposer(address)` and selector `0x74ec29a0`.
```solidity
function isProposer(address account) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isProposerCall {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`isProposer(address)`](isProposerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isProposerReturn {
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
            impl ::core::convert::From<isProposerCall> for UnderlyingRustTuple<'_> {
                fn from(value: isProposerCall) -> Self {
                    (value.account,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isProposerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { account: tuple.0 }
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
            impl ::core::convert::From<isProposerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isProposerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isProposerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isProposerCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isProposerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isProposer(address)";
            const SELECTOR: [u8; 4] = [116u8, 236u8, 41u8, 160u8];
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
    /**Function with signature `renounceRole(bytes32,address)` and selector `0x36568abe`.
```solidity
function renounceRole(bytes32 role, address callerConfirmation) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceRoleCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub callerConfirmation: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`renounceRole(bytes32,address)`](renounceRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceRoleReturn {}
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
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<renounceRoleCall> for UnderlyingRustTuple<'_> {
                fn from(value: renounceRoleCall) -> Self {
                    (value.role, value.callerConfirmation)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for renounceRoleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        role: tuple.0,
                        callerConfirmation: tuple.1,
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
            impl ::core::convert::From<renounceRoleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: renounceRoleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for renounceRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceRoleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceRoleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "renounceRole(bytes32,address)";
            const SELECTOR: [u8; 4] = [54u8, 86u8, 138u8, 190u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.callerConfirmation,
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
    /**Function with signature `revokeRole(bytes32,address)` and selector `0xd547741f`.
```solidity
function revokeRole(bytes32 role, address account) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct revokeRoleCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`revokeRole(bytes32,address)`](revokeRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct revokeRoleReturn {}
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
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<revokeRoleCall> for UnderlyingRustTuple<'_> {
                fn from(value: revokeRoleCall) -> Self {
                    (value.role, value.account)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for revokeRoleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        role: tuple.0,
                        account: tuple.1,
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
            impl ::core::convert::From<revokeRoleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: revokeRoleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for revokeRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for revokeRoleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = revokeRoleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "revokeRole(bytes32,address)";
            const SELECTOR: [u8; 4] = [213u8, 71u8, 116u8, 31u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
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
    /**Function with signature `schedule(address,uint256,bytes,bytes32,bytes32,uint256)` and selector `0x01d5062a`.
```solidity
function schedule(address target, uint256 value, bytes memory data, bytes32 predecessor, bytes32 salt, uint256 delay) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct scheduleCall {
        #[allow(missing_docs)]
        pub target: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub predecessor: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub salt: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub delay: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`schedule(address,uint256,bytes,bytes32,bytes32,uint256)`](scheduleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct scheduleReturn {}
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
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<scheduleCall> for UnderlyingRustTuple<'_> {
                fn from(value: scheduleCall) -> Self {
                    (
                        value.target,
                        value.value,
                        value.data,
                        value.predecessor,
                        value.salt,
                        value.delay,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for scheduleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        target: tuple.0,
                        value: tuple.1,
                        data: tuple.2,
                        predecessor: tuple.3,
                        salt: tuple.4,
                        delay: tuple.5,
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
            impl ::core::convert::From<scheduleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: scheduleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for scheduleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for scheduleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = scheduleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "schedule(address,uint256,bytes,bytes32,bytes32,uint256)";
            const SELECTOR: [u8; 4] = [1u8, 213u8, 6u8, 42u8];
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.predecessor),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.delay),
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
    /**Function with signature `scheduleBatch(address[],uint256[],bytes[],bytes32,bytes32,uint256)` and selector `0x8f2a0bb0`.
```solidity
function scheduleBatch(address[] memory targets, uint256[] memory values, bytes[] memory payloads, bytes32 predecessor, bytes32 salt, uint256 delay) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct scheduleBatchCall {
        #[allow(missing_docs)]
        pub targets: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        #[allow(missing_docs)]
        pub values: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        #[allow(missing_docs)]
        pub payloads: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
        #[allow(missing_docs)]
        pub predecessor: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub salt: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub delay: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`scheduleBatch(address[],uint256[],bytes[],bytes32,bytes32,uint256)`](scheduleBatchCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct scheduleBatchReturn {}
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
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<scheduleBatchCall> for UnderlyingRustTuple<'_> {
                fn from(value: scheduleBatchCall) -> Self {
                    (
                        value.targets,
                        value.values,
                        value.payloads,
                        value.predecessor,
                        value.salt,
                        value.delay,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for scheduleBatchCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targets: tuple.0,
                        values: tuple.1,
                        payloads: tuple.2,
                        predecessor: tuple.3,
                        salt: tuple.4,
                        delay: tuple.5,
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
            impl ::core::convert::From<scheduleBatchReturn> for UnderlyingRustTuple<'_> {
                fn from(value: scheduleBatchReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for scheduleBatchReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for scheduleBatchCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = scheduleBatchReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "scheduleBatch(address[],uint256[],bytes[],bytes32,bytes32,uint256)";
            const SELECTOR: [u8; 4] = [143u8, 42u8, 11u8, 176u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.payloads),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.predecessor),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.delay),
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
    /**Function with signature `updateDelay(uint256)` and selector `0x64d62353`.
```solidity
function updateDelay(uint256 newDelay) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateDelayCall {
        #[allow(missing_docs)]
        pub newDelay: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`updateDelay(uint256)`](updateDelayCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateDelayReturn {}
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
            impl ::core::convert::From<updateDelayCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateDelayCall) -> Self {
                    (value.newDelay,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateDelayCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newDelay: tuple.0 }
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
            impl ::core::convert::From<updateDelayReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateDelayReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateDelayReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateDelayCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateDelayReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateDelay(uint256)";
            const SELECTOR: [u8; 4] = [100u8, 214u8, 35u8, 83u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.newDelay),
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
    ///Container for all the [`TangleTimelock`](self) function calls.
    pub enum TangleTimelockCalls {
        #[allow(missing_docs)]
        CANCELLER_ROLE(CANCELLER_ROLECall),
        #[allow(missing_docs)]
        DEFAULT_ADMIN_ROLE(DEFAULT_ADMIN_ROLECall),
        #[allow(missing_docs)]
        EXECUTOR_ROLE(EXECUTOR_ROLECall),
        #[allow(missing_docs)]
        MAX_DELAY(MAX_DELAYCall),
        #[allow(missing_docs)]
        MIN_DELAY(MIN_DELAYCall),
        #[allow(missing_docs)]
        PROPOSER_ROLE(PROPOSER_ROLECall),
        #[allow(missing_docs)]
        UPGRADE_INTERFACE_VERSION(UPGRADE_INTERFACE_VERSIONCall),
        #[allow(missing_docs)]
        cancel(cancelCall),
        #[allow(missing_docs)]
        execute(executeCall),
        #[allow(missing_docs)]
        executeBatch(executeBatchCall),
        #[allow(missing_docs)]
        getMinDelay(getMinDelayCall),
        #[allow(missing_docs)]
        getOperationState(getOperationStateCall),
        #[allow(missing_docs)]
        getRoleAdmin(getRoleAdminCall),
        #[allow(missing_docs)]
        getTimestamp(getTimestampCall),
        #[allow(missing_docs)]
        grantRole(grantRoleCall),
        #[allow(missing_docs)]
        hasRole(hasRoleCall),
        #[allow(missing_docs)]
        hashOperation(hashOperationCall),
        #[allow(missing_docs)]
        hashOperationBatch(hashOperationBatchCall),
        #[allow(missing_docs)]
        initialize(initializeCall),
        #[allow(missing_docs)]
        isCanceller(isCancellerCall),
        #[allow(missing_docs)]
        isExecutor(isExecutorCall),
        #[allow(missing_docs)]
        isOperation(isOperationCall),
        #[allow(missing_docs)]
        isOperationDone(isOperationDoneCall),
        #[allow(missing_docs)]
        isOperationPending(isOperationPendingCall),
        #[allow(missing_docs)]
        isOperationReady(isOperationReadyCall),
        #[allow(missing_docs)]
        isProposer(isProposerCall),
        #[allow(missing_docs)]
        onERC1155BatchReceived(onERC1155BatchReceivedCall),
        #[allow(missing_docs)]
        onERC1155Received(onERC1155ReceivedCall),
        #[allow(missing_docs)]
        onERC721Received(onERC721ReceivedCall),
        #[allow(missing_docs)]
        proxiableUUID(proxiableUUIDCall),
        #[allow(missing_docs)]
        renounceRole(renounceRoleCall),
        #[allow(missing_docs)]
        revokeRole(revokeRoleCall),
        #[allow(missing_docs)]
        schedule(scheduleCall),
        #[allow(missing_docs)]
        scheduleBatch(scheduleBatchCall),
        #[allow(missing_docs)]
        supportsInterface(supportsInterfaceCall),
        #[allow(missing_docs)]
        updateDelay(updateDelayCall),
        #[allow(missing_docs)]
        upgradeToAndCall(upgradeToAndCallCall),
    }
    #[automatically_derived]
    impl TangleTimelockCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [1u8, 213u8, 6u8, 42u8],
            [1u8, 255u8, 201u8, 167u8],
            [7u8, 189u8, 2u8, 101u8],
            [19u8, 64u8, 8u8, 211u8],
            [19u8, 188u8, 159u8, 32u8],
            [21u8, 11u8, 122u8, 2u8],
            [36u8, 138u8, 156u8, 163u8],
            [42u8, 176u8, 245u8, 41u8],
            [47u8, 47u8, 241u8, 93u8],
            [49u8, 213u8, 7u8, 80u8],
            [54u8, 86u8, 138u8, 190u8],
            [65u8, 37u8, 255u8, 144u8],
            [79u8, 30u8, 242u8, 134u8],
            [82u8, 209u8, 144u8, 45u8],
            [88u8, 75u8, 21u8, 62u8],
            [100u8, 214u8, 35u8, 83u8],
            [116u8, 236u8, 41u8, 160u8],
            [121u8, 88u8, 0u8, 76u8],
            [128u8, 101u8, 101u8, 127u8],
            [143u8, 42u8, 11u8, 176u8],
            [143u8, 97u8, 244u8, 245u8],
            [145u8, 209u8, 72u8, 84u8],
            [159u8, 129u8, 174u8, 215u8],
            [162u8, 23u8, 253u8, 223u8],
            [173u8, 60u8, 177u8, 204u8],
            [176u8, 142u8, 81u8, 192u8],
            [177u8, 197u8, 244u8, 39u8],
            [180u8, 38u8, 71u8, 94u8],
            [188u8, 25u8, 124u8, 129u8],
            [196u8, 196u8, 199u8, 179u8],
            [196u8, 210u8, 82u8, 245u8],
            [212u8, 92u8, 68u8, 53u8],
            [213u8, 71u8, 116u8, 31u8],
            [222u8, 191u8, 218u8, 48u8],
            [227u8, 131u8, 53u8, 229u8],
            [242u8, 58u8, 110u8, 97u8],
            [242u8, 122u8, 12u8, 146u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for TangleTimelockCalls {
        const NAME: &'static str = "TangleTimelockCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 37usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::CANCELLER_ROLE(_) => {
                    <CANCELLER_ROLECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::DEFAULT_ADMIN_ROLE(_) => {
                    <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::EXECUTOR_ROLE(_) => {
                    <EXECUTOR_ROLECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::MAX_DELAY(_) => {
                    <MAX_DELAYCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::MIN_DELAY(_) => {
                    <MIN_DELAYCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::PROPOSER_ROLE(_) => {
                    <PROPOSER_ROLECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::UPGRADE_INTERFACE_VERSION(_) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::cancel(_) => <cancelCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::execute(_) => <executeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::executeBatch(_) => {
                    <executeBatchCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getMinDelay(_) => {
                    <getMinDelayCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperationState(_) => {
                    <getOperationStateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getRoleAdmin(_) => {
                    <getRoleAdminCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTimestamp(_) => {
                    <getTimestampCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::grantRole(_) => {
                    <grantRoleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::hasRole(_) => <hasRoleCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::hashOperation(_) => {
                    <hashOperationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::hashOperationBatch(_) => {
                    <hashOperationBatchCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isCanceller(_) => {
                    <isCancellerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isExecutor(_) => {
                    <isExecutorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isOperation(_) => {
                    <isOperationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isOperationDone(_) => {
                    <isOperationDoneCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isOperationPending(_) => {
                    <isOperationPendingCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isOperationReady(_) => {
                    <isOperationReadyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isProposer(_) => {
                    <isProposerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::onERC1155BatchReceived(_) => {
                    <onERC1155BatchReceivedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::onERC1155Received(_) => {
                    <onERC1155ReceivedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::onERC721Received(_) => {
                    <onERC721ReceivedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proxiableUUID(_) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceRole(_) => {
                    <renounceRoleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::revokeRole(_) => {
                    <revokeRoleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::schedule(_) => <scheduleCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::scheduleBatch(_) => {
                    <scheduleBatchCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::supportsInterface(_) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateDelay(_) => {
                    <updateDelayCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::upgradeToAndCall(_) => {
                    <upgradeToAndCallCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<TangleTimelockCalls>] = &[
                {
                    fn schedule(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockCalls> {
                        <scheduleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockCalls::schedule)
                    }
                    schedule
                },
                {
                    fn supportsInterface(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockCalls> {
                        <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockCalls::supportsInterface)
                    }
                    supportsInterface
                },
                {
                    fn EXECUTOR_ROLE(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockCalls> {
                        <EXECUTOR_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockCalls::EXECUTOR_ROLE)
                    }
                    EXECUTOR_ROLE
                },
                {
                    fn execute(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockCalls> {
                        <executeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockCalls::execute)
                    }
                    execute
                },
                {
                    fn isOperationReady(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockCalls> {
                        <isOperationReadyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockCalls::isOperationReady)
                    }
                    isOperationReady
                },
                {
                    fn onERC721Received(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockCalls> {
                        <onERC721ReceivedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockCalls::onERC721Received)
                    }
                    onERC721Received
                },
                {
                    fn getRoleAdmin(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockCalls> {
                        <getRoleAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockCalls::getRoleAdmin)
                    }
                    getRoleAdmin
                },
                {
                    fn isOperationDone(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockCalls> {
                        <isOperationDoneCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockCalls::isOperationDone)
                    }
                    isOperationDone
                },
                {
                    fn grantRole(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockCalls> {
                        <grantRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockCalls::grantRole)
                    }
                    grantRole
                },
                {
                    fn isOperation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockCalls> {
                        <isOperationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockCalls::isOperation)
                    }
                    isOperation
                },
                {
                    fn renounceRole(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockCalls> {
                        <renounceRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockCalls::renounceRole)
                    }
                    renounceRole
                },
                {
                    fn MAX_DELAY(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockCalls> {
                        <MAX_DELAYCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockCalls::MAX_DELAY)
                    }
                    MAX_DELAY
                },
                {
                    fn upgradeToAndCall(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockCalls> {
                        <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockCalls::upgradeToAndCall)
                    }
                    upgradeToAndCall
                },
                {
                    fn proxiableUUID(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockCalls> {
                        <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockCalls::proxiableUUID)
                    }
                    proxiableUUID
                },
                {
                    fn isOperationPending(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockCalls> {
                        <isOperationPendingCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockCalls::isOperationPending)
                    }
                    isOperationPending
                },
                {
                    fn updateDelay(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockCalls> {
                        <updateDelayCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockCalls::updateDelay)
                    }
                    updateDelay
                },
                {
                    fn isProposer(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockCalls> {
                        <isProposerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockCalls::isProposer)
                    }
                    isProposer
                },
                {
                    fn getOperationState(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockCalls> {
                        <getOperationStateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockCalls::getOperationState)
                    }
                    getOperationState
                },
                {
                    fn hashOperation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockCalls> {
                        <hashOperationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockCalls::hashOperation)
                    }
                    hashOperation
                },
                {
                    fn scheduleBatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockCalls> {
                        <scheduleBatchCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockCalls::scheduleBatch)
                    }
                    scheduleBatch
                },
                {
                    fn PROPOSER_ROLE(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockCalls> {
                        <PROPOSER_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockCalls::PROPOSER_ROLE)
                    }
                    PROPOSER_ROLE
                },
                {
                    fn hasRole(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockCalls> {
                        <hasRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockCalls::hasRole)
                    }
                    hasRole
                },
                {
                    fn MIN_DELAY(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockCalls> {
                        <MIN_DELAYCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockCalls::MIN_DELAY)
                    }
                    MIN_DELAY
                },
                {
                    fn DEFAULT_ADMIN_ROLE(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockCalls> {
                        <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockCalls::DEFAULT_ADMIN_ROLE)
                    }
                    DEFAULT_ADMIN_ROLE
                },
                {
                    fn UPGRADE_INTERFACE_VERSION(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockCalls> {
                        <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockCalls::UPGRADE_INTERFACE_VERSION)
                    }
                    UPGRADE_INTERFACE_VERSION
                },
                {
                    fn CANCELLER_ROLE(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockCalls> {
                        <CANCELLER_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockCalls::CANCELLER_ROLE)
                    }
                    CANCELLER_ROLE
                },
                {
                    fn hashOperationBatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockCalls> {
                        <hashOperationBatchCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockCalls::hashOperationBatch)
                    }
                    hashOperationBatch
                },
                {
                    fn isCanceller(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockCalls> {
                        <isCancellerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockCalls::isCanceller)
                    }
                    isCanceller
                },
                {
                    fn onERC1155BatchReceived(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockCalls> {
                        <onERC1155BatchReceivedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockCalls::onERC1155BatchReceived)
                    }
                    onERC1155BatchReceived
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockCalls::initialize)
                    }
                    initialize
                },
                {
                    fn cancel(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockCalls> {
                        <cancelCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockCalls::cancel)
                    }
                    cancel
                },
                {
                    fn getTimestamp(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockCalls> {
                        <getTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockCalls::getTimestamp)
                    }
                    getTimestamp
                },
                {
                    fn revokeRole(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockCalls> {
                        <revokeRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockCalls::revokeRole)
                    }
                    revokeRole
                },
                {
                    fn isExecutor(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockCalls> {
                        <isExecutorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockCalls::isExecutor)
                    }
                    isExecutor
                },
                {
                    fn executeBatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockCalls> {
                        <executeBatchCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockCalls::executeBatch)
                    }
                    executeBatch
                },
                {
                    fn onERC1155Received(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockCalls> {
                        <onERC1155ReceivedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockCalls::onERC1155Received)
                    }
                    onERC1155Received
                },
                {
                    fn getMinDelay(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockCalls> {
                        <getMinDelayCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockCalls::getMinDelay)
                    }
                    getMinDelay
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
                Self::CANCELLER_ROLE(inner) => {
                    <CANCELLER_ROLECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DEFAULT_ADMIN_ROLE(inner) => {
                    <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EXECUTOR_ROLE(inner) => {
                    <EXECUTOR_ROLECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MAX_DELAY(inner) => {
                    <MAX_DELAYCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::MIN_DELAY(inner) => {
                    <MIN_DELAYCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::PROPOSER_ROLE(inner) => {
                    <PROPOSER_ROLECall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::execute(inner) => {
                    <executeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::executeBatch(inner) => {
                    <executeBatchCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getMinDelay(inner) => {
                    <getMinDelayCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperationState(inner) => {
                    <getOperationStateCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getRoleAdmin(inner) => {
                    <getRoleAdminCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getTimestamp(inner) => {
                    <getTimestampCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::grantRole(inner) => {
                    <grantRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::hasRole(inner) => {
                    <hasRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::hashOperation(inner) => {
                    <hashOperationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::hashOperationBatch(inner) => {
                    <hashOperationBatchCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isCanceller(inner) => {
                    <isCancellerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isExecutor(inner) => {
                    <isExecutorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isOperation(inner) => {
                    <isOperationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isOperationDone(inner) => {
                    <isOperationDoneCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isOperationPending(inner) => {
                    <isOperationPendingCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isOperationReady(inner) => {
                    <isOperationReadyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isProposer(inner) => {
                    <isProposerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::proxiableUUID(inner) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceRole(inner) => {
                    <renounceRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::revokeRole(inner) => {
                    <revokeRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::schedule(inner) => {
                    <scheduleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::scheduleBatch(inner) => {
                    <scheduleBatchCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::supportsInterface(inner) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateDelay(inner) => {
                    <updateDelayCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::upgradeToAndCall(inner) => {
                    <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::CANCELLER_ROLE(inner) => {
                    <CANCELLER_ROLECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DEFAULT_ADMIN_ROLE(inner) => {
                    <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EXECUTOR_ROLE(inner) => {
                    <EXECUTOR_ROLECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MAX_DELAY(inner) => {
                    <MAX_DELAYCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MIN_DELAY(inner) => {
                    <MIN_DELAYCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::PROPOSER_ROLE(inner) => {
                    <PROPOSER_ROLECall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::execute(inner) => {
                    <executeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::executeBatch(inner) => {
                    <executeBatchCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getMinDelay(inner) => {
                    <getMinDelayCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperationState(inner) => {
                    <getOperationStateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getRoleAdmin(inner) => {
                    <getRoleAdminCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getTimestamp(inner) => {
                    <getTimestampCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::grantRole(inner) => {
                    <grantRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::hasRole(inner) => {
                    <hasRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::hashOperation(inner) => {
                    <hashOperationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::hashOperationBatch(inner) => {
                    <hashOperationBatchCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::isCanceller(inner) => {
                    <isCancellerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isExecutor(inner) => {
                    <isExecutorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isOperation(inner) => {
                    <isOperationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isOperationDone(inner) => {
                    <isOperationDoneCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isOperationPending(inner) => {
                    <isOperationPendingCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isOperationReady(inner) => {
                    <isOperationReadyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isProposer(inner) => {
                    <isProposerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
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
                Self::proxiableUUID(inner) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::renounceRole(inner) => {
                    <renounceRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::revokeRole(inner) => {
                    <revokeRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::schedule(inner) => {
                    <scheduleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::scheduleBatch(inner) => {
                    <scheduleBatchCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::supportsInterface(inner) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateDelay(inner) => {
                    <updateDelayCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
            }
        }
    }
    ///Container for all the [`TangleTimelock`](self) custom errors.
    pub enum TangleTimelockErrors {
        #[allow(missing_docs)]
        AccessControlBadConfirmation(AccessControlBadConfirmation),
        #[allow(missing_docs)]
        AccessControlUnauthorizedAccount(AccessControlUnauthorizedAccount),
        #[allow(missing_docs)]
        AddressEmptyCode(AddressEmptyCode),
        #[allow(missing_docs)]
        ERC1967InvalidImplementation(ERC1967InvalidImplementation),
        #[allow(missing_docs)]
        ERC1967NonPayable(ERC1967NonPayable),
        #[allow(missing_docs)]
        FailedCall(FailedCall),
        #[allow(missing_docs)]
        InvalidInitialization(InvalidInitialization),
        #[allow(missing_docs)]
        NotInitializing(NotInitializing),
        #[allow(missing_docs)]
        TimelockInsufficientDelay(TimelockInsufficientDelay),
        #[allow(missing_docs)]
        TimelockInvalidOperationLength(TimelockInvalidOperationLength),
        #[allow(missing_docs)]
        TimelockUnauthorizedCaller(TimelockUnauthorizedCaller),
        #[allow(missing_docs)]
        TimelockUnexecutedPredecessor(TimelockUnexecutedPredecessor),
        #[allow(missing_docs)]
        TimelockUnexpectedOperationState(TimelockUnexpectedOperationState),
        #[allow(missing_docs)]
        UUPSUnauthorizedCallContext(UUPSUnauthorizedCallContext),
        #[allow(missing_docs)]
        UUPSUnsupportedProxiableUUID(UUPSUnsupportedProxiableUUID),
    }
    #[automatically_derived]
    impl TangleTimelockErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [76u8, 156u8, 140u8, 227u8],
            [84u8, 51u8, 102u8, 9u8],
            [94u8, 173u8, 142u8, 181u8],
            [102u8, 151u8, 178u8, 50u8],
            [144u8, 169u8, 166u8, 24u8],
            [153u8, 150u8, 179u8, 21u8],
            [170u8, 29u8, 73u8, 164u8],
            [179u8, 152u8, 151u8, 159u8],
            [214u8, 189u8, 162u8, 117u8],
            [215u8, 230u8, 188u8, 248u8],
            [224u8, 124u8, 141u8, 186u8],
            [226u8, 81u8, 125u8, 63u8],
            [226u8, 133u8, 12u8, 89u8],
            [249u8, 46u8, 232u8, 169u8],
            [255u8, 176u8, 50u8, 17u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for TangleTimelockErrors {
        const NAME: &'static str = "TangleTimelockErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 15usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AccessControlBadConfirmation(_) => {
                    <AccessControlBadConfirmation as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AccessControlUnauthorizedAccount(_) => {
                    <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AddressEmptyCode(_) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::SELECTOR
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
                Self::InvalidInitialization(_) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotInitializing(_) => {
                    <NotInitializing as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TimelockInsufficientDelay(_) => {
                    <TimelockInsufficientDelay as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TimelockInvalidOperationLength(_) => {
                    <TimelockInvalidOperationLength as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TimelockUnauthorizedCaller(_) => {
                    <TimelockUnauthorizedCaller as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TimelockUnexecutedPredecessor(_) => {
                    <TimelockUnexecutedPredecessor as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TimelockUnexpectedOperationState(_) => {
                    <TimelockUnexpectedOperationState as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<TangleTimelockErrors>] = &[
                {
                    fn ERC1967InvalidImplementation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockErrors> {
                        <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockErrors::ERC1967InvalidImplementation)
                    }
                    ERC1967InvalidImplementation
                },
                {
                    fn TimelockInsufficientDelay(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockErrors> {
                        <TimelockInsufficientDelay as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockErrors::TimelockInsufficientDelay)
                    }
                    TimelockInsufficientDelay
                },
                {
                    fn TimelockUnexpectedOperationState(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockErrors> {
                        <TimelockUnexpectedOperationState as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockErrors::TimelockUnexpectedOperationState)
                    }
                    TimelockUnexpectedOperationState
                },
                {
                    fn AccessControlBadConfirmation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockErrors> {
                        <AccessControlBadConfirmation as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockErrors::AccessControlBadConfirmation)
                    }
                    AccessControlBadConfirmation
                },
                {
                    fn TimelockUnexecutedPredecessor(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockErrors> {
                        <TimelockUnexecutedPredecessor as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockErrors::TimelockUnexecutedPredecessor)
                    }
                    TimelockUnexecutedPredecessor
                },
                {
                    fn AddressEmptyCode(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockErrors> {
                        <AddressEmptyCode as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockErrors::AddressEmptyCode)
                    }
                    AddressEmptyCode
                },
                {
                    fn UUPSUnsupportedProxiableUUID(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockErrors> {
                        <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockErrors::UUPSUnsupportedProxiableUUID)
                    }
                    UUPSUnsupportedProxiableUUID
                },
                {
                    fn ERC1967NonPayable(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockErrors> {
                        <ERC1967NonPayable as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockErrors::ERC1967NonPayable)
                    }
                    ERC1967NonPayable
                },
                {
                    fn FailedCall(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockErrors> {
                        <FailedCall as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockErrors::FailedCall)
                    }
                    FailedCall
                },
                {
                    fn NotInitializing(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockErrors> {
                        <NotInitializing as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockErrors::NotInitializing)
                    }
                    NotInitializing
                },
                {
                    fn UUPSUnauthorizedCallContext(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockErrors> {
                        <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockErrors::UUPSUnauthorizedCallContext)
                    }
                    UUPSUnauthorizedCallContext
                },
                {
                    fn AccessControlUnauthorizedAccount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockErrors> {
                        <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockErrors::AccessControlUnauthorizedAccount)
                    }
                    AccessControlUnauthorizedAccount
                },
                {
                    fn TimelockUnauthorizedCaller(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockErrors> {
                        <TimelockUnauthorizedCaller as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockErrors::TimelockUnauthorizedCaller)
                    }
                    TimelockUnauthorizedCaller
                },
                {
                    fn InvalidInitialization(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockErrors> {
                        <InvalidInitialization as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockErrors::InvalidInitialization)
                    }
                    InvalidInitialization
                },
                {
                    fn TimelockInvalidOperationLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TangleTimelockErrors> {
                        <TimelockInvalidOperationLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TangleTimelockErrors::TimelockInvalidOperationLength)
                    }
                    TimelockInvalidOperationLength
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
                Self::AccessControlBadConfirmation(inner) => {
                    <AccessControlBadConfirmation as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::AccessControlUnauthorizedAccount(inner) => {
                    <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::AddressEmptyCode(inner) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::TimelockInsufficientDelay(inner) => {
                    <TimelockInsufficientDelay as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TimelockInvalidOperationLength(inner) => {
                    <TimelockInvalidOperationLength as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TimelockUnauthorizedCaller(inner) => {
                    <TimelockUnauthorizedCaller as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TimelockUnexecutedPredecessor(inner) => {
                    <TimelockUnexecutedPredecessor as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TimelockUnexpectedOperationState(inner) => {
                    <TimelockUnexpectedOperationState as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::AccessControlBadConfirmation(inner) => {
                    <AccessControlBadConfirmation as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::AccessControlUnauthorizedAccount(inner) => {
                    <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::AddressEmptyCode(inner) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::TimelockInsufficientDelay(inner) => {
                    <TimelockInsufficientDelay as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TimelockInvalidOperationLength(inner) => {
                    <TimelockInvalidOperationLength as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TimelockUnauthorizedCaller(inner) => {
                    <TimelockUnauthorizedCaller as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TimelockUnexecutedPredecessor(inner) => {
                    <TimelockUnexecutedPredecessor as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TimelockUnexpectedOperationState(inner) => {
                    <TimelockUnexpectedOperationState as alloy_sol_types::SolError>::abi_encode_raw(
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
    ///Container for all the [`TangleTimelock`](self) events.
    pub enum TangleTimelockEvents {
        #[allow(missing_docs)]
        CallExecuted(CallExecuted),
        #[allow(missing_docs)]
        CallSalt(CallSalt),
        #[allow(missing_docs)]
        CallScheduled(CallScheduled),
        #[allow(missing_docs)]
        Cancelled(Cancelled),
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        MinDelayChange(MinDelayChange),
        #[allow(missing_docs)]
        RoleAdminChanged(RoleAdminChanged),
        #[allow(missing_docs)]
        RoleGranted(RoleGranted),
        #[allow(missing_docs)]
        RoleRevoked(RoleRevoked),
        #[allow(missing_docs)]
        Upgraded(Upgraded),
    }
    #[automatically_derived]
    impl TangleTimelockEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                17u8,
                194u8,
                79u8,
                78u8,
                173u8,
                22u8,
                80u8,
                124u8,
                105u8,
                172u8,
                70u8,
                127u8,
                189u8,
                94u8,
                78u8,
                237u8,
                95u8,
                181u8,
                198u8,
                153u8,
                98u8,
                109u8,
                44u8,
                198u8,
                214u8,
                100u8,
                33u8,
                223u8,
                37u8,
                56u8,
                134u8,
                213u8,
            ],
            [
                32u8,
                253u8,
                165u8,
                253u8,
                39u8,
                161u8,
                234u8,
                123u8,
                245u8,
                185u8,
                86u8,
                127u8,
                20u8,
                58u8,
                197u8,
                71u8,
                11u8,
                176u8,
                89u8,
                55u8,
                74u8,
                39u8,
                232u8,
                246u8,
                124u8,
                180u8,
                79u8,
                148u8,
                111u8,
                109u8,
                3u8,
                135u8,
            ],
            [
                47u8,
                135u8,
                136u8,
                17u8,
                126u8,
                126u8,
                255u8,
                29u8,
                130u8,
                233u8,
                38u8,
                236u8,
                121u8,
                73u8,
                1u8,
                209u8,
                124u8,
                120u8,
                2u8,
                74u8,
                80u8,
                39u8,
                9u8,
                64u8,
                48u8,
                69u8,
                64u8,
                167u8,
                51u8,
                101u8,
                111u8,
                13u8,
            ],
            [
                76u8,
                244u8,
                65u8,
                12u8,
                197u8,
                112u8,
                64u8,
                228u8,
                72u8,
                98u8,
                239u8,
                15u8,
                69u8,
                243u8,
                221u8,
                90u8,
                94u8,
                2u8,
                219u8,
                142u8,
                184u8,
                173u8,
                214u8,
                72u8,
                212u8,
                176u8,
                226u8,
                54u8,
                241u8,
                208u8,
                125u8,
                202u8,
            ],
            [
                186u8,
                161u8,
                235u8,
                34u8,
                242u8,
                164u8,
                146u8,
                186u8,
                26u8,
                95u8,
                234u8,
                97u8,
                184u8,
                223u8,
                77u8,
                39u8,
                198u8,
                200u8,
                181u8,
                243u8,
                151u8,
                30u8,
                99u8,
                187u8,
                88u8,
                250u8,
                20u8,
                255u8,
                114u8,
                238u8,
                219u8,
                112u8,
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
                189u8,
                121u8,
                184u8,
                111u8,
                254u8,
                10u8,
                184u8,
                232u8,
                119u8,
                97u8,
                81u8,
                81u8,
                66u8,
                23u8,
                205u8,
                124u8,
                172u8,
                213u8,
                44u8,
                144u8,
                159u8,
                102u8,
                71u8,
                92u8,
                58u8,
                244u8,
                78u8,
                18u8,
                159u8,
                11u8,
                0u8,
                255u8,
            ],
            [
                194u8,
                97u8,
                126u8,
                250u8,
                105u8,
                186u8,
                182u8,
                103u8,
                130u8,
                250u8,
                33u8,
                149u8,
                67u8,
                113u8,
                67u8,
                56u8,
                72u8,
                156u8,
                78u8,
                158u8,
                23u8,
                130u8,
                113u8,
                86u8,
                10u8,
                145u8,
                184u8,
                44u8,
                63u8,
                97u8,
                43u8,
                88u8,
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
                246u8,
                57u8,
                31u8,
                92u8,
                50u8,
                217u8,
                198u8,
                157u8,
                42u8,
                71u8,
                234u8,
                103u8,
                11u8,
                68u8,
                41u8,
                116u8,
                181u8,
                57u8,
                53u8,
                209u8,
                237u8,
                199u8,
                253u8,
                100u8,
                235u8,
                33u8,
                224u8,
                71u8,
                168u8,
                57u8,
                23u8,
                27u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for TangleTimelockEvents {
        const NAME: &'static str = "TangleTimelockEvents";
        const COUNT: usize = 10usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<CallExecuted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <CallExecuted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::CallExecuted)
                }
                Some(<CallSalt as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <CallSalt as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::CallSalt)
                }
                Some(<CallScheduled as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <CallScheduled as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::CallScheduled)
                }
                Some(<Cancelled as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Cancelled as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Cancelled)
                }
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Initialized)
                }
                Some(<MinDelayChange as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <MinDelayChange as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::MinDelayChange)
                }
                Some(<RoleAdminChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RoleAdminChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::RoleAdminChanged)
                }
                Some(<RoleGranted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RoleGranted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::RoleGranted)
                }
                Some(<RoleRevoked as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RoleRevoked as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::RoleRevoked)
                }
                Some(<Upgraded as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Upgraded as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Upgraded)
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
    impl alloy_sol_types::private::IntoLogData for TangleTimelockEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::CallExecuted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::CallSalt(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::CallScheduled(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Cancelled(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::MinDelayChange(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RoleAdminChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RoleGranted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RoleRevoked(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::CallExecuted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::CallSalt(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::CallScheduled(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Cancelled(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::MinDelayChange(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RoleAdminChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RoleGranted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RoleRevoked(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`TangleTimelock`](self) contract instance.

See the [wrapper's documentation](`TangleTimelockInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> TangleTimelockInstance<T, P, N> {
        TangleTimelockInstance::<T, P, N>::new(address, provider)
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
        Output = alloy_contract::Result<TangleTimelockInstance<T, P, N>>,
    > {
        TangleTimelockInstance::<T, P, N>::deploy(provider)
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
        TangleTimelockInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`TangleTimelock`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`TangleTimelock`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct TangleTimelockInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for TangleTimelockInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("TangleTimelockInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > TangleTimelockInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`TangleTimelock`](self) contract instance.

See the [wrapper's documentation](`TangleTimelockInstance`) for more details.*/
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
        ) -> alloy_contract::Result<TangleTimelockInstance<T, P, N>> {
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
    impl<T, P: ::core::clone::Clone, N> TangleTimelockInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> TangleTimelockInstance<T, P, N> {
            TangleTimelockInstance {
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
    > TangleTimelockInstance<T, P, N> {
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
        ///Creates a new call builder for the [`CANCELLER_ROLE`] function.
        pub fn CANCELLER_ROLE(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, CANCELLER_ROLECall, N> {
            self.call_builder(&CANCELLER_ROLECall {})
        }
        ///Creates a new call builder for the [`DEFAULT_ADMIN_ROLE`] function.
        pub fn DEFAULT_ADMIN_ROLE(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, DEFAULT_ADMIN_ROLECall, N> {
            self.call_builder(&DEFAULT_ADMIN_ROLECall {})
        }
        ///Creates a new call builder for the [`EXECUTOR_ROLE`] function.
        pub fn EXECUTOR_ROLE(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, EXECUTOR_ROLECall, N> {
            self.call_builder(&EXECUTOR_ROLECall {})
        }
        ///Creates a new call builder for the [`MAX_DELAY`] function.
        pub fn MAX_DELAY(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, MAX_DELAYCall, N> {
            self.call_builder(&MAX_DELAYCall {})
        }
        ///Creates a new call builder for the [`MIN_DELAY`] function.
        pub fn MIN_DELAY(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, MIN_DELAYCall, N> {
            self.call_builder(&MIN_DELAYCall {})
        }
        ///Creates a new call builder for the [`PROPOSER_ROLE`] function.
        pub fn PROPOSER_ROLE(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, PROPOSER_ROLECall, N> {
            self.call_builder(&PROPOSER_ROLECall {})
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
            id: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, cancelCall, N> {
            self.call_builder(&cancelCall { id })
        }
        ///Creates a new call builder for the [`execute`] function.
        pub fn execute(
            &self,
            target: alloy::sol_types::private::Address,
            value: alloy::sol_types::private::primitives::aliases::U256,
            payload: alloy::sol_types::private::Bytes,
            predecessor: alloy::sol_types::private::FixedBytes<32>,
            salt: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, executeCall, N> {
            self.call_builder(
                &executeCall {
                    target,
                    value,
                    payload,
                    predecessor,
                    salt,
                },
            )
        }
        ///Creates a new call builder for the [`executeBatch`] function.
        pub fn executeBatch(
            &self,
            targets: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            values: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
            payloads: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
            predecessor: alloy::sol_types::private::FixedBytes<32>,
            salt: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, executeBatchCall, N> {
            self.call_builder(
                &executeBatchCall {
                    targets,
                    values,
                    payloads,
                    predecessor,
                    salt,
                },
            )
        }
        ///Creates a new call builder for the [`getMinDelay`] function.
        pub fn getMinDelay(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getMinDelayCall, N> {
            self.call_builder(&getMinDelayCall {})
        }
        ///Creates a new call builder for the [`getOperationState`] function.
        pub fn getOperationState(
            &self,
            id: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperationStateCall, N> {
            self.call_builder(&getOperationStateCall { id })
        }
        ///Creates a new call builder for the [`getRoleAdmin`] function.
        pub fn getRoleAdmin(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getRoleAdminCall, N> {
            self.call_builder(&getRoleAdminCall { role })
        }
        ///Creates a new call builder for the [`getTimestamp`] function.
        pub fn getTimestamp(
            &self,
            id: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTimestampCall, N> {
            self.call_builder(&getTimestampCall { id })
        }
        ///Creates a new call builder for the [`grantRole`] function.
        pub fn grantRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, grantRoleCall, N> {
            self.call_builder(&grantRoleCall { role, account })
        }
        ///Creates a new call builder for the [`hasRole`] function.
        pub fn hasRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, hasRoleCall, N> {
            self.call_builder(&hasRoleCall { role, account })
        }
        ///Creates a new call builder for the [`hashOperation`] function.
        pub fn hashOperation(
            &self,
            target: alloy::sol_types::private::Address,
            value: alloy::sol_types::private::primitives::aliases::U256,
            data: alloy::sol_types::private::Bytes,
            predecessor: alloy::sol_types::private::FixedBytes<32>,
            salt: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, hashOperationCall, N> {
            self.call_builder(
                &hashOperationCall {
                    target,
                    value,
                    data,
                    predecessor,
                    salt,
                },
            )
        }
        ///Creates a new call builder for the [`hashOperationBatch`] function.
        pub fn hashOperationBatch(
            &self,
            targets: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            values: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
            payloads: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
            predecessor: alloy::sol_types::private::FixedBytes<32>,
            salt: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, hashOperationBatchCall, N> {
            self.call_builder(
                &hashOperationBatchCall {
                    targets,
                    values,
                    payloads,
                    predecessor,
                    salt,
                },
            )
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            minDelay: alloy::sol_types::private::primitives::aliases::U256,
            proposers: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            executors: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            admin: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(
                &initializeCall {
                    minDelay,
                    proposers,
                    executors,
                    admin,
                },
            )
        }
        ///Creates a new call builder for the [`isCanceller`] function.
        pub fn isCanceller(
            &self,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, isCancellerCall, N> {
            self.call_builder(&isCancellerCall { account })
        }
        ///Creates a new call builder for the [`isExecutor`] function.
        pub fn isExecutor(
            &self,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, isExecutorCall, N> {
            self.call_builder(&isExecutorCall { account })
        }
        ///Creates a new call builder for the [`isOperation`] function.
        pub fn isOperation(
            &self,
            id: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, isOperationCall, N> {
            self.call_builder(&isOperationCall { id })
        }
        ///Creates a new call builder for the [`isOperationDone`] function.
        pub fn isOperationDone(
            &self,
            id: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, isOperationDoneCall, N> {
            self.call_builder(&isOperationDoneCall { id })
        }
        ///Creates a new call builder for the [`isOperationPending`] function.
        pub fn isOperationPending(
            &self,
            id: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, isOperationPendingCall, N> {
            self.call_builder(&isOperationPendingCall { id })
        }
        ///Creates a new call builder for the [`isOperationReady`] function.
        pub fn isOperationReady(
            &self,
            id: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, isOperationReadyCall, N> {
            self.call_builder(&isOperationReadyCall { id })
        }
        ///Creates a new call builder for the [`isProposer`] function.
        pub fn isProposer(
            &self,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, isProposerCall, N> {
            self.call_builder(&isProposerCall { account })
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
        ///Creates a new call builder for the [`proxiableUUID`] function.
        pub fn proxiableUUID(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, proxiableUUIDCall, N> {
            self.call_builder(&proxiableUUIDCall {})
        }
        ///Creates a new call builder for the [`renounceRole`] function.
        pub fn renounceRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            callerConfirmation: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceRoleCall, N> {
            self.call_builder(
                &renounceRoleCall {
                    role,
                    callerConfirmation,
                },
            )
        }
        ///Creates a new call builder for the [`revokeRole`] function.
        pub fn revokeRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, revokeRoleCall, N> {
            self.call_builder(&revokeRoleCall { role, account })
        }
        ///Creates a new call builder for the [`schedule`] function.
        pub fn schedule(
            &self,
            target: alloy::sol_types::private::Address,
            value: alloy::sol_types::private::primitives::aliases::U256,
            data: alloy::sol_types::private::Bytes,
            predecessor: alloy::sol_types::private::FixedBytes<32>,
            salt: alloy::sol_types::private::FixedBytes<32>,
            delay: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, scheduleCall, N> {
            self.call_builder(
                &scheduleCall {
                    target,
                    value,
                    data,
                    predecessor,
                    salt,
                    delay,
                },
            )
        }
        ///Creates a new call builder for the [`scheduleBatch`] function.
        pub fn scheduleBatch(
            &self,
            targets: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            values: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
            payloads: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
            predecessor: alloy::sol_types::private::FixedBytes<32>,
            salt: alloy::sol_types::private::FixedBytes<32>,
            delay: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, scheduleBatchCall, N> {
            self.call_builder(
                &scheduleBatchCall {
                    targets,
                    values,
                    payloads,
                    predecessor,
                    salt,
                    delay,
                },
            )
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
        ///Creates a new call builder for the [`updateDelay`] function.
        pub fn updateDelay(
            &self,
            newDelay: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateDelayCall, N> {
            self.call_builder(&updateDelayCall { newDelay })
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
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > TangleTimelockInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`CallExecuted`] event.
        pub fn CallExecuted_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, CallExecuted, N> {
            self.event_filter::<CallExecuted>()
        }
        ///Creates a new event filter for the [`CallSalt`] event.
        pub fn CallSalt_filter(&self) -> alloy_contract::Event<T, &P, CallSalt, N> {
            self.event_filter::<CallSalt>()
        }
        ///Creates a new event filter for the [`CallScheduled`] event.
        pub fn CallScheduled_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, CallScheduled, N> {
            self.event_filter::<CallScheduled>()
        }
        ///Creates a new event filter for the [`Cancelled`] event.
        pub fn Cancelled_filter(&self) -> alloy_contract::Event<T, &P, Cancelled, N> {
            self.event_filter::<Cancelled>()
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`MinDelayChange`] event.
        pub fn MinDelayChange_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, MinDelayChange, N> {
            self.event_filter::<MinDelayChange>()
        }
        ///Creates a new event filter for the [`RoleAdminChanged`] event.
        pub fn RoleAdminChanged_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, RoleAdminChanged, N> {
            self.event_filter::<RoleAdminChanged>()
        }
        ///Creates a new event filter for the [`RoleGranted`] event.
        pub fn RoleGranted_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, RoleGranted, N> {
            self.event_filter::<RoleGranted>()
        }
        ///Creates a new event filter for the [`RoleRevoked`] event.
        pub fn RoleRevoked_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, RoleRevoked, N> {
            self.event_filter::<RoleRevoked>()
        }
        ///Creates a new event filter for the [`Upgraded`] event.
        pub fn Upgraded_filter(&self) -> alloy_contract::Event<T, &P, Upgraded, N> {
            self.event_filter::<Upgraded>()
        }
    }
}
