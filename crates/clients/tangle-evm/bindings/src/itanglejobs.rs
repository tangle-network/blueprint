///Module containing a contract's types and functions.
/**

```solidity
library Types {
    struct JobCall { uint8 jobIndex; address caller; uint64 createdAt; uint32 resultCount; uint256 payment; bool completed; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod Types {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct JobCall { uint8 jobIndex; address caller; uint64 createdAt; uint32 resultCount; uint256 payment; bool completed; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct JobCall {
        #[allow(missing_docs)]
        pub jobIndex: u8,
        #[allow(missing_docs)]
        pub caller: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub createdAt: u64,
        #[allow(missing_docs)]
        pub resultCount: u32,
        #[allow(missing_docs)]
        pub payment: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub completed: bool,
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
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Bool,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u8,
            alloy::sol_types::private::Address,
            u64,
            u32,
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
        impl ::core::convert::From<JobCall> for UnderlyingRustTuple<'_> {
            fn from(value: JobCall) -> Self {
                (
                    value.jobIndex,
                    value.caller,
                    value.createdAt,
                    value.resultCount,
                    value.payment,
                    value.completed,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for JobCall {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    jobIndex: tuple.0,
                    caller: tuple.1,
                    createdAt: tuple.2,
                    resultCount: tuple.3,
                    payment: tuple.4,
                    completed: tuple.5,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for JobCall {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for JobCall {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.jobIndex),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.caller,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.createdAt),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.resultCount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.payment),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.completed,
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
        impl alloy_sol_types::SolType for JobCall {
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
        impl alloy_sol_types::SolStruct for JobCall {
            const NAME: &'static str = "JobCall";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "JobCall(uint8 jobIndex,address caller,uint64 createdAt,uint32 resultCount,uint256 payment,bool completed)",
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
                        8,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.jobIndex)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.caller,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.createdAt)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.resultCount)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.payment)
                        .0,
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(
                            &self.completed,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for JobCall {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.jobIndex,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.caller,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.createdAt,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.resultCount,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.payment,
                    )
                    + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.completed,
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
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.jobIndex,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.caller,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.createdAt,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.resultCount,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.payment,
                    out,
                );
                <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.completed,
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
    /**Creates a new wrapper around an on-chain [`Types`](self) contract instance.

See the [wrapper's documentation](`TypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> TypesInstance<T, P, N> {
        TypesInstance::<T, P, N>::new(address, provider)
    }
    /**A [`Types`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`Types`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct TypesInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for TypesInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("TypesInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > TypesInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`Types`](self) contract instance.

See the [wrapper's documentation](`TypesInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> TypesInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> TypesInstance<T, P, N> {
            TypesInstance {
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
    > TypesInstance<T, P, N> {
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
    > TypesInstance<T, P, N> {
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
library Types {
    struct JobCall {
        uint8 jobIndex;
        address caller;
        uint64 createdAt;
        uint32 resultCount;
        uint256 payment;
        bool completed;
    }
}

interface ITangleJobs {
    event JobCompleted(uint64 indexed serviceId, uint64 indexed callId, uint32 resultCount);
    event JobResultSubmitted(uint64 indexed serviceId, uint64 indexed callId, address indexed operator, bytes result);
    event JobSubmitted(uint64 indexed serviceId, uint64 indexed callId, uint8 indexed jobIndex, address caller, bytes inputs);

    function getJobCall(uint64 serviceId, uint64 callId) external view returns (Types.JobCall memory);
    function submitAggregatedResult(uint64 serviceId, uint64 callId, bytes memory output, uint256 signerBitmap, uint256[2] memory aggregatedSignature, uint256[4] memory aggregatedPubkey) external;
    function submitJob(uint64 serviceId, uint8 jobIndex, bytes memory inputs) external payable returns (uint64 callId);
    function submitJobs(uint64 serviceId, uint8[] memory jobIndices, bytes[] memory inputs) external payable returns (uint64[] memory callIds);
    function submitResult(uint64 serviceId, uint64 callId, bytes memory result) external;
    function submitResults(uint64 serviceId, uint64[] memory callIds, bytes[] memory results) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "getJobCall",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "callId",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct Types.JobCall",
        "components": [
          {
            "name": "jobIndex",
            "type": "uint8",
            "internalType": "uint8"
          },
          {
            "name": "caller",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "createdAt",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "resultCount",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "payment",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "completed",
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
    "name": "submitAggregatedResult",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "callId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "output",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "signerBitmap",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "aggregatedSignature",
        "type": "uint256[2]",
        "internalType": "uint256[2]"
      },
      {
        "name": "aggregatedPubkey",
        "type": "uint256[4]",
        "internalType": "uint256[4]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "submitJob",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "jobIndex",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "inputs",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "callId",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "submitJobs",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "jobIndices",
        "type": "uint8[]",
        "internalType": "uint8[]"
      },
      {
        "name": "inputs",
        "type": "bytes[]",
        "internalType": "bytes[]"
      }
    ],
    "outputs": [
      {
        "name": "callIds",
        "type": "uint64[]",
        "internalType": "uint64[]"
      }
    ],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "submitResult",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "callId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "result",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "submitResults",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "callIds",
        "type": "uint64[]",
        "internalType": "uint64[]"
      },
      {
        "name": "results",
        "type": "bytes[]",
        "internalType": "bytes[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "JobCompleted",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "indexed": true,
        "internalType": "uint64"
      },
      {
        "name": "callId",
        "type": "uint64",
        "indexed": true,
        "internalType": "uint64"
      },
      {
        "name": "resultCount",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "JobResultSubmitted",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "indexed": true,
        "internalType": "uint64"
      },
      {
        "name": "callId",
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
        "name": "result",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "JobSubmitted",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "indexed": true,
        "internalType": "uint64"
      },
      {
        "name": "callId",
        "type": "uint64",
        "indexed": true,
        "internalType": "uint64"
      },
      {
        "name": "jobIndex",
        "type": "uint8",
        "indexed": true,
        "internalType": "uint8"
      },
      {
        "name": "caller",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "inputs",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
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
pub mod ITangleJobs {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"",
    );
    /**Event with signature `JobCompleted(uint64,uint64,uint32)` and selector `0xb5ffe15f61c0f965634ac694da1fbea788e328225b83f9a207152cba6c5a3c63`.
```solidity
event JobCompleted(uint64 indexed serviceId, uint64 indexed callId, uint32 resultCount);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct JobCompleted {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub callId: u64,
        #[allow(missing_docs)]
        pub resultCount: u32,
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
        impl alloy_sol_types::SolEvent for JobCompleted {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            const SIGNATURE: &'static str = "JobCompleted(uint64,uint64,uint32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                181u8,
                255u8,
                225u8,
                95u8,
                97u8,
                192u8,
                249u8,
                101u8,
                99u8,
                74u8,
                198u8,
                148u8,
                218u8,
                31u8,
                190u8,
                167u8,
                136u8,
                227u8,
                40u8,
                34u8,
                91u8,
                131u8,
                249u8,
                162u8,
                7u8,
                21u8,
                44u8,
                186u8,
                108u8,
                90u8,
                60u8,
                99u8,
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
                    callId: topics.2,
                    resultCount: data.0,
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
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.resultCount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.serviceId.clone(),
                    self.callId.clone(),
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.callId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for JobCompleted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&JobCompleted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &JobCompleted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `JobResultSubmitted(uint64,uint64,address,bytes)` and selector `0x8a04f99fe23284e874e8dd017345248279cc59701695e73bcdbeb6241bcd2ac6`.
```solidity
event JobResultSubmitted(uint64 indexed serviceId, uint64 indexed callId, address indexed operator, bytes result);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct JobResultSubmitted {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub callId: u64,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub result: alloy::sol_types::private::Bytes,
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
        impl alloy_sol_types::SolEvent for JobResultSubmitted {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "JobResultSubmitted(uint64,uint64,address,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                138u8,
                4u8,
                249u8,
                159u8,
                226u8,
                50u8,
                132u8,
                232u8,
                116u8,
                232u8,
                221u8,
                1u8,
                115u8,
                69u8,
                36u8,
                130u8,
                121u8,
                204u8,
                89u8,
                112u8,
                22u8,
                149u8,
                231u8,
                59u8,
                205u8,
                190u8,
                182u8,
                36u8,
                27u8,
                205u8,
                42u8,
                198u8,
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
                    callId: topics.2,
                    operator: topics.3,
                    result: data.0,
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.result,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.serviceId.clone(),
                    self.callId.clone(),
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.callId);
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for JobResultSubmitted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&JobResultSubmitted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &JobResultSubmitted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `JobSubmitted(uint64,uint64,uint8,address,bytes)` and selector `0xde37cc48d21778e1c9a075c4e41c5aff6918c3ea6151221f0af3ce8121a29db5`.
```solidity
event JobSubmitted(uint64 indexed serviceId, uint64 indexed callId, uint8 indexed jobIndex, address caller, bytes inputs);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct JobSubmitted {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub callId: u64,
        #[allow(missing_docs)]
        pub jobIndex: u8,
        #[allow(missing_docs)]
        pub caller: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub inputs: alloy::sol_types::private::Bytes,
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
        impl alloy_sol_types::SolEvent for JobSubmitted {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            const SIGNATURE: &'static str = "JobSubmitted(uint64,uint64,uint8,address,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                222u8,
                55u8,
                204u8,
                72u8,
                210u8,
                23u8,
                120u8,
                225u8,
                201u8,
                160u8,
                117u8,
                196u8,
                228u8,
                28u8,
                90u8,
                255u8,
                105u8,
                24u8,
                195u8,
                234u8,
                97u8,
                81u8,
                34u8,
                31u8,
                10u8,
                243u8,
                206u8,
                129u8,
                33u8,
                162u8,
                157u8,
                181u8,
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
                    callId: topics.2,
                    jobIndex: topics.3,
                    caller: data.0,
                    inputs: data.1,
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
                        &self.caller,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.inputs,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.serviceId.clone(),
                    self.callId.clone(),
                    self.jobIndex.clone(),
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.callId);
                out[3usize] = <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.jobIndex);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for JobSubmitted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&JobSubmitted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &JobSubmitted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Function with signature `getJobCall(uint64,uint64)` and selector `0xa672bc0a`.
```solidity
function getJobCall(uint64 serviceId, uint64 callId) external view returns (Types.JobCall memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getJobCallCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub callId: u64,
    }
    ///Container type for the return parameters of the [`getJobCall(uint64,uint64)`](getJobCallCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getJobCallReturn {
        #[allow(missing_docs)]
        pub _0: <Types::JobCall as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Uint<64>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64, u64);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getJobCallCall> for UnderlyingRustTuple<'_> {
                fn from(value: getJobCallCall) -> Self {
                    (value.serviceId, value.callId)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getJobCallCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        serviceId: tuple.0,
                        callId: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (Types::JobCall,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Types::JobCall as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getJobCallReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getJobCallReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getJobCallReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getJobCallCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getJobCallReturn;
            type ReturnTuple<'a> = (Types::JobCall,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getJobCall(uint64,uint64)";
            const SELECTOR: [u8; 4] = [166u8, 114u8, 188u8, 10u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.callId),
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
    /**Function with signature `submitAggregatedResult(uint64,uint64,bytes,uint256,uint256[2],uint256[4])` and selector `0xc325ae12`.
```solidity
function submitAggregatedResult(uint64 serviceId, uint64 callId, bytes memory output, uint256 signerBitmap, uint256[2] memory aggregatedSignature, uint256[4] memory aggregatedPubkey) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct submitAggregatedResultCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub callId: u64,
        #[allow(missing_docs)]
        pub output: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub signerBitmap: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub aggregatedSignature: [alloy::sol_types::private::primitives::aliases::U256; 2usize],
        #[allow(missing_docs)]
        pub aggregatedPubkey: [alloy::sol_types::private::primitives::aliases::U256; 4usize],
    }
    ///Container type for the return parameters of the [`submitAggregatedResult(uint64,uint64,bytes,uint256,uint256[2],uint256[4])`](submitAggregatedResultCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct submitAggregatedResultReturn {}
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
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::Uint<256>,
                    2usize,
                >,
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::Uint<256>,
                    4usize,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
                u64,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::primitives::aliases::U256,
                [alloy::sol_types::private::primitives::aliases::U256; 2usize],
                [alloy::sol_types::private::primitives::aliases::U256; 4usize],
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
            impl ::core::convert::From<submitAggregatedResultCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: submitAggregatedResultCall) -> Self {
                    (
                        value.serviceId,
                        value.callId,
                        value.output,
                        value.signerBitmap,
                        value.aggregatedSignature,
                        value.aggregatedPubkey,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for submitAggregatedResultCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        serviceId: tuple.0,
                        callId: tuple.1,
                        output: tuple.2,
                        signerBitmap: tuple.3,
                        aggregatedSignature: tuple.4,
                        aggregatedPubkey: tuple.5,
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
            impl ::core::convert::From<submitAggregatedResultReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: submitAggregatedResultReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for submitAggregatedResultReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for submitAggregatedResultCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::Uint<256>,
                    2usize,
                >,
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::Uint<256>,
                    4usize,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = submitAggregatedResultReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "submitAggregatedResult(uint64,uint64,bytes,uint256,uint256[2],uint256[4])";
            const SELECTOR: [u8; 4] = [195u8, 37u8, 174u8, 18u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.callId),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.output,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.signerBitmap),
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::SolType>::tokenize(&self.aggregatedSignature),
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        4usize,
                    > as alloy_sol_types::SolType>::tokenize(&self.aggregatedPubkey),
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
    /**Function with signature `submitJob(uint64,uint8,bytes)` and selector `0x3413e8ee`.
```solidity
function submitJob(uint64 serviceId, uint8 jobIndex, bytes memory inputs) external payable returns (uint64 callId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct submitJobCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub jobIndex: u8,
        #[allow(missing_docs)]
        pub inputs: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`submitJob(uint64,uint8,bytes)`](submitJobCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct submitJobReturn {
        #[allow(missing_docs)]
        pub callId: u64,
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
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64, u8, alloy::sol_types::private::Bytes);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<submitJobCall> for UnderlyingRustTuple<'_> {
                fn from(value: submitJobCall) -> Self {
                    (value.serviceId, value.jobIndex, value.inputs)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for submitJobCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        serviceId: tuple.0,
                        jobIndex: tuple.1,
                        inputs: tuple.2,
                    }
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
            impl ::core::convert::From<submitJobReturn> for UnderlyingRustTuple<'_> {
                fn from(value: submitJobReturn) -> Self {
                    (value.callId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for submitJobReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { callId: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for submitJobCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = submitJobReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "submitJob(uint64,uint8,bytes)";
            const SELECTOR: [u8; 4] = [52u8, 19u8, 232u8, 238u8];
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
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.jobIndex),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.inputs,
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
    /**Function with signature `submitJobs(uint64,uint8[],bytes[])` and selector `0x1a59f858`.
```solidity
function submitJobs(uint64 serviceId, uint8[] memory jobIndices, bytes[] memory inputs) external payable returns (uint64[] memory callIds);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct submitJobsCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub jobIndices: alloy::sol_types::private::Vec<u8>,
        #[allow(missing_docs)]
        pub inputs: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
    }
    ///Container type for the return parameters of the [`submitJobs(uint64,uint8[],bytes[])`](submitJobsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct submitJobsReturn {
        #[allow(missing_docs)]
        pub callIds: alloy::sol_types::private::Vec<u64>,
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<8>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
                alloy::sol_types::private::Vec<u8>,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
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
            impl ::core::convert::From<submitJobsCall> for UnderlyingRustTuple<'_> {
                fn from(value: submitJobsCall) -> Self {
                    (value.serviceId, value.jobIndices, value.inputs)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for submitJobsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        serviceId: tuple.0,
                        jobIndices: tuple.1,
                        inputs: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<64>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Vec<u64>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<submitJobsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: submitJobsReturn) -> Self {
                    (value.callIds,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for submitJobsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { callIds: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for submitJobsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<8>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = submitJobsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<64>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "submitJobs(uint64,uint8[],bytes[])";
            const SELECTOR: [u8; 4] = [26u8, 89u8, 248u8, 88u8];
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
                        alloy::sol_types::sol_data::Uint<8>,
                    > as alloy_sol_types::SolType>::tokenize(&self.jobIndices),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::SolType>::tokenize(&self.inputs),
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
    /**Function with signature `submitResult(uint64,uint64,bytes)` and selector `0x2d07e655`.
```solidity
function submitResult(uint64 serviceId, uint64 callId, bytes memory result) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct submitResultCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub callId: u64,
        #[allow(missing_docs)]
        pub result: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`submitResult(uint64,uint64,bytes)`](submitResultCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct submitResultReturn {}
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
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64, u64, alloy::sol_types::private::Bytes);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<submitResultCall> for UnderlyingRustTuple<'_> {
                fn from(value: submitResultCall) -> Self {
                    (value.serviceId, value.callId, value.result)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for submitResultCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        serviceId: tuple.0,
                        callId: tuple.1,
                        result: tuple.2,
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
            impl ::core::convert::From<submitResultReturn> for UnderlyingRustTuple<'_> {
                fn from(value: submitResultReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for submitResultReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for submitResultCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = submitResultReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "submitResult(uint64,uint64,bytes)";
            const SELECTOR: [u8; 4] = [45u8, 7u8, 230u8, 85u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.callId),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.result,
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
    /**Function with signature `submitResults(uint64,uint64[],bytes[])` and selector `0xaacdba9f`.
```solidity
function submitResults(uint64 serviceId, uint64[] memory callIds, bytes[] memory results) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct submitResultsCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub callIds: alloy::sol_types::private::Vec<u64>,
        #[allow(missing_docs)]
        pub results: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
    }
    ///Container type for the return parameters of the [`submitResults(uint64,uint64[],bytes[])`](submitResultsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct submitResultsReturn {}
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<64>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
                alloy::sol_types::private::Vec<u64>,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
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
            impl ::core::convert::From<submitResultsCall> for UnderlyingRustTuple<'_> {
                fn from(value: submitResultsCall) -> Self {
                    (value.serviceId, value.callIds, value.results)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for submitResultsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        serviceId: tuple.0,
                        callIds: tuple.1,
                        results: tuple.2,
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
            impl ::core::convert::From<submitResultsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: submitResultsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for submitResultsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for submitResultsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<64>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = submitResultsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "submitResults(uint64,uint64[],bytes[])";
            const SELECTOR: [u8; 4] = [170u8, 205u8, 186u8, 159u8];
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
                        alloy::sol_types::sol_data::Uint<64>,
                    > as alloy_sol_types::SolType>::tokenize(&self.callIds),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::SolType>::tokenize(&self.results),
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
    ///Container for all the [`ITangleJobs`](self) function calls.
    pub enum ITangleJobsCalls {
        #[allow(missing_docs)]
        getJobCall(getJobCallCall),
        #[allow(missing_docs)]
        submitAggregatedResult(submitAggregatedResultCall),
        #[allow(missing_docs)]
        submitJob(submitJobCall),
        #[allow(missing_docs)]
        submitJobs(submitJobsCall),
        #[allow(missing_docs)]
        submitResult(submitResultCall),
        #[allow(missing_docs)]
        submitResults(submitResultsCall),
    }
    #[automatically_derived]
    impl ITangleJobsCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [26u8, 89u8, 248u8, 88u8],
            [45u8, 7u8, 230u8, 85u8],
            [52u8, 19u8, 232u8, 238u8],
            [166u8, 114u8, 188u8, 10u8],
            [170u8, 205u8, 186u8, 159u8],
            [195u8, 37u8, 174u8, 18u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ITangleJobsCalls {
        const NAME: &'static str = "ITangleJobsCalls";
        const MIN_DATA_LENGTH: usize = 64usize;
        const COUNT: usize = 6usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::getJobCall(_) => {
                    <getJobCallCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::submitAggregatedResult(_) => {
                    <submitAggregatedResultCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::submitJob(_) => {
                    <submitJobCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::submitJobs(_) => {
                    <submitJobsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::submitResult(_) => {
                    <submitResultCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::submitResults(_) => {
                    <submitResultsCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<ITangleJobsCalls>] = &[
                {
                    fn submitJobs(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleJobsCalls> {
                        <submitJobsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleJobsCalls::submitJobs)
                    }
                    submitJobs
                },
                {
                    fn submitResult(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleJobsCalls> {
                        <submitResultCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleJobsCalls::submitResult)
                    }
                    submitResult
                },
                {
                    fn submitJob(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleJobsCalls> {
                        <submitJobCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleJobsCalls::submitJob)
                    }
                    submitJob
                },
                {
                    fn getJobCall(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleJobsCalls> {
                        <getJobCallCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleJobsCalls::getJobCall)
                    }
                    getJobCall
                },
                {
                    fn submitResults(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleJobsCalls> {
                        <submitResultsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleJobsCalls::submitResults)
                    }
                    submitResults
                },
                {
                    fn submitAggregatedResult(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ITangleJobsCalls> {
                        <submitAggregatedResultCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ITangleJobsCalls::submitAggregatedResult)
                    }
                    submitAggregatedResult
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
                Self::getJobCall(inner) => {
                    <getJobCallCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::submitAggregatedResult(inner) => {
                    <submitAggregatedResultCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::submitJob(inner) => {
                    <submitJobCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::submitJobs(inner) => {
                    <submitJobsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::submitResult(inner) => {
                    <submitResultCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::submitResults(inner) => {
                    <submitResultsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::getJobCall(inner) => {
                    <getJobCallCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::submitAggregatedResult(inner) => {
                    <submitAggregatedResultCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::submitJob(inner) => {
                    <submitJobCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::submitJobs(inner) => {
                    <submitJobsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::submitResult(inner) => {
                    <submitResultCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::submitResults(inner) => {
                    <submitResultsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`ITangleJobs`](self) events.
    pub enum ITangleJobsEvents {
        #[allow(missing_docs)]
        JobCompleted(JobCompleted),
        #[allow(missing_docs)]
        JobResultSubmitted(JobResultSubmitted),
        #[allow(missing_docs)]
        JobSubmitted(JobSubmitted),
    }
    #[automatically_derived]
    impl ITangleJobsEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                138u8,
                4u8,
                249u8,
                159u8,
                226u8,
                50u8,
                132u8,
                232u8,
                116u8,
                232u8,
                221u8,
                1u8,
                115u8,
                69u8,
                36u8,
                130u8,
                121u8,
                204u8,
                89u8,
                112u8,
                22u8,
                149u8,
                231u8,
                59u8,
                205u8,
                190u8,
                182u8,
                36u8,
                27u8,
                205u8,
                42u8,
                198u8,
            ],
            [
                181u8,
                255u8,
                225u8,
                95u8,
                97u8,
                192u8,
                249u8,
                101u8,
                99u8,
                74u8,
                198u8,
                148u8,
                218u8,
                31u8,
                190u8,
                167u8,
                136u8,
                227u8,
                40u8,
                34u8,
                91u8,
                131u8,
                249u8,
                162u8,
                7u8,
                21u8,
                44u8,
                186u8,
                108u8,
                90u8,
                60u8,
                99u8,
            ],
            [
                222u8,
                55u8,
                204u8,
                72u8,
                210u8,
                23u8,
                120u8,
                225u8,
                201u8,
                160u8,
                117u8,
                196u8,
                228u8,
                28u8,
                90u8,
                255u8,
                105u8,
                24u8,
                195u8,
                234u8,
                97u8,
                81u8,
                34u8,
                31u8,
                10u8,
                243u8,
                206u8,
                129u8,
                33u8,
                162u8,
                157u8,
                181u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for ITangleJobsEvents {
        const NAME: &'static str = "ITangleJobsEvents";
        const COUNT: usize = 3usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<JobCompleted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <JobCompleted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::JobCompleted)
                }
                Some(
                    <JobResultSubmitted as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <JobResultSubmitted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::JobResultSubmitted)
                }
                Some(<JobSubmitted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <JobSubmitted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::JobSubmitted)
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
    impl alloy_sol_types::private::IntoLogData for ITangleJobsEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::JobCompleted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::JobResultSubmitted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::JobSubmitted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::JobCompleted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::JobResultSubmitted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::JobSubmitted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ITangleJobs`](self) contract instance.

See the [wrapper's documentation](`ITangleJobsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ITangleJobsInstance<T, P, N> {
        ITangleJobsInstance::<T, P, N>::new(address, provider)
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
        Output = alloy_contract::Result<ITangleJobsInstance<T, P, N>>,
    > {
        ITangleJobsInstance::<T, P, N>::deploy(provider)
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
        ITangleJobsInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`ITangleJobs`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ITangleJobs`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ITangleJobsInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ITangleJobsInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ITangleJobsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ITangleJobsInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`ITangleJobs`](self) contract instance.

See the [wrapper's documentation](`ITangleJobsInstance`) for more details.*/
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
        ) -> alloy_contract::Result<ITangleJobsInstance<T, P, N>> {
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
    impl<T, P: ::core::clone::Clone, N> ITangleJobsInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ITangleJobsInstance<T, P, N> {
            ITangleJobsInstance {
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
    > ITangleJobsInstance<T, P, N> {
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
        ///Creates a new call builder for the [`getJobCall`] function.
        pub fn getJobCall(
            &self,
            serviceId: u64,
            callId: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, getJobCallCall, N> {
            self.call_builder(
                &getJobCallCall {
                    serviceId,
                    callId,
                },
            )
        }
        ///Creates a new call builder for the [`submitAggregatedResult`] function.
        pub fn submitAggregatedResult(
            &self,
            serviceId: u64,
            callId: u64,
            output: alloy::sol_types::private::Bytes,
            signerBitmap: alloy::sol_types::private::primitives::aliases::U256,
            aggregatedSignature: [alloy::sol_types::private::primitives::aliases::U256; 2usize],
            aggregatedPubkey: [alloy::sol_types::private::primitives::aliases::U256; 4usize],
        ) -> alloy_contract::SolCallBuilder<T, &P, submitAggregatedResultCall, N> {
            self.call_builder(
                &submitAggregatedResultCall {
                    serviceId,
                    callId,
                    output,
                    signerBitmap,
                    aggregatedSignature,
                    aggregatedPubkey,
                },
            )
        }
        ///Creates a new call builder for the [`submitJob`] function.
        pub fn submitJob(
            &self,
            serviceId: u64,
            jobIndex: u8,
            inputs: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, submitJobCall, N> {
            self.call_builder(
                &submitJobCall {
                    serviceId,
                    jobIndex,
                    inputs,
                },
            )
        }
        ///Creates a new call builder for the [`submitJobs`] function.
        pub fn submitJobs(
            &self,
            serviceId: u64,
            jobIndices: alloy::sol_types::private::Vec<u8>,
            inputs: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
        ) -> alloy_contract::SolCallBuilder<T, &P, submitJobsCall, N> {
            self.call_builder(
                &submitJobsCall {
                    serviceId,
                    jobIndices,
                    inputs,
                },
            )
        }
        ///Creates a new call builder for the [`submitResult`] function.
        pub fn submitResult(
            &self,
            serviceId: u64,
            callId: u64,
            result: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, submitResultCall, N> {
            self.call_builder(
                &submitResultCall {
                    serviceId,
                    callId,
                    result,
                },
            )
        }
        ///Creates a new call builder for the [`submitResults`] function.
        pub fn submitResults(
            &self,
            serviceId: u64,
            callIds: alloy::sol_types::private::Vec<u64>,
            results: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
        ) -> alloy_contract::SolCallBuilder<T, &P, submitResultsCall, N> {
            self.call_builder(
                &submitResultsCall {
                    serviceId,
                    callIds,
                    results,
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
    > ITangleJobsInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`JobCompleted`] event.
        pub fn JobCompleted_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, JobCompleted, N> {
            self.event_filter::<JobCompleted>()
        }
        ///Creates a new event filter for the [`JobResultSubmitted`] event.
        pub fn JobResultSubmitted_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, JobResultSubmitted, N> {
            self.event_filter::<JobResultSubmitted>()
        }
        ///Creates a new event filter for the [`JobSubmitted`] event.
        pub fn JobSubmitted_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, JobSubmitted, N> {
            self.event_filter::<JobSubmitted>()
        }
    }
}
