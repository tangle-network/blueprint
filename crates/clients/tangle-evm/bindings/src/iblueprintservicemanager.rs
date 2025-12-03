///Module containing a contract's types and functions.
/**

```solidity
library Assets {
    type Kind is uint8;
    struct Asset { Kind kind; bytes32 data; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod Assets {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Kind(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Kind> for u8 {
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
        impl Kind {
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
        impl alloy_sol_types::SolType for Kind {
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
        impl alloy_sol_types::EventTopic for Kind {
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
    /**```solidity
struct Asset { Kind kind; bytes32 data; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Asset {
        #[allow(missing_docs)]
        pub kind: <Kind as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::FixedBytes<32>,
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
        type UnderlyingSolTuple<'a> = (Kind, alloy::sol_types::sol_data::FixedBytes<32>);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <Kind as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<Asset> for UnderlyingRustTuple<'_> {
            fn from(value: Asset) -> Self {
                (value.kind, value.data)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Asset {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    kind: tuple.0,
                    data: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Asset {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Asset {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <Kind as alloy_sol_types::SolType>::tokenize(&self.kind),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.data),
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
        impl alloy_sol_types::SolType for Asset {
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
        impl alloy_sol_types::SolStruct for Asset {
            const NAME: &'static str = "Asset";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed("Asset(uint8 kind,bytes32 data)")
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
                    <Kind as alloy_sol_types::SolType>::eip712_data_word(&self.kind).0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.data)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Asset {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <Kind as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.kind,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.data)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <Kind as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.kind,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.data,
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
    /**Creates a new wrapper around an on-chain [`Assets`](self) contract instance.

See the [wrapper's documentation](`AssetsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> AssetsInstance<T, P, N> {
        AssetsInstance::<T, P, N>::new(address, provider)
    }
    /**A [`Assets`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`Assets`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct AssetsInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for AssetsInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("AssetsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > AssetsInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`Assets`](self) contract instance.

See the [wrapper's documentation](`AssetsInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> AssetsInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> AssetsInstance<T, P, N> {
            AssetsInstance {
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
    > AssetsInstance<T, P, N> {
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
    > AssetsInstance<T, P, N> {
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
///Module containing a contract's types and functions.
/**

```solidity
library ServiceOperators {
    struct OperatorPreferences { bytes ecdsaPublicKey; string rpcAddress; }
    struct RequestParams { uint64 requestId; address requester; OperatorPreferences[] operators; bytes requestInputs; address[] permittedCallers; uint64 ttl; Assets.Asset paymentAsset; uint256 amount; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod ServiceOperators {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct OperatorPreferences { bytes ecdsaPublicKey; string rpcAddress; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorPreferences {
        #[allow(missing_docs)]
        pub ecdsaPublicKey: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub rpcAddress: alloy::sol_types::private::String,
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
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::String,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Bytes,
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
        impl ::core::convert::From<OperatorPreferences> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorPreferences) -> Self {
                (value.ecdsaPublicKey, value.rpcAddress)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorPreferences {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    ecdsaPublicKey: tuple.0,
                    rpcAddress: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OperatorPreferences {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OperatorPreferences {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.ecdsaPublicKey,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.rpcAddress,
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
        impl alloy_sol_types::SolType for OperatorPreferences {
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
        impl alloy_sol_types::SolStruct for OperatorPreferences {
            const NAME: &'static str = "OperatorPreferences";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OperatorPreferences(bytes ecdsaPublicKey,string rpcAddress)",
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.ecdsaPublicKey,
                        )
                        .0,
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.rpcAddress,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OperatorPreferences {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.ecdsaPublicKey,
                    )
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.rpcAddress,
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
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.ecdsaPublicKey,
                    out,
                );
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.rpcAddress,
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
struct RequestParams { uint64 requestId; address requester; OperatorPreferences[] operators; bytes requestInputs; address[] permittedCallers; uint64 ttl; Assets.Asset paymentAsset; uint256 amount; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RequestParams {
        #[allow(missing_docs)]
        pub requestId: u64,
        #[allow(missing_docs)]
        pub requester: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operators: alloy::sol_types::private::Vec<
            <OperatorPreferences as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub requestInputs: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub permittedCallers: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub ttl: u64,
        #[allow(missing_docs)]
        pub paymentAsset: <Assets::Asset as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Array<OperatorPreferences>,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            alloy::sol_types::sol_data::Uint<64>,
            Assets::Asset,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u64,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Vec<
                <OperatorPreferences as alloy::sol_types::SolType>::RustType,
            >,
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            u64,
            <Assets::Asset as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<RequestParams> for UnderlyingRustTuple<'_> {
            fn from(value: RequestParams) -> Self {
                (
                    value.requestId,
                    value.requester,
                    value.operators,
                    value.requestInputs,
                    value.permittedCallers,
                    value.ttl,
                    value.paymentAsset,
                    value.amount,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RequestParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    requestId: tuple.0,
                    requester: tuple.1,
                    operators: tuple.2,
                    requestInputs: tuple.3,
                    permittedCallers: tuple.4,
                    ttl: tuple.5,
                    paymentAsset: tuple.6,
                    amount: tuple.7,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for RequestParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for RequestParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.requestId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.requester,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        OperatorPreferences,
                    > as alloy_sol_types::SolType>::tokenize(&self.operators),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.requestInputs,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.permittedCallers),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.ttl),
                    <Assets::Asset as alloy_sol_types::SolType>::tokenize(
                        &self.paymentAsset,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
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
        impl alloy_sol_types::SolType for RequestParams {
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
        impl alloy_sol_types::SolStruct for RequestParams {
            const NAME: &'static str = "RequestParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "RequestParams(uint64 requestId,address requester,OperatorPreferences[] operators,bytes requestInputs,address[] permittedCallers,uint64 ttl,Assets.Asset paymentAsset,uint256 amount)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(2);
                components
                    .push(
                        <OperatorPreferences as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <OperatorPreferences as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(
                        <Assets::Asset as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <Assets::Asset as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.requestId)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.requester,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        OperatorPreferences,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.operators)
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.requestInputs,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.permittedCallers,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.ttl)
                        .0,
                    <Assets::Asset as alloy_sol_types::SolType>::eip712_data_word(
                            &self.paymentAsset,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amount)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for RequestParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.requestId,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.requester,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        OperatorPreferences,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operators,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.requestInputs,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.permittedCallers,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.ttl)
                    + <Assets::Asset as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.paymentAsset,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amount,
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
                    &rust.requestId,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.requester,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    OperatorPreferences,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operators,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.requestInputs,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Address,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.permittedCallers,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.ttl, out);
                <Assets::Asset as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.paymentAsset,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amount,
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
    /**Creates a new wrapper around an on-chain [`ServiceOperators`](self) contract instance.

See the [wrapper's documentation](`ServiceOperatorsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ServiceOperatorsInstance<T, P, N> {
        ServiceOperatorsInstance::<T, P, N>::new(address, provider)
    }
    /**A [`ServiceOperators`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ServiceOperators`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ServiceOperatorsInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ServiceOperatorsInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ServiceOperatorsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ServiceOperatorsInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`ServiceOperators`](self) contract instance.

See the [wrapper's documentation](`ServiceOperatorsInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> ServiceOperatorsInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ServiceOperatorsInstance<T, P, N> {
            ServiceOperatorsInstance {
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
    > ServiceOperatorsInstance<T, P, N> {
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
    > ServiceOperatorsInstance<T, P, N> {
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
library Assets {
    type Kind is uint8;
    struct Asset {
        Kind kind;
        bytes32 data;
    }
}

library ServiceOperators {
    struct OperatorPreferences {
        bytes ecdsaPublicKey;
        string rpcAddress;
    }
    struct RequestParams {
        uint64 requestId;
        address requester;
        OperatorPreferences[] operators;
        bytes requestInputs;
        address[] permittedCallers;
        uint64 ttl;
        Assets.Asset paymentAsset;
        uint256 amount;
    }
}

interface IBlueprintServiceManager {
    function canJoin(uint64 serviceId, ServiceOperators.OperatorPreferences memory operator) external view returns (bool allowed);
    function canLeave(uint64 serviceId, ServiceOperators.OperatorPreferences memory operator) external view returns (bool allowed);
    function getHeartbeatInterval(uint64 serviceId) external view returns (bool useDefault, uint64 interval);
    function getHeartbeatThreshold(uint64 serviceId) external view returns (bool useDefault, uint8 threshold);
    function getSlashingWindow(uint64 serviceId) external view returns (bool useDefault, uint64 window);
    function onApprove(ServiceOperators.OperatorPreferences memory operator, uint64 requestId, uint8 restakingPercent) external payable;
    function onBlueprintCreated(uint64 blueprintId, address owner, address mbsm) external;
    function onJobCall(uint64 serviceId, uint8 job, uint64 jobCallId, bytes memory inputs) external payable;
    function onJobResult(uint64 serviceId, uint8 job, uint64 jobCallId, ServiceOperators.OperatorPreferences memory operator, bytes memory inputs, bytes memory outputs) external payable;
    function onOperatorJoined(uint64 serviceId, ServiceOperators.OperatorPreferences memory operator) external;
    function onOperatorLeft(uint64 serviceId, ServiceOperators.OperatorPreferences memory operator) external;
    function onRegister(ServiceOperators.OperatorPreferences memory operator, bytes memory registrationInputs) external payable;
    function onReject(ServiceOperators.OperatorPreferences memory operator, uint64 requestId) external;
    function onRequest(ServiceOperators.RequestParams memory params) external payable;
    function onServiceInitialized(uint64 requestId, uint64 serviceId, address owner, address[] memory permittedCallers, uint64 ttl) external;
    function onServiceTermination(uint64 serviceId, address owner) external;
    function onSlash(uint64 serviceId, bytes memory offender, uint8 slashPercent) external;
    function onUnappliedSlash(uint64 serviceId, bytes memory offender, uint8 slashPercent) external;
    function onUnregister(ServiceOperators.OperatorPreferences memory operator) external;
    function onUpdateRpcAddress(ServiceOperators.OperatorPreferences memory operator) external payable;
    function queryDeveloperPaymentAddress(uint64 serviceId) external view returns (address payable developerPaymentAddress);
    function queryDisputeOrigin(uint64 serviceId) external view returns (address disputeOrigin);
    function queryIsPaymentAssetAllowed(uint64 serviceId, Assets.Asset memory asset) external view returns (bool isAllowed);
    function querySlashingOrigin(uint64 serviceId) external view returns (address slashingOrigin);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "canJoin",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "operator",
        "type": "tuple",
        "internalType": "struct ServiceOperators.OperatorPreferences",
        "components": [
          {
            "name": "ecdsaPublicKey",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "rpcAddress",
            "type": "string",
            "internalType": "string"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "allowed",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "canLeave",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "operator",
        "type": "tuple",
        "internalType": "struct ServiceOperators.OperatorPreferences",
        "components": [
          {
            "name": "ecdsaPublicKey",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "rpcAddress",
            "type": "string",
            "internalType": "string"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "allowed",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getHeartbeatInterval",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [
      {
        "name": "useDefault",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "interval",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getHeartbeatThreshold",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [
      {
        "name": "useDefault",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "threshold",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getSlashingWindow",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [
      {
        "name": "useDefault",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "window",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "onApprove",
    "inputs": [
      {
        "name": "operator",
        "type": "tuple",
        "internalType": "struct ServiceOperators.OperatorPreferences",
        "components": [
          {
            "name": "ecdsaPublicKey",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "rpcAddress",
            "type": "string",
            "internalType": "string"
          }
        ]
      },
      {
        "name": "requestId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "restakingPercent",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "onBlueprintCreated",
    "inputs": [
      {
        "name": "blueprintId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "mbsm",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "onJobCall",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "job",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "jobCallId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "inputs",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "onJobResult",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "job",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "jobCallId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "operator",
        "type": "tuple",
        "internalType": "struct ServiceOperators.OperatorPreferences",
        "components": [
          {
            "name": "ecdsaPublicKey",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "rpcAddress",
            "type": "string",
            "internalType": "string"
          }
        ]
      },
      {
        "name": "inputs",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "outputs",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "onOperatorJoined",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "operator",
        "type": "tuple",
        "internalType": "struct ServiceOperators.OperatorPreferences",
        "components": [
          {
            "name": "ecdsaPublicKey",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "rpcAddress",
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
    "name": "onOperatorLeft",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "operator",
        "type": "tuple",
        "internalType": "struct ServiceOperators.OperatorPreferences",
        "components": [
          {
            "name": "ecdsaPublicKey",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "rpcAddress",
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
    "name": "onRegister",
    "inputs": [
      {
        "name": "operator",
        "type": "tuple",
        "internalType": "struct ServiceOperators.OperatorPreferences",
        "components": [
          {
            "name": "ecdsaPublicKey",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "rpcAddress",
            "type": "string",
            "internalType": "string"
          }
        ]
      },
      {
        "name": "registrationInputs",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "onReject",
    "inputs": [
      {
        "name": "operator",
        "type": "tuple",
        "internalType": "struct ServiceOperators.OperatorPreferences",
        "components": [
          {
            "name": "ecdsaPublicKey",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "rpcAddress",
            "type": "string",
            "internalType": "string"
          }
        ]
      },
      {
        "name": "requestId",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "onRequest",
    "inputs": [
      {
        "name": "params",
        "type": "tuple",
        "internalType": "struct ServiceOperators.RequestParams",
        "components": [
          {
            "name": "requestId",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "requester",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "operators",
            "type": "tuple[]",
            "internalType": "struct ServiceOperators.OperatorPreferences[]",
            "components": [
              {
                "name": "ecdsaPublicKey",
                "type": "bytes",
                "internalType": "bytes"
              },
              {
                "name": "rpcAddress",
                "type": "string",
                "internalType": "string"
              }
            ]
          },
          {
            "name": "requestInputs",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "permittedCallers",
            "type": "address[]",
            "internalType": "address[]"
          },
          {
            "name": "ttl",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "paymentAsset",
            "type": "tuple",
            "internalType": "struct Assets.Asset",
            "components": [
              {
                "name": "kind",
                "type": "uint8",
                "internalType": "enum Assets.Kind"
              },
              {
                "name": "data",
                "type": "bytes32",
                "internalType": "bytes32"
              }
            ]
          },
          {
            "name": "amount",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "onServiceInitialized",
    "inputs": [
      {
        "name": "requestId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "permittedCallers",
        "type": "address[]",
        "internalType": "address[]"
      },
      {
        "name": "ttl",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "onServiceTermination",
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
    "name": "onSlash",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "offender",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "slashPercent",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "onUnappliedSlash",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "offender",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "slashPercent",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "onUnregister",
    "inputs": [
      {
        "name": "operator",
        "type": "tuple",
        "internalType": "struct ServiceOperators.OperatorPreferences",
        "components": [
          {
            "name": "ecdsaPublicKey",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "rpcAddress",
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
    "name": "onUpdateRpcAddress",
    "inputs": [
      {
        "name": "operator",
        "type": "tuple",
        "internalType": "struct ServiceOperators.OperatorPreferences",
        "components": [
          {
            "name": "ecdsaPublicKey",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "rpcAddress",
            "type": "string",
            "internalType": "string"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "queryDeveloperPaymentAddress",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [
      {
        "name": "developerPaymentAddress",
        "type": "address",
        "internalType": "address payable"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "queryDisputeOrigin",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [
      {
        "name": "disputeOrigin",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "queryIsPaymentAssetAllowed",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "asset",
        "type": "tuple",
        "internalType": "struct Assets.Asset",
        "components": [
          {
            "name": "kind",
            "type": "uint8",
            "internalType": "enum Assets.Kind"
          },
          {
            "name": "data",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "isAllowed",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "querySlashingOrigin",
    "inputs": [
      {
        "name": "serviceId",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [
      {
        "name": "slashingOrigin",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
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
pub mod IBlueprintServiceManager {
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
    /**Function with signature `canJoin(uint64,(bytes,string))` and selector `0x760c379f`.
```solidity
function canJoin(uint64 serviceId, ServiceOperators.OperatorPreferences memory operator) external view returns (bool allowed);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct canJoinCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub operator: <ServiceOperators::OperatorPreferences as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`canJoin(uint64,(bytes,string))`](canJoinCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct canJoinReturn {
        #[allow(missing_docs)]
        pub allowed: bool,
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
                ServiceOperators::OperatorPreferences,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
                <ServiceOperators::OperatorPreferences as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<canJoinCall> for UnderlyingRustTuple<'_> {
                fn from(value: canJoinCall) -> Self {
                    (value.serviceId, value.operator)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for canJoinCall {
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
            impl ::core::convert::From<canJoinReturn> for UnderlyingRustTuple<'_> {
                fn from(value: canJoinReturn) -> Self {
                    (value.allowed,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for canJoinReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { allowed: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for canJoinCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                ServiceOperators::OperatorPreferences,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = canJoinReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "canJoin(uint64,(bytes,string))";
            const SELECTOR: [u8; 4] = [118u8, 12u8, 55u8, 159u8];
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
                    <ServiceOperators::OperatorPreferences as alloy_sol_types::SolType>::tokenize(
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
    /**Function with signature `canLeave(uint64,(bytes,string))` and selector `0x25489146`.
```solidity
function canLeave(uint64 serviceId, ServiceOperators.OperatorPreferences memory operator) external view returns (bool allowed);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct canLeaveCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub operator: <ServiceOperators::OperatorPreferences as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`canLeave(uint64,(bytes,string))`](canLeaveCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct canLeaveReturn {
        #[allow(missing_docs)]
        pub allowed: bool,
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
                ServiceOperators::OperatorPreferences,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
                <ServiceOperators::OperatorPreferences as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<canLeaveCall> for UnderlyingRustTuple<'_> {
                fn from(value: canLeaveCall) -> Self {
                    (value.serviceId, value.operator)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for canLeaveCall {
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
            impl ::core::convert::From<canLeaveReturn> for UnderlyingRustTuple<'_> {
                fn from(value: canLeaveReturn) -> Self {
                    (value.allowed,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for canLeaveReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { allowed: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for canLeaveCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                ServiceOperators::OperatorPreferences,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = canLeaveReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "canLeave(uint64,(bytes,string))";
            const SELECTOR: [u8; 4] = [37u8, 72u8, 145u8, 70u8];
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
                    <ServiceOperators::OperatorPreferences as alloy_sol_types::SolType>::tokenize(
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
    /**Function with signature `getHeartbeatInterval(uint64)` and selector `0x0068080b`.
```solidity
function getHeartbeatInterval(uint64 serviceId) external view returns (bool useDefault, uint64 interval);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getHeartbeatIntervalCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
    }
    ///Container type for the return parameters of the [`getHeartbeatInterval(uint64)`](getHeartbeatIntervalCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getHeartbeatIntervalReturn {
        #[allow(missing_docs)]
        pub useDefault: bool,
        #[allow(missing_docs)]
        pub interval: u64,
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
            impl ::core::convert::From<getHeartbeatIntervalCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getHeartbeatIntervalCall) -> Self {
                    (value.serviceId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getHeartbeatIntervalCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { serviceId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<64>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool, u64);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getHeartbeatIntervalReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getHeartbeatIntervalReturn) -> Self {
                    (value.useDefault, value.interval)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getHeartbeatIntervalReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        useDefault: tuple.0,
                        interval: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getHeartbeatIntervalCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getHeartbeatIntervalReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getHeartbeatInterval(uint64)";
            const SELECTOR: [u8; 4] = [0u8, 104u8, 8u8, 11u8];
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
    /**Function with signature `getHeartbeatThreshold(uint64)` and selector `0x92cbb78e`.
```solidity
function getHeartbeatThreshold(uint64 serviceId) external view returns (bool useDefault, uint8 threshold);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getHeartbeatThresholdCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
    }
    ///Container type for the return parameters of the [`getHeartbeatThreshold(uint64)`](getHeartbeatThresholdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getHeartbeatThresholdReturn {
        #[allow(missing_docs)]
        pub useDefault: bool,
        #[allow(missing_docs)]
        pub threshold: u8,
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
            impl ::core::convert::From<getHeartbeatThresholdCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getHeartbeatThresholdCall) -> Self {
                    (value.serviceId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getHeartbeatThresholdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { serviceId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<8>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool, u8);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getHeartbeatThresholdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getHeartbeatThresholdReturn) -> Self {
                    (value.useDefault, value.threshold)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getHeartbeatThresholdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        useDefault: tuple.0,
                        threshold: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getHeartbeatThresholdCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getHeartbeatThresholdReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<8>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getHeartbeatThreshold(uint64)";
            const SELECTOR: [u8; 4] = [146u8, 203u8, 183u8, 142u8];
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
    /**Function with signature `getSlashingWindow(uint64)` and selector `0x3fdadfcc`.
```solidity
function getSlashingWindow(uint64 serviceId) external view returns (bool useDefault, uint64 window);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSlashingWindowCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
    }
    ///Container type for the return parameters of the [`getSlashingWindow(uint64)`](getSlashingWindowCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSlashingWindowReturn {
        #[allow(missing_docs)]
        pub useDefault: bool,
        #[allow(missing_docs)]
        pub window: u64,
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
            impl ::core::convert::From<getSlashingWindowCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getSlashingWindowCall) -> Self {
                    (value.serviceId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getSlashingWindowCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { serviceId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<64>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool, u64);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getSlashingWindowReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getSlashingWindowReturn) -> Self {
                    (value.useDefault, value.window)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getSlashingWindowReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        useDefault: tuple.0,
                        window: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getSlashingWindowCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getSlashingWindowReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getSlashingWindow(uint64)";
            const SELECTOR: [u8; 4] = [63u8, 218u8, 223u8, 204u8];
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
    /**Function with signature `onApprove((bytes,string),uint64,uint8)` and selector `0x5287cd5f`.
```solidity
function onApprove(ServiceOperators.OperatorPreferences memory operator, uint64 requestId, uint8 restakingPercent) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onApproveCall {
        #[allow(missing_docs)]
        pub operator: <ServiceOperators::OperatorPreferences as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub requestId: u64,
        #[allow(missing_docs)]
        pub restakingPercent: u8,
    }
    ///Container type for the return parameters of the [`onApprove((bytes,string),uint64,uint8)`](onApproveCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onApproveReturn {}
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
                ServiceOperators::OperatorPreferences,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ServiceOperators::OperatorPreferences as alloy::sol_types::SolType>::RustType,
                u64,
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
            impl ::core::convert::From<onApproveCall> for UnderlyingRustTuple<'_> {
                fn from(value: onApproveCall) -> Self {
                    (value.operator, value.requestId, value.restakingPercent)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for onApproveCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        requestId: tuple.1,
                        restakingPercent: tuple.2,
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
            impl ::core::convert::From<onApproveReturn> for UnderlyingRustTuple<'_> {
                fn from(value: onApproveReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for onApproveReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for onApproveCall {
            type Parameters<'a> = (
                ServiceOperators::OperatorPreferences,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = onApproveReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "onApprove((bytes,string),uint64,uint8)";
            const SELECTOR: [u8; 4] = [82u8, 135u8, 205u8, 95u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <ServiceOperators::OperatorPreferences as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.requestId),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.restakingPercent),
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
    /**Function with signature `onBlueprintCreated(uint64,address,address)` and selector `0x0b6535d7`.
```solidity
function onBlueprintCreated(uint64 blueprintId, address owner, address mbsm) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onBlueprintCreatedCall {
        #[allow(missing_docs)]
        pub blueprintId: u64,
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub mbsm: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`onBlueprintCreated(uint64,address,address)`](onBlueprintCreatedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onBlueprintCreatedReturn {}
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
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
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
            impl ::core::convert::From<onBlueprintCreatedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: onBlueprintCreatedCall) -> Self {
                    (value.blueprintId, value.owner, value.mbsm)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for onBlueprintCreatedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        blueprintId: tuple.0,
                        owner: tuple.1,
                        mbsm: tuple.2,
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
            impl ::core::convert::From<onBlueprintCreatedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: onBlueprintCreatedReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for onBlueprintCreatedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for onBlueprintCreatedCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = onBlueprintCreatedReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "onBlueprintCreated(uint64,address,address)";
            const SELECTOR: [u8; 4] = [11u8, 101u8, 53u8, 215u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.blueprintId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.mbsm,
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
    /**Function with signature `onJobCall(uint64,uint8,uint64,bytes)` and selector `0x9838caa3`.
```solidity
function onJobCall(uint64 serviceId, uint8 job, uint64 jobCallId, bytes memory inputs) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onJobCallCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub job: u8,
        #[allow(missing_docs)]
        pub jobCallId: u64,
        #[allow(missing_docs)]
        pub inputs: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`onJobCall(uint64,uint8,uint64,bytes)`](onJobCallCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onJobCallReturn {}
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
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
                u8,
                u64,
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
            impl ::core::convert::From<onJobCallCall> for UnderlyingRustTuple<'_> {
                fn from(value: onJobCallCall) -> Self {
                    (value.serviceId, value.job, value.jobCallId, value.inputs)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for onJobCallCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        serviceId: tuple.0,
                        job: tuple.1,
                        jobCallId: tuple.2,
                        inputs: tuple.3,
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
            impl ::core::convert::From<onJobCallReturn> for UnderlyingRustTuple<'_> {
                fn from(value: onJobCallReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for onJobCallReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for onJobCallCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = onJobCallReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "onJobCall(uint64,uint8,uint64,bytes)";
            const SELECTOR: [u8; 4] = [152u8, 56u8, 202u8, 163u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.job),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.jobCallId),
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
    /**Function with signature `onJobResult(uint64,uint8,uint64,(bytes,string),bytes,bytes)` and selector `0x29b98dc7`.
```solidity
function onJobResult(uint64 serviceId, uint8 job, uint64 jobCallId, ServiceOperators.OperatorPreferences memory operator, bytes memory inputs, bytes memory outputs) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onJobResultCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub job: u8,
        #[allow(missing_docs)]
        pub jobCallId: u64,
        #[allow(missing_docs)]
        pub operator: <ServiceOperators::OperatorPreferences as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub inputs: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub outputs: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`onJobResult(uint64,uint8,uint64,(bytes,string),bytes,bytes)`](onJobResultCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onJobResultReturn {}
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
                alloy::sol_types::sol_data::Uint<64>,
                ServiceOperators::OperatorPreferences,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
                u8,
                u64,
                <ServiceOperators::OperatorPreferences as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<onJobResultCall> for UnderlyingRustTuple<'_> {
                fn from(value: onJobResultCall) -> Self {
                    (
                        value.serviceId,
                        value.job,
                        value.jobCallId,
                        value.operator,
                        value.inputs,
                        value.outputs,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for onJobResultCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        serviceId: tuple.0,
                        job: tuple.1,
                        jobCallId: tuple.2,
                        operator: tuple.3,
                        inputs: tuple.4,
                        outputs: tuple.5,
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
            impl ::core::convert::From<onJobResultReturn> for UnderlyingRustTuple<'_> {
                fn from(value: onJobResultReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for onJobResultReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for onJobResultCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<64>,
                ServiceOperators::OperatorPreferences,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = onJobResultReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "onJobResult(uint64,uint8,uint64,(bytes,string),bytes,bytes)";
            const SELECTOR: [u8; 4] = [41u8, 185u8, 141u8, 199u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.job),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.jobCallId),
                    <ServiceOperators::OperatorPreferences as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.inputs,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.outputs,
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
    /**Function with signature `onOperatorJoined(uint64,(bytes,string))` and selector `0x89270616`.
```solidity
function onOperatorJoined(uint64 serviceId, ServiceOperators.OperatorPreferences memory operator) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onOperatorJoinedCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub operator: <ServiceOperators::OperatorPreferences as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`onOperatorJoined(uint64,(bytes,string))`](onOperatorJoinedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onOperatorJoinedReturn {}
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
                ServiceOperators::OperatorPreferences,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
                <ServiceOperators::OperatorPreferences as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<onOperatorJoinedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: onOperatorJoinedCall) -> Self {
                    (value.serviceId, value.operator)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for onOperatorJoinedCall {
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
            impl ::core::convert::From<onOperatorJoinedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: onOperatorJoinedReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for onOperatorJoinedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for onOperatorJoinedCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                ServiceOperators::OperatorPreferences,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = onOperatorJoinedReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "onOperatorJoined(uint64,(bytes,string))";
            const SELECTOR: [u8; 4] = [137u8, 39u8, 6u8, 22u8];
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
                    <ServiceOperators::OperatorPreferences as alloy_sol_types::SolType>::tokenize(
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
    /**Function with signature `onOperatorLeft(uint64,(bytes,string))` and selector `0x10f772b8`.
```solidity
function onOperatorLeft(uint64 serviceId, ServiceOperators.OperatorPreferences memory operator) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onOperatorLeftCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub operator: <ServiceOperators::OperatorPreferences as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`onOperatorLeft(uint64,(bytes,string))`](onOperatorLeftCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onOperatorLeftReturn {}
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
                ServiceOperators::OperatorPreferences,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
                <ServiceOperators::OperatorPreferences as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<onOperatorLeftCall> for UnderlyingRustTuple<'_> {
                fn from(value: onOperatorLeftCall) -> Self {
                    (value.serviceId, value.operator)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for onOperatorLeftCall {
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
            impl ::core::convert::From<onOperatorLeftReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: onOperatorLeftReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for onOperatorLeftReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for onOperatorLeftCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                ServiceOperators::OperatorPreferences,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = onOperatorLeftReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "onOperatorLeft(uint64,(bytes,string))";
            const SELECTOR: [u8; 4] = [16u8, 247u8, 114u8, 184u8];
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
                    <ServiceOperators::OperatorPreferences as alloy_sol_types::SolType>::tokenize(
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
    /**Function with signature `onRegister((bytes,string),bytes)` and selector `0xc4b44839`.
```solidity
function onRegister(ServiceOperators.OperatorPreferences memory operator, bytes memory registrationInputs) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onRegisterCall {
        #[allow(missing_docs)]
        pub operator: <ServiceOperators::OperatorPreferences as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub registrationInputs: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`onRegister((bytes,string),bytes)`](onRegisterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onRegisterReturn {}
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
                ServiceOperators::OperatorPreferences,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ServiceOperators::OperatorPreferences as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<onRegisterCall> for UnderlyingRustTuple<'_> {
                fn from(value: onRegisterCall) -> Self {
                    (value.operator, value.registrationInputs)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for onRegisterCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        registrationInputs: tuple.1,
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
            impl ::core::convert::From<onRegisterReturn> for UnderlyingRustTuple<'_> {
                fn from(value: onRegisterReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for onRegisterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for onRegisterCall {
            type Parameters<'a> = (
                ServiceOperators::OperatorPreferences,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = onRegisterReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "onRegister((bytes,string),bytes)";
            const SELECTOR: [u8; 4] = [196u8, 180u8, 72u8, 57u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <ServiceOperators::OperatorPreferences as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.registrationInputs,
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
    /**Function with signature `onReject((bytes,string),uint64)` and selector `0xa9c380e1`.
```solidity
function onReject(ServiceOperators.OperatorPreferences memory operator, uint64 requestId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onRejectCall {
        #[allow(missing_docs)]
        pub operator: <ServiceOperators::OperatorPreferences as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub requestId: u64,
    }
    ///Container type for the return parameters of the [`onReject((bytes,string),uint64)`](onRejectCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onRejectReturn {}
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
                ServiceOperators::OperatorPreferences,
                alloy::sol_types::sol_data::Uint<64>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ServiceOperators::OperatorPreferences as alloy::sol_types::SolType>::RustType,
                u64,
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
            impl ::core::convert::From<onRejectCall> for UnderlyingRustTuple<'_> {
                fn from(value: onRejectCall) -> Self {
                    (value.operator, value.requestId)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for onRejectCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        requestId: tuple.1,
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
            impl ::core::convert::From<onRejectReturn> for UnderlyingRustTuple<'_> {
                fn from(value: onRejectReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for onRejectReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for onRejectCall {
            type Parameters<'a> = (
                ServiceOperators::OperatorPreferences,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = onRejectReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "onReject((bytes,string),uint64)";
            const SELECTOR: [u8; 4] = [169u8, 195u8, 128u8, 225u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <ServiceOperators::OperatorPreferences as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.requestId),
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
    /**Function with signature `onRequest((uint64,address,(bytes,string)[],bytes,address[],uint64,(uint8,bytes32),uint256))` and selector `0xa40a9527`.
```solidity
function onRequest(ServiceOperators.RequestParams memory params) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onRequestCall {
        #[allow(missing_docs)]
        pub params: <ServiceOperators::RequestParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`onRequest((uint64,address,(bytes,string)[],bytes,address[],uint64,(uint8,bytes32),uint256))`](onRequestCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onRequestReturn {}
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
            type UnderlyingSolTuple<'a> = (ServiceOperators::RequestParams,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ServiceOperators::RequestParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<onRequestCall> for UnderlyingRustTuple<'_> {
                fn from(value: onRequestCall) -> Self {
                    (value.params,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for onRequestCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { params: tuple.0 }
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
            impl ::core::convert::From<onRequestReturn> for UnderlyingRustTuple<'_> {
                fn from(value: onRequestReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for onRequestReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for onRequestCall {
            type Parameters<'a> = (ServiceOperators::RequestParams,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = onRequestReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "onRequest((uint64,address,(bytes,string)[],bytes,address[],uint64,(uint8,bytes32),uint256))";
            const SELECTOR: [u8; 4] = [164u8, 10u8, 149u8, 39u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <ServiceOperators::RequestParams as alloy_sol_types::SolType>::tokenize(
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
    /**Function with signature `onServiceInitialized(uint64,uint64,address,address[],uint64)` and selector `0xd7deb482`.
```solidity
function onServiceInitialized(uint64 requestId, uint64 serviceId, address owner, address[] memory permittedCallers, uint64 ttl) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onServiceInitializedCall {
        #[allow(missing_docs)]
        pub requestId: u64,
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub permittedCallers: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub ttl: u64,
    }
    ///Container type for the return parameters of the [`onServiceInitialized(uint64,uint64,address,address[],uint64)`](onServiceInitializedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onServiceInitializedReturn {}
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
                u64,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                u64,
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
            impl ::core::convert::From<onServiceInitializedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: onServiceInitializedCall) -> Self {
                    (
                        value.requestId,
                        value.serviceId,
                        value.owner,
                        value.permittedCallers,
                        value.ttl,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for onServiceInitializedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        requestId: tuple.0,
                        serviceId: tuple.1,
                        owner: tuple.2,
                        permittedCallers: tuple.3,
                        ttl: tuple.4,
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
            impl ::core::convert::From<onServiceInitializedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: onServiceInitializedReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for onServiceInitializedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for onServiceInitializedCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = onServiceInitializedReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "onServiceInitialized(uint64,uint64,address,address[],uint64)";
            const SELECTOR: [u8; 4] = [215u8, 222u8, 180u8, 130u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.requestId),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.serviceId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.permittedCallers),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.ttl),
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
    /**Function with signature `onServiceTermination(uint64,address)` and selector `0xa24e8a90`.
```solidity
function onServiceTermination(uint64 serviceId, address owner) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onServiceTerminationCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`onServiceTermination(uint64,address)`](onServiceTerminationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onServiceTerminationReturn {}
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
            impl ::core::convert::From<onServiceTerminationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: onServiceTerminationCall) -> Self {
                    (value.serviceId, value.owner)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for onServiceTerminationCall {
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
            impl ::core::convert::From<onServiceTerminationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: onServiceTerminationReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for onServiceTerminationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for onServiceTerminationCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = onServiceTerminationReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "onServiceTermination(uint64,address)";
            const SELECTOR: [u8; 4] = [162u8, 78u8, 138u8, 144u8];
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
    /**Function with signature `onSlash(uint64,bytes,uint8)` and selector `0x7e14b919`.
```solidity
function onSlash(uint64 serviceId, bytes memory offender, uint8 slashPercent) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onSlashCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub offender: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub slashPercent: u8,
    }
    ///Container type for the return parameters of the [`onSlash(uint64,bytes,uint8)`](onSlashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onSlashReturn {}
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
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<8>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64, alloy::sol_types::private::Bytes, u8);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<onSlashCall> for UnderlyingRustTuple<'_> {
                fn from(value: onSlashCall) -> Self {
                    (value.serviceId, value.offender, value.slashPercent)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for onSlashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        serviceId: tuple.0,
                        offender: tuple.1,
                        slashPercent: tuple.2,
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
            impl ::core::convert::From<onSlashReturn> for UnderlyingRustTuple<'_> {
                fn from(value: onSlashReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for onSlashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for onSlashCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<8>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = onSlashReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "onSlash(uint64,bytes,uint8)";
            const SELECTOR: [u8; 4] = [126u8, 20u8, 185u8, 25u8];
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.offender,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.slashPercent),
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
    /**Function with signature `onUnappliedSlash(uint64,bytes,uint8)` and selector `0x0881898f`.
```solidity
function onUnappliedSlash(uint64 serviceId, bytes memory offender, uint8 slashPercent) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onUnappliedSlashCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub offender: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub slashPercent: u8,
    }
    ///Container type for the return parameters of the [`onUnappliedSlash(uint64,bytes,uint8)`](onUnappliedSlashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onUnappliedSlashReturn {}
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
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<8>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64, alloy::sol_types::private::Bytes, u8);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<onUnappliedSlashCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: onUnappliedSlashCall) -> Self {
                    (value.serviceId, value.offender, value.slashPercent)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for onUnappliedSlashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        serviceId: tuple.0,
                        offender: tuple.1,
                        slashPercent: tuple.2,
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
            impl ::core::convert::From<onUnappliedSlashReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: onUnappliedSlashReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for onUnappliedSlashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for onUnappliedSlashCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<8>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = onUnappliedSlashReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "onUnappliedSlash(uint64,bytes,uint8)";
            const SELECTOR: [u8; 4] = [8u8, 129u8, 137u8, 143u8];
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.offender,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.slashPercent),
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
    /**Function with signature `onUnregister((bytes,string))` and selector `0xe0503b71`.
```solidity
function onUnregister(ServiceOperators.OperatorPreferences memory operator) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onUnregisterCall {
        #[allow(missing_docs)]
        pub operator: <ServiceOperators::OperatorPreferences as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`onUnregister((bytes,string))`](onUnregisterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onUnregisterReturn {}
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
            type UnderlyingSolTuple<'a> = (ServiceOperators::OperatorPreferences,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ServiceOperators::OperatorPreferences as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<onUnregisterCall> for UnderlyingRustTuple<'_> {
                fn from(value: onUnregisterCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for onUnregisterCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
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
            impl ::core::convert::From<onUnregisterReturn> for UnderlyingRustTuple<'_> {
                fn from(value: onUnregisterReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for onUnregisterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for onUnregisterCall {
            type Parameters<'a> = (ServiceOperators::OperatorPreferences,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = onUnregisterReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "onUnregister((bytes,string))";
            const SELECTOR: [u8; 4] = [224u8, 80u8, 59u8, 113u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <ServiceOperators::OperatorPreferences as alloy_sol_types::SolType>::tokenize(
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
    /**Function with signature `onUpdateRpcAddress((bytes,string))` and selector `0xdcfee99a`.
```solidity
function onUpdateRpcAddress(ServiceOperators.OperatorPreferences memory operator) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onUpdateRpcAddressCall {
        #[allow(missing_docs)]
        pub operator: <ServiceOperators::OperatorPreferences as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`onUpdateRpcAddress((bytes,string))`](onUpdateRpcAddressCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onUpdateRpcAddressReturn {}
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
            type UnderlyingSolTuple<'a> = (ServiceOperators::OperatorPreferences,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ServiceOperators::OperatorPreferences as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<onUpdateRpcAddressCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: onUpdateRpcAddressCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for onUpdateRpcAddressCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
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
            impl ::core::convert::From<onUpdateRpcAddressReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: onUpdateRpcAddressReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for onUpdateRpcAddressReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for onUpdateRpcAddressCall {
            type Parameters<'a> = (ServiceOperators::OperatorPreferences,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = onUpdateRpcAddressReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "onUpdateRpcAddress((bytes,string))";
            const SELECTOR: [u8; 4] = [220u8, 254u8, 233u8, 154u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <ServiceOperators::OperatorPreferences as alloy_sol_types::SolType>::tokenize(
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
    /**Function with signature `queryDeveloperPaymentAddress(uint64)` and selector `0x08179f35`.
```solidity
function queryDeveloperPaymentAddress(uint64 serviceId) external view returns (address developerPaymentAddress);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct queryDeveloperPaymentAddressCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
    }
    ///Container type for the return parameters of the [`queryDeveloperPaymentAddress(uint64)`](queryDeveloperPaymentAddressCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct queryDeveloperPaymentAddressReturn {
        #[allow(missing_docs)]
        pub developerPaymentAddress: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<queryDeveloperPaymentAddressCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: queryDeveloperPaymentAddressCall) -> Self {
                    (value.serviceId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for queryDeveloperPaymentAddressCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { serviceId: tuple.0 }
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
            impl ::core::convert::From<queryDeveloperPaymentAddressReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: queryDeveloperPaymentAddressReturn) -> Self {
                    (value.developerPaymentAddress,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for queryDeveloperPaymentAddressReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        developerPaymentAddress: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for queryDeveloperPaymentAddressCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = queryDeveloperPaymentAddressReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "queryDeveloperPaymentAddress(uint64)";
            const SELECTOR: [u8; 4] = [8u8, 23u8, 159u8, 53u8];
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
    /**Function with signature `queryDisputeOrigin(uint64)` and selector `0x14b4df4c`.
```solidity
function queryDisputeOrigin(uint64 serviceId) external view returns (address disputeOrigin);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct queryDisputeOriginCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
    }
    ///Container type for the return parameters of the [`queryDisputeOrigin(uint64)`](queryDisputeOriginCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct queryDisputeOriginReturn {
        #[allow(missing_docs)]
        pub disputeOrigin: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<queryDisputeOriginCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: queryDisputeOriginCall) -> Self {
                    (value.serviceId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for queryDisputeOriginCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { serviceId: tuple.0 }
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
            impl ::core::convert::From<queryDisputeOriginReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: queryDisputeOriginReturn) -> Self {
                    (value.disputeOrigin,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for queryDisputeOriginReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { disputeOrigin: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for queryDisputeOriginCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = queryDisputeOriginReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "queryDisputeOrigin(uint64)";
            const SELECTOR: [u8; 4] = [20u8, 180u8, 223u8, 76u8];
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
    /**Function with signature `queryIsPaymentAssetAllowed(uint64,(uint8,bytes32))` and selector `0x3c3aa64c`.
```solidity
function queryIsPaymentAssetAllowed(uint64 serviceId, Assets.Asset memory asset) external view returns (bool isAllowed);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct queryIsPaymentAssetAllowedCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
        #[allow(missing_docs)]
        pub asset: <Assets::Asset as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`queryIsPaymentAssetAllowed(uint64,(uint8,bytes32))`](queryIsPaymentAssetAllowedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct queryIsPaymentAssetAllowedReturn {
        #[allow(missing_docs)]
        pub isAllowed: bool,
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
                Assets::Asset,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
                <Assets::Asset as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<queryIsPaymentAssetAllowedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: queryIsPaymentAssetAllowedCall) -> Self {
                    (value.serviceId, value.asset)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for queryIsPaymentAssetAllowedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        serviceId: tuple.0,
                        asset: tuple.1,
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
            impl ::core::convert::From<queryIsPaymentAssetAllowedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: queryIsPaymentAssetAllowedReturn) -> Self {
                    (value.isAllowed,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for queryIsPaymentAssetAllowedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { isAllowed: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for queryIsPaymentAssetAllowedCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>, Assets::Asset);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = queryIsPaymentAssetAllowedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "queryIsPaymentAssetAllowed(uint64,(uint8,bytes32))";
            const SELECTOR: [u8; 4] = [60u8, 58u8, 166u8, 76u8];
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
                    <Assets::Asset as alloy_sol_types::SolType>::tokenize(&self.asset),
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
    /**Function with signature `querySlashingOrigin(uint64)` and selector `0x74ceeb55`.
```solidity
function querySlashingOrigin(uint64 serviceId) external view returns (address slashingOrigin);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct querySlashingOriginCall {
        #[allow(missing_docs)]
        pub serviceId: u64,
    }
    ///Container type for the return parameters of the [`querySlashingOrigin(uint64)`](querySlashingOriginCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct querySlashingOriginReturn {
        #[allow(missing_docs)]
        pub slashingOrigin: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<querySlashingOriginCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: querySlashingOriginCall) -> Self {
                    (value.serviceId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for querySlashingOriginCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { serviceId: tuple.0 }
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
            impl ::core::convert::From<querySlashingOriginReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: querySlashingOriginReturn) -> Self {
                    (value.slashingOrigin,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for querySlashingOriginReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { slashingOrigin: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for querySlashingOriginCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = querySlashingOriginReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "querySlashingOrigin(uint64)";
            const SELECTOR: [u8; 4] = [116u8, 206u8, 235u8, 85u8];
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
    ///Container for all the [`IBlueprintServiceManager`](self) function calls.
    pub enum IBlueprintServiceManagerCalls {
        #[allow(missing_docs)]
        canJoin(canJoinCall),
        #[allow(missing_docs)]
        canLeave(canLeaveCall),
        #[allow(missing_docs)]
        getHeartbeatInterval(getHeartbeatIntervalCall),
        #[allow(missing_docs)]
        getHeartbeatThreshold(getHeartbeatThresholdCall),
        #[allow(missing_docs)]
        getSlashingWindow(getSlashingWindowCall),
        #[allow(missing_docs)]
        onApprove(onApproveCall),
        #[allow(missing_docs)]
        onBlueprintCreated(onBlueprintCreatedCall),
        #[allow(missing_docs)]
        onJobCall(onJobCallCall),
        #[allow(missing_docs)]
        onJobResult(onJobResultCall),
        #[allow(missing_docs)]
        onOperatorJoined(onOperatorJoinedCall),
        #[allow(missing_docs)]
        onOperatorLeft(onOperatorLeftCall),
        #[allow(missing_docs)]
        onRegister(onRegisterCall),
        #[allow(missing_docs)]
        onReject(onRejectCall),
        #[allow(missing_docs)]
        onRequest(onRequestCall),
        #[allow(missing_docs)]
        onServiceInitialized(onServiceInitializedCall),
        #[allow(missing_docs)]
        onServiceTermination(onServiceTerminationCall),
        #[allow(missing_docs)]
        onSlash(onSlashCall),
        #[allow(missing_docs)]
        onUnappliedSlash(onUnappliedSlashCall),
        #[allow(missing_docs)]
        onUnregister(onUnregisterCall),
        #[allow(missing_docs)]
        onUpdateRpcAddress(onUpdateRpcAddressCall),
        #[allow(missing_docs)]
        queryDeveloperPaymentAddress(queryDeveloperPaymentAddressCall),
        #[allow(missing_docs)]
        queryDisputeOrigin(queryDisputeOriginCall),
        #[allow(missing_docs)]
        queryIsPaymentAssetAllowed(queryIsPaymentAssetAllowedCall),
        #[allow(missing_docs)]
        querySlashingOrigin(querySlashingOriginCall),
    }
    #[automatically_derived]
    impl IBlueprintServiceManagerCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [0u8, 104u8, 8u8, 11u8],
            [8u8, 23u8, 159u8, 53u8],
            [8u8, 129u8, 137u8, 143u8],
            [11u8, 101u8, 53u8, 215u8],
            [16u8, 247u8, 114u8, 184u8],
            [20u8, 180u8, 223u8, 76u8],
            [37u8, 72u8, 145u8, 70u8],
            [41u8, 185u8, 141u8, 199u8],
            [60u8, 58u8, 166u8, 76u8],
            [63u8, 218u8, 223u8, 204u8],
            [82u8, 135u8, 205u8, 95u8],
            [116u8, 206u8, 235u8, 85u8],
            [118u8, 12u8, 55u8, 159u8],
            [126u8, 20u8, 185u8, 25u8],
            [137u8, 39u8, 6u8, 22u8],
            [146u8, 203u8, 183u8, 142u8],
            [152u8, 56u8, 202u8, 163u8],
            [162u8, 78u8, 138u8, 144u8],
            [164u8, 10u8, 149u8, 39u8],
            [169u8, 195u8, 128u8, 225u8],
            [196u8, 180u8, 72u8, 57u8],
            [215u8, 222u8, 180u8, 130u8],
            [220u8, 254u8, 233u8, 154u8],
            [224u8, 80u8, 59u8, 113u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for IBlueprintServiceManagerCalls {
        const NAME: &'static str = "IBlueprintServiceManagerCalls";
        const MIN_DATA_LENGTH: usize = 32usize;
        const COUNT: usize = 24usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::canJoin(_) => <canJoinCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::canLeave(_) => <canLeaveCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getHeartbeatInterval(_) => {
                    <getHeartbeatIntervalCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getHeartbeatThreshold(_) => {
                    <getHeartbeatThresholdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getSlashingWindow(_) => {
                    <getSlashingWindowCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::onApprove(_) => {
                    <onApproveCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::onBlueprintCreated(_) => {
                    <onBlueprintCreatedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::onJobCall(_) => {
                    <onJobCallCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::onJobResult(_) => {
                    <onJobResultCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::onOperatorJoined(_) => {
                    <onOperatorJoinedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::onOperatorLeft(_) => {
                    <onOperatorLeftCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::onRegister(_) => {
                    <onRegisterCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::onReject(_) => <onRejectCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::onRequest(_) => {
                    <onRequestCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::onServiceInitialized(_) => {
                    <onServiceInitializedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::onServiceTermination(_) => {
                    <onServiceTerminationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::onSlash(_) => <onSlashCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::onUnappliedSlash(_) => {
                    <onUnappliedSlashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::onUnregister(_) => {
                    <onUnregisterCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::onUpdateRpcAddress(_) => {
                    <onUpdateRpcAddressCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::queryDeveloperPaymentAddress(_) => {
                    <queryDeveloperPaymentAddressCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::queryDisputeOrigin(_) => {
                    <queryDisputeOriginCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::queryIsPaymentAssetAllowed(_) => {
                    <queryIsPaymentAssetAllowedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::querySlashingOrigin(_) => {
                    <querySlashingOriginCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<IBlueprintServiceManagerCalls>] = &[
                {
                    fn getHeartbeatInterval(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IBlueprintServiceManagerCalls> {
                        <getHeartbeatIntervalCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IBlueprintServiceManagerCalls::getHeartbeatInterval)
                    }
                    getHeartbeatInterval
                },
                {
                    fn queryDeveloperPaymentAddress(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IBlueprintServiceManagerCalls> {
                        <queryDeveloperPaymentAddressCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IBlueprintServiceManagerCalls::queryDeveloperPaymentAddress,
                            )
                    }
                    queryDeveloperPaymentAddress
                },
                {
                    fn onUnappliedSlash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IBlueprintServiceManagerCalls> {
                        <onUnappliedSlashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IBlueprintServiceManagerCalls::onUnappliedSlash)
                    }
                    onUnappliedSlash
                },
                {
                    fn onBlueprintCreated(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IBlueprintServiceManagerCalls> {
                        <onBlueprintCreatedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IBlueprintServiceManagerCalls::onBlueprintCreated)
                    }
                    onBlueprintCreated
                },
                {
                    fn onOperatorLeft(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IBlueprintServiceManagerCalls> {
                        <onOperatorLeftCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IBlueprintServiceManagerCalls::onOperatorLeft)
                    }
                    onOperatorLeft
                },
                {
                    fn queryDisputeOrigin(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IBlueprintServiceManagerCalls> {
                        <queryDisputeOriginCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IBlueprintServiceManagerCalls::queryDisputeOrigin)
                    }
                    queryDisputeOrigin
                },
                {
                    fn canLeave(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IBlueprintServiceManagerCalls> {
                        <canLeaveCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IBlueprintServiceManagerCalls::canLeave)
                    }
                    canLeave
                },
                {
                    fn onJobResult(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IBlueprintServiceManagerCalls> {
                        <onJobResultCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IBlueprintServiceManagerCalls::onJobResult)
                    }
                    onJobResult
                },
                {
                    fn queryIsPaymentAssetAllowed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IBlueprintServiceManagerCalls> {
                        <queryIsPaymentAssetAllowedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IBlueprintServiceManagerCalls::queryIsPaymentAssetAllowed,
                            )
                    }
                    queryIsPaymentAssetAllowed
                },
                {
                    fn getSlashingWindow(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IBlueprintServiceManagerCalls> {
                        <getSlashingWindowCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IBlueprintServiceManagerCalls::getSlashingWindow)
                    }
                    getSlashingWindow
                },
                {
                    fn onApprove(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IBlueprintServiceManagerCalls> {
                        <onApproveCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IBlueprintServiceManagerCalls::onApprove)
                    }
                    onApprove
                },
                {
                    fn querySlashingOrigin(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IBlueprintServiceManagerCalls> {
                        <querySlashingOriginCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IBlueprintServiceManagerCalls::querySlashingOrigin)
                    }
                    querySlashingOrigin
                },
                {
                    fn canJoin(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IBlueprintServiceManagerCalls> {
                        <canJoinCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IBlueprintServiceManagerCalls::canJoin)
                    }
                    canJoin
                },
                {
                    fn onSlash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IBlueprintServiceManagerCalls> {
                        <onSlashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IBlueprintServiceManagerCalls::onSlash)
                    }
                    onSlash
                },
                {
                    fn onOperatorJoined(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IBlueprintServiceManagerCalls> {
                        <onOperatorJoinedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IBlueprintServiceManagerCalls::onOperatorJoined)
                    }
                    onOperatorJoined
                },
                {
                    fn getHeartbeatThreshold(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IBlueprintServiceManagerCalls> {
                        <getHeartbeatThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IBlueprintServiceManagerCalls::getHeartbeatThreshold)
                    }
                    getHeartbeatThreshold
                },
                {
                    fn onJobCall(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IBlueprintServiceManagerCalls> {
                        <onJobCallCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IBlueprintServiceManagerCalls::onJobCall)
                    }
                    onJobCall
                },
                {
                    fn onServiceTermination(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IBlueprintServiceManagerCalls> {
                        <onServiceTerminationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IBlueprintServiceManagerCalls::onServiceTermination)
                    }
                    onServiceTermination
                },
                {
                    fn onRequest(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IBlueprintServiceManagerCalls> {
                        <onRequestCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IBlueprintServiceManagerCalls::onRequest)
                    }
                    onRequest
                },
                {
                    fn onReject(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IBlueprintServiceManagerCalls> {
                        <onRejectCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IBlueprintServiceManagerCalls::onReject)
                    }
                    onReject
                },
                {
                    fn onRegister(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IBlueprintServiceManagerCalls> {
                        <onRegisterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IBlueprintServiceManagerCalls::onRegister)
                    }
                    onRegister
                },
                {
                    fn onServiceInitialized(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IBlueprintServiceManagerCalls> {
                        <onServiceInitializedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IBlueprintServiceManagerCalls::onServiceInitialized)
                    }
                    onServiceInitialized
                },
                {
                    fn onUpdateRpcAddress(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IBlueprintServiceManagerCalls> {
                        <onUpdateRpcAddressCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IBlueprintServiceManagerCalls::onUpdateRpcAddress)
                    }
                    onUpdateRpcAddress
                },
                {
                    fn onUnregister(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IBlueprintServiceManagerCalls> {
                        <onUnregisterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IBlueprintServiceManagerCalls::onUnregister)
                    }
                    onUnregister
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
                Self::canJoin(inner) => {
                    <canJoinCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::canLeave(inner) => {
                    <canLeaveCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getHeartbeatInterval(inner) => {
                    <getHeartbeatIntervalCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getHeartbeatThreshold(inner) => {
                    <getHeartbeatThresholdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getSlashingWindow(inner) => {
                    <getSlashingWindowCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::onApprove(inner) => {
                    <onApproveCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::onBlueprintCreated(inner) => {
                    <onBlueprintCreatedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::onJobCall(inner) => {
                    <onJobCallCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::onJobResult(inner) => {
                    <onJobResultCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::onOperatorJoined(inner) => {
                    <onOperatorJoinedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::onOperatorLeft(inner) => {
                    <onOperatorLeftCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::onRegister(inner) => {
                    <onRegisterCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::onReject(inner) => {
                    <onRejectCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::onRequest(inner) => {
                    <onRequestCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::onServiceInitialized(inner) => {
                    <onServiceInitializedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::onServiceTermination(inner) => {
                    <onServiceTerminationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::onSlash(inner) => {
                    <onSlashCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::onUnappliedSlash(inner) => {
                    <onUnappliedSlashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::onUnregister(inner) => {
                    <onUnregisterCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::onUpdateRpcAddress(inner) => {
                    <onUpdateRpcAddressCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::queryDeveloperPaymentAddress(inner) => {
                    <queryDeveloperPaymentAddressCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::queryDisputeOrigin(inner) => {
                    <queryDisputeOriginCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::queryIsPaymentAssetAllowed(inner) => {
                    <queryIsPaymentAssetAllowedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::querySlashingOrigin(inner) => {
                    <querySlashingOriginCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::canJoin(inner) => {
                    <canJoinCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::canLeave(inner) => {
                    <canLeaveCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getHeartbeatInterval(inner) => {
                    <getHeartbeatIntervalCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getHeartbeatThreshold(inner) => {
                    <getHeartbeatThresholdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getSlashingWindow(inner) => {
                    <getSlashingWindowCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::onApprove(inner) => {
                    <onApproveCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::onBlueprintCreated(inner) => {
                    <onBlueprintCreatedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::onJobCall(inner) => {
                    <onJobCallCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::onJobResult(inner) => {
                    <onJobResultCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::onOperatorJoined(inner) => {
                    <onOperatorJoinedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::onOperatorLeft(inner) => {
                    <onOperatorLeftCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::onRegister(inner) => {
                    <onRegisterCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::onReject(inner) => {
                    <onRejectCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::onRequest(inner) => {
                    <onRequestCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::onServiceInitialized(inner) => {
                    <onServiceInitializedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::onServiceTermination(inner) => {
                    <onServiceTerminationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::onSlash(inner) => {
                    <onSlashCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::onUnappliedSlash(inner) => {
                    <onUnappliedSlashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::onUnregister(inner) => {
                    <onUnregisterCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::onUpdateRpcAddress(inner) => {
                    <onUpdateRpcAddressCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::queryDeveloperPaymentAddress(inner) => {
                    <queryDeveloperPaymentAddressCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::queryDisputeOrigin(inner) => {
                    <queryDisputeOriginCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::queryIsPaymentAssetAllowed(inner) => {
                    <queryIsPaymentAssetAllowedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::querySlashingOrigin(inner) => {
                    <querySlashingOriginCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IBlueprintServiceManager`](self) contract instance.

See the [wrapper's documentation](`IBlueprintServiceManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IBlueprintServiceManagerInstance<T, P, N> {
        IBlueprintServiceManagerInstance::<T, P, N>::new(address, provider)
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
        Output = alloy_contract::Result<IBlueprintServiceManagerInstance<T, P, N>>,
    > {
        IBlueprintServiceManagerInstance::<T, P, N>::deploy(provider)
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
        IBlueprintServiceManagerInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`IBlueprintServiceManager`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IBlueprintServiceManager`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IBlueprintServiceManagerInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IBlueprintServiceManagerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IBlueprintServiceManagerInstance")
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
    > IBlueprintServiceManagerInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IBlueprintServiceManager`](self) contract instance.

See the [wrapper's documentation](`IBlueprintServiceManagerInstance`) for more details.*/
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
        ) -> alloy_contract::Result<IBlueprintServiceManagerInstance<T, P, N>> {
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
    impl<T, P: ::core::clone::Clone, N> IBlueprintServiceManagerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IBlueprintServiceManagerInstance<T, P, N> {
            IBlueprintServiceManagerInstance {
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
    > IBlueprintServiceManagerInstance<T, P, N> {
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
        ///Creates a new call builder for the [`canJoin`] function.
        pub fn canJoin(
            &self,
            serviceId: u64,
            operator: <ServiceOperators::OperatorPreferences as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, canJoinCall, N> {
            self.call_builder(&canJoinCall { serviceId, operator })
        }
        ///Creates a new call builder for the [`canLeave`] function.
        pub fn canLeave(
            &self,
            serviceId: u64,
            operator: <ServiceOperators::OperatorPreferences as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, canLeaveCall, N> {
            self.call_builder(
                &canLeaveCall {
                    serviceId,
                    operator,
                },
            )
        }
        ///Creates a new call builder for the [`getHeartbeatInterval`] function.
        pub fn getHeartbeatInterval(
            &self,
            serviceId: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, getHeartbeatIntervalCall, N> {
            self.call_builder(
                &getHeartbeatIntervalCall {
                    serviceId,
                },
            )
        }
        ///Creates a new call builder for the [`getHeartbeatThreshold`] function.
        pub fn getHeartbeatThreshold(
            &self,
            serviceId: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, getHeartbeatThresholdCall, N> {
            self.call_builder(
                &getHeartbeatThresholdCall {
                    serviceId,
                },
            )
        }
        ///Creates a new call builder for the [`getSlashingWindow`] function.
        pub fn getSlashingWindow(
            &self,
            serviceId: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, getSlashingWindowCall, N> {
            self.call_builder(&getSlashingWindowCall { serviceId })
        }
        ///Creates a new call builder for the [`onApprove`] function.
        pub fn onApprove(
            &self,
            operator: <ServiceOperators::OperatorPreferences as alloy::sol_types::SolType>::RustType,
            requestId: u64,
            restakingPercent: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, onApproveCall, N> {
            self.call_builder(
                &onApproveCall {
                    operator,
                    requestId,
                    restakingPercent,
                },
            )
        }
        ///Creates a new call builder for the [`onBlueprintCreated`] function.
        pub fn onBlueprintCreated(
            &self,
            blueprintId: u64,
            owner: alloy::sol_types::private::Address,
            mbsm: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, onBlueprintCreatedCall, N> {
            self.call_builder(
                &onBlueprintCreatedCall {
                    blueprintId,
                    owner,
                    mbsm,
                },
            )
        }
        ///Creates a new call builder for the [`onJobCall`] function.
        pub fn onJobCall(
            &self,
            serviceId: u64,
            job: u8,
            jobCallId: u64,
            inputs: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, onJobCallCall, N> {
            self.call_builder(
                &onJobCallCall {
                    serviceId,
                    job,
                    jobCallId,
                    inputs,
                },
            )
        }
        ///Creates a new call builder for the [`onJobResult`] function.
        pub fn onJobResult(
            &self,
            serviceId: u64,
            job: u8,
            jobCallId: u64,
            operator: <ServiceOperators::OperatorPreferences as alloy::sol_types::SolType>::RustType,
            inputs: alloy::sol_types::private::Bytes,
            outputs: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, onJobResultCall, N> {
            self.call_builder(
                &onJobResultCall {
                    serviceId,
                    job,
                    jobCallId,
                    operator,
                    inputs,
                    outputs,
                },
            )
        }
        ///Creates a new call builder for the [`onOperatorJoined`] function.
        pub fn onOperatorJoined(
            &self,
            serviceId: u64,
            operator: <ServiceOperators::OperatorPreferences as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, onOperatorJoinedCall, N> {
            self.call_builder(
                &onOperatorJoinedCall {
                    serviceId,
                    operator,
                },
            )
        }
        ///Creates a new call builder for the [`onOperatorLeft`] function.
        pub fn onOperatorLeft(
            &self,
            serviceId: u64,
            operator: <ServiceOperators::OperatorPreferences as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, onOperatorLeftCall, N> {
            self.call_builder(
                &onOperatorLeftCall {
                    serviceId,
                    operator,
                },
            )
        }
        ///Creates a new call builder for the [`onRegister`] function.
        pub fn onRegister(
            &self,
            operator: <ServiceOperators::OperatorPreferences as alloy::sol_types::SolType>::RustType,
            registrationInputs: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, onRegisterCall, N> {
            self.call_builder(
                &onRegisterCall {
                    operator,
                    registrationInputs,
                },
            )
        }
        ///Creates a new call builder for the [`onReject`] function.
        pub fn onReject(
            &self,
            operator: <ServiceOperators::OperatorPreferences as alloy::sol_types::SolType>::RustType,
            requestId: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, onRejectCall, N> {
            self.call_builder(
                &onRejectCall {
                    operator,
                    requestId,
                },
            )
        }
        ///Creates a new call builder for the [`onRequest`] function.
        pub fn onRequest(
            &self,
            params: <ServiceOperators::RequestParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, onRequestCall, N> {
            self.call_builder(&onRequestCall { params })
        }
        ///Creates a new call builder for the [`onServiceInitialized`] function.
        pub fn onServiceInitialized(
            &self,
            requestId: u64,
            serviceId: u64,
            owner: alloy::sol_types::private::Address,
            permittedCallers: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            ttl: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, onServiceInitializedCall, N> {
            self.call_builder(
                &onServiceInitializedCall {
                    requestId,
                    serviceId,
                    owner,
                    permittedCallers,
                    ttl,
                },
            )
        }
        ///Creates a new call builder for the [`onServiceTermination`] function.
        pub fn onServiceTermination(
            &self,
            serviceId: u64,
            owner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, onServiceTerminationCall, N> {
            self.call_builder(
                &onServiceTerminationCall {
                    serviceId,
                    owner,
                },
            )
        }
        ///Creates a new call builder for the [`onSlash`] function.
        pub fn onSlash(
            &self,
            serviceId: u64,
            offender: alloy::sol_types::private::Bytes,
            slashPercent: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, onSlashCall, N> {
            self.call_builder(
                &onSlashCall {
                    serviceId,
                    offender,
                    slashPercent,
                },
            )
        }
        ///Creates a new call builder for the [`onUnappliedSlash`] function.
        pub fn onUnappliedSlash(
            &self,
            serviceId: u64,
            offender: alloy::sol_types::private::Bytes,
            slashPercent: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, onUnappliedSlashCall, N> {
            self.call_builder(
                &onUnappliedSlashCall {
                    serviceId,
                    offender,
                    slashPercent,
                },
            )
        }
        ///Creates a new call builder for the [`onUnregister`] function.
        pub fn onUnregister(
            &self,
            operator: <ServiceOperators::OperatorPreferences as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, onUnregisterCall, N> {
            self.call_builder(&onUnregisterCall { operator })
        }
        ///Creates a new call builder for the [`onUpdateRpcAddress`] function.
        pub fn onUpdateRpcAddress(
            &self,
            operator: <ServiceOperators::OperatorPreferences as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, onUpdateRpcAddressCall, N> {
            self.call_builder(&onUpdateRpcAddressCall { operator })
        }
        ///Creates a new call builder for the [`queryDeveloperPaymentAddress`] function.
        pub fn queryDeveloperPaymentAddress(
            &self,
            serviceId: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, queryDeveloperPaymentAddressCall, N> {
            self.call_builder(
                &queryDeveloperPaymentAddressCall {
                    serviceId,
                },
            )
        }
        ///Creates a new call builder for the [`queryDisputeOrigin`] function.
        pub fn queryDisputeOrigin(
            &self,
            serviceId: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, queryDisputeOriginCall, N> {
            self.call_builder(
                &queryDisputeOriginCall {
                    serviceId,
                },
            )
        }
        ///Creates a new call builder for the [`queryIsPaymentAssetAllowed`] function.
        pub fn queryIsPaymentAssetAllowed(
            &self,
            serviceId: u64,
            asset: <Assets::Asset as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, queryIsPaymentAssetAllowedCall, N> {
            self.call_builder(
                &queryIsPaymentAssetAllowedCall {
                    serviceId,
                    asset,
                },
            )
        }
        ///Creates a new call builder for the [`querySlashingOrigin`] function.
        pub fn querySlashingOrigin(
            &self,
            serviceId: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, querySlashingOriginCall, N> {
            self.call_builder(
                &querySlashingOriginCall {
                    serviceId,
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
    > IBlueprintServiceManagerInstance<T, P, N> {
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
