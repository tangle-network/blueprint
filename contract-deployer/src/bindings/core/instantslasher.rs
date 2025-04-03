#![allow(clippy::all, clippy::pedantic, clippy::nursery, warnings, unknown_lints, rustdoc::all, elided_lifetimes_in_paths)]
use InstantSlasher::*;

///Module containing a contract's types and functions.
/**

```solidity
library IAllocationManagerTypes {
    struct SlashingParams { address operator; uint32 operatorSetId; address[] strategies; uint256[] wadsToSlash; string description; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IAllocationManagerTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct SlashingParams { address operator; uint32 operatorSetId; address[] strategies; uint256[] wadsToSlash; string description; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SlashingParams {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSetId: u32,
        #[allow(missing_docs)]
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub wadsToSlash: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
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
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            alloy::sol_types::sol_data::String,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            u32,
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
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
        impl ::core::convert::From<SlashingParams> for UnderlyingRustTuple<'_> {
            fn from(value: SlashingParams) -> Self {
                (
                    value.operator,
                    value.operatorSetId,
                    value.strategies,
                    value.wadsToSlash,
                    value.description,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SlashingParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    operator: tuple.0,
                    operatorSetId: tuple.1,
                    strategies: tuple.2,
                    wadsToSlash: tuple.3,
                    description: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for SlashingParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for SlashingParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSetId),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.wadsToSlash),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.description,
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
        impl alloy_sol_types::SolType for SlashingParams {
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
        impl alloy_sol_types::SolStruct for SlashingParams {
            const NAME: &'static str = "SlashingParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "SlashingParams(address operator,uint32 operatorSetId,address[] strategies,uint256[] wadsToSlash,string description)",
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.operator,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.operatorSetId)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.strategies)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.wadsToSlash)
                        .0,
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.description,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for SlashingParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operator,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operatorSetId,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategies,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.wadsToSlash,
                    )
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.description,
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
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operator,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operatorSetId,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Address,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.strategies,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<256>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.wadsToSlash,
                    out,
                );
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.description,
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
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IAllocationManagerTypes`](self) contract instance.

See the [wrapper's documentation](`IAllocationManagerTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IAllocationManagerTypesInstance<T, P, N> {
        IAllocationManagerTypesInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IAllocationManagerTypes`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IAllocationManagerTypes`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IAllocationManagerTypesInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IAllocationManagerTypesInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IAllocationManagerTypesInstance")
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
    > IAllocationManagerTypesInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IAllocationManagerTypes`](self) contract instance.

See the [wrapper's documentation](`IAllocationManagerTypesInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IAllocationManagerTypesInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IAllocationManagerTypesInstance<T, P, N> {
            IAllocationManagerTypesInstance {
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
    > IAllocationManagerTypesInstance<T, P, N> {
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
    > IAllocationManagerTypesInstance<T, P, N> {
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
library IAllocationManagerTypes {
    struct SlashingParams {
        address operator;
        uint32 operatorSetId;
        address[] strategies;
        uint256[] wadsToSlash;
        string description;
    }
}

interface InstantSlasher {
    error OnlySlasher();

    event OperatorSlashed(uint256 indexed slashingRequestId, address indexed operator, uint32 indexed operatorSetId, uint256[] wadsToSlash, string description);

    constructor(address _allocationManager, address _slashingRegistryCoordinator, address _slasher);

    function allocationManager() external view returns (address);
    function fulfillSlashingRequest(IAllocationManagerTypes.SlashingParams memory _slashingParams) external;
    function nextRequestId() external view returns (uint256);
    function slasher() external view returns (address);
    function slashingRegistryCoordinator() external view returns (address);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_allocationManager",
        "type": "address",
        "internalType": "contract IAllocationManager"
      },
      {
        "name": "_slashingRegistryCoordinator",
        "type": "address",
        "internalType": "contract ISlashingRegistryCoordinator"
      },
      {
        "name": "_slasher",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "allocationManager",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IAllocationManager"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "fulfillSlashingRequest",
    "inputs": [
      {
        "name": "_slashingParams",
        "type": "tuple",
        "internalType": "struct IAllocationManagerTypes.SlashingParams",
        "components": [
          {
            "name": "operator",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "operatorSetId",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          },
          {
            "name": "wadsToSlash",
            "type": "uint256[]",
            "internalType": "uint256[]"
          },
          {
            "name": "description",
            "type": "string",
            "internalType": "string"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "nextRequestId",
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
    "name": "slasher",
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
    "name": "slashingRegistryCoordinator",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract ISlashingRegistryCoordinator"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "event",
    "name": "OperatorSlashed",
    "inputs": [
      {
        "name": "slashingRequestId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "operatorSetId",
        "type": "uint32",
        "indexed": true,
        "internalType": "uint32"
      },
      {
        "name": "wadsToSlash",
        "type": "uint256[]",
        "indexed": false,
        "internalType": "uint256[]"
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
    "type": "error",
    "name": "OnlySlasher",
    "inputs": []
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
pub mod InstantSlasher {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60e060405234801561000f575f5ffd5b506040516109fd3803806109fd83398101604081905261002e91610062565b6001600160a01b0392831660805290821660a0521660c0526100ac565b6001600160a01b038116811461005f575f5ffd5b50565b5f5f5f60608486031215610074575f5ffd5b835161007f8161004b565b60208501519093506100908161004b565b60408501519092506100a18161004b565b809150509250925092565b60805160a05160c05161090e6100ef5f395f8181608e015261021301525f818160f4015281816101a9015261029701525f818160cd0152610268015261090e5ff3fe608060405234801561000f575f5ffd5b5060043610610055575f3560e01c80636a669b41146100595780636a84a9851461006e578063b134427114610089578063ca8aa7c7146100c8578063cf1d6b42146100ef575b5f5ffd5b61006c6100673660046103be565b610116565b005b6100765f5481565b6040519081526020015b60405180910390f35b6100b07f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610080565b6100b07f000000000000000000000000000000000000000000000000000000000000000081565b6100b07f000000000000000000000000000000000000000000000000000000000000000081565b61011f33610211565b5f8054818061012d836103fb565b909155509050610145816101408461061d565b610266565b6040805160018082528183019092525f916020808301908036833701905050905061017360208401846106d5565b815f81518110610185576101856106f0565b6001600160a01b03928316602091820292909201015260405162cf2ab560e01b81527f00000000000000000000000000000000000000000000000000000000000000009091169062cf2ab5906101df908490600401610704565b5f604051808303815f87803b1580156101f6575f5ffd5b505af1158015610208573d5f5f3e3d5ffd5b50505050505050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316816001600160a01b03161461026357604051637e57b1e160e01b815260040160405180910390fd5b50565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663363520577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663de1164bb6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156102f1573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610315919061074f565b836040518363ffffffff1660e01b81526004016103339291906107e7565b5f604051808303815f87803b15801561034a575f5ffd5b505af115801561035c573d5f5f3e3d5ffd5b50505050806020015163ffffffff16815f01516001600160a01b0316837f8a83cf9afb09a981314f4fb353b95b003451da170a99f48d8db6474b06d79f3b846060015185608001516040516103b29291906108ab565b60405180910390a45050565b5f602082840312156103ce575f5ffd5b81356001600160401b038111156103e3575f5ffd5b820160a081850312156103f4575f5ffd5b9392505050565b5f6001820161041857634e487b7160e01b5f52601160045260245ffd5b5060010190565b634e487b7160e01b5f52604160045260245ffd5b60405160a081016001600160401b03811182821017156104555761045561041f565b60405290565b604051601f8201601f191681016001600160401b03811182821017156104835761048361041f565b604052919050565b6001600160a01b0381168114610263575f5ffd5b80356104aa8161048b565b919050565b803563ffffffff811681146104aa575f5ffd5b5f6001600160401b038211156104da576104da61041f565b5060051b60200190565b5f82601f8301126104f3575f5ffd5b8135610506610501826104c2565b61045b565b8082825260208201915060208360051b860101925085831115610527575f5ffd5b602085015b8381101561054d57803561053f8161048b565b83526020928301920161052c565b5095945050505050565b5f82601f830112610566575f5ffd5b8135610574610501826104c2565b8082825260208201915060208360051b860101925085831115610595575f5ffd5b602085015b8381101561054d57803583526020928301920161059a565b5f82601f8301126105c1575f5ffd5b81356001600160401b038111156105da576105da61041f565b6105ed601f8201601f191660200161045b565b818152846020838601011115610601575f5ffd5b816020850160208301375f918101602001919091529392505050565b5f60a0823603121561062d575f5ffd5b610635610433565b61063e8361049f565b815261064c602084016104af565b602082015260408301356001600160401b03811115610669575f5ffd5b610675368286016104e4565b60408301525060608301356001600160401b03811115610693575f5ffd5b61069f36828601610557565b60608301525060808301356001600160401b038111156106bd575f5ffd5b6106c9368286016105b2565b60808301525092915050565b5f602082840312156106e5575f5ffd5b81356103f48161048b565b634e487b7160e01b5f52603260045260245ffd5b602080825282518282018190525f918401906040840190835b818110156107445783516001600160a01b031683526020938401939092019160010161071d565b509095945050505050565b5f6020828403121561075f575f5ffd5b81516103f48161048b565b5f8151808452602084019350602083015f5b8281101561079a57815186526020958601959091019060010161077c565b5093949350505050565b5f81518084525f5b818110156107c8576020818501810151868301820152016107ac565b505f602082860101526020601f19601f83011685010191505092915050565b6001600160a01b038381168252604060208084018290528451909216818401528382015163ffffffff16606084015283015160a06080840152805160e084018190525f92919091019082906101008501905b808310156108645783516001600160a01b031682526020938401936001939093019290910190610839565b506060860151858203603f190160a08701529250610882818461076a565b925050506080840151603f198483030160c08501526108a182826107a4565b9695505050505050565b604081525f6108bd604083018561076a565b82810360208401526108cf81856107a4565b9594505050505056fea26469706673582212209f99a504fbd442db2c7b564806808845b01297403e36e6ea2901566da49c8b8e64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xE0`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa\t\xFD8\x03\x80a\t\xFD\x839\x81\x01`@\x81\x90Ra\0.\x91a\0bV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x80R\x90\x82\x16`\xA0R\x16`\xC0Ra\0\xACV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0_W__\xFD[PV[___``\x84\x86\x03\x12\x15a\0tW__\xFD[\x83Qa\0\x7F\x81a\0KV[` \x85\x01Q\x90\x93Pa\0\x90\x81a\0KV[`@\x85\x01Q\x90\x92Pa\0\xA1\x81a\0KV[\x80\x91PP\x92P\x92P\x92V[`\x80Q`\xA0Q`\xC0Qa\t\x0Ea\0\xEF_9_\x81\x81`\x8E\x01Ra\x02\x13\x01R_\x81\x81`\xF4\x01R\x81\x81a\x01\xA9\x01Ra\x02\x97\x01R_\x81\x81`\xCD\x01Ra\x02h\x01Ra\t\x0E_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0UW_5`\xE0\x1C\x80cjf\x9BA\x14a\0YW\x80cj\x84\xA9\x85\x14a\0nW\x80c\xB14Bq\x14a\0\x89W\x80c\xCA\x8A\xA7\xC7\x14a\0\xC8W\x80c\xCF\x1DkB\x14a\0\xEFW[__\xFD[a\0la\0g6`\x04a\x03\xBEV[a\x01\x16V[\0[a\0v_T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xB0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\x80V[a\0\xB0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xB0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\x1F3a\x02\x11V[_\x80T\x81\x80a\x01-\x83a\x03\xFBV[\x90\x91UP\x90Pa\x01E\x81a\x01@\x84a\x06\x1DV[a\x02fV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90Pa\x01s` \x84\x01\x84a\x06\xD5V[\x81_\x81Q\x81\x10a\x01\x85Wa\x01\x85a\x06\xF0V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x01R`@Qb\xCF*\xB5`\xE0\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90b\xCF*\xB5\x90a\x01\xDF\x90\x84\x90`\x04\x01a\x07\x04V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x01\xF6W__\xFD[PZ\xF1\x15\x80\x15a\x02\x08W=__>=_\xFD[PPPPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a\x02cW`@Qc~W\xB1\xE1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c65 W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xDE\x11d\xBB`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xF1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x15\x91\x90a\x07OV[\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x033\x92\x91\x90a\x07\xE7V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x03JW__\xFD[PZ\xF1\x15\x80\x15a\x03\\W=__>=_\xFD[PPPP\x80` \x01Qc\xFF\xFF\xFF\xFF\x16\x81_\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F\x8A\x83\xCF\x9A\xFB\t\xA9\x811OO\xB3S\xB9[\x004Q\xDA\x17\n\x99\xF4\x8D\x8D\xB6GK\x06\xD7\x9F;\x84``\x01Q\x85`\x80\x01Q`@Qa\x03\xB2\x92\x91\x90a\x08\xABV[`@Q\x80\x91\x03\x90\xA4PPV[_` \x82\x84\x03\x12\x15a\x03\xCEW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x03\xE3W__\xFD[\x82\x01`\xA0\x81\x85\x03\x12\x15a\x03\xF4W__\xFD[\x93\x92PPPV[_`\x01\x82\x01a\x04\x18WcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[P`\x01\x01\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x04UWa\x04Ua\x04\x1FV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x04\x83Wa\x04\x83a\x04\x1FV[`@R\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02cW__\xFD[\x805a\x04\xAA\x81a\x04\x8BV[\x91\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x04\xAAW__\xFD[_`\x01`\x01`@\x1B\x03\x82\x11\x15a\x04\xDAWa\x04\xDAa\x04\x1FV[P`\x05\x1B` \x01\x90V[_\x82`\x1F\x83\x01\x12a\x04\xF3W__\xFD[\x815a\x05\x06a\x05\x01\x82a\x04\xC2V[a\x04[V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a\x05'W__\xFD[` \x85\x01[\x83\x81\x10\x15a\x05MW\x805a\x05?\x81a\x04\x8BV[\x83R` \x92\x83\x01\x92\x01a\x05,V[P\x95\x94PPPPPV[_\x82`\x1F\x83\x01\x12a\x05fW__\xFD[\x815a\x05ta\x05\x01\x82a\x04\xC2V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a\x05\x95W__\xFD[` \x85\x01[\x83\x81\x10\x15a\x05MW\x805\x83R` \x92\x83\x01\x92\x01a\x05\x9AV[_\x82`\x1F\x83\x01\x12a\x05\xC1W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x05\xDAWa\x05\xDAa\x04\x1FV[a\x05\xED`\x1F\x82\x01`\x1F\x19\x16` \x01a\x04[V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x06\x01W__\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_`\xA0\x826\x03\x12\x15a\x06-W__\xFD[a\x065a\x043V[a\x06>\x83a\x04\x9FV[\x81Ra\x06L` \x84\x01a\x04\xAFV[` \x82\x01R`@\x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06iW__\xFD[a\x06u6\x82\x86\x01a\x04\xE4V[`@\x83\x01RP``\x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06\x93W__\xFD[a\x06\x9F6\x82\x86\x01a\x05WV[``\x83\x01RP`\x80\x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06\xBDW__\xFD[a\x06\xC96\x82\x86\x01a\x05\xB2V[`\x80\x83\x01RP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x06\xE5W__\xFD[\x815a\x03\xF4\x81a\x04\x8BV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x07DW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x07\x1DV[P\x90\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a\x07_W__\xFD[\x81Qa\x03\xF4\x81a\x04\x8BV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a\x07\x9AW\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a\x07|V[P\x93\x94\x93PPPPV[_\x81Q\x80\x84R_[\x81\x81\x10\x15a\x07\xC8W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x07\xACV[P_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x82R`@` \x80\x84\x01\x82\x90R\x84Q\x90\x92\x16\x81\x84\x01R\x83\x82\x01Qc\xFF\xFF\xFF\xFF\x16``\x84\x01R\x83\x01Q`\xA0`\x80\x84\x01R\x80Q`\xE0\x84\x01\x81\x90R_\x92\x91\x90\x91\x01\x90\x82\x90a\x01\0\x85\x01\x90[\x80\x83\x10\x15a\x08dW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x93\x84\x01\x93`\x01\x93\x90\x93\x01\x92\x90\x91\x01\x90a\x089V[P``\x86\x01Q\x85\x82\x03`?\x19\x01`\xA0\x87\x01R\x92Pa\x08\x82\x81\x84a\x07jV[\x92PPP`\x80\x84\x01Q`?\x19\x84\x83\x03\x01`\xC0\x85\x01Ra\x08\xA1\x82\x82a\x07\xA4V[\x96\x95PPPPPPV[`@\x81R_a\x08\xBD`@\x83\x01\x85a\x07jV[\x82\x81\x03` \x84\x01Ra\x08\xCF\x81\x85a\x07\xA4V[\x95\x94PPPPPV\xFE\xA2dipfsX\"\x12 \x9F\x99\xA5\x04\xFB\xD4B\xDB,{VH\x06\x80\x88E\xB0\x12\x97@>6\xE6\xEA)\x01Vm\xA4\x9C\x8B\x8EdsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b5060043610610055575f3560e01c80636a669b41146100595780636a84a9851461006e578063b134427114610089578063ca8aa7c7146100c8578063cf1d6b42146100ef575b5f5ffd5b61006c6100673660046103be565b610116565b005b6100765f5481565b6040519081526020015b60405180910390f35b6100b07f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610080565b6100b07f000000000000000000000000000000000000000000000000000000000000000081565b6100b07f000000000000000000000000000000000000000000000000000000000000000081565b61011f33610211565b5f8054818061012d836103fb565b909155509050610145816101408461061d565b610266565b6040805160018082528183019092525f916020808301908036833701905050905061017360208401846106d5565b815f81518110610185576101856106f0565b6001600160a01b03928316602091820292909201015260405162cf2ab560e01b81527f00000000000000000000000000000000000000000000000000000000000000009091169062cf2ab5906101df908490600401610704565b5f604051808303815f87803b1580156101f6575f5ffd5b505af1158015610208573d5f5f3e3d5ffd5b50505050505050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316816001600160a01b03161461026357604051637e57b1e160e01b815260040160405180910390fd5b50565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663363520577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663de1164bb6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156102f1573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610315919061074f565b836040518363ffffffff1660e01b81526004016103339291906107e7565b5f604051808303815f87803b15801561034a575f5ffd5b505af115801561035c573d5f5f3e3d5ffd5b50505050806020015163ffffffff16815f01516001600160a01b0316837f8a83cf9afb09a981314f4fb353b95b003451da170a99f48d8db6474b06d79f3b846060015185608001516040516103b29291906108ab565b60405180910390a45050565b5f602082840312156103ce575f5ffd5b81356001600160401b038111156103e3575f5ffd5b820160a081850312156103f4575f5ffd5b9392505050565b5f6001820161041857634e487b7160e01b5f52601160045260245ffd5b5060010190565b634e487b7160e01b5f52604160045260245ffd5b60405160a081016001600160401b03811182821017156104555761045561041f565b60405290565b604051601f8201601f191681016001600160401b03811182821017156104835761048361041f565b604052919050565b6001600160a01b0381168114610263575f5ffd5b80356104aa8161048b565b919050565b803563ffffffff811681146104aa575f5ffd5b5f6001600160401b038211156104da576104da61041f565b5060051b60200190565b5f82601f8301126104f3575f5ffd5b8135610506610501826104c2565b61045b565b8082825260208201915060208360051b860101925085831115610527575f5ffd5b602085015b8381101561054d57803561053f8161048b565b83526020928301920161052c565b5095945050505050565b5f82601f830112610566575f5ffd5b8135610574610501826104c2565b8082825260208201915060208360051b860101925085831115610595575f5ffd5b602085015b8381101561054d57803583526020928301920161059a565b5f82601f8301126105c1575f5ffd5b81356001600160401b038111156105da576105da61041f565b6105ed601f8201601f191660200161045b565b818152846020838601011115610601575f5ffd5b816020850160208301375f918101602001919091529392505050565b5f60a0823603121561062d575f5ffd5b610635610433565b61063e8361049f565b815261064c602084016104af565b602082015260408301356001600160401b03811115610669575f5ffd5b610675368286016104e4565b60408301525060608301356001600160401b03811115610693575f5ffd5b61069f36828601610557565b60608301525060808301356001600160401b038111156106bd575f5ffd5b6106c9368286016105b2565b60808301525092915050565b5f602082840312156106e5575f5ffd5b81356103f48161048b565b634e487b7160e01b5f52603260045260245ffd5b602080825282518282018190525f918401906040840190835b818110156107445783516001600160a01b031683526020938401939092019160010161071d565b509095945050505050565b5f6020828403121561075f575f5ffd5b81516103f48161048b565b5f8151808452602084019350602083015f5b8281101561079a57815186526020958601959091019060010161077c565b5093949350505050565b5f81518084525f5b818110156107c8576020818501810151868301820152016107ac565b505f602082860101526020601f19601f83011685010191505092915050565b6001600160a01b038381168252604060208084018290528451909216818401528382015163ffffffff16606084015283015160a06080840152805160e084018190525f92919091019082906101008501905b808310156108645783516001600160a01b031682526020938401936001939093019290910190610839565b506060860151858203603f190160a08701529250610882818461076a565b925050506080840151603f198483030160c08501526108a182826107a4565b9695505050505050565b604081525f6108bd604083018561076a565b82810360208401526108cf81856107a4565b9594505050505056fea26469706673582212209f99a504fbd442db2c7b564806808845b01297403e36e6ea2901566da49c8b8e64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0UW_5`\xE0\x1C\x80cjf\x9BA\x14a\0YW\x80cj\x84\xA9\x85\x14a\0nW\x80c\xB14Bq\x14a\0\x89W\x80c\xCA\x8A\xA7\xC7\x14a\0\xC8W\x80c\xCF\x1DkB\x14a\0\xEFW[__\xFD[a\0la\0g6`\x04a\x03\xBEV[a\x01\x16V[\0[a\0v_T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xB0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\x80V[a\0\xB0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xB0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\x1F3a\x02\x11V[_\x80T\x81\x80a\x01-\x83a\x03\xFBV[\x90\x91UP\x90Pa\x01E\x81a\x01@\x84a\x06\x1DV[a\x02fV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90Pa\x01s` \x84\x01\x84a\x06\xD5V[\x81_\x81Q\x81\x10a\x01\x85Wa\x01\x85a\x06\xF0V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x01R`@Qb\xCF*\xB5`\xE0\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90b\xCF*\xB5\x90a\x01\xDF\x90\x84\x90`\x04\x01a\x07\x04V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x01\xF6W__\xFD[PZ\xF1\x15\x80\x15a\x02\x08W=__>=_\xFD[PPPPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a\x02cW`@Qc~W\xB1\xE1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c65 W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xDE\x11d\xBB`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xF1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x15\x91\x90a\x07OV[\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x033\x92\x91\x90a\x07\xE7V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x03JW__\xFD[PZ\xF1\x15\x80\x15a\x03\\W=__>=_\xFD[PPPP\x80` \x01Qc\xFF\xFF\xFF\xFF\x16\x81_\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F\x8A\x83\xCF\x9A\xFB\t\xA9\x811OO\xB3S\xB9[\x004Q\xDA\x17\n\x99\xF4\x8D\x8D\xB6GK\x06\xD7\x9F;\x84``\x01Q\x85`\x80\x01Q`@Qa\x03\xB2\x92\x91\x90a\x08\xABV[`@Q\x80\x91\x03\x90\xA4PPV[_` \x82\x84\x03\x12\x15a\x03\xCEW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x03\xE3W__\xFD[\x82\x01`\xA0\x81\x85\x03\x12\x15a\x03\xF4W__\xFD[\x93\x92PPPV[_`\x01\x82\x01a\x04\x18WcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[P`\x01\x01\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x04UWa\x04Ua\x04\x1FV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x04\x83Wa\x04\x83a\x04\x1FV[`@R\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02cW__\xFD[\x805a\x04\xAA\x81a\x04\x8BV[\x91\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x04\xAAW__\xFD[_`\x01`\x01`@\x1B\x03\x82\x11\x15a\x04\xDAWa\x04\xDAa\x04\x1FV[P`\x05\x1B` \x01\x90V[_\x82`\x1F\x83\x01\x12a\x04\xF3W__\xFD[\x815a\x05\x06a\x05\x01\x82a\x04\xC2V[a\x04[V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a\x05'W__\xFD[` \x85\x01[\x83\x81\x10\x15a\x05MW\x805a\x05?\x81a\x04\x8BV[\x83R` \x92\x83\x01\x92\x01a\x05,V[P\x95\x94PPPPPV[_\x82`\x1F\x83\x01\x12a\x05fW__\xFD[\x815a\x05ta\x05\x01\x82a\x04\xC2V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a\x05\x95W__\xFD[` \x85\x01[\x83\x81\x10\x15a\x05MW\x805\x83R` \x92\x83\x01\x92\x01a\x05\x9AV[_\x82`\x1F\x83\x01\x12a\x05\xC1W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x05\xDAWa\x05\xDAa\x04\x1FV[a\x05\xED`\x1F\x82\x01`\x1F\x19\x16` \x01a\x04[V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x06\x01W__\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_`\xA0\x826\x03\x12\x15a\x06-W__\xFD[a\x065a\x043V[a\x06>\x83a\x04\x9FV[\x81Ra\x06L` \x84\x01a\x04\xAFV[` \x82\x01R`@\x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06iW__\xFD[a\x06u6\x82\x86\x01a\x04\xE4V[`@\x83\x01RP``\x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06\x93W__\xFD[a\x06\x9F6\x82\x86\x01a\x05WV[``\x83\x01RP`\x80\x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06\xBDW__\xFD[a\x06\xC96\x82\x86\x01a\x05\xB2V[`\x80\x83\x01RP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x06\xE5W__\xFD[\x815a\x03\xF4\x81a\x04\x8BV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x07DW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x07\x1DV[P\x90\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a\x07_W__\xFD[\x81Qa\x03\xF4\x81a\x04\x8BV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a\x07\x9AW\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a\x07|V[P\x93\x94\x93PPPPV[_\x81Q\x80\x84R_[\x81\x81\x10\x15a\x07\xC8W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x07\xACV[P_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x82R`@` \x80\x84\x01\x82\x90R\x84Q\x90\x92\x16\x81\x84\x01R\x83\x82\x01Qc\xFF\xFF\xFF\xFF\x16``\x84\x01R\x83\x01Q`\xA0`\x80\x84\x01R\x80Q`\xE0\x84\x01\x81\x90R_\x92\x91\x90\x91\x01\x90\x82\x90a\x01\0\x85\x01\x90[\x80\x83\x10\x15a\x08dW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x93\x84\x01\x93`\x01\x93\x90\x93\x01\x92\x90\x91\x01\x90a\x089V[P``\x86\x01Q\x85\x82\x03`?\x19\x01`\xA0\x87\x01R\x92Pa\x08\x82\x81\x84a\x07jV[\x92PPP`\x80\x84\x01Q`?\x19\x84\x83\x03\x01`\xC0\x85\x01Ra\x08\xA1\x82\x82a\x07\xA4V[\x96\x95PPPPPPV[`@\x81R_a\x08\xBD`@\x83\x01\x85a\x07jV[\x82\x81\x03` \x84\x01Ra\x08\xCF\x81\x85a\x07\xA4V[\x95\x94PPPPPV\xFE\xA2dipfsX\"\x12 \x9F\x99\xA5\x04\xFB\xD4B\xDB,{VH\x06\x80\x88E\xB0\x12\x97@>6\xE6\xEA)\x01Vm\xA4\x9C\x8B\x8EdsolcC\0\x08\x1B\x003",
    );
    /**Custom error with signature `OnlySlasher()` and selector `0x7e57b1e1`.
```solidity
error OnlySlasher();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlySlasher {}
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
        impl ::core::convert::From<OnlySlasher> for UnderlyingRustTuple<'_> {
            fn from(value: OnlySlasher) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlySlasher {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlySlasher {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlySlasher()";
            const SELECTOR: [u8; 4] = [126u8, 87u8, 177u8, 225u8];
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
    /**Event with signature `OperatorSlashed(uint256,address,uint32,uint256[],string)` and selector `0x8a83cf9afb09a981314f4fb353b95b003451da170a99f48d8db6474b06d79f3b`.
```solidity
event OperatorSlashed(uint256 indexed slashingRequestId, address indexed operator, uint32 indexed operatorSetId, uint256[] wadsToSlash, string description);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorSlashed {
        #[allow(missing_docs)]
        pub slashingRequestId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSetId: u32,
        #[allow(missing_docs)]
        pub wadsToSlash: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
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
        impl alloy_sol_types::SolEvent for OperatorSlashed {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::String,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            const SIGNATURE: &'static str = "OperatorSlashed(uint256,address,uint32,uint256[],string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                138u8,
                131u8,
                207u8,
                154u8,
                251u8,
                9u8,
                169u8,
                129u8,
                49u8,
                79u8,
                79u8,
                179u8,
                83u8,
                185u8,
                91u8,
                0u8,
                52u8,
                81u8,
                218u8,
                23u8,
                10u8,
                153u8,
                244u8,
                141u8,
                141u8,
                182u8,
                71u8,
                75u8,
                6u8,
                215u8,
                159u8,
                59u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    slashingRequestId: topics.1,
                    operator: topics.2,
                    operatorSetId: topics.3,
                    wadsToSlash: data.0,
                    description: data.1,
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.wadsToSlash),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.description,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.slashingRequestId.clone(),
                    self.operator.clone(),
                    self.operatorSetId.clone(),
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
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.slashingRequestId);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                out[3usize] = <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.operatorSetId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorSlashed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorSlashed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorSlashed) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address _allocationManager, address _slashingRegistryCoordinator, address _slasher);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _allocationManager: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _slashingRegistryCoordinator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _slasher: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (
                        value._allocationManager,
                        value._slashingRegistryCoordinator,
                        value._slasher,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _allocationManager: tuple.0,
                        _slashingRegistryCoordinator: tuple.1,
                        _slasher: tuple.2,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
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
                        &self._allocationManager,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._slashingRegistryCoordinator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._slasher,
                    ),
                )
            }
        }
    };
    /**Function with signature `allocationManager()` and selector `0xca8aa7c7`.
```solidity
function allocationManager() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allocationManagerCall {}
    ///Container type for the return parameters of the [`allocationManager()`](allocationManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allocationManagerReturn {
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
            impl ::core::convert::From<allocationManagerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: allocationManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for allocationManagerCall {
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
            impl ::core::convert::From<allocationManagerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: allocationManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for allocationManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for allocationManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = allocationManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "allocationManager()";
            const SELECTOR: [u8; 4] = [202u8, 138u8, 167u8, 199u8];
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
    /**Function with signature `fulfillSlashingRequest((address,uint32,address[],uint256[],string))` and selector `0x6a669b41`.
```solidity
function fulfillSlashingRequest(IAllocationManagerTypes.SlashingParams memory _slashingParams) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct fulfillSlashingRequestCall {
        #[allow(missing_docs)]
        pub _slashingParams: <IAllocationManagerTypes::SlashingParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`fulfillSlashingRequest((address,uint32,address[],uint256[],string))`](fulfillSlashingRequestCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct fulfillSlashingRequestReturn {}
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
            type UnderlyingSolTuple<'a> = (IAllocationManagerTypes::SlashingParams,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IAllocationManagerTypes::SlashingParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<fulfillSlashingRequestCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: fulfillSlashingRequestCall) -> Self {
                    (value._slashingParams,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for fulfillSlashingRequestCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _slashingParams: tuple.0 }
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
            impl ::core::convert::From<fulfillSlashingRequestReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: fulfillSlashingRequestReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for fulfillSlashingRequestReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for fulfillSlashingRequestCall {
            type Parameters<'a> = (IAllocationManagerTypes::SlashingParams,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = fulfillSlashingRequestReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "fulfillSlashingRequest((address,uint32,address[],uint256[],string))";
            const SELECTOR: [u8; 4] = [106u8, 102u8, 155u8, 65u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IAllocationManagerTypes::SlashingParams as alloy_sol_types::SolType>::tokenize(
                        &self._slashingParams,
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
    /**Function with signature `nextRequestId()` and selector `0x6a84a985`.
```solidity
function nextRequestId() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nextRequestIdCall {}
    ///Container type for the return parameters of the [`nextRequestId()`](nextRequestIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nextRequestIdReturn {
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
            impl ::core::convert::From<nextRequestIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: nextRequestIdCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nextRequestIdCall {
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
            impl ::core::convert::From<nextRequestIdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: nextRequestIdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nextRequestIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for nextRequestIdCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = nextRequestIdReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "nextRequestId()";
            const SELECTOR: [u8; 4] = [106u8, 132u8, 169u8, 133u8];
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
    /**Function with signature `slasher()` and selector `0xb1344271`.
```solidity
function slasher() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slasherCall {}
    ///Container type for the return parameters of the [`slasher()`](slasherCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slasherReturn {
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
            impl ::core::convert::From<slasherCall> for UnderlyingRustTuple<'_> {
                fn from(value: slasherCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for slasherCall {
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
            impl ::core::convert::From<slasherReturn> for UnderlyingRustTuple<'_> {
                fn from(value: slasherReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for slasherReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for slasherCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = slasherReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "slasher()";
            const SELECTOR: [u8; 4] = [177u8, 52u8, 66u8, 113u8];
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
    /**Function with signature `slashingRegistryCoordinator()` and selector `0xcf1d6b42`.
```solidity
function slashingRegistryCoordinator() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashingRegistryCoordinatorCall {}
    ///Container type for the return parameters of the [`slashingRegistryCoordinator()`](slashingRegistryCoordinatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashingRegistryCoordinatorReturn {
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
            impl ::core::convert::From<slashingRegistryCoordinatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: slashingRegistryCoordinatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for slashingRegistryCoordinatorCall {
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
            impl ::core::convert::From<slashingRegistryCoordinatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: slashingRegistryCoordinatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for slashingRegistryCoordinatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for slashingRegistryCoordinatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = slashingRegistryCoordinatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "slashingRegistryCoordinator()";
            const SELECTOR: [u8; 4] = [207u8, 29u8, 107u8, 66u8];
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
    ///Container for all the [`InstantSlasher`](self) function calls.
    pub enum InstantSlasherCalls {
        #[allow(missing_docs)]
        allocationManager(allocationManagerCall),
        #[allow(missing_docs)]
        fulfillSlashingRequest(fulfillSlashingRequestCall),
        #[allow(missing_docs)]
        nextRequestId(nextRequestIdCall),
        #[allow(missing_docs)]
        slasher(slasherCall),
        #[allow(missing_docs)]
        slashingRegistryCoordinator(slashingRegistryCoordinatorCall),
    }
    #[automatically_derived]
    impl InstantSlasherCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [106u8, 102u8, 155u8, 65u8],
            [106u8, 132u8, 169u8, 133u8],
            [177u8, 52u8, 66u8, 113u8],
            [202u8, 138u8, 167u8, 199u8],
            [207u8, 29u8, 107u8, 66u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for InstantSlasherCalls {
        const NAME: &'static str = "InstantSlasherCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 5usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::allocationManager(_) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::fulfillSlashingRequest(_) => {
                    <fulfillSlashingRequestCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::nextRequestId(_) => {
                    <nextRequestIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::slasher(_) => <slasherCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::slashingRegistryCoordinator(_) => {
                    <slashingRegistryCoordinatorCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<InstantSlasherCalls>] = &[
                {
                    fn fulfillSlashingRequest(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<InstantSlasherCalls> {
                        <fulfillSlashingRequestCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(InstantSlasherCalls::fulfillSlashingRequest)
                    }
                    fulfillSlashingRequest
                },
                {
                    fn nextRequestId(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<InstantSlasherCalls> {
                        <nextRequestIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(InstantSlasherCalls::nextRequestId)
                    }
                    nextRequestId
                },
                {
                    fn slasher(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<InstantSlasherCalls> {
                        <slasherCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(InstantSlasherCalls::slasher)
                    }
                    slasher
                },
                {
                    fn allocationManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<InstantSlasherCalls> {
                        <allocationManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(InstantSlasherCalls::allocationManager)
                    }
                    allocationManager
                },
                {
                    fn slashingRegistryCoordinator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<InstantSlasherCalls> {
                        <slashingRegistryCoordinatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(InstantSlasherCalls::slashingRegistryCoordinator)
                    }
                    slashingRegistryCoordinator
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
                Self::allocationManager(inner) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::fulfillSlashingRequest(inner) => {
                    <fulfillSlashingRequestCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::nextRequestId(inner) => {
                    <nextRequestIdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::slasher(inner) => {
                    <slasherCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::slashingRegistryCoordinator(inner) => {
                    <slashingRegistryCoordinatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::allocationManager(inner) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::fulfillSlashingRequest(inner) => {
                    <fulfillSlashingRequestCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::nextRequestId(inner) => {
                    <nextRequestIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::slasher(inner) => {
                    <slasherCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::slashingRegistryCoordinator(inner) => {
                    <slashingRegistryCoordinatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`InstantSlasher`](self) custom errors.
    pub enum InstantSlasherErrors {
        #[allow(missing_docs)]
        OnlySlasher(OnlySlasher),
    }
    #[automatically_derived]
    impl InstantSlasherErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[[126u8, 87u8, 177u8, 225u8]];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for InstantSlasherErrors {
        const NAME: &'static str = "InstantSlasherErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 1usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::OnlySlasher(_) => {
                    <OnlySlasher as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<InstantSlasherErrors>] = &[
                {
                    fn OnlySlasher(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<InstantSlasherErrors> {
                        <OnlySlasher as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(InstantSlasherErrors::OnlySlasher)
                    }
                    OnlySlasher
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
                Self::OnlySlasher(inner) => {
                    <OnlySlasher as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::OnlySlasher(inner) => {
                    <OnlySlasher as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`InstantSlasher`](self) events.
    pub enum InstantSlasherEvents {
        #[allow(missing_docs)]
        OperatorSlashed(OperatorSlashed),
    }
    #[automatically_derived]
    impl InstantSlasherEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                138u8,
                131u8,
                207u8,
                154u8,
                251u8,
                9u8,
                169u8,
                129u8,
                49u8,
                79u8,
                79u8,
                179u8,
                83u8,
                185u8,
                91u8,
                0u8,
                52u8,
                81u8,
                218u8,
                23u8,
                10u8,
                153u8,
                244u8,
                141u8,
                141u8,
                182u8,
                71u8,
                75u8,
                6u8,
                215u8,
                159u8,
                59u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for InstantSlasherEvents {
        const NAME: &'static str = "InstantSlasherEvents";
        const COUNT: usize = 1usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<OperatorSlashed as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorSlashed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorSlashed)
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
    impl alloy_sol_types::private::IntoLogData for InstantSlasherEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::OperatorSlashed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::OperatorSlashed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`InstantSlasher`](self) contract instance.

See the [wrapper's documentation](`InstantSlasherInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> InstantSlasherInstance<T, P, N> {
        InstantSlasherInstance::<T, P, N>::new(address, provider)
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
        _allocationManager: alloy::sol_types::private::Address,
        _slashingRegistryCoordinator: alloy::sol_types::private::Address,
        _slasher: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<InstantSlasherInstance<T, P, N>>,
    > {
        InstantSlasherInstance::<
            T,
            P,
            N,
        >::deploy(provider, _allocationManager, _slashingRegistryCoordinator, _slasher)
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
        _allocationManager: alloy::sol_types::private::Address,
        _slashingRegistryCoordinator: alloy::sol_types::private::Address,
        _slasher: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        InstantSlasherInstance::<
            T,
            P,
            N,
        >::deploy_builder(
            provider,
            _allocationManager,
            _slashingRegistryCoordinator,
            _slasher,
        )
    }
    /**A [`InstantSlasher`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`InstantSlasher`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct InstantSlasherInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for InstantSlasherInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("InstantSlasherInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > InstantSlasherInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`InstantSlasher`](self) contract instance.

See the [wrapper's documentation](`InstantSlasherInstance`) for more details.*/
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
            _allocationManager: alloy::sol_types::private::Address,
            _slashingRegistryCoordinator: alloy::sol_types::private::Address,
            _slasher: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<InstantSlasherInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _allocationManager,
                _slashingRegistryCoordinator,
                _slasher,
            );
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
            _allocationManager: alloy::sol_types::private::Address,
            _slashingRegistryCoordinator: alloy::sol_types::private::Address,
            _slasher: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _allocationManager,
                            _slashingRegistryCoordinator,
                            _slasher,
                        },
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
    impl<T, P: ::core::clone::Clone, N> InstantSlasherInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> InstantSlasherInstance<T, P, N> {
            InstantSlasherInstance {
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
    > InstantSlasherInstance<T, P, N> {
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
        ///Creates a new call builder for the [`allocationManager`] function.
        pub fn allocationManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, allocationManagerCall, N> {
            self.call_builder(&allocationManagerCall {})
        }
        ///Creates a new call builder for the [`fulfillSlashingRequest`] function.
        pub fn fulfillSlashingRequest(
            &self,
            _slashingParams: <IAllocationManagerTypes::SlashingParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, fulfillSlashingRequestCall, N> {
            self.call_builder(
                &fulfillSlashingRequestCall {
                    _slashingParams,
                },
            )
        }
        ///Creates a new call builder for the [`nextRequestId`] function.
        pub fn nextRequestId(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, nextRequestIdCall, N> {
            self.call_builder(&nextRequestIdCall {})
        }
        ///Creates a new call builder for the [`slasher`] function.
        pub fn slasher(&self) -> alloy_contract::SolCallBuilder<T, &P, slasherCall, N> {
            self.call_builder(&slasherCall {})
        }
        ///Creates a new call builder for the [`slashingRegistryCoordinator`] function.
        pub fn slashingRegistryCoordinator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, slashingRegistryCoordinatorCall, N> {
            self.call_builder(&slashingRegistryCoordinatorCall {})
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > InstantSlasherInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`OperatorSlashed`] event.
        pub fn OperatorSlashed_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorSlashed, N> {
            self.event_filter::<OperatorSlashed>()
        }
    }
}

