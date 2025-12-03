///Module containing a contract's types and functions.
/**

```solidity
library IOperatorStatusRegistry {
    type StatusCode is uint8;
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IOperatorStatusRegistry {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StatusCode(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<StatusCode> for u8 {
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
        impl StatusCode {
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
        impl alloy_sol_types::SolType for StatusCode {
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
        impl alloy_sol_types::EventTopic for StatusCode {
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
    /**Creates a new wrapper around an on-chain [`IOperatorStatusRegistry`](self) contract instance.

See the [wrapper's documentation](`IOperatorStatusRegistryInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IOperatorStatusRegistryInstance<T, P, N> {
        IOperatorStatusRegistryInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IOperatorStatusRegistry`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IOperatorStatusRegistry`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IOperatorStatusRegistryInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IOperatorStatusRegistryInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IOperatorStatusRegistryInstance")
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
    > IOperatorStatusRegistryInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IOperatorStatusRegistry`](self) contract instance.

See the [wrapper's documentation](`IOperatorStatusRegistryInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IOperatorStatusRegistryInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IOperatorStatusRegistryInstance<T, P, N> {
            IOperatorStatusRegistryInstance {
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
    > IOperatorStatusRegistryInstance<T, P, N> {
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
    > IOperatorStatusRegistryInstance<T, P, N> {
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
library IOperatorStatusRegistry {
    type StatusCode is uint8;
}

interface OperatorStatusRegistry {
    struct HeartbeatConfig {
        uint64 interval;
        uint8 maxMissed;
        bool customMetrics;
    }
    struct MetricDefinition {
        string name;
        uint256 minValue;
        uint256 maxValue;
        bool required;
    }
    struct OperatorState {
        uint256 lastHeartbeat;
        uint64 consecutiveBeats;
        uint8 missedBeats;
        IOperatorStatusRegistry.StatusCode status;
        bytes32 lastMetricsHash;
    }

    error ECDSAInvalidSignature();
    error ECDSAInvalidSignatureLength(uint256 length);
    error ECDSAInvalidSignatureS(bytes32 s);

    event HeartbeatConfigUpdated(uint64 indexed serviceId, uint64 interval, uint8 maxMissed);
    event HeartbeatReceived(uint64 indexed serviceId, uint64 indexed blueprintId, address indexed operator, uint8 statusCode, uint256 timestamp);
    event MetricReported(uint64 indexed serviceId, address indexed operator, string metricName, uint256 value);
    event OperatorCameOnline(uint64 indexed serviceId, address indexed operator);
    event OperatorWentOffline(uint64 indexed serviceId, address indexed operator, uint8 missedBeats);
    event SlashingTriggered(uint64 indexed serviceId, address indexed operator, string reason);
    event StatusChanged(uint64 indexed serviceId, address indexed operator, IOperatorStatusRegistry.StatusCode oldStatus, IOperatorStatusRegistry.StatusCode newStatus);

    constructor(address _tangleCore);

    function DEFAULT_HEARTBEAT_INTERVAL() external view returns (uint64);
    function DEFAULT_MAX_MISSED_HEARTBEATS() external view returns (uint8);
    function DOMAIN_SEPARATOR() external view returns (bytes32);
    function HEARTBEAT_TYPEHASH() external view returns (bytes32);
    function addMetricDefinition(uint64 serviceId, string memory name, uint256 minValue, uint256 maxValue, bool required) external;
    function checkOperatorStatus(uint64 serviceId, address operator) external;
    function checkOperatorsStatus(uint64 serviceId, address[] memory operators) external;
    function configureHeartbeat(uint64 serviceId, uint64 interval, uint8 maxMissed) external;
    function enableCustomMetrics(uint64 serviceId, bool enabled) external;
    function getHeartbeatConfig(uint64 serviceId) external view returns (HeartbeatConfig memory);
    function getLastHeartbeat(uint64 serviceId, address operator) external view returns (uint256);
    function getMetricDefinitions(uint64 serviceId) external view returns (MetricDefinition[] memory);
    function getMetricValue(uint64 serviceId, address operator, string memory metricName) external view returns (uint256);
    function getOnlineOperatorCount(uint64 serviceId) external view returns (uint256);
    function getOnlineOperators(uint64 serviceId) external view returns (address[] memory);
    function getOperatorState(uint64 serviceId, address operator) external view returns (OperatorState memory);
    function getOperatorStatus(uint64 serviceId, address operator) external view returns (IOperatorStatusRegistry.StatusCode);
    function getSlashableOperators(uint64 serviceId) external view returns (address[] memory operators);
    function goOffline(uint64 serviceId) external;
    function goOnline(uint64 serviceId) external;
    function heartbeatConfigs(uint64) external view returns (uint64 interval, uint8 maxMissed, bool customMetrics);
    function isHeartbeatCurrent(uint64 serviceId, address operator) external view returns (bool);
    function isOnline(uint64 serviceId, address operator) external view returns (bool);
    function metricValues(uint64, address, string memory) external view returns (uint256);
    function operatorStates(uint64, address) external view returns (uint256 lastHeartbeat, uint64 consecutiveBeats, uint8 missedBeats, IOperatorStatusRegistry.StatusCode status, bytes32 lastMetricsHash);
    function registerServiceOwner(uint64 serviceId, address owner) external;
    function reportForSlashing(uint64 serviceId, address operator, string memory reason) external;
    function serviceMetrics(uint64, uint256) external view returns (string memory name, uint256 minValue, uint256 maxValue, bool required);
    function serviceOwners(uint64) external view returns (address);
    function setSlashingOracle(address oracle) external;
    function slashingOracle() external view returns (address);
    function submitHeartbeat(uint64 serviceId, uint64 blueprintId, uint8 statusCode, bytes memory metrics, bytes memory signature) external;
    function submitHeartbeatDirect(uint64 serviceId, uint64 blueprintId, uint8 statusCode, bytes memory metrics) external;
    function tangleCore() external view returns (address);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_tangleCore",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "DEFAULT_HEARTBEAT_INTERVAL",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "DEFAULT_MAX_MISSED_HEARTBEATS",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "DOMAIN_SEPARATOR",
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
    "name": "HEARTBEAT_TYPEHASH",
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
    "name": "addMetricDefinition",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "name",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "minValue",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "maxValue",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "required",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "checkOperatorStatus",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "checkOperatorsStatus",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "operators",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "configureHeartbeat",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "interval",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "maxMissed",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "enableCustomMetrics",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "enabled",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getHeartbeatConfig",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct OperatorStatusRegistry.HeartbeatConfig",
        "components": [
          {
            "name": "interval",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "maxMissed",
            "type": "uint8",
            "internalType": "uint8"
          },
          {
            "name": "customMetrics",
            "type": "bool",
            "internalType": "bool"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getLastHeartbeat",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "operator",
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
    "name": "getMetricDefinitions",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct OperatorStatusRegistry.MetricDefinition[]",
        "components": [
          {
            "name": "name",
            "type": "string",
            "internalType": "string"
          },
          {
            "name": "minValue",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "maxValue",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "required",
            "type": "bool",
            "internalType": "bool"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getMetricValue",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "metricName",
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
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOnlineOperatorCount",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
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
    "name": "getOnlineOperators",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorState",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct OperatorStatusRegistry.OperatorState",
        "components": [
          {
            "name": "lastHeartbeat",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "consecutiveBeats",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "missedBeats",
            "type": "uint8",
            "internalType": "uint8"
          },
          {
            "name": "status",
            "type": "uint8",
            "internalType": "enum IOperatorStatusRegistry.StatusCode"
          },
          {
            "name": "lastMetricsHash",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorStatus",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "enum IOperatorStatusRegistry.StatusCode"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getSlashableOperators",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [
      {
        "name": "operators",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "goOffline",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "goOnline",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "heartbeatConfigs",
    "inputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [
      {
        "name": "interval",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "maxMissed",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "customMetrics",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "isHeartbeatCurrent",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "operator",
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
    "name": "isOnline",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "operator",
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
    "name": "metricValues",
    "inputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
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
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "operatorStates",
    "inputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "lastHeartbeat",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "consecutiveBeats",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "missedBeats",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "status",
        "type": "uint8",
        "internalType": "enum IOperatorStatusRegistry.StatusCode"
      },
      {
        "name": "lastMetricsHash",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "registerServiceOwner",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "reportForSlashing",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "reason",
        "type": "string",
        "internalType": "string"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "serviceMetrics",
    "inputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "name",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "minValue",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "maxValue",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "required",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "serviceOwners",
    "inputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
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
    "name": "setSlashingOracle",
    "inputs": [
      {
        "name": "oracle",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "slashingOracle",
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
    "name": "submitHeartbeat",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "blueprintId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "statusCode",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "metrics",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "signature",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "submitHeartbeatDirect",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "blueprintId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "statusCode",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "metrics",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "tangleCore",
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
    "type": "event",
    "name": "HeartbeatConfigUpdated",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "indexed": true,
        "internalType": "uint64"
      },
      {
        "name": "interval",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
      },
      {
        "name": "maxMissed",
        "type": "uint8",
        "indexed": false,
        "internalType": "uint8"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "HeartbeatReceived",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "indexed": true,
        "internalType": "uint64"
      },
      {
        "name": "blueprintId",
        "type": "uint64",
        "indexed": true,
        "internalType": "uint64"
      },
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "statusCode",
        "type": "uint8",
        "indexed": false,
        "internalType": "uint8"
      },
      {
        "name": "timestamp",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "MetricReported",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "indexed": true,
        "internalType": "uint64"
      },
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "metricName",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "value",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorCameOnline",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "indexed": true,
        "internalType": "uint64"
      },
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorWentOffline",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "indexed": true,
        "internalType": "uint64"
      },
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "missedBeats",
        "type": "uint8",
        "indexed": false,
        "internalType": "uint8"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SlashingTriggered",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "indexed": true,
        "internalType": "uint64"
      },
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
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
    "name": "StatusChanged",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "indexed": true,
        "internalType": "uint64"
      },
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "oldStatus",
        "type": "uint8",
        "indexed": false,
        "internalType": "enum IOperatorStatusRegistry.StatusCode"
      },
      {
        "name": "newStatus",
        "type": "uint8",
        "indexed": false,
        "internalType": "enum IOperatorStatusRegistry.StatusCode"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "ECDSAInvalidSignature",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ECDSAInvalidSignatureLength",
    "inputs": [
      {
        "name": "length",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "ECDSAInvalidSignatureS",
    "inputs": [
      {
        "name": "s",
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
pub mod OperatorStatusRegistry {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60c08060405234610104576020816124e6803803809161001f8285610108565b83398101031261010457516001600160a01b03811681036101045760a05260405160208101907f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f82527f36ffc258c865193ae10c3cf640450ab772fdb8da1dfcae7862ad1205a5567f4c60408201527fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc660608201524660808201523060a082015260a081526100cf60c082610108565b5190206080526040516123a690816101408239608051816114b7015260a051818181610767015281816111c101526116530152f35b5f80fd5b601f909101601f19168101906001600160401b0382119082101761012b57604052565b634e487b7160e01b5f52604160045260245ffdfe6080806040526004361015610012575f80fd5b5f3560e01c908163057785501461162f575080630758236f146115d95780630c76697a1461158657806322f1ec93146114f65780632c957688146114da5780633644e515146114a05780633e6e34a71461140d57806340235a9c146113415780635685cf681461129157806356c4e17d1461125157806359dcea12146111f05780635a936dc6146111ac5780635cce98a61461115457806361d6b86c1461113957806362c7e8fc146110d657806371e7388c14610fd85780637b9f64b214610fa057806384ef732214610f5e57806396686c1e14610e765780639cbdae2214610deb578063adff830c14610cdf578063ae470a8514610ab6578063b074e9dd14610995578063b99f67591461073a578063ba1fb10314610710578063c1ef9ddf146105d4578063c5d960bb146104ce578063cfe34749146104a6578063d413a5801461032b578063d551162c146102d8578063da435a7c1461027f578063ee1c039014610249578063f9107f3b146101d05763f9f1676214610192575f80fd5b346101cc575f3660031901126101cc5760206040517fe1675f8364c07a4d60a07503f0d700a7bcacd82251dff0f070e5235de6c6d28a8152f35b5f80fd5b346101cc5760403660031901126101cc576101e9611741565b60243580151581036101cc576001600160401b036102479216805f52600360205261022160018060a01b0360405f2054163314611a84565b5f525f60205260405f209060ff60481b825491151560481b169060ff60481b1916179055565b005b346101cc5760403660031901126101cc576020610275610267611741565b61026f61176d565b90611c5e565b6040519015158152f35b346101cc5760203660031901126101cc576001600160401b036102a0611741565b165f525f602052606060405f205460ff604051916001600160401b0381168352818160401c16602084015260481c1615156040820152f35b346101cc5760206001600160401b03816102f1366119f9565b949092165f526005835260405f209060018060a01b03165f52825260405f2083604051948593843782019081520301902054604051908152f35b346101cc5760a03660031901126101cc57610344611741565b61034c611757565b90610355611959565b906064356001600160401b0381116101cc57610375903690600401611969565b9290916084356001600160401b0381116101cc576104486104426103a0610451933690600401611969565b919060405160208101906001600160401b0360c01b8860c01b1682526001600160401b0360c01b8c60c01b1660288201528a8a60308301376103f56030828d81015f838201520301601f198101835282611825565b51902060405160208101917f19457468657265756d205369676e6564204d6573736167653a0a3332000000008352603c820152603c8152610437605c82611825565b519020923691611996565b9061225d565b90929192612297565b336001600160a01b039091160361046d57610247943391611d79565b60405162461bcd60e51b8152602060048201526011602482015270496e76616c6964207369676e617475726560781b6044820152606490fd5b346101cc575f3660031901126101cc576006546040516001600160a01b039091168152602090f35b346101cc5760203660031901126101cc576001600160401b036104ef611741565b165f81815260016020818152604080842033855290915290912001805460481c60ff16919060058310156105c0576003831461057b57690400000000000000000060ff60481b19825416179055805f5260026020526105513360405f2061212e565b5061055f604051809361190a565b600460208301525f8051602061237a83398151915260403393a3005b60405162461bcd60e51b815260206004820152601f60248201527f43616e6e6f7420676f206f66666c696e65207768696c6520736c6173686564006044820152606490fd5b634e487b7160e01b5f52602160045260245ffd5b346101cc5760203660031901126101cc576001600160401b036105f5611741565b165f52600460205260405f2080549061060d82611a6d565b9161061b6040519384611825565b8083526020830180925f5260205f205f915b8383106106c357848660405191829160208301906020845251809152604083019060408160051b85010192915f905b82821061066b57505050500390f35b919360019193955060208091603f1989820301855287519060608061069984516080855260808501906118e6565b9385810151868501526040810151604085015201511515910152960192019201859493919261065c565b600460206001926040516106d6816117ef565b6106df86611846565b815284860154838201526002860154604082015260ff6003870154161515606082015281520192019201919061062d565b346101cc5760403660031901126101cc5761024761072c611741565b61073461176d565b90611ae5565b346101cc5760603660031901126101cc57610753611741565b61075b611757565b610763611959565b90337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614801561096f575b8015610947575b15610911576001600160401b031690603c82106108d75760ff169160018310610892577fc9599ed962624a858ec59bae0ed86c75f4db65fe04570021277edbedd04ea564916001600160401b036040921693845f525f60205261088560ff845f205460481c168451906108108261180a565b84825261086a60ff6020840186815288850193151584528a5f525f6020526001600160401b03808a5f20965116166001600160401b03198654161785555116839060ff60401b82549160401b169060ff60401b1916179055565b51815460ff60481b191690151560481b60ff60481b16179055565b82519182526020820152a2005b60405162461bcd60e51b815260206004820152601760248201527f4d6178206d6973736564206d757374206265203e3d20310000000000000000006044820152606490fd5b60405162461bcd60e51b8152602060048201526012602482015271125b9d195c9d985b081d1bdbc81cda1bdc9d60721b6044820152606490fd5b60405162461bcd60e51b815260206004820152600e60248201526d139bdd08185d5d1a1bdc9a5e995960921b6044820152606490fd5b506001600160401b0383165f908152600360205260409020546001600160a01b03161561079e565b506001600160401b0383165f52600360205260018060a01b0360405f2054163314610797565b346101cc5760203660031901126101cc576001600160401b036109b6611741565b165f81815260016020818152604080842033855290915290912001805460481c60ff16919060058310156105c05760038314610a7157805469ffff0000000000000000191669010000000000000000001790555f818152600260205260409020610a21903390612209565b50610a55604051809333847fc9862c5f02eefbdcea01c207ae538e1d304dc93026870f48951e48a0f4c8470c5f80a361190a565b600160208301525f8051602061237a83398151915260403393a3005b60405162461bcd60e51b815260206004820152601e60248201527f43616e6e6f7420676f206f6e6c696e65207768696c6520736c617368656400006044820152606490fd5b346101cc5760a03660031901126101cc57610acf611741565b6024356001600160401b0381116101cc57610aee903690600401611969565b90916084358015159291908390036101cc576001600160401b0316805f526003602052610b2860018060a01b0360405f2054163314611a84565b5f526004602052610b4b60405f209360405192610b44846117ef565b3691611996565b81526020810192604435845260408201906064358252606083019384528054600160401b811015610cb857610b8591600182018155611783565b929092610ccc5751938451946001600160401b038611610cb857610ba9845461179c565b601f8111610c73575b50602090601f8711600114610c0e57958091600396975f92610c03575b50508160011b915f1990871b1c19161783555b516001830155516002820155019051151560ff801983541691161790555f80f35b015190508780610bcf565b90601f19871691855f52815f20925f5b818110610c5b57509160019391896003999a9410610c44575b505050811b018355610be2565b01515f1983891b60f8161c19169055878080610c37565b92936020600181928786015181550195019301610c1e565b845f5260205f20601f880160051c81019160208910610cae575b601f0160051c01905b818110610ca35750610bb2565b5f8155600101610c96565b9091508190610c8d565b634e487b7160e01b5f52604160045260245ffd5b634e487b7160e01b5f525f60045260245ffd5b346101cc57610ced366119f9565b600654929391926001600160a01b03163303610db05760406001600160401b037f1e2909cf45d70cf003f334b73c93330ce7e572782dfc82fab79deb8855a7c791931693845f526001602052815f2060018060a01b0387165f526020526001825f2001690300000000000000000060ff60481b19825416179055845f526002602052610d85825f209660018060a01b0316809761212e565b5082825193849260208452816020850152848401375f828201840152601f01601f19168101030190a3005b60405162461bcd60e51b81526020600482015260136024820152724e6f7420736c617368696e67206f7261636c6560681b6044820152606490fd5b346101cc5760603660031901126101cc57610e04611741565b610e0c61176d565b6044356001600160401b0381116101cc5760209283926001600160401b03610e39859436906004016119db565b92165f526005835260405f209060018060a01b03165f52825260405f20604051938285935191829101845e82019081520301902054604051908152f35b346101cc5760403660031901126101cc57610e8f611741565b602435906001600160401b0382116101cc57366023830112156101cc578160040135916001600160401b0383116101cc573660248460051b830101116101cc5782906001600160401b035f9316925b82811015610247576024600582901b830101356001600160a01b03811691908290036101cc57303b156101cc576040519163ba1fb10360e01b835285600484015260248301525f8260448183305af1918215610f5357600192610f43575b5001610ede565b5f610f4d91611825565b85610f3c565b6040513d5f823e3d90fd5b346101cc5760203660031901126101cc576004356001600160a01b038116908190036101cc576bffffffffffffffffffffffff60a01b60065416176006555f80f35b346101cc5760203660031901126101cc576001600160401b03610fc1611741565b165f526002602052602060405f2054604051908152f35b346101cc5760403660031901126101cc57610ff1611741565b6001600160401b0361100161176d565b915f6080604051611011816117d4565b8281528260208201528260408201528260608201520152165f52600160205260405f209060018060a01b03165f5260205260405f20604051611052816117d4565b8154815260018201549160208201906001600160401b038416825260ff6040840194818160401c16865260481c16606084019060058110156105c05760a0956001600160401b0360026110ce9560ff9486520154956080880196875260405197518852511660208701525116604085015251606084019061190a565b516080820152f35b346101cc5760403660031901126101cc576110ef611741565b6001600160401b036110ff61176d565b91165f52600160205260405f209060018060a01b03165f52602052602060ff600160405f20015460481c16611137604051809261190a565bf35b346101cc575f3660031901126101cc57602060405160038152f35b346101cc5760803660031901126101cc5761116d611741565b611175611757565b9061117e611959565b91606435926001600160401b0384116101cc576111a2610247943690600401611969565b9390923391611d79565b346101cc575f3660031901126101cc576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346101cc5760203660031901126101cc57611209611741565b50604051610ca061121a8183611825565b60648252601f19019036906020013761124d60405161123a602082611825565b5f81525f36813760405191829182611917565b0390f35b346101cc5760203660031901126101cc576001600160401b03611272611741565b165f526003602052602060018060a01b0360405f205416604051908152f35b346101cc5760403660031901126101cc576112aa611741565b6001600160401b036112ba61176d565b9116805f52600160205260405f2060018060a01b0383165f5260205260ff600160405f20015460481c1660058110156105c05715908115611303575b6020826040519015158152f35b90505f52600160205260405f209060018060a01b03165f5260205260ff600160405f20015460481c1660058110156105c057600160209114826112f6565b346101cc5760203660031901126101cc576001600160401b03611362611741565b16805f52600260205260405f205461137981611a6d565b916113876040519384611825565b818352601f1961139683611a6d565b013660208501375f5b8281106113b4576040518061124d8682611917565b815f5260026020526113c98160405f2061200c565b9054855160039290921b1c6001600160a01b031691908110156113f95760019160208260051b870101520161139f565b634e487b7160e01b5f52603260045260245ffd5b346101cc5760403660031901126101cc57611426611741565b6001600160401b0361143661176d565b91165f52600160205260405f209060018060a01b03165f5260205260a060405f2080549061149960026001830154920154916040519384526001600160401b038116602085015260ff8160401c16604085015260ff606085019160481c1661190a565b6080820152f35b346101cc575f3660031901126101cc5760206040517f00000000000000000000000000000000000000000000000000000000000000008152f35b346101cc575f3660031901126101cc57602060405161012c8152f35b346101cc5760403660031901126101cc5761150f611741565b6001600160401b0360243591165f52600460205260405f2080548210156101cc576115709161153d91611783565b5061154781611846565b9060018101549060ff6003600283015492015416906040519485946080865260808601906118e6565b9260208501526040840152151560608301520390f35b346101cc5760403660031901126101cc5761159f611741565b6001600160401b036115af61176d565b91165f52600160205260405f209060018060a01b03165f52602052602060405f2054604051908152f35b346101cc5760203660031901126101cc5760606116056115f7611741565b6115ff611a4f565b50611d04565b60408051916001600160401b03815116835260ff6020820151166020840152015115156040820152f35b346101cc5760403660031901126101cc57611648611741565b61165061176d565b917f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316330361170c57506001600160401b03165f818152600360205260409020546001600160a01b03166116d2575f90815260036020526040902080546001600160a01b0319166001600160a01b03909216919091179055005b60405162461bcd60e51b8152602060048201526012602482015271105b1c9958591e481c9959da5cdd195c995960721b6044820152606490fd5b62461bcd60e51b815260206004820152601060248201526f4f6e6c792054616e676c6520636f726560801b6044820152606490fd5b600435906001600160401b03821682036101cc57565b602435906001600160401b03821682036101cc57565b602435906001600160a01b03821682036101cc57565b80548210156113f9575f5260205f209060021b01905f90565b90600182811c921680156117ca575b60208310146117b657565b634e487b7160e01b5f52602260045260245ffd5b91607f16916117ab565b60a081019081106001600160401b03821117610cb857604052565b608081019081106001600160401b03821117610cb857604052565b606081019081106001600160401b03821117610cb857604052565b90601f801991011681019081106001600160401b03821117610cb857604052565b9060405191825f8254926118598461179c565b80845293600181169081156118c45750600114611880575b5061187e92500383611825565b565b90505f9291925260205f20905f915b8183106118a857505090602061187e928201015f611871565b602091935080600191548385890101520191019091849261188f565b90506020925061187e94915060ff191682840152151560051b8201015f611871565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b9060058210156105c05752565b60206040818301928281528451809452019201905f5b81811061193a5750505090565b82516001600160a01b031684526020938401939092019160010161192d565b6044359060ff821682036101cc57565b9181601f840112156101cc578235916001600160401b0383116101cc57602083818601950101116101cc57565b9291926001600160401b038211610cb857604051916119bf601f8201601f191660200184611825565b8294818452818301116101cc578281602093845f960137010152565b9080601f830112156101cc578160206119f693359101611996565b90565b60606003198201126101cc576004356001600160401b03811681036101cc57916024356001600160a01b03811681036101cc5791604435906001600160401b0382116101cc57611a4b91600401611969565b9091565b60405190611a5c8261180a565b5f6040838281528260208201520152565b6001600160401b038111610cb85760051b60200190565b15611a8b57565b60405162461bcd60e51b81526020600482015260116024820152702737ba1039b2b93b34b1b29037bbb732b960791b6044820152606490fd5b91908203918211611ad157565b634e487b7160e01b5f52601160045260245ffd5b906001600160401b03821690815f52600160205260405f2060018060a01b0382165f52602052611b1860405f2093611d04565b9280548015611b5757611b2b9042611ac4565b6001600160401b03855116908115611c4a5760019160ff91041691019360ff855460401c168211611b5e575b5050505050565b845468ffffffffffffffffff191660ff60401b604084901b161785556020015160ff1681101580611c2f575b611b95575b80611b57565b835f8051602061237a83398151915292847f44fd32b677704ce68e7763897c49733b8f5289018ac60a5c926802d63759db4d602060409560ff611c0b9a5460481c1695690200000000000000000060ff60481b19825416179055835f5260028252865f209460018060a01b0316998a809661212e565b508651908152a3611c1e8251809261190a565b60026020820152a35f808080611b8f565b5060ff845460481c1660058110156105c05760021415611b8a565b634e487b7160e01b5f52601260045260245ffd5b906001600160401b03611c7083611d04565b92165f52600160205260405f209060018060a01b03165f5260205260405f2060405190611c9c826117d4565b8054825260ff60018201546001600160401b0381166020850152818160401c16604085015260481c169060058210156105c057600291606084015201546080820152518015611cfe57611cf76001600160401b039142611ac4565b9151161190565b50505f90565b6001600160401b0390611d15611a4f565b50165f525f60205260405f2060405190611d2e8261180a565b546001600160401b03811680835260ff8260401c169060ff602085019383855260481c161515604085015215611d6f575b15611d68575090565b6003905290565b61012c8352611d5f565b93949192906001600160401b03851694855f52600160205260405f2060018060a01b0386165f5260205260405f2094611db182611d04565b90600187019460ff865460481c16974281556002611dd18c8a3691611996565b6020815191012091015560ff60401b1986541686556001600160401b03865416906001600160401b038214611ad1576001600160401b03600160ff9301166001600160401b0319885416178755169283155f14611f88575f965b600588101596876105c057805460ff60481b191660488a901b60ff60481b1617905560058910156105c0576001600160401b039a8a9460028b148a81611f79575b50918560407f658918e3147f13dd068ec21437b4c25c21682a8dc2129348671ead000db3e7b9979593819795611f2c575b0151151580611f23575b611f11575b5050835196875250504260208601526001600160a01b0316998a951693a46105c057808203611edc575b50505050565b5f8051602061237a83398151915291611f07604092611efd8451809461190a565b602083019061190a565ba35f808080611ed6565b611f1a93612021565b5f808083611eac565b50821515611ea7565b5f8a815260026020528290206001600160a01b03841690611f4e908290612209565b508a7fc9862c5f02eefbdcea01c207ae538e1d304dc93026870f48951e48a0f4c8470c5f80a3611e9d565b5f9a506002141590508a611e6c565b6064841015611f9957600196611e2b565b60019660c88510611e2b5760405160208152601b60208201527f50726f746f636f6c2076696f6c6174696f6e207265706f72746564000000000060408201528a7f1e2909cf45d70cf003f334b73c93330ce7e572782dfc82fab79deb8855a7c791606060018060a01b03871693a3611e2b565b80548210156113f9575f5260205f2001905f90565b60408410611ed6575f925b60408401808511611ad1578510611b57578484116101cc5783810184860381016040828203126101cc5781356001600160401b0381116101cc576020916120749184016119db565b9101356001600160401b038416805f52600560205260405f2060018060a01b0387165f526020528160405f2060206040518092875180848a01845e820190815203019020557f23ed02bd3605bdea6a8afa76c46f00d274860ba6cea980f2585b696df9e182bd604051604081526120ee60408201866118e6565b60208201949094526001600160a01b0388169381900390a35180604001604011611ad157604090850101809411611ad1578484111561202c575050505050565b906001820191815f528260205260405f20548015155f14612201575f198101818111611ad15782545f19810191908211611ad1578181036121b6575b505050805480156121a2575f190190612183828261200c565b8154905f199060031b1b19169055555f526020525f6040812055600190565b634e487b7160e01b5f52603160045260245ffd5b6121ec6121c66121d6938661200c565b90549060031b1c9283928661200c565b819391549060031b91821b915f19901b19161790565b90555f528360205260405f20555f808061216a565b505050505f90565b6001810190825f528160205260405f2054155f14612256578054600160401b811015610cb8576122436121d682600187940185558461200c565b905554915f5260205260405f2055600190565b5050505f90565b815191906041830361228d576122869250602082015190606060408401519301515f1a906122f7565b9192909190565b50505f9160029190565b60048110156105c057806122a9575050565b600181036122c05763f645eedf60e01b5f5260045ffd5b600281036122db575063fce698f760e01b5f5260045260245ffd5b6003146122e55750565b6335e2f38360e21b5f5260045260245ffd5b91907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0841161236e579160209360809260ff5f9560405194855216868401526040830152606082015282805260015afa15610f53575f516001600160a01b0381161561236457905f905f90565b505f906001905f90565b5050505f916003919056fe228824b86c256469125f525ce18c6c2d0a9e133d13b8ec7a2c96a193b0c28a09a164736f6c634300081a000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xC0\x80`@R4a\x01\x04W` \x81a$\xE6\x808\x03\x80\x91a\0\x1F\x82\x85a\x01\x08V[\x839\x81\x01\x03\x12a\x01\x04WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x01\x04W`\xA0R`@Q` \x81\x01\x90\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x82R\x7F6\xFF\xC2X\xC8e\x19:\xE1\x0C<\xF6@E\n\xB7r\xFD\xB8\xDA\x1D\xFC\xAExb\xAD\x12\x05\xA5V\x7FL`@\x82\x01R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xA0\x81Ra\0\xCF`\xC0\x82a\x01\x08V[Q\x90 `\x80R`@Qa#\xA6\x90\x81a\x01@\x829`\x80Q\x81a\x14\xB7\x01R`\xA0Q\x81\x81\x81a\x07g\x01R\x81\x81a\x11\xC1\x01Ra\x16S\x01R\xF3[_\x80\xFD[`\x1F\x90\x91\x01`\x1F\x19\x16\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x90\x82\x10\x17a\x01+W`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_5`\xE0\x1C\x90\x81c\x05w\x85P\x14a\x16/WP\x80c\x07X#o\x14a\x15\xD9W\x80c\x0Cviz\x14a\x15\x86W\x80c\"\xF1\xEC\x93\x14a\x14\xF6W\x80c,\x95v\x88\x14a\x14\xDAW\x80c6D\xE5\x15\x14a\x14\xA0W\x80c>n4\xA7\x14a\x14\rW\x80c@#Z\x9C\x14a\x13AW\x80cV\x85\xCFh\x14a\x12\x91W\x80cV\xC4\xE1}\x14a\x12QW\x80cY\xDC\xEA\x12\x14a\x11\xF0W\x80cZ\x93m\xC6\x14a\x11\xACW\x80c\\\xCE\x98\xA6\x14a\x11TW\x80ca\xD6\xB8l\x14a\x119W\x80cb\xC7\xE8\xFC\x14a\x10\xD6W\x80cq\xE78\x8C\x14a\x0F\xD8W\x80c{\x9Fd\xB2\x14a\x0F\xA0W\x80c\x84\xEFs\"\x14a\x0F^W\x80c\x96hl\x1E\x14a\x0EvW\x80c\x9C\xBD\xAE\"\x14a\r\xEBW\x80c\xAD\xFF\x83\x0C\x14a\x0C\xDFW\x80c\xAEG\n\x85\x14a\n\xB6W\x80c\xB0t\xE9\xDD\x14a\t\x95W\x80c\xB9\x9FgY\x14a\x07:W\x80c\xBA\x1F\xB1\x03\x14a\x07\x10W\x80c\xC1\xEF\x9D\xDF\x14a\x05\xD4W\x80c\xC5\xD9`\xBB\x14a\x04\xCEW\x80c\xCF\xE3GI\x14a\x04\xA6W\x80c\xD4\x13\xA5\x80\x14a\x03+W\x80c\xD5Q\x16,\x14a\x02\xD8W\x80c\xDACZ|\x14a\x02\x7FW\x80c\xEE\x1C\x03\x90\x14a\x02IW\x80c\xF9\x10\x7F;\x14a\x01\xD0Wc\xF9\xF1gb\x14a\x01\x92W_\x80\xFD[4a\x01\xCCW_6`\x03\x19\x01\x12a\x01\xCCW` `@Q\x7F\xE1g_\x83d\xC0zM`\xA0u\x03\xF0\xD7\0\xA7\xBC\xAC\xD8\"Q\xDF\xF0\xF0p\xE5#]\xE6\xC6\xD2\x8A\x81R\xF3[_\x80\xFD[4a\x01\xCCW`@6`\x03\x19\x01\x12a\x01\xCCWa\x01\xE9a\x17AV[`$5\x80\x15\x15\x81\x03a\x01\xCCW`\x01`\x01`@\x1B\x03a\x02G\x92\x16\x80_R`\x03` Ra\x02!`\x01\x80`\xA0\x1B\x03`@_ T\x163\x14a\x1A\x84V[_R_` R`@_ \x90`\xFF`H\x1B\x82T\x91\x15\x15`H\x1B\x16\x90`\xFF`H\x1B\x19\x16\x17\x90UV[\0[4a\x01\xCCW`@6`\x03\x19\x01\x12a\x01\xCCW` a\x02ua\x02ga\x17AV[a\x02oa\x17mV[\x90a\x1C^V[`@Q\x90\x15\x15\x81R\xF3[4a\x01\xCCW` 6`\x03\x19\x01\x12a\x01\xCCW`\x01`\x01`@\x1B\x03a\x02\xA0a\x17AV[\x16_R_` R```@_ T`\xFF`@Q\x91`\x01`\x01`@\x1B\x03\x81\x16\x83R\x81\x81`@\x1C\x16` \x84\x01R`H\x1C\x16\x15\x15`@\x82\x01R\xF3[4a\x01\xCCW` `\x01`\x01`@\x1B\x03\x81a\x02\xF16a\x19\xF9V[\x94\x90\x92\x16_R`\x05\x83R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R\x82R`@_ \x83`@Q\x94\x85\x93\x847\x82\x01\x90\x81R\x03\x01\x90 T`@Q\x90\x81R\xF3[4a\x01\xCCW`\xA06`\x03\x19\x01\x12a\x01\xCCWa\x03Da\x17AV[a\x03La\x17WV[\x90a\x03Ua\x19YV[\x90`d5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xCCWa\x03u\x906\x90`\x04\x01a\x19iV[\x92\x90\x91`\x845`\x01`\x01`@\x1B\x03\x81\x11a\x01\xCCWa\x04Ha\x04Ba\x03\xA0a\x04Q\x936\x90`\x04\x01a\x19iV[\x91\x90`@Q` \x81\x01\x90`\x01`\x01`@\x1B\x03`\xC0\x1B\x88`\xC0\x1B\x16\x82R`\x01`\x01`@\x1B\x03`\xC0\x1B\x8C`\xC0\x1B\x16`(\x82\x01R\x8A\x8A`0\x83\x017a\x03\xF5`0\x82\x8D\x81\x01_\x83\x82\x01R\x03\x01`\x1F\x19\x81\x01\x83R\x82a\x18%V[Q\x90 `@Q` \x81\x01\x91\x7F\x19Ethereum Signed Message:\n32\0\0\0\0\x83R`<\x82\x01R`<\x81Ra\x047`\\\x82a\x18%V[Q\x90 \x926\x91a\x19\x96V[\x90a\"]V[\x90\x92\x91\x92a\"\x97V[3`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x03a\x04mWa\x02G\x943\x91a\x1DyV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01RpInvalid signature`x\x1B`D\x82\x01R`d\x90\xFD[4a\x01\xCCW_6`\x03\x19\x01\x12a\x01\xCCW`\x06T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x01\xCCW` 6`\x03\x19\x01\x12a\x01\xCCW`\x01`\x01`@\x1B\x03a\x04\xEFa\x17AV[\x16_\x81\x81R`\x01` \x81\x81R`@\x80\x84 3\x85R\x90\x91R\x90\x91 \x01\x80T`H\x1C`\xFF\x16\x91\x90`\x05\x83\x10\x15a\x05\xC0W`\x03\x83\x14a\x05{Wi\x04\0\0\0\0\0\0\0\0\0`\xFF`H\x1B\x19\x82T\x16\x17\x90U\x80_R`\x02` Ra\x05Q3`@_ a!.V[Pa\x05_`@Q\x80\x93a\x19\nV[`\x04` \x83\x01R_\x80Q` a#z\x839\x81Q\x91R`@3\x93\xA3\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FCannot go offline while slashed\0`D\x82\x01R`d\x90\xFD[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[4a\x01\xCCW` 6`\x03\x19\x01\x12a\x01\xCCW`\x01`\x01`@\x1B\x03a\x05\xF5a\x17AV[\x16_R`\x04` R`@_ \x80T\x90a\x06\r\x82a\x1AmV[\x91a\x06\x1B`@Q\x93\x84a\x18%V[\x80\x83R` \x83\x01\x80\x92_R` _ _\x91[\x83\x83\x10a\x06\xC3W\x84\x86`@Q\x91\x82\x91` \x83\x01\x90` \x84RQ\x80\x91R`@\x83\x01\x90`@\x81`\x05\x1B\x85\x01\x01\x92\x91_\x90[\x82\x82\x10a\x06kWPPPP\x03\x90\xF3[\x91\x93`\x01\x91\x93\x95P` \x80\x91`?\x19\x89\x82\x03\x01\x85R\x87Q\x90``\x80a\x06\x99\x84Q`\x80\x85R`\x80\x85\x01\x90a\x18\xE6V[\x93\x85\x81\x01Q\x86\x85\x01R`@\x81\x01Q`@\x85\x01R\x01Q\x15\x15\x91\x01R\x96\x01\x92\x01\x92\x01\x85\x94\x93\x91\x92a\x06\\V[`\x04` `\x01\x92`@Qa\x06\xD6\x81a\x17\xEFV[a\x06\xDF\x86a\x18FV[\x81R\x84\x86\x01T\x83\x82\x01R`\x02\x86\x01T`@\x82\x01R`\xFF`\x03\x87\x01T\x16\x15\x15``\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x06-V[4a\x01\xCCW`@6`\x03\x19\x01\x12a\x01\xCCWa\x02Ga\x07,a\x17AV[a\x074a\x17mV[\x90a\x1A\xE5V[4a\x01\xCCW``6`\x03\x19\x01\x12a\x01\xCCWa\x07Sa\x17AV[a\x07[a\x17WV[a\x07ca\x19YV[\x903\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14\x80\x15a\toW[\x80\x15a\tGW[\x15a\t\x11W`\x01`\x01`@\x1B\x03\x16\x90`<\x82\x10a\x08\xD7W`\xFF\x16\x91`\x01\x83\x10a\x08\x92W\x7F\xC9Y\x9E\xD9bbJ\x85\x8E\xC5\x9B\xAE\x0E\xD8lu\xF4\xDBe\xFE\x04W\0!'~\xDB\xED\xD0N\xA5d\x91`\x01`\x01`@\x1B\x03`@\x92\x16\x93\x84_R_` Ra\x08\x85`\xFF\x84_ T`H\x1C\x16\x84Q\x90a\x08\x10\x82a\x18\nV[\x84\x82Ra\x08j`\xFF` \x84\x01\x86\x81R\x88\x85\x01\x93\x15\x15\x84R\x8A_R_` R`\x01`\x01`@\x1B\x03\x80\x8A_ \x96Q\x16\x16`\x01`\x01`@\x1B\x03\x19\x86T\x16\x17\x85UQ\x16\x83\x90`\xFF`@\x1B\x82T\x91`@\x1B\x16\x90`\xFF`@\x1B\x19\x16\x17\x90UV[Q\x81T`\xFF`H\x1B\x19\x16\x90\x15\x15`H\x1B`\xFF`H\x1B\x16\x17\x90UV[\x82Q\x91\x82R` \x82\x01R\xA2\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FMax missed must be >= 1\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rq\x12[\x9D\x19\\\x9D\x98[\x08\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x82\x01R`d\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x13\x9B\xDD\x08\x18]]\x1A\x1B\xDC\x9A^\x99Y`\x92\x1B`D\x82\x01R`d\x90\xFD[P`\x01`\x01`@\x1B\x03\x83\x16_\x90\x81R`\x03` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x07\x9EV[P`\x01`\x01`@\x1B\x03\x83\x16_R`\x03` R`\x01\x80`\xA0\x1B\x03`@_ T\x163\x14a\x07\x97V[4a\x01\xCCW` 6`\x03\x19\x01\x12a\x01\xCCW`\x01`\x01`@\x1B\x03a\t\xB6a\x17AV[\x16_\x81\x81R`\x01` \x81\x81R`@\x80\x84 3\x85R\x90\x91R\x90\x91 \x01\x80T`H\x1C`\xFF\x16\x91\x90`\x05\x83\x10\x15a\x05\xC0W`\x03\x83\x14a\nqW\x80Ti\xFF\xFF\0\0\0\0\0\0\0\0\x19\x16i\x01\0\0\0\0\0\0\0\0\0\x17\x90U_\x81\x81R`\x02` R`@\x90 a\n!\x903\x90a\"\tV[Pa\nU`@Q\x80\x933\x84\x7F\xC9\x86,_\x02\xEE\xFB\xDC\xEA\x01\xC2\x07\xAES\x8E\x1D0M\xC90&\x87\x0FH\x95\x1EH\xA0\xF4\xC8G\x0C_\x80\xA3a\x19\nV[`\x01` \x83\x01R_\x80Q` a#z\x839\x81Q\x91R`@3\x93\xA3\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FCannot go online while slashed\0\0`D\x82\x01R`d\x90\xFD[4a\x01\xCCW`\xA06`\x03\x19\x01\x12a\x01\xCCWa\n\xCFa\x17AV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xCCWa\n\xEE\x906\x90`\x04\x01a\x19iV[\x90\x91`\x845\x80\x15\x15\x92\x91\x90\x83\x90\x03a\x01\xCCW`\x01`\x01`@\x1B\x03\x16\x80_R`\x03` Ra\x0B(`\x01\x80`\xA0\x1B\x03`@_ T\x163\x14a\x1A\x84V[_R`\x04` Ra\x0BK`@_ \x93`@Q\x92a\x0BD\x84a\x17\xEFV[6\x91a\x19\x96V[\x81R` \x81\x01\x92`D5\x84R`@\x82\x01\x90`d5\x82R``\x83\x01\x93\x84R\x80T`\x01`@\x1B\x81\x10\x15a\x0C\xB8Wa\x0B\x85\x91`\x01\x82\x01\x81Ua\x17\x83V[\x92\x90\x92a\x0C\xCCWQ\x93\x84Q\x94`\x01`\x01`@\x1B\x03\x86\x11a\x0C\xB8Wa\x0B\xA9\x84Ta\x17\x9CV[`\x1F\x81\x11a\x0CsW[P` \x90`\x1F\x87\x11`\x01\x14a\x0C\x0EW\x95\x80\x91`\x03\x96\x97_\x92a\x0C\x03W[PP\x81`\x01\x1B\x91_\x19\x90\x87\x1B\x1C\x19\x16\x17\x83U[Q`\x01\x83\x01UQ`\x02\x82\x01U\x01\x90Q\x15\x15`\xFF\x80\x19\x83T\x16\x91\x16\x17\x90U_\x80\xF3[\x01Q\x90P\x87\x80a\x0B\xCFV[\x90`\x1F\x19\x87\x16\x91\x85_R\x81_ \x92_[\x81\x81\x10a\x0C[WP\x91`\x01\x93\x91\x89`\x03\x99\x9A\x94\x10a\x0CDW[PPP\x81\x1B\x01\x83Ua\x0B\xE2V[\x01Q_\x19\x83\x89\x1B`\xF8\x16\x1C\x19\x16\x90U\x87\x80\x80a\x0C7V[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x0C\x1EV[\x84_R` _ `\x1F\x88\x01`\x05\x1C\x81\x01\x91` \x89\x10a\x0C\xAEW[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x0C\xA3WPa\x0B\xB2V[_\x81U`\x01\x01a\x0C\x96V[\x90\x91P\x81\x90a\x0C\x8DV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[4a\x01\xCCWa\x0C\xED6a\x19\xF9V[`\x06T\x92\x93\x91\x92`\x01`\x01`\xA0\x1B\x03\x163\x03a\r\xB0W`@`\x01`\x01`@\x1B\x03\x7F\x1E)\t\xCFE\xD7\x0C\xF0\x03\xF34\xB7<\x933\x0C\xE7\xE5rx-\xFC\x82\xFA\xB7\x9D\xEB\x88U\xA7\xC7\x91\x93\x16\x93\x84_R`\x01` R\x81_ `\x01\x80`\xA0\x1B\x03\x87\x16_R` R`\x01\x82_ \x01i\x03\0\0\0\0\0\0\0\0\0`\xFF`H\x1B\x19\x82T\x16\x17\x90U\x84_R`\x02` Ra\r\x85\x82_ \x96`\x01\x80`\xA0\x1B\x03\x16\x80\x97a!.V[P\x82\x82Q\x93\x84\x92` \x84R\x81` \x85\x01R\x84\x84\x017_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x81\x01\x03\x01\x90\xA3\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01RrNot slashing oracle`h\x1B`D\x82\x01R`d\x90\xFD[4a\x01\xCCW``6`\x03\x19\x01\x12a\x01\xCCWa\x0E\x04a\x17AV[a\x0E\x0Ca\x17mV[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xCCW` \x92\x83\x92`\x01`\x01`@\x1B\x03a\x0E9\x85\x946\x90`\x04\x01a\x19\xDBV[\x92\x16_R`\x05\x83R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R\x82R`@_ `@Q\x93\x82\x85\x93Q\x91\x82\x91\x01\x84^\x82\x01\x90\x81R\x03\x01\x90 T`@Q\x90\x81R\xF3[4a\x01\xCCW`@6`\x03\x19\x01\x12a\x01\xCCWa\x0E\x8Fa\x17AV[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x01\xCCW6`#\x83\x01\x12\x15a\x01\xCCW\x81`\x04\x015\x91`\x01`\x01`@\x1B\x03\x83\x11a\x01\xCCW6`$\x84`\x05\x1B\x83\x01\x01\x11a\x01\xCCW\x82\x90`\x01`\x01`@\x1B\x03_\x93\x16\x92[\x82\x81\x10\x15a\x02GW`$`\x05\x82\x90\x1B\x83\x01\x015`\x01`\x01`\xA0\x1B\x03\x81\x16\x91\x90\x82\x90\x03a\x01\xCCW0;\x15a\x01\xCCW`@Q\x91c\xBA\x1F\xB1\x03`\xE0\x1B\x83R\x85`\x04\x84\x01R`$\x83\x01R_\x82`D\x81\x830Z\xF1\x91\x82\x15a\x0FSW`\x01\x92a\x0FCW[P\x01a\x0E\xDEV[_a\x0FM\x91a\x18%V[\x85a\x0F<V[`@Q=_\x82>=\x90\xFD[4a\x01\xCCW` 6`\x03\x19\x01\x12a\x01\xCCW`\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\x01\xCCWk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B`\x06T\x16\x17`\x06U_\x80\xF3[4a\x01\xCCW` 6`\x03\x19\x01\x12a\x01\xCCW`\x01`\x01`@\x1B\x03a\x0F\xC1a\x17AV[\x16_R`\x02` R` `@_ T`@Q\x90\x81R\xF3[4a\x01\xCCW`@6`\x03\x19\x01\x12a\x01\xCCWa\x0F\xF1a\x17AV[`\x01`\x01`@\x1B\x03a\x10\x01a\x17mV[\x91_`\x80`@Qa\x10\x11\x81a\x17\xD4V[\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x01R\x16_R`\x01` R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ `@Qa\x10R\x81a\x17\xD4V[\x81T\x81R`\x01\x82\x01T\x91` \x82\x01\x90`\x01`\x01`@\x1B\x03\x84\x16\x82R`\xFF`@\x84\x01\x94\x81\x81`@\x1C\x16\x86R`H\x1C\x16``\x84\x01\x90`\x05\x81\x10\x15a\x05\xC0W`\xA0\x95`\x01`\x01`@\x1B\x03`\x02a\x10\xCE\x95`\xFF\x94\x86R\x01T\x95`\x80\x88\x01\x96\x87R`@Q\x97Q\x88RQ\x16` \x87\x01RQ\x16`@\x85\x01RQ``\x84\x01\x90a\x19\nV[Q`\x80\x82\x01R\xF3[4a\x01\xCCW`@6`\x03\x19\x01\x12a\x01\xCCWa\x10\xEFa\x17AV[`\x01`\x01`@\x1B\x03a\x10\xFFa\x17mV[\x91\x16_R`\x01` R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R` `\xFF`\x01`@_ \x01T`H\x1C\x16a\x117`@Q\x80\x92a\x19\nV[\xF3[4a\x01\xCCW_6`\x03\x19\x01\x12a\x01\xCCW` `@Q`\x03\x81R\xF3[4a\x01\xCCW`\x806`\x03\x19\x01\x12a\x01\xCCWa\x11ma\x17AV[a\x11ua\x17WV[\x90a\x11~a\x19YV[\x91`d5\x92`\x01`\x01`@\x1B\x03\x84\x11a\x01\xCCWa\x11\xA2a\x02G\x946\x90`\x04\x01a\x19iV[\x93\x90\x923\x91a\x1DyV[4a\x01\xCCW_6`\x03\x19\x01\x12a\x01\xCCW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x01\xCCW` 6`\x03\x19\x01\x12a\x01\xCCWa\x12\ta\x17AV[P`@Qa\x0C\xA0a\x12\x1A\x81\x83a\x18%V[`d\x82R`\x1F\x19\x01\x906\x90` \x017a\x12M`@Qa\x12:` \x82a\x18%V[_\x81R_6\x817`@Q\x91\x82\x91\x82a\x19\x17V[\x03\x90\xF3[4a\x01\xCCW` 6`\x03\x19\x01\x12a\x01\xCCW`\x01`\x01`@\x1B\x03a\x12ra\x17AV[\x16_R`\x03` R` `\x01\x80`\xA0\x1B\x03`@_ T\x16`@Q\x90\x81R\xF3[4a\x01\xCCW`@6`\x03\x19\x01\x12a\x01\xCCWa\x12\xAAa\x17AV[`\x01`\x01`@\x1B\x03a\x12\xBAa\x17mV[\x91\x16\x80_R`\x01` R`@_ `\x01\x80`\xA0\x1B\x03\x83\x16_R` R`\xFF`\x01`@_ \x01T`H\x1C\x16`\x05\x81\x10\x15a\x05\xC0W\x15\x90\x81\x15a\x13\x03W[` \x82`@Q\x90\x15\x15\x81R\xF3[\x90P_R`\x01` R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`\xFF`\x01`@_ \x01T`H\x1C\x16`\x05\x81\x10\x15a\x05\xC0W`\x01` \x91\x14\x82a\x12\xF6V[4a\x01\xCCW` 6`\x03\x19\x01\x12a\x01\xCCW`\x01`\x01`@\x1B\x03a\x13ba\x17AV[\x16\x80_R`\x02` R`@_ Ta\x13y\x81a\x1AmV[\x91a\x13\x87`@Q\x93\x84a\x18%V[\x81\x83R`\x1F\x19a\x13\x96\x83a\x1AmV[\x016` \x85\x017_[\x82\x81\x10a\x13\xB4W`@Q\x80a\x12M\x86\x82a\x19\x17V[\x81_R`\x02` Ra\x13\xC9\x81`@_ a \x0CV[\x90T\x85Q`\x03\x92\x90\x92\x1B\x1C`\x01`\x01`\xA0\x1B\x03\x16\x91\x90\x81\x10\x15a\x13\xF9W`\x01\x91` \x82`\x05\x1B\x87\x01\x01R\x01a\x13\x9FV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[4a\x01\xCCW`@6`\x03\x19\x01\x12a\x01\xCCWa\x14&a\x17AV[`\x01`\x01`@\x1B\x03a\x146a\x17mV[\x91\x16_R`\x01` R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`\xA0`@_ \x80T\x90a\x14\x99`\x02`\x01\x83\x01T\x92\x01T\x91`@Q\x93\x84R`\x01`\x01`@\x1B\x03\x81\x16` \x85\x01R`\xFF\x81`@\x1C\x16`@\x85\x01R`\xFF``\x85\x01\x91`H\x1C\x16a\x19\nV[`\x80\x82\x01R\xF3[4a\x01\xCCW_6`\x03\x19\x01\x12a\x01\xCCW` `@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xF3[4a\x01\xCCW_6`\x03\x19\x01\x12a\x01\xCCW` `@Qa\x01,\x81R\xF3[4a\x01\xCCW`@6`\x03\x19\x01\x12a\x01\xCCWa\x15\x0Fa\x17AV[`\x01`\x01`@\x1B\x03`$5\x91\x16_R`\x04` R`@_ \x80T\x82\x10\x15a\x01\xCCWa\x15p\x91a\x15=\x91a\x17\x83V[Pa\x15G\x81a\x18FV[\x90`\x01\x81\x01T\x90`\xFF`\x03`\x02\x83\x01T\x92\x01T\x16\x90`@Q\x94\x85\x94`\x80\x86R`\x80\x86\x01\x90a\x18\xE6V[\x92` \x85\x01R`@\x84\x01R\x15\x15``\x83\x01R\x03\x90\xF3[4a\x01\xCCW`@6`\x03\x19\x01\x12a\x01\xCCWa\x15\x9Fa\x17AV[`\x01`\x01`@\x1B\x03a\x15\xAFa\x17mV[\x91\x16_R`\x01` R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R` `@_ T`@Q\x90\x81R\xF3[4a\x01\xCCW` 6`\x03\x19\x01\x12a\x01\xCCW``a\x16\x05a\x15\xF7a\x17AV[a\x15\xFFa\x1AOV[Pa\x1D\x04V[`@\x80Q\x91`\x01`\x01`@\x1B\x03\x81Q\x16\x83R`\xFF` \x82\x01Q\x16` \x84\x01R\x01Q\x15\x15`@\x82\x01R\xF3[4a\x01\xCCW`@6`\x03\x19\x01\x12a\x01\xCCWa\x16Ha\x17AV[a\x16Pa\x17mV[\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a\x17\x0CWP`\x01`\x01`@\x1B\x03\x16_\x81\x81R`\x03` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16a\x16\xD2W_\x90\x81R`\x03` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rq\x10[\x1C\x99XY\x1EH\x1C\x99Y\xDA\\\xDD\x19\\\x99Y`r\x1B`D\x82\x01R`d\x90\xFD[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RoOnly Tangle core`\x80\x1B`D\x82\x01R`d\x90\xFD[`\x045\x90`\x01`\x01`@\x1B\x03\x82\x16\x82\x03a\x01\xCCWV[`$5\x90`\x01`\x01`@\x1B\x03\x82\x16\x82\x03a\x01\xCCWV[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01\xCCWV[\x80T\x82\x10\x15a\x13\xF9W_R` _ \x90`\x02\x1B\x01\x90_\x90V[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x17\xCAW[` \x83\x10\x14a\x17\xB6WV[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a\x17\xABV[`\xA0\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x0C\xB8W`@RV[`\x80\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x0C\xB8W`@RV[``\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x0C\xB8W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x0C\xB8W`@RV[\x90`@Q\x91\x82_\x82T\x92a\x18Y\x84a\x17\x9CV[\x80\x84R\x93`\x01\x81\x16\x90\x81\x15a\x18\xC4WP`\x01\x14a\x18\x80W[Pa\x18~\x92P\x03\x83a\x18%V[V[\x90P_\x92\x91\x92R` _ \x90_\x91[\x81\x83\x10a\x18\xA8WPP\x90` a\x18~\x92\x82\x01\x01_a\x18qV[` \x91\x93P\x80`\x01\x91T\x83\x85\x89\x01\x01R\x01\x91\x01\x90\x91\x84\x92a\x18\x8FV[\x90P` \x92Pa\x18~\x94\x91P`\xFF\x19\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01_a\x18qV[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90`\x05\x82\x10\x15a\x05\xC0WRV[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x19:WPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x19-V[`D5\x90`\xFF\x82\x16\x82\x03a\x01\xCCWV[\x91\x81`\x1F\x84\x01\x12\x15a\x01\xCCW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x01\xCCW` \x83\x81\x86\x01\x95\x01\x01\x11a\x01\xCCWV[\x92\x91\x92`\x01`\x01`@\x1B\x03\x82\x11a\x0C\xB8W`@Q\x91a\x19\xBF`\x1F\x82\x01`\x1F\x19\x16` \x01\x84a\x18%V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x01\xCCW\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x01\xCCW\x81` a\x19\xF6\x935\x91\x01a\x19\x96V[\x90V[```\x03\x19\x82\x01\x12a\x01\xCCW`\x045`\x01`\x01`@\x1B\x03\x81\x16\x81\x03a\x01\xCCW\x91`$5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x01\xCCW\x91`D5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x01\xCCWa\x1AK\x91`\x04\x01a\x19iV[\x90\x91V[`@Q\x90a\x1A\\\x82a\x18\nV[_`@\x83\x82\x81R\x82` \x82\x01R\x01RV[`\x01`\x01`@\x1B\x03\x81\x11a\x0C\xB8W`\x05\x1B` \x01\x90V[\x15a\x1A\x8BWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp'7\xBA\x109\xB2\xB9;4\xB1\xB2\x907\xBB\xB72\xB9`y\x1B`D\x82\x01R`d\x90\xFD[\x91\x90\x82\x03\x91\x82\x11a\x1A\xD1WV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x90`\x01`\x01`@\x1B\x03\x82\x16\x90\x81_R`\x01` R`@_ `\x01\x80`\xA0\x1B\x03\x82\x16_R` Ra\x1B\x18`@_ \x93a\x1D\x04V[\x92\x80T\x80\x15a\x1BWWa\x1B+\x90Ba\x1A\xC4V[`\x01`\x01`@\x1B\x03\x85Q\x16\x90\x81\x15a\x1CJW`\x01\x91`\xFF\x91\x04\x16\x91\x01\x93`\xFF\x85T`@\x1C\x16\x82\x11a\x1B^W[PPPPPV[\x84Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\xFF`@\x1B`@\x84\x90\x1B\x16\x17\x85U` \x01Q`\xFF\x16\x81\x10\x15\x80a\x1C/W[a\x1B\x95W[\x80a\x1BWV[\x83_\x80Q` a#z\x839\x81Q\x91R\x92\x84\x7FD\xFD2\xB6wpL\xE6\x8Ewc\x89|Is;\x8FR\x89\x01\x8A\xC6\n\\\x92h\x02\xD67Y\xDBM` `@\x95`\xFFa\x1C\x0B\x9AT`H\x1C\x16\x95i\x02\0\0\0\0\0\0\0\0\0`\xFF`H\x1B\x19\x82T\x16\x17\x90U\x83_R`\x02\x82R\x86_ \x94`\x01\x80`\xA0\x1B\x03\x16\x99\x8A\x80\x96a!.V[P\x86Q\x90\x81R\xA3a\x1C\x1E\x82Q\x80\x92a\x19\nV[`\x02` \x82\x01R\xA3_\x80\x80\x80a\x1B\x8FV[P`\xFF\x84T`H\x1C\x16`\x05\x81\x10\x15a\x05\xC0W`\x02\x14\x15a\x1B\x8AV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[\x90`\x01`\x01`@\x1B\x03a\x1Cp\x83a\x1D\x04V[\x92\x16_R`\x01` R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ `@Q\x90a\x1C\x9C\x82a\x17\xD4V[\x80T\x82R`\xFF`\x01\x82\x01T`\x01`\x01`@\x1B\x03\x81\x16` \x85\x01R\x81\x81`@\x1C\x16`@\x85\x01R`H\x1C\x16\x90`\x05\x82\x10\x15a\x05\xC0W`\x02\x91``\x84\x01R\x01T`\x80\x82\x01RQ\x80\x15a\x1C\xFEWa\x1C\xF7`\x01`\x01`@\x1B\x03\x91Ba\x1A\xC4V[\x91Q\x16\x11\x90V[PP_\x90V[`\x01`\x01`@\x1B\x03\x90a\x1D\x15a\x1AOV[P\x16_R_` R`@_ `@Q\x90a\x1D.\x82a\x18\nV[T`\x01`\x01`@\x1B\x03\x81\x16\x80\x83R`\xFF\x82`@\x1C\x16\x90`\xFF` \x85\x01\x93\x83\x85R`H\x1C\x16\x15\x15`@\x85\x01R\x15a\x1DoW[\x15a\x1DhWP\x90V[`\x03\x90R\x90V[a\x01,\x83Ra\x1D_V[\x93\x94\x91\x92\x90`\x01`\x01`@\x1B\x03\x85\x16\x94\x85_R`\x01` R`@_ `\x01\x80`\xA0\x1B\x03\x86\x16_R` R`@_ \x94a\x1D\xB1\x82a\x1D\x04V[\x90`\x01\x87\x01\x94`\xFF\x86T`H\x1C\x16\x97B\x81U`\x02a\x1D\xD1\x8C\x8A6\x91a\x19\x96V[` \x81Q\x91\x01 \x91\x01U`\xFF`@\x1B\x19\x86T\x16\x86U`\x01`\x01`@\x1B\x03\x86T\x16\x90`\x01`\x01`@\x1B\x03\x82\x14a\x1A\xD1W`\x01`\x01`@\x1B\x03`\x01`\xFF\x93\x01\x16`\x01`\x01`@\x1B\x03\x19\x88T\x16\x17\x87U\x16\x92\x83\x15_\x14a\x1F\x88W_\x96[`\x05\x88\x10\x15\x96\x87a\x05\xC0W\x80T`\xFF`H\x1B\x19\x16`H\x8A\x90\x1B`\xFF`H\x1B\x16\x17\x90U`\x05\x89\x10\x15a\x05\xC0W`\x01`\x01`@\x1B\x03\x9A\x8A\x94`\x02\x8B\x14\x8A\x81a\x1FyW[P\x91\x85`@\x7Fe\x89\x18\xE3\x14\x7F\x13\xDD\x06\x8E\xC2\x147\xB4\xC2\\!h*\x8D\xC2\x12\x93Hg\x1E\xAD\0\r\xB3\xE7\xB9\x97\x95\x93\x81\x97\x95a\x1F,W[\x01Q\x15\x15\x80a\x1F#W[a\x1F\x11W[PP\x83Q\x96\x87RPPB` \x86\x01R`\x01`\x01`\xA0\x1B\x03\x16\x99\x8A\x95\x16\x93\xA4a\x05\xC0W\x80\x82\x03a\x1E\xDCW[PPPPV[_\x80Q` a#z\x839\x81Q\x91R\x91a\x1F\x07`@\x92a\x1E\xFD\x84Q\x80\x94a\x19\nV[` \x83\x01\x90a\x19\nV[\xA3_\x80\x80\x80a\x1E\xD6V[a\x1F\x1A\x93a !V[_\x80\x80\x83a\x1E\xACV[P\x82\x15\x15a\x1E\xA7V[_\x8A\x81R`\x02` R\x82\x90 `\x01`\x01`\xA0\x1B\x03\x84\x16\x90a\x1FN\x90\x82\x90a\"\tV[P\x8A\x7F\xC9\x86,_\x02\xEE\xFB\xDC\xEA\x01\xC2\x07\xAES\x8E\x1D0M\xC90&\x87\x0FH\x95\x1EH\xA0\xF4\xC8G\x0C_\x80\xA3a\x1E\x9DV[_\x9AP`\x02\x14\x15\x90P\x8Aa\x1ElV[`d\x84\x10\x15a\x1F\x99W`\x01\x96a\x1E+V[`\x01\x96`\xC8\x85\x10a\x1E+W`@Q` \x81R`\x1B` \x82\x01R\x7FProtocol violation reported\0\0\0\0\0`@\x82\x01R\x8A\x7F\x1E)\t\xCFE\xD7\x0C\xF0\x03\xF34\xB7<\x933\x0C\xE7\xE5rx-\xFC\x82\xFA\xB7\x9D\xEB\x88U\xA7\xC7\x91```\x01\x80`\xA0\x1B\x03\x87\x16\x93\xA3a\x1E+V[\x80T\x82\x10\x15a\x13\xF9W_R` _ \x01\x90_\x90V[`@\x84\x10a\x1E\xD6W_\x92[`@\x84\x01\x80\x85\x11a\x1A\xD1W\x85\x10a\x1BWW\x84\x84\x11a\x01\xCCW\x83\x81\x01\x84\x86\x03\x81\x01`@\x82\x82\x03\x12a\x01\xCCW\x815`\x01`\x01`@\x1B\x03\x81\x11a\x01\xCCW` \x91a t\x91\x84\x01a\x19\xDBV[\x91\x015`\x01`\x01`@\x1B\x03\x84\x16\x80_R`\x05` R`@_ `\x01\x80`\xA0\x1B\x03\x87\x16_R` R\x81`@_ ` `@Q\x80\x92\x87Q\x80\x84\x8A\x01\x84^\x82\x01\x90\x81R\x03\x01\x90 U\x7F#\xED\x02\xBD6\x05\xBD\xEAj\x8A\xFAv\xC4o\0\xD2t\x86\x0B\xA6\xCE\xA9\x80\xF2X[im\xF9\xE1\x82\xBD`@Q`@\x81Ra \xEE`@\x82\x01\x86a\x18\xE6V[` \x82\x01\x94\x90\x94R`\x01`\x01`\xA0\x1B\x03\x88\x16\x93\x81\x90\x03\x90\xA3Q\x80`@\x01`@\x11a\x1A\xD1W`@\x90\x85\x01\x01\x80\x94\x11a\x1A\xD1W\x84\x84\x11\x15a ,WPPPPPV[\x90`\x01\x82\x01\x91\x81_R\x82` R`@_ T\x80\x15\x15_\x14a\"\x01W_\x19\x81\x01\x81\x81\x11a\x1A\xD1W\x82T_\x19\x81\x01\x91\x90\x82\x11a\x1A\xD1W\x81\x81\x03a!\xB6W[PPP\x80T\x80\x15a!\xA2W_\x19\x01\x90a!\x83\x82\x82a \x0CV[\x81T\x90_\x19\x90`\x03\x1B\x1B\x19\x16\x90UU_R` R_`@\x81 U`\x01\x90V[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[a!\xECa!\xC6a!\xD6\x93\x86a \x0CV[\x90T\x90`\x03\x1B\x1C\x92\x83\x92\x86a \x0CV[\x81\x93\x91T\x90`\x03\x1B\x91\x82\x1B\x91_\x19\x90\x1B\x19\x16\x17\x90V[\x90U_R\x83` R`@_ U_\x80\x80a!jV[PPPP_\x90V[`\x01\x81\x01\x90\x82_R\x81` R`@_ T\x15_\x14a\"VW\x80T`\x01`@\x1B\x81\x10\x15a\x0C\xB8Wa\"Ca!\xD6\x82`\x01\x87\x94\x01\x85U\x84a \x0CV[\x90UT\x91_R` R`@_ U`\x01\x90V[PPP_\x90V[\x81Q\x91\x90`A\x83\x03a\"\x8DWa\"\x86\x92P` \x82\x01Q\x90```@\x84\x01Q\x93\x01Q_\x1A\x90a\"\xF7V[\x91\x92\x90\x91\x90V[PP_\x91`\x02\x91\x90V[`\x04\x81\x10\x15a\x05\xC0W\x80a\"\xA9WPPV[`\x01\x81\x03a\"\xC0Wc\xF6E\xEE\xDF`\xE0\x1B_R`\x04_\xFD[`\x02\x81\x03a\"\xDBWPc\xFC\xE6\x98\xF7`\xE0\x1B_R`\x04R`$_\xFD[`\x03\x14a\"\xE5WPV[c5\xE2\xF3\x83`\xE2\x1B_R`\x04R`$_\xFD[\x91\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11a#nW\x91` \x93`\x80\x92`\xFF_\x95`@Q\x94\x85R\x16\x86\x84\x01R`@\x83\x01R``\x82\x01R\x82\x80R`\x01Z\xFA\x15a\x0FSW_Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a#dW\x90_\x90_\x90V[P_\x90`\x01\x90_\x90V[PPP_\x91`\x03\x91\x90V\xFE\"\x88$\xB8l%di\x12_R\\\xE1\x8Cl-\n\x9E\x13=\x13\xB8\xECz,\x96\xA1\x93\xB0\xC2\x8A\t\xA1dsolcC\0\x08\x1A\0\n",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080806040526004361015610012575f80fd5b5f3560e01c908163057785501461162f575080630758236f146115d95780630c76697a1461158657806322f1ec93146114f65780632c957688146114da5780633644e515146114a05780633e6e34a71461140d57806340235a9c146113415780635685cf681461129157806356c4e17d1461125157806359dcea12146111f05780635a936dc6146111ac5780635cce98a61461115457806361d6b86c1461113957806362c7e8fc146110d657806371e7388c14610fd85780637b9f64b214610fa057806384ef732214610f5e57806396686c1e14610e765780639cbdae2214610deb578063adff830c14610cdf578063ae470a8514610ab6578063b074e9dd14610995578063b99f67591461073a578063ba1fb10314610710578063c1ef9ddf146105d4578063c5d960bb146104ce578063cfe34749146104a6578063d413a5801461032b578063d551162c146102d8578063da435a7c1461027f578063ee1c039014610249578063f9107f3b146101d05763f9f1676214610192575f80fd5b346101cc575f3660031901126101cc5760206040517fe1675f8364c07a4d60a07503f0d700a7bcacd82251dff0f070e5235de6c6d28a8152f35b5f80fd5b346101cc5760403660031901126101cc576101e9611741565b60243580151581036101cc576001600160401b036102479216805f52600360205261022160018060a01b0360405f2054163314611a84565b5f525f60205260405f209060ff60481b825491151560481b169060ff60481b1916179055565b005b346101cc5760403660031901126101cc576020610275610267611741565b61026f61176d565b90611c5e565b6040519015158152f35b346101cc5760203660031901126101cc576001600160401b036102a0611741565b165f525f602052606060405f205460ff604051916001600160401b0381168352818160401c16602084015260481c1615156040820152f35b346101cc5760206001600160401b03816102f1366119f9565b949092165f526005835260405f209060018060a01b03165f52825260405f2083604051948593843782019081520301902054604051908152f35b346101cc5760a03660031901126101cc57610344611741565b61034c611757565b90610355611959565b906064356001600160401b0381116101cc57610375903690600401611969565b9290916084356001600160401b0381116101cc576104486104426103a0610451933690600401611969565b919060405160208101906001600160401b0360c01b8860c01b1682526001600160401b0360c01b8c60c01b1660288201528a8a60308301376103f56030828d81015f838201520301601f198101835282611825565b51902060405160208101917f19457468657265756d205369676e6564204d6573736167653a0a3332000000008352603c820152603c8152610437605c82611825565b519020923691611996565b9061225d565b90929192612297565b336001600160a01b039091160361046d57610247943391611d79565b60405162461bcd60e51b8152602060048201526011602482015270496e76616c6964207369676e617475726560781b6044820152606490fd5b346101cc575f3660031901126101cc576006546040516001600160a01b039091168152602090f35b346101cc5760203660031901126101cc576001600160401b036104ef611741565b165f81815260016020818152604080842033855290915290912001805460481c60ff16919060058310156105c0576003831461057b57690400000000000000000060ff60481b19825416179055805f5260026020526105513360405f2061212e565b5061055f604051809361190a565b600460208301525f8051602061237a83398151915260403393a3005b60405162461bcd60e51b815260206004820152601f60248201527f43616e6e6f7420676f206f66666c696e65207768696c6520736c6173686564006044820152606490fd5b634e487b7160e01b5f52602160045260245ffd5b346101cc5760203660031901126101cc576001600160401b036105f5611741565b165f52600460205260405f2080549061060d82611a6d565b9161061b6040519384611825565b8083526020830180925f5260205f205f915b8383106106c357848660405191829160208301906020845251809152604083019060408160051b85010192915f905b82821061066b57505050500390f35b919360019193955060208091603f1989820301855287519060608061069984516080855260808501906118e6565b9385810151868501526040810151604085015201511515910152960192019201859493919261065c565b600460206001926040516106d6816117ef565b6106df86611846565b815284860154838201526002860154604082015260ff6003870154161515606082015281520192019201919061062d565b346101cc5760403660031901126101cc5761024761072c611741565b61073461176d565b90611ae5565b346101cc5760603660031901126101cc57610753611741565b61075b611757565b610763611959565b90337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614801561096f575b8015610947575b15610911576001600160401b031690603c82106108d75760ff169160018310610892577fc9599ed962624a858ec59bae0ed86c75f4db65fe04570021277edbedd04ea564916001600160401b036040921693845f525f60205261088560ff845f205460481c168451906108108261180a565b84825261086a60ff6020840186815288850193151584528a5f525f6020526001600160401b03808a5f20965116166001600160401b03198654161785555116839060ff60401b82549160401b169060ff60401b1916179055565b51815460ff60481b191690151560481b60ff60481b16179055565b82519182526020820152a2005b60405162461bcd60e51b815260206004820152601760248201527f4d6178206d6973736564206d757374206265203e3d20310000000000000000006044820152606490fd5b60405162461bcd60e51b8152602060048201526012602482015271125b9d195c9d985b081d1bdbc81cda1bdc9d60721b6044820152606490fd5b60405162461bcd60e51b815260206004820152600e60248201526d139bdd08185d5d1a1bdc9a5e995960921b6044820152606490fd5b506001600160401b0383165f908152600360205260409020546001600160a01b03161561079e565b506001600160401b0383165f52600360205260018060a01b0360405f2054163314610797565b346101cc5760203660031901126101cc576001600160401b036109b6611741565b165f81815260016020818152604080842033855290915290912001805460481c60ff16919060058310156105c05760038314610a7157805469ffff0000000000000000191669010000000000000000001790555f818152600260205260409020610a21903390612209565b50610a55604051809333847fc9862c5f02eefbdcea01c207ae538e1d304dc93026870f48951e48a0f4c8470c5f80a361190a565b600160208301525f8051602061237a83398151915260403393a3005b60405162461bcd60e51b815260206004820152601e60248201527f43616e6e6f7420676f206f6e6c696e65207768696c6520736c617368656400006044820152606490fd5b346101cc5760a03660031901126101cc57610acf611741565b6024356001600160401b0381116101cc57610aee903690600401611969565b90916084358015159291908390036101cc576001600160401b0316805f526003602052610b2860018060a01b0360405f2054163314611a84565b5f526004602052610b4b60405f209360405192610b44846117ef565b3691611996565b81526020810192604435845260408201906064358252606083019384528054600160401b811015610cb857610b8591600182018155611783565b929092610ccc5751938451946001600160401b038611610cb857610ba9845461179c565b601f8111610c73575b50602090601f8711600114610c0e57958091600396975f92610c03575b50508160011b915f1990871b1c19161783555b516001830155516002820155019051151560ff801983541691161790555f80f35b015190508780610bcf565b90601f19871691855f52815f20925f5b818110610c5b57509160019391896003999a9410610c44575b505050811b018355610be2565b01515f1983891b60f8161c19169055878080610c37565b92936020600181928786015181550195019301610c1e565b845f5260205f20601f880160051c81019160208910610cae575b601f0160051c01905b818110610ca35750610bb2565b5f8155600101610c96565b9091508190610c8d565b634e487b7160e01b5f52604160045260245ffd5b634e487b7160e01b5f525f60045260245ffd5b346101cc57610ced366119f9565b600654929391926001600160a01b03163303610db05760406001600160401b037f1e2909cf45d70cf003f334b73c93330ce7e572782dfc82fab79deb8855a7c791931693845f526001602052815f2060018060a01b0387165f526020526001825f2001690300000000000000000060ff60481b19825416179055845f526002602052610d85825f209660018060a01b0316809761212e565b5082825193849260208452816020850152848401375f828201840152601f01601f19168101030190a3005b60405162461bcd60e51b81526020600482015260136024820152724e6f7420736c617368696e67206f7261636c6560681b6044820152606490fd5b346101cc5760603660031901126101cc57610e04611741565b610e0c61176d565b6044356001600160401b0381116101cc5760209283926001600160401b03610e39859436906004016119db565b92165f526005835260405f209060018060a01b03165f52825260405f20604051938285935191829101845e82019081520301902054604051908152f35b346101cc5760403660031901126101cc57610e8f611741565b602435906001600160401b0382116101cc57366023830112156101cc578160040135916001600160401b0383116101cc573660248460051b830101116101cc5782906001600160401b035f9316925b82811015610247576024600582901b830101356001600160a01b03811691908290036101cc57303b156101cc576040519163ba1fb10360e01b835285600484015260248301525f8260448183305af1918215610f5357600192610f43575b5001610ede565b5f610f4d91611825565b85610f3c565b6040513d5f823e3d90fd5b346101cc5760203660031901126101cc576004356001600160a01b038116908190036101cc576bffffffffffffffffffffffff60a01b60065416176006555f80f35b346101cc5760203660031901126101cc576001600160401b03610fc1611741565b165f526002602052602060405f2054604051908152f35b346101cc5760403660031901126101cc57610ff1611741565b6001600160401b0361100161176d565b915f6080604051611011816117d4565b8281528260208201528260408201528260608201520152165f52600160205260405f209060018060a01b03165f5260205260405f20604051611052816117d4565b8154815260018201549160208201906001600160401b038416825260ff6040840194818160401c16865260481c16606084019060058110156105c05760a0956001600160401b0360026110ce9560ff9486520154956080880196875260405197518852511660208701525116604085015251606084019061190a565b516080820152f35b346101cc5760403660031901126101cc576110ef611741565b6001600160401b036110ff61176d565b91165f52600160205260405f209060018060a01b03165f52602052602060ff600160405f20015460481c16611137604051809261190a565bf35b346101cc575f3660031901126101cc57602060405160038152f35b346101cc5760803660031901126101cc5761116d611741565b611175611757565b9061117e611959565b91606435926001600160401b0384116101cc576111a2610247943690600401611969565b9390923391611d79565b346101cc575f3660031901126101cc576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346101cc5760203660031901126101cc57611209611741565b50604051610ca061121a8183611825565b60648252601f19019036906020013761124d60405161123a602082611825565b5f81525f36813760405191829182611917565b0390f35b346101cc5760203660031901126101cc576001600160401b03611272611741565b165f526003602052602060018060a01b0360405f205416604051908152f35b346101cc5760403660031901126101cc576112aa611741565b6001600160401b036112ba61176d565b9116805f52600160205260405f2060018060a01b0383165f5260205260ff600160405f20015460481c1660058110156105c05715908115611303575b6020826040519015158152f35b90505f52600160205260405f209060018060a01b03165f5260205260ff600160405f20015460481c1660058110156105c057600160209114826112f6565b346101cc5760203660031901126101cc576001600160401b03611362611741565b16805f52600260205260405f205461137981611a6d565b916113876040519384611825565b818352601f1961139683611a6d565b013660208501375f5b8281106113b4576040518061124d8682611917565b815f5260026020526113c98160405f2061200c565b9054855160039290921b1c6001600160a01b031691908110156113f95760019160208260051b870101520161139f565b634e487b7160e01b5f52603260045260245ffd5b346101cc5760403660031901126101cc57611426611741565b6001600160401b0361143661176d565b91165f52600160205260405f209060018060a01b03165f5260205260a060405f2080549061149960026001830154920154916040519384526001600160401b038116602085015260ff8160401c16604085015260ff606085019160481c1661190a565b6080820152f35b346101cc575f3660031901126101cc5760206040517f00000000000000000000000000000000000000000000000000000000000000008152f35b346101cc575f3660031901126101cc57602060405161012c8152f35b346101cc5760403660031901126101cc5761150f611741565b6001600160401b0360243591165f52600460205260405f2080548210156101cc576115709161153d91611783565b5061154781611846565b9060018101549060ff6003600283015492015416906040519485946080865260808601906118e6565b9260208501526040840152151560608301520390f35b346101cc5760403660031901126101cc5761159f611741565b6001600160401b036115af61176d565b91165f52600160205260405f209060018060a01b03165f52602052602060405f2054604051908152f35b346101cc5760203660031901126101cc5760606116056115f7611741565b6115ff611a4f565b50611d04565b60408051916001600160401b03815116835260ff6020820151166020840152015115156040820152f35b346101cc5760403660031901126101cc57611648611741565b61165061176d565b917f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316330361170c57506001600160401b03165f818152600360205260409020546001600160a01b03166116d2575f90815260036020526040902080546001600160a01b0319166001600160a01b03909216919091179055005b60405162461bcd60e51b8152602060048201526012602482015271105b1c9958591e481c9959da5cdd195c995960721b6044820152606490fd5b62461bcd60e51b815260206004820152601060248201526f4f6e6c792054616e676c6520636f726560801b6044820152606490fd5b600435906001600160401b03821682036101cc57565b602435906001600160401b03821682036101cc57565b602435906001600160a01b03821682036101cc57565b80548210156113f9575f5260205f209060021b01905f90565b90600182811c921680156117ca575b60208310146117b657565b634e487b7160e01b5f52602260045260245ffd5b91607f16916117ab565b60a081019081106001600160401b03821117610cb857604052565b608081019081106001600160401b03821117610cb857604052565b606081019081106001600160401b03821117610cb857604052565b90601f801991011681019081106001600160401b03821117610cb857604052565b9060405191825f8254926118598461179c565b80845293600181169081156118c45750600114611880575b5061187e92500383611825565b565b90505f9291925260205f20905f915b8183106118a857505090602061187e928201015f611871565b602091935080600191548385890101520191019091849261188f565b90506020925061187e94915060ff191682840152151560051b8201015f611871565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b9060058210156105c05752565b60206040818301928281528451809452019201905f5b81811061193a5750505090565b82516001600160a01b031684526020938401939092019160010161192d565b6044359060ff821682036101cc57565b9181601f840112156101cc578235916001600160401b0383116101cc57602083818601950101116101cc57565b9291926001600160401b038211610cb857604051916119bf601f8201601f191660200184611825565b8294818452818301116101cc578281602093845f960137010152565b9080601f830112156101cc578160206119f693359101611996565b90565b60606003198201126101cc576004356001600160401b03811681036101cc57916024356001600160a01b03811681036101cc5791604435906001600160401b0382116101cc57611a4b91600401611969565b9091565b60405190611a5c8261180a565b5f6040838281528260208201520152565b6001600160401b038111610cb85760051b60200190565b15611a8b57565b60405162461bcd60e51b81526020600482015260116024820152702737ba1039b2b93b34b1b29037bbb732b960791b6044820152606490fd5b91908203918211611ad157565b634e487b7160e01b5f52601160045260245ffd5b906001600160401b03821690815f52600160205260405f2060018060a01b0382165f52602052611b1860405f2093611d04565b9280548015611b5757611b2b9042611ac4565b6001600160401b03855116908115611c4a5760019160ff91041691019360ff855460401c168211611b5e575b5050505050565b845468ffffffffffffffffff191660ff60401b604084901b161785556020015160ff1681101580611c2f575b611b95575b80611b57565b835f8051602061237a83398151915292847f44fd32b677704ce68e7763897c49733b8f5289018ac60a5c926802d63759db4d602060409560ff611c0b9a5460481c1695690200000000000000000060ff60481b19825416179055835f5260028252865f209460018060a01b0316998a809661212e565b508651908152a3611c1e8251809261190a565b60026020820152a35f808080611b8f565b5060ff845460481c1660058110156105c05760021415611b8a565b634e487b7160e01b5f52601260045260245ffd5b906001600160401b03611c7083611d04565b92165f52600160205260405f209060018060a01b03165f5260205260405f2060405190611c9c826117d4565b8054825260ff60018201546001600160401b0381166020850152818160401c16604085015260481c169060058210156105c057600291606084015201546080820152518015611cfe57611cf76001600160401b039142611ac4565b9151161190565b50505f90565b6001600160401b0390611d15611a4f565b50165f525f60205260405f2060405190611d2e8261180a565b546001600160401b03811680835260ff8260401c169060ff602085019383855260481c161515604085015215611d6f575b15611d68575090565b6003905290565b61012c8352611d5f565b93949192906001600160401b03851694855f52600160205260405f2060018060a01b0386165f5260205260405f2094611db182611d04565b90600187019460ff865460481c16974281556002611dd18c8a3691611996565b6020815191012091015560ff60401b1986541686556001600160401b03865416906001600160401b038214611ad1576001600160401b03600160ff9301166001600160401b0319885416178755169283155f14611f88575f965b600588101596876105c057805460ff60481b191660488a901b60ff60481b1617905560058910156105c0576001600160401b039a8a9460028b148a81611f79575b50918560407f658918e3147f13dd068ec21437b4c25c21682a8dc2129348671ead000db3e7b9979593819795611f2c575b0151151580611f23575b611f11575b5050835196875250504260208601526001600160a01b0316998a951693a46105c057808203611edc575b50505050565b5f8051602061237a83398151915291611f07604092611efd8451809461190a565b602083019061190a565ba35f808080611ed6565b611f1a93612021565b5f808083611eac565b50821515611ea7565b5f8a815260026020528290206001600160a01b03841690611f4e908290612209565b508a7fc9862c5f02eefbdcea01c207ae538e1d304dc93026870f48951e48a0f4c8470c5f80a3611e9d565b5f9a506002141590508a611e6c565b6064841015611f9957600196611e2b565b60019660c88510611e2b5760405160208152601b60208201527f50726f746f636f6c2076696f6c6174696f6e207265706f72746564000000000060408201528a7f1e2909cf45d70cf003f334b73c93330ce7e572782dfc82fab79deb8855a7c791606060018060a01b03871693a3611e2b565b80548210156113f9575f5260205f2001905f90565b60408410611ed6575f925b60408401808511611ad1578510611b57578484116101cc5783810184860381016040828203126101cc5781356001600160401b0381116101cc576020916120749184016119db565b9101356001600160401b038416805f52600560205260405f2060018060a01b0387165f526020528160405f2060206040518092875180848a01845e820190815203019020557f23ed02bd3605bdea6a8afa76c46f00d274860ba6cea980f2585b696df9e182bd604051604081526120ee60408201866118e6565b60208201949094526001600160a01b0388169381900390a35180604001604011611ad157604090850101809411611ad1578484111561202c575050505050565b906001820191815f528260205260405f20548015155f14612201575f198101818111611ad15782545f19810191908211611ad1578181036121b6575b505050805480156121a2575f190190612183828261200c565b8154905f199060031b1b19169055555f526020525f6040812055600190565b634e487b7160e01b5f52603160045260245ffd5b6121ec6121c66121d6938661200c565b90549060031b1c9283928661200c565b819391549060031b91821b915f19901b19161790565b90555f528360205260405f20555f808061216a565b505050505f90565b6001810190825f528160205260405f2054155f14612256578054600160401b811015610cb8576122436121d682600187940185558461200c565b905554915f5260205260405f2055600190565b5050505f90565b815191906041830361228d576122869250602082015190606060408401519301515f1a906122f7565b9192909190565b50505f9160029190565b60048110156105c057806122a9575050565b600181036122c05763f645eedf60e01b5f5260045ffd5b600281036122db575063fce698f760e01b5f5260045260245ffd5b6003146122e55750565b6335e2f38360e21b5f5260045260245ffd5b91907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0841161236e579160209360809260ff5f9560405194855216868401526040830152606082015282805260015afa15610f53575f516001600160a01b0381161561236457905f905f90565b505f906001905f90565b5050505f916003919056fe228824b86c256469125f525ce18c6c2d0a9e133d13b8ec7a2c96a193b0c28a09a164736f6c634300081a000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_5`\xE0\x1C\x90\x81c\x05w\x85P\x14a\x16/WP\x80c\x07X#o\x14a\x15\xD9W\x80c\x0Cviz\x14a\x15\x86W\x80c\"\xF1\xEC\x93\x14a\x14\xF6W\x80c,\x95v\x88\x14a\x14\xDAW\x80c6D\xE5\x15\x14a\x14\xA0W\x80c>n4\xA7\x14a\x14\rW\x80c@#Z\x9C\x14a\x13AW\x80cV\x85\xCFh\x14a\x12\x91W\x80cV\xC4\xE1}\x14a\x12QW\x80cY\xDC\xEA\x12\x14a\x11\xF0W\x80cZ\x93m\xC6\x14a\x11\xACW\x80c\\\xCE\x98\xA6\x14a\x11TW\x80ca\xD6\xB8l\x14a\x119W\x80cb\xC7\xE8\xFC\x14a\x10\xD6W\x80cq\xE78\x8C\x14a\x0F\xD8W\x80c{\x9Fd\xB2\x14a\x0F\xA0W\x80c\x84\xEFs\"\x14a\x0F^W\x80c\x96hl\x1E\x14a\x0EvW\x80c\x9C\xBD\xAE\"\x14a\r\xEBW\x80c\xAD\xFF\x83\x0C\x14a\x0C\xDFW\x80c\xAEG\n\x85\x14a\n\xB6W\x80c\xB0t\xE9\xDD\x14a\t\x95W\x80c\xB9\x9FgY\x14a\x07:W\x80c\xBA\x1F\xB1\x03\x14a\x07\x10W\x80c\xC1\xEF\x9D\xDF\x14a\x05\xD4W\x80c\xC5\xD9`\xBB\x14a\x04\xCEW\x80c\xCF\xE3GI\x14a\x04\xA6W\x80c\xD4\x13\xA5\x80\x14a\x03+W\x80c\xD5Q\x16,\x14a\x02\xD8W\x80c\xDACZ|\x14a\x02\x7FW\x80c\xEE\x1C\x03\x90\x14a\x02IW\x80c\xF9\x10\x7F;\x14a\x01\xD0Wc\xF9\xF1gb\x14a\x01\x92W_\x80\xFD[4a\x01\xCCW_6`\x03\x19\x01\x12a\x01\xCCW` `@Q\x7F\xE1g_\x83d\xC0zM`\xA0u\x03\xF0\xD7\0\xA7\xBC\xAC\xD8\"Q\xDF\xF0\xF0p\xE5#]\xE6\xC6\xD2\x8A\x81R\xF3[_\x80\xFD[4a\x01\xCCW`@6`\x03\x19\x01\x12a\x01\xCCWa\x01\xE9a\x17AV[`$5\x80\x15\x15\x81\x03a\x01\xCCW`\x01`\x01`@\x1B\x03a\x02G\x92\x16\x80_R`\x03` Ra\x02!`\x01\x80`\xA0\x1B\x03`@_ T\x163\x14a\x1A\x84V[_R_` R`@_ \x90`\xFF`H\x1B\x82T\x91\x15\x15`H\x1B\x16\x90`\xFF`H\x1B\x19\x16\x17\x90UV[\0[4a\x01\xCCW`@6`\x03\x19\x01\x12a\x01\xCCW` a\x02ua\x02ga\x17AV[a\x02oa\x17mV[\x90a\x1C^V[`@Q\x90\x15\x15\x81R\xF3[4a\x01\xCCW` 6`\x03\x19\x01\x12a\x01\xCCW`\x01`\x01`@\x1B\x03a\x02\xA0a\x17AV[\x16_R_` R```@_ T`\xFF`@Q\x91`\x01`\x01`@\x1B\x03\x81\x16\x83R\x81\x81`@\x1C\x16` \x84\x01R`H\x1C\x16\x15\x15`@\x82\x01R\xF3[4a\x01\xCCW` `\x01`\x01`@\x1B\x03\x81a\x02\xF16a\x19\xF9V[\x94\x90\x92\x16_R`\x05\x83R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R\x82R`@_ \x83`@Q\x94\x85\x93\x847\x82\x01\x90\x81R\x03\x01\x90 T`@Q\x90\x81R\xF3[4a\x01\xCCW`\xA06`\x03\x19\x01\x12a\x01\xCCWa\x03Da\x17AV[a\x03La\x17WV[\x90a\x03Ua\x19YV[\x90`d5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xCCWa\x03u\x906\x90`\x04\x01a\x19iV[\x92\x90\x91`\x845`\x01`\x01`@\x1B\x03\x81\x11a\x01\xCCWa\x04Ha\x04Ba\x03\xA0a\x04Q\x936\x90`\x04\x01a\x19iV[\x91\x90`@Q` \x81\x01\x90`\x01`\x01`@\x1B\x03`\xC0\x1B\x88`\xC0\x1B\x16\x82R`\x01`\x01`@\x1B\x03`\xC0\x1B\x8C`\xC0\x1B\x16`(\x82\x01R\x8A\x8A`0\x83\x017a\x03\xF5`0\x82\x8D\x81\x01_\x83\x82\x01R\x03\x01`\x1F\x19\x81\x01\x83R\x82a\x18%V[Q\x90 `@Q` \x81\x01\x91\x7F\x19Ethereum Signed Message:\n32\0\0\0\0\x83R`<\x82\x01R`<\x81Ra\x047`\\\x82a\x18%V[Q\x90 \x926\x91a\x19\x96V[\x90a\"]V[\x90\x92\x91\x92a\"\x97V[3`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x03a\x04mWa\x02G\x943\x91a\x1DyV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01RpInvalid signature`x\x1B`D\x82\x01R`d\x90\xFD[4a\x01\xCCW_6`\x03\x19\x01\x12a\x01\xCCW`\x06T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x01\xCCW` 6`\x03\x19\x01\x12a\x01\xCCW`\x01`\x01`@\x1B\x03a\x04\xEFa\x17AV[\x16_\x81\x81R`\x01` \x81\x81R`@\x80\x84 3\x85R\x90\x91R\x90\x91 \x01\x80T`H\x1C`\xFF\x16\x91\x90`\x05\x83\x10\x15a\x05\xC0W`\x03\x83\x14a\x05{Wi\x04\0\0\0\0\0\0\0\0\0`\xFF`H\x1B\x19\x82T\x16\x17\x90U\x80_R`\x02` Ra\x05Q3`@_ a!.V[Pa\x05_`@Q\x80\x93a\x19\nV[`\x04` \x83\x01R_\x80Q` a#z\x839\x81Q\x91R`@3\x93\xA3\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FCannot go offline while slashed\0`D\x82\x01R`d\x90\xFD[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[4a\x01\xCCW` 6`\x03\x19\x01\x12a\x01\xCCW`\x01`\x01`@\x1B\x03a\x05\xF5a\x17AV[\x16_R`\x04` R`@_ \x80T\x90a\x06\r\x82a\x1AmV[\x91a\x06\x1B`@Q\x93\x84a\x18%V[\x80\x83R` \x83\x01\x80\x92_R` _ _\x91[\x83\x83\x10a\x06\xC3W\x84\x86`@Q\x91\x82\x91` \x83\x01\x90` \x84RQ\x80\x91R`@\x83\x01\x90`@\x81`\x05\x1B\x85\x01\x01\x92\x91_\x90[\x82\x82\x10a\x06kWPPPP\x03\x90\xF3[\x91\x93`\x01\x91\x93\x95P` \x80\x91`?\x19\x89\x82\x03\x01\x85R\x87Q\x90``\x80a\x06\x99\x84Q`\x80\x85R`\x80\x85\x01\x90a\x18\xE6V[\x93\x85\x81\x01Q\x86\x85\x01R`@\x81\x01Q`@\x85\x01R\x01Q\x15\x15\x91\x01R\x96\x01\x92\x01\x92\x01\x85\x94\x93\x91\x92a\x06\\V[`\x04` `\x01\x92`@Qa\x06\xD6\x81a\x17\xEFV[a\x06\xDF\x86a\x18FV[\x81R\x84\x86\x01T\x83\x82\x01R`\x02\x86\x01T`@\x82\x01R`\xFF`\x03\x87\x01T\x16\x15\x15``\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x06-V[4a\x01\xCCW`@6`\x03\x19\x01\x12a\x01\xCCWa\x02Ga\x07,a\x17AV[a\x074a\x17mV[\x90a\x1A\xE5V[4a\x01\xCCW``6`\x03\x19\x01\x12a\x01\xCCWa\x07Sa\x17AV[a\x07[a\x17WV[a\x07ca\x19YV[\x903\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14\x80\x15a\toW[\x80\x15a\tGW[\x15a\t\x11W`\x01`\x01`@\x1B\x03\x16\x90`<\x82\x10a\x08\xD7W`\xFF\x16\x91`\x01\x83\x10a\x08\x92W\x7F\xC9Y\x9E\xD9bbJ\x85\x8E\xC5\x9B\xAE\x0E\xD8lu\xF4\xDBe\xFE\x04W\0!'~\xDB\xED\xD0N\xA5d\x91`\x01`\x01`@\x1B\x03`@\x92\x16\x93\x84_R_` Ra\x08\x85`\xFF\x84_ T`H\x1C\x16\x84Q\x90a\x08\x10\x82a\x18\nV[\x84\x82Ra\x08j`\xFF` \x84\x01\x86\x81R\x88\x85\x01\x93\x15\x15\x84R\x8A_R_` R`\x01`\x01`@\x1B\x03\x80\x8A_ \x96Q\x16\x16`\x01`\x01`@\x1B\x03\x19\x86T\x16\x17\x85UQ\x16\x83\x90`\xFF`@\x1B\x82T\x91`@\x1B\x16\x90`\xFF`@\x1B\x19\x16\x17\x90UV[Q\x81T`\xFF`H\x1B\x19\x16\x90\x15\x15`H\x1B`\xFF`H\x1B\x16\x17\x90UV[\x82Q\x91\x82R` \x82\x01R\xA2\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FMax missed must be >= 1\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rq\x12[\x9D\x19\\\x9D\x98[\x08\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x82\x01R`d\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x13\x9B\xDD\x08\x18]]\x1A\x1B\xDC\x9A^\x99Y`\x92\x1B`D\x82\x01R`d\x90\xFD[P`\x01`\x01`@\x1B\x03\x83\x16_\x90\x81R`\x03` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x07\x9EV[P`\x01`\x01`@\x1B\x03\x83\x16_R`\x03` R`\x01\x80`\xA0\x1B\x03`@_ T\x163\x14a\x07\x97V[4a\x01\xCCW` 6`\x03\x19\x01\x12a\x01\xCCW`\x01`\x01`@\x1B\x03a\t\xB6a\x17AV[\x16_\x81\x81R`\x01` \x81\x81R`@\x80\x84 3\x85R\x90\x91R\x90\x91 \x01\x80T`H\x1C`\xFF\x16\x91\x90`\x05\x83\x10\x15a\x05\xC0W`\x03\x83\x14a\nqW\x80Ti\xFF\xFF\0\0\0\0\0\0\0\0\x19\x16i\x01\0\0\0\0\0\0\0\0\0\x17\x90U_\x81\x81R`\x02` R`@\x90 a\n!\x903\x90a\"\tV[Pa\nU`@Q\x80\x933\x84\x7F\xC9\x86,_\x02\xEE\xFB\xDC\xEA\x01\xC2\x07\xAES\x8E\x1D0M\xC90&\x87\x0FH\x95\x1EH\xA0\xF4\xC8G\x0C_\x80\xA3a\x19\nV[`\x01` \x83\x01R_\x80Q` a#z\x839\x81Q\x91R`@3\x93\xA3\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FCannot go online while slashed\0\0`D\x82\x01R`d\x90\xFD[4a\x01\xCCW`\xA06`\x03\x19\x01\x12a\x01\xCCWa\n\xCFa\x17AV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xCCWa\n\xEE\x906\x90`\x04\x01a\x19iV[\x90\x91`\x845\x80\x15\x15\x92\x91\x90\x83\x90\x03a\x01\xCCW`\x01`\x01`@\x1B\x03\x16\x80_R`\x03` Ra\x0B(`\x01\x80`\xA0\x1B\x03`@_ T\x163\x14a\x1A\x84V[_R`\x04` Ra\x0BK`@_ \x93`@Q\x92a\x0BD\x84a\x17\xEFV[6\x91a\x19\x96V[\x81R` \x81\x01\x92`D5\x84R`@\x82\x01\x90`d5\x82R``\x83\x01\x93\x84R\x80T`\x01`@\x1B\x81\x10\x15a\x0C\xB8Wa\x0B\x85\x91`\x01\x82\x01\x81Ua\x17\x83V[\x92\x90\x92a\x0C\xCCWQ\x93\x84Q\x94`\x01`\x01`@\x1B\x03\x86\x11a\x0C\xB8Wa\x0B\xA9\x84Ta\x17\x9CV[`\x1F\x81\x11a\x0CsW[P` \x90`\x1F\x87\x11`\x01\x14a\x0C\x0EW\x95\x80\x91`\x03\x96\x97_\x92a\x0C\x03W[PP\x81`\x01\x1B\x91_\x19\x90\x87\x1B\x1C\x19\x16\x17\x83U[Q`\x01\x83\x01UQ`\x02\x82\x01U\x01\x90Q\x15\x15`\xFF\x80\x19\x83T\x16\x91\x16\x17\x90U_\x80\xF3[\x01Q\x90P\x87\x80a\x0B\xCFV[\x90`\x1F\x19\x87\x16\x91\x85_R\x81_ \x92_[\x81\x81\x10a\x0C[WP\x91`\x01\x93\x91\x89`\x03\x99\x9A\x94\x10a\x0CDW[PPP\x81\x1B\x01\x83Ua\x0B\xE2V[\x01Q_\x19\x83\x89\x1B`\xF8\x16\x1C\x19\x16\x90U\x87\x80\x80a\x0C7V[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x0C\x1EV[\x84_R` _ `\x1F\x88\x01`\x05\x1C\x81\x01\x91` \x89\x10a\x0C\xAEW[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x0C\xA3WPa\x0B\xB2V[_\x81U`\x01\x01a\x0C\x96V[\x90\x91P\x81\x90a\x0C\x8DV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[4a\x01\xCCWa\x0C\xED6a\x19\xF9V[`\x06T\x92\x93\x91\x92`\x01`\x01`\xA0\x1B\x03\x163\x03a\r\xB0W`@`\x01`\x01`@\x1B\x03\x7F\x1E)\t\xCFE\xD7\x0C\xF0\x03\xF34\xB7<\x933\x0C\xE7\xE5rx-\xFC\x82\xFA\xB7\x9D\xEB\x88U\xA7\xC7\x91\x93\x16\x93\x84_R`\x01` R\x81_ `\x01\x80`\xA0\x1B\x03\x87\x16_R` R`\x01\x82_ \x01i\x03\0\0\0\0\0\0\0\0\0`\xFF`H\x1B\x19\x82T\x16\x17\x90U\x84_R`\x02` Ra\r\x85\x82_ \x96`\x01\x80`\xA0\x1B\x03\x16\x80\x97a!.V[P\x82\x82Q\x93\x84\x92` \x84R\x81` \x85\x01R\x84\x84\x017_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x81\x01\x03\x01\x90\xA3\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01RrNot slashing oracle`h\x1B`D\x82\x01R`d\x90\xFD[4a\x01\xCCW``6`\x03\x19\x01\x12a\x01\xCCWa\x0E\x04a\x17AV[a\x0E\x0Ca\x17mV[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xCCW` \x92\x83\x92`\x01`\x01`@\x1B\x03a\x0E9\x85\x946\x90`\x04\x01a\x19\xDBV[\x92\x16_R`\x05\x83R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R\x82R`@_ `@Q\x93\x82\x85\x93Q\x91\x82\x91\x01\x84^\x82\x01\x90\x81R\x03\x01\x90 T`@Q\x90\x81R\xF3[4a\x01\xCCW`@6`\x03\x19\x01\x12a\x01\xCCWa\x0E\x8Fa\x17AV[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x01\xCCW6`#\x83\x01\x12\x15a\x01\xCCW\x81`\x04\x015\x91`\x01`\x01`@\x1B\x03\x83\x11a\x01\xCCW6`$\x84`\x05\x1B\x83\x01\x01\x11a\x01\xCCW\x82\x90`\x01`\x01`@\x1B\x03_\x93\x16\x92[\x82\x81\x10\x15a\x02GW`$`\x05\x82\x90\x1B\x83\x01\x015`\x01`\x01`\xA0\x1B\x03\x81\x16\x91\x90\x82\x90\x03a\x01\xCCW0;\x15a\x01\xCCW`@Q\x91c\xBA\x1F\xB1\x03`\xE0\x1B\x83R\x85`\x04\x84\x01R`$\x83\x01R_\x82`D\x81\x830Z\xF1\x91\x82\x15a\x0FSW`\x01\x92a\x0FCW[P\x01a\x0E\xDEV[_a\x0FM\x91a\x18%V[\x85a\x0F<V[`@Q=_\x82>=\x90\xFD[4a\x01\xCCW` 6`\x03\x19\x01\x12a\x01\xCCW`\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\x01\xCCWk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B`\x06T\x16\x17`\x06U_\x80\xF3[4a\x01\xCCW` 6`\x03\x19\x01\x12a\x01\xCCW`\x01`\x01`@\x1B\x03a\x0F\xC1a\x17AV[\x16_R`\x02` R` `@_ T`@Q\x90\x81R\xF3[4a\x01\xCCW`@6`\x03\x19\x01\x12a\x01\xCCWa\x0F\xF1a\x17AV[`\x01`\x01`@\x1B\x03a\x10\x01a\x17mV[\x91_`\x80`@Qa\x10\x11\x81a\x17\xD4V[\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x01R\x16_R`\x01` R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ `@Qa\x10R\x81a\x17\xD4V[\x81T\x81R`\x01\x82\x01T\x91` \x82\x01\x90`\x01`\x01`@\x1B\x03\x84\x16\x82R`\xFF`@\x84\x01\x94\x81\x81`@\x1C\x16\x86R`H\x1C\x16``\x84\x01\x90`\x05\x81\x10\x15a\x05\xC0W`\xA0\x95`\x01`\x01`@\x1B\x03`\x02a\x10\xCE\x95`\xFF\x94\x86R\x01T\x95`\x80\x88\x01\x96\x87R`@Q\x97Q\x88RQ\x16` \x87\x01RQ\x16`@\x85\x01RQ``\x84\x01\x90a\x19\nV[Q`\x80\x82\x01R\xF3[4a\x01\xCCW`@6`\x03\x19\x01\x12a\x01\xCCWa\x10\xEFa\x17AV[`\x01`\x01`@\x1B\x03a\x10\xFFa\x17mV[\x91\x16_R`\x01` R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R` `\xFF`\x01`@_ \x01T`H\x1C\x16a\x117`@Q\x80\x92a\x19\nV[\xF3[4a\x01\xCCW_6`\x03\x19\x01\x12a\x01\xCCW` `@Q`\x03\x81R\xF3[4a\x01\xCCW`\x806`\x03\x19\x01\x12a\x01\xCCWa\x11ma\x17AV[a\x11ua\x17WV[\x90a\x11~a\x19YV[\x91`d5\x92`\x01`\x01`@\x1B\x03\x84\x11a\x01\xCCWa\x11\xA2a\x02G\x946\x90`\x04\x01a\x19iV[\x93\x90\x923\x91a\x1DyV[4a\x01\xCCW_6`\x03\x19\x01\x12a\x01\xCCW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x01\xCCW` 6`\x03\x19\x01\x12a\x01\xCCWa\x12\ta\x17AV[P`@Qa\x0C\xA0a\x12\x1A\x81\x83a\x18%V[`d\x82R`\x1F\x19\x01\x906\x90` \x017a\x12M`@Qa\x12:` \x82a\x18%V[_\x81R_6\x817`@Q\x91\x82\x91\x82a\x19\x17V[\x03\x90\xF3[4a\x01\xCCW` 6`\x03\x19\x01\x12a\x01\xCCW`\x01`\x01`@\x1B\x03a\x12ra\x17AV[\x16_R`\x03` R` `\x01\x80`\xA0\x1B\x03`@_ T\x16`@Q\x90\x81R\xF3[4a\x01\xCCW`@6`\x03\x19\x01\x12a\x01\xCCWa\x12\xAAa\x17AV[`\x01`\x01`@\x1B\x03a\x12\xBAa\x17mV[\x91\x16\x80_R`\x01` R`@_ `\x01\x80`\xA0\x1B\x03\x83\x16_R` R`\xFF`\x01`@_ \x01T`H\x1C\x16`\x05\x81\x10\x15a\x05\xC0W\x15\x90\x81\x15a\x13\x03W[` \x82`@Q\x90\x15\x15\x81R\xF3[\x90P_R`\x01` R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`\xFF`\x01`@_ \x01T`H\x1C\x16`\x05\x81\x10\x15a\x05\xC0W`\x01` \x91\x14\x82a\x12\xF6V[4a\x01\xCCW` 6`\x03\x19\x01\x12a\x01\xCCW`\x01`\x01`@\x1B\x03a\x13ba\x17AV[\x16\x80_R`\x02` R`@_ Ta\x13y\x81a\x1AmV[\x91a\x13\x87`@Q\x93\x84a\x18%V[\x81\x83R`\x1F\x19a\x13\x96\x83a\x1AmV[\x016` \x85\x017_[\x82\x81\x10a\x13\xB4W`@Q\x80a\x12M\x86\x82a\x19\x17V[\x81_R`\x02` Ra\x13\xC9\x81`@_ a \x0CV[\x90T\x85Q`\x03\x92\x90\x92\x1B\x1C`\x01`\x01`\xA0\x1B\x03\x16\x91\x90\x81\x10\x15a\x13\xF9W`\x01\x91` \x82`\x05\x1B\x87\x01\x01R\x01a\x13\x9FV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[4a\x01\xCCW`@6`\x03\x19\x01\x12a\x01\xCCWa\x14&a\x17AV[`\x01`\x01`@\x1B\x03a\x146a\x17mV[\x91\x16_R`\x01` R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`\xA0`@_ \x80T\x90a\x14\x99`\x02`\x01\x83\x01T\x92\x01T\x91`@Q\x93\x84R`\x01`\x01`@\x1B\x03\x81\x16` \x85\x01R`\xFF\x81`@\x1C\x16`@\x85\x01R`\xFF``\x85\x01\x91`H\x1C\x16a\x19\nV[`\x80\x82\x01R\xF3[4a\x01\xCCW_6`\x03\x19\x01\x12a\x01\xCCW` `@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xF3[4a\x01\xCCW_6`\x03\x19\x01\x12a\x01\xCCW` `@Qa\x01,\x81R\xF3[4a\x01\xCCW`@6`\x03\x19\x01\x12a\x01\xCCWa\x15\x0Fa\x17AV[`\x01`\x01`@\x1B\x03`$5\x91\x16_R`\x04` R`@_ \x80T\x82\x10\x15a\x01\xCCWa\x15p\x91a\x15=\x91a\x17\x83V[Pa\x15G\x81a\x18FV[\x90`\x01\x81\x01T\x90`\xFF`\x03`\x02\x83\x01T\x92\x01T\x16\x90`@Q\x94\x85\x94`\x80\x86R`\x80\x86\x01\x90a\x18\xE6V[\x92` \x85\x01R`@\x84\x01R\x15\x15``\x83\x01R\x03\x90\xF3[4a\x01\xCCW`@6`\x03\x19\x01\x12a\x01\xCCWa\x15\x9Fa\x17AV[`\x01`\x01`@\x1B\x03a\x15\xAFa\x17mV[\x91\x16_R`\x01` R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R` `@_ T`@Q\x90\x81R\xF3[4a\x01\xCCW` 6`\x03\x19\x01\x12a\x01\xCCW``a\x16\x05a\x15\xF7a\x17AV[a\x15\xFFa\x1AOV[Pa\x1D\x04V[`@\x80Q\x91`\x01`\x01`@\x1B\x03\x81Q\x16\x83R`\xFF` \x82\x01Q\x16` \x84\x01R\x01Q\x15\x15`@\x82\x01R\xF3[4a\x01\xCCW`@6`\x03\x19\x01\x12a\x01\xCCWa\x16Ha\x17AV[a\x16Pa\x17mV[\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a\x17\x0CWP`\x01`\x01`@\x1B\x03\x16_\x81\x81R`\x03` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16a\x16\xD2W_\x90\x81R`\x03` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rq\x10[\x1C\x99XY\x1EH\x1C\x99Y\xDA\\\xDD\x19\\\x99Y`r\x1B`D\x82\x01R`d\x90\xFD[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RoOnly Tangle core`\x80\x1B`D\x82\x01R`d\x90\xFD[`\x045\x90`\x01`\x01`@\x1B\x03\x82\x16\x82\x03a\x01\xCCWV[`$5\x90`\x01`\x01`@\x1B\x03\x82\x16\x82\x03a\x01\xCCWV[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01\xCCWV[\x80T\x82\x10\x15a\x13\xF9W_R` _ \x90`\x02\x1B\x01\x90_\x90V[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x17\xCAW[` \x83\x10\x14a\x17\xB6WV[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a\x17\xABV[`\xA0\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x0C\xB8W`@RV[`\x80\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x0C\xB8W`@RV[``\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x0C\xB8W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x0C\xB8W`@RV[\x90`@Q\x91\x82_\x82T\x92a\x18Y\x84a\x17\x9CV[\x80\x84R\x93`\x01\x81\x16\x90\x81\x15a\x18\xC4WP`\x01\x14a\x18\x80W[Pa\x18~\x92P\x03\x83a\x18%V[V[\x90P_\x92\x91\x92R` _ \x90_\x91[\x81\x83\x10a\x18\xA8WPP\x90` a\x18~\x92\x82\x01\x01_a\x18qV[` \x91\x93P\x80`\x01\x91T\x83\x85\x89\x01\x01R\x01\x91\x01\x90\x91\x84\x92a\x18\x8FV[\x90P` \x92Pa\x18~\x94\x91P`\xFF\x19\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01_a\x18qV[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90`\x05\x82\x10\x15a\x05\xC0WRV[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x19:WPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x19-V[`D5\x90`\xFF\x82\x16\x82\x03a\x01\xCCWV[\x91\x81`\x1F\x84\x01\x12\x15a\x01\xCCW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x01\xCCW` \x83\x81\x86\x01\x95\x01\x01\x11a\x01\xCCWV[\x92\x91\x92`\x01`\x01`@\x1B\x03\x82\x11a\x0C\xB8W`@Q\x91a\x19\xBF`\x1F\x82\x01`\x1F\x19\x16` \x01\x84a\x18%V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x01\xCCW\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x01\xCCW\x81` a\x19\xF6\x935\x91\x01a\x19\x96V[\x90V[```\x03\x19\x82\x01\x12a\x01\xCCW`\x045`\x01`\x01`@\x1B\x03\x81\x16\x81\x03a\x01\xCCW\x91`$5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x01\xCCW\x91`D5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x01\xCCWa\x1AK\x91`\x04\x01a\x19iV[\x90\x91V[`@Q\x90a\x1A\\\x82a\x18\nV[_`@\x83\x82\x81R\x82` \x82\x01R\x01RV[`\x01`\x01`@\x1B\x03\x81\x11a\x0C\xB8W`\x05\x1B` \x01\x90V[\x15a\x1A\x8BWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp'7\xBA\x109\xB2\xB9;4\xB1\xB2\x907\xBB\xB72\xB9`y\x1B`D\x82\x01R`d\x90\xFD[\x91\x90\x82\x03\x91\x82\x11a\x1A\xD1WV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x90`\x01`\x01`@\x1B\x03\x82\x16\x90\x81_R`\x01` R`@_ `\x01\x80`\xA0\x1B\x03\x82\x16_R` Ra\x1B\x18`@_ \x93a\x1D\x04V[\x92\x80T\x80\x15a\x1BWWa\x1B+\x90Ba\x1A\xC4V[`\x01`\x01`@\x1B\x03\x85Q\x16\x90\x81\x15a\x1CJW`\x01\x91`\xFF\x91\x04\x16\x91\x01\x93`\xFF\x85T`@\x1C\x16\x82\x11a\x1B^W[PPPPPV[\x84Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\xFF`@\x1B`@\x84\x90\x1B\x16\x17\x85U` \x01Q`\xFF\x16\x81\x10\x15\x80a\x1C/W[a\x1B\x95W[\x80a\x1BWV[\x83_\x80Q` a#z\x839\x81Q\x91R\x92\x84\x7FD\xFD2\xB6wpL\xE6\x8Ewc\x89|Is;\x8FR\x89\x01\x8A\xC6\n\\\x92h\x02\xD67Y\xDBM` `@\x95`\xFFa\x1C\x0B\x9AT`H\x1C\x16\x95i\x02\0\0\0\0\0\0\0\0\0`\xFF`H\x1B\x19\x82T\x16\x17\x90U\x83_R`\x02\x82R\x86_ \x94`\x01\x80`\xA0\x1B\x03\x16\x99\x8A\x80\x96a!.V[P\x86Q\x90\x81R\xA3a\x1C\x1E\x82Q\x80\x92a\x19\nV[`\x02` \x82\x01R\xA3_\x80\x80\x80a\x1B\x8FV[P`\xFF\x84T`H\x1C\x16`\x05\x81\x10\x15a\x05\xC0W`\x02\x14\x15a\x1B\x8AV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[\x90`\x01`\x01`@\x1B\x03a\x1Cp\x83a\x1D\x04V[\x92\x16_R`\x01` R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ `@Q\x90a\x1C\x9C\x82a\x17\xD4V[\x80T\x82R`\xFF`\x01\x82\x01T`\x01`\x01`@\x1B\x03\x81\x16` \x85\x01R\x81\x81`@\x1C\x16`@\x85\x01R`H\x1C\x16\x90`\x05\x82\x10\x15a\x05\xC0W`\x02\x91``\x84\x01R\x01T`\x80\x82\x01RQ\x80\x15a\x1C\xFEWa\x1C\xF7`\x01`\x01`@\x1B\x03\x91Ba\x1A\xC4V[\x91Q\x16\x11\x90V[PP_\x90V[`\x01`\x01`@\x1B\x03\x90a\x1D\x15a\x1AOV[P\x16_R_` R`@_ `@Q\x90a\x1D.\x82a\x18\nV[T`\x01`\x01`@\x1B\x03\x81\x16\x80\x83R`\xFF\x82`@\x1C\x16\x90`\xFF` \x85\x01\x93\x83\x85R`H\x1C\x16\x15\x15`@\x85\x01R\x15a\x1DoW[\x15a\x1DhWP\x90V[`\x03\x90R\x90V[a\x01,\x83Ra\x1D_V[\x93\x94\x91\x92\x90`\x01`\x01`@\x1B\x03\x85\x16\x94\x85_R`\x01` R`@_ `\x01\x80`\xA0\x1B\x03\x86\x16_R` R`@_ \x94a\x1D\xB1\x82a\x1D\x04V[\x90`\x01\x87\x01\x94`\xFF\x86T`H\x1C\x16\x97B\x81U`\x02a\x1D\xD1\x8C\x8A6\x91a\x19\x96V[` \x81Q\x91\x01 \x91\x01U`\xFF`@\x1B\x19\x86T\x16\x86U`\x01`\x01`@\x1B\x03\x86T\x16\x90`\x01`\x01`@\x1B\x03\x82\x14a\x1A\xD1W`\x01`\x01`@\x1B\x03`\x01`\xFF\x93\x01\x16`\x01`\x01`@\x1B\x03\x19\x88T\x16\x17\x87U\x16\x92\x83\x15_\x14a\x1F\x88W_\x96[`\x05\x88\x10\x15\x96\x87a\x05\xC0W\x80T`\xFF`H\x1B\x19\x16`H\x8A\x90\x1B`\xFF`H\x1B\x16\x17\x90U`\x05\x89\x10\x15a\x05\xC0W`\x01`\x01`@\x1B\x03\x9A\x8A\x94`\x02\x8B\x14\x8A\x81a\x1FyW[P\x91\x85`@\x7Fe\x89\x18\xE3\x14\x7F\x13\xDD\x06\x8E\xC2\x147\xB4\xC2\\!h*\x8D\xC2\x12\x93Hg\x1E\xAD\0\r\xB3\xE7\xB9\x97\x95\x93\x81\x97\x95a\x1F,W[\x01Q\x15\x15\x80a\x1F#W[a\x1F\x11W[PP\x83Q\x96\x87RPPB` \x86\x01R`\x01`\x01`\xA0\x1B\x03\x16\x99\x8A\x95\x16\x93\xA4a\x05\xC0W\x80\x82\x03a\x1E\xDCW[PPPPV[_\x80Q` a#z\x839\x81Q\x91R\x91a\x1F\x07`@\x92a\x1E\xFD\x84Q\x80\x94a\x19\nV[` \x83\x01\x90a\x19\nV[\xA3_\x80\x80\x80a\x1E\xD6V[a\x1F\x1A\x93a !V[_\x80\x80\x83a\x1E\xACV[P\x82\x15\x15a\x1E\xA7V[_\x8A\x81R`\x02` R\x82\x90 `\x01`\x01`\xA0\x1B\x03\x84\x16\x90a\x1FN\x90\x82\x90a\"\tV[P\x8A\x7F\xC9\x86,_\x02\xEE\xFB\xDC\xEA\x01\xC2\x07\xAES\x8E\x1D0M\xC90&\x87\x0FH\x95\x1EH\xA0\xF4\xC8G\x0C_\x80\xA3a\x1E\x9DV[_\x9AP`\x02\x14\x15\x90P\x8Aa\x1ElV[`d\x84\x10\x15a\x1F\x99W`\x01\x96a\x1E+V[`\x01\x96`\xC8\x85\x10a\x1E+W`@Q` \x81R`\x1B` \x82\x01R\x7FProtocol violation reported\0\0\0\0\0`@\x82\x01R\x8A\x7F\x1E)\t\xCFE\xD7\x0C\xF0\x03\xF34\xB7<\x933\x0C\xE7\xE5rx-\xFC\x82\xFA\xB7\x9D\xEB\x88U\xA7\xC7\x91```\x01\x80`\xA0\x1B\x03\x87\x16\x93\xA3a\x1E+V[\x80T\x82\x10\x15a\x13\xF9W_R` _ \x01\x90_\x90V[`@\x84\x10a\x1E\xD6W_\x92[`@\x84\x01\x80\x85\x11a\x1A\xD1W\x85\x10a\x1BWW\x84\x84\x11a\x01\xCCW\x83\x81\x01\x84\x86\x03\x81\x01`@\x82\x82\x03\x12a\x01\xCCW\x815`\x01`\x01`@\x1B\x03\x81\x11a\x01\xCCW` \x91a t\x91\x84\x01a\x19\xDBV[\x91\x015`\x01`\x01`@\x1B\x03\x84\x16\x80_R`\x05` R`@_ `\x01\x80`\xA0\x1B\x03\x87\x16_R` R\x81`@_ ` `@Q\x80\x92\x87Q\x80\x84\x8A\x01\x84^\x82\x01\x90\x81R\x03\x01\x90 U\x7F#\xED\x02\xBD6\x05\xBD\xEAj\x8A\xFAv\xC4o\0\xD2t\x86\x0B\xA6\xCE\xA9\x80\xF2X[im\xF9\xE1\x82\xBD`@Q`@\x81Ra \xEE`@\x82\x01\x86a\x18\xE6V[` \x82\x01\x94\x90\x94R`\x01`\x01`\xA0\x1B\x03\x88\x16\x93\x81\x90\x03\x90\xA3Q\x80`@\x01`@\x11a\x1A\xD1W`@\x90\x85\x01\x01\x80\x94\x11a\x1A\xD1W\x84\x84\x11\x15a ,WPPPPPV[\x90`\x01\x82\x01\x91\x81_R\x82` R`@_ T\x80\x15\x15_\x14a\"\x01W_\x19\x81\x01\x81\x81\x11a\x1A\xD1W\x82T_\x19\x81\x01\x91\x90\x82\x11a\x1A\xD1W\x81\x81\x03a!\xB6W[PPP\x80T\x80\x15a!\xA2W_\x19\x01\x90a!\x83\x82\x82a \x0CV[\x81T\x90_\x19\x90`\x03\x1B\x1B\x19\x16\x90UU_R` R_`@\x81 U`\x01\x90V[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[a!\xECa!\xC6a!\xD6\x93\x86a \x0CV[\x90T\x90`\x03\x1B\x1C\x92\x83\x92\x86a \x0CV[\x81\x93\x91T\x90`\x03\x1B\x91\x82\x1B\x91_\x19\x90\x1B\x19\x16\x17\x90V[\x90U_R\x83` R`@_ U_\x80\x80a!jV[PPPP_\x90V[`\x01\x81\x01\x90\x82_R\x81` R`@_ T\x15_\x14a\"VW\x80T`\x01`@\x1B\x81\x10\x15a\x0C\xB8Wa\"Ca!\xD6\x82`\x01\x87\x94\x01\x85U\x84a \x0CV[\x90UT\x91_R` R`@_ U`\x01\x90V[PPP_\x90V[\x81Q\x91\x90`A\x83\x03a\"\x8DWa\"\x86\x92P` \x82\x01Q\x90```@\x84\x01Q\x93\x01Q_\x1A\x90a\"\xF7V[\x91\x92\x90\x91\x90V[PP_\x91`\x02\x91\x90V[`\x04\x81\x10\x15a\x05\xC0W\x80a\"\xA9WPPV[`\x01\x81\x03a\"\xC0Wc\xF6E\xEE\xDF`\xE0\x1B_R`\x04_\xFD[`\x02\x81\x03a\"\xDBWPc\xFC\xE6\x98\xF7`\xE0\x1B_R`\x04R`$_\xFD[`\x03\x14a\"\xE5WPV[c5\xE2\xF3\x83`\xE2\x1B_R`\x04R`$_\xFD[\x91\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11a#nW\x91` \x93`\x80\x92`\xFF_\x95`@Q\x94\x85R\x16\x86\x84\x01R`@\x83\x01R``\x82\x01R\x82\x80R`\x01Z\xFA\x15a\x0FSW_Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a#dW\x90_\x90_\x90V[P_\x90`\x01\x90_\x90V[PPP_\x91`\x03\x91\x90V\xFE\"\x88$\xB8l%di\x12_R\\\xE1\x8Cl-\n\x9E\x13=\x13\xB8\xECz,\x96\xA1\x93\xB0\xC2\x8A\t\xA1dsolcC\0\x08\x1A\0\n",
    );
    /**```solidity
struct HeartbeatConfig { uint64 interval; uint8 maxMissed; bool customMetrics; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct HeartbeatConfig {
        #[allow(missing_docs)]
        pub interval: u64,
        #[allow(missing_docs)]
        pub maxMissed: u8,
        #[allow(missing_docs)]
        pub customMetrics: bool,
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
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<8>,
            alloy::sol_types::sol_data::Bool,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u64, u8, bool);
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
        impl ::core::convert::From<HeartbeatConfig> for UnderlyingRustTuple<'_> {
            fn from(value: HeartbeatConfig) -> Self {
                (value.interval, value.maxMissed, value.customMetrics)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for HeartbeatConfig {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    interval: tuple.0,
                    maxMissed: tuple.1,
                    customMetrics: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for HeartbeatConfig {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for HeartbeatConfig {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.interval),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxMissed),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.customMetrics,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for HeartbeatConfig {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for HeartbeatConfig {
            const NAME: &'static str = "HeartbeatConfig";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "HeartbeatConfig(uint64 interval,uint8 maxMissed,bool customMetrics)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.interval)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.maxMissed)
                        .0,
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(
                            &self.customMetrics,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for HeartbeatConfig {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.interval,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.maxMissed,
                    )
                    + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.customMetrics,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.interval,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.maxMissed,
                    out,
                );
                <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.customMetrics,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    /**```solidity
struct MetricDefinition { string name; uint256 minValue; uint256 maxValue; bool required; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MetricDefinition {
        #[allow(missing_docs)]
        pub name: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub minValue: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub maxValue: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub required: bool,
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
            alloy::sol_types::sol_data::String,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Bool,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::String,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            bool,
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
        impl ::core::convert::From<MetricDefinition> for UnderlyingRustTuple<'_> {
            fn from(value: MetricDefinition) -> Self {
                (value.name, value.minValue, value.maxValue, value.required)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MetricDefinition {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    name: tuple.0,
                    minValue: tuple.1,
                    maxValue: tuple.2,
                    required: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for MetricDefinition {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for MetricDefinition {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.name,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.minValue),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxValue),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.required,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for MetricDefinition {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for MetricDefinition {
            const NAME: &'static str = "MetricDefinition";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "MetricDefinition(string name,uint256 minValue,uint256 maxValue,bool required)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.name,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.minValue)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.maxValue)
                        .0,
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(
                            &self.required,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for MetricDefinition {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.name,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.minValue,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.maxValue,
                    )
                    + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.required,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.name,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.minValue,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.maxValue,
                    out,
                );
                <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.required,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    /**```solidity
struct OperatorState { uint256 lastHeartbeat; uint64 consecutiveBeats; uint8 missedBeats; IOperatorStatusRegistry.StatusCode status; bytes32 lastMetricsHash; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorState {
        #[allow(missing_docs)]
        pub lastHeartbeat: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub consecutiveBeats: u64,
        #[allow(missing_docs)]
        pub missedBeats: u8,
        #[allow(missing_docs)]
        pub status: <IOperatorStatusRegistry::StatusCode as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub lastMetricsHash: alloy::sol_types::private::FixedBytes<32>,
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
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<8>,
            IOperatorStatusRegistry::StatusCode,
            alloy::sol_types::sol_data::FixedBytes<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            u64,
            u8,
            <IOperatorStatusRegistry::StatusCode as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<OperatorState> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorState) -> Self {
                (
                    value.lastHeartbeat,
                    value.consecutiveBeats,
                    value.missedBeats,
                    value.status,
                    value.lastMetricsHash,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorState {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    lastHeartbeat: tuple.0,
                    consecutiveBeats: tuple.1,
                    missedBeats: tuple.2,
                    status: tuple.3,
                    lastMetricsHash: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OperatorState {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OperatorState {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.lastHeartbeat),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.consecutiveBeats),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.missedBeats),
                    <IOperatorStatusRegistry::StatusCode as alloy_sol_types::SolType>::tokenize(
                        &self.status,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.lastMetricsHash),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for OperatorState {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for OperatorState {
            const NAME: &'static str = "OperatorState";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OperatorState(uint256 lastHeartbeat,uint64 consecutiveBeats,uint8 missedBeats,uint8 status,bytes32 lastMetricsHash)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.lastHeartbeat)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.consecutiveBeats,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.missedBeats)
                        .0,
                    <IOperatorStatusRegistry::StatusCode as alloy_sol_types::SolType>::eip712_data_word(
                            &self.status,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.lastMetricsHash,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OperatorState {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.lastHeartbeat,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.consecutiveBeats,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.missedBeats,
                    )
                    + <IOperatorStatusRegistry::StatusCode as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.status,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.lastMetricsHash,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.lastHeartbeat,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.consecutiveBeats,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.missedBeats,
                    out,
                );
                <IOperatorStatusRegistry::StatusCode as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.status,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.lastMetricsHash,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    /**Custom error with signature `ECDSAInvalidSignature()` and selector `0xf645eedf`.
```solidity
error ECDSAInvalidSignature();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECDSAInvalidSignature {}
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
        impl ::core::convert::From<ECDSAInvalidSignature> for UnderlyingRustTuple<'_> {
            fn from(value: ECDSAInvalidSignature) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECDSAInvalidSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECDSAInvalidSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECDSAInvalidSignature()";
            const SELECTOR: [u8; 4] = [246u8, 69u8, 238u8, 223u8];
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
    /**Custom error with signature `ECDSAInvalidSignatureLength(uint256)` and selector `0xfce698f7`.
```solidity
error ECDSAInvalidSignatureLength(uint256 length);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECDSAInvalidSignatureLength {
        #[allow(missing_docs)]
        pub length: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<ECDSAInvalidSignatureLength>
        for UnderlyingRustTuple<'_> {
            fn from(value: ECDSAInvalidSignatureLength) -> Self {
                (value.length,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ECDSAInvalidSignatureLength {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { length: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECDSAInvalidSignatureLength {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECDSAInvalidSignatureLength(uint256)";
            const SELECTOR: [u8; 4] = [252u8, 230u8, 152u8, 247u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.length),
                )
            }
        }
    };
    /**Custom error with signature `ECDSAInvalidSignatureS(bytes32)` and selector `0xd78bce0c`.
```solidity
error ECDSAInvalidSignatureS(bytes32 s);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECDSAInvalidSignatureS {
        #[allow(missing_docs)]
        pub s: alloy::sol_types::private::FixedBytes<32>,
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
        impl ::core::convert::From<ECDSAInvalidSignatureS> for UnderlyingRustTuple<'_> {
            fn from(value: ECDSAInvalidSignatureS) -> Self {
                (value.s,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECDSAInvalidSignatureS {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { s: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECDSAInvalidSignatureS {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECDSAInvalidSignatureS(bytes32)";
            const SELECTOR: [u8; 4] = [215u8, 139u8, 206u8, 12u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.s),
                )
            }
        }
    };
    /**Event with signature `HeartbeatConfigUpdated(uint64,uint64,uint8)` and selector `0xc9599ed962624a858ec59bae0ed86c75f4db65fe04570021277edbedd04ea564`.
```solidity
event HeartbeatConfigUpdated(uint64 indexed serviceId, uint64 interval, uint8 maxMissed);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct HeartbeatConfigUpdated {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub interval: u64,
        #[allow(missing_docs)]
        pub maxMissed: u8,
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
        impl alloy_sol_types::SolEvent for HeartbeatConfigUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            const SIGNATURE: &'static str = "HeartbeatConfigUpdated(uint64,uint64,uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                201u8,
                89u8,
                158u8,
                217u8,
                98u8,
                98u8,
                74u8,
                133u8,
                142u8,
                197u8,
                155u8,
                174u8,
                14u8,
                216u8,
                108u8,
                117u8,
                244u8,
                219u8,
                101u8,
                254u8,
                4u8,
                87u8,
                0u8,
                33u8,
                39u8,
                126u8,
                219u8,
                237u8,
                208u8,
                78u8,
                165u8,
                100u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    serviceId: topics.1,
                    interval: data.0,
                    maxMissed: data.1,
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.interval),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxMissed),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.serviceId.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.serviceId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for HeartbeatConfigUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&HeartbeatConfigUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &HeartbeatConfigUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `HeartbeatReceived(uint64,uint64,address,uint8,uint256)` and selector `0x658918e3147f13dd068ec21437b4c25c21682a8dc2129348671ead000db3e7b9`.
```solidity
event HeartbeatReceived(uint64 indexed serviceId, uint64 indexed blueprintId, address indexed operator, uint8 statusCode, uint256 timestamp);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct HeartbeatReceived {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub blueprintId: u64,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub statusCode: u8,
        #[allow(missing_docs)]
        pub timestamp: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for HeartbeatReceived {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "HeartbeatReceived(uint64,uint64,address,uint8,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                101u8,
                137u8,
                24u8,
                227u8,
                20u8,
                127u8,
                19u8,
                221u8,
                6u8,
                142u8,
                194u8,
                20u8,
                55u8,
                180u8,
                194u8,
                92u8,
                33u8,
                104u8,
                42u8,
                141u8,
                194u8,
                18u8,
                147u8,
                72u8,
                103u8,
                30u8,
                173u8,
                0u8,
                13u8,
                179u8,
                231u8,
                185u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    serviceId: topics.1,
                    blueprintId: topics.2,
                    operator: topics.3,
                    statusCode: data.0,
                    timestamp: data.1,
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
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.statusCode),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.timestamp),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.serviceId.clone(),
                    self.blueprintId.clone(),
                    self.operator.clone(),
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
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.serviceId);
                out[2usize] = <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.blueprintId);
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for HeartbeatReceived {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&HeartbeatReceived> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &HeartbeatReceived) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `MetricReported(uint64,address,string,uint256)` and selector `0x23ed02bd3605bdea6a8afa76c46f00d274860ba6cea980f2585b696df9e182bd`.
```solidity
event MetricReported(uint64 indexed serviceId, address indexed operator, string metricName, uint256 value);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct MetricReported {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub metricName: alloy::sol_types::private::String,
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for MetricReported {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "MetricReported(uint64,address,string,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                35u8,
                237u8,
                2u8,
                189u8,
                54u8,
                5u8,
                189u8,
                234u8,
                106u8,
                138u8,
                250u8,
                118u8,
                196u8,
                111u8,
                0u8,
                210u8,
                116u8,
                134u8,
                11u8,
                166u8,
                206u8,
                169u8,
                128u8,
                242u8,
                88u8,
                91u8,
                105u8,
                109u8,
                249u8,
                225u8,
                130u8,
                189u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    serviceId: topics.1,
                    operator: topics.2,
                    metricName: data.0,
                    value: data.1,
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.metricName,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.serviceId.clone(),
                    self.operator.clone(),
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
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.serviceId);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for MetricReported {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&MetricReported> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &MetricReported) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorCameOnline(uint64,address)` and selector `0xc9862c5f02eefbdcea01c207ae538e1d304dc93026870f48951e48a0f4c8470c`.
```solidity
event OperatorCameOnline(uint64 indexed serviceId, address indexed operator);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorCameOnline {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for OperatorCameOnline {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorCameOnline(uint64,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                201u8,
                134u8,
                44u8,
                95u8,
                2u8,
                238u8,
                251u8,
                220u8,
                234u8,
                1u8,
                194u8,
                7u8,
                174u8,
                83u8,
                142u8,
                29u8,
                48u8,
                77u8,
                201u8,
                48u8,
                38u8,
                135u8,
                15u8,
                72u8,
                149u8,
                30u8,
                72u8,
                160u8,
                244u8,
                200u8,
                71u8,
                12u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    serviceId: topics.1,
                    operator: topics.2,
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
                    self.serviceId.clone(),
                    self.operator.clone(),
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
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.serviceId);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorCameOnline {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorCameOnline> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorCameOnline) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorWentOffline(uint64,address,uint8)` and selector `0x44fd32b677704ce68e7763897c49733b8f5289018ac60a5c926802d63759db4d`.
```solidity
event OperatorWentOffline(uint64 indexed serviceId, address indexed operator, uint8 missedBeats);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorWentOffline {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub missedBeats: u8,
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
        impl alloy_sol_types::SolEvent for OperatorWentOffline {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorWentOffline(uint64,address,uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                68u8,
                253u8,
                50u8,
                182u8,
                119u8,
                112u8,
                76u8,
                230u8,
                142u8,
                119u8,
                99u8,
                137u8,
                124u8,
                73u8,
                115u8,
                59u8,
                143u8,
                82u8,
                137u8,
                1u8,
                138u8,
                198u8,
                10u8,
                92u8,
                146u8,
                104u8,
                2u8,
                214u8,
                55u8,
                89u8,
                219u8,
                77u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    serviceId: topics.1,
                    operator: topics.2,
                    missedBeats: data.0,
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
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.missedBeats),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.serviceId.clone(),
                    self.operator.clone(),
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
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.serviceId);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorWentOffline {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorWentOffline> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorWentOffline) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `SlashingTriggered(uint64,address,string)` and selector `0x1e2909cf45d70cf003f334b73c93330ce7e572782dfc82fab79deb8855a7c791`.
```solidity
event SlashingTriggered(uint64 indexed serviceId, address indexed operator, string reason);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SlashingTriggered {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for SlashingTriggered {
            type DataTuple<'a> = (alloy::sol_types::sol_data::String,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "SlashingTriggered(uint64,address,string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                30u8,
                41u8,
                9u8,
                207u8,
                69u8,
                215u8,
                12u8,
                240u8,
                3u8,
                243u8,
                52u8,
                183u8,
                60u8,
                147u8,
                51u8,
                12u8,
                231u8,
                229u8,
                114u8,
                120u8,
                45u8,
                252u8,
                130u8,
                250u8,
                183u8,
                157u8,
                235u8,
                136u8,
                85u8,
                167u8,
                199u8,
                145u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    serviceId: topics.1,
                    operator: topics.2,
                    reason: data.0,
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.reason,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.serviceId.clone(),
                    self.operator.clone(),
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
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.serviceId);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for SlashingTriggered {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SlashingTriggered> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &SlashingTriggered) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `StatusChanged(uint64,address,uint8,uint8)` and selector `0x228824b86c256469125f525ce18c6c2d0a9e133d13b8ec7a2c96a193b0c28a09`.
```solidity
event StatusChanged(uint64 indexed serviceId, address indexed operator, IOperatorStatusRegistry.StatusCode oldStatus, IOperatorStatusRegistry.StatusCode newStatus);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct StatusChanged {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub oldStatus: <IOperatorStatusRegistry::StatusCode as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub newStatus: <IOperatorStatusRegistry::StatusCode as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for StatusChanged {
            type DataTuple<'a> = (
                IOperatorStatusRegistry::StatusCode,
                IOperatorStatusRegistry::StatusCode,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "StatusChanged(uint64,address,uint8,uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                34u8,
                136u8,
                36u8,
                184u8,
                108u8,
                37u8,
                100u8,
                105u8,
                18u8,
                95u8,
                82u8,
                92u8,
                225u8,
                140u8,
                108u8,
                45u8,
                10u8,
                158u8,
                19u8,
                61u8,
                19u8,
                184u8,
                236u8,
                122u8,
                44u8,
                150u8,
                161u8,
                147u8,
                176u8,
                194u8,
                138u8,
                9u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    serviceId: topics.1,
                    operator: topics.2,
                    oldStatus: data.0,
                    newStatus: data.1,
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
                    <IOperatorStatusRegistry::StatusCode as alloy_sol_types::SolType>::tokenize(
                        &self.oldStatus,
                    ),
                    <IOperatorStatusRegistry::StatusCode as alloy_sol_types::SolType>::tokenize(
                        &self.newStatus,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.serviceId.clone(),
                    self.operator.clone(),
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
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.serviceId);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for StatusChanged {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&StatusChanged> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &StatusChanged) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address _tangleCore);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _tangleCore: alloy::sol_types::private::Address,
    }
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value._tangleCore,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _tangleCore: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
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
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._tangleCore,
                    ),
                )
            }
        }
    };
    /**Function with signature `DEFAULT_HEARTBEAT_INTERVAL()` and selector `0x2c957688`.
```solidity
function DEFAULT_HEARTBEAT_INTERVAL() external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DEFAULT_HEARTBEAT_INTERVALCall {}
    ///Container type for the return parameters of the [`DEFAULT_HEARTBEAT_INTERVAL()`](DEFAULT_HEARTBEAT_INTERVALCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DEFAULT_HEARTBEAT_INTERVALReturn {
        #[allow(missing_docs)]
        pub _0: u64,
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
            impl ::core::convert::From<DEFAULT_HEARTBEAT_INTERVALCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: DEFAULT_HEARTBEAT_INTERVALCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DEFAULT_HEARTBEAT_INTERVALCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
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
            impl ::core::convert::From<DEFAULT_HEARTBEAT_INTERVALReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: DEFAULT_HEARTBEAT_INTERVALReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DEFAULT_HEARTBEAT_INTERVALReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for DEFAULT_HEARTBEAT_INTERVALCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = DEFAULT_HEARTBEAT_INTERVALReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DEFAULT_HEARTBEAT_INTERVAL()";
            const SELECTOR: [u8; 4] = [44u8, 149u8, 118u8, 136u8];
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
    /**Function with signature `DEFAULT_MAX_MISSED_HEARTBEATS()` and selector `0x61d6b86c`.
```solidity
function DEFAULT_MAX_MISSED_HEARTBEATS() external view returns (uint8);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DEFAULT_MAX_MISSED_HEARTBEATSCall {}
    ///Container type for the return parameters of the [`DEFAULT_MAX_MISSED_HEARTBEATS()`](DEFAULT_MAX_MISSED_HEARTBEATSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DEFAULT_MAX_MISSED_HEARTBEATSReturn {
        #[allow(missing_docs)]
        pub _0: u8,
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
            impl ::core::convert::From<DEFAULT_MAX_MISSED_HEARTBEATSCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: DEFAULT_MAX_MISSED_HEARTBEATSCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DEFAULT_MAX_MISSED_HEARTBEATSCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8,);
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
            impl ::core::convert::From<DEFAULT_MAX_MISSED_HEARTBEATSReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: DEFAULT_MAX_MISSED_HEARTBEATSReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DEFAULT_MAX_MISSED_HEARTBEATSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for DEFAULT_MAX_MISSED_HEARTBEATSCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = DEFAULT_MAX_MISSED_HEARTBEATSReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DEFAULT_MAX_MISSED_HEARTBEATS()";
            const SELECTOR: [u8; 4] = [97u8, 214u8, 184u8, 108u8];
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
    /**Function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`.
```solidity
function DOMAIN_SEPARATOR() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DOMAIN_SEPARATORCall {}
    ///Container type for the return parameters of the [`DOMAIN_SEPARATOR()`](DOMAIN_SEPARATORCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DOMAIN_SEPARATORReturn {
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
            impl ::core::convert::From<DOMAIN_SEPARATORCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: DOMAIN_SEPARATORCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DOMAIN_SEPARATORCall {
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
            impl ::core::convert::From<DOMAIN_SEPARATORReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: DOMAIN_SEPARATORReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DOMAIN_SEPARATORReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for DOMAIN_SEPARATORCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = DOMAIN_SEPARATORReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DOMAIN_SEPARATOR()";
            const SELECTOR: [u8; 4] = [54u8, 68u8, 229u8, 21u8];
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
    /**Function with signature `HEARTBEAT_TYPEHASH()` and selector `0xf9f16762`.
```solidity
function HEARTBEAT_TYPEHASH() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct HEARTBEAT_TYPEHASHCall {}
    ///Container type for the return parameters of the [`HEARTBEAT_TYPEHASH()`](HEARTBEAT_TYPEHASHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct HEARTBEAT_TYPEHASHReturn {
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
            impl ::core::convert::From<HEARTBEAT_TYPEHASHCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: HEARTBEAT_TYPEHASHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for HEARTBEAT_TYPEHASHCall {
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
            impl ::core::convert::From<HEARTBEAT_TYPEHASHReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: HEARTBEAT_TYPEHASHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for HEARTBEAT_TYPEHASHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for HEARTBEAT_TYPEHASHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = HEARTBEAT_TYPEHASHReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "HEARTBEAT_TYPEHASH()";
            const SELECTOR: [u8; 4] = [249u8, 241u8, 103u8, 98u8];
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
    /**Function with signature `addMetricDefinition(uint64,string,uint256,uint256,bool)` and selector `0xae470a85`.
```solidity
function addMetricDefinition(uint64 serviceId, string memory name, uint256 minValue, uint256 maxValue, bool required) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addMetricDefinitionCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub name: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub minValue: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub maxValue: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub required: bool,
    }
    ///Container type for the return parameters of the [`addMetricDefinition(uint64,string,uint256,uint256,bool)`](addMetricDefinitionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addMetricDefinitionReturn {}
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
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
                alloy::sol_types::private::String,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                bool,
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
            impl ::core::convert::From<addMetricDefinitionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: addMetricDefinitionCall) -> Self {
                    (
                        value.serviceId,
                        value.name,
                        value.minValue,
                        value.maxValue,
                        value.required,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addMetricDefinitionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        serviceId: tuple.0,
                        name: tuple.1,
                        minValue: tuple.2,
                        maxValue: tuple.3,
                        required: tuple.4,
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
            impl ::core::convert::From<addMetricDefinitionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: addMetricDefinitionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addMetricDefinitionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addMetricDefinitionCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bool,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addMetricDefinitionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addMetricDefinition(uint64,string,uint256,uint256,bool)";
            const SELECTOR: [u8; 4] = [174u8, 71u8, 10u8, 133u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.serviceId),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.name,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.minValue),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxValue),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.required,
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
    /**Function with signature `checkOperatorStatus(uint64,address)` and selector `0xba1fb103`.
```solidity
function checkOperatorStatus(uint64 serviceId, address operator) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkOperatorStatusCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`checkOperatorStatus(uint64,address)`](checkOperatorStatusCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkOperatorStatusReturn {}
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
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64, alloy::sol_types::private::Address);
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
            impl ::core::convert::From<checkOperatorStatusCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkOperatorStatusCall) -> Self {
                    (value.serviceId, value.operator)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkOperatorStatusCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        serviceId: tuple.0,
                        operator: tuple.1,
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
            impl ::core::convert::From<checkOperatorStatusReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkOperatorStatusReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkOperatorStatusReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for checkOperatorStatusCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = checkOperatorStatusReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "checkOperatorStatus(uint64,address)";
            const SELECTOR: [u8; 4] = [186u8, 31u8, 177u8, 3u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.serviceId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
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
    /**Function with signature `checkOperatorsStatus(uint64,address[])` and selector `0x96686c1e`.
```solidity
function checkOperatorsStatus(uint64 serviceId, address[] memory operators) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkOperatorsStatusCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub operators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    ///Container type for the return parameters of the [`checkOperatorsStatus(uint64,address[])`](checkOperatorsStatusCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkOperatorsStatusReturn {}
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
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<checkOperatorsStatusCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkOperatorsStatusCall) -> Self {
                    (value.serviceId, value.operators)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkOperatorsStatusCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        serviceId: tuple.0,
                        operators: tuple.1,
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
            impl ::core::convert::From<checkOperatorsStatusReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkOperatorsStatusReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkOperatorsStatusReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for checkOperatorsStatusCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = checkOperatorsStatusReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "checkOperatorsStatus(uint64,address[])";
            const SELECTOR: [u8; 4] = [150u8, 104u8, 108u8, 30u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.serviceId),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.operators),
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
    /**Function with signature `configureHeartbeat(uint64,uint64,uint8)` and selector `0xb99f6759`.
```solidity
function configureHeartbeat(uint64 serviceId, uint64 interval, uint8 maxMissed) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct configureHeartbeatCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub interval: u64,
        #[allow(missing_docs)]
        pub maxMissed: u8,
    }
    ///Container type for the return parameters of the [`configureHeartbeat(uint64,uint64,uint8)`](configureHeartbeatCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct configureHeartbeatReturn {}
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
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64, u64, u8);
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
            impl ::core::convert::From<configureHeartbeatCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: configureHeartbeatCall) -> Self {
                    (value.serviceId, value.interval, value.maxMissed)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for configureHeartbeatCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        serviceId: tuple.0,
                        interval: tuple.1,
                        maxMissed: tuple.2,
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
            impl ::core::convert::From<configureHeartbeatReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: configureHeartbeatReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for configureHeartbeatReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for configureHeartbeatCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = configureHeartbeatReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "configureHeartbeat(uint64,uint64,uint8)";
            const SELECTOR: [u8; 4] = [185u8, 159u8, 103u8, 89u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.serviceId),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.interval),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxMissed),
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
    /**Function with signature `enableCustomMetrics(uint64,bool)` and selector `0xf9107f3b`.
```solidity
function enableCustomMetrics(uint64 serviceId, bool enabled) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct enableCustomMetricsCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub enabled: bool,
    }
    ///Container type for the return parameters of the [`enableCustomMetrics(uint64,bool)`](enableCustomMetricsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct enableCustomMetricsReturn {}
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
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64, bool);
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
            impl ::core::convert::From<enableCustomMetricsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: enableCustomMetricsCall) -> Self {
                    (value.serviceId, value.enabled)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for enableCustomMetricsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        serviceId: tuple.0,
                        enabled: tuple.1,
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
            impl ::core::convert::From<enableCustomMetricsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: enableCustomMetricsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for enableCustomMetricsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for enableCustomMetricsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Bool,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = enableCustomMetricsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "enableCustomMetrics(uint64,bool)";
            const SELECTOR: [u8; 4] = [249u8, 16u8, 127u8, 59u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.serviceId),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.enabled,
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
    /**Function with signature `getHeartbeatConfig(uint64)` and selector `0x0758236f`.
```solidity
function getHeartbeatConfig(uint64 serviceId) external view returns (HeartbeatConfig memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getHeartbeatConfigCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
    }
    ///Container type for the return parameters of the [`getHeartbeatConfig(uint64)`](getHeartbeatConfigCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getHeartbeatConfigReturn {
        #[allow(missing_docs)]
        pub _0: <HeartbeatConfig as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
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
            impl ::core::convert::From<getHeartbeatConfigCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getHeartbeatConfigCall) -> Self {
                    (value.serviceId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getHeartbeatConfigCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { serviceId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (HeartbeatConfig,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <HeartbeatConfig as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getHeartbeatConfigReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getHeartbeatConfigReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getHeartbeatConfigReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getHeartbeatConfigCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getHeartbeatConfigReturn;
            type ReturnTuple<'a> = (HeartbeatConfig,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getHeartbeatConfig(uint64)";
            const SELECTOR: [u8; 4] = [7u8, 88u8, 35u8, 111u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.serviceId),
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
    /**Function with signature `getLastHeartbeat(uint64,address)` and selector `0x0c76697a`.
```solidity
function getLastHeartbeat(uint64 serviceId, address operator) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastHeartbeatCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getLastHeartbeat(uint64,address)`](getLastHeartbeatCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastHeartbeatReturn {
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
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64, alloy::sol_types::private::Address);
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
            impl ::core::convert::From<getLastHeartbeatCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLastHeartbeatCall) -> Self {
                    (value.serviceId, value.operator)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLastHeartbeatCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        serviceId: tuple.0,
                        operator: tuple.1,
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
            impl ::core::convert::From<getLastHeartbeatReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLastHeartbeatReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLastHeartbeatReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLastHeartbeatCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLastHeartbeatReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLastHeartbeat(uint64,address)";
            const SELECTOR: [u8; 4] = [12u8, 118u8, 105u8, 122u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.serviceId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
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
    /**Function with signature `getMetricDefinitions(uint64)` and selector `0xc1ef9ddf`.
```solidity
function getMetricDefinitions(uint64 serviceId) external view returns (MetricDefinition[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMetricDefinitionsCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
    }
    ///Container type for the return parameters of the [`getMetricDefinitions(uint64)`](getMetricDefinitionsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMetricDefinitionsReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
            <MetricDefinition as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
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
            impl ::core::convert::From<getMetricDefinitionsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getMetricDefinitionsCall) -> Self {
                    (value.serviceId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMetricDefinitionsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { serviceId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<MetricDefinition>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <MetricDefinition as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getMetricDefinitionsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getMetricDefinitionsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMetricDefinitionsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getMetricDefinitionsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getMetricDefinitionsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<MetricDefinition>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getMetricDefinitions(uint64)";
            const SELECTOR: [u8; 4] = [193u8, 239u8, 157u8, 223u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.serviceId),
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
    /**Function with signature `getMetricValue(uint64,address,string)` and selector `0xd551162c`.
```solidity
function getMetricValue(uint64 serviceId, address operator, string memory metricName) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMetricValueCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub metricName: alloy::sol_types::private::String,
    }
    ///Container type for the return parameters of the [`getMetricValue(uint64,address,string)`](getMetricValueCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMetricValueReturn {
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
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::String,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<getMetricValueCall> for UnderlyingRustTuple<'_> {
                fn from(value: getMetricValueCall) -> Self {
                    (value.serviceId, value.operator, value.metricName)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getMetricValueCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        serviceId: tuple.0,
                        operator: tuple.1,
                        metricName: tuple.2,
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
            impl ::core::convert::From<getMetricValueReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getMetricValueReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMetricValueReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getMetricValueCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::String,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getMetricValueReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getMetricValue(uint64,address,string)";
            const SELECTOR: [u8; 4] = [213u8, 81u8, 22u8, 44u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.serviceId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.metricName,
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
    /**Function with signature `getOnlineOperatorCount(uint64)` and selector `0x7b9f64b2`.
```solidity
function getOnlineOperatorCount(uint64 serviceId) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOnlineOperatorCountCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
    }
    ///Container type for the return parameters of the [`getOnlineOperatorCount(uint64)`](getOnlineOperatorCountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOnlineOperatorCountReturn {
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
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
            impl ::core::convert::From<getOnlineOperatorCountCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOnlineOperatorCountCall) -> Self {
                    (value.serviceId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOnlineOperatorCountCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { serviceId: tuple.0 }
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
            impl ::core::convert::From<getOnlineOperatorCountReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOnlineOperatorCountReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOnlineOperatorCountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOnlineOperatorCountCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOnlineOperatorCountReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOnlineOperatorCount(uint64)";
            const SELECTOR: [u8; 4] = [123u8, 159u8, 100u8, 178u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.serviceId),
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
    /**Function with signature `getOnlineOperators(uint64)` and selector `0x40235a9c`.
```solidity
function getOnlineOperators(uint64 serviceId) external view returns (address[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOnlineOperatorsCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
    }
    ///Container type for the return parameters of the [`getOnlineOperators(uint64)`](getOnlineOperatorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOnlineOperatorsReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
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
            impl ::core::convert::From<getOnlineOperatorsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOnlineOperatorsCall) -> Self {
                    (value.serviceId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOnlineOperatorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { serviceId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<getOnlineOperatorsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOnlineOperatorsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOnlineOperatorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOnlineOperatorsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOnlineOperatorsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOnlineOperators(uint64)";
            const SELECTOR: [u8; 4] = [64u8, 35u8, 90u8, 156u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.serviceId),
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
    /**Function with signature `getOperatorState(uint64,address)` and selector `0x71e7388c`.
```solidity
function getOperatorState(uint64 serviceId, address operator) external view returns (OperatorState memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorStateCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getOperatorState(uint64,address)`](getOperatorStateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorStateReturn {
        #[allow(missing_docs)]
        pub _0: <OperatorState as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64, alloy::sol_types::private::Address);
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
            impl ::core::convert::From<getOperatorStateCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorStateCall) -> Self {
                    (value.serviceId, value.operator)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorStateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        serviceId: tuple.0,
                        operator: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (OperatorState,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OperatorState as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getOperatorStateReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorStateReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorStateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorStateCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorStateReturn;
            type ReturnTuple<'a> = (OperatorState,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorState(uint64,address)";
            const SELECTOR: [u8; 4] = [113u8, 231u8, 56u8, 140u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.serviceId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
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
    /**Function with signature `getOperatorStatus(uint64,address)` and selector `0x62c7e8fc`.
```solidity
function getOperatorStatus(uint64 serviceId, address operator) external view returns (IOperatorStatusRegistry.StatusCode);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorStatusCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getOperatorStatus(uint64,address)`](getOperatorStatusCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorStatusReturn {
        #[allow(missing_docs)]
        pub _0: <IOperatorStatusRegistry::StatusCode as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64, alloy::sol_types::private::Address);
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
            impl ::core::convert::From<getOperatorStatusCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorStatusCall) -> Self {
                    (value.serviceId, value.operator)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorStatusCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        serviceId: tuple.0,
                        operator: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IOperatorStatusRegistry::StatusCode,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IOperatorStatusRegistry::StatusCode as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getOperatorStatusReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorStatusReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorStatusReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorStatusCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorStatusReturn;
            type ReturnTuple<'a> = (IOperatorStatusRegistry::StatusCode,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorStatus(uint64,address)";
            const SELECTOR: [u8; 4] = [98u8, 199u8, 232u8, 252u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.serviceId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
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
    /**Function with signature `getSlashableOperators(uint64)` and selector `0x59dcea12`.
```solidity
function getSlashableOperators(uint64 serviceId) external view returns (address[] memory operators);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSlashableOperatorsCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
    }
    ///Container type for the return parameters of the [`getSlashableOperators(uint64)`](getSlashableOperatorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSlashableOperatorsReturn {
        #[allow(missing_docs)]
        pub operators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
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
            impl ::core::convert::From<getSlashableOperatorsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getSlashableOperatorsCall) -> Self {
                    (value.serviceId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getSlashableOperatorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { serviceId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<getSlashableOperatorsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getSlashableOperatorsReturn) -> Self {
                    (value.operators,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getSlashableOperatorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operators: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getSlashableOperatorsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getSlashableOperatorsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getSlashableOperators(uint64)";
            const SELECTOR: [u8; 4] = [89u8, 220u8, 234u8, 18u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.serviceId),
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
    /**Function with signature `goOffline(uint64)` and selector `0xc5d960bb`.
```solidity
function goOffline(uint64 serviceId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct goOfflineCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
    }
    ///Container type for the return parameters of the [`goOffline(uint64)`](goOfflineCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct goOfflineReturn {}
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
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
            impl ::core::convert::From<goOfflineCall> for UnderlyingRustTuple<'_> {
                fn from(value: goOfflineCall) -> Self {
                    (value.serviceId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for goOfflineCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { serviceId: tuple.0 }
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
            impl ::core::convert::From<goOfflineReturn> for UnderlyingRustTuple<'_> {
                fn from(value: goOfflineReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for goOfflineReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for goOfflineCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = goOfflineReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "goOffline(uint64)";
            const SELECTOR: [u8; 4] = [197u8, 217u8, 96u8, 187u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.serviceId),
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
    /**Function with signature `goOnline(uint64)` and selector `0xb074e9dd`.
```solidity
function goOnline(uint64 serviceId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct goOnlineCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
    }
    ///Container type for the return parameters of the [`goOnline(uint64)`](goOnlineCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct goOnlineReturn {}
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
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
            impl ::core::convert::From<goOnlineCall> for UnderlyingRustTuple<'_> {
                fn from(value: goOnlineCall) -> Self {
                    (value.serviceId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for goOnlineCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { serviceId: tuple.0 }
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
            impl ::core::convert::From<goOnlineReturn> for UnderlyingRustTuple<'_> {
                fn from(value: goOnlineReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for goOnlineReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for goOnlineCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = goOnlineReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "goOnline(uint64)";
            const SELECTOR: [u8; 4] = [176u8, 116u8, 233u8, 221u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.serviceId),
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
    /**Function with signature `heartbeatConfigs(uint64)` and selector `0xda435a7c`.
```solidity
function heartbeatConfigs(uint64) external view returns (uint64 interval, uint8 maxMissed, bool customMetrics);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct heartbeatConfigsCall {
        #[allow(missing_docs)]
        pub _0: u64,
    }
    ///Container type for the return parameters of the [`heartbeatConfigs(uint64)`](heartbeatConfigsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct heartbeatConfigsReturn {
        #[allow(missing_docs)]
        pub interval: u64,
        #[allow(missing_docs)]
        pub maxMissed: u8,
        #[allow(missing_docs)]
        pub customMetrics: bool,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
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
            impl ::core::convert::From<heartbeatConfigsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: heartbeatConfigsCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for heartbeatConfigsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64, u8, bool);
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
            impl ::core::convert::From<heartbeatConfigsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: heartbeatConfigsReturn) -> Self {
                    (value.interval, value.maxMissed, value.customMetrics)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for heartbeatConfigsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        interval: tuple.0,
                        maxMissed: tuple.1,
                        customMetrics: tuple.2,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for heartbeatConfigsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = heartbeatConfigsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Bool,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "heartbeatConfigs(uint64)";
            const SELECTOR: [u8; 4] = [218u8, 67u8, 90u8, 124u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
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
    /**Function with signature `isHeartbeatCurrent(uint64,address)` and selector `0xee1c0390`.
```solidity
function isHeartbeatCurrent(uint64 serviceId, address operator) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isHeartbeatCurrentCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`isHeartbeatCurrent(uint64,address)`](isHeartbeatCurrentCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isHeartbeatCurrentReturn {
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
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64, alloy::sol_types::private::Address);
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
            impl ::core::convert::From<isHeartbeatCurrentCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isHeartbeatCurrentCall) -> Self {
                    (value.serviceId, value.operator)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isHeartbeatCurrentCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        serviceId: tuple.0,
                        operator: tuple.1,
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
            impl ::core::convert::From<isHeartbeatCurrentReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isHeartbeatCurrentReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isHeartbeatCurrentReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isHeartbeatCurrentCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isHeartbeatCurrentReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isHeartbeatCurrent(uint64,address)";
            const SELECTOR: [u8; 4] = [238u8, 28u8, 3u8, 144u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.serviceId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
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
    /**Function with signature `isOnline(uint64,address)` and selector `0x5685cf68`.
```solidity
function isOnline(uint64 serviceId, address operator) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOnlineCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`isOnline(uint64,address)`](isOnlineCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOnlineReturn {
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
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64, alloy::sol_types::private::Address);
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
            impl ::core::convert::From<isOnlineCall> for UnderlyingRustTuple<'_> {
                fn from(value: isOnlineCall) -> Self {
                    (value.serviceId, value.operator)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isOnlineCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        serviceId: tuple.0,
                        operator: tuple.1,
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
            impl ::core::convert::From<isOnlineReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isOnlineReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isOnlineReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isOnlineCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isOnlineReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isOnline(uint64,address)";
            const SELECTOR: [u8; 4] = [86u8, 133u8, 207u8, 104u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.serviceId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
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
    /**Function with signature `metricValues(uint64,address,string)` and selector `0x9cbdae22`.
```solidity
function metricValues(uint64, address, string memory) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct metricValuesCall {
        #[allow(missing_docs)]
        pub _0: u64,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _2: alloy::sol_types::private::String,
    }
    ///Container type for the return parameters of the [`metricValues(uint64,address,string)`](metricValuesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct metricValuesReturn {
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
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::String,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<metricValuesCall> for UnderlyingRustTuple<'_> {
                fn from(value: metricValuesCall) -> Self {
                    (value._0, value._1, value._2)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for metricValuesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                        _2: tuple.2,
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
            impl ::core::convert::From<metricValuesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: metricValuesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for metricValuesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for metricValuesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::String,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = metricValuesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "metricValues(uint64,address,string)";
            const SELECTOR: [u8; 4] = [156u8, 189u8, 174u8, 34u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._1,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self._2,
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
    /**Function with signature `operatorStates(uint64,address)` and selector `0x3e6e34a7`.
```solidity
function operatorStates(uint64, address) external view returns (uint256 lastHeartbeat, uint64 consecutiveBeats, uint8 missedBeats, IOperatorStatusRegistry.StatusCode status, bytes32 lastMetricsHash);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorStatesCall {
        #[allow(missing_docs)]
        pub _0: u64,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`operatorStates(uint64,address)`](operatorStatesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorStatesReturn {
        #[allow(missing_docs)]
        pub lastHeartbeat: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub consecutiveBeats: u64,
        #[allow(missing_docs)]
        pub missedBeats: u8,
        #[allow(missing_docs)]
        pub status: <IOperatorStatusRegistry::StatusCode as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub lastMetricsHash: alloy::sol_types::private::FixedBytes<32>,
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
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64, alloy::sol_types::private::Address);
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
            impl ::core::convert::From<operatorStatesCall> for UnderlyingRustTuple<'_> {
                fn from(value: operatorStatesCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for operatorStatesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<8>,
                IOperatorStatusRegistry::StatusCode,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                u64,
                u8,
                <IOperatorStatusRegistry::StatusCode as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<operatorStatesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: operatorStatesReturn) -> Self {
                    (
                        value.lastHeartbeat,
                        value.consecutiveBeats,
                        value.missedBeats,
                        value.status,
                        value.lastMetricsHash,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for operatorStatesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        lastHeartbeat: tuple.0,
                        consecutiveBeats: tuple.1,
                        missedBeats: tuple.2,
                        status: tuple.3,
                        lastMetricsHash: tuple.4,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for operatorStatesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = operatorStatesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<8>,
                IOperatorStatusRegistry::StatusCode,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "operatorStates(uint64,address)";
            const SELECTOR: [u8; 4] = [62u8, 110u8, 52u8, 167u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._1,
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
    /**Function with signature `registerServiceOwner(uint64,address)` and selector `0x05778550`.
```solidity
function registerServiceOwner(uint64 serviceId, address owner) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerServiceOwnerCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`registerServiceOwner(uint64,address)`](registerServiceOwnerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerServiceOwnerReturn {}
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
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64, alloy::sol_types::private::Address);
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
            impl ::core::convert::From<registerServiceOwnerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerServiceOwnerCall) -> Self {
                    (value.serviceId, value.owner)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerServiceOwnerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        serviceId: tuple.0,
                        owner: tuple.1,
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
            impl ::core::convert::From<registerServiceOwnerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerServiceOwnerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerServiceOwnerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerServiceOwnerCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerServiceOwnerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registerServiceOwner(uint64,address)";
            const SELECTOR: [u8; 4] = [5u8, 119u8, 133u8, 80u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.serviceId),
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
    /**Function with signature `reportForSlashing(uint64,address,string)` and selector `0xadff830c`.
```solidity
function reportForSlashing(uint64 serviceId, address operator, string memory reason) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct reportForSlashingCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub reason: alloy::sol_types::private::String,
    }
    ///Container type for the return parameters of the [`reportForSlashing(uint64,address,string)`](reportForSlashingCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct reportForSlashingReturn {}
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
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::String,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<reportForSlashingCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: reportForSlashingCall) -> Self {
                    (value.serviceId, value.operator, value.reason)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for reportForSlashingCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        serviceId: tuple.0,
                        operator: tuple.1,
                        reason: tuple.2,
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
            impl ::core::convert::From<reportForSlashingReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: reportForSlashingReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for reportForSlashingReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for reportForSlashingCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::String,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = reportForSlashingReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "reportForSlashing(uint64,address,string)";
            const SELECTOR: [u8; 4] = [173u8, 255u8, 131u8, 12u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.serviceId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
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
    /**Function with signature `serviceMetrics(uint64,uint256)` and selector `0x22f1ec93`.
```solidity
function serviceMetrics(uint64, uint256) external view returns (string memory name, uint256 minValue, uint256 maxValue, bool required);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct serviceMetricsCall {
        #[allow(missing_docs)]
        pub _0: u64,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`serviceMetrics(uint64,uint256)`](serviceMetricsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct serviceMetricsReturn {
        #[allow(missing_docs)]
        pub name: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub minValue: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub maxValue: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub required: bool,
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
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
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
            impl ::core::convert::From<serviceMetricsCall> for UnderlyingRustTuple<'_> {
                fn from(value: serviceMetricsCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for serviceMetricsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::String,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                bool,
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
            impl ::core::convert::From<serviceMetricsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: serviceMetricsReturn) -> Self {
                    (value.name, value.minValue, value.maxValue, value.required)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for serviceMetricsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        name: tuple.0,
                        minValue: tuple.1,
                        maxValue: tuple.2,
                        required: tuple.3,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for serviceMetricsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = serviceMetricsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bool,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "serviceMetrics(uint64,uint256)";
            const SELECTOR: [u8; 4] = [34u8, 241u8, 236u8, 147u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
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
    /**Function with signature `serviceOwners(uint64)` and selector `0x56c4e17d`.
```solidity
function serviceOwners(uint64) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct serviceOwnersCall {
        #[allow(missing_docs)]
        pub _0: u64,
    }
    ///Container type for the return parameters of the [`serviceOwners(uint64)`](serviceOwnersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct serviceOwnersReturn {
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
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
            impl ::core::convert::From<serviceOwnersCall> for UnderlyingRustTuple<'_> {
                fn from(value: serviceOwnersCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for serviceOwnersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
            impl ::core::convert::From<serviceOwnersReturn> for UnderlyingRustTuple<'_> {
                fn from(value: serviceOwnersReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for serviceOwnersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for serviceOwnersCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = serviceOwnersReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "serviceOwners(uint64)";
            const SELECTOR: [u8; 4] = [86u8, 196u8, 225u8, 125u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
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
    /**Function with signature `setSlashingOracle(address)` and selector `0x84ef7322`.
```solidity
function setSlashingOracle(address oracle) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setSlashingOracleCall {
        #[allow(missing_docs)]
        pub oracle: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setSlashingOracle(address)`](setSlashingOracleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setSlashingOracleReturn {}
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
            impl ::core::convert::From<setSlashingOracleCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setSlashingOracleCall) -> Self {
                    (value.oracle,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setSlashingOracleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { oracle: tuple.0 }
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
            impl ::core::convert::From<setSlashingOracleReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setSlashingOracleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setSlashingOracleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setSlashingOracleCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setSlashingOracleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setSlashingOracle(address)";
            const SELECTOR: [u8; 4] = [132u8, 239u8, 115u8, 34u8];
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
                        &self.oracle,
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
    /**Function with signature `slashingOracle()` and selector `0xcfe34749`.
```solidity
function slashingOracle() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashingOracleCall {}
    ///Container type for the return parameters of the [`slashingOracle()`](slashingOracleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashingOracleReturn {
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
            impl ::core::convert::From<slashingOracleCall> for UnderlyingRustTuple<'_> {
                fn from(value: slashingOracleCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for slashingOracleCall {
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
            impl ::core::convert::From<slashingOracleReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: slashingOracleReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for slashingOracleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for slashingOracleCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = slashingOracleReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "slashingOracle()";
            const SELECTOR: [u8; 4] = [207u8, 227u8, 71u8, 73u8];
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
    /**Function with signature `submitHeartbeat(uint64,uint64,uint8,bytes,bytes)` and selector `0xd413a580`.
```solidity
function submitHeartbeat(uint64 serviceId, uint64 blueprintId, uint8 statusCode, bytes memory metrics, bytes memory signature) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct submitHeartbeatCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub blueprintId: u64,
        #[allow(missing_docs)]
        pub statusCode: u8,
        #[allow(missing_docs)]
        pub metrics: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`submitHeartbeat(uint64,uint64,uint8,bytes,bytes)`](submitHeartbeatCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct submitHeartbeatReturn {}
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
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
                u64,
                u8,
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
            impl ::core::convert::From<submitHeartbeatCall> for UnderlyingRustTuple<'_> {
                fn from(value: submitHeartbeatCall) -> Self {
                    (
                        value.serviceId,
                        value.blueprintId,
                        value.statusCode,
                        value.metrics,
                        value.signature,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for submitHeartbeatCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        serviceId: tuple.0,
                        blueprintId: tuple.1,
                        statusCode: tuple.2,
                        metrics: tuple.3,
                        signature: tuple.4,
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
            impl ::core::convert::From<submitHeartbeatReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: submitHeartbeatReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for submitHeartbeatReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for submitHeartbeatCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = submitHeartbeatReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "submitHeartbeat(uint64,uint64,uint8,bytes,bytes)";
            const SELECTOR: [u8; 4] = [212u8, 19u8, 165u8, 128u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.serviceId),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.blueprintId),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.statusCode),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.metrics,
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
    /**Function with signature `submitHeartbeatDirect(uint64,uint64,uint8,bytes)` and selector `0x5cce98a6`.
```solidity
function submitHeartbeatDirect(uint64 serviceId, uint64 blueprintId, uint8 statusCode, bytes memory metrics) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct submitHeartbeatDirectCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub blueprintId: u64,
        #[allow(missing_docs)]
        pub statusCode: u8,
        #[allow(missing_docs)]
        pub metrics: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`submitHeartbeatDirect(uint64,uint64,uint8,bytes)`](submitHeartbeatDirectCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct submitHeartbeatDirectReturn {}
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
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
                u64,
                u8,
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
            impl ::core::convert::From<submitHeartbeatDirectCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: submitHeartbeatDirectCall) -> Self {
                    (value.serviceId, value.blueprintId, value.statusCode, value.metrics)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for submitHeartbeatDirectCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        serviceId: tuple.0,
                        blueprintId: tuple.1,
                        statusCode: tuple.2,
                        metrics: tuple.3,
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
            impl ::core::convert::From<submitHeartbeatDirectReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: submitHeartbeatDirectReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for submitHeartbeatDirectReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for submitHeartbeatDirectCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = submitHeartbeatDirectReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "submitHeartbeatDirect(uint64,uint64,uint8,bytes)";
            const SELECTOR: [u8; 4] = [92u8, 206u8, 152u8, 166u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.serviceId),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.blueprintId),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.statusCode),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.metrics,
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
    /**Function with signature `tangleCore()` and selector `0x5a936dc6`.
```solidity
function tangleCore() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct tangleCoreCall {}
    ///Container type for the return parameters of the [`tangleCore()`](tangleCoreCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct tangleCoreReturn {
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
            impl ::core::convert::From<tangleCoreCall> for UnderlyingRustTuple<'_> {
                fn from(value: tangleCoreCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for tangleCoreCall {
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
            impl ::core::convert::From<tangleCoreReturn> for UnderlyingRustTuple<'_> {
                fn from(value: tangleCoreReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for tangleCoreReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for tangleCoreCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = tangleCoreReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "tangleCore()";
            const SELECTOR: [u8; 4] = [90u8, 147u8, 109u8, 198u8];
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
    ///Container for all the [`OperatorStatusRegistry`](self) function calls.
    pub enum OperatorStatusRegistryCalls {
        #[allow(missing_docs)]
        DEFAULT_HEARTBEAT_INTERVAL(DEFAULT_HEARTBEAT_INTERVALCall),
        #[allow(missing_docs)]
        DEFAULT_MAX_MISSED_HEARTBEATS(DEFAULT_MAX_MISSED_HEARTBEATSCall),
        #[allow(missing_docs)]
        DOMAIN_SEPARATOR(DOMAIN_SEPARATORCall),
        #[allow(missing_docs)]
        HEARTBEAT_TYPEHASH(HEARTBEAT_TYPEHASHCall),
        #[allow(missing_docs)]
        addMetricDefinition(addMetricDefinitionCall),
        #[allow(missing_docs)]
        checkOperatorStatus(checkOperatorStatusCall),
        #[allow(missing_docs)]
        checkOperatorsStatus(checkOperatorsStatusCall),
        #[allow(missing_docs)]
        configureHeartbeat(configureHeartbeatCall),
        #[allow(missing_docs)]
        enableCustomMetrics(enableCustomMetricsCall),
        #[allow(missing_docs)]
        getHeartbeatConfig(getHeartbeatConfigCall),
        #[allow(missing_docs)]
        getLastHeartbeat(getLastHeartbeatCall),
        #[allow(missing_docs)]
        getMetricDefinitions(getMetricDefinitionsCall),
        #[allow(missing_docs)]
        getMetricValue(getMetricValueCall),
        #[allow(missing_docs)]
        getOnlineOperatorCount(getOnlineOperatorCountCall),
        #[allow(missing_docs)]
        getOnlineOperators(getOnlineOperatorsCall),
        #[allow(missing_docs)]
        getOperatorState(getOperatorStateCall),
        #[allow(missing_docs)]
        getOperatorStatus(getOperatorStatusCall),
        #[allow(missing_docs)]
        getSlashableOperators(getSlashableOperatorsCall),
        #[allow(missing_docs)]
        goOffline(goOfflineCall),
        #[allow(missing_docs)]
        goOnline(goOnlineCall),
        #[allow(missing_docs)]
        heartbeatConfigs(heartbeatConfigsCall),
        #[allow(missing_docs)]
        isHeartbeatCurrent(isHeartbeatCurrentCall),
        #[allow(missing_docs)]
        isOnline(isOnlineCall),
        #[allow(missing_docs)]
        metricValues(metricValuesCall),
        #[allow(missing_docs)]
        operatorStates(operatorStatesCall),
        #[allow(missing_docs)]
        registerServiceOwner(registerServiceOwnerCall),
        #[allow(missing_docs)]
        reportForSlashing(reportForSlashingCall),
        #[allow(missing_docs)]
        serviceMetrics(serviceMetricsCall),
        #[allow(missing_docs)]
        serviceOwners(serviceOwnersCall),
        #[allow(missing_docs)]
        setSlashingOracle(setSlashingOracleCall),
        #[allow(missing_docs)]
        slashingOracle(slashingOracleCall),
        #[allow(missing_docs)]
        submitHeartbeat(submitHeartbeatCall),
        #[allow(missing_docs)]
        submitHeartbeatDirect(submitHeartbeatDirectCall),
        #[allow(missing_docs)]
        tangleCore(tangleCoreCall),
    }
    #[automatically_derived]
    impl OperatorStatusRegistryCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [5u8, 119u8, 133u8, 80u8],
            [7u8, 88u8, 35u8, 111u8],
            [12u8, 118u8, 105u8, 122u8],
            [34u8, 241u8, 236u8, 147u8],
            [44u8, 149u8, 118u8, 136u8],
            [54u8, 68u8, 229u8, 21u8],
            [62u8, 110u8, 52u8, 167u8],
            [64u8, 35u8, 90u8, 156u8],
            [86u8, 133u8, 207u8, 104u8],
            [86u8, 196u8, 225u8, 125u8],
            [89u8, 220u8, 234u8, 18u8],
            [90u8, 147u8, 109u8, 198u8],
            [92u8, 206u8, 152u8, 166u8],
            [97u8, 214u8, 184u8, 108u8],
            [98u8, 199u8, 232u8, 252u8],
            [113u8, 231u8, 56u8, 140u8],
            [123u8, 159u8, 100u8, 178u8],
            [132u8, 239u8, 115u8, 34u8],
            [150u8, 104u8, 108u8, 30u8],
            [156u8, 189u8, 174u8, 34u8],
            [173u8, 255u8, 131u8, 12u8],
            [174u8, 71u8, 10u8, 133u8],
            [176u8, 116u8, 233u8, 221u8],
            [185u8, 159u8, 103u8, 89u8],
            [186u8, 31u8, 177u8, 3u8],
            [193u8, 239u8, 157u8, 223u8],
            [197u8, 217u8, 96u8, 187u8],
            [207u8, 227u8, 71u8, 73u8],
            [212u8, 19u8, 165u8, 128u8],
            [213u8, 81u8, 22u8, 44u8],
            [218u8, 67u8, 90u8, 124u8],
            [238u8, 28u8, 3u8, 144u8],
            [249u8, 16u8, 127u8, 59u8],
            [249u8, 241u8, 103u8, 98u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for OperatorStatusRegistryCalls {
        const NAME: &'static str = "OperatorStatusRegistryCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 34usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::DEFAULT_HEARTBEAT_INTERVAL(_) => {
                    <DEFAULT_HEARTBEAT_INTERVALCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::DEFAULT_MAX_MISSED_HEARTBEATS(_) => {
                    <DEFAULT_MAX_MISSED_HEARTBEATSCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::DOMAIN_SEPARATOR(_) => {
                    <DOMAIN_SEPARATORCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::HEARTBEAT_TYPEHASH(_) => {
                    <HEARTBEAT_TYPEHASHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addMetricDefinition(_) => {
                    <addMetricDefinitionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::checkOperatorStatus(_) => {
                    <checkOperatorStatusCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::checkOperatorsStatus(_) => {
                    <checkOperatorsStatusCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::configureHeartbeat(_) => {
                    <configureHeartbeatCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::enableCustomMetrics(_) => {
                    <enableCustomMetricsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getHeartbeatConfig(_) => {
                    <getHeartbeatConfigCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLastHeartbeat(_) => {
                    <getLastHeartbeatCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getMetricDefinitions(_) => {
                    <getMetricDefinitionsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getMetricValue(_) => {
                    <getMetricValueCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOnlineOperatorCount(_) => {
                    <getOnlineOperatorCountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOnlineOperators(_) => {
                    <getOnlineOperatorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorState(_) => {
                    <getOperatorStateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorStatus(_) => {
                    <getOperatorStatusCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getSlashableOperators(_) => {
                    <getSlashableOperatorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::goOffline(_) => {
                    <goOfflineCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::goOnline(_) => <goOnlineCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::heartbeatConfigs(_) => {
                    <heartbeatConfigsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isHeartbeatCurrent(_) => {
                    <isHeartbeatCurrentCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isOnline(_) => <isOnlineCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::metricValues(_) => {
                    <metricValuesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::operatorStates(_) => {
                    <operatorStatesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registerServiceOwner(_) => {
                    <registerServiceOwnerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::reportForSlashing(_) => {
                    <reportForSlashingCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::serviceMetrics(_) => {
                    <serviceMetricsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::serviceOwners(_) => {
                    <serviceOwnersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setSlashingOracle(_) => {
                    <setSlashingOracleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::slashingOracle(_) => {
                    <slashingOracleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::submitHeartbeat(_) => {
                    <submitHeartbeatCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::submitHeartbeatDirect(_) => {
                    <submitHeartbeatDirectCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::tangleCore(_) => {
                    <tangleCoreCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<OperatorStatusRegistryCalls>] = &[
                {
                    fn registerServiceOwner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorStatusRegistryCalls> {
                        <registerServiceOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OperatorStatusRegistryCalls::registerServiceOwner)
                    }
                    registerServiceOwner
                },
                {
                    fn getHeartbeatConfig(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorStatusRegistryCalls> {
                        <getHeartbeatConfigCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OperatorStatusRegistryCalls::getHeartbeatConfig)
                    }
                    getHeartbeatConfig
                },
                {
                    fn getLastHeartbeat(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorStatusRegistryCalls> {
                        <getLastHeartbeatCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OperatorStatusRegistryCalls::getLastHeartbeat)
                    }
                    getLastHeartbeat
                },
                {
                    fn serviceMetrics(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorStatusRegistryCalls> {
                        <serviceMetricsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OperatorStatusRegistryCalls::serviceMetrics)
                    }
                    serviceMetrics
                },
                {
                    fn DEFAULT_HEARTBEAT_INTERVAL(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorStatusRegistryCalls> {
                        <DEFAULT_HEARTBEAT_INTERVALCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OperatorStatusRegistryCalls::DEFAULT_HEARTBEAT_INTERVAL)
                    }
                    DEFAULT_HEARTBEAT_INTERVAL
                },
                {
                    fn DOMAIN_SEPARATOR(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorStatusRegistryCalls> {
                        <DOMAIN_SEPARATORCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OperatorStatusRegistryCalls::DOMAIN_SEPARATOR)
                    }
                    DOMAIN_SEPARATOR
                },
                {
                    fn operatorStates(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorStatusRegistryCalls> {
                        <operatorStatesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OperatorStatusRegistryCalls::operatorStates)
                    }
                    operatorStates
                },
                {
                    fn getOnlineOperators(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorStatusRegistryCalls> {
                        <getOnlineOperatorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OperatorStatusRegistryCalls::getOnlineOperators)
                    }
                    getOnlineOperators
                },
                {
                    fn isOnline(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorStatusRegistryCalls> {
                        <isOnlineCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OperatorStatusRegistryCalls::isOnline)
                    }
                    isOnline
                },
                {
                    fn serviceOwners(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorStatusRegistryCalls> {
                        <serviceOwnersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OperatorStatusRegistryCalls::serviceOwners)
                    }
                    serviceOwners
                },
                {
                    fn getSlashableOperators(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorStatusRegistryCalls> {
                        <getSlashableOperatorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OperatorStatusRegistryCalls::getSlashableOperators)
                    }
                    getSlashableOperators
                },
                {
                    fn tangleCore(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorStatusRegistryCalls> {
                        <tangleCoreCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OperatorStatusRegistryCalls::tangleCore)
                    }
                    tangleCore
                },
                {
                    fn submitHeartbeatDirect(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorStatusRegistryCalls> {
                        <submitHeartbeatDirectCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OperatorStatusRegistryCalls::submitHeartbeatDirect)
                    }
                    submitHeartbeatDirect
                },
                {
                    fn DEFAULT_MAX_MISSED_HEARTBEATS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorStatusRegistryCalls> {
                        <DEFAULT_MAX_MISSED_HEARTBEATSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                OperatorStatusRegistryCalls::DEFAULT_MAX_MISSED_HEARTBEATS,
                            )
                    }
                    DEFAULT_MAX_MISSED_HEARTBEATS
                },
                {
                    fn getOperatorStatus(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorStatusRegistryCalls> {
                        <getOperatorStatusCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OperatorStatusRegistryCalls::getOperatorStatus)
                    }
                    getOperatorStatus
                },
                {
                    fn getOperatorState(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorStatusRegistryCalls> {
                        <getOperatorStateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OperatorStatusRegistryCalls::getOperatorState)
                    }
                    getOperatorState
                },
                {
                    fn getOnlineOperatorCount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorStatusRegistryCalls> {
                        <getOnlineOperatorCountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OperatorStatusRegistryCalls::getOnlineOperatorCount)
                    }
                    getOnlineOperatorCount
                },
                {
                    fn setSlashingOracle(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorStatusRegistryCalls> {
                        <setSlashingOracleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OperatorStatusRegistryCalls::setSlashingOracle)
                    }
                    setSlashingOracle
                },
                {
                    fn checkOperatorsStatus(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorStatusRegistryCalls> {
                        <checkOperatorsStatusCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OperatorStatusRegistryCalls::checkOperatorsStatus)
                    }
                    checkOperatorsStatus
                },
                {
                    fn metricValues(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorStatusRegistryCalls> {
                        <metricValuesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OperatorStatusRegistryCalls::metricValues)
                    }
                    metricValues
                },
                {
                    fn reportForSlashing(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorStatusRegistryCalls> {
                        <reportForSlashingCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OperatorStatusRegistryCalls::reportForSlashing)
                    }
                    reportForSlashing
                },
                {
                    fn addMetricDefinition(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorStatusRegistryCalls> {
                        <addMetricDefinitionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OperatorStatusRegistryCalls::addMetricDefinition)
                    }
                    addMetricDefinition
                },
                {
                    fn goOnline(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorStatusRegistryCalls> {
                        <goOnlineCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OperatorStatusRegistryCalls::goOnline)
                    }
                    goOnline
                },
                {
                    fn configureHeartbeat(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorStatusRegistryCalls> {
                        <configureHeartbeatCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OperatorStatusRegistryCalls::configureHeartbeat)
                    }
                    configureHeartbeat
                },
                {
                    fn checkOperatorStatus(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorStatusRegistryCalls> {
                        <checkOperatorStatusCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OperatorStatusRegistryCalls::checkOperatorStatus)
                    }
                    checkOperatorStatus
                },
                {
                    fn getMetricDefinitions(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorStatusRegistryCalls> {
                        <getMetricDefinitionsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OperatorStatusRegistryCalls::getMetricDefinitions)
                    }
                    getMetricDefinitions
                },
                {
                    fn goOffline(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorStatusRegistryCalls> {
                        <goOfflineCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OperatorStatusRegistryCalls::goOffline)
                    }
                    goOffline
                },
                {
                    fn slashingOracle(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorStatusRegistryCalls> {
                        <slashingOracleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OperatorStatusRegistryCalls::slashingOracle)
                    }
                    slashingOracle
                },
                {
                    fn submitHeartbeat(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorStatusRegistryCalls> {
                        <submitHeartbeatCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OperatorStatusRegistryCalls::submitHeartbeat)
                    }
                    submitHeartbeat
                },
                {
                    fn getMetricValue(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorStatusRegistryCalls> {
                        <getMetricValueCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OperatorStatusRegistryCalls::getMetricValue)
                    }
                    getMetricValue
                },
                {
                    fn heartbeatConfigs(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorStatusRegistryCalls> {
                        <heartbeatConfigsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OperatorStatusRegistryCalls::heartbeatConfigs)
                    }
                    heartbeatConfigs
                },
                {
                    fn isHeartbeatCurrent(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorStatusRegistryCalls> {
                        <isHeartbeatCurrentCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OperatorStatusRegistryCalls::isHeartbeatCurrent)
                    }
                    isHeartbeatCurrent
                },
                {
                    fn enableCustomMetrics(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorStatusRegistryCalls> {
                        <enableCustomMetricsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OperatorStatusRegistryCalls::enableCustomMetrics)
                    }
                    enableCustomMetrics
                },
                {
                    fn HEARTBEAT_TYPEHASH(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorStatusRegistryCalls> {
                        <HEARTBEAT_TYPEHASHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OperatorStatusRegistryCalls::HEARTBEAT_TYPEHASH)
                    }
                    HEARTBEAT_TYPEHASH
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
                Self::DEFAULT_HEARTBEAT_INTERVAL(inner) => {
                    <DEFAULT_HEARTBEAT_INTERVALCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DEFAULT_MAX_MISSED_HEARTBEATS(inner) => {
                    <DEFAULT_MAX_MISSED_HEARTBEATSCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DOMAIN_SEPARATOR(inner) => {
                    <DOMAIN_SEPARATORCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::HEARTBEAT_TYPEHASH(inner) => {
                    <HEARTBEAT_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::addMetricDefinition(inner) => {
                    <addMetricDefinitionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::checkOperatorStatus(inner) => {
                    <checkOperatorStatusCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::checkOperatorsStatus(inner) => {
                    <checkOperatorsStatusCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::configureHeartbeat(inner) => {
                    <configureHeartbeatCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::enableCustomMetrics(inner) => {
                    <enableCustomMetricsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getHeartbeatConfig(inner) => {
                    <getHeartbeatConfigCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLastHeartbeat(inner) => {
                    <getLastHeartbeatCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getMetricDefinitions(inner) => {
                    <getMetricDefinitionsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getMetricValue(inner) => {
                    <getMetricValueCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOnlineOperatorCount(inner) => {
                    <getOnlineOperatorCountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOnlineOperators(inner) => {
                    <getOnlineOperatorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorState(inner) => {
                    <getOperatorStateCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorStatus(inner) => {
                    <getOperatorStatusCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getSlashableOperators(inner) => {
                    <getSlashableOperatorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::goOffline(inner) => {
                    <goOfflineCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::goOnline(inner) => {
                    <goOnlineCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::heartbeatConfigs(inner) => {
                    <heartbeatConfigsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isHeartbeatCurrent(inner) => {
                    <isHeartbeatCurrentCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isOnline(inner) => {
                    <isOnlineCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::metricValues(inner) => {
                    <metricValuesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::operatorStates(inner) => {
                    <operatorStatesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::registerServiceOwner(inner) => {
                    <registerServiceOwnerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::reportForSlashing(inner) => {
                    <reportForSlashingCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::serviceMetrics(inner) => {
                    <serviceMetricsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::serviceOwners(inner) => {
                    <serviceOwnersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setSlashingOracle(inner) => {
                    <setSlashingOracleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::slashingOracle(inner) => {
                    <slashingOracleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::submitHeartbeat(inner) => {
                    <submitHeartbeatCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::submitHeartbeatDirect(inner) => {
                    <submitHeartbeatDirectCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::tangleCore(inner) => {
                    <tangleCoreCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::DEFAULT_HEARTBEAT_INTERVAL(inner) => {
                    <DEFAULT_HEARTBEAT_INTERVALCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DEFAULT_MAX_MISSED_HEARTBEATS(inner) => {
                    <DEFAULT_MAX_MISSED_HEARTBEATSCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DOMAIN_SEPARATOR(inner) => {
                    <DOMAIN_SEPARATORCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::HEARTBEAT_TYPEHASH(inner) => {
                    <HEARTBEAT_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::addMetricDefinition(inner) => {
                    <addMetricDefinitionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::checkOperatorStatus(inner) => {
                    <checkOperatorStatusCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::checkOperatorsStatus(inner) => {
                    <checkOperatorsStatusCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::configureHeartbeat(inner) => {
                    <configureHeartbeatCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::enableCustomMetrics(inner) => {
                    <enableCustomMetricsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getHeartbeatConfig(inner) => {
                    <getHeartbeatConfigCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLastHeartbeat(inner) => {
                    <getLastHeartbeatCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getMetricDefinitions(inner) => {
                    <getMetricDefinitionsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getMetricValue(inner) => {
                    <getMetricValueCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOnlineOperatorCount(inner) => {
                    <getOnlineOperatorCountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOnlineOperators(inner) => {
                    <getOnlineOperatorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorState(inner) => {
                    <getOperatorStateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorStatus(inner) => {
                    <getOperatorStatusCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getSlashableOperators(inner) => {
                    <getSlashableOperatorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::goOffline(inner) => {
                    <goOfflineCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::goOnline(inner) => {
                    <goOnlineCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::heartbeatConfigs(inner) => {
                    <heartbeatConfigsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isHeartbeatCurrent(inner) => {
                    <isHeartbeatCurrentCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isOnline(inner) => {
                    <isOnlineCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::metricValues(inner) => {
                    <metricValuesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::operatorStates(inner) => {
                    <operatorStatesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::registerServiceOwner(inner) => {
                    <registerServiceOwnerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::reportForSlashing(inner) => {
                    <reportForSlashingCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::serviceMetrics(inner) => {
                    <serviceMetricsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::serviceOwners(inner) => {
                    <serviceOwnersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setSlashingOracle(inner) => {
                    <setSlashingOracleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::slashingOracle(inner) => {
                    <slashingOracleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::submitHeartbeat(inner) => {
                    <submitHeartbeatCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::submitHeartbeatDirect(inner) => {
                    <submitHeartbeatDirectCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::tangleCore(inner) => {
                    <tangleCoreCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`OperatorStatusRegistry`](self) custom errors.
    pub enum OperatorStatusRegistryErrors {
        #[allow(missing_docs)]
        ECDSAInvalidSignature(ECDSAInvalidSignature),
        #[allow(missing_docs)]
        ECDSAInvalidSignatureLength(ECDSAInvalidSignatureLength),
        #[allow(missing_docs)]
        ECDSAInvalidSignatureS(ECDSAInvalidSignatureS),
    }
    #[automatically_derived]
    impl OperatorStatusRegistryErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [215u8, 139u8, 206u8, 12u8],
            [246u8, 69u8, 238u8, 223u8],
            [252u8, 230u8, 152u8, 247u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for OperatorStatusRegistryErrors {
        const NAME: &'static str = "OperatorStatusRegistryErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 3usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::ECDSAInvalidSignature(_) => {
                    <ECDSAInvalidSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ECDSAInvalidSignatureLength(_) => {
                    <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ECDSAInvalidSignatureS(_) => {
                    <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<OperatorStatusRegistryErrors>] = &[
                {
                    fn ECDSAInvalidSignatureS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorStatusRegistryErrors> {
                        <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OperatorStatusRegistryErrors::ECDSAInvalidSignatureS)
                    }
                    ECDSAInvalidSignatureS
                },
                {
                    fn ECDSAInvalidSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorStatusRegistryErrors> {
                        <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OperatorStatusRegistryErrors::ECDSAInvalidSignature)
                    }
                    ECDSAInvalidSignature
                },
                {
                    fn ECDSAInvalidSignatureLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OperatorStatusRegistryErrors> {
                        <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                OperatorStatusRegistryErrors::ECDSAInvalidSignatureLength,
                            )
                    }
                    ECDSAInvalidSignatureLength
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
                Self::ECDSAInvalidSignature(inner) => {
                    <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ECDSAInvalidSignatureLength(inner) => {
                    <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ECDSAInvalidSignatureS(inner) => {
                    <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::ECDSAInvalidSignature(inner) => {
                    <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ECDSAInvalidSignatureLength(inner) => {
                    <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ECDSAInvalidSignatureS(inner) => {
                    <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`OperatorStatusRegistry`](self) events.
    pub enum OperatorStatusRegistryEvents {
        #[allow(missing_docs)]
        HeartbeatConfigUpdated(HeartbeatConfigUpdated),
        #[allow(missing_docs)]
        HeartbeatReceived(HeartbeatReceived),
        #[allow(missing_docs)]
        MetricReported(MetricReported),
        #[allow(missing_docs)]
        OperatorCameOnline(OperatorCameOnline),
        #[allow(missing_docs)]
        OperatorWentOffline(OperatorWentOffline),
        #[allow(missing_docs)]
        SlashingTriggered(SlashingTriggered),
        #[allow(missing_docs)]
        StatusChanged(StatusChanged),
    }
    #[automatically_derived]
    impl OperatorStatusRegistryEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                30u8,
                41u8,
                9u8,
                207u8,
                69u8,
                215u8,
                12u8,
                240u8,
                3u8,
                243u8,
                52u8,
                183u8,
                60u8,
                147u8,
                51u8,
                12u8,
                231u8,
                229u8,
                114u8,
                120u8,
                45u8,
                252u8,
                130u8,
                250u8,
                183u8,
                157u8,
                235u8,
                136u8,
                85u8,
                167u8,
                199u8,
                145u8,
            ],
            [
                34u8,
                136u8,
                36u8,
                184u8,
                108u8,
                37u8,
                100u8,
                105u8,
                18u8,
                95u8,
                82u8,
                92u8,
                225u8,
                140u8,
                108u8,
                45u8,
                10u8,
                158u8,
                19u8,
                61u8,
                19u8,
                184u8,
                236u8,
                122u8,
                44u8,
                150u8,
                161u8,
                147u8,
                176u8,
                194u8,
                138u8,
                9u8,
            ],
            [
                35u8,
                237u8,
                2u8,
                189u8,
                54u8,
                5u8,
                189u8,
                234u8,
                106u8,
                138u8,
                250u8,
                118u8,
                196u8,
                111u8,
                0u8,
                210u8,
                116u8,
                134u8,
                11u8,
                166u8,
                206u8,
                169u8,
                128u8,
                242u8,
                88u8,
                91u8,
                105u8,
                109u8,
                249u8,
                225u8,
                130u8,
                189u8,
            ],
            [
                68u8,
                253u8,
                50u8,
                182u8,
                119u8,
                112u8,
                76u8,
                230u8,
                142u8,
                119u8,
                99u8,
                137u8,
                124u8,
                73u8,
                115u8,
                59u8,
                143u8,
                82u8,
                137u8,
                1u8,
                138u8,
                198u8,
                10u8,
                92u8,
                146u8,
                104u8,
                2u8,
                214u8,
                55u8,
                89u8,
                219u8,
                77u8,
            ],
            [
                101u8,
                137u8,
                24u8,
                227u8,
                20u8,
                127u8,
                19u8,
                221u8,
                6u8,
                142u8,
                194u8,
                20u8,
                55u8,
                180u8,
                194u8,
                92u8,
                33u8,
                104u8,
                42u8,
                141u8,
                194u8,
                18u8,
                147u8,
                72u8,
                103u8,
                30u8,
                173u8,
                0u8,
                13u8,
                179u8,
                231u8,
                185u8,
            ],
            [
                201u8,
                89u8,
                158u8,
                217u8,
                98u8,
                98u8,
                74u8,
                133u8,
                142u8,
                197u8,
                155u8,
                174u8,
                14u8,
                216u8,
                108u8,
                117u8,
                244u8,
                219u8,
                101u8,
                254u8,
                4u8,
                87u8,
                0u8,
                33u8,
                39u8,
                126u8,
                219u8,
                237u8,
                208u8,
                78u8,
                165u8,
                100u8,
            ],
            [
                201u8,
                134u8,
                44u8,
                95u8,
                2u8,
                238u8,
                251u8,
                220u8,
                234u8,
                1u8,
                194u8,
                7u8,
                174u8,
                83u8,
                142u8,
                29u8,
                48u8,
                77u8,
                201u8,
                48u8,
                38u8,
                135u8,
                15u8,
                72u8,
                149u8,
                30u8,
                72u8,
                160u8,
                244u8,
                200u8,
                71u8,
                12u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for OperatorStatusRegistryEvents {
        const NAME: &'static str = "OperatorStatusRegistryEvents";
        const COUNT: usize = 7usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <HeartbeatConfigUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <HeartbeatConfigUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::HeartbeatConfigUpdated)
                }
                Some(
                    <HeartbeatReceived as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <HeartbeatReceived as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::HeartbeatReceived)
                }
                Some(<MetricReported as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <MetricReported as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::MetricReported)
                }
                Some(
                    <OperatorCameOnline as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorCameOnline as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorCameOnline)
                }
                Some(
                    <OperatorWentOffline as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorWentOffline as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorWentOffline)
                }
                Some(
                    <SlashingTriggered as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <SlashingTriggered as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::SlashingTriggered)
                }
                Some(<StatusChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <StatusChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::StatusChanged)
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
    impl alloy_sol_types::private::IntoLogData for OperatorStatusRegistryEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::HeartbeatConfigUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::HeartbeatReceived(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::MetricReported(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorCameOnline(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorWentOffline(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SlashingTriggered(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::StatusChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::HeartbeatConfigUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::HeartbeatReceived(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::MetricReported(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorCameOnline(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorWentOffline(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SlashingTriggered(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::StatusChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`OperatorStatusRegistry`](self) contract instance.

See the [wrapper's documentation](`OperatorStatusRegistryInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> OperatorStatusRegistryInstance<T, P, N> {
        OperatorStatusRegistryInstance::<T, P, N>::new(address, provider)
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
        _tangleCore: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<OperatorStatusRegistryInstance<T, P, N>>,
    > {
        OperatorStatusRegistryInstance::<T, P, N>::deploy(provider, _tangleCore)
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
    >(
        provider: P,
        _tangleCore: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        OperatorStatusRegistryInstance::<T, P, N>::deploy_builder(provider, _tangleCore)
    }
    /**A [`OperatorStatusRegistry`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`OperatorStatusRegistry`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct OperatorStatusRegistryInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for OperatorStatusRegistryInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("OperatorStatusRegistryInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > OperatorStatusRegistryInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`OperatorStatusRegistry`](self) contract instance.

See the [wrapper's documentation](`OperatorStatusRegistryInstance`) for more details.*/
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
            _tangleCore: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<OperatorStatusRegistryInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, _tangleCore);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            provider: P,
            _tangleCore: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall { _tangleCore },
                    )[..],
                ]
                    .concat()
                    .into(),
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
    impl<T, P: ::core::clone::Clone, N> OperatorStatusRegistryInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> OperatorStatusRegistryInstance<T, P, N> {
            OperatorStatusRegistryInstance {
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
    > OperatorStatusRegistryInstance<T, P, N> {
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
        ///Creates a new call builder for the [`DEFAULT_HEARTBEAT_INTERVAL`] function.
        pub fn DEFAULT_HEARTBEAT_INTERVAL(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, DEFAULT_HEARTBEAT_INTERVALCall, N> {
            self.call_builder(&DEFAULT_HEARTBEAT_INTERVALCall {})
        }
        ///Creates a new call builder for the [`DEFAULT_MAX_MISSED_HEARTBEATS`] function.
        pub fn DEFAULT_MAX_MISSED_HEARTBEATS(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            DEFAULT_MAX_MISSED_HEARTBEATSCall,
            N,
        > {
            self.call_builder(
                &DEFAULT_MAX_MISSED_HEARTBEATSCall {
                },
            )
        }
        ///Creates a new call builder for the [`DOMAIN_SEPARATOR`] function.
        pub fn DOMAIN_SEPARATOR(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, DOMAIN_SEPARATORCall, N> {
            self.call_builder(&DOMAIN_SEPARATORCall {})
        }
        ///Creates a new call builder for the [`HEARTBEAT_TYPEHASH`] function.
        pub fn HEARTBEAT_TYPEHASH(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, HEARTBEAT_TYPEHASHCall, N> {
            self.call_builder(&HEARTBEAT_TYPEHASHCall {})
        }
        ///Creates a new call builder for the [`addMetricDefinition`] function.
        pub fn addMetricDefinition(
            &self,
            serviceId: u64,
            name: alloy::sol_types::private::String,
            minValue: alloy::sol_types::private::primitives::aliases::U256,
            maxValue: alloy::sol_types::private::primitives::aliases::U256,
            required: bool,
        ) -> alloy_contract::SolCallBuilder<T, &P, addMetricDefinitionCall, N> {
            self.call_builder(
                &addMetricDefinitionCall {
                    serviceId,
                    name,
                    minValue,
                    maxValue,
                    required,
                },
            )
        }
        ///Creates a new call builder for the [`checkOperatorStatus`] function.
        pub fn checkOperatorStatus(
            &self,
            serviceId: u64,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, checkOperatorStatusCall, N> {
            self.call_builder(
                &checkOperatorStatusCall {
                    serviceId,
                    operator,
                },
            )
        }
        ///Creates a new call builder for the [`checkOperatorsStatus`] function.
        pub fn checkOperatorsStatus(
            &self,
            serviceId: u64,
            operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, checkOperatorsStatusCall, N> {
            self.call_builder(
                &checkOperatorsStatusCall {
                    serviceId,
                    operators,
                },
            )
        }
        ///Creates a new call builder for the [`configureHeartbeat`] function.
        pub fn configureHeartbeat(
            &self,
            serviceId: u64,
            interval: u64,
            maxMissed: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, configureHeartbeatCall, N> {
            self.call_builder(
                &configureHeartbeatCall {
                    serviceId,
                    interval,
                    maxMissed,
                },
            )
        }
        ///Creates a new call builder for the [`enableCustomMetrics`] function.
        pub fn enableCustomMetrics(
            &self,
            serviceId: u64,
            enabled: bool,
        ) -> alloy_contract::SolCallBuilder<T, &P, enableCustomMetricsCall, N> {
            self.call_builder(
                &enableCustomMetricsCall {
                    serviceId,
                    enabled,
                },
            )
        }
        ///Creates a new call builder for the [`getHeartbeatConfig`] function.
        pub fn getHeartbeatConfig(
            &self,
            serviceId: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, getHeartbeatConfigCall, N> {
            self.call_builder(
                &getHeartbeatConfigCall {
                    serviceId,
                },
            )
        }
        ///Creates a new call builder for the [`getLastHeartbeat`] function.
        pub fn getLastHeartbeat(
            &self,
            serviceId: u64,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getLastHeartbeatCall, N> {
            self.call_builder(
                &getLastHeartbeatCall {
                    serviceId,
                    operator,
                },
            )
        }
        ///Creates a new call builder for the [`getMetricDefinitions`] function.
        pub fn getMetricDefinitions(
            &self,
            serviceId: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, getMetricDefinitionsCall, N> {
            self.call_builder(
                &getMetricDefinitionsCall {
                    serviceId,
                },
            )
        }
        ///Creates a new call builder for the [`getMetricValue`] function.
        pub fn getMetricValue(
            &self,
            serviceId: u64,
            operator: alloy::sol_types::private::Address,
            metricName: alloy::sol_types::private::String,
        ) -> alloy_contract::SolCallBuilder<T, &P, getMetricValueCall, N> {
            self.call_builder(
                &getMetricValueCall {
                    serviceId,
                    operator,
                    metricName,
                },
            )
        }
        ///Creates a new call builder for the [`getOnlineOperatorCount`] function.
        pub fn getOnlineOperatorCount(
            &self,
            serviceId: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOnlineOperatorCountCall, N> {
            self.call_builder(
                &getOnlineOperatorCountCall {
                    serviceId,
                },
            )
        }
        ///Creates a new call builder for the [`getOnlineOperators`] function.
        pub fn getOnlineOperators(
            &self,
            serviceId: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOnlineOperatorsCall, N> {
            self.call_builder(
                &getOnlineOperatorsCall {
                    serviceId,
                },
            )
        }
        ///Creates a new call builder for the [`getOperatorState`] function.
        pub fn getOperatorState(
            &self,
            serviceId: u64,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorStateCall, N> {
            self.call_builder(
                &getOperatorStateCall {
                    serviceId,
                    operator,
                },
            )
        }
        ///Creates a new call builder for the [`getOperatorStatus`] function.
        pub fn getOperatorStatus(
            &self,
            serviceId: u64,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorStatusCall, N> {
            self.call_builder(
                &getOperatorStatusCall {
                    serviceId,
                    operator,
                },
            )
        }
        ///Creates a new call builder for the [`getSlashableOperators`] function.
        pub fn getSlashableOperators(
            &self,
            serviceId: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, getSlashableOperatorsCall, N> {
            self.call_builder(
                &getSlashableOperatorsCall {
                    serviceId,
                },
            )
        }
        ///Creates a new call builder for the [`goOffline`] function.
        pub fn goOffline(
            &self,
            serviceId: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, goOfflineCall, N> {
            self.call_builder(&goOfflineCall { serviceId })
        }
        ///Creates a new call builder for the [`goOnline`] function.
        pub fn goOnline(
            &self,
            serviceId: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, goOnlineCall, N> {
            self.call_builder(&goOnlineCall { serviceId })
        }
        ///Creates a new call builder for the [`heartbeatConfigs`] function.
        pub fn heartbeatConfigs(
            &self,
            _0: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, heartbeatConfigsCall, N> {
            self.call_builder(&heartbeatConfigsCall { _0 })
        }
        ///Creates a new call builder for the [`isHeartbeatCurrent`] function.
        pub fn isHeartbeatCurrent(
            &self,
            serviceId: u64,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, isHeartbeatCurrentCall, N> {
            self.call_builder(
                &isHeartbeatCurrentCall {
                    serviceId,
                    operator,
                },
            )
        }
        ///Creates a new call builder for the [`isOnline`] function.
        pub fn isOnline(
            &self,
            serviceId: u64,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, isOnlineCall, N> {
            self.call_builder(
                &isOnlineCall {
                    serviceId,
                    operator,
                },
            )
        }
        ///Creates a new call builder for the [`metricValues`] function.
        pub fn metricValues(
            &self,
            _0: u64,
            _1: alloy::sol_types::private::Address,
            _2: alloy::sol_types::private::String,
        ) -> alloy_contract::SolCallBuilder<T, &P, metricValuesCall, N> {
            self.call_builder(&metricValuesCall { _0, _1, _2 })
        }
        ///Creates a new call builder for the [`operatorStates`] function.
        pub fn operatorStates(
            &self,
            _0: u64,
            _1: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, operatorStatesCall, N> {
            self.call_builder(&operatorStatesCall { _0, _1 })
        }
        ///Creates a new call builder for the [`registerServiceOwner`] function.
        pub fn registerServiceOwner(
            &self,
            serviceId: u64,
            owner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerServiceOwnerCall, N> {
            self.call_builder(
                &registerServiceOwnerCall {
                    serviceId,
                    owner,
                },
            )
        }
        ///Creates a new call builder for the [`reportForSlashing`] function.
        pub fn reportForSlashing(
            &self,
            serviceId: u64,
            operator: alloy::sol_types::private::Address,
            reason: alloy::sol_types::private::String,
        ) -> alloy_contract::SolCallBuilder<T, &P, reportForSlashingCall, N> {
            self.call_builder(
                &reportForSlashingCall {
                    serviceId,
                    operator,
                    reason,
                },
            )
        }
        ///Creates a new call builder for the [`serviceMetrics`] function.
        pub fn serviceMetrics(
            &self,
            _0: u64,
            _1: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, serviceMetricsCall, N> {
            self.call_builder(&serviceMetricsCall { _0, _1 })
        }
        ///Creates a new call builder for the [`serviceOwners`] function.
        pub fn serviceOwners(
            &self,
            _0: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, serviceOwnersCall, N> {
            self.call_builder(&serviceOwnersCall { _0 })
        }
        ///Creates a new call builder for the [`setSlashingOracle`] function.
        pub fn setSlashingOracle(
            &self,
            oracle: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setSlashingOracleCall, N> {
            self.call_builder(&setSlashingOracleCall { oracle })
        }
        ///Creates a new call builder for the [`slashingOracle`] function.
        pub fn slashingOracle(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, slashingOracleCall, N> {
            self.call_builder(&slashingOracleCall {})
        }
        ///Creates a new call builder for the [`submitHeartbeat`] function.
        pub fn submitHeartbeat(
            &self,
            serviceId: u64,
            blueprintId: u64,
            statusCode: u8,
            metrics: alloy::sol_types::private::Bytes,
            signature: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, submitHeartbeatCall, N> {
            self.call_builder(
                &submitHeartbeatCall {
                    serviceId,
                    blueprintId,
                    statusCode,
                    metrics,
                    signature,
                },
            )
        }
        ///Creates a new call builder for the [`submitHeartbeatDirect`] function.
        pub fn submitHeartbeatDirect(
            &self,
            serviceId: u64,
            blueprintId: u64,
            statusCode: u8,
            metrics: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, submitHeartbeatDirectCall, N> {
            self.call_builder(
                &submitHeartbeatDirectCall {
                    serviceId,
                    blueprintId,
                    statusCode,
                    metrics,
                },
            )
        }
        ///Creates a new call builder for the [`tangleCore`] function.
        pub fn tangleCore(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, tangleCoreCall, N> {
            self.call_builder(&tangleCoreCall {})
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > OperatorStatusRegistryInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`HeartbeatConfigUpdated`] event.
        pub fn HeartbeatConfigUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, HeartbeatConfigUpdated, N> {
            self.event_filter::<HeartbeatConfigUpdated>()
        }
        ///Creates a new event filter for the [`HeartbeatReceived`] event.
        pub fn HeartbeatReceived_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, HeartbeatReceived, N> {
            self.event_filter::<HeartbeatReceived>()
        }
        ///Creates a new event filter for the [`MetricReported`] event.
        pub fn MetricReported_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, MetricReported, N> {
            self.event_filter::<MetricReported>()
        }
        ///Creates a new event filter for the [`OperatorCameOnline`] event.
        pub fn OperatorCameOnline_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorCameOnline, N> {
            self.event_filter::<OperatorCameOnline>()
        }
        ///Creates a new event filter for the [`OperatorWentOffline`] event.
        pub fn OperatorWentOffline_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorWentOffline, N> {
            self.event_filter::<OperatorWentOffline>()
        }
        ///Creates a new event filter for the [`SlashingTriggered`] event.
        pub fn SlashingTriggered_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, SlashingTriggered, N> {
            self.event_filter::<SlashingTriggered>()
        }
        ///Creates a new event filter for the [`StatusChanged`] event.
        pub fn StatusChanged_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, StatusChanged, N> {
            self.event_filter::<StatusChanged>()
        }
    }
}
