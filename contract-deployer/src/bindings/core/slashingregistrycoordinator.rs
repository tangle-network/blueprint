#![allow(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    warnings,
    unknown_lints,
    rustdoc::all,
    elided_lifetimes_in_paths
)]
use SlashingRegistryCoordinator::*;

///Module containing a contract's types and functions.
/**

```solidity
library BN254 {
    struct G1Point { uint256 X; uint256 Y; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod BN254 {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
    struct G1Point { uint256 X; uint256 Y; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct G1Point {
        #[allow(missing_docs)]
        pub X: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub Y: alloy::sol_types::private::primitives::aliases::U256,
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<G1Point> for UnderlyingRustTuple<'_> {
            fn from(value: G1Point) -> Self {
                (value.X, value.Y)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for G1Point {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    X: tuple.0,
                    Y: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for G1Point {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for G1Point {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.X,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.Y,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for G1Point {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for G1Point {
            const NAME: &'static str = "G1Point";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed("G1Point(uint256 X,uint256 Y)")
            }
            #[inline]
            fn eip712_components()
            -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.X)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.Y)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for G1Point {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.X)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.Y)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.X, out);
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.Y, out);
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`BN254`](self) contract instance.

    See the [wrapper's documentation](`BN254Instance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> BN254Instance<T, P, N> {
        BN254Instance::<T, P, N>::new(address, provider)
    }
    /**A [`BN254`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`BN254`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct BN254Instance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for BN254Instance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("BN254Instance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > BN254Instance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`BN254`](self) contract instance.

        See the [wrapper's documentation](`BN254Instance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
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
    impl<T, P: ::core::clone::Clone, N> BN254Instance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> BN254Instance<T, P, N> {
            BN254Instance {
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
    > BN254Instance<T, P, N>
    {
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
    > BN254Instance<T, P, N>
    {
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
library ISlashingRegistryCoordinatorTypes {
    type OperatorStatus is uint8;
    struct OperatorInfo { bytes32 operatorId; OperatorStatus status; }
    struct OperatorKickParam { uint8 quorumNumber; address operator; }
    struct OperatorSetParam { uint32 maxOperatorCount; uint16 kickBIPsOfOperatorStake; uint16 kickBIPsOfTotalStake; }
    struct QuorumBitmapUpdate { uint32 updateBlockNumber; uint32 nextUpdateBlockNumber; uint192 quorumBitmap; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod ISlashingRegistryCoordinatorTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorStatus(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<OperatorStatus> for u8 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::Token<'_>
            {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(self).0
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::abi_encoded_size(
                    self,
                )
            }
        }
        #[automatically_derived]
        impl OperatorStatus {
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
        impl alloy_sol_types::SolType for OperatorStatus {
            type RustType = u8;
            type Token<'a> =
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> =
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OperatorStatus {
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::EventTopic>::encode_topic(
                    rust,
                )
            }
        }
    };
    /**```solidity
    struct OperatorInfo { bytes32 operatorId; OperatorStatus status; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorInfo {
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub status: <OperatorStatus as alloy::sol_types::SolType>::RustType,
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
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>, OperatorStatus);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
            <OperatorStatus as alloy::sol_types::SolType>::RustType,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OperatorInfo> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorInfo) -> Self {
                (value.operatorId, value.status)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorInfo {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    operatorId: tuple.0,
                    status: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OperatorInfo {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OperatorInfo {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
                    <OperatorStatus as alloy_sol_types::SolType>::tokenize(&self.status),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for OperatorInfo {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for OperatorInfo {
            const NAME: &'static str = "OperatorInfo";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OperatorInfo(bytes32 operatorId,uint8 status)",
                )
            }
            #[inline]
            fn eip712_components()
            -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.operatorId)
                        .0,
                    <OperatorStatus as alloy_sol_types::SolType>::eip712_data_word(
                            &self.status,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OperatorInfo {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operatorId,
                    )
                    + <OperatorStatus as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.status,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operatorId,
                    out,
                );
                <OperatorStatus as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.status,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    /**```solidity
    struct OperatorKickParam { uint8 quorumNumber; address operator; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorKickParam {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
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
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<8>,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u8, alloy::sol_types::private::Address);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OperatorKickParam> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorKickParam) -> Self {
                (value.quorumNumber, value.operator)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorKickParam {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    quorumNumber: tuple.0,
                    operator: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OperatorKickParam {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OperatorKickParam {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumber,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for OperatorKickParam {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for OperatorKickParam {
            const NAME: &'static str = "OperatorKickParam";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OperatorKickParam(uint8 quorumNumber,address operator)",
                )
            }
            #[inline]
            fn eip712_components()
            -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.quorumNumber)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.operator,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OperatorKickParam {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.quorumNumber,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operator,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.quorumNumber,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operator,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    /**```solidity
    struct OperatorSetParam { uint32 maxOperatorCount; uint16 kickBIPsOfOperatorStake; uint16 kickBIPsOfTotalStake; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorSetParam {
        #[allow(missing_docs)]
        pub maxOperatorCount: u32,
        #[allow(missing_docs)]
        pub kickBIPsOfOperatorStake: u16,
        #[allow(missing_docs)]
        pub kickBIPsOfTotalStake: u16,
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
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<16>,
            alloy::sol_types::sol_data::Uint<16>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u32, u16, u16);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OperatorSetParam> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorSetParam) -> Self {
                (
                    value.maxOperatorCount,
                    value.kickBIPsOfOperatorStake,
                    value.kickBIPsOfTotalStake,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorSetParam {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    maxOperatorCount: tuple.0,
                    kickBIPsOfOperatorStake: tuple.1,
                    kickBIPsOfTotalStake: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OperatorSetParam {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OperatorSetParam {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.maxOperatorCount,
                    ),
                    <alloy::sol_types::sol_data::Uint<16> as alloy_sol_types::SolType>::tokenize(
                        &self.kickBIPsOfOperatorStake,
                    ),
                    <alloy::sol_types::sol_data::Uint<16> as alloy_sol_types::SolType>::tokenize(
                        &self.kickBIPsOfTotalStake,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for OperatorSetParam {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for OperatorSetParam {
            const NAME: &'static str = "OperatorSetParam";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OperatorSetParam(uint32 maxOperatorCount,uint16 kickBIPsOfOperatorStake,uint16 kickBIPsOfTotalStake)",
                )
            }
            #[inline]
            fn eip712_components()
            -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
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
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.maxOperatorCount,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.kickBIPsOfOperatorStake,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.kickBIPsOfTotalStake,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OperatorSetParam {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.maxOperatorCount,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.kickBIPsOfOperatorStake,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.kickBIPsOfTotalStake,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.maxOperatorCount,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    16,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.kickBIPsOfOperatorStake,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    16,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.kickBIPsOfTotalStake,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    /**```solidity
    struct QuorumBitmapUpdate { uint32 updateBlockNumber; uint32 nextUpdateBlockNumber; uint192 quorumBitmap; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct QuorumBitmapUpdate {
        #[allow(missing_docs)]
        pub updateBlockNumber: u32,
        #[allow(missing_docs)]
        pub nextUpdateBlockNumber: u32,
        #[allow(missing_docs)]
        pub quorumBitmap: alloy::sol_types::private::primitives::aliases::U192,
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
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<192>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u32,
            u32,
            alloy::sol_types::private::primitives::aliases::U192,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<QuorumBitmapUpdate> for UnderlyingRustTuple<'_> {
            fn from(value: QuorumBitmapUpdate) -> Self {
                (
                    value.updateBlockNumber,
                    value.nextUpdateBlockNumber,
                    value.quorumBitmap,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for QuorumBitmapUpdate {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    updateBlockNumber: tuple.0,
                    nextUpdateBlockNumber: tuple.1,
                    quorumBitmap: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for QuorumBitmapUpdate {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for QuorumBitmapUpdate {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.updateBlockNumber,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.nextUpdateBlockNumber,
                    ),
                    <alloy::sol_types::sol_data::Uint<192> as alloy_sol_types::SolType>::tokenize(
                        &self.quorumBitmap,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for QuorumBitmapUpdate {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for QuorumBitmapUpdate {
            const NAME: &'static str = "QuorumBitmapUpdate";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "QuorumBitmapUpdate(uint32 updateBlockNumber,uint32 nextUpdateBlockNumber,uint192 quorumBitmap)",
                )
            }
            #[inline]
            fn eip712_components()
            -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
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
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.updateBlockNumber,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.nextUpdateBlockNumber,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        192,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.quorumBitmap)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for QuorumBitmapUpdate {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.updateBlockNumber,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.nextUpdateBlockNumber,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        192,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.quorumBitmap,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.updateBlockNumber,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nextUpdateBlockNumber,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    192,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.quorumBitmap,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ISlashingRegistryCoordinatorTypes`](self) contract instance.

    See the [wrapper's documentation](`ISlashingRegistryCoordinatorTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ISlashingRegistryCoordinatorTypesInstance<T, P, N> {
        ISlashingRegistryCoordinatorTypesInstance::<T, P, N>::new(address, provider)
    }
    /**A [`ISlashingRegistryCoordinatorTypes`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`ISlashingRegistryCoordinatorTypes`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ISlashingRegistryCoordinatorTypesInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ISlashingRegistryCoordinatorTypesInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ISlashingRegistryCoordinatorTypesInstance")
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
    > ISlashingRegistryCoordinatorTypesInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`ISlashingRegistryCoordinatorTypes`](self) contract instance.

        See the [wrapper's documentation](`ISlashingRegistryCoordinatorTypesInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
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
    impl<T, P: ::core::clone::Clone, N> ISlashingRegistryCoordinatorTypesInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ISlashingRegistryCoordinatorTypesInstance<T, P, N> {
            ISlashingRegistryCoordinatorTypesInstance {
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
    > ISlashingRegistryCoordinatorTypesInstance<T, P, N>
    {
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
    > ISlashingRegistryCoordinatorTypesInstance<T, P, N>
    {
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
library IStakeRegistryTypes {
    type StakeType is uint8;
    struct StrategyParams { address strategy; uint96 multiplier; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IStakeRegistryTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StakeType(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<StakeType> for u8 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::Token<'_>
            {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(self).0
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::abi_encoded_size(
                    self,
                )
            }
        }
        #[automatically_derived]
        impl StakeType {
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
        impl alloy_sol_types::SolType for StakeType {
            type RustType = u8;
            type Token<'a> =
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> =
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for StakeType {
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::EventTopic>::encode_topic(
                    rust,
                )
            }
        }
    };
    /**```solidity
    struct StrategyParams { address strategy; uint96 multiplier; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StrategyParams {
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub multiplier: alloy::sol_types::private::primitives::aliases::U96,
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
            alloy::sol_types::sol_data::Uint<96>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U96,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<StrategyParams> for UnderlyingRustTuple<'_> {
            fn from(value: StrategyParams) -> Self {
                (value.strategy, value.multiplier)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StrategyParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    strategy: tuple.0,
                    multiplier: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for StrategyParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for StrategyParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<96> as alloy_sol_types::SolType>::tokenize(
                        &self.multiplier,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for StrategyParams {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for StrategyParams {
            const NAME: &'static str = "StrategyParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "StrategyParams(address strategy,uint96 multiplier)",
                )
            }
            #[inline]
            fn eip712_components()
            -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
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
                            &self.strategy,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.multiplier)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for StrategyParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategy,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.multiplier,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.strategy,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    96,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.multiplier,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IStakeRegistryTypes`](self) contract instance.

    See the [wrapper's documentation](`IStakeRegistryTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IStakeRegistryTypesInstance<T, P, N> {
        IStakeRegistryTypesInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IStakeRegistryTypes`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`IStakeRegistryTypes`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IStakeRegistryTypesInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IStakeRegistryTypesInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IStakeRegistryTypesInstance")
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
    > IStakeRegistryTypesInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`IStakeRegistryTypes`](self) contract instance.

        See the [wrapper's documentation](`IStakeRegistryTypesInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
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
    impl<T, P: ::core::clone::Clone, N> IStakeRegistryTypesInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IStakeRegistryTypesInstance<T, P, N> {
            IStakeRegistryTypesInstance {
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
    > IStakeRegistryTypesInstance<T, P, N>
    {
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
    > IStakeRegistryTypesInstance<T, P, N>
    {
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
library BN254 {
    struct G1Point {
        uint256 X;
        uint256 Y;
    }
}

library ISlashingRegistryCoordinatorTypes {
    type OperatorStatus is uint8;
    struct OperatorInfo {
        bytes32 operatorId;
        OperatorStatus status;
    }
    struct OperatorKickParam {
        uint8 quorumNumber;
        address operator;
    }
    struct OperatorSetParam {
        uint32 maxOperatorCount;
        uint16 kickBIPsOfOperatorStake;
        uint16 kickBIPsOfTotalStake;
    }
    struct QuorumBitmapUpdate {
        uint32 updateBlockNumber;
        uint32 nextUpdateBlockNumber;
        uint192 quorumBitmap;
    }
}

library IStakeRegistryTypes {
    type StakeType is uint8;
    struct StrategyParams {
        address strategy;
        uint96 multiplier;
    }
}

interface SlashingRegistryCoordinator {
    error AlreadyRegisteredForQuorums();
    error BitmapCannotBeZero();
    error BitmapEmpty();
    error BitmapUpdateIsAfterBlockNumber();
    error BitmapValueTooLarge();
    error BytesArrayLengthTooLong();
    error BytesArrayNotOrdered();
    error CannotChurnSelf();
    error CannotKickOperatorAboveThreshold();
    error CannotReregisterYet();
    error ChurnApproverSaltUsed();
    error CurrentlyPaused();
    error ExpModFailed();
    error InputAddressZero();
    error InputLengthMismatch();
    error InsufficientStakeForChurn();
    error InvalidAVS();
    error InvalidNewPausedStatus();
    error InvalidRegistrationType();
    error InvalidShortString();
    error InvalidSignature();
    error LookAheadPeriodTooLong();
    error MaxOperatorCountReached();
    error MaxQuorumsReached();
    error NextBitmapUpdateIsBeforeBlockNumber();
    error NotRegistered();
    error NotRegisteredForQuorum();
    error NotSorted();
    error OnlyAllocationManager();
    error OnlyEjector();
    error OnlyPauser();
    error OnlyUnpauser();
    error OperatorNotRegistered();
    error QuorumDoesNotExist();
    error QuorumOperatorCountMismatch();
    error SignatureExpired();
    error StringTooLong(string str);

    event AVSUpdated(address prevAVS, address newAVS);
    event ChurnApproverUpdated(address prevChurnApprover, address newChurnApprover);
    event EIP712DomainChanged();
    event EjectionCooldownUpdated(uint256 prevEjectionCooldown, uint256 newEjectionCooldown);
    event EjectorUpdated(address prevEjector, address newEjector);
    event Initialized(uint8 version);
    event OperatorDeregistered(address indexed operator, bytes32 indexed operatorId);
    event OperatorRegistered(address indexed operator, bytes32 indexed operatorId);
    event OperatorSetParamsUpdated(uint8 indexed quorumNumber, ISlashingRegistryCoordinatorTypes.OperatorSetParam operatorSetParams);
    event OperatorSocketUpdate(bytes32 indexed operatorId, string socket);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event Paused(address indexed account, uint256 newPausedStatus);
    event QuorumBlockNumberUpdated(uint8 indexed quorumNumber, uint256 blocknumber);
    event QuorumCreated(uint8 indexed quorumNumber, ISlashingRegistryCoordinatorTypes.OperatorSetParam operatorSetParams, uint96 minimumStake, IStakeRegistryTypes.StrategyParams[] strategyParams, IStakeRegistryTypes.StakeType stakeType, uint32 lookAheadPeriod);
    event Unpaused(address indexed account, uint256 newPausedStatus);

    constructor(address _stakeRegistry, address _blsApkRegistry, address _indexRegistry, address _socketRegistry, address _allocationManager, address _pauserRegistry, string _version);

    function OPERATOR_CHURN_APPROVAL_TYPEHASH() external view returns (bytes32);
    function PUBKEY_REGISTRATION_TYPEHASH() external view returns (bytes32);
    function allocationManager() external view returns (address);
    function avs() external view returns (address);
    function blsApkRegistry() external view returns (address);
    function calculateOperatorChurnApprovalDigestHash(address registeringOperator, bytes32 registeringOperatorId, ISlashingRegistryCoordinatorTypes.OperatorKickParam[] memory operatorKickParams, bytes32 salt, uint256 expiry) external view returns (bytes32);
    function calculatePubkeyRegistrationMessageHash(address operator) external view returns (bytes32);
    function churnApprover() external view returns (address);
    function createSlashableStakeQuorum(ISlashingRegistryCoordinatorTypes.OperatorSetParam memory operatorSetParams, uint96 minimumStake, IStakeRegistryTypes.StrategyParams[] memory strategyParams, uint32 lookAheadPeriod) external;
    function createTotalDelegatedStakeQuorum(ISlashingRegistryCoordinatorTypes.OperatorSetParam memory operatorSetParams, uint96 minimumStake, IStakeRegistryTypes.StrategyParams[] memory strategyParams) external;
    function deregisterOperator(address operator, address avs, uint32[] memory operatorSetIds) external;
    function domainSeparator() external view returns (bytes32);
    function eip712Domain() external view returns (bytes1 fields, string memory name, string memory version, uint256 chainId, address verifyingContract, bytes32 salt, uint256[] memory extensions);
    function ejectOperator(address operator, bytes memory quorumNumbers) external;
    function ejectionCooldown() external view returns (uint256);
    function ejector() external view returns (address);
    function getCurrentQuorumBitmap(bytes32 operatorId) external view returns (uint192);
    function getOperator(address operator) external view returns (ISlashingRegistryCoordinatorTypes.OperatorInfo memory);
    function getOperatorFromId(bytes32 operatorId) external view returns (address);
    function getOperatorId(address operator) external view returns (bytes32);
    function getOperatorSetParams(uint8 quorumNumber) external view returns (ISlashingRegistryCoordinatorTypes.OperatorSetParam memory);
    function getOperatorStatus(address operator) external view returns (ISlashingRegistryCoordinatorTypes.OperatorStatus);
    function getQuorumBitmapAtBlockNumberByIndex(bytes32 operatorId, uint32 blockNumber, uint256 index) external view returns (uint192);
    function getQuorumBitmapHistoryLength(bytes32 operatorId) external view returns (uint256);
    function getQuorumBitmapIndicesAtBlockNumber(uint32 blockNumber, bytes32[] memory operatorIds) external view returns (uint32[] memory);
    function getQuorumBitmapUpdateByIndex(bytes32 operatorId, uint256 index) external view returns (ISlashingRegistryCoordinatorTypes.QuorumBitmapUpdate memory);
    function indexRegistry() external view returns (address);
    function initialize(address initialOwner, address churnApprover, address ejector, uint256 initialPausedStatus, address avs) external;
    function isChurnApproverSaltUsed(bytes32) external view returns (bool);
    function lastEjectionTimestamp(address) external view returns (uint256);
    function owner() external view returns (address);
    function pause(uint256 newPausedStatus) external;
    function pauseAll() external;
    function paused(uint8 index) external view returns (bool);
    function paused() external view returns (uint256);
    function pauserRegistry() external view returns (address);
    function pubkeyRegistrationMessageHash(address operator) external view returns (BN254.G1Point memory);
    function quorumCount() external view returns (uint8);
    function quorumUpdateBlockNumber(uint8) external view returns (uint256);
    function registerOperator(address operator, address avs, uint32[] memory operatorSetIds, bytes memory data) external;
    function renounceOwnership() external;
    function setAVS(address _avs) external;
    function setChurnApprover(address _churnApprover) external;
    function setEjectionCooldown(uint256 _ejectionCooldown) external;
    function setEjector(address _ejector) external;
    function setOperatorSetParams(uint8 quorumNumber, ISlashingRegistryCoordinatorTypes.OperatorSetParam memory operatorSetParams) external;
    function socketRegistry() external view returns (address);
    function stakeRegistry() external view returns (address);
    function supportsAVS(address _avs) external view returns (bool);
    function transferOwnership(address newOwner) external;
    function unpause(uint256 newPausedStatus) external;
    function updateOperators(address[] memory operators) external;
    function updateOperatorsForQuorum(address[][] memory operatorsPerQuorum, bytes memory quorumNumbers) external;
    function updateSocket(string memory socket) external;
    function version() external view returns (string memory);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_stakeRegistry",
        "type": "address",
        "internalType": "contract IStakeRegistry"
      },
      {
        "name": "_blsApkRegistry",
        "type": "address",
        "internalType": "contract IBLSApkRegistry"
      },
      {
        "name": "_indexRegistry",
        "type": "address",
        "internalType": "contract IIndexRegistry"
      },
      {
        "name": "_socketRegistry",
        "type": "address",
        "internalType": "contract ISocketRegistry"
      },
      {
        "name": "_allocationManager",
        "type": "address",
        "internalType": "contract IAllocationManager"
      },
      {
        "name": "_pauserRegistry",
        "type": "address",
        "internalType": "contract IPauserRegistry"
      },
      {
        "name": "_version",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "OPERATOR_CHURN_APPROVAL_TYPEHASH",
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
    "name": "PUBKEY_REGISTRATION_TYPEHASH",
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
    "name": "avs",
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
    "name": "blsApkRegistry",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IBLSApkRegistry"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "calculateOperatorChurnApprovalDigestHash",
    "inputs": [
      {
        "name": "registeringOperator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "registeringOperatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "operatorKickParams",
        "type": "tuple[]",
        "internalType": "struct ISlashingRegistryCoordinatorTypes.OperatorKickParam[]",
        "components": [
          {
            "name": "quorumNumber",
            "type": "uint8",
            "internalType": "uint8"
          },
          {
            "name": "operator",
            "type": "address",
            "internalType": "address"
          }
        ]
      },
      {
        "name": "salt",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "expiry",
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "calculatePubkeyRegistrationMessageHash",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
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
    "name": "churnApprover",
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
    "name": "createSlashableStakeQuorum",
    "inputs": [
      {
        "name": "operatorSetParams",
        "type": "tuple",
        "internalType": "struct ISlashingRegistryCoordinatorTypes.OperatorSetParam",
        "components": [
          {
            "name": "maxOperatorCount",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "kickBIPsOfOperatorStake",
            "type": "uint16",
            "internalType": "uint16"
          },
          {
            "name": "kickBIPsOfTotalStake",
            "type": "uint16",
            "internalType": "uint16"
          }
        ]
      },
      {
        "name": "minimumStake",
        "type": "uint96",
        "internalType": "uint96"
      },
      {
        "name": "strategyParams",
        "type": "tuple[]",
        "internalType": "struct IStakeRegistryTypes.StrategyParams[]",
        "components": [
          {
            "name": "strategy",
            "type": "address",
            "internalType": "contract IStrategy"
          },
          {
            "name": "multiplier",
            "type": "uint96",
            "internalType": "uint96"
          }
        ]
      },
      {
        "name": "lookAheadPeriod",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "createTotalDelegatedStakeQuorum",
    "inputs": [
      {
        "name": "operatorSetParams",
        "type": "tuple",
        "internalType": "struct ISlashingRegistryCoordinatorTypes.OperatorSetParam",
        "components": [
          {
            "name": "maxOperatorCount",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "kickBIPsOfOperatorStake",
            "type": "uint16",
            "internalType": "uint16"
          },
          {
            "name": "kickBIPsOfTotalStake",
            "type": "uint16",
            "internalType": "uint16"
          }
        ]
      },
      {
        "name": "minimumStake",
        "type": "uint96",
        "internalType": "uint96"
      },
      {
        "name": "strategyParams",
        "type": "tuple[]",
        "internalType": "struct IStakeRegistryTypes.StrategyParams[]",
        "components": [
          {
            "name": "strategy",
            "type": "address",
            "internalType": "contract IStrategy"
          },
          {
            "name": "multiplier",
            "type": "uint96",
            "internalType": "uint96"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "deregisterOperator",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "avs",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "operatorSetIds",
        "type": "uint32[]",
        "internalType": "uint32[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "domainSeparator",
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
    "name": "ejectOperator",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "quorumNumbers",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "ejectionCooldown",
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
    "name": "ejector",
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
    "name": "getCurrentQuorumBitmap",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint192",
        "internalType": "uint192"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperator",
    "inputs": [
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
        "internalType": "struct ISlashingRegistryCoordinatorTypes.OperatorInfo",
        "components": [
          {
            "name": "operatorId",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "status",
            "type": "uint8",
            "internalType": "enum ISlashingRegistryCoordinatorTypes.OperatorStatus"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorFromId",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
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
    "name": "getOperatorId",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
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
    "name": "getOperatorSetParams",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct ISlashingRegistryCoordinatorTypes.OperatorSetParam",
        "components": [
          {
            "name": "maxOperatorCount",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "kickBIPsOfOperatorStake",
            "type": "uint16",
            "internalType": "uint16"
          },
          {
            "name": "kickBIPsOfTotalStake",
            "type": "uint16",
            "internalType": "uint16"
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
        "name": "operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "enum ISlashingRegistryCoordinatorTypes.OperatorStatus"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getQuorumBitmapAtBlockNumberByIndex",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "blockNumber",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "index",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint192",
        "internalType": "uint192"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getQuorumBitmapHistoryLength",
    "inputs": [
      {
        "name": "operatorId",
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
    "name": "getQuorumBitmapIndicesAtBlockNumber",
    "inputs": [
      {
        "name": "blockNumber",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "operatorIds",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint32[]",
        "internalType": "uint32[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getQuorumBitmapUpdateByIndex",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "index",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct ISlashingRegistryCoordinatorTypes.QuorumBitmapUpdate",
        "components": [
          {
            "name": "updateBlockNumber",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "nextUpdateBlockNumber",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "quorumBitmap",
            "type": "uint192",
            "internalType": "uint192"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "indexRegistry",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IIndexRegistry"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "initialize",
    "inputs": [
      {
        "name": "initialOwner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "churnApprover",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "ejector",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "initialPausedStatus",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "avs",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "isChurnApproverSaltUsed",
    "inputs": [
      {
        "name": "",
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
    "name": "lastEjectionTimestamp",
    "inputs": [
      {
        "name": "",
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
    "name": "owner",
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
    "name": "pause",
    "inputs": [
      {
        "name": "newPausedStatus",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "pauseAll",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "paused",
    "inputs": [
      {
        "name": "index",
        "type": "uint8",
        "internalType": "uint8"
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
    "name": "paused",
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
    "name": "pauserRegistry",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IPauserRegistry"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "pubkeyRegistrationMessageHash",
    "inputs": [
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
        "internalType": "struct BN254.G1Point",
        "components": [
          {
            "name": "X",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "Y",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "quorumCount",
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
    "name": "quorumUpdateBlockNumber",
    "inputs": [
      {
        "name": "",
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
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "registerOperator",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "avs",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "operatorSetIds",
        "type": "uint32[]",
        "internalType": "uint32[]"
      },
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "renounceOwnership",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setAVS",
    "inputs": [
      {
        "name": "_avs",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setChurnApprover",
    "inputs": [
      {
        "name": "_churnApprover",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setEjectionCooldown",
    "inputs": [
      {
        "name": "_ejectionCooldown",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setEjector",
    "inputs": [
      {
        "name": "_ejector",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setOperatorSetParams",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "operatorSetParams",
        "type": "tuple",
        "internalType": "struct ISlashingRegistryCoordinatorTypes.OperatorSetParam",
        "components": [
          {
            "name": "maxOperatorCount",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "kickBIPsOfOperatorStake",
            "type": "uint16",
            "internalType": "uint16"
          },
          {
            "name": "kickBIPsOfTotalStake",
            "type": "uint16",
            "internalType": "uint16"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "socketRegistry",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract ISocketRegistry"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "stakeRegistry",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IStakeRegistry"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "supportsAVS",
    "inputs": [
      {
        "name": "_avs",
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
    "name": "transferOwnership",
    "inputs": [
      {
        "name": "newOwner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "unpause",
    "inputs": [
      {
        "name": "newPausedStatus",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateOperators",
    "inputs": [
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
    "name": "updateOperatorsForQuorum",
    "inputs": [
      {
        "name": "operatorsPerQuorum",
        "type": "address[][]",
        "internalType": "address[][]"
      },
      {
        "name": "quorumNumbers",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateSocket",
    "inputs": [
      {
        "name": "socket",
        "type": "string",
        "internalType": "string"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
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
    "type": "event",
    "name": "AVSUpdated",
    "inputs": [
      {
        "name": "prevAVS",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "newAVS",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ChurnApproverUpdated",
    "inputs": [
      {
        "name": "prevChurnApprover",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "newChurnApprover",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "EIP712DomainChanged",
    "inputs": [],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "EjectionCooldownUpdated",
    "inputs": [
      {
        "name": "prevEjectionCooldown",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newEjectionCooldown",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "EjectorUpdated",
    "inputs": [
      {
        "name": "prevEjector",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "newEjector",
        "type": "address",
        "indexed": false,
        "internalType": "address"
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
        "type": "uint8",
        "indexed": false,
        "internalType": "uint8"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorDeregistered",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "operatorId",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorRegistered",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "operatorId",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorSetParamsUpdated",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "indexed": true,
        "internalType": "uint8"
      },
      {
        "name": "operatorSetParams",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct ISlashingRegistryCoordinatorTypes.OperatorSetParam",
        "components": [
          {
            "name": "maxOperatorCount",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "kickBIPsOfOperatorStake",
            "type": "uint16",
            "internalType": "uint16"
          },
          {
            "name": "kickBIPsOfTotalStake",
            "type": "uint16",
            "internalType": "uint16"
          }
        ]
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorSocketUpdate",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "socket",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OwnershipTransferred",
    "inputs": [
      {
        "name": "previousOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Paused",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newPausedStatus",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "QuorumBlockNumberUpdated",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "indexed": true,
        "internalType": "uint8"
      },
      {
        "name": "blocknumber",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "QuorumCreated",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "indexed": true,
        "internalType": "uint8"
      },
      {
        "name": "operatorSetParams",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct ISlashingRegistryCoordinatorTypes.OperatorSetParam",
        "components": [
          {
            "name": "maxOperatorCount",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "kickBIPsOfOperatorStake",
            "type": "uint16",
            "internalType": "uint16"
          },
          {
            "name": "kickBIPsOfTotalStake",
            "type": "uint16",
            "internalType": "uint16"
          }
        ]
      },
      {
        "name": "minimumStake",
        "type": "uint96",
        "indexed": false,
        "internalType": "uint96"
      },
      {
        "name": "strategyParams",
        "type": "tuple[]",
        "indexed": false,
        "internalType": "struct IStakeRegistryTypes.StrategyParams[]",
        "components": [
          {
            "name": "strategy",
            "type": "address",
            "internalType": "contract IStrategy"
          },
          {
            "name": "multiplier",
            "type": "uint96",
            "internalType": "uint96"
          }
        ]
      },
      {
        "name": "stakeType",
        "type": "uint8",
        "indexed": false,
        "internalType": "enum IStakeRegistryTypes.StakeType"
      },
      {
        "name": "lookAheadPeriod",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Unpaused",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newPausedStatus",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "AlreadyRegisteredForQuorums",
    "inputs": []
  },
  {
    "type": "error",
    "name": "BitmapCannotBeZero",
    "inputs": []
  },
  {
    "type": "error",
    "name": "BitmapEmpty",
    "inputs": []
  },
  {
    "type": "error",
    "name": "BitmapUpdateIsAfterBlockNumber",
    "inputs": []
  },
  {
    "type": "error",
    "name": "BitmapValueTooLarge",
    "inputs": []
  },
  {
    "type": "error",
    "name": "BytesArrayLengthTooLong",
    "inputs": []
  },
  {
    "type": "error",
    "name": "BytesArrayNotOrdered",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CannotChurnSelf",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CannotKickOperatorAboveThreshold",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CannotReregisterYet",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ChurnApproverSaltUsed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CurrentlyPaused",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ExpModFailed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InputAddressZero",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InputLengthMismatch",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InsufficientStakeForChurn",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidAVS",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidNewPausedStatus",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidRegistrationType",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidShortString",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidSignature",
    "inputs": []
  },
  {
    "type": "error",
    "name": "LookAheadPeriodTooLong",
    "inputs": []
  },
  {
    "type": "error",
    "name": "MaxOperatorCountReached",
    "inputs": []
  },
  {
    "type": "error",
    "name": "MaxQuorumsReached",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NextBitmapUpdateIsBeforeBlockNumber",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotRegistered",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotRegisteredForQuorum",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotSorted",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyAllocationManager",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyEjector",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyPauser",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyUnpauser",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OperatorNotRegistered",
    "inputs": []
  },
  {
    "type": "error",
    "name": "QuorumDoesNotExist",
    "inputs": []
  },
  {
    "type": "error",
    "name": "QuorumOperatorCountMismatch",
    "inputs": []
  },
  {
    "type": "error",
    "name": "SignatureExpired",
    "inputs": []
  },
  {
    "type": "error",
    "name": "StringTooLong",
    "inputs": [
      {
        "name": "str",
        "type": "string",
        "internalType": "string"
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
pub mod SlashingRegistryCoordinator {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x610160604052348015610010575f5ffd5b50604051615b88380380615b8883398101604081905261002f916101d9565b6001600160a01b0380881660c05280871660a05280861660e0528085166080528316610100528181610060816100ad565b61012052506001600160a01b03811661008c576040516339b190bb60e11b815260040160405180910390fd5b6001600160a01b0316610140526100a16100f3565b50505050505050610357565b5f5f829050601f815111156100e0578260405163305a27a960e01b81526004016100d791906102fc565b60405180910390fd5b80516100eb82610331565b179392505050565b603254610100900460ff161561015b5760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b60648201526084016100d7565b60325460ff908116146101ac576032805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b6001600160a01b03811681146101c2575f5ffd5b50565b634e487b7160e01b5f52604160045260245ffd5b5f5f5f5f5f5f5f60e0888a0312156101ef575f5ffd5b87516101fa816101ae565b602089015190975061020b816101ae565b604089015190965061021c816101ae565b606089015190955061022d816101ae565b608089015190945061023e816101ae565b60a089015190935061024f816101ae565b60c08901519092506001600160401b0381111561026a575f5ffd5b8801601f81018a1361027a575f5ffd5b80516001600160401b03811115610293576102936101c5565b604051601f8201601f19908116603f011681016001600160401b03811182821017156102c1576102c16101c5565b6040528181528282016020018c10156102d8575f5ffd5b8160208401602083015e5f6020838301015280935050505092959891949750929550565b602081525f82518060208401528060208501604085015e5f604082850101526040601f19601f83011684010191505092915050565b80516020808301519190811015610351575f198160200360031b1b821691505b50919050565b60805160a05160c05160e05161010051610120516101405161573f6104495f395f818161069b01528181610b87015281816113aa015261185b01525f6112f801525f81816107c801528181611e7f015281816124310152818161255c015261355a01525f81816106f101528181610f2e015281816121b1015281816126960152612f9d01525f81816105f201528181611aaa01528181612136015281816124cf0152818161261501528181612efb015261382501525f81816105cb01528181610cf0015281816120bd0152818161270b01528181612c100152612e8201525f818161087d0152611c77015261573f5ff3fe608060405234801561000f575f5ffd5b50600436106102e3575f3560e01c80635df459461161018d578063a65497c6116100e0578063de1164bb1161008f578063de1164bb146107ea578063e65797ad146107fd578063ea32afae14610878578063f2fde38b1461089f578063f698da25146108b2578063fabc1cbc146108ba578063fd39105a146108cd575f5ffd5b8063a65497c61461073a578063a96f783e1461074d578063b526578714610756578063c391425e14610769578063c63fd50214610789578063ca0de8821461079c578063ca8aa7c7146107c3575f5ffd5b806384ca52131161013c57806384ca521314610670578063871ef04914610683578063886f1195146106965780638da5cb5b146106bd5780639aa1653d146106ce5780639e9923c2146106ec5780639feab85914610713575f5ffd5b80635df45946146105c657806368304835146105ed5780636e3b17db14610614578063715018a614610627578063734479921461062f5780638281ab751461064257806384b0196e14610655575f5ffd5b8063296bb06411610245578063530b97a4116101f4578063530b97a41461053857806354fd4d501461054b5780635865c60c14610560578063595c6a67146105805780635ac86ab7146105885780635b0b829f146105ab5780635c975abb146105be575f5ffd5b8063296bb064146104a657806329d1e0c3146104b95780632cdd1e86146104cc578063303ca956146104df5780633c2a7f4c146104f25780633eef3a51146105125780635140a54814610525575f5ffd5b8063125e0584116102a1578063125e05841461039f57806313542a4e146103be578063136439dd146103e65780631478851f146103f95780631eb812da1461042b578063249a0c421461047457806328f61b3114610493575f5ffd5b8062cf2ab5146102e757806303fd3492146102fc57806304ec63511461032e578063054310e6146103595780630cf4b767146103795780630d3f21341461038c575b5f5ffd5b6102fa6102f53660046140f2565b610908565b005b61031b61030a366004614123565b5f9081526002602052604090205490565b6040519081526020015b60405180910390f35b61034161033c36600461414b565b610aa9565b6040516001600160c01b039091168152602001610325565b60075461036c906001600160a01b031681565b6040516103259190614180565b6102fa610387366004614205565b610ac1565b6102fa61039a366004614123565b610b23565b61031b6103ad366004614236565b60096020525f908152604090205481565b61031b6103cc366004614236565b6001600160a01b03165f9081526003602052604090205490565b6102fa6103f4366004614123565b610b70565b61041b610407366004614123565b60046020525f908152604090205460ff1681565b6040519015158152602001610325565b61043e610439366004614251565b610c4a565b60408051825163ffffffff908116825260208085015190911690820152918101516001600160c01b031690820152606001610325565b61031b610482366004614286565b60056020525f908152604090205481565b60085461036c906001600160a01b031681565b61036c6104b4366004614123565b610cd8565b6102fa6104c7366004614236565b610d61565b6102fa6104da366004614236565b610d72565b6102fa6104ed366004614303565b610d83565b610505610500366004614236565b610df7565b6040516103259190614360565b6102fa610520366004614481565b610e1b565b6102fa61053336600461452d565b610e37565b6102fa61054636600461460a565b61115b565b6105536112f1565b604051610325919061469c565b61057361056e366004614236565b611321565b60405161032591906146d6565b6102fa611393565b61041b610596366004614286565b603354600160ff9092169190911b9081161490565b6102fa6105b93660046146f1565b611447565b60335461031b565b61036c7f000000000000000000000000000000000000000000000000000000000000000081565b61036c7f000000000000000000000000000000000000000000000000000000000000000081565b6102fa610622366004614723565b611463565b6102fa61148f565b61031b61063d366004614236565b6114a0565b6102fa61065036600461476f565b611504565b61065d611519565b60405161032597969594939291906147c1565b61031b61067e3660046148eb565b6115b2565b610341610691366004614123565b6115fb565b61036c7f000000000000000000000000000000000000000000000000000000000000000081565b6096546001600160a01b031661036c565b5f546106da9060ff1681565b60405160ff9091168152602001610325565b61036c7f000000000000000000000000000000000000000000000000000000000000000081565b61031b7f2bd82124057f0913bc3b772ce7b83e8057c1ad1f3510fc83778be20f10ec5de681565b6102fa610748366004614236565b611605565b61031b600a5481565b61041b610764366004614236565b611616565b61077c610777366004614950565b61162a565b6040516103259190614a35565b6102fa610797366004614a47565b611638565b61031b7f4d404e3276e7ac2163d8ee476afa6a41d1f68fb71f2d8b6546b24e55ce01b72a81565b61036c7f000000000000000000000000000000000000000000000000000000000000000081565b600b5461036c906001600160a01b031681565b61086b61080b366004614286565b60408051606080820183525f808352602080840182905292840181905260ff9490941684526001825292829020825193840183525463ffffffff8116845261ffff600160201b8204811692850192909252600160301b9004169082015290565b6040516103259190614ad4565b61036c7f000000000000000000000000000000000000000000000000000000000000000081565b6102fa6108ad366004614236565b6117da565b61031b611850565b6102fa6108c8366004614123565b611859565b6108fb6108db366004614236565b6001600160a01b03165f9081526003602052604090206001015460ff1690565b6040516103259190614b07565b6033546002906004908116036109315760405163840a48d560e01b815260040160405180910390fd5b5f5b8251811015610aa4576040805160018082528183019092525f916020808301908036833701905050905083828151811061096f5761096f614b15565b6020026020010151815f8151811061098957610989614b15565b6001600160a01b0392909216602092830291909101909101526040805160018082528183019092525f9181602001602082028036833701905050905060035f8685815181106109da576109da614b15565b60200260200101516001600160a01b03166001600160a01b031681526020019081526020015f205f0154815f81518110610a1657610a16614b15565b6020026020010181815250505f610a45825f81518110610a3857610a38614b15565b6020026020010151611970565b90505f610a5a826001600160c01b031661197c565b90505f5b8151811015610a9357610a8b8585848481518110610a7e57610a7e614b15565b016020015160f81c611a45565b600101610a5e565b505060019093019250610933915050565b505050565b5f610ab76002858585611b7c565b90505b9392505050565b6001335f9081526003602052604090206001015460ff166002811115610ae957610ae96146ae565b14610b075760405163aba4733960e01b815260040160405180910390fd5b335f90815260036020526040902054610b209082611c60565b50565b610b2b611d0b565b600a80549082905560408051828152602081018490527fa77a91bea7b6d95a8eb5a54878a1d9e3c875e26c86a9b70e3420c5c5db193b62910160405180910390a15050565b60405163237dfb4760e11b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906346fbf68e90610bbc903390600401614180565b602060405180830381865afa158015610bd7573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610bfb9190614b38565b610c1857604051631d77d47760e21b815260040160405180910390fd5b6033548181168114610c3d5760405163c61dca5d60e01b815260040160405180910390fd5b610c4682611d65565b5050565b604080516060810182525f80825260208201819052918101919091525f838152600260205260409020805483908110610c8557610c85614b15565b5f91825260209182902060408051606081018252919092015463ffffffff8082168352600160201b820416938201939093526001600160c01b03600160401b909304929092169082015290505b92915050565b6040516308f6629d60e31b8152600481018290525f907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906347b314e890602401602060405180830381865afa158015610d3d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610cd29190614b51565b610d69611d0b565b610b2081611da2565b610d7a611d0b565b610b2081611e0b565b610d8b611e74565b603354600190600290811603610db45760405163840a48d560e01b815260040160405180910390fd5b610dbd83611616565b610dda576040516366e565df60e01b815260040160405180910390fd5b5f610de483611ebd565b9050610df08582611f65565b5050505050565b604080518082019091525f8082526020820152610cd2610e16836114a0565b612213565b610e23611d0b565b610e3184848460018561229d565b50505050565b603354600290600490811603610e605760405163840a48d560e01b815260040160405180910390fd5b610ea283838080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201829052505460ff1692506127ba915050565b5083518214610ec45760405163aaad13f760e01b815260040160405180910390fd5b5f5b82811015610df0575f848483818110610ee157610ee1614b15565b885192013560f81c92505f9188915084908110610f0057610f00614b15565b60209081029190910101516040516379a0849160e11b815260ff841660048201529091506001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063f341092290602401602060405180830381865afa158015610f73573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f979190614b6c565b63ffffffff16815114610fbd57604051638e5aeee760e01b815260040160405180910390fd5b5f81516001600160401b03811115610fd757610fd7613fbb565b604051908082528060200260200182016040528015611000578160200160208202803683370190505b5090505f805b83518110156110f6575f84828151811061102257611022614b15565b6020026020010151905060035f826001600160a01b03166001600160a01b031681526020019081526020015f205f015484838151811061106457611064614b15565b6020026020010181815250505f611086858481518110610a3857610a38614b15565b905060016001600160c01b03821660ff89161c8116146110b95760405163d053aa2160e01b815260040160405180910390fd5b836001600160a01b0316826001600160a01b0316116110eb5760405163ba50f91160e01b815260040160405180910390fd5b509150600101611006565b50611102838386611a45565b60ff84165f81815260056020908152604091829020439081905591519182527f46077d55330763f16269fd75e5761663f4192d2791747c0189b16ad31db07db4910160405180910390a250505050806001019050610ec6565b603254610100900460ff161580801561117b5750603254600160ff909116105b806111955750303b158015611195575060325460ff166001145b6111fd5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b6032805460ff191660011790558015611220576032805461ff0019166101001790555b6112766040518060400160405280601681526020017520ab29a932b3b4b9ba393ca1b7b7b93234b730ba37b960511b8152506040518060400160405280600681526020016576302e302e3160d01b8152506127ee565b61127f8661281f565b61128885611da2565b61129183611d65565b61129a84611e0b565b6112a382612870565b80156112e9576032805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b505050505050565b606061131c7f00000000000000000000000000000000000000000000000000000000000000006128dc565b905090565b604080518082019091525f80825260208201526001600160a01b0382165f908152600360209081526040918290208251808401909352805483526001810154909183019060ff166002811115611379576113796146ae565b600281111561138a5761138a6146ae565b90525092915050565b60405163237dfb4760e11b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906346fbf68e906113df903390600401614180565b602060405180830381865afa1580156113fa573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061141e9190614b38565b61143b57604051631d77d47760e21b815260040160405180910390fd5b6114455f19611d65565b565b61144f611d0b565b8161145981612919565b610aa48383612941565b61146b6129d2565b6001600160a01b0382165f908152600960205260409020429055610c4682826129fd565b611497611d0b565b6114455f61281f565b5f610cd27f2bd82124057f0913bc3b772ce7b83e8057c1ad1f3510fc83778be20f10ec5de6836040516020016114e99291909182526001600160a01b0316602082015260400190565b60405160208183030381529060405280519060200120612a93565b61150c611d0b565b610aa48383835f5f61229d565b5f6060805f5f5f606060c8545f5f1b148015611535575060c954155b6115795760405162461bcd60e51b81526020600482015260156024820152741152540dcc4c8e88155b9a5b9a5d1a585b1a5e9959605a1b60448201526064016111f4565b611581612abf565b611589612b4f565b604080515f80825260208201909252600f60f81b9b939a50919850469750309650945092509050565b5f6115f17f4d404e3276e7ac2163d8ee476afa6a41d1f68fb71f2d8b6546b24e55ce01b72a87878787876040516020016114e996959493929190614b87565b9695505050505050565b5f610cd282611970565b61160d611d0b565b610b2081612870565b600b546001600160a01b0391821691161490565b6060610aba60028484612b5e565b611640611e74565b6033545f906001908116036116685760405163840a48d560e01b815260040160405180910390fd5b61167185611616565b61168e576040516366e565df60e01b815260040160405180910390fd5b5f61169885611ebd565b90505f80806116a986880188614d16565b9250925092505f6116ba8b83612c0d565b90505f8460018111156116cf576116cf6146ae565b03611778575f6116e38c8388876001612ca6565b5190505f5b8651811015611771575f87828151811061170457611704614b15565b0160209081015160f81c5f8181526001909252604090912054845191925063ffffffff169084908490811061173b5761173b614b15565b602002602001015163ffffffff1611156117685760405163c6b9e76760e01b815260040160405180910390fd5b506001016116e8565b50506117cd565b600184600181111561178c5761178c6146ae565b036117b4575f8061179f898b018b614d71565b945094505050506117718d84898886866130ed565b60405163354bb8ab60e01b815260040160405180910390fd5b5050505050505050505050565b6117e2611d0b565b6001600160a01b0381166118475760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b60648201526084016111f4565b610b208161281f565b5f61131c6132e2565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156118b5573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906118d99190614b51565b6001600160a01b0316336001600160a01b03161461190a5760405163794821ff60e01b815260040160405180910390fd5b603354801982198116146119315760405163c61dca5d60e01b815260040160405180910390fd5b603382905560405182815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c906020015b60405180910390a25050565b5f610cd26002836132eb565b60605f5f61198984613355565b61ffff166001600160401b038111156119a4576119a4613fbb565b6040519080825280601f01601f1916602001820160405280156119ce576020820181803683370190505b5090505f805b8251821080156119e5575061010081105b15611a3b576001811b935085841615611a2b578060f81b838381518110611a0e57611a0e614b15565b60200101906001600160f81b03191690815f1a9053508160010191505b611a3481614e7e565b90506119d4565b5090949350505050565b6040805160018082528183019092525f916020820181803683370190505090508160f81b815f81518110611a7b57611a7b614b15565b60200101906001600160f81b03191690815f1a905350604051636c3fb4bf60e01b81525f906001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690636c3fb4bf90611ae390889088908890600401614e96565b5f604051808303815f875af1158015611afe573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052611b259190810190614f26565b90505f5b85518110156112e957818181518110611b4457611b44614b15565b602002602001015115611b7457611b74868281518110611b6657611b66614b15565b6020026020010151846129fd565b600101611b29565b5f838152602085905260408120805482919084908110611b9e57611b9e614b15565b5f91825260209182902060408051606081018252929091015463ffffffff808216808552600160201b8304821695850195909552600160401b9091046001600160c01b03169183019190915290925085161015611c0e57604051636cb19aff60e01b815260040160405180910390fd5b602081015163ffffffff161580611c345750806020015163ffffffff168463ffffffff16105b611c515760405163bbba60cb60e01b815260040160405180910390fd5b6040015190505b949350505050565b6040516378219b3f60e11b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063f043367e90611cae9085908590600401614fb3565b5f604051808303815f87803b158015611cc5575f5ffd5b505af1158015611cd7573d5f5f3e3d5ffd5b50505050817fec2963ab21c1e50e1e582aa542af2e4bf7bf38e6e1403c27b42e1c5d6e621eaa82604051611964919061469c565b6096546001600160a01b031633146114455760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016111f4565b603381905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a250565b6007546040517f315457d8a8fe60f04af17c16e2f5a5e1db612b31648e58030360759ef8f3528c91611de1916001600160a01b03909116908490614fcb565b60405180910390a1600780546001600160a01b0319166001600160a01b0392909216919091179055565b6008546040517f8f30ab09f43a6c157d7fce7e0a13c003042c1c95e8a72e7a146a21c0caa24dc991611e4a916001600160a01b03909116908490614fcb565b60405180910390a1600880546001600160a01b0319166001600160a01b0392909216919091179055565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614611445576040516323d871a560e01b815260040160405180910390fd5b60605f82516001600160401b03811115611ed957611ed9613fbb565b6040519080825280601f01601f191660200182016040528015611f03576020820181803683370190505b5090505f5b8351811015611f5e57838181518110611f2357611f23614b15565b602002602001015160f81b828281518110611f4057611f40614b15565b60200101906001600160f81b03191690815f1a905350600101611f08565b5092915050565b6001600160a01b0382165f90815260036020526040812080549091611f8982611970565b905060018084015460ff166002811115611fa557611fa56146ae565b14611fc35760405163aba4733960e01b815260040160405180910390fd5b5f8054611fd490869060ff166127ba565b90506001600160c01b038116611ffd576040516368b6a87560e11b815260040160405180910390fd5b6120146001600160c01b0382811690841681161490565b6120315760405163d053aa2160e01b815260040160405180910390fd5b6001600160c01b038181161983161661204a848261337f565b6001600160c01b0381166120a6576001600160a01b0387165f81815260036020526040808220600101805460ff19166002179055518692917f396fdcb180cb0fea26928113fb0fd1c3549863f9cd563e6a184f1d578116c8e491a35b60405163f4e24fe560e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063f4e24fe5906120f4908a908a90600401614fe5565b5f604051808303815f87803b15801561210b575f5ffd5b505af115801561211d573d5f5f3e3d5ffd5b505060405163bd29b8cd60e01b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016925063bd29b8cd915061216f9087908a90600401614fb3565b5f604051808303815f87803b158015612186575f5ffd5b505af1158015612198573d5f5f3e3d5ffd5b505060405163bd29b8cd60e01b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016925063bd29b8cd91506121ea9087908a90600401614fb3565b5f604051808303815f87803b158015612201575f5ffd5b505af11580156117cd573d5f5f3e3d5ffd5b604080518082019091525f80825260208201525f80806122405f5160206156ea5f395f51905f528661501c565b90505b61224c8161338b565b90935091505f5160206156ea5f395f51905f528283098303612284576040805180820190915290815260208101919091529392505050565b5f5160206156ea5f395f51905f52600182089050612243565b5f5460ff1660c081106122c357604051633cb89c9760e01b815260040160405180910390fd5b5f80546001919081906122da90849060ff1661502f565b92506101000a81548160ff021916908360ff1602179055506122fc8187612941565b6040805160018082528183019092525f91816020015b604080518082019091525f8152606060208201528152602001906001900390816123125790505090505f85516001600160401b0381111561235557612355613fbb565b60405190808252806020026020018201604052801561237e578160200160208202803683370190505b5090505f5b86518110156123db5786818151811061239e5761239e614b15565b60200260200101515f01518282815181106123bb576123bb614b15565b6001600160a01b0390921660209283029190910190910152600101612383565b5060405180604001604052808460ff1663ffffffff16815260200182815250825f8151811061240c5761240c614b15565b6020908102919091010152600b54604051630130fc2760e51b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000081169263261f84e09261246c9291909116908690600401615048565b5f604051808303815f87803b158015612483575f5ffd5b505af1158015612495573d5f5f3e3d5ffd5b505f92506124a1915050565b8560018111156124b3576124b36146ae565b0361253a57604051633aea0b9d60e11b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906375d4173a906125089086908b908b90600401615158565b5f604051808303815f87803b15801561251f575f5ffd5b505af1158015612531573d5f5f3e3d5ffd5b5050505061267e565b600185600181111561254e5761254e6146ae565b0361267e578363ffffffff167f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316632981eb776040518163ffffffff1660e01b8152600401602060405180830381865afa1580156125b6573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906125da9190614b6c565b63ffffffff16116125fe57604051630bd441b960e21b815260040160405180910390fd5b604051630662d3e160e51b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063cc5a7c20906126509086908b9089908c9060040161518b565b5f604051808303815f87803b158015612667575f5ffd5b505af1158015612679573d5f5f3e3d5ffd5b505050505b60405163136ca0f960e11b815260ff841660048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906326d941f2906024015f604051808303815f87803b1580156126df575f5ffd5b505af11580156126f1573d5f5f3e3d5ffd5b505060405163136ca0f960e11b815260ff861660048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031692506326d941f291506024015f604051808303815f87803b158015612756575f5ffd5b505af1158015612768573d5f5f3e3d5ffd5b505050508260ff167fa34835bc2b673ec37fcf1591a91295b163fc2e181e4ea4e733beb27de1ceac4c89898989896040516127a79594939291906151c1565b60405180910390a25b5050505050505050565b5f5f6127c584613407565b9050808360ff166001901b11610aba5760405163ca95733360e01b815260040160405180910390fd5b603254610100900460ff166128155760405162461bcd60e51b81526004016111f490615242565b610c4682826134c2565b609680546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a35050565b600b546040516001600160a01b03909116907f9770f3cadfdcbb6f93af935e86047111590c3768271d237e4a2bc0b874bed693906128b19083908590614fcb565b60405180910390a150600b80546001600160a01b0319166001600160a01b0392909216919091179055565b60605f6128e883613510565b6040805160208082528183019092529192505f91906020820181803683375050509182525060208101929092525090565b5f5460ff90811690821610610b2057604051637310cff560e11b815260040160405180910390fd5b60ff82165f8181526001602090815260409182902084518154928601518487015161ffff908116600160301b0267ffff0000000000001991909216600160201b0265ffffffffffff1990951663ffffffff909316929092179390931716919091179055517f3ee6fe8d54610244c3e9d3c066ae4aee997884aa28f10616ae821925401318ac90611964908490614ad4565b6008546001600160a01b03163314611445576040516376d8ab1760e11b815260040160405180910390fd5b6001600160a01b0382165f90815260036020526040902060018082015460ff166002811115612a2e57612a2e6146ae565b14612a4c576040516325ec6c1f60e01b815260040160405180910390fd5b80545f8054612a5f90859060ff166127ba565b90505f612a6b83611970565b9050612a846001600160c01b0383811690831681161490565b156112e9576112e98686613537565b5f610cd2612a9f6132e2565b8360405161190160f01b8152600281019290925260228201526042902090565b606060ca8054612ace9061528d565b80601f0160208091040260200160405190810160405280929190818152602001828054612afa9061528d565b8015612b455780601f10612b1c57610100808354040283529160200191612b45565b820191905f5260205f20905b815481529060010190602001808311612b2857829003601f168201915b5050505050905090565b606060cb8054612ace9061528d565b60605f82516001600160401b03811115612b7a57612b7a613fbb565b604051908082528060200260200182016040528015612ba3578160200160208202803683370190505b5090505f5b8351811015612c0457612bd58686868481518110612bc857612bc8614b15565b60200260200101516135d5565b828281518110612be757612be7614b15565b63ffffffff90921660209283029190910190910152600101612ba8565b50949350505050565b5f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166303c5a6b68484612c4887610df7565b6040518463ffffffff1660e01b8152600401612c66939291906152e7565b6020604051808303815f875af1158015612c82573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610aba9190615360565b612cca60405180606001604052806060815260200160608152602001606081525090565b5f8054612cdb90869060ff166127ba565b90505f612ce787611970565b90506001600160c01b038216612d10576040516313ca465760e01b815260040160405180910390fd5b8082166001600160c01b031615612d3a57604051630c6816cd60e01b815260040160405180910390fd5b600a546001600160a01b0389165f908152600960205260409020546001600160c01b0383811690851617914291612d719190615377565b10612d8f57604051631968677d60e11b815260040160405180910390fd5b612d99888261337f565b612da38887611c60565b60016001600160a01b038a165f9081526003602052604090206001015460ff166002811115612dd457612dd46146ae565b14612e6b57604080518082018252898152600160208083018281526001600160a01b038e165f908152600390925293902082518155925183820180549394939192909160ff191690836002811115612e2e57612e2e6146ae565b0217905550506040518991506001600160a01b038b16907fe8e68cef1c3a761ed7be7e8463a375f27f7bc335e51824223cacce636ec5c3fe905f90a35b604051631fd93ca960e11b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690633fb2795290612eb9908c908b90600401614fe5565b5f604051808303815f87803b158015612ed0575f5ffd5b505af1158015612ee2573d5f5f3e3d5ffd5b5050604051632550477760e01b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016925063255047779150612f36908c908c908c9060040161538a565b5f604051808303815f875af1158015612f51573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052612f789190810190615414565b60408087019190915260208601919091525162bff04d60e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169062bff04d90612fd3908b908b90600401614fb3565b5f604051808303815f875af1158015612fee573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052613015919081019061546d565b845284156130e1575f5b87518110156130df575f60015f8a848151811061303e5761303e614b15565b0160209081015160f81c82528181019290925260409081015f208151606081018352905463ffffffff811680835261ffff600160201b8304811695840195909552600160301b909104909316918101919091528751805191935090849081106130a9576130a9614b15565b602002602001015163ffffffff1611156130d65760405163c6b9e76760e01b815260040160405180910390fd5b5060010161301f565b505b50505095945050505050565b835182511461310f5760405163aaad13f760e01b815260040160405180910390fd5b61311b868684846136ed565b5f613129878787875f612ca6565b90505f5b85518110156127b0575f60015f88848151811061314c5761314c614b15565b0160209081015160f81c82528181019290925260409081015f208151606081018352905463ffffffff811680835261ffff600160201b8304811695840195909552600160301b909104909316918101919091528451805191935090849081106131b7576131b7614b15565b602002602001015163ffffffff1611156132d95761324b8783815181106131e0576131e0614b15565b602001015160f81c60f81b60f81c8460400151848151811061320457613204614b15565b60200260200101518b8660200151868151811061322357613223614b15565b602002602001015189878151811061323d5761323d614b15565b602002602001015186613798565b6040805160018082528183019092525f9160208201818036833701905050905087838151811061327d5761327d614b15565b602001015160f81c60f81b815f8151811061329a5761329a614b15565b60200101906001600160f81b03191690815f1a9053506132d78684815181106132c5576132c5614b15565b602002602001015160200151826129fd565b505b5060010161312d565b5f61131c613919565b5f81815260208390526040812054808203613309575f915050610cd2565b5f8381526020859052604090206133216001836154fc565b8154811061333157613331614b15565b5f91825260209091200154600160401b90046001600160c01b03169150610cd29050565b5f805b8215610cd2576133696001846154fc565b90921691806133778161550f565b915050613358565b610c466002838361398c565b5f80805f5160206156ea5f395f51905f5260035f5160206156ea5f395f51905f52865f5160206156ea5f395f51905f52888909090890505f6133fb827f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f525f5160206156ea5f395f51905f52613b45565b91959194509092505050565b5f6101008251111561342c57604051637da54e4760e11b815260040160405180910390fd5b81515f0361343b57505f919050565b5f5f835f8151811061344f5761344f614b15565b0160200151600160f89190911c81901b92505b84518110156134b95784818151811061347d5761347d614b15565b0160200151600160f89190911c1b91508282116134ad57604051631019106960e31b815260040160405180910390fd5b91811791600101613462565b50909392505050565b603254610100900460ff166134e95760405162461bcd60e51b81526004016111f490615242565b60ca6134f58382615573565b5060cb6135028282615573565b50505f60c881905560c95550565b5f60ff8216601f811115610cd257604051632cd44ac360e21b815260040160405180910390fd5b604080516060810182526001600160a01b038481168252600b54811660208301527f00000000000000000000000000000000000000000000000000000000000000001691636e3492b59190810161358d85613bbe565b8152506040518263ffffffff1660e01b81526004016135ac919061562d565b5f604051808303815f87803b1580156135c3575f5ffd5b505af11580156112e9573d5f5f3e3d5ffd5b5f81815260208490526040812054815b818110156136585760016135f982846154fc565b61360391906154fc565b92508463ffffffff16865f8681526020019081526020015f208463ffffffff168154811061363357613633614b15565b5f9182526020909120015463ffffffff1611613650575050610aba565b6001016135e5565b5060405162461bcd60e51b815260206004820152605c60248201527f5265676973747279436f6f7264696e61746f722e67657451756f72756d42697460448201527f6d6170496e6465784174426c6f636b4e756d6265723a206e6f206269746d617060648201527f2075706461746520666f756e6420666f72206f70657261746f72496400000000608482015260a4016111f4565b6020808201515f9081526004909152604090205460ff161561372257604051636fbefec360e11b815260040160405180910390fd5b428160400151101561374757604051630819bdcd60e01b815260040160405180910390fd5b602080820180515f9081526004909252604091829020805460ff19166001179055600754905191830151610e31926001600160a01b039092169161379191889188918891906115b2565b8351613c63565b6020808301516001600160a01b038082165f81815260039094526040909320549192908716036137db576040516356168b4160e11b815260040160405180910390fd5b8760ff16845f015160ff161461380457604051638e5aeee760e01b815260040160405180910390fd5b604051635401ed2760e01b81526004810182905260ff891660248201525f907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635401ed2790604401602060405180830381865afa158015613872573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906138969190615669565b90506138a28185613c8b565b6001600160601b0316866001600160601b0316116138d357604051634c44995d60e01b815260040160405180910390fd5b6138dd8885613cae565b6001600160601b0316816001600160601b03161061390e5760405163b187e86960e01b815260040160405180910390fd5b505050505050505050565b5f7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f613943613cc7565b61394b613d1f565b60408051602081019490945283019190915260608201524660808201523060a082015260c00160405160208183030381529060405280519060200120905090565b5f8281526020849052604081205490819003613a30575f83815260208581526040808320815160608101835263ffffffff43811682528185018681526001600160c01b03808a16958401958652845460018101865594885295909620915191909201805495519351909416600160401b026001600160401b03938316600160201b0267ffffffffffffffff1990961691909216179390931716919091179055610e31565b5f838152602085905260408120613a486001846154fc565b81548110613a5857613a58614b15565b5f918252602090912001805490915063ffffffff438116911603613a995780546001600160401b0316600160401b6001600160c01b03851602178155610df0565b805463ffffffff438116600160201b81810267ffffffff00000000199094169390931784555f8781526020898152604080832081516060810183529485528483018481526001600160c01b03808c1693870193845282546001810184559286529390942094519401805493519151909216600160401b026001600160401b0391861690960267ffffffffffffffff19909316939094169290921717919091169190911790555050505050565b5f5f613b4f613f7f565b613b57613f9d565b602080825281810181905260408201819052606082018890526080820187905260a082018690528260c08360056107d05a03fa92508280613b9457fe5b5082613bb35760405163d51edae360e01b815260040160405180910390fd5b505195945050505050565b60605f82516001600160401b03811115613bda57613bda613fbb565b604051908082528060200260200182016040528015613c03578160200160208202803683370190505b5090505f5b8351811015611f5e57838181518110613c2357613c23614b15565b602001015160f81c60f81b60f81c60ff16828281518110613c4657613c46614b15565b63ffffffff90921660209283029190910190910152600101613c08565b613c6e838383613d4f565b610aa457604051638baa579f60e01b815260040160405180910390fd5b60208101515f9061271090613ca49061ffff1685615684565b610aba91906156a6565b60408101515f9061271090613ca49061ffff1685615684565b5f5f613cd1612abf565b805190915015613ce8578051602090910120919050565b60c8548015613cf75792915050565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a4709250505090565b5f5f613d29612b4f565b805190915015613d40578051602090910120919050565b60c9548015613cf75792915050565b5f5f5f613d5c8585613da3565b90925090505f816004811115613d7457613d746146ae565b148015613d925750856001600160a01b0316826001600160a01b0316145b806115f157506115f1868686613de5565b5f5f8251604103613dd7576020830151604084015160608501515f1a613dcb87828585613ecc565b94509450505050613dde565b505f905060025b9250929050565b5f5f5f856001600160a01b0316631626ba7e60e01b8686604051602401613e0d929190614fb3565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b0319909416939093179092529051613e4b91906156d3565b5f60405180830381855afa9150503d805f8114613e83576040519150601f19603f3d011682016040523d82523d5f602084013e613e88565b606091505b5091509150818015613e9c57506020815110155b80156115f157508051630b135d3f60e11b90613ec19083016020908101908401615360565b149695505050505050565b5f806fa2a8918ca85bafe22016d0b997e4df60600160ff1b03831115613ef757505f90506003613f76565b604080515f8082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa158015613f48573d5f5f3e3d5ffd5b5050604051601f1901519150506001600160a01b038116613f70575f60019250925050613f76565b91505f90505b94509492505050565b60405180602001604052806001906020820280368337509192915050565b6040518060c001604052806006906020820280368337509192915050565b634e487b7160e01b5f52604160045260245ffd5b604051606081016001600160401b0381118282101715613ff157613ff1613fbb565b60405290565b604080519081016001600160401b0381118282101715613ff157613ff1613fbb565b604051601f8201601f191681016001600160401b038111828210171561404157614041613fbb565b604052919050565b5f6001600160401b0382111561406157614061613fbb565b5060051b60200190565b6001600160a01b0381168114610b20575f5ffd5b5f82601f83011261408e575f5ffd5b81356140a161409c82614049565b614019565b8082825260208201915060208360051b8601019250858311156140c2575f5ffd5b602085015b838110156140e85780356140da8161406b565b8352602092830192016140c7565b5095945050505050565b5f60208284031215614102575f5ffd5b81356001600160401b03811115614117575f5ffd5b611c588482850161407f565b5f60208284031215614133575f5ffd5b5035919050565b63ffffffff81168114610b20575f5ffd5b5f5f5f6060848603121561415d575f5ffd5b83359250602084013561416f8161413a565b929592945050506040919091013590565b6001600160a01b0391909116815260200190565b5f82601f8301126141a3575f5ffd5b8135602083015f5f6001600160401b038411156141c2576141c2613fbb565b50601f8301601f19166020016141d781614019565b9150508281528583830111156141eb575f5ffd5b828260208301375f92810160200192909252509392505050565b5f60208284031215614215575f5ffd5b81356001600160401b0381111561422a575f5ffd5b611c5884828501614194565b5f60208284031215614246575f5ffd5b8135610aba8161406b565b5f5f60408385031215614262575f5ffd5b50508035926020909101359150565b803560ff81168114614281575f5ffd5b919050565b5f60208284031215614296575f5ffd5b610aba82614271565b5f82601f8301126142ae575f5ffd5b81356142bc61409c82614049565b8082825260208201915060208360051b8601019250858311156142dd575f5ffd5b602085015b838110156140e85780356142f58161413a565b8352602092830192016142e2565b5f5f5f60608486031215614315575f5ffd5b83356143208161406b565b925060208401356143308161406b565b915060408401356001600160401b0381111561434a575f5ffd5b6143568682870161429f565b9150509250925092565b815181526020808301519082015260408101610cd2565b803561ffff81168114614281575f5ffd5b5f60608284031215614398575f5ffd5b6143a0613fcf565b905081356143ad8161413a565b81526143bb60208301614377565b60208201526143cc60408301614377565b604082015292915050565b6001600160601b0381168114610b20575f5ffd5b5f82601f8301126143fa575f5ffd5b813561440861409c82614049565b8082825260208201915060208360061b860101925085831115614429575f5ffd5b602085015b838110156140e85760408188031215614445575f5ffd5b61444d613ff7565b81356144588161406b565b81526020820135614468816143d7565b602082810191909152908452929092019160400161442e565b5f5f5f5f60c08587031215614494575f5ffd5b61449e8686614388565b935060608501356144ae816143d7565b925060808501356001600160401b038111156144c8575f5ffd5b6144d4878288016143eb565b92505060a08501356144e58161413a565b939692955090935050565b5f5f83601f840112614500575f5ffd5b5081356001600160401b03811115614516575f5ffd5b602083019150836020828501011115613dde575f5ffd5b5f5f5f6040848603121561453f575f5ffd5b83356001600160401b03811115614554575f5ffd5b8401601f81018613614564575f5ffd5b803561457261409c82614049565b8082825260208201915060208360051b850101925088831115614593575f5ffd5b602084015b838110156145d35780356001600160401b038111156145b5575f5ffd5b6145c48b60208389010161407f565b84525060209283019201614598565b50955050505060208401356001600160401b038111156145f1575f5ffd5b6145fd868287016144f0565b9497909650939450505050565b5f5f5f5f5f60a0868803121561461e575f5ffd5b85356146298161406b565b945060208601356146398161406b565b935060408601356146498161406b565b92506060860135915060808601356146608161406b565b809150509295509295909350565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b602081525f610aba602083018461466e565b634e487b7160e01b5f52602160045260245ffd5b600381106146d2576146d26146ae565b9052565b815181526020808301516040830191611f5e908401826146c2565b5f5f60808385031215614702575f5ffd5b61470b83614271565b915061471a8460208501614388565b90509250929050565b5f5f60408385031215614734575f5ffd5b823561473f8161406b565b915060208301356001600160401b03811115614759575f5ffd5b61476585828601614194565b9150509250929050565b5f5f5f60a08486031215614781575f5ffd5b61478b8585614388565b9250606084013561479b816143d7565b915060808401356001600160401b038111156147b5575f5ffd5b614356868287016143eb565b60ff60f81b8816815260e060208201525f6147df60e083018961466e565b82810360408401526147f1818961466e565b606084018890526001600160a01b038716608085015260a0840186905283810360c0850152845180825260208087019350909101905f5b81811015614846578351835260209384019390920191600101614828565b50909b9a5050505050505050505050565b5f82601f830112614866575f5ffd5b813561487461409c82614049565b8082825260208201915060208360061b860101925085831115614895575f5ffd5b602085015b838110156140e857604081880312156148b1575f5ffd5b6148b9613ff7565b6148c282614271565b815260208201356148d28161406b565b602082810191909152908452929092019160400161489a565b5f5f5f5f5f60a086880312156148ff575f5ffd5b853561490a8161406b565b94506020860135935060408601356001600160401b0381111561492b575f5ffd5b61493788828901614857565b9598949750949560608101359550608001359392505050565b5f5f60408385031215614961575f5ffd5b823561496c8161413a565b915060208301356001600160401b03811115614986575f5ffd5b8301601f81018513614996575f5ffd5b80356149a461409c82614049565b8082825260208201915060208360051b8501019250878311156149c5575f5ffd5b6020840193505b828410156149e75783358252602093840193909101906149cc565b809450505050509250929050565b5f8151808452602084019350602083015f5b82811015614a2b57815163ffffffff16865260209586019590910190600101614a07565b5093949350505050565b602081525f610aba60208301846149f5565b5f5f5f5f5f60808688031215614a5b575f5ffd5b8535614a668161406b565b94506020860135614a768161406b565b935060408601356001600160401b03811115614a90575f5ffd5b614a9c8882890161429f565b93505060608601356001600160401b03811115614ab7575f5ffd5b614ac3888289016144f0565b969995985093965092949392505050565b60608101610cd2828463ffffffff815116825261ffff602082015116602083015261ffff60408201511660408301525050565b60208101610cd282846146c2565b634e487b7160e01b5f52603260045260245ffd5b80518015158114614281575f5ffd5b5f60208284031215614b48575f5ffd5b610aba82614b29565b5f60208284031215614b61575f5ffd5b8151610aba8161406b565b5f60208284031215614b7c575f5ffd5b8151610aba8161413a565b5f60c0820188835260018060a01b038816602084015286604084015260c0606084015280865180835260e0850191506020880192505f5b81811015614bf4578351805160ff1684526020908101516001600160a01b03168185015290930192604090920191600101614bbe565b50506080840195909552505060a00152949350505050565b803560028110614281575f5ffd5b5f60408284031215614c2a575f5ffd5b614c32613ff7565b823581526020928301359281019290925250919050565b5f82601f830112614c58575f5ffd5b614c60613ff7565b806040840185811115614c71575f5ffd5b845b81811015614c8b578035845260209384019301614c73565b509095945050505050565b5f818303610100811215614ca8575f5ffd5b614cb0613fcf565b9150614cbc8484614c1a565b8252614ccb8460408501614c1a565b60208301526080607f1982011215614ce1575f5ffd5b50614cea613ff7565b614cf78460808501614c49565b8152614d068460c08501614c49565b6020820152604082015292915050565b5f5f5f6101408486031215614d29575f5ffd5b614d3284614c0c565b925060208401356001600160401b03811115614d4c575f5ffd5b614d5886828701614194565b925050614d688560408601614c96565b90509250925092565b5f5f5f5f5f6101808688031215614d86575f5ffd5b614d8f86614c0c565b945060208601356001600160401b03811115614da9575f5ffd5b614db588828901614194565b945050614dc58760408801614c96565b92506101408601356001600160401b03811115614de0575f5ffd5b614dec88828901614857565b9250506101608601356001600160401b03811115614e08575f5ffd5b860160608189031215614e19575f5ffd5b614e21613fcf565b81356001600160401b03811115614e36575f5ffd5b614e428a828501614194565b8252506020828101359082015260409182013591810191909152949793965091945092919050565b634e487b7160e01b5f52601160045260245ffd5b5f60018201614e8f57614e8f614e6a565b5060010190565b606080825284519082018190525f9060208601906080840190835b81811015614ed85783516001600160a01b0316835260209384019390920191600101614eb1565b5050838103602080860191909152865180835291810192508601905f5b81811015614f13578251845260209384019390920191600101614ef5565b50505060ff841660408401529050611c58565b5f60208284031215614f36575f5ffd5b81516001600160401b03811115614f4b575f5ffd5b8201601f81018413614f5b575f5ffd5b8051614f6961409c82614049565b8082825260208201915060208360051b850101925086831115614f8a575f5ffd5b6020840193505b828410156115f157614fa284614b29565b825260209384019390910190614f91565b828152604060208201525f610ab7604083018461466e565b6001600160a01b0392831681529116602082015260400190565b6001600160a01b03831681526040602082018190525f90610ab79083018461466e565b634e487b7160e01b5f52601260045260245ffd5b5f8261502a5761502a615008565b500690565b60ff8181168382160190811115610cd257610cd2614e6a565b5f6040820160018060a01b03851683526040602084015280845180835260608501915060608160051b8601019250602086015f5b828110156150fd57868503605f190184528151805163ffffffff168652602090810151604082880181905281519088018190529101905f9060608801905b808310156150e55783516001600160a01b0316825260209384019360019390930192909101906150ba565b5096505050602093840193919091019060010161507c565b5092979650505050505050565b5f8151808452602084019350602083015f5b82811015614a2b57815180516001600160a01b031687526020908101516001600160601b0316818801526040909601959091019060010161511c565b60ff841681526001600160601b0383166020820152606060408201525f615182606083018461510a565b95945050505050565b60ff851681526001600160601b038416602082015263ffffffff83166040820152608060608201525f6115f1608083018461510a565b6151f0818763ffffffff815116825261ffff602082015116602083015261ffff60408201511660408301525050565b6001600160601b038516606082015260e060808201525f61521460e083018661510a565b905060028410615226576152266146ae565b8360a083015263ffffffff831660c08301529695505050505050565b6020808252602b908201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960408201526a6e697469616c697a696e6760a81b606082015260800190565b600181811c908216806152a157607f821691505b6020821081036152bf57634e487b7160e01b5f52602260045260245ffd5b50919050565b805f5b6002811015610e315781518452602093840193909101906001016152c8565b6001600160a01b0384168152825180516020808401919091520151604082015261016081016020848101518051606085015290810151608084015250604084015161533660a0840182516152c5565b6020015161534760e08401826152c5565b5082516101208301526020830151610140830152611c58565b5f60208284031215615370575f5ffd5b5051919050565b80820180821115610cd257610cd2614e6a565b60018060a01b0384168152826020820152606060408201525f615182606083018461466e565b5f82601f8301126153bf575f5ffd5b81516153cd61409c82614049565b8082825260208201915060208360051b8601019250858311156153ee575f5ffd5b602085015b838110156140e8578051615406816143d7565b8352602092830192016153f3565b5f5f60408385031215615425575f5ffd5b82516001600160401b0381111561543a575f5ffd5b615446858286016153b0565b92505060208301516001600160401b03811115615461575f5ffd5b614765858286016153b0565b5f6020828403121561547d575f5ffd5b81516001600160401b03811115615492575f5ffd5b8201601f810184136154a2575f5ffd5b80516154b061409c82614049565b8082825260208201915060208360051b8501019250868311156154d1575f5ffd5b6020840193505b828410156115f15783516154eb8161413a565b8252602093840193909101906154d8565b81810381811115610cd257610cd2614e6a565b5f61ffff821661ffff810361552657615526614e6a565b60010192915050565b601f821115610aa457805f5260205f20601f840160051c810160208510156155545750805b601f840160051c820191505b81811015610df0575f8155600101615560565b81516001600160401b0381111561558c5761558c613fbb565b6155a08161559a845461528d565b8461552f565b6020601f8211600181146155d2575f83156155bb5750848201515b5f19600385901b1c1916600184901b178455610df0565b5f84815260208120601f198516915b8281101561560157878501518255602094850194600190920191016155e1565b508482101561561e57868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b602080825282516001600160a01b039081168383015290830151166040808301919091528201516060808301525f90611c5860808401826149f5565b5f60208284031215615679575f5ffd5b8151610aba816143d7565b6001600160601b038181168382160290811690818114611f5e57611f5e614e6a565b5f6001600160601b038316806156be576156be615008565b806001600160601b0384160491505092915050565b5f82518060208501845e5f92019182525091905056fe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a2646970667358221220a432d9f08962ade1f83c57c85ffe1b12b9eb8498575b39e50949d773e7a5528a64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01``@R4\x80\x15a\0\x10W__\xFD[P`@Qa[\x888\x03\x80a[\x88\x839\x81\x01`@\x81\x90Ra\0/\x91a\x01\xD9V[`\x01`\x01`\xA0\x1B\x03\x80\x88\x16`\xC0R\x80\x87\x16`\xA0R\x80\x86\x16`\xE0R\x80\x85\x16`\x80R\x83\x16a\x01\0R\x81\x81a\0`\x81a\0\xADV[a\x01 RP`\x01`\x01`\xA0\x1B\x03\x81\x16a\0\x8CW`@Qc9\xB1\x90\xBB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x16a\x01@Ra\0\xA1a\0\xF3V[PPPPPPPa\x03WV[__\x82\x90P`\x1F\x81Q\x11\x15a\0\xE0W\x82`@Qc0Z'\xA9`\xE0\x1B\x81R`\x04\x01a\0\xD7\x91\x90a\x02\xFCV[`@Q\x80\x91\x03\x90\xFD[\x80Qa\0\xEB\x82a\x031V[\x17\x93\x92PPPV[`2Ta\x01\0\x90\x04`\xFF\x16\x15a\x01[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01a\0\xD7V[`2T`\xFF\x90\x81\x16\x14a\x01\xACW`2\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xC2W__\xFD[PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_______`\xE0\x88\x8A\x03\x12\x15a\x01\xEFW__\xFD[\x87Qa\x01\xFA\x81a\x01\xAEV[` \x89\x01Q\x90\x97Pa\x02\x0B\x81a\x01\xAEV[`@\x89\x01Q\x90\x96Pa\x02\x1C\x81a\x01\xAEV[``\x89\x01Q\x90\x95Pa\x02-\x81a\x01\xAEV[`\x80\x89\x01Q\x90\x94Pa\x02>\x81a\x01\xAEV[`\xA0\x89\x01Q\x90\x93Pa\x02O\x81a\x01\xAEV[`\xC0\x89\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02jW__\xFD[\x88\x01`\x1F\x81\x01\x8A\x13a\x02zW__\xFD[\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02\x93Wa\x02\x93a\x01\xC5V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x02\xC1Wa\x02\xC1a\x01\xC5V[`@R\x81\x81R\x82\x82\x01` \x01\x8C\x10\x15a\x02\xD8W__\xFD[\x81` \x84\x01` \x83\x01^_` \x83\x83\x01\x01R\x80\x93PPPP\x92\x95\x98\x91\x94\x97P\x92\x95PV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\x03QW_\x19\x81` \x03`\x03\x1B\x1B\x82\x16\x91P[P\x91\x90PV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@QaW?a\x04I_9_\x81\x81a\x06\x9B\x01R\x81\x81a\x0B\x87\x01R\x81\x81a\x13\xAA\x01Ra\x18[\x01R_a\x12\xF8\x01R_\x81\x81a\x07\xC8\x01R\x81\x81a\x1E\x7F\x01R\x81\x81a$1\x01R\x81\x81a%\\\x01Ra5Z\x01R_\x81\x81a\x06\xF1\x01R\x81\x81a\x0F.\x01R\x81\x81a!\xB1\x01R\x81\x81a&\x96\x01Ra/\x9D\x01R_\x81\x81a\x05\xF2\x01R\x81\x81a\x1A\xAA\x01R\x81\x81a!6\x01R\x81\x81a$\xCF\x01R\x81\x81a&\x15\x01R\x81\x81a.\xFB\x01Ra8%\x01R_\x81\x81a\x05\xCB\x01R\x81\x81a\x0C\xF0\x01R\x81\x81a \xBD\x01R\x81\x81a'\x0B\x01R\x81\x81a,\x10\x01Ra.\x82\x01R_\x81\x81a\x08}\x01Ra\x1Cw\x01RaW?_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x02\xE3W_5`\xE0\x1C\x80c]\xF4YF\x11a\x01\x8DW\x80c\xA6T\x97\xC6\x11a\0\xE0W\x80c\xDE\x11d\xBB\x11a\0\x8FW\x80c\xDE\x11d\xBB\x14a\x07\xEAW\x80c\xE6W\x97\xAD\x14a\x07\xFDW\x80c\xEA2\xAF\xAE\x14a\x08xW\x80c\xF2\xFD\xE3\x8B\x14a\x08\x9FW\x80c\xF6\x98\xDA%\x14a\x08\xB2W\x80c\xFA\xBC\x1C\xBC\x14a\x08\xBAW\x80c\xFD9\x10Z\x14a\x08\xCDW__\xFD[\x80c\xA6T\x97\xC6\x14a\x07:W\x80c\xA9ox>\x14a\x07MW\x80c\xB5&W\x87\x14a\x07VW\x80c\xC3\x91B^\x14a\x07iW\x80c\xC6?\xD5\x02\x14a\x07\x89W\x80c\xCA\r\xE8\x82\x14a\x07\x9CW\x80c\xCA\x8A\xA7\xC7\x14a\x07\xC3W__\xFD[\x80c\x84\xCAR\x13\x11a\x01<W\x80c\x84\xCAR\x13\x14a\x06pW\x80c\x87\x1E\xF0I\x14a\x06\x83W\x80c\x88o\x11\x95\x14a\x06\x96W\x80c\x8D\xA5\xCB[\x14a\x06\xBDW\x80c\x9A\xA1e=\x14a\x06\xCEW\x80c\x9E\x99#\xC2\x14a\x06\xECW\x80c\x9F\xEA\xB8Y\x14a\x07\x13W__\xFD[\x80c]\xF4YF\x14a\x05\xC6W\x80ch0H5\x14a\x05\xEDW\x80cn;\x17\xDB\x14a\x06\x14W\x80cqP\x18\xA6\x14a\x06'W\x80csDy\x92\x14a\x06/W\x80c\x82\x81\xABu\x14a\x06BW\x80c\x84\xB0\x19n\x14a\x06UW__\xFD[\x80c)k\xB0d\x11a\x02EW\x80cS\x0B\x97\xA4\x11a\x01\xF4W\x80cS\x0B\x97\xA4\x14a\x058W\x80cT\xFDMP\x14a\x05KW\x80cXe\xC6\x0C\x14a\x05`W\x80cY\\jg\x14a\x05\x80W\x80cZ\xC8j\xB7\x14a\x05\x88W\x80c[\x0B\x82\x9F\x14a\x05\xABW\x80c\\\x97Z\xBB\x14a\x05\xBEW__\xFD[\x80c)k\xB0d\x14a\x04\xA6W\x80c)\xD1\xE0\xC3\x14a\x04\xB9W\x80c,\xDD\x1E\x86\x14a\x04\xCCW\x80c0<\xA9V\x14a\x04\xDFW\x80c<*\x7FL\x14a\x04\xF2W\x80c>\xEF:Q\x14a\x05\x12W\x80cQ@\xA5H\x14a\x05%W__\xFD[\x80c\x12^\x05\x84\x11a\x02\xA1W\x80c\x12^\x05\x84\x14a\x03\x9FW\x80c\x13T*N\x14a\x03\xBEW\x80c\x13d9\xDD\x14a\x03\xE6W\x80c\x14x\x85\x1F\x14a\x03\xF9W\x80c\x1E\xB8\x12\xDA\x14a\x04+W\x80c$\x9A\x0CB\x14a\x04tW\x80c(\xF6\x1B1\x14a\x04\x93W__\xFD[\x80b\xCF*\xB5\x14a\x02\xE7W\x80c\x03\xFD4\x92\x14a\x02\xFCW\x80c\x04\xECcQ\x14a\x03.W\x80c\x05C\x10\xE6\x14a\x03YW\x80c\x0C\xF4\xB7g\x14a\x03yW\x80c\r?!4\x14a\x03\x8CW[__\xFD[a\x02\xFAa\x02\xF56`\x04a@\xF2V[a\t\x08V[\0[a\x03\x1Ba\x03\n6`\x04aA#V[_\x90\x81R`\x02` R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x03Aa\x03<6`\x04aAKV[a\n\xA9V[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R` \x01a\x03%V[`\x07Ta\x03l\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Qa\x03%\x91\x90aA\x80V[a\x02\xFAa\x03\x876`\x04aB\x05V[a\n\xC1V[a\x02\xFAa\x03\x9A6`\x04aA#V[a\x0B#V[a\x03\x1Ba\x03\xAD6`\x04aB6V[`\t` R_\x90\x81R`@\x90 T\x81V[a\x03\x1Ba\x03\xCC6`\x04aB6V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x03` R`@\x90 T\x90V[a\x02\xFAa\x03\xF46`\x04aA#V[a\x0BpV[a\x04\x1Ba\x04\x076`\x04aA#V[`\x04` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x03%V[a\x04>a\x0496`\x04aBQV[a\x0CJV[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x85\x01Q\x90\x91\x16\x90\x82\x01R\x91\x81\x01Q`\x01`\x01`\xC0\x1B\x03\x16\x90\x82\x01R``\x01a\x03%V[a\x03\x1Ba\x04\x826`\x04aB\x86V[`\x05` R_\x90\x81R`@\x90 T\x81V[`\x08Ta\x03l\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03la\x04\xB46`\x04aA#V[a\x0C\xD8V[a\x02\xFAa\x04\xC76`\x04aB6V[a\raV[a\x02\xFAa\x04\xDA6`\x04aB6V[a\rrV[a\x02\xFAa\x04\xED6`\x04aC\x03V[a\r\x83V[a\x05\x05a\x05\x006`\x04aB6V[a\r\xF7V[`@Qa\x03%\x91\x90aC`V[a\x02\xFAa\x05 6`\x04aD\x81V[a\x0E\x1BV[a\x02\xFAa\x0536`\x04aE-V[a\x0E7V[a\x02\xFAa\x05F6`\x04aF\nV[a\x11[V[a\x05Sa\x12\xF1V[`@Qa\x03%\x91\x90aF\x9CV[a\x05sa\x05n6`\x04aB6V[a\x13!V[`@Qa\x03%\x91\x90aF\xD6V[a\x02\xFAa\x13\x93V[a\x04\x1Ba\x05\x966`\x04aB\x86V[`3T`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[a\x02\xFAa\x05\xB96`\x04aF\xF1V[a\x14GV[`3Ta\x03\x1BV[a\x03l\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03l\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xFAa\x06\"6`\x04aG#V[a\x14cV[a\x02\xFAa\x14\x8FV[a\x03\x1Ba\x06=6`\x04aB6V[a\x14\xA0V[a\x02\xFAa\x06P6`\x04aGoV[a\x15\x04V[a\x06]a\x15\x19V[`@Qa\x03%\x97\x96\x95\x94\x93\x92\x91\x90aG\xC1V[a\x03\x1Ba\x06~6`\x04aH\xEBV[a\x15\xB2V[a\x03Aa\x06\x916`\x04aA#V[a\x15\xFBV[a\x03l\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\x96T`\x01`\x01`\xA0\x1B\x03\x16a\x03lV[_Ta\x06\xDA\x90`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x03%V[a\x03l\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\x1B\x7F+\xD8!$\x05\x7F\t\x13\xBC;w,\xE7\xB8>\x80W\xC1\xAD\x1F5\x10\xFC\x83w\x8B\xE2\x0F\x10\xEC]\xE6\x81V[a\x02\xFAa\x07H6`\x04aB6V[a\x16\x05V[a\x03\x1B`\nT\x81V[a\x04\x1Ba\x07d6`\x04aB6V[a\x16\x16V[a\x07|a\x07w6`\x04aIPV[a\x16*V[`@Qa\x03%\x91\x90aJ5V[a\x02\xFAa\x07\x976`\x04aJGV[a\x168V[a\x03\x1B\x7FM@N2v\xE7\xAC!c\xD8\xEEGj\xFAjA\xD1\xF6\x8F\xB7\x1F-\x8BeF\xB2NU\xCE\x01\xB7*\x81V[a\x03l\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\x0BTa\x03l\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x08ka\x08\x0B6`\x04aB\x86V[`@\x80Q``\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x92\x84\x01\x81\x90R`\xFF\x94\x90\x94\x16\x84R`\x01\x82R\x92\x82\x90 \x82Q\x93\x84\x01\x83RTc\xFF\xFF\xFF\xFF\x81\x16\x84Ra\xFF\xFF`\x01` \x1B\x82\x04\x81\x16\x92\x85\x01\x92\x90\x92R`\x01`0\x1B\x90\x04\x16\x90\x82\x01R\x90V[`@Qa\x03%\x91\x90aJ\xD4V[a\x03l\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xFAa\x08\xAD6`\x04aB6V[a\x17\xDAV[a\x03\x1Ba\x18PV[a\x02\xFAa\x08\xC86`\x04aA#V[a\x18YV[a\x08\xFBa\x08\xDB6`\x04aB6V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x03` R`@\x90 `\x01\x01T`\xFF\x16\x90V[`@Qa\x03%\x91\x90aK\x07V[`3T`\x02\x90`\x04\x90\x81\x16\x03a\t1W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x82Q\x81\x10\x15a\n\xA4W`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x83\x82\x81Q\x81\x10a\toWa\toaK\x15V[` \x02` \x01\x01Q\x81_\x81Q\x81\x10a\t\x89Wa\t\x89aK\x15V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91\x81` \x01` \x82\x02\x806\x837\x01\x90PP\x90P`\x03_\x86\x85\x81Q\x81\x10a\t\xDAWa\t\xDAaK\x15V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _\x01T\x81_\x81Q\x81\x10a\n\x16Wa\n\x16aK\x15V[` \x02` \x01\x01\x81\x81RPP_a\nE\x82_\x81Q\x81\x10a\n8Wa\n8aK\x15V[` \x02` \x01\x01Qa\x19pV[\x90P_a\nZ\x82`\x01`\x01`\xC0\x1B\x03\x16a\x19|V[\x90P_[\x81Q\x81\x10\x15a\n\x93Wa\n\x8B\x85\x85\x84\x84\x81Q\x81\x10a\n~Wa\n~aK\x15V[\x01` \x01Q`\xF8\x1Ca\x1AEV[`\x01\x01a\n^V[PP`\x01\x90\x93\x01\x92Pa\t3\x91PPV[PPPV[_a\n\xB7`\x02\x85\x85\x85a\x1B|V[\x90P[\x93\x92PPPV[`\x013_\x90\x81R`\x03` R`@\x90 `\x01\x01T`\xFF\x16`\x02\x81\x11\x15a\n\xE9Wa\n\xE9aF\xAEV[\x14a\x0B\x07W`@Qc\xAB\xA4s9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3_\x90\x81R`\x03` R`@\x90 Ta\x0B \x90\x82a\x1C`V[PV[a\x0B+a\x1D\x0BV[`\n\x80T\x90\x82\x90U`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x7F\xA7z\x91\xBE\xA7\xB6\xD9Z\x8E\xB5\xA5Hx\xA1\xD9\xE3\xC8u\xE2l\x86\xA9\xB7\x0E4 \xC5\xC5\xDB\x19;b\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[`@Qc#}\xFBG`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cF\xFB\xF6\x8E\x90a\x0B\xBC\x903\x90`\x04\x01aA\x80V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xD7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xFB\x91\x90aK8V[a\x0C\x18W`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`3T\x81\x81\x16\x81\x14a\x0C=W`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0CF\x82a\x1DeV[PPV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R_\x83\x81R`\x02` R`@\x90 \x80T\x83\x90\x81\x10a\x0C\x85Wa\x0C\x85aK\x15V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01`\xC0\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90P[\x92\x91PPV[`@Qc\x08\xF6b\x9D`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R_\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cG\xB3\x14\xE8\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r=W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xD2\x91\x90aKQV[a\ria\x1D\x0BV[a\x0B \x81a\x1D\xA2V[a\rza\x1D\x0BV[a\x0B \x81a\x1E\x0BV[a\r\x8Ba\x1EtV[`3T`\x01\x90`\x02\x90\x81\x16\x03a\r\xB4W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\r\xBD\x83a\x16\x16V[a\r\xDAW`@Qcf\xE5e\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\r\xE4\x83a\x1E\xBDV[\x90Pa\r\xF0\x85\x82a\x1FeV[PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01Ra\x0C\xD2a\x0E\x16\x83a\x14\xA0V[a\"\x13V[a\x0E#a\x1D\x0BV[a\x0E1\x84\x84\x84`\x01\x85a\"\x9DV[PPPPV[`3T`\x02\x90`\x04\x90\x81\x16\x03a\x0E`W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0E\xA2\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x82\x90RPT`\xFF\x16\x92Pa'\xBA\x91PPV[P\x83Q\x82\x14a\x0E\xC4W`@Qc\xAA\xAD\x13\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x82\x81\x10\x15a\r\xF0W_\x84\x84\x83\x81\x81\x10a\x0E\xE1Wa\x0E\xE1aK\x15V[\x88Q\x92\x015`\xF8\x1C\x92P_\x91\x88\x91P\x84\x90\x81\x10a\x0F\0Wa\x0F\0aK\x15V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qcy\xA0\x84\x91`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xF3A\t\"\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0FsW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x97\x91\x90aKlV[c\xFF\xFF\xFF\xFF\x16\x81Q\x14a\x0F\xBDW`@Qc\x8EZ\xEE\xE7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0F\xD7Wa\x0F\xD7a?\xBBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x10\0W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80[\x83Q\x81\x10\x15a\x10\xF6W_\x84\x82\x81Q\x81\x10a\x10\"Wa\x10\"aK\x15V[` \x02` \x01\x01Q\x90P`\x03_\x82`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _\x01T\x84\x83\x81Q\x81\x10a\x10dWa\x10daK\x15V[` \x02` \x01\x01\x81\x81RPP_a\x10\x86\x85\x84\x81Q\x81\x10a\n8Wa\n8aK\x15V[\x90P`\x01`\x01`\x01`\xC0\x1B\x03\x82\x16`\xFF\x89\x16\x1C\x81\x16\x14a\x10\xB9W`@Qc\xD0S\xAA!`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x11a\x10\xEBW`@Qc\xBAP\xF9\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x91P`\x01\x01a\x10\x06V[Pa\x11\x02\x83\x83\x86a\x1AEV[`\xFF\x84\x16_\x81\x81R`\x05` \x90\x81R`@\x91\x82\x90 C\x90\x81\x90U\x91Q\x91\x82R\x7FF\x07}U3\x07c\xF1bi\xFDu\xE5v\x16c\xF4\x19-'\x91t|\x01\x89\xB1j\xD3\x1D\xB0}\xB4\x91\x01`@Q\x80\x91\x03\x90\xA2PPPP\x80`\x01\x01\x90Pa\x0E\xC6V[`2Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x11{WP`2T`\x01`\xFF\x90\x91\x16\x10[\x80a\x11\x95WP0;\x15\x80\x15a\x11\x95WP`2T`\xFF\x16`\x01\x14[a\x11\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`2\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x12 W`2\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x12v`@Q\x80`@\x01`@R\x80`\x16\x81R` \x01u \xAB)\xA92\xB3\xB4\xB9\xBA9<\xA1\xB7\xB7\xB924\xB70\xBA7\xB9`Q\x1B\x81RP`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01ev0.0.1`\xD0\x1B\x81RPa'\xEEV[a\x12\x7F\x86a(\x1FV[a\x12\x88\x85a\x1D\xA2V[a\x12\x91\x83a\x1DeV[a\x12\x9A\x84a\x1E\x0BV[a\x12\xA3\x82a(pV[\x80\x15a\x12\xE9W`2\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPV[``a\x13\x1C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a(\xDCV[\x90P\x90V[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x03` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x81\x01T\x90\x91\x83\x01\x90`\xFF\x16`\x02\x81\x11\x15a\x13yWa\x13yaF\xAEV[`\x02\x81\x11\x15a\x13\x8AWa\x13\x8AaF\xAEV[\x90RP\x92\x91PPV[`@Qc#}\xFBG`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cF\xFB\xF6\x8E\x90a\x13\xDF\x903\x90`\x04\x01aA\x80V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xFAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x1E\x91\x90aK8V[a\x14;W`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x14E_\x19a\x1DeV[V[a\x14Oa\x1D\x0BV[\x81a\x14Y\x81a)\x19V[a\n\xA4\x83\x83a)AV[a\x14ka)\xD2V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\t` R`@\x90 B\x90Ua\x0CF\x82\x82a)\xFDV[a\x14\x97a\x1D\x0BV[a\x14E_a(\x1FV[_a\x0C\xD2\x7F+\xD8!$\x05\x7F\t\x13\xBC;w,\xE7\xB8>\x80W\xC1\xAD\x1F5\x10\xFC\x83w\x8B\xE2\x0F\x10\xEC]\xE6\x83`@Q` \x01a\x14\xE9\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a*\x93V[a\x15\x0Ca\x1D\x0BV[a\n\xA4\x83\x83\x83__a\"\x9DV[_``\x80___```\xC8T__\x1B\x14\x80\x15a\x155WP`\xC9T\x15[a\x15yW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x11RT\r\xCCL\x8E\x88\x15[\x9A[\x9A]\x1AX[\x1A^\x99Y`Z\x1B`D\x82\x01R`d\x01a\x11\xF4V[a\x15\x81a*\xBFV[a\x15\x89a+OV[`@\x80Q_\x80\x82R` \x82\x01\x90\x92R`\x0F`\xF8\x1B\x9B\x93\x9AP\x91\x98PF\x97P0\x96P\x94P\x92P\x90PV[_a\x15\xF1\x7FM@N2v\xE7\xAC!c\xD8\xEEGj\xFAjA\xD1\xF6\x8F\xB7\x1F-\x8BeF\xB2NU\xCE\x01\xB7*\x87\x87\x87\x87\x87`@Q` \x01a\x14\xE9\x96\x95\x94\x93\x92\x91\x90aK\x87V[\x96\x95PPPPPPV[_a\x0C\xD2\x82a\x19pV[a\x16\ra\x1D\x0BV[a\x0B \x81a(pV[`\x0BT`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x16\x14\x90V[``a\n\xBA`\x02\x84\x84a+^V[a\x16@a\x1EtV[`3T_\x90`\x01\x90\x81\x16\x03a\x16hW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x16q\x85a\x16\x16V[a\x16\x8EW`@Qcf\xE5e\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x16\x98\x85a\x1E\xBDV[\x90P_\x80\x80a\x16\xA9\x86\x88\x01\x88aM\x16V[\x92P\x92P\x92P_a\x16\xBA\x8B\x83a,\rV[\x90P_\x84`\x01\x81\x11\x15a\x16\xCFWa\x16\xCFaF\xAEV[\x03a\x17xW_a\x16\xE3\x8C\x83\x88\x87`\x01a,\xA6V[Q\x90P_[\x86Q\x81\x10\x15a\x17qW_\x87\x82\x81Q\x81\x10a\x17\x04Wa\x17\x04aK\x15V[\x01` \x90\x81\x01Q`\xF8\x1C_\x81\x81R`\x01\x90\x92R`@\x90\x91 T\x84Q\x91\x92Pc\xFF\xFF\xFF\xFF\x16\x90\x84\x90\x84\x90\x81\x10a\x17;Wa\x17;aK\x15V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x17hW`@Qc\xC6\xB9\xE7g`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01\x01a\x16\xE8V[PPa\x17\xCDV[`\x01\x84`\x01\x81\x11\x15a\x17\x8CWa\x17\x8CaF\xAEV[\x03a\x17\xB4W_\x80a\x17\x9F\x89\x8B\x01\x8BaMqV[\x94P\x94PPPPa\x17q\x8D\x84\x89\x88\x86\x86a0\xEDV[`@Qc5K\xB8\xAB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPPPPV[a\x17\xE2a\x1D\x0BV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x18GW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x11\xF4V[a\x0B \x81a(\x1FV[_a\x13\x1Ca2\xE2V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xB5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xD9\x91\x90aKQV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x19\nW`@QcyH!\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`3T\x80\x19\x82\x19\x81\x16\x14a\x191W`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`3\x82\x90U`@Q\x82\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01[`@Q\x80\x91\x03\x90\xA2PPV[_a\x0C\xD2`\x02\x83a2\xEBV[``__a\x19\x89\x84a3UV[a\xFF\xFF\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\xA4Wa\x19\xA4a?\xBBV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x19\xCEW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P_\x80[\x82Q\x82\x10\x80\x15a\x19\xE5WPa\x01\0\x81\x10[\x15a\x1A;W`\x01\x81\x1B\x93P\x85\x84\x16\x15a\x1A+W\x80`\xF8\x1B\x83\x83\x81Q\x81\x10a\x1A\x0EWa\x1A\x0EaK\x15V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP\x81`\x01\x01\x91P[a\x1A4\x81aN~V[\x90Pa\x19\xD4V[P\x90\x94\x93PPPPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x81`\xF8\x1B\x81_\x81Q\x81\x10a\x1A{Wa\x1A{aK\x15V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`@Qcl?\xB4\xBF`\xE0\x1B\x81R_\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cl?\xB4\xBF\x90a\x1A\xE3\x90\x88\x90\x88\x90\x88\x90`\x04\x01aN\x96V[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1A\xFEW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1B%\x91\x90\x81\x01\x90aO&V[\x90P_[\x85Q\x81\x10\x15a\x12\xE9W\x81\x81\x81Q\x81\x10a\x1BDWa\x1BDaK\x15V[` \x02` \x01\x01Q\x15a\x1BtWa\x1Bt\x86\x82\x81Q\x81\x10a\x1BfWa\x1BfaK\x15V[` \x02` \x01\x01Q\x84a)\xFDV[`\x01\x01a\x1B)V[_\x83\x81R` \x85\x90R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x1B\x9EWa\x1B\x9EaK\x15V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x85R`\x01` \x1B\x83\x04\x82\x16\x95\x85\x01\x95\x90\x95R`\x01`@\x1B\x90\x91\x04`\x01`\x01`\xC0\x1B\x03\x16\x91\x83\x01\x91\x90\x91R\x90\x92P\x85\x16\x10\x15a\x1C\x0EW`@Qcl\xB1\x9A\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a\x1C4WP\x80` \x01Qc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10[a\x1CQW`@Qc\xBB\xBA`\xCB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x01Q\x90P[\x94\x93PPPPV[`@Qcx!\x9B?`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xF0C6~\x90a\x1C\xAE\x90\x85\x90\x85\x90`\x04\x01aO\xB3V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1C\xC5W__\xFD[PZ\xF1\x15\x80\x15a\x1C\xD7W=__>=_\xFD[PPPP\x81\x7F\xEC)c\xAB!\xC1\xE5\x0E\x1EX*\xA5B\xAF.K\xF7\xBF8\xE6\xE1@<'\xB4.\x1C]nb\x1E\xAA\x82`@Qa\x19d\x91\x90aF\x9CV[`\x96T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x14EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x11\xF4V[`3\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2PV[`\x07T`@Q\x7F1TW\xD8\xA8\xFE`\xF0J\xF1|\x16\xE2\xF5\xA5\xE1\xDBa+1d\x8EX\x03\x03`u\x9E\xF8\xF3R\x8C\x91a\x1D\xE1\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x84\x90aO\xCBV[`@Q\x80\x91\x03\x90\xA1`\x07\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x08T`@Q\x7F\x8F0\xAB\t\xF4:l\x15}\x7F\xCE~\n\x13\xC0\x03\x04,\x1C\x95\xE8\xA7.z\x14j!\xC0\xCA\xA2M\xC9\x91a\x1EJ\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x84\x90aO\xCBV[`@Q\x80\x91\x03\x90\xA1`\x08\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x14EW`@Qc#\xD8q\xA5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E\xD9Wa\x1E\xD9a?\xBBV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1F\x03W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P_[\x83Q\x81\x10\x15a\x1F^W\x83\x81\x81Q\x81\x10a\x1F#Wa\x1F#aK\x15V[` \x02` \x01\x01Q`\xF8\x1B\x82\x82\x81Q\x81\x10a\x1F@Wa\x1F@aK\x15V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a\x1F\x08V[P\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x03` R`@\x81 \x80T\x90\x91a\x1F\x89\x82a\x19pV[\x90P`\x01\x80\x84\x01T`\xFF\x16`\x02\x81\x11\x15a\x1F\xA5Wa\x1F\xA5aF\xAEV[\x14a\x1F\xC3W`@Qc\xAB\xA4s9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80Ta\x1F\xD4\x90\x86\x90`\xFF\x16a'\xBAV[\x90P`\x01`\x01`\xC0\x1B\x03\x81\x16a\x1F\xFDW`@Qch\xB6\xA8u`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a \x14`\x01`\x01`\xC0\x1B\x03\x82\x81\x16\x90\x84\x16\x81\x16\x14\x90V[a 1W`@Qc\xD0S\xAA!`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xC0\x1B\x03\x81\x81\x16\x19\x83\x16\x16a J\x84\x82a3\x7FV[`\x01`\x01`\xC0\x1B\x03\x81\x16a \xA6W`\x01`\x01`\xA0\x1B\x03\x87\x16_\x81\x81R`\x03` R`@\x80\x82 `\x01\x01\x80T`\xFF\x19\x16`\x02\x17\x90UQ\x86\x92\x91\x7F9o\xDC\xB1\x80\xCB\x0F\xEA&\x92\x81\x13\xFB\x0F\xD1\xC3T\x98c\xF9\xCDV>j\x18O\x1DW\x81\x16\xC8\xE4\x91\xA3[`@Qc\xF4\xE2O\xE5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xF4\xE2O\xE5\x90a \xF4\x90\x8A\x90\x8A\x90`\x04\x01aO\xE5V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a!\x0BW__\xFD[PZ\xF1\x15\x80\x15a!\x1DW=__>=_\xFD[PP`@Qc\xBD)\xB8\xCD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc\xBD)\xB8\xCD\x91Pa!o\x90\x87\x90\x8A\x90`\x04\x01aO\xB3V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a!\x86W__\xFD[PZ\xF1\x15\x80\x15a!\x98W=__>=_\xFD[PP`@Qc\xBD)\xB8\xCD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc\xBD)\xB8\xCD\x91Pa!\xEA\x90\x87\x90\x8A\x90`\x04\x01aO\xB3V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\"\x01W__\xFD[PZ\xF1\x15\x80\x15a\x17\xCDW=__>=_\xFD[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R_\x80\x80a\"@_Q` aV\xEA_9_Q\x90_R\x86aP\x1CV[\x90P[a\"L\x81a3\x8BV[\x90\x93P\x91P_Q` aV\xEA_9_Q\x90_R\x82\x83\t\x83\x03a\"\x84W`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[_Q` aV\xEA_9_Q\x90_R`\x01\x82\x08\x90Pa\"CV[_T`\xFF\x16`\xC0\x81\x10a\"\xC3W`@Qc<\xB8\x9C\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80T`\x01\x91\x90\x81\x90a\"\xDA\x90\x84\x90`\xFF\x16aP/V[\x92Pa\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UPa\"\xFC\x81\x87a)AV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x81R``` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a#\x12W\x90PP\x90P_\x85Q`\x01`\x01`@\x1B\x03\x81\x11\x15a#UWa#Ua?\xBBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a#~W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x86Q\x81\x10\x15a#\xDBW\x86\x81\x81Q\x81\x10a#\x9EWa#\x9EaK\x15V[` \x02` \x01\x01Q_\x01Q\x82\x82\x81Q\x81\x10a#\xBBWa#\xBBaK\x15V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a#\x83V[P`@Q\x80`@\x01`@R\x80\x84`\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82\x81RP\x82_\x81Q\x81\x10a$\x0CWa$\x0CaK\x15V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x0BT`@Qc\x010\xFC'`\xE5\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x92c&\x1F\x84\xE0\x92a$l\x92\x91\x90\x91\x16\x90\x86\x90`\x04\x01aPHV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a$\x83W__\xFD[PZ\xF1\x15\x80\x15a$\x95W=__>=_\xFD[P_\x92Pa$\xA1\x91PPV[\x85`\x01\x81\x11\x15a$\xB3Wa$\xB3aF\xAEV[\x03a%:W`@Qc:\xEA\x0B\x9D`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cu\xD4\x17:\x90a%\x08\x90\x86\x90\x8B\x90\x8B\x90`\x04\x01aQXV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a%\x1FW__\xFD[PZ\xF1\x15\x80\x15a%1W=__>=_\xFD[PPPPa&~V[`\x01\x85`\x01\x81\x11\x15a%NWa%NaF\xAEV[\x03a&~W\x83c\xFF\xFF\xFF\xFF\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c)\x81\xEBw`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%\xB6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%\xDA\x91\x90aKlV[c\xFF\xFF\xFF\xFF\x16\x11a%\xFEW`@Qc\x0B\xD4A\xB9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\x06b\xD3\xE1`\xE5\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xCCZ| \x90a&P\x90\x86\x90\x8B\x90\x89\x90\x8C\x90`\x04\x01aQ\x8BV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a&gW__\xFD[PZ\xF1\x15\x80\x15a&yW=__>=_\xFD[PPPP[`@Qc\x13l\xA0\xF9`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c&\xD9A\xF2\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a&\xDFW__\xFD[PZ\xF1\x15\x80\x15a&\xF1W=__>=_\xFD[PP`@Qc\x13l\xA0\xF9`\xE1\x1B\x81R`\xFF\x86\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92Pc&\xD9A\xF2\x91P`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a'VW__\xFD[PZ\xF1\x15\x80\x15a'hW=__>=_\xFD[PPPP\x82`\xFF\x16\x7F\xA3H5\xBC+g>\xC3\x7F\xCF\x15\x91\xA9\x12\x95\xB1c\xFC.\x18\x1EN\xA4\xE73\xBE\xB2}\xE1\xCE\xACL\x89\x89\x89\x89\x89`@Qa'\xA7\x95\x94\x93\x92\x91\x90aQ\xC1V[`@Q\x80\x91\x03\x90\xA2[PPPPPPPPV[__a'\xC5\x84a4\x07V[\x90P\x80\x83`\xFF\x16`\x01\x90\x1B\x11a\n\xBAW`@Qc\xCA\x95s3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`2Ta\x01\0\x90\x04`\xFF\x16a(\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x11\xF4\x90aRBV[a\x0CF\x82\x82a4\xC2V[`\x96\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPV[`\x0BT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x7F\x97p\xF3\xCA\xDF\xDC\xBBo\x93\xAF\x93^\x86\x04q\x11Y\x0C7h'\x1D#~J+\xC0\xB8t\xBE\xD6\x93\x90a(\xB1\x90\x83\x90\x85\x90aO\xCBV[`@Q\x80\x91\x03\x90\xA1P`\x0B\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[``_a(\xE8\x83a5\x10V[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P_\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[_T`\xFF\x90\x81\x16\x90\x82\x16\x10a\x0B W`@Qcs\x10\xCF\xF5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x82\x16_\x81\x81R`\x01` \x90\x81R`@\x91\x82\x90 \x84Q\x81T\x92\x86\x01Q\x84\x87\x01Qa\xFF\xFF\x90\x81\x16`\x01`0\x1B\x02g\xFF\xFF\0\0\0\0\0\0\x19\x91\x90\x92\x16`\x01` \x1B\x02e\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x95\x16c\xFF\xFF\xFF\xFF\x90\x93\x16\x92\x90\x92\x17\x93\x90\x93\x17\x16\x91\x90\x91\x17\x90UQ\x7F>\xE6\xFE\x8DTa\x02D\xC3\xE9\xD3\xC0f\xAEJ\xEE\x99x\x84\xAA(\xF1\x06\x16\xAE\x82\x19%@\x13\x18\xAC\x90a\x19d\x90\x84\x90aJ\xD4V[`\x08T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x14EW`@Qcv\xD8\xAB\x17`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x03` R`@\x90 `\x01\x80\x82\x01T`\xFF\x16`\x02\x81\x11\x15a*.Wa*.aF\xAEV[\x14a*LW`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80T_\x80Ta*_\x90\x85\x90`\xFF\x16a'\xBAV[\x90P_a*k\x83a\x19pV[\x90Pa*\x84`\x01`\x01`\xC0\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x16\x14\x90V[\x15a\x12\xE9Wa\x12\xE9\x86\x86a57V[_a\x0C\xD2a*\x9Fa2\xE2V[\x83`@Qa\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x90 \x90V[```\xCA\x80Ta*\xCE\x90aR\x8DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta*\xFA\x90aR\x8DV[\x80\x15a+EW\x80`\x1F\x10a+\x1CWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a+EV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a+(W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[```\xCB\x80Ta*\xCE\x90aR\x8DV[``_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a+zWa+za?\xBBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a+\xA3W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83Q\x81\x10\x15a,\x04Wa+\xD5\x86\x86\x86\x84\x81Q\x81\x10a+\xC8Wa+\xC8aK\x15V[` \x02` \x01\x01Qa5\xD5V[\x82\x82\x81Q\x81\x10a+\xE7Wa+\xE7aK\x15V[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a+\xA8V[P\x94\x93PPPPV[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x03\xC5\xA6\xB6\x84\x84a,H\x87a\r\xF7V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a,f\x93\x92\x91\x90aR\xE7V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a,\x82W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xBA\x91\x90aS`V[a,\xCA`@Q\x80``\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81RP\x90V[_\x80Ta,\xDB\x90\x86\x90`\xFF\x16a'\xBAV[\x90P_a,\xE7\x87a\x19pV[\x90P`\x01`\x01`\xC0\x1B\x03\x82\x16a-\x10W`@Qc\x13\xCAFW`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x82\x16`\x01`\x01`\xC0\x1B\x03\x16\x15a-:W`@Qc\x0Ch\x16\xCD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\nT`\x01`\x01`\xA0\x1B\x03\x89\x16_\x90\x81R`\t` R`@\x90 T`\x01`\x01`\xC0\x1B\x03\x83\x81\x16\x90\x85\x16\x17\x91B\x91a-q\x91\x90aSwV[\x10a-\x8FW`@Qc\x19hg}`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a-\x99\x88\x82a3\x7FV[a-\xA3\x88\x87a\x1C`V[`\x01`\x01`\x01`\xA0\x1B\x03\x8A\x16_\x90\x81R`\x03` R`@\x90 `\x01\x01T`\xFF\x16`\x02\x81\x11\x15a-\xD4Wa-\xD4aF\xAEV[\x14a.kW`@\x80Q\x80\x82\x01\x82R\x89\x81R`\x01` \x80\x83\x01\x82\x81R`\x01`\x01`\xA0\x1B\x03\x8E\x16_\x90\x81R`\x03\x90\x92R\x93\x90 \x82Q\x81U\x92Q\x83\x82\x01\x80T\x93\x94\x93\x91\x92\x90\x91`\xFF\x19\x16\x90\x83`\x02\x81\x11\x15a..Wa..aF\xAEV[\x02\x17\x90UPP`@Q\x89\x91P`\x01`\x01`\xA0\x1B\x03\x8B\x16\x90\x7F\xE8\xE6\x8C\xEF\x1C:v\x1E\xD7\xBE~\x84c\xA3u\xF2\x7F{\xC35\xE5\x18$\"<\xAC\xCEcn\xC5\xC3\xFE\x90_\x90\xA3[`@Qc\x1F\xD9<\xA9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c?\xB2yR\x90a.\xB9\x90\x8C\x90\x8B\x90`\x04\x01aO\xE5V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a.\xD0W__\xFD[PZ\xF1\x15\x80\x15a.\xE2W=__>=_\xFD[PP`@Qc%PGw`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc%PGw\x91Pa/6\x90\x8C\x90\x8C\x90\x8C\x90`\x04\x01aS\x8AV[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a/QW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra/x\x91\x90\x81\x01\x90aT\x14V[`@\x80\x87\x01\x91\x90\x91R` \x86\x01\x91\x90\x91RQb\xBF\xF0M`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90b\xBF\xF0M\x90a/\xD3\x90\x8B\x90\x8B\x90`\x04\x01aO\xB3V[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a/\xEEW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra0\x15\x91\x90\x81\x01\x90aTmV[\x84R\x84\x15a0\xE1W_[\x87Q\x81\x10\x15a0\xDFW_`\x01_\x8A\x84\x81Q\x81\x10a0>Wa0>aK\x15V[\x01` \x90\x81\x01Q`\xF8\x1C\x82R\x81\x81\x01\x92\x90\x92R`@\x90\x81\x01_ \x81Q``\x81\x01\x83R\x90Tc\xFF\xFF\xFF\xFF\x81\x16\x80\x83Ra\xFF\xFF`\x01` \x1B\x83\x04\x81\x16\x95\x84\x01\x95\x90\x95R`\x01`0\x1B\x90\x91\x04\x90\x93\x16\x91\x81\x01\x91\x90\x91R\x87Q\x80Q\x91\x93P\x90\x84\x90\x81\x10a0\xA9Wa0\xA9aK\x15V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a0\xD6W`@Qc\xC6\xB9\xE7g`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01\x01a0\x1FV[P[PPP\x95\x94PPPPPV[\x83Q\x82Q\x14a1\x0FW`@Qc\xAA\xAD\x13\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a1\x1B\x86\x86\x84\x84a6\xEDV[_a1)\x87\x87\x87\x87_a,\xA6V[\x90P_[\x85Q\x81\x10\x15a'\xB0W_`\x01_\x88\x84\x81Q\x81\x10a1LWa1LaK\x15V[\x01` \x90\x81\x01Q`\xF8\x1C\x82R\x81\x81\x01\x92\x90\x92R`@\x90\x81\x01_ \x81Q``\x81\x01\x83R\x90Tc\xFF\xFF\xFF\xFF\x81\x16\x80\x83Ra\xFF\xFF`\x01` \x1B\x83\x04\x81\x16\x95\x84\x01\x95\x90\x95R`\x01`0\x1B\x90\x91\x04\x90\x93\x16\x91\x81\x01\x91\x90\x91R\x84Q\x80Q\x91\x93P\x90\x84\x90\x81\x10a1\xB7Wa1\xB7aK\x15V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a2\xD9Wa2K\x87\x83\x81Q\x81\x10a1\xE0Wa1\xE0aK\x15V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x84`@\x01Q\x84\x81Q\x81\x10a2\x04Wa2\x04aK\x15V[` \x02` \x01\x01Q\x8B\x86` \x01Q\x86\x81Q\x81\x10a2#Wa2#aK\x15V[` \x02` \x01\x01Q\x89\x87\x81Q\x81\x10a2=Wa2=aK\x15V[` \x02` \x01\x01Q\x86a7\x98V[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x87\x83\x81Q\x81\x10a2}Wa2}aK\x15V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x81_\x81Q\x81\x10a2\x9AWa2\x9AaK\x15V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SPa2\xD7\x86\x84\x81Q\x81\x10a2\xC5Wa2\xC5aK\x15V[` \x02` \x01\x01Q` \x01Q\x82a)\xFDV[P[P`\x01\x01a1-V[_a\x13\x1Ca9\x19V[_\x81\x81R` \x83\x90R`@\x81 T\x80\x82\x03a3\tW_\x91PPa\x0C\xD2V[_\x83\x81R` \x85\x90R`@\x90 a3!`\x01\x83aT\xFCV[\x81T\x81\x10a31Wa31aK\x15V[_\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01`\xC0\x1B\x03\x16\x91Pa\x0C\xD2\x90PV[_\x80[\x82\x15a\x0C\xD2Wa3i`\x01\x84aT\xFCV[\x90\x92\x16\x91\x80a3w\x81aU\x0FV[\x91PPa3XV[a\x0CF`\x02\x83\x83a9\x8CV[_\x80\x80_Q` aV\xEA_9_Q\x90_R`\x03_Q` aV\xEA_9_Q\x90_R\x86_Q` aV\xEA_9_Q\x90_R\x88\x89\t\t\x08\x90P_a3\xFB\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R_Q` aV\xEA_9_Q\x90_Ra;EV[\x91\x95\x91\x94P\x90\x92PPPV[_a\x01\0\x82Q\x11\x15a4,W`@Qc}\xA5NG`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81Q_\x03a4;WP_\x91\x90PV[__\x83_\x81Q\x81\x10a4OWa4OaK\x15V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x81\x90\x1B\x92P[\x84Q\x81\x10\x15a4\xB9W\x84\x81\x81Q\x81\x10a4}Wa4}aK\x15V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x1B\x91P\x82\x82\x11a4\xADW`@Qc\x10\x19\x10i`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91\x81\x17\x91`\x01\x01a4bV[P\x90\x93\x92PPPV[`2Ta\x01\0\x90\x04`\xFF\x16a4\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x11\xF4\x90aRBV[`\xCAa4\xF5\x83\x82aUsV[P`\xCBa5\x02\x82\x82aUsV[PP_`\xC8\x81\x90U`\xC9UPV[_`\xFF\x82\x16`\x1F\x81\x11\x15a\x0C\xD2W`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x82R`\x0BT\x81\x16` \x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91cn4\x92\xB5\x91\x90\x81\x01a5\x8D\x85a;\xBEV[\x81RP`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a5\xAC\x91\x90aV-V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a5\xC3W__\xFD[PZ\xF1\x15\x80\x15a\x12\xE9W=__>=_\xFD[_\x81\x81R` \x84\x90R`@\x81 T\x81[\x81\x81\x10\x15a6XW`\x01a5\xF9\x82\x84aT\xFCV[a6\x03\x91\x90aT\xFCV[\x92P\x84c\xFF\xFF\xFF\xFF\x16\x86_\x86\x81R` \x01\x90\x81R` \x01_ \x84c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a63Wa63aK\x15V[_\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a6PWPPa\n\xBAV[`\x01\x01a5\xE5V[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FRegistryCoordinator.getQuorumBit`D\x82\x01R\x7FmapIndexAtBlockNumber: no bitmap`d\x82\x01R\x7F update found for operatorId\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x11\xF4V[` \x80\x82\x01Q_\x90\x81R`\x04\x90\x91R`@\x90 T`\xFF\x16\x15a7\"W`@Qco\xBE\xFE\xC3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[B\x81`@\x01Q\x10\x15a7GW`@Qc\x08\x19\xBD\xCD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x80\x82\x01\x80Q_\x90\x81R`\x04\x90\x92R`@\x91\x82\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U`\x07T\x90Q\x91\x83\x01Qa\x0E1\x92`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91a7\x91\x91\x88\x91\x88\x91\x88\x91\x90a\x15\xB2V[\x83Qa<cV[` \x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x80\x82\x16_\x81\x81R`\x03\x90\x94R`@\x90\x93 T\x91\x92\x90\x87\x16\x03a7\xDBW`@QcV\x16\x8BA`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x87`\xFF\x16\x84_\x01Q`\xFF\x16\x14a8\x04W`@Qc\x8EZ\xEE\xE7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@QcT\x01\xED'`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`\xFF\x89\x16`$\x82\x01R_\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cT\x01\xED'\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8rW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8\x96\x91\x90aViV[\x90Pa8\xA2\x81\x85a<\x8BV[`\x01`\x01``\x1B\x03\x16\x86`\x01`\x01``\x1B\x03\x16\x11a8\xD3W`@QcLD\x99]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a8\xDD\x88\x85a<\xAEV[`\x01`\x01``\x1B\x03\x16\x81`\x01`\x01``\x1B\x03\x16\x10a9\x0EW`@Qc\xB1\x87\xE8i`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPPV[_\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fa9Ca<\xC7V[a9Ka=\x1FV[`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[_\x82\x81R` \x84\x90R`@\x81 T\x90\x81\x90\x03a:0W_\x83\x81R` \x85\x81R`@\x80\x83 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x86\x81R`\x01`\x01`\xC0\x1B\x03\x80\x8A\x16\x95\x84\x01\x95\x86R\x84T`\x01\x81\x01\x86U\x94\x88R\x95\x90\x96 \x91Q\x91\x90\x92\x01\x80T\x95Q\x93Q\x90\x94\x16`\x01`@\x1B\x02`\x01`\x01`@\x1B\x03\x93\x83\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x96\x16\x91\x90\x92\x16\x17\x93\x90\x93\x17\x16\x91\x90\x91\x17\x90Ua\x0E1V[_\x83\x81R` \x85\x90R`@\x81 a:H`\x01\x84aT\xFCV[\x81T\x81\x10a:XWa:XaK\x15V[_\x91\x82R` \x90\x91 \x01\x80T\x90\x91Pc\xFF\xFF\xFF\xFFC\x81\x16\x91\x16\x03a:\x99W\x80T`\x01`\x01`@\x1B\x03\x16`\x01`@\x1B`\x01`\x01`\xC0\x1B\x03\x85\x16\x02\x17\x81Ua\r\xF0V[\x80Tc\xFF\xFF\xFF\xFFC\x81\x16`\x01` \x1B\x81\x81\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x94\x16\x93\x90\x93\x17\x84U_\x87\x81R` \x89\x81R`@\x80\x83 \x81Q``\x81\x01\x83R\x94\x85R\x84\x83\x01\x84\x81R`\x01`\x01`\xC0\x1B\x03\x80\x8C\x16\x93\x87\x01\x93\x84R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x94Q\x94\x01\x80T\x93Q\x91Q\x90\x92\x16`\x01`@\x1B\x02`\x01`\x01`@\x1B\x03\x91\x86\x16\x90\x96\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x93\x90\x94\x16\x92\x90\x92\x17\x17\x91\x90\x91\x16\x91\x90\x91\x17\x90UPPPPPV[__a;Oa?\x7FV[a;Wa?\x9DV[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80a;\x94W\xFE[P\x82a;\xB3W`@Qc\xD5\x1E\xDA\xE3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PQ\x95\x94PPPPPV[``_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a;\xDAWa;\xDAa?\xBBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a<\x03W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83Q\x81\x10\x15a\x1F^W\x83\x81\x81Q\x81\x10a<#Wa<#aK\x15V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16\x82\x82\x81Q\x81\x10a<FWa<FaK\x15V[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a<\x08V[a<n\x83\x83\x83a=OV[a\n\xA4W`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Q_\x90a'\x10\x90a<\xA4\x90a\xFF\xFF\x16\x85aV\x84V[a\n\xBA\x91\x90aV\xA6V[`@\x81\x01Q_\x90a'\x10\x90a<\xA4\x90a\xFF\xFF\x16\x85aV\x84V[__a<\xD1a*\xBFV[\x80Q\x90\x91P\x15a<\xE8W\x80Q` \x90\x91\x01 \x91\x90PV[`\xC8T\x80\x15a<\xF7W\x92\x91PPV[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x92PPP\x90V[__a=)a+OV[\x80Q\x90\x91P\x15a=@W\x80Q` \x90\x91\x01 \x91\x90PV[`\xC9T\x80\x15a<\xF7W\x92\x91PPV[___a=\\\x85\x85a=\xA3V[\x90\x92P\x90P_\x81`\x04\x81\x11\x15a=tWa=taF\xAEV[\x14\x80\x15a=\x92WP\x85`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14[\x80a\x15\xF1WPa\x15\xF1\x86\x86\x86a=\xE5V[__\x82Q`A\x03a=\xD7W` \x83\x01Q`@\x84\x01Q``\x85\x01Q_\x1Aa=\xCB\x87\x82\x85\x85a>\xCCV[\x94P\x94PPPPa=\xDEV[P_\x90P`\x02[\x92P\x92\x90PV[___\x85`\x01`\x01`\xA0\x1B\x03\x16c\x16&\xBA~`\xE0\x1B\x86\x86`@Q`$\x01a>\r\x92\x91\x90aO\xB3V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90Qa>K\x91\x90aV\xD3V[_`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80_\x81\x14a>\x83W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a>\x88V[``\x91P[P\x91P\x91P\x81\x80\x15a>\x9CWP` \x81Q\x10\x15[\x80\x15a\x15\xF1WP\x80Qc\x0B\x13]?`\xE1\x1B\x90a>\xC1\x90\x83\x01` \x90\x81\x01\x90\x84\x01aS`V[\x14\x96\x95PPPPPPV[_\x80o\xA2\xA8\x91\x8C\xA8[\xAF\xE2 \x16\xD0\xB9\x97\xE4\xDF``\x01`\xFF\x1B\x03\x83\x11\x15a>\xF7WP_\x90P`\x03a?vV[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a?HW=__>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a?pW_`\x01\x92P\x92PPa?vV[\x91P_\x90P[\x94P\x94\x92PPPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a?\xF1Wa?\xF1a?\xBBV[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a?\xF1Wa?\xF1a?\xBBV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a@AWa@Aa?\xBBV[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a@aWa@aa?\xBBV[P`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0B W__\xFD[_\x82`\x1F\x83\x01\x12a@\x8EW__\xFD[\x815a@\xA1a@\x9C\x82a@IV[a@\x19V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a@\xC2W__\xFD[` \x85\x01[\x83\x81\x10\x15a@\xE8W\x805a@\xDA\x81a@kV[\x83R` \x92\x83\x01\x92\x01a@\xC7V[P\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15aA\x02W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aA\x17W__\xFD[a\x1CX\x84\x82\x85\x01a@\x7FV[_` \x82\x84\x03\x12\x15aA3W__\xFD[P5\x91\x90PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0B W__\xFD[___``\x84\x86\x03\x12\x15aA]W__\xFD[\x835\x92P` \x84\x015aAo\x81aA:V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[_\x82`\x1F\x83\x01\x12aA\xA3W__\xFD[\x815` \x83\x01__`\x01`\x01`@\x1B\x03\x84\x11\x15aA\xC2WaA\xC2a?\xBBV[P`\x1F\x83\x01`\x1F\x19\x16` \x01aA\xD7\x81a@\x19V[\x91PP\x82\x81R\x85\x83\x83\x01\x11\x15aA\xEBW__\xFD[\x82\x82` \x83\x017_\x92\x81\x01` \x01\x92\x90\x92RP\x93\x92PPPV[_` \x82\x84\x03\x12\x15aB\x15W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aB*W__\xFD[a\x1CX\x84\x82\x85\x01aA\x94V[_` \x82\x84\x03\x12\x15aBFW__\xFD[\x815a\n\xBA\x81a@kV[__`@\x83\x85\x03\x12\x15aBbW__\xFD[PP\x805\x92` \x90\x91\x015\x91PV[\x805`\xFF\x81\x16\x81\x14aB\x81W__\xFD[\x91\x90PV[_` \x82\x84\x03\x12\x15aB\x96W__\xFD[a\n\xBA\x82aBqV[_\x82`\x1F\x83\x01\x12aB\xAEW__\xFD[\x815aB\xBCa@\x9C\x82a@IV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aB\xDDW__\xFD[` \x85\x01[\x83\x81\x10\x15a@\xE8W\x805aB\xF5\x81aA:V[\x83R` \x92\x83\x01\x92\x01aB\xE2V[___``\x84\x86\x03\x12\x15aC\x15W__\xFD[\x835aC \x81a@kV[\x92P` \x84\x015aC0\x81a@kV[\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aCJW__\xFD[aCV\x86\x82\x87\x01aB\x9FV[\x91PP\x92P\x92P\x92V[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x81\x01a\x0C\xD2V[\x805a\xFF\xFF\x81\x16\x81\x14aB\x81W__\xFD[_``\x82\x84\x03\x12\x15aC\x98W__\xFD[aC\xA0a?\xCFV[\x90P\x815aC\xAD\x81aA:V[\x81RaC\xBB` \x83\x01aCwV[` \x82\x01RaC\xCC`@\x83\x01aCwV[`@\x82\x01R\x92\x91PPV[`\x01`\x01``\x1B\x03\x81\x16\x81\x14a\x0B W__\xFD[_\x82`\x1F\x83\x01\x12aC\xFAW__\xFD[\x815aD\x08a@\x9C\x82a@IV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aD)W__\xFD[` \x85\x01[\x83\x81\x10\x15a@\xE8W`@\x81\x88\x03\x12\x15aDEW__\xFD[aDMa?\xF7V[\x815aDX\x81a@kV[\x81R` \x82\x015aDh\x81aC\xD7V[` \x82\x81\x01\x91\x90\x91R\x90\x84R\x92\x90\x92\x01\x91`@\x01aD.V[____`\xC0\x85\x87\x03\x12\x15aD\x94W__\xFD[aD\x9E\x86\x86aC\x88V[\x93P``\x85\x015aD\xAE\x81aC\xD7V[\x92P`\x80\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aD\xC8W__\xFD[aD\xD4\x87\x82\x88\x01aC\xEBV[\x92PP`\xA0\x85\x015aD\xE5\x81aA:V[\x93\x96\x92\x95P\x90\x93PPV[__\x83`\x1F\x84\x01\x12aE\0W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aE\x16W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a=\xDEW__\xFD[___`@\x84\x86\x03\x12\x15aE?W__\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15aETW__\xFD[\x84\x01`\x1F\x81\x01\x86\x13aEdW__\xFD[\x805aEra@\x9C\x82a@IV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x88\x83\x11\x15aE\x93W__\xFD[` \x84\x01[\x83\x81\x10\x15aE\xD3W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aE\xB5W__\xFD[aE\xC4\x8B` \x83\x89\x01\x01a@\x7FV[\x84RP` \x92\x83\x01\x92\x01aE\x98V[P\x95PPPP` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aE\xF1W__\xFD[aE\xFD\x86\x82\x87\x01aD\xF0V[\x94\x97\x90\x96P\x93\x94PPPPV[_____`\xA0\x86\x88\x03\x12\x15aF\x1EW__\xFD[\x855aF)\x81a@kV[\x94P` \x86\x015aF9\x81a@kV[\x93P`@\x86\x015aFI\x81a@kV[\x92P``\x86\x015\x91P`\x80\x86\x015aF`\x81a@kV[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\n\xBA` \x83\x01\x84aFnV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x03\x81\x10aF\xD2WaF\xD2aF\xAEV[\x90RV[\x81Q\x81R` \x80\x83\x01Q`@\x83\x01\x91a\x1F^\x90\x84\x01\x82aF\xC2V[__`\x80\x83\x85\x03\x12\x15aG\x02W__\xFD[aG\x0B\x83aBqV[\x91PaG\x1A\x84` \x85\x01aC\x88V[\x90P\x92P\x92\x90PV[__`@\x83\x85\x03\x12\x15aG4W__\xFD[\x825aG?\x81a@kV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aGYW__\xFD[aGe\x85\x82\x86\x01aA\x94V[\x91PP\x92P\x92\x90PV[___`\xA0\x84\x86\x03\x12\x15aG\x81W__\xFD[aG\x8B\x85\x85aC\x88V[\x92P``\x84\x015aG\x9B\x81aC\xD7V[\x91P`\x80\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aG\xB5W__\xFD[aCV\x86\x82\x87\x01aC\xEBV[`\xFF`\xF8\x1B\x88\x16\x81R`\xE0` \x82\x01R_aG\xDF`\xE0\x83\x01\x89aFnV[\x82\x81\x03`@\x84\x01RaG\xF1\x81\x89aFnV[``\x84\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x85\x01R`\xA0\x84\x01\x86\x90R\x83\x81\x03`\xC0\x85\x01R\x84Q\x80\x82R` \x80\x87\x01\x93P\x90\x91\x01\x90_[\x81\x81\x10\x15aHFW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aH(V[P\x90\x9B\x9APPPPPPPPPPPV[_\x82`\x1F\x83\x01\x12aHfW__\xFD[\x815aHta@\x9C\x82a@IV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aH\x95W__\xFD[` \x85\x01[\x83\x81\x10\x15a@\xE8W`@\x81\x88\x03\x12\x15aH\xB1W__\xFD[aH\xB9a?\xF7V[aH\xC2\x82aBqV[\x81R` \x82\x015aH\xD2\x81a@kV[` \x82\x81\x01\x91\x90\x91R\x90\x84R\x92\x90\x92\x01\x91`@\x01aH\x9AV[_____`\xA0\x86\x88\x03\x12\x15aH\xFFW__\xFD[\x855aI\n\x81a@kV[\x94P` \x86\x015\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aI+W__\xFD[aI7\x88\x82\x89\x01aHWV[\x95\x98\x94\x97P\x94\x95``\x81\x015\x95P`\x80\x015\x93\x92PPPV[__`@\x83\x85\x03\x12\x15aIaW__\xFD[\x825aIl\x81aA:V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aI\x86W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13aI\x96W__\xFD[\x805aI\xA4a@\x9C\x82a@IV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15aI\xC5W__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aI\xE7W\x835\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aI\xCCV[\x80\x94PPPPP\x92P\x92\x90PV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aJ+W\x81Qc\xFF\xFF\xFF\xFF\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aJ\x07V[P\x93\x94\x93PPPPV[` \x81R_a\n\xBA` \x83\x01\x84aI\xF5V[_____`\x80\x86\x88\x03\x12\x15aJ[W__\xFD[\x855aJf\x81a@kV[\x94P` \x86\x015aJv\x81a@kV[\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\x90W__\xFD[aJ\x9C\x88\x82\x89\x01aB\x9FV[\x93PP``\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\xB7W__\xFD[aJ\xC3\x88\x82\x89\x01aD\xF0V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[``\x81\x01a\x0C\xD2\x82\x84c\xFF\xFF\xFF\xFF\x81Q\x16\x82Ra\xFF\xFF` \x82\x01Q\x16` \x83\x01Ra\xFF\xFF`@\x82\x01Q\x16`@\x83\x01RPPV[` \x81\x01a\x0C\xD2\x82\x84aF\xC2V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80Q\x80\x15\x15\x81\x14aB\x81W__\xFD[_` \x82\x84\x03\x12\x15aKHW__\xFD[a\n\xBA\x82aK)V[_` \x82\x84\x03\x12\x15aKaW__\xFD[\x81Qa\n\xBA\x81a@kV[_` \x82\x84\x03\x12\x15aK|W__\xFD[\x81Qa\n\xBA\x81aA:V[_`\xC0\x82\x01\x88\x83R`\x01\x80`\xA0\x1B\x03\x88\x16` \x84\x01R\x86`@\x84\x01R`\xC0``\x84\x01R\x80\x86Q\x80\x83R`\xE0\x85\x01\x91P` \x88\x01\x92P_[\x81\x81\x10\x15aK\xF4W\x83Q\x80Q`\xFF\x16\x84R` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81\x85\x01R\x90\x93\x01\x92`@\x90\x92\x01\x91`\x01\x01aK\xBEV[PP`\x80\x84\x01\x95\x90\x95RPP`\xA0\x01R\x94\x93PPPPV[\x805`\x02\x81\x10aB\x81W__\xFD[_`@\x82\x84\x03\x12\x15aL*W__\xFD[aL2a?\xF7V[\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[_\x82`\x1F\x83\x01\x12aLXW__\xFD[aL`a?\xF7V[\x80`@\x84\x01\x85\x81\x11\x15aLqW__\xFD[\x84[\x81\x81\x10\x15aL\x8BW\x805\x84R` \x93\x84\x01\x93\x01aLsV[P\x90\x95\x94PPPPPV[_\x81\x83\x03a\x01\0\x81\x12\x15aL\xA8W__\xFD[aL\xB0a?\xCFV[\x91PaL\xBC\x84\x84aL\x1AV[\x82RaL\xCB\x84`@\x85\x01aL\x1AV[` \x83\x01R`\x80`\x7F\x19\x82\x01\x12\x15aL\xE1W__\xFD[PaL\xEAa?\xF7V[aL\xF7\x84`\x80\x85\x01aLIV[\x81RaM\x06\x84`\xC0\x85\x01aLIV[` \x82\x01R`@\x82\x01R\x92\x91PPV[___a\x01@\x84\x86\x03\x12\x15aM)W__\xFD[aM2\x84aL\x0CV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aMLW__\xFD[aMX\x86\x82\x87\x01aA\x94V[\x92PPaMh\x85`@\x86\x01aL\x96V[\x90P\x92P\x92P\x92V[_____a\x01\x80\x86\x88\x03\x12\x15aM\x86W__\xFD[aM\x8F\x86aL\x0CV[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aM\xA9W__\xFD[aM\xB5\x88\x82\x89\x01aA\x94V[\x94PPaM\xC5\x87`@\x88\x01aL\x96V[\x92Pa\x01@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aM\xE0W__\xFD[aM\xEC\x88\x82\x89\x01aHWV[\x92PPa\x01`\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aN\x08W__\xFD[\x86\x01``\x81\x89\x03\x12\x15aN\x19W__\xFD[aN!a?\xCFV[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aN6W__\xFD[aNB\x8A\x82\x85\x01aA\x94V[\x82RP` \x82\x81\x015\x90\x82\x01R`@\x91\x82\x015\x91\x81\x01\x91\x90\x91R\x94\x97\x93\x96P\x91\x94P\x92\x91\x90PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_`\x01\x82\x01aN\x8FWaN\x8FaNjV[P`\x01\x01\x90V[``\x80\x82R\x84Q\x90\x82\x01\x81\x90R_\x90` \x86\x01\x90`\x80\x84\x01\x90\x83[\x81\x81\x10\x15aN\xD8W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aN\xB1V[PP\x83\x81\x03` \x80\x86\x01\x91\x90\x91R\x86Q\x80\x83R\x91\x81\x01\x92P\x86\x01\x90_[\x81\x81\x10\x15aO\x13W\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aN\xF5V[PPP`\xFF\x84\x16`@\x84\x01R\x90Pa\x1CXV[_` \x82\x84\x03\x12\x15aO6W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aOKW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aO[W__\xFD[\x80QaOia@\x9C\x82a@IV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15aO\x8AW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a\x15\xF1WaO\xA2\x84aK)V[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aO\x91V[\x82\x81R`@` \x82\x01R_a\n\xB7`@\x83\x01\x84aFnV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x16` \x82\x01R`@\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90a\n\xB7\x90\x83\x01\x84aFnV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82aP*WaP*aP\x08V[P\x06\x90V[`\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x0C\xD2Wa\x0C\xD2aNjV[_`@\x82\x01`\x01\x80`\xA0\x1B\x03\x85\x16\x83R`@` \x84\x01R\x80\x84Q\x80\x83R``\x85\x01\x91P``\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aP\xFDW\x86\x85\x03`_\x19\x01\x84R\x81Q\x80Qc\xFF\xFF\xFF\xFF\x16\x86R` \x90\x81\x01Q`@\x82\x88\x01\x81\x90R\x81Q\x90\x88\x01\x81\x90R\x91\x01\x90_\x90``\x88\x01\x90[\x80\x83\x10\x15aP\xE5W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x93\x84\x01\x93`\x01\x93\x90\x93\x01\x92\x90\x91\x01\x90aP\xBAV[P\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aP|V[P\x92\x97\x96PPPPPPPV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aJ+W\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x87R` \x90\x81\x01Q`\x01`\x01``\x1B\x03\x16\x81\x88\x01R`@\x90\x96\x01\x95\x90\x91\x01\x90`\x01\x01aQ\x1CV[`\xFF\x84\x16\x81R`\x01`\x01``\x1B\x03\x83\x16` \x82\x01R```@\x82\x01R_aQ\x82``\x83\x01\x84aQ\nV[\x95\x94PPPPPV[`\xFF\x85\x16\x81R`\x01`\x01``\x1B\x03\x84\x16` \x82\x01Rc\xFF\xFF\xFF\xFF\x83\x16`@\x82\x01R`\x80``\x82\x01R_a\x15\xF1`\x80\x83\x01\x84aQ\nV[aQ\xF0\x81\x87c\xFF\xFF\xFF\xFF\x81Q\x16\x82Ra\xFF\xFF` \x82\x01Q\x16` \x83\x01Ra\xFF\xFF`@\x82\x01Q\x16`@\x83\x01RPPV[`\x01`\x01``\x1B\x03\x85\x16``\x82\x01R`\xE0`\x80\x82\x01R_aR\x14`\xE0\x83\x01\x86aQ\nV[\x90P`\x02\x84\x10aR&WaR&aF\xAEV[\x83`\xA0\x83\x01Rc\xFF\xFF\xFF\xFF\x83\x16`\xC0\x83\x01R\x96\x95PPPPPPV[` \x80\x82R`+\x90\x82\x01R\x7FInitializable: contract is not i`@\x82\x01Rjnitializing`\xA8\x1B``\x82\x01R`\x80\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80aR\xA1W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aR\xBFWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[\x80_[`\x02\x81\x10\x15a\x0E1W\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01aR\xC8V[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R\x82Q\x80Q` \x80\x84\x01\x91\x90\x91R\x01Q`@\x82\x01Ra\x01`\x81\x01` \x84\x81\x01Q\x80Q``\x85\x01R\x90\x81\x01Q`\x80\x84\x01RP`@\x84\x01QaS6`\xA0\x84\x01\x82QaR\xC5V[` \x01QaSG`\xE0\x84\x01\x82aR\xC5V[P\x82Qa\x01 \x83\x01R` \x83\x01Qa\x01@\x83\x01Ra\x1CXV[_` \x82\x84\x03\x12\x15aSpW__\xFD[PQ\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x0C\xD2Wa\x0C\xD2aNjV[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R_aQ\x82``\x83\x01\x84aFnV[_\x82`\x1F\x83\x01\x12aS\xBFW__\xFD[\x81QaS\xCDa@\x9C\x82a@IV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aS\xEEW__\xFD[` \x85\x01[\x83\x81\x10\x15a@\xE8W\x80QaT\x06\x81aC\xD7V[\x83R` \x92\x83\x01\x92\x01aS\xF3V[__`@\x83\x85\x03\x12\x15aT%W__\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aT:W__\xFD[aTF\x85\x82\x86\x01aS\xB0V[\x92PP` \x83\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aTaW__\xFD[aGe\x85\x82\x86\x01aS\xB0V[_` \x82\x84\x03\x12\x15aT}W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aT\x92W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aT\xA2W__\xFD[\x80QaT\xB0a@\x9C\x82a@IV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15aT\xD1W__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a\x15\xF1W\x83QaT\xEB\x81aA:V[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aT\xD8V[\x81\x81\x03\x81\x81\x11\x15a\x0C\xD2Wa\x0C\xD2aNjV[_a\xFF\xFF\x82\x16a\xFF\xFF\x81\x03aU&WaU&aNjV[`\x01\x01\x92\x91PPV[`\x1F\x82\x11\x15a\n\xA4W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15aUTWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\r\xF0W_\x81U`\x01\x01aU`V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aU\x8CWaU\x8Ca?\xBBV[aU\xA0\x81aU\x9A\x84TaR\x8DV[\x84aU/V[` `\x1F\x82\x11`\x01\x81\x14aU\xD2W_\x83\x15aU\xBBWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\r\xF0V[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15aV\x01W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01aU\xE1V[P\x84\x82\x10\x15aV\x1EW\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[` \x80\x82R\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83\x83\x01R\x90\x83\x01Q\x16`@\x80\x83\x01\x91\x90\x91R\x82\x01Q``\x80\x83\x01R_\x90a\x1CX`\x80\x84\x01\x82aI\xF5V[_` \x82\x84\x03\x12\x15aVyW__\xFD[\x81Qa\n\xBA\x81aC\xD7V[`\x01`\x01``\x1B\x03\x81\x81\x16\x83\x82\x16\x02\x90\x81\x16\x90\x81\x81\x14a\x1F^Wa\x1F^aNjV[_`\x01`\x01``\x1B\x03\x83\x16\x80aV\xBEWaV\xBEaP\x08V[\x80`\x01`\x01``\x1B\x03\x84\x16\x04\x91PP\x92\x91PPV[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 \xA42\xD9\xF0\x89b\xAD\xE1\xF8<W\xC8_\xFE\x1B\x12\xB9\xEB\x84\x98W[9\xE5\tI\xD7s\xE7\xA5R\x8AdsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b50600436106102e3575f3560e01c80635df459461161018d578063a65497c6116100e0578063de1164bb1161008f578063de1164bb146107ea578063e65797ad146107fd578063ea32afae14610878578063f2fde38b1461089f578063f698da25146108b2578063fabc1cbc146108ba578063fd39105a146108cd575f5ffd5b8063a65497c61461073a578063a96f783e1461074d578063b526578714610756578063c391425e14610769578063c63fd50214610789578063ca0de8821461079c578063ca8aa7c7146107c3575f5ffd5b806384ca52131161013c57806384ca521314610670578063871ef04914610683578063886f1195146106965780638da5cb5b146106bd5780639aa1653d146106ce5780639e9923c2146106ec5780639feab85914610713575f5ffd5b80635df45946146105c657806368304835146105ed5780636e3b17db14610614578063715018a614610627578063734479921461062f5780638281ab751461064257806384b0196e14610655575f5ffd5b8063296bb06411610245578063530b97a4116101f4578063530b97a41461053857806354fd4d501461054b5780635865c60c14610560578063595c6a67146105805780635ac86ab7146105885780635b0b829f146105ab5780635c975abb146105be575f5ffd5b8063296bb064146104a657806329d1e0c3146104b95780632cdd1e86146104cc578063303ca956146104df5780633c2a7f4c146104f25780633eef3a51146105125780635140a54814610525575f5ffd5b8063125e0584116102a1578063125e05841461039f57806313542a4e146103be578063136439dd146103e65780631478851f146103f95780631eb812da1461042b578063249a0c421461047457806328f61b3114610493575f5ffd5b8062cf2ab5146102e757806303fd3492146102fc57806304ec63511461032e578063054310e6146103595780630cf4b767146103795780630d3f21341461038c575b5f5ffd5b6102fa6102f53660046140f2565b610908565b005b61031b61030a366004614123565b5f9081526002602052604090205490565b6040519081526020015b60405180910390f35b61034161033c36600461414b565b610aa9565b6040516001600160c01b039091168152602001610325565b60075461036c906001600160a01b031681565b6040516103259190614180565b6102fa610387366004614205565b610ac1565b6102fa61039a366004614123565b610b23565b61031b6103ad366004614236565b60096020525f908152604090205481565b61031b6103cc366004614236565b6001600160a01b03165f9081526003602052604090205490565b6102fa6103f4366004614123565b610b70565b61041b610407366004614123565b60046020525f908152604090205460ff1681565b6040519015158152602001610325565b61043e610439366004614251565b610c4a565b60408051825163ffffffff908116825260208085015190911690820152918101516001600160c01b031690820152606001610325565b61031b610482366004614286565b60056020525f908152604090205481565b60085461036c906001600160a01b031681565b61036c6104b4366004614123565b610cd8565b6102fa6104c7366004614236565b610d61565b6102fa6104da366004614236565b610d72565b6102fa6104ed366004614303565b610d83565b610505610500366004614236565b610df7565b6040516103259190614360565b6102fa610520366004614481565b610e1b565b6102fa61053336600461452d565b610e37565b6102fa61054636600461460a565b61115b565b6105536112f1565b604051610325919061469c565b61057361056e366004614236565b611321565b60405161032591906146d6565b6102fa611393565b61041b610596366004614286565b603354600160ff9092169190911b9081161490565b6102fa6105b93660046146f1565b611447565b60335461031b565b61036c7f000000000000000000000000000000000000000000000000000000000000000081565b61036c7f000000000000000000000000000000000000000000000000000000000000000081565b6102fa610622366004614723565b611463565b6102fa61148f565b61031b61063d366004614236565b6114a0565b6102fa61065036600461476f565b611504565b61065d611519565b60405161032597969594939291906147c1565b61031b61067e3660046148eb565b6115b2565b610341610691366004614123565b6115fb565b61036c7f000000000000000000000000000000000000000000000000000000000000000081565b6096546001600160a01b031661036c565b5f546106da9060ff1681565b60405160ff9091168152602001610325565b61036c7f000000000000000000000000000000000000000000000000000000000000000081565b61031b7f2bd82124057f0913bc3b772ce7b83e8057c1ad1f3510fc83778be20f10ec5de681565b6102fa610748366004614236565b611605565b61031b600a5481565b61041b610764366004614236565b611616565b61077c610777366004614950565b61162a565b6040516103259190614a35565b6102fa610797366004614a47565b611638565b61031b7f4d404e3276e7ac2163d8ee476afa6a41d1f68fb71f2d8b6546b24e55ce01b72a81565b61036c7f000000000000000000000000000000000000000000000000000000000000000081565b600b5461036c906001600160a01b031681565b61086b61080b366004614286565b60408051606080820183525f808352602080840182905292840181905260ff9490941684526001825292829020825193840183525463ffffffff8116845261ffff600160201b8204811692850192909252600160301b9004169082015290565b6040516103259190614ad4565b61036c7f000000000000000000000000000000000000000000000000000000000000000081565b6102fa6108ad366004614236565b6117da565b61031b611850565b6102fa6108c8366004614123565b611859565b6108fb6108db366004614236565b6001600160a01b03165f9081526003602052604090206001015460ff1690565b6040516103259190614b07565b6033546002906004908116036109315760405163840a48d560e01b815260040160405180910390fd5b5f5b8251811015610aa4576040805160018082528183019092525f916020808301908036833701905050905083828151811061096f5761096f614b15565b6020026020010151815f8151811061098957610989614b15565b6001600160a01b0392909216602092830291909101909101526040805160018082528183019092525f9181602001602082028036833701905050905060035f8685815181106109da576109da614b15565b60200260200101516001600160a01b03166001600160a01b031681526020019081526020015f205f0154815f81518110610a1657610a16614b15565b6020026020010181815250505f610a45825f81518110610a3857610a38614b15565b6020026020010151611970565b90505f610a5a826001600160c01b031661197c565b90505f5b8151811015610a9357610a8b8585848481518110610a7e57610a7e614b15565b016020015160f81c611a45565b600101610a5e565b505060019093019250610933915050565b505050565b5f610ab76002858585611b7c565b90505b9392505050565b6001335f9081526003602052604090206001015460ff166002811115610ae957610ae96146ae565b14610b075760405163aba4733960e01b815260040160405180910390fd5b335f90815260036020526040902054610b209082611c60565b50565b610b2b611d0b565b600a80549082905560408051828152602081018490527fa77a91bea7b6d95a8eb5a54878a1d9e3c875e26c86a9b70e3420c5c5db193b62910160405180910390a15050565b60405163237dfb4760e11b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906346fbf68e90610bbc903390600401614180565b602060405180830381865afa158015610bd7573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610bfb9190614b38565b610c1857604051631d77d47760e21b815260040160405180910390fd5b6033548181168114610c3d5760405163c61dca5d60e01b815260040160405180910390fd5b610c4682611d65565b5050565b604080516060810182525f80825260208201819052918101919091525f838152600260205260409020805483908110610c8557610c85614b15565b5f91825260209182902060408051606081018252919092015463ffffffff8082168352600160201b820416938201939093526001600160c01b03600160401b909304929092169082015290505b92915050565b6040516308f6629d60e31b8152600481018290525f907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906347b314e890602401602060405180830381865afa158015610d3d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610cd29190614b51565b610d69611d0b565b610b2081611da2565b610d7a611d0b565b610b2081611e0b565b610d8b611e74565b603354600190600290811603610db45760405163840a48d560e01b815260040160405180910390fd5b610dbd83611616565b610dda576040516366e565df60e01b815260040160405180910390fd5b5f610de483611ebd565b9050610df08582611f65565b5050505050565b604080518082019091525f8082526020820152610cd2610e16836114a0565b612213565b610e23611d0b565b610e3184848460018561229d565b50505050565b603354600290600490811603610e605760405163840a48d560e01b815260040160405180910390fd5b610ea283838080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201829052505460ff1692506127ba915050565b5083518214610ec45760405163aaad13f760e01b815260040160405180910390fd5b5f5b82811015610df0575f848483818110610ee157610ee1614b15565b885192013560f81c92505f9188915084908110610f0057610f00614b15565b60209081029190910101516040516379a0849160e11b815260ff841660048201529091506001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063f341092290602401602060405180830381865afa158015610f73573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f979190614b6c565b63ffffffff16815114610fbd57604051638e5aeee760e01b815260040160405180910390fd5b5f81516001600160401b03811115610fd757610fd7613fbb565b604051908082528060200260200182016040528015611000578160200160208202803683370190505b5090505f805b83518110156110f6575f84828151811061102257611022614b15565b6020026020010151905060035f826001600160a01b03166001600160a01b031681526020019081526020015f205f015484838151811061106457611064614b15565b6020026020010181815250505f611086858481518110610a3857610a38614b15565b905060016001600160c01b03821660ff89161c8116146110b95760405163d053aa2160e01b815260040160405180910390fd5b836001600160a01b0316826001600160a01b0316116110eb5760405163ba50f91160e01b815260040160405180910390fd5b509150600101611006565b50611102838386611a45565b60ff84165f81815260056020908152604091829020439081905591519182527f46077d55330763f16269fd75e5761663f4192d2791747c0189b16ad31db07db4910160405180910390a250505050806001019050610ec6565b603254610100900460ff161580801561117b5750603254600160ff909116105b806111955750303b158015611195575060325460ff166001145b6111fd5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b6032805460ff191660011790558015611220576032805461ff0019166101001790555b6112766040518060400160405280601681526020017520ab29a932b3b4b9ba393ca1b7b7b93234b730ba37b960511b8152506040518060400160405280600681526020016576302e302e3160d01b8152506127ee565b61127f8661281f565b61128885611da2565b61129183611d65565b61129a84611e0b565b6112a382612870565b80156112e9576032805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b505050505050565b606061131c7f00000000000000000000000000000000000000000000000000000000000000006128dc565b905090565b604080518082019091525f80825260208201526001600160a01b0382165f908152600360209081526040918290208251808401909352805483526001810154909183019060ff166002811115611379576113796146ae565b600281111561138a5761138a6146ae565b90525092915050565b60405163237dfb4760e11b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906346fbf68e906113df903390600401614180565b602060405180830381865afa1580156113fa573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061141e9190614b38565b61143b57604051631d77d47760e21b815260040160405180910390fd5b6114455f19611d65565b565b61144f611d0b565b8161145981612919565b610aa48383612941565b61146b6129d2565b6001600160a01b0382165f908152600960205260409020429055610c4682826129fd565b611497611d0b565b6114455f61281f565b5f610cd27f2bd82124057f0913bc3b772ce7b83e8057c1ad1f3510fc83778be20f10ec5de6836040516020016114e99291909182526001600160a01b0316602082015260400190565b60405160208183030381529060405280519060200120612a93565b61150c611d0b565b610aa48383835f5f61229d565b5f6060805f5f5f606060c8545f5f1b148015611535575060c954155b6115795760405162461bcd60e51b81526020600482015260156024820152741152540dcc4c8e88155b9a5b9a5d1a585b1a5e9959605a1b60448201526064016111f4565b611581612abf565b611589612b4f565b604080515f80825260208201909252600f60f81b9b939a50919850469750309650945092509050565b5f6115f17f4d404e3276e7ac2163d8ee476afa6a41d1f68fb71f2d8b6546b24e55ce01b72a87878787876040516020016114e996959493929190614b87565b9695505050505050565b5f610cd282611970565b61160d611d0b565b610b2081612870565b600b546001600160a01b0391821691161490565b6060610aba60028484612b5e565b611640611e74565b6033545f906001908116036116685760405163840a48d560e01b815260040160405180910390fd5b61167185611616565b61168e576040516366e565df60e01b815260040160405180910390fd5b5f61169885611ebd565b90505f80806116a986880188614d16565b9250925092505f6116ba8b83612c0d565b90505f8460018111156116cf576116cf6146ae565b03611778575f6116e38c8388876001612ca6565b5190505f5b8651811015611771575f87828151811061170457611704614b15565b0160209081015160f81c5f8181526001909252604090912054845191925063ffffffff169084908490811061173b5761173b614b15565b602002602001015163ffffffff1611156117685760405163c6b9e76760e01b815260040160405180910390fd5b506001016116e8565b50506117cd565b600184600181111561178c5761178c6146ae565b036117b4575f8061179f898b018b614d71565b945094505050506117718d84898886866130ed565b60405163354bb8ab60e01b815260040160405180910390fd5b5050505050505050505050565b6117e2611d0b565b6001600160a01b0381166118475760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b60648201526084016111f4565b610b208161281f565b5f61131c6132e2565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156118b5573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906118d99190614b51565b6001600160a01b0316336001600160a01b03161461190a5760405163794821ff60e01b815260040160405180910390fd5b603354801982198116146119315760405163c61dca5d60e01b815260040160405180910390fd5b603382905560405182815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c906020015b60405180910390a25050565b5f610cd26002836132eb565b60605f5f61198984613355565b61ffff166001600160401b038111156119a4576119a4613fbb565b6040519080825280601f01601f1916602001820160405280156119ce576020820181803683370190505b5090505f805b8251821080156119e5575061010081105b15611a3b576001811b935085841615611a2b578060f81b838381518110611a0e57611a0e614b15565b60200101906001600160f81b03191690815f1a9053508160010191505b611a3481614e7e565b90506119d4565b5090949350505050565b6040805160018082528183019092525f916020820181803683370190505090508160f81b815f81518110611a7b57611a7b614b15565b60200101906001600160f81b03191690815f1a905350604051636c3fb4bf60e01b81525f906001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690636c3fb4bf90611ae390889088908890600401614e96565b5f604051808303815f875af1158015611afe573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052611b259190810190614f26565b90505f5b85518110156112e957818181518110611b4457611b44614b15565b602002602001015115611b7457611b74868281518110611b6657611b66614b15565b6020026020010151846129fd565b600101611b29565b5f838152602085905260408120805482919084908110611b9e57611b9e614b15565b5f91825260209182902060408051606081018252929091015463ffffffff808216808552600160201b8304821695850195909552600160401b9091046001600160c01b03169183019190915290925085161015611c0e57604051636cb19aff60e01b815260040160405180910390fd5b602081015163ffffffff161580611c345750806020015163ffffffff168463ffffffff16105b611c515760405163bbba60cb60e01b815260040160405180910390fd5b6040015190505b949350505050565b6040516378219b3f60e11b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063f043367e90611cae9085908590600401614fb3565b5f604051808303815f87803b158015611cc5575f5ffd5b505af1158015611cd7573d5f5f3e3d5ffd5b50505050817fec2963ab21c1e50e1e582aa542af2e4bf7bf38e6e1403c27b42e1c5d6e621eaa82604051611964919061469c565b6096546001600160a01b031633146114455760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016111f4565b603381905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a250565b6007546040517f315457d8a8fe60f04af17c16e2f5a5e1db612b31648e58030360759ef8f3528c91611de1916001600160a01b03909116908490614fcb565b60405180910390a1600780546001600160a01b0319166001600160a01b0392909216919091179055565b6008546040517f8f30ab09f43a6c157d7fce7e0a13c003042c1c95e8a72e7a146a21c0caa24dc991611e4a916001600160a01b03909116908490614fcb565b60405180910390a1600880546001600160a01b0319166001600160a01b0392909216919091179055565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614611445576040516323d871a560e01b815260040160405180910390fd5b60605f82516001600160401b03811115611ed957611ed9613fbb565b6040519080825280601f01601f191660200182016040528015611f03576020820181803683370190505b5090505f5b8351811015611f5e57838181518110611f2357611f23614b15565b602002602001015160f81b828281518110611f4057611f40614b15565b60200101906001600160f81b03191690815f1a905350600101611f08565b5092915050565b6001600160a01b0382165f90815260036020526040812080549091611f8982611970565b905060018084015460ff166002811115611fa557611fa56146ae565b14611fc35760405163aba4733960e01b815260040160405180910390fd5b5f8054611fd490869060ff166127ba565b90506001600160c01b038116611ffd576040516368b6a87560e11b815260040160405180910390fd5b6120146001600160c01b0382811690841681161490565b6120315760405163d053aa2160e01b815260040160405180910390fd5b6001600160c01b038181161983161661204a848261337f565b6001600160c01b0381166120a6576001600160a01b0387165f81815260036020526040808220600101805460ff19166002179055518692917f396fdcb180cb0fea26928113fb0fd1c3549863f9cd563e6a184f1d578116c8e491a35b60405163f4e24fe560e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063f4e24fe5906120f4908a908a90600401614fe5565b5f604051808303815f87803b15801561210b575f5ffd5b505af115801561211d573d5f5f3e3d5ffd5b505060405163bd29b8cd60e01b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016925063bd29b8cd915061216f9087908a90600401614fb3565b5f604051808303815f87803b158015612186575f5ffd5b505af1158015612198573d5f5f3e3d5ffd5b505060405163bd29b8cd60e01b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016925063bd29b8cd91506121ea9087908a90600401614fb3565b5f604051808303815f87803b158015612201575f5ffd5b505af11580156117cd573d5f5f3e3d5ffd5b604080518082019091525f80825260208201525f80806122405f5160206156ea5f395f51905f528661501c565b90505b61224c8161338b565b90935091505f5160206156ea5f395f51905f528283098303612284576040805180820190915290815260208101919091529392505050565b5f5160206156ea5f395f51905f52600182089050612243565b5f5460ff1660c081106122c357604051633cb89c9760e01b815260040160405180910390fd5b5f80546001919081906122da90849060ff1661502f565b92506101000a81548160ff021916908360ff1602179055506122fc8187612941565b6040805160018082528183019092525f91816020015b604080518082019091525f8152606060208201528152602001906001900390816123125790505090505f85516001600160401b0381111561235557612355613fbb565b60405190808252806020026020018201604052801561237e578160200160208202803683370190505b5090505f5b86518110156123db5786818151811061239e5761239e614b15565b60200260200101515f01518282815181106123bb576123bb614b15565b6001600160a01b0390921660209283029190910190910152600101612383565b5060405180604001604052808460ff1663ffffffff16815260200182815250825f8151811061240c5761240c614b15565b6020908102919091010152600b54604051630130fc2760e51b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000081169263261f84e09261246c9291909116908690600401615048565b5f604051808303815f87803b158015612483575f5ffd5b505af1158015612495573d5f5f3e3d5ffd5b505f92506124a1915050565b8560018111156124b3576124b36146ae565b0361253a57604051633aea0b9d60e11b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906375d4173a906125089086908b908b90600401615158565b5f604051808303815f87803b15801561251f575f5ffd5b505af1158015612531573d5f5f3e3d5ffd5b5050505061267e565b600185600181111561254e5761254e6146ae565b0361267e578363ffffffff167f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316632981eb776040518163ffffffff1660e01b8152600401602060405180830381865afa1580156125b6573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906125da9190614b6c565b63ffffffff16116125fe57604051630bd441b960e21b815260040160405180910390fd5b604051630662d3e160e51b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063cc5a7c20906126509086908b9089908c9060040161518b565b5f604051808303815f87803b158015612667575f5ffd5b505af1158015612679573d5f5f3e3d5ffd5b505050505b60405163136ca0f960e11b815260ff841660048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906326d941f2906024015f604051808303815f87803b1580156126df575f5ffd5b505af11580156126f1573d5f5f3e3d5ffd5b505060405163136ca0f960e11b815260ff861660048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031692506326d941f291506024015f604051808303815f87803b158015612756575f5ffd5b505af1158015612768573d5f5f3e3d5ffd5b505050508260ff167fa34835bc2b673ec37fcf1591a91295b163fc2e181e4ea4e733beb27de1ceac4c89898989896040516127a79594939291906151c1565b60405180910390a25b5050505050505050565b5f5f6127c584613407565b9050808360ff166001901b11610aba5760405163ca95733360e01b815260040160405180910390fd5b603254610100900460ff166128155760405162461bcd60e51b81526004016111f490615242565b610c4682826134c2565b609680546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a35050565b600b546040516001600160a01b03909116907f9770f3cadfdcbb6f93af935e86047111590c3768271d237e4a2bc0b874bed693906128b19083908590614fcb565b60405180910390a150600b80546001600160a01b0319166001600160a01b0392909216919091179055565b60605f6128e883613510565b6040805160208082528183019092529192505f91906020820181803683375050509182525060208101929092525090565b5f5460ff90811690821610610b2057604051637310cff560e11b815260040160405180910390fd5b60ff82165f8181526001602090815260409182902084518154928601518487015161ffff908116600160301b0267ffff0000000000001991909216600160201b0265ffffffffffff1990951663ffffffff909316929092179390931716919091179055517f3ee6fe8d54610244c3e9d3c066ae4aee997884aa28f10616ae821925401318ac90611964908490614ad4565b6008546001600160a01b03163314611445576040516376d8ab1760e11b815260040160405180910390fd5b6001600160a01b0382165f90815260036020526040902060018082015460ff166002811115612a2e57612a2e6146ae565b14612a4c576040516325ec6c1f60e01b815260040160405180910390fd5b80545f8054612a5f90859060ff166127ba565b90505f612a6b83611970565b9050612a846001600160c01b0383811690831681161490565b156112e9576112e98686613537565b5f610cd2612a9f6132e2565b8360405161190160f01b8152600281019290925260228201526042902090565b606060ca8054612ace9061528d565b80601f0160208091040260200160405190810160405280929190818152602001828054612afa9061528d565b8015612b455780601f10612b1c57610100808354040283529160200191612b45565b820191905f5260205f20905b815481529060010190602001808311612b2857829003601f168201915b5050505050905090565b606060cb8054612ace9061528d565b60605f82516001600160401b03811115612b7a57612b7a613fbb565b604051908082528060200260200182016040528015612ba3578160200160208202803683370190505b5090505f5b8351811015612c0457612bd58686868481518110612bc857612bc8614b15565b60200260200101516135d5565b828281518110612be757612be7614b15565b63ffffffff90921660209283029190910190910152600101612ba8565b50949350505050565b5f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166303c5a6b68484612c4887610df7565b6040518463ffffffff1660e01b8152600401612c66939291906152e7565b6020604051808303815f875af1158015612c82573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610aba9190615360565b612cca60405180606001604052806060815260200160608152602001606081525090565b5f8054612cdb90869060ff166127ba565b90505f612ce787611970565b90506001600160c01b038216612d10576040516313ca465760e01b815260040160405180910390fd5b8082166001600160c01b031615612d3a57604051630c6816cd60e01b815260040160405180910390fd5b600a546001600160a01b0389165f908152600960205260409020546001600160c01b0383811690851617914291612d719190615377565b10612d8f57604051631968677d60e11b815260040160405180910390fd5b612d99888261337f565b612da38887611c60565b60016001600160a01b038a165f9081526003602052604090206001015460ff166002811115612dd457612dd46146ae565b14612e6b57604080518082018252898152600160208083018281526001600160a01b038e165f908152600390925293902082518155925183820180549394939192909160ff191690836002811115612e2e57612e2e6146ae565b0217905550506040518991506001600160a01b038b16907fe8e68cef1c3a761ed7be7e8463a375f27f7bc335e51824223cacce636ec5c3fe905f90a35b604051631fd93ca960e11b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690633fb2795290612eb9908c908b90600401614fe5565b5f604051808303815f87803b158015612ed0575f5ffd5b505af1158015612ee2573d5f5f3e3d5ffd5b5050604051632550477760e01b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016925063255047779150612f36908c908c908c9060040161538a565b5f604051808303815f875af1158015612f51573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052612f789190810190615414565b60408087019190915260208601919091525162bff04d60e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169062bff04d90612fd3908b908b90600401614fb3565b5f604051808303815f875af1158015612fee573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052613015919081019061546d565b845284156130e1575f5b87518110156130df575f60015f8a848151811061303e5761303e614b15565b0160209081015160f81c82528181019290925260409081015f208151606081018352905463ffffffff811680835261ffff600160201b8304811695840195909552600160301b909104909316918101919091528751805191935090849081106130a9576130a9614b15565b602002602001015163ffffffff1611156130d65760405163c6b9e76760e01b815260040160405180910390fd5b5060010161301f565b505b50505095945050505050565b835182511461310f5760405163aaad13f760e01b815260040160405180910390fd5b61311b868684846136ed565b5f613129878787875f612ca6565b90505f5b85518110156127b0575f60015f88848151811061314c5761314c614b15565b0160209081015160f81c82528181019290925260409081015f208151606081018352905463ffffffff811680835261ffff600160201b8304811695840195909552600160301b909104909316918101919091528451805191935090849081106131b7576131b7614b15565b602002602001015163ffffffff1611156132d95761324b8783815181106131e0576131e0614b15565b602001015160f81c60f81b60f81c8460400151848151811061320457613204614b15565b60200260200101518b8660200151868151811061322357613223614b15565b602002602001015189878151811061323d5761323d614b15565b602002602001015186613798565b6040805160018082528183019092525f9160208201818036833701905050905087838151811061327d5761327d614b15565b602001015160f81c60f81b815f8151811061329a5761329a614b15565b60200101906001600160f81b03191690815f1a9053506132d78684815181106132c5576132c5614b15565b602002602001015160200151826129fd565b505b5060010161312d565b5f61131c613919565b5f81815260208390526040812054808203613309575f915050610cd2565b5f8381526020859052604090206133216001836154fc565b8154811061333157613331614b15565b5f91825260209091200154600160401b90046001600160c01b03169150610cd29050565b5f805b8215610cd2576133696001846154fc565b90921691806133778161550f565b915050613358565b610c466002838361398c565b5f80805f5160206156ea5f395f51905f5260035f5160206156ea5f395f51905f52865f5160206156ea5f395f51905f52888909090890505f6133fb827f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f525f5160206156ea5f395f51905f52613b45565b91959194509092505050565b5f6101008251111561342c57604051637da54e4760e11b815260040160405180910390fd5b81515f0361343b57505f919050565b5f5f835f8151811061344f5761344f614b15565b0160200151600160f89190911c81901b92505b84518110156134b95784818151811061347d5761347d614b15565b0160200151600160f89190911c1b91508282116134ad57604051631019106960e31b815260040160405180910390fd5b91811791600101613462565b50909392505050565b603254610100900460ff166134e95760405162461bcd60e51b81526004016111f490615242565b60ca6134f58382615573565b5060cb6135028282615573565b50505f60c881905560c95550565b5f60ff8216601f811115610cd257604051632cd44ac360e21b815260040160405180910390fd5b604080516060810182526001600160a01b038481168252600b54811660208301527f00000000000000000000000000000000000000000000000000000000000000001691636e3492b59190810161358d85613bbe565b8152506040518263ffffffff1660e01b81526004016135ac919061562d565b5f604051808303815f87803b1580156135c3575f5ffd5b505af11580156112e9573d5f5f3e3d5ffd5b5f81815260208490526040812054815b818110156136585760016135f982846154fc565b61360391906154fc565b92508463ffffffff16865f8681526020019081526020015f208463ffffffff168154811061363357613633614b15565b5f9182526020909120015463ffffffff1611613650575050610aba565b6001016135e5565b5060405162461bcd60e51b815260206004820152605c60248201527f5265676973747279436f6f7264696e61746f722e67657451756f72756d42697460448201527f6d6170496e6465784174426c6f636b4e756d6265723a206e6f206269746d617060648201527f2075706461746520666f756e6420666f72206f70657261746f72496400000000608482015260a4016111f4565b6020808201515f9081526004909152604090205460ff161561372257604051636fbefec360e11b815260040160405180910390fd5b428160400151101561374757604051630819bdcd60e01b815260040160405180910390fd5b602080820180515f9081526004909252604091829020805460ff19166001179055600754905191830151610e31926001600160a01b039092169161379191889188918891906115b2565b8351613c63565b6020808301516001600160a01b038082165f81815260039094526040909320549192908716036137db576040516356168b4160e11b815260040160405180910390fd5b8760ff16845f015160ff161461380457604051638e5aeee760e01b815260040160405180910390fd5b604051635401ed2760e01b81526004810182905260ff891660248201525f907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635401ed2790604401602060405180830381865afa158015613872573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906138969190615669565b90506138a28185613c8b565b6001600160601b0316866001600160601b0316116138d357604051634c44995d60e01b815260040160405180910390fd5b6138dd8885613cae565b6001600160601b0316816001600160601b03161061390e5760405163b187e86960e01b815260040160405180910390fd5b505050505050505050565b5f7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f613943613cc7565b61394b613d1f565b60408051602081019490945283019190915260608201524660808201523060a082015260c00160405160208183030381529060405280519060200120905090565b5f8281526020849052604081205490819003613a30575f83815260208581526040808320815160608101835263ffffffff43811682528185018681526001600160c01b03808a16958401958652845460018101865594885295909620915191909201805495519351909416600160401b026001600160401b03938316600160201b0267ffffffffffffffff1990961691909216179390931716919091179055610e31565b5f838152602085905260408120613a486001846154fc565b81548110613a5857613a58614b15565b5f918252602090912001805490915063ffffffff438116911603613a995780546001600160401b0316600160401b6001600160c01b03851602178155610df0565b805463ffffffff438116600160201b81810267ffffffff00000000199094169390931784555f8781526020898152604080832081516060810183529485528483018481526001600160c01b03808c1693870193845282546001810184559286529390942094519401805493519151909216600160401b026001600160401b0391861690960267ffffffffffffffff19909316939094169290921717919091169190911790555050505050565b5f5f613b4f613f7f565b613b57613f9d565b602080825281810181905260408201819052606082018890526080820187905260a082018690528260c08360056107d05a03fa92508280613b9457fe5b5082613bb35760405163d51edae360e01b815260040160405180910390fd5b505195945050505050565b60605f82516001600160401b03811115613bda57613bda613fbb565b604051908082528060200260200182016040528015613c03578160200160208202803683370190505b5090505f5b8351811015611f5e57838181518110613c2357613c23614b15565b602001015160f81c60f81b60f81c60ff16828281518110613c4657613c46614b15565b63ffffffff90921660209283029190910190910152600101613c08565b613c6e838383613d4f565b610aa457604051638baa579f60e01b815260040160405180910390fd5b60208101515f9061271090613ca49061ffff1685615684565b610aba91906156a6565b60408101515f9061271090613ca49061ffff1685615684565b5f5f613cd1612abf565b805190915015613ce8578051602090910120919050565b60c8548015613cf75792915050565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a4709250505090565b5f5f613d29612b4f565b805190915015613d40578051602090910120919050565b60c9548015613cf75792915050565b5f5f5f613d5c8585613da3565b90925090505f816004811115613d7457613d746146ae565b148015613d925750856001600160a01b0316826001600160a01b0316145b806115f157506115f1868686613de5565b5f5f8251604103613dd7576020830151604084015160608501515f1a613dcb87828585613ecc565b94509450505050613dde565b505f905060025b9250929050565b5f5f5f856001600160a01b0316631626ba7e60e01b8686604051602401613e0d929190614fb3565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b0319909416939093179092529051613e4b91906156d3565b5f60405180830381855afa9150503d805f8114613e83576040519150601f19603f3d011682016040523d82523d5f602084013e613e88565b606091505b5091509150818015613e9c57506020815110155b80156115f157508051630b135d3f60e11b90613ec19083016020908101908401615360565b149695505050505050565b5f806fa2a8918ca85bafe22016d0b997e4df60600160ff1b03831115613ef757505f90506003613f76565b604080515f8082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa158015613f48573d5f5f3e3d5ffd5b5050604051601f1901519150506001600160a01b038116613f70575f60019250925050613f76565b91505f90505b94509492505050565b60405180602001604052806001906020820280368337509192915050565b6040518060c001604052806006906020820280368337509192915050565b634e487b7160e01b5f52604160045260245ffd5b604051606081016001600160401b0381118282101715613ff157613ff1613fbb565b60405290565b604080519081016001600160401b0381118282101715613ff157613ff1613fbb565b604051601f8201601f191681016001600160401b038111828210171561404157614041613fbb565b604052919050565b5f6001600160401b0382111561406157614061613fbb565b5060051b60200190565b6001600160a01b0381168114610b20575f5ffd5b5f82601f83011261408e575f5ffd5b81356140a161409c82614049565b614019565b8082825260208201915060208360051b8601019250858311156140c2575f5ffd5b602085015b838110156140e85780356140da8161406b565b8352602092830192016140c7565b5095945050505050565b5f60208284031215614102575f5ffd5b81356001600160401b03811115614117575f5ffd5b611c588482850161407f565b5f60208284031215614133575f5ffd5b5035919050565b63ffffffff81168114610b20575f5ffd5b5f5f5f6060848603121561415d575f5ffd5b83359250602084013561416f8161413a565b929592945050506040919091013590565b6001600160a01b0391909116815260200190565b5f82601f8301126141a3575f5ffd5b8135602083015f5f6001600160401b038411156141c2576141c2613fbb565b50601f8301601f19166020016141d781614019565b9150508281528583830111156141eb575f5ffd5b828260208301375f92810160200192909252509392505050565b5f60208284031215614215575f5ffd5b81356001600160401b0381111561422a575f5ffd5b611c5884828501614194565b5f60208284031215614246575f5ffd5b8135610aba8161406b565b5f5f60408385031215614262575f5ffd5b50508035926020909101359150565b803560ff81168114614281575f5ffd5b919050565b5f60208284031215614296575f5ffd5b610aba82614271565b5f82601f8301126142ae575f5ffd5b81356142bc61409c82614049565b8082825260208201915060208360051b8601019250858311156142dd575f5ffd5b602085015b838110156140e85780356142f58161413a565b8352602092830192016142e2565b5f5f5f60608486031215614315575f5ffd5b83356143208161406b565b925060208401356143308161406b565b915060408401356001600160401b0381111561434a575f5ffd5b6143568682870161429f565b9150509250925092565b815181526020808301519082015260408101610cd2565b803561ffff81168114614281575f5ffd5b5f60608284031215614398575f5ffd5b6143a0613fcf565b905081356143ad8161413a565b81526143bb60208301614377565b60208201526143cc60408301614377565b604082015292915050565b6001600160601b0381168114610b20575f5ffd5b5f82601f8301126143fa575f5ffd5b813561440861409c82614049565b8082825260208201915060208360061b860101925085831115614429575f5ffd5b602085015b838110156140e85760408188031215614445575f5ffd5b61444d613ff7565b81356144588161406b565b81526020820135614468816143d7565b602082810191909152908452929092019160400161442e565b5f5f5f5f60c08587031215614494575f5ffd5b61449e8686614388565b935060608501356144ae816143d7565b925060808501356001600160401b038111156144c8575f5ffd5b6144d4878288016143eb565b92505060a08501356144e58161413a565b939692955090935050565b5f5f83601f840112614500575f5ffd5b5081356001600160401b03811115614516575f5ffd5b602083019150836020828501011115613dde575f5ffd5b5f5f5f6040848603121561453f575f5ffd5b83356001600160401b03811115614554575f5ffd5b8401601f81018613614564575f5ffd5b803561457261409c82614049565b8082825260208201915060208360051b850101925088831115614593575f5ffd5b602084015b838110156145d35780356001600160401b038111156145b5575f5ffd5b6145c48b60208389010161407f565b84525060209283019201614598565b50955050505060208401356001600160401b038111156145f1575f5ffd5b6145fd868287016144f0565b9497909650939450505050565b5f5f5f5f5f60a0868803121561461e575f5ffd5b85356146298161406b565b945060208601356146398161406b565b935060408601356146498161406b565b92506060860135915060808601356146608161406b565b809150509295509295909350565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b602081525f610aba602083018461466e565b634e487b7160e01b5f52602160045260245ffd5b600381106146d2576146d26146ae565b9052565b815181526020808301516040830191611f5e908401826146c2565b5f5f60808385031215614702575f5ffd5b61470b83614271565b915061471a8460208501614388565b90509250929050565b5f5f60408385031215614734575f5ffd5b823561473f8161406b565b915060208301356001600160401b03811115614759575f5ffd5b61476585828601614194565b9150509250929050565b5f5f5f60a08486031215614781575f5ffd5b61478b8585614388565b9250606084013561479b816143d7565b915060808401356001600160401b038111156147b5575f5ffd5b614356868287016143eb565b60ff60f81b8816815260e060208201525f6147df60e083018961466e565b82810360408401526147f1818961466e565b606084018890526001600160a01b038716608085015260a0840186905283810360c0850152845180825260208087019350909101905f5b81811015614846578351835260209384019390920191600101614828565b50909b9a5050505050505050505050565b5f82601f830112614866575f5ffd5b813561487461409c82614049565b8082825260208201915060208360061b860101925085831115614895575f5ffd5b602085015b838110156140e857604081880312156148b1575f5ffd5b6148b9613ff7565b6148c282614271565b815260208201356148d28161406b565b602082810191909152908452929092019160400161489a565b5f5f5f5f5f60a086880312156148ff575f5ffd5b853561490a8161406b565b94506020860135935060408601356001600160401b0381111561492b575f5ffd5b61493788828901614857565b9598949750949560608101359550608001359392505050565b5f5f60408385031215614961575f5ffd5b823561496c8161413a565b915060208301356001600160401b03811115614986575f5ffd5b8301601f81018513614996575f5ffd5b80356149a461409c82614049565b8082825260208201915060208360051b8501019250878311156149c5575f5ffd5b6020840193505b828410156149e75783358252602093840193909101906149cc565b809450505050509250929050565b5f8151808452602084019350602083015f5b82811015614a2b57815163ffffffff16865260209586019590910190600101614a07565b5093949350505050565b602081525f610aba60208301846149f5565b5f5f5f5f5f60808688031215614a5b575f5ffd5b8535614a668161406b565b94506020860135614a768161406b565b935060408601356001600160401b03811115614a90575f5ffd5b614a9c8882890161429f565b93505060608601356001600160401b03811115614ab7575f5ffd5b614ac3888289016144f0565b969995985093965092949392505050565b60608101610cd2828463ffffffff815116825261ffff602082015116602083015261ffff60408201511660408301525050565b60208101610cd282846146c2565b634e487b7160e01b5f52603260045260245ffd5b80518015158114614281575f5ffd5b5f60208284031215614b48575f5ffd5b610aba82614b29565b5f60208284031215614b61575f5ffd5b8151610aba8161406b565b5f60208284031215614b7c575f5ffd5b8151610aba8161413a565b5f60c0820188835260018060a01b038816602084015286604084015260c0606084015280865180835260e0850191506020880192505f5b81811015614bf4578351805160ff1684526020908101516001600160a01b03168185015290930192604090920191600101614bbe565b50506080840195909552505060a00152949350505050565b803560028110614281575f5ffd5b5f60408284031215614c2a575f5ffd5b614c32613ff7565b823581526020928301359281019290925250919050565b5f82601f830112614c58575f5ffd5b614c60613ff7565b806040840185811115614c71575f5ffd5b845b81811015614c8b578035845260209384019301614c73565b509095945050505050565b5f818303610100811215614ca8575f5ffd5b614cb0613fcf565b9150614cbc8484614c1a565b8252614ccb8460408501614c1a565b60208301526080607f1982011215614ce1575f5ffd5b50614cea613ff7565b614cf78460808501614c49565b8152614d068460c08501614c49565b6020820152604082015292915050565b5f5f5f6101408486031215614d29575f5ffd5b614d3284614c0c565b925060208401356001600160401b03811115614d4c575f5ffd5b614d5886828701614194565b925050614d688560408601614c96565b90509250925092565b5f5f5f5f5f6101808688031215614d86575f5ffd5b614d8f86614c0c565b945060208601356001600160401b03811115614da9575f5ffd5b614db588828901614194565b945050614dc58760408801614c96565b92506101408601356001600160401b03811115614de0575f5ffd5b614dec88828901614857565b9250506101608601356001600160401b03811115614e08575f5ffd5b860160608189031215614e19575f5ffd5b614e21613fcf565b81356001600160401b03811115614e36575f5ffd5b614e428a828501614194565b8252506020828101359082015260409182013591810191909152949793965091945092919050565b634e487b7160e01b5f52601160045260245ffd5b5f60018201614e8f57614e8f614e6a565b5060010190565b606080825284519082018190525f9060208601906080840190835b81811015614ed85783516001600160a01b0316835260209384019390920191600101614eb1565b5050838103602080860191909152865180835291810192508601905f5b81811015614f13578251845260209384019390920191600101614ef5565b50505060ff841660408401529050611c58565b5f60208284031215614f36575f5ffd5b81516001600160401b03811115614f4b575f5ffd5b8201601f81018413614f5b575f5ffd5b8051614f6961409c82614049565b8082825260208201915060208360051b850101925086831115614f8a575f5ffd5b6020840193505b828410156115f157614fa284614b29565b825260209384019390910190614f91565b828152604060208201525f610ab7604083018461466e565b6001600160a01b0392831681529116602082015260400190565b6001600160a01b03831681526040602082018190525f90610ab79083018461466e565b634e487b7160e01b5f52601260045260245ffd5b5f8261502a5761502a615008565b500690565b60ff8181168382160190811115610cd257610cd2614e6a565b5f6040820160018060a01b03851683526040602084015280845180835260608501915060608160051b8601019250602086015f5b828110156150fd57868503605f190184528151805163ffffffff168652602090810151604082880181905281519088018190529101905f9060608801905b808310156150e55783516001600160a01b0316825260209384019360019390930192909101906150ba565b5096505050602093840193919091019060010161507c565b5092979650505050505050565b5f8151808452602084019350602083015f5b82811015614a2b57815180516001600160a01b031687526020908101516001600160601b0316818801526040909601959091019060010161511c565b60ff841681526001600160601b0383166020820152606060408201525f615182606083018461510a565b95945050505050565b60ff851681526001600160601b038416602082015263ffffffff83166040820152608060608201525f6115f1608083018461510a565b6151f0818763ffffffff815116825261ffff602082015116602083015261ffff60408201511660408301525050565b6001600160601b038516606082015260e060808201525f61521460e083018661510a565b905060028410615226576152266146ae565b8360a083015263ffffffff831660c08301529695505050505050565b6020808252602b908201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960408201526a6e697469616c697a696e6760a81b606082015260800190565b600181811c908216806152a157607f821691505b6020821081036152bf57634e487b7160e01b5f52602260045260245ffd5b50919050565b805f5b6002811015610e315781518452602093840193909101906001016152c8565b6001600160a01b0384168152825180516020808401919091520151604082015261016081016020848101518051606085015290810151608084015250604084015161533660a0840182516152c5565b6020015161534760e08401826152c5565b5082516101208301526020830151610140830152611c58565b5f60208284031215615370575f5ffd5b5051919050565b80820180821115610cd257610cd2614e6a565b60018060a01b0384168152826020820152606060408201525f615182606083018461466e565b5f82601f8301126153bf575f5ffd5b81516153cd61409c82614049565b8082825260208201915060208360051b8601019250858311156153ee575f5ffd5b602085015b838110156140e8578051615406816143d7565b8352602092830192016153f3565b5f5f60408385031215615425575f5ffd5b82516001600160401b0381111561543a575f5ffd5b615446858286016153b0565b92505060208301516001600160401b03811115615461575f5ffd5b614765858286016153b0565b5f6020828403121561547d575f5ffd5b81516001600160401b03811115615492575f5ffd5b8201601f810184136154a2575f5ffd5b80516154b061409c82614049565b8082825260208201915060208360051b8501019250868311156154d1575f5ffd5b6020840193505b828410156115f15783516154eb8161413a565b8252602093840193909101906154d8565b81810381811115610cd257610cd2614e6a565b5f61ffff821661ffff810361552657615526614e6a565b60010192915050565b601f821115610aa457805f5260205f20601f840160051c810160208510156155545750805b601f840160051c820191505b81811015610df0575f8155600101615560565b81516001600160401b0381111561558c5761558c613fbb565b6155a08161559a845461528d565b8461552f565b6020601f8211600181146155d2575f83156155bb5750848201515b5f19600385901b1c1916600184901b178455610df0565b5f84815260208120601f198516915b8281101561560157878501518255602094850194600190920191016155e1565b508482101561561e57868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b602080825282516001600160a01b039081168383015290830151166040808301919091528201516060808301525f90611c5860808401826149f5565b5f60208284031215615679575f5ffd5b8151610aba816143d7565b6001600160601b038181168382160290811690818114611f5e57611f5e614e6a565b5f6001600160601b038316806156be576156be615008565b806001600160601b0384160491505092915050565b5f82518060208501845e5f92019182525091905056fe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a2646970667358221220a432d9f08962ade1f83c57c85ffe1b12b9eb8498575b39e50949d773e7a5528a64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x02\xE3W_5`\xE0\x1C\x80c]\xF4YF\x11a\x01\x8DW\x80c\xA6T\x97\xC6\x11a\0\xE0W\x80c\xDE\x11d\xBB\x11a\0\x8FW\x80c\xDE\x11d\xBB\x14a\x07\xEAW\x80c\xE6W\x97\xAD\x14a\x07\xFDW\x80c\xEA2\xAF\xAE\x14a\x08xW\x80c\xF2\xFD\xE3\x8B\x14a\x08\x9FW\x80c\xF6\x98\xDA%\x14a\x08\xB2W\x80c\xFA\xBC\x1C\xBC\x14a\x08\xBAW\x80c\xFD9\x10Z\x14a\x08\xCDW__\xFD[\x80c\xA6T\x97\xC6\x14a\x07:W\x80c\xA9ox>\x14a\x07MW\x80c\xB5&W\x87\x14a\x07VW\x80c\xC3\x91B^\x14a\x07iW\x80c\xC6?\xD5\x02\x14a\x07\x89W\x80c\xCA\r\xE8\x82\x14a\x07\x9CW\x80c\xCA\x8A\xA7\xC7\x14a\x07\xC3W__\xFD[\x80c\x84\xCAR\x13\x11a\x01<W\x80c\x84\xCAR\x13\x14a\x06pW\x80c\x87\x1E\xF0I\x14a\x06\x83W\x80c\x88o\x11\x95\x14a\x06\x96W\x80c\x8D\xA5\xCB[\x14a\x06\xBDW\x80c\x9A\xA1e=\x14a\x06\xCEW\x80c\x9E\x99#\xC2\x14a\x06\xECW\x80c\x9F\xEA\xB8Y\x14a\x07\x13W__\xFD[\x80c]\xF4YF\x14a\x05\xC6W\x80ch0H5\x14a\x05\xEDW\x80cn;\x17\xDB\x14a\x06\x14W\x80cqP\x18\xA6\x14a\x06'W\x80csDy\x92\x14a\x06/W\x80c\x82\x81\xABu\x14a\x06BW\x80c\x84\xB0\x19n\x14a\x06UW__\xFD[\x80c)k\xB0d\x11a\x02EW\x80cS\x0B\x97\xA4\x11a\x01\xF4W\x80cS\x0B\x97\xA4\x14a\x058W\x80cT\xFDMP\x14a\x05KW\x80cXe\xC6\x0C\x14a\x05`W\x80cY\\jg\x14a\x05\x80W\x80cZ\xC8j\xB7\x14a\x05\x88W\x80c[\x0B\x82\x9F\x14a\x05\xABW\x80c\\\x97Z\xBB\x14a\x05\xBEW__\xFD[\x80c)k\xB0d\x14a\x04\xA6W\x80c)\xD1\xE0\xC3\x14a\x04\xB9W\x80c,\xDD\x1E\x86\x14a\x04\xCCW\x80c0<\xA9V\x14a\x04\xDFW\x80c<*\x7FL\x14a\x04\xF2W\x80c>\xEF:Q\x14a\x05\x12W\x80cQ@\xA5H\x14a\x05%W__\xFD[\x80c\x12^\x05\x84\x11a\x02\xA1W\x80c\x12^\x05\x84\x14a\x03\x9FW\x80c\x13T*N\x14a\x03\xBEW\x80c\x13d9\xDD\x14a\x03\xE6W\x80c\x14x\x85\x1F\x14a\x03\xF9W\x80c\x1E\xB8\x12\xDA\x14a\x04+W\x80c$\x9A\x0CB\x14a\x04tW\x80c(\xF6\x1B1\x14a\x04\x93W__\xFD[\x80b\xCF*\xB5\x14a\x02\xE7W\x80c\x03\xFD4\x92\x14a\x02\xFCW\x80c\x04\xECcQ\x14a\x03.W\x80c\x05C\x10\xE6\x14a\x03YW\x80c\x0C\xF4\xB7g\x14a\x03yW\x80c\r?!4\x14a\x03\x8CW[__\xFD[a\x02\xFAa\x02\xF56`\x04a@\xF2V[a\t\x08V[\0[a\x03\x1Ba\x03\n6`\x04aA#V[_\x90\x81R`\x02` R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x03Aa\x03<6`\x04aAKV[a\n\xA9V[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R` \x01a\x03%V[`\x07Ta\x03l\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Qa\x03%\x91\x90aA\x80V[a\x02\xFAa\x03\x876`\x04aB\x05V[a\n\xC1V[a\x02\xFAa\x03\x9A6`\x04aA#V[a\x0B#V[a\x03\x1Ba\x03\xAD6`\x04aB6V[`\t` R_\x90\x81R`@\x90 T\x81V[a\x03\x1Ba\x03\xCC6`\x04aB6V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x03` R`@\x90 T\x90V[a\x02\xFAa\x03\xF46`\x04aA#V[a\x0BpV[a\x04\x1Ba\x04\x076`\x04aA#V[`\x04` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x03%V[a\x04>a\x0496`\x04aBQV[a\x0CJV[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x85\x01Q\x90\x91\x16\x90\x82\x01R\x91\x81\x01Q`\x01`\x01`\xC0\x1B\x03\x16\x90\x82\x01R``\x01a\x03%V[a\x03\x1Ba\x04\x826`\x04aB\x86V[`\x05` R_\x90\x81R`@\x90 T\x81V[`\x08Ta\x03l\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03la\x04\xB46`\x04aA#V[a\x0C\xD8V[a\x02\xFAa\x04\xC76`\x04aB6V[a\raV[a\x02\xFAa\x04\xDA6`\x04aB6V[a\rrV[a\x02\xFAa\x04\xED6`\x04aC\x03V[a\r\x83V[a\x05\x05a\x05\x006`\x04aB6V[a\r\xF7V[`@Qa\x03%\x91\x90aC`V[a\x02\xFAa\x05 6`\x04aD\x81V[a\x0E\x1BV[a\x02\xFAa\x0536`\x04aE-V[a\x0E7V[a\x02\xFAa\x05F6`\x04aF\nV[a\x11[V[a\x05Sa\x12\xF1V[`@Qa\x03%\x91\x90aF\x9CV[a\x05sa\x05n6`\x04aB6V[a\x13!V[`@Qa\x03%\x91\x90aF\xD6V[a\x02\xFAa\x13\x93V[a\x04\x1Ba\x05\x966`\x04aB\x86V[`3T`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[a\x02\xFAa\x05\xB96`\x04aF\xF1V[a\x14GV[`3Ta\x03\x1BV[a\x03l\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03l\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xFAa\x06\"6`\x04aG#V[a\x14cV[a\x02\xFAa\x14\x8FV[a\x03\x1Ba\x06=6`\x04aB6V[a\x14\xA0V[a\x02\xFAa\x06P6`\x04aGoV[a\x15\x04V[a\x06]a\x15\x19V[`@Qa\x03%\x97\x96\x95\x94\x93\x92\x91\x90aG\xC1V[a\x03\x1Ba\x06~6`\x04aH\xEBV[a\x15\xB2V[a\x03Aa\x06\x916`\x04aA#V[a\x15\xFBV[a\x03l\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\x96T`\x01`\x01`\xA0\x1B\x03\x16a\x03lV[_Ta\x06\xDA\x90`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x03%V[a\x03l\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\x1B\x7F+\xD8!$\x05\x7F\t\x13\xBC;w,\xE7\xB8>\x80W\xC1\xAD\x1F5\x10\xFC\x83w\x8B\xE2\x0F\x10\xEC]\xE6\x81V[a\x02\xFAa\x07H6`\x04aB6V[a\x16\x05V[a\x03\x1B`\nT\x81V[a\x04\x1Ba\x07d6`\x04aB6V[a\x16\x16V[a\x07|a\x07w6`\x04aIPV[a\x16*V[`@Qa\x03%\x91\x90aJ5V[a\x02\xFAa\x07\x976`\x04aJGV[a\x168V[a\x03\x1B\x7FM@N2v\xE7\xAC!c\xD8\xEEGj\xFAjA\xD1\xF6\x8F\xB7\x1F-\x8BeF\xB2NU\xCE\x01\xB7*\x81V[a\x03l\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\x0BTa\x03l\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x08ka\x08\x0B6`\x04aB\x86V[`@\x80Q``\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x92\x84\x01\x81\x90R`\xFF\x94\x90\x94\x16\x84R`\x01\x82R\x92\x82\x90 \x82Q\x93\x84\x01\x83RTc\xFF\xFF\xFF\xFF\x81\x16\x84Ra\xFF\xFF`\x01` \x1B\x82\x04\x81\x16\x92\x85\x01\x92\x90\x92R`\x01`0\x1B\x90\x04\x16\x90\x82\x01R\x90V[`@Qa\x03%\x91\x90aJ\xD4V[a\x03l\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xFAa\x08\xAD6`\x04aB6V[a\x17\xDAV[a\x03\x1Ba\x18PV[a\x02\xFAa\x08\xC86`\x04aA#V[a\x18YV[a\x08\xFBa\x08\xDB6`\x04aB6V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x03` R`@\x90 `\x01\x01T`\xFF\x16\x90V[`@Qa\x03%\x91\x90aK\x07V[`3T`\x02\x90`\x04\x90\x81\x16\x03a\t1W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x82Q\x81\x10\x15a\n\xA4W`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x83\x82\x81Q\x81\x10a\toWa\toaK\x15V[` \x02` \x01\x01Q\x81_\x81Q\x81\x10a\t\x89Wa\t\x89aK\x15V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91\x81` \x01` \x82\x02\x806\x837\x01\x90PP\x90P`\x03_\x86\x85\x81Q\x81\x10a\t\xDAWa\t\xDAaK\x15V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _\x01T\x81_\x81Q\x81\x10a\n\x16Wa\n\x16aK\x15V[` \x02` \x01\x01\x81\x81RPP_a\nE\x82_\x81Q\x81\x10a\n8Wa\n8aK\x15V[` \x02` \x01\x01Qa\x19pV[\x90P_a\nZ\x82`\x01`\x01`\xC0\x1B\x03\x16a\x19|V[\x90P_[\x81Q\x81\x10\x15a\n\x93Wa\n\x8B\x85\x85\x84\x84\x81Q\x81\x10a\n~Wa\n~aK\x15V[\x01` \x01Q`\xF8\x1Ca\x1AEV[`\x01\x01a\n^V[PP`\x01\x90\x93\x01\x92Pa\t3\x91PPV[PPPV[_a\n\xB7`\x02\x85\x85\x85a\x1B|V[\x90P[\x93\x92PPPV[`\x013_\x90\x81R`\x03` R`@\x90 `\x01\x01T`\xFF\x16`\x02\x81\x11\x15a\n\xE9Wa\n\xE9aF\xAEV[\x14a\x0B\x07W`@Qc\xAB\xA4s9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3_\x90\x81R`\x03` R`@\x90 Ta\x0B \x90\x82a\x1C`V[PV[a\x0B+a\x1D\x0BV[`\n\x80T\x90\x82\x90U`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x7F\xA7z\x91\xBE\xA7\xB6\xD9Z\x8E\xB5\xA5Hx\xA1\xD9\xE3\xC8u\xE2l\x86\xA9\xB7\x0E4 \xC5\xC5\xDB\x19;b\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[`@Qc#}\xFBG`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cF\xFB\xF6\x8E\x90a\x0B\xBC\x903\x90`\x04\x01aA\x80V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xD7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xFB\x91\x90aK8V[a\x0C\x18W`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`3T\x81\x81\x16\x81\x14a\x0C=W`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0CF\x82a\x1DeV[PPV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R_\x83\x81R`\x02` R`@\x90 \x80T\x83\x90\x81\x10a\x0C\x85Wa\x0C\x85aK\x15V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01`\xC0\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90P[\x92\x91PPV[`@Qc\x08\xF6b\x9D`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R_\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cG\xB3\x14\xE8\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r=W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xD2\x91\x90aKQV[a\ria\x1D\x0BV[a\x0B \x81a\x1D\xA2V[a\rza\x1D\x0BV[a\x0B \x81a\x1E\x0BV[a\r\x8Ba\x1EtV[`3T`\x01\x90`\x02\x90\x81\x16\x03a\r\xB4W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\r\xBD\x83a\x16\x16V[a\r\xDAW`@Qcf\xE5e\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\r\xE4\x83a\x1E\xBDV[\x90Pa\r\xF0\x85\x82a\x1FeV[PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01Ra\x0C\xD2a\x0E\x16\x83a\x14\xA0V[a\"\x13V[a\x0E#a\x1D\x0BV[a\x0E1\x84\x84\x84`\x01\x85a\"\x9DV[PPPPV[`3T`\x02\x90`\x04\x90\x81\x16\x03a\x0E`W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0E\xA2\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x82\x90RPT`\xFF\x16\x92Pa'\xBA\x91PPV[P\x83Q\x82\x14a\x0E\xC4W`@Qc\xAA\xAD\x13\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x82\x81\x10\x15a\r\xF0W_\x84\x84\x83\x81\x81\x10a\x0E\xE1Wa\x0E\xE1aK\x15V[\x88Q\x92\x015`\xF8\x1C\x92P_\x91\x88\x91P\x84\x90\x81\x10a\x0F\0Wa\x0F\0aK\x15V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qcy\xA0\x84\x91`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xF3A\t\"\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0FsW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x97\x91\x90aKlV[c\xFF\xFF\xFF\xFF\x16\x81Q\x14a\x0F\xBDW`@Qc\x8EZ\xEE\xE7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0F\xD7Wa\x0F\xD7a?\xBBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x10\0W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80[\x83Q\x81\x10\x15a\x10\xF6W_\x84\x82\x81Q\x81\x10a\x10\"Wa\x10\"aK\x15V[` \x02` \x01\x01Q\x90P`\x03_\x82`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _\x01T\x84\x83\x81Q\x81\x10a\x10dWa\x10daK\x15V[` \x02` \x01\x01\x81\x81RPP_a\x10\x86\x85\x84\x81Q\x81\x10a\n8Wa\n8aK\x15V[\x90P`\x01`\x01`\x01`\xC0\x1B\x03\x82\x16`\xFF\x89\x16\x1C\x81\x16\x14a\x10\xB9W`@Qc\xD0S\xAA!`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x11a\x10\xEBW`@Qc\xBAP\xF9\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x91P`\x01\x01a\x10\x06V[Pa\x11\x02\x83\x83\x86a\x1AEV[`\xFF\x84\x16_\x81\x81R`\x05` \x90\x81R`@\x91\x82\x90 C\x90\x81\x90U\x91Q\x91\x82R\x7FF\x07}U3\x07c\xF1bi\xFDu\xE5v\x16c\xF4\x19-'\x91t|\x01\x89\xB1j\xD3\x1D\xB0}\xB4\x91\x01`@Q\x80\x91\x03\x90\xA2PPPP\x80`\x01\x01\x90Pa\x0E\xC6V[`2Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x11{WP`2T`\x01`\xFF\x90\x91\x16\x10[\x80a\x11\x95WP0;\x15\x80\x15a\x11\x95WP`2T`\xFF\x16`\x01\x14[a\x11\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`2\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x12 W`2\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x12v`@Q\x80`@\x01`@R\x80`\x16\x81R` \x01u \xAB)\xA92\xB3\xB4\xB9\xBA9<\xA1\xB7\xB7\xB924\xB70\xBA7\xB9`Q\x1B\x81RP`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01ev0.0.1`\xD0\x1B\x81RPa'\xEEV[a\x12\x7F\x86a(\x1FV[a\x12\x88\x85a\x1D\xA2V[a\x12\x91\x83a\x1DeV[a\x12\x9A\x84a\x1E\x0BV[a\x12\xA3\x82a(pV[\x80\x15a\x12\xE9W`2\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPV[``a\x13\x1C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a(\xDCV[\x90P\x90V[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x03` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x81\x01T\x90\x91\x83\x01\x90`\xFF\x16`\x02\x81\x11\x15a\x13yWa\x13yaF\xAEV[`\x02\x81\x11\x15a\x13\x8AWa\x13\x8AaF\xAEV[\x90RP\x92\x91PPV[`@Qc#}\xFBG`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cF\xFB\xF6\x8E\x90a\x13\xDF\x903\x90`\x04\x01aA\x80V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xFAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x1E\x91\x90aK8V[a\x14;W`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x14E_\x19a\x1DeV[V[a\x14Oa\x1D\x0BV[\x81a\x14Y\x81a)\x19V[a\n\xA4\x83\x83a)AV[a\x14ka)\xD2V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\t` R`@\x90 B\x90Ua\x0CF\x82\x82a)\xFDV[a\x14\x97a\x1D\x0BV[a\x14E_a(\x1FV[_a\x0C\xD2\x7F+\xD8!$\x05\x7F\t\x13\xBC;w,\xE7\xB8>\x80W\xC1\xAD\x1F5\x10\xFC\x83w\x8B\xE2\x0F\x10\xEC]\xE6\x83`@Q` \x01a\x14\xE9\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a*\x93V[a\x15\x0Ca\x1D\x0BV[a\n\xA4\x83\x83\x83__a\"\x9DV[_``\x80___```\xC8T__\x1B\x14\x80\x15a\x155WP`\xC9T\x15[a\x15yW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x11RT\r\xCCL\x8E\x88\x15[\x9A[\x9A]\x1AX[\x1A^\x99Y`Z\x1B`D\x82\x01R`d\x01a\x11\xF4V[a\x15\x81a*\xBFV[a\x15\x89a+OV[`@\x80Q_\x80\x82R` \x82\x01\x90\x92R`\x0F`\xF8\x1B\x9B\x93\x9AP\x91\x98PF\x97P0\x96P\x94P\x92P\x90PV[_a\x15\xF1\x7FM@N2v\xE7\xAC!c\xD8\xEEGj\xFAjA\xD1\xF6\x8F\xB7\x1F-\x8BeF\xB2NU\xCE\x01\xB7*\x87\x87\x87\x87\x87`@Q` \x01a\x14\xE9\x96\x95\x94\x93\x92\x91\x90aK\x87V[\x96\x95PPPPPPV[_a\x0C\xD2\x82a\x19pV[a\x16\ra\x1D\x0BV[a\x0B \x81a(pV[`\x0BT`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x16\x14\x90V[``a\n\xBA`\x02\x84\x84a+^V[a\x16@a\x1EtV[`3T_\x90`\x01\x90\x81\x16\x03a\x16hW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x16q\x85a\x16\x16V[a\x16\x8EW`@Qcf\xE5e\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x16\x98\x85a\x1E\xBDV[\x90P_\x80\x80a\x16\xA9\x86\x88\x01\x88aM\x16V[\x92P\x92P\x92P_a\x16\xBA\x8B\x83a,\rV[\x90P_\x84`\x01\x81\x11\x15a\x16\xCFWa\x16\xCFaF\xAEV[\x03a\x17xW_a\x16\xE3\x8C\x83\x88\x87`\x01a,\xA6V[Q\x90P_[\x86Q\x81\x10\x15a\x17qW_\x87\x82\x81Q\x81\x10a\x17\x04Wa\x17\x04aK\x15V[\x01` \x90\x81\x01Q`\xF8\x1C_\x81\x81R`\x01\x90\x92R`@\x90\x91 T\x84Q\x91\x92Pc\xFF\xFF\xFF\xFF\x16\x90\x84\x90\x84\x90\x81\x10a\x17;Wa\x17;aK\x15V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x17hW`@Qc\xC6\xB9\xE7g`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01\x01a\x16\xE8V[PPa\x17\xCDV[`\x01\x84`\x01\x81\x11\x15a\x17\x8CWa\x17\x8CaF\xAEV[\x03a\x17\xB4W_\x80a\x17\x9F\x89\x8B\x01\x8BaMqV[\x94P\x94PPPPa\x17q\x8D\x84\x89\x88\x86\x86a0\xEDV[`@Qc5K\xB8\xAB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPPPPV[a\x17\xE2a\x1D\x0BV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x18GW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x11\xF4V[a\x0B \x81a(\x1FV[_a\x13\x1Ca2\xE2V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xB5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xD9\x91\x90aKQV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x19\nW`@QcyH!\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`3T\x80\x19\x82\x19\x81\x16\x14a\x191W`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`3\x82\x90U`@Q\x82\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01[`@Q\x80\x91\x03\x90\xA2PPV[_a\x0C\xD2`\x02\x83a2\xEBV[``__a\x19\x89\x84a3UV[a\xFF\xFF\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\xA4Wa\x19\xA4a?\xBBV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x19\xCEW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P_\x80[\x82Q\x82\x10\x80\x15a\x19\xE5WPa\x01\0\x81\x10[\x15a\x1A;W`\x01\x81\x1B\x93P\x85\x84\x16\x15a\x1A+W\x80`\xF8\x1B\x83\x83\x81Q\x81\x10a\x1A\x0EWa\x1A\x0EaK\x15V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP\x81`\x01\x01\x91P[a\x1A4\x81aN~V[\x90Pa\x19\xD4V[P\x90\x94\x93PPPPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x81`\xF8\x1B\x81_\x81Q\x81\x10a\x1A{Wa\x1A{aK\x15V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`@Qcl?\xB4\xBF`\xE0\x1B\x81R_\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cl?\xB4\xBF\x90a\x1A\xE3\x90\x88\x90\x88\x90\x88\x90`\x04\x01aN\x96V[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1A\xFEW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1B%\x91\x90\x81\x01\x90aO&V[\x90P_[\x85Q\x81\x10\x15a\x12\xE9W\x81\x81\x81Q\x81\x10a\x1BDWa\x1BDaK\x15V[` \x02` \x01\x01Q\x15a\x1BtWa\x1Bt\x86\x82\x81Q\x81\x10a\x1BfWa\x1BfaK\x15V[` \x02` \x01\x01Q\x84a)\xFDV[`\x01\x01a\x1B)V[_\x83\x81R` \x85\x90R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x1B\x9EWa\x1B\x9EaK\x15V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x85R`\x01` \x1B\x83\x04\x82\x16\x95\x85\x01\x95\x90\x95R`\x01`@\x1B\x90\x91\x04`\x01`\x01`\xC0\x1B\x03\x16\x91\x83\x01\x91\x90\x91R\x90\x92P\x85\x16\x10\x15a\x1C\x0EW`@Qcl\xB1\x9A\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a\x1C4WP\x80` \x01Qc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10[a\x1CQW`@Qc\xBB\xBA`\xCB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x01Q\x90P[\x94\x93PPPPV[`@Qcx!\x9B?`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xF0C6~\x90a\x1C\xAE\x90\x85\x90\x85\x90`\x04\x01aO\xB3V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1C\xC5W__\xFD[PZ\xF1\x15\x80\x15a\x1C\xD7W=__>=_\xFD[PPPP\x81\x7F\xEC)c\xAB!\xC1\xE5\x0E\x1EX*\xA5B\xAF.K\xF7\xBF8\xE6\xE1@<'\xB4.\x1C]nb\x1E\xAA\x82`@Qa\x19d\x91\x90aF\x9CV[`\x96T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x14EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x11\xF4V[`3\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2PV[`\x07T`@Q\x7F1TW\xD8\xA8\xFE`\xF0J\xF1|\x16\xE2\xF5\xA5\xE1\xDBa+1d\x8EX\x03\x03`u\x9E\xF8\xF3R\x8C\x91a\x1D\xE1\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x84\x90aO\xCBV[`@Q\x80\x91\x03\x90\xA1`\x07\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x08T`@Q\x7F\x8F0\xAB\t\xF4:l\x15}\x7F\xCE~\n\x13\xC0\x03\x04,\x1C\x95\xE8\xA7.z\x14j!\xC0\xCA\xA2M\xC9\x91a\x1EJ\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x84\x90aO\xCBV[`@Q\x80\x91\x03\x90\xA1`\x08\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x14EW`@Qc#\xD8q\xA5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E\xD9Wa\x1E\xD9a?\xBBV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1F\x03W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P_[\x83Q\x81\x10\x15a\x1F^W\x83\x81\x81Q\x81\x10a\x1F#Wa\x1F#aK\x15V[` \x02` \x01\x01Q`\xF8\x1B\x82\x82\x81Q\x81\x10a\x1F@Wa\x1F@aK\x15V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a\x1F\x08V[P\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x03` R`@\x81 \x80T\x90\x91a\x1F\x89\x82a\x19pV[\x90P`\x01\x80\x84\x01T`\xFF\x16`\x02\x81\x11\x15a\x1F\xA5Wa\x1F\xA5aF\xAEV[\x14a\x1F\xC3W`@Qc\xAB\xA4s9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80Ta\x1F\xD4\x90\x86\x90`\xFF\x16a'\xBAV[\x90P`\x01`\x01`\xC0\x1B\x03\x81\x16a\x1F\xFDW`@Qch\xB6\xA8u`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a \x14`\x01`\x01`\xC0\x1B\x03\x82\x81\x16\x90\x84\x16\x81\x16\x14\x90V[a 1W`@Qc\xD0S\xAA!`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xC0\x1B\x03\x81\x81\x16\x19\x83\x16\x16a J\x84\x82a3\x7FV[`\x01`\x01`\xC0\x1B\x03\x81\x16a \xA6W`\x01`\x01`\xA0\x1B\x03\x87\x16_\x81\x81R`\x03` R`@\x80\x82 `\x01\x01\x80T`\xFF\x19\x16`\x02\x17\x90UQ\x86\x92\x91\x7F9o\xDC\xB1\x80\xCB\x0F\xEA&\x92\x81\x13\xFB\x0F\xD1\xC3T\x98c\xF9\xCDV>j\x18O\x1DW\x81\x16\xC8\xE4\x91\xA3[`@Qc\xF4\xE2O\xE5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xF4\xE2O\xE5\x90a \xF4\x90\x8A\x90\x8A\x90`\x04\x01aO\xE5V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a!\x0BW__\xFD[PZ\xF1\x15\x80\x15a!\x1DW=__>=_\xFD[PP`@Qc\xBD)\xB8\xCD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc\xBD)\xB8\xCD\x91Pa!o\x90\x87\x90\x8A\x90`\x04\x01aO\xB3V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a!\x86W__\xFD[PZ\xF1\x15\x80\x15a!\x98W=__>=_\xFD[PP`@Qc\xBD)\xB8\xCD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc\xBD)\xB8\xCD\x91Pa!\xEA\x90\x87\x90\x8A\x90`\x04\x01aO\xB3V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\"\x01W__\xFD[PZ\xF1\x15\x80\x15a\x17\xCDW=__>=_\xFD[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R_\x80\x80a\"@_Q` aV\xEA_9_Q\x90_R\x86aP\x1CV[\x90P[a\"L\x81a3\x8BV[\x90\x93P\x91P_Q` aV\xEA_9_Q\x90_R\x82\x83\t\x83\x03a\"\x84W`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[_Q` aV\xEA_9_Q\x90_R`\x01\x82\x08\x90Pa\"CV[_T`\xFF\x16`\xC0\x81\x10a\"\xC3W`@Qc<\xB8\x9C\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80T`\x01\x91\x90\x81\x90a\"\xDA\x90\x84\x90`\xFF\x16aP/V[\x92Pa\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UPa\"\xFC\x81\x87a)AV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x81R``` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a#\x12W\x90PP\x90P_\x85Q`\x01`\x01`@\x1B\x03\x81\x11\x15a#UWa#Ua?\xBBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a#~W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x86Q\x81\x10\x15a#\xDBW\x86\x81\x81Q\x81\x10a#\x9EWa#\x9EaK\x15V[` \x02` \x01\x01Q_\x01Q\x82\x82\x81Q\x81\x10a#\xBBWa#\xBBaK\x15V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a#\x83V[P`@Q\x80`@\x01`@R\x80\x84`\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82\x81RP\x82_\x81Q\x81\x10a$\x0CWa$\x0CaK\x15V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x0BT`@Qc\x010\xFC'`\xE5\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x92c&\x1F\x84\xE0\x92a$l\x92\x91\x90\x91\x16\x90\x86\x90`\x04\x01aPHV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a$\x83W__\xFD[PZ\xF1\x15\x80\x15a$\x95W=__>=_\xFD[P_\x92Pa$\xA1\x91PPV[\x85`\x01\x81\x11\x15a$\xB3Wa$\xB3aF\xAEV[\x03a%:W`@Qc:\xEA\x0B\x9D`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cu\xD4\x17:\x90a%\x08\x90\x86\x90\x8B\x90\x8B\x90`\x04\x01aQXV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a%\x1FW__\xFD[PZ\xF1\x15\x80\x15a%1W=__>=_\xFD[PPPPa&~V[`\x01\x85`\x01\x81\x11\x15a%NWa%NaF\xAEV[\x03a&~W\x83c\xFF\xFF\xFF\xFF\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c)\x81\xEBw`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%\xB6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%\xDA\x91\x90aKlV[c\xFF\xFF\xFF\xFF\x16\x11a%\xFEW`@Qc\x0B\xD4A\xB9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\x06b\xD3\xE1`\xE5\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xCCZ| \x90a&P\x90\x86\x90\x8B\x90\x89\x90\x8C\x90`\x04\x01aQ\x8BV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a&gW__\xFD[PZ\xF1\x15\x80\x15a&yW=__>=_\xFD[PPPP[`@Qc\x13l\xA0\xF9`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c&\xD9A\xF2\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a&\xDFW__\xFD[PZ\xF1\x15\x80\x15a&\xF1W=__>=_\xFD[PP`@Qc\x13l\xA0\xF9`\xE1\x1B\x81R`\xFF\x86\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92Pc&\xD9A\xF2\x91P`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a'VW__\xFD[PZ\xF1\x15\x80\x15a'hW=__>=_\xFD[PPPP\x82`\xFF\x16\x7F\xA3H5\xBC+g>\xC3\x7F\xCF\x15\x91\xA9\x12\x95\xB1c\xFC.\x18\x1EN\xA4\xE73\xBE\xB2}\xE1\xCE\xACL\x89\x89\x89\x89\x89`@Qa'\xA7\x95\x94\x93\x92\x91\x90aQ\xC1V[`@Q\x80\x91\x03\x90\xA2[PPPPPPPPV[__a'\xC5\x84a4\x07V[\x90P\x80\x83`\xFF\x16`\x01\x90\x1B\x11a\n\xBAW`@Qc\xCA\x95s3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`2Ta\x01\0\x90\x04`\xFF\x16a(\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x11\xF4\x90aRBV[a\x0CF\x82\x82a4\xC2V[`\x96\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPV[`\x0BT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x7F\x97p\xF3\xCA\xDF\xDC\xBBo\x93\xAF\x93^\x86\x04q\x11Y\x0C7h'\x1D#~J+\xC0\xB8t\xBE\xD6\x93\x90a(\xB1\x90\x83\x90\x85\x90aO\xCBV[`@Q\x80\x91\x03\x90\xA1P`\x0B\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[``_a(\xE8\x83a5\x10V[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P_\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[_T`\xFF\x90\x81\x16\x90\x82\x16\x10a\x0B W`@Qcs\x10\xCF\xF5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x82\x16_\x81\x81R`\x01` \x90\x81R`@\x91\x82\x90 \x84Q\x81T\x92\x86\x01Q\x84\x87\x01Qa\xFF\xFF\x90\x81\x16`\x01`0\x1B\x02g\xFF\xFF\0\0\0\0\0\0\x19\x91\x90\x92\x16`\x01` \x1B\x02e\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x95\x16c\xFF\xFF\xFF\xFF\x90\x93\x16\x92\x90\x92\x17\x93\x90\x93\x17\x16\x91\x90\x91\x17\x90UQ\x7F>\xE6\xFE\x8DTa\x02D\xC3\xE9\xD3\xC0f\xAEJ\xEE\x99x\x84\xAA(\xF1\x06\x16\xAE\x82\x19%@\x13\x18\xAC\x90a\x19d\x90\x84\x90aJ\xD4V[`\x08T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x14EW`@Qcv\xD8\xAB\x17`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x03` R`@\x90 `\x01\x80\x82\x01T`\xFF\x16`\x02\x81\x11\x15a*.Wa*.aF\xAEV[\x14a*LW`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80T_\x80Ta*_\x90\x85\x90`\xFF\x16a'\xBAV[\x90P_a*k\x83a\x19pV[\x90Pa*\x84`\x01`\x01`\xC0\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x16\x14\x90V[\x15a\x12\xE9Wa\x12\xE9\x86\x86a57V[_a\x0C\xD2a*\x9Fa2\xE2V[\x83`@Qa\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x90 \x90V[```\xCA\x80Ta*\xCE\x90aR\x8DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta*\xFA\x90aR\x8DV[\x80\x15a+EW\x80`\x1F\x10a+\x1CWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a+EV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a+(W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[```\xCB\x80Ta*\xCE\x90aR\x8DV[``_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a+zWa+za?\xBBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a+\xA3W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83Q\x81\x10\x15a,\x04Wa+\xD5\x86\x86\x86\x84\x81Q\x81\x10a+\xC8Wa+\xC8aK\x15V[` \x02` \x01\x01Qa5\xD5V[\x82\x82\x81Q\x81\x10a+\xE7Wa+\xE7aK\x15V[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a+\xA8V[P\x94\x93PPPPV[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x03\xC5\xA6\xB6\x84\x84a,H\x87a\r\xF7V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a,f\x93\x92\x91\x90aR\xE7V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a,\x82W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xBA\x91\x90aS`V[a,\xCA`@Q\x80``\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81RP\x90V[_\x80Ta,\xDB\x90\x86\x90`\xFF\x16a'\xBAV[\x90P_a,\xE7\x87a\x19pV[\x90P`\x01`\x01`\xC0\x1B\x03\x82\x16a-\x10W`@Qc\x13\xCAFW`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x82\x16`\x01`\x01`\xC0\x1B\x03\x16\x15a-:W`@Qc\x0Ch\x16\xCD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\nT`\x01`\x01`\xA0\x1B\x03\x89\x16_\x90\x81R`\t` R`@\x90 T`\x01`\x01`\xC0\x1B\x03\x83\x81\x16\x90\x85\x16\x17\x91B\x91a-q\x91\x90aSwV[\x10a-\x8FW`@Qc\x19hg}`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a-\x99\x88\x82a3\x7FV[a-\xA3\x88\x87a\x1C`V[`\x01`\x01`\x01`\xA0\x1B\x03\x8A\x16_\x90\x81R`\x03` R`@\x90 `\x01\x01T`\xFF\x16`\x02\x81\x11\x15a-\xD4Wa-\xD4aF\xAEV[\x14a.kW`@\x80Q\x80\x82\x01\x82R\x89\x81R`\x01` \x80\x83\x01\x82\x81R`\x01`\x01`\xA0\x1B\x03\x8E\x16_\x90\x81R`\x03\x90\x92R\x93\x90 \x82Q\x81U\x92Q\x83\x82\x01\x80T\x93\x94\x93\x91\x92\x90\x91`\xFF\x19\x16\x90\x83`\x02\x81\x11\x15a..Wa..aF\xAEV[\x02\x17\x90UPP`@Q\x89\x91P`\x01`\x01`\xA0\x1B\x03\x8B\x16\x90\x7F\xE8\xE6\x8C\xEF\x1C:v\x1E\xD7\xBE~\x84c\xA3u\xF2\x7F{\xC35\xE5\x18$\"<\xAC\xCEcn\xC5\xC3\xFE\x90_\x90\xA3[`@Qc\x1F\xD9<\xA9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c?\xB2yR\x90a.\xB9\x90\x8C\x90\x8B\x90`\x04\x01aO\xE5V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a.\xD0W__\xFD[PZ\xF1\x15\x80\x15a.\xE2W=__>=_\xFD[PP`@Qc%PGw`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc%PGw\x91Pa/6\x90\x8C\x90\x8C\x90\x8C\x90`\x04\x01aS\x8AV[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a/QW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra/x\x91\x90\x81\x01\x90aT\x14V[`@\x80\x87\x01\x91\x90\x91R` \x86\x01\x91\x90\x91RQb\xBF\xF0M`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90b\xBF\xF0M\x90a/\xD3\x90\x8B\x90\x8B\x90`\x04\x01aO\xB3V[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a/\xEEW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra0\x15\x91\x90\x81\x01\x90aTmV[\x84R\x84\x15a0\xE1W_[\x87Q\x81\x10\x15a0\xDFW_`\x01_\x8A\x84\x81Q\x81\x10a0>Wa0>aK\x15V[\x01` \x90\x81\x01Q`\xF8\x1C\x82R\x81\x81\x01\x92\x90\x92R`@\x90\x81\x01_ \x81Q``\x81\x01\x83R\x90Tc\xFF\xFF\xFF\xFF\x81\x16\x80\x83Ra\xFF\xFF`\x01` \x1B\x83\x04\x81\x16\x95\x84\x01\x95\x90\x95R`\x01`0\x1B\x90\x91\x04\x90\x93\x16\x91\x81\x01\x91\x90\x91R\x87Q\x80Q\x91\x93P\x90\x84\x90\x81\x10a0\xA9Wa0\xA9aK\x15V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a0\xD6W`@Qc\xC6\xB9\xE7g`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01\x01a0\x1FV[P[PPP\x95\x94PPPPPV[\x83Q\x82Q\x14a1\x0FW`@Qc\xAA\xAD\x13\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a1\x1B\x86\x86\x84\x84a6\xEDV[_a1)\x87\x87\x87\x87_a,\xA6V[\x90P_[\x85Q\x81\x10\x15a'\xB0W_`\x01_\x88\x84\x81Q\x81\x10a1LWa1LaK\x15V[\x01` \x90\x81\x01Q`\xF8\x1C\x82R\x81\x81\x01\x92\x90\x92R`@\x90\x81\x01_ \x81Q``\x81\x01\x83R\x90Tc\xFF\xFF\xFF\xFF\x81\x16\x80\x83Ra\xFF\xFF`\x01` \x1B\x83\x04\x81\x16\x95\x84\x01\x95\x90\x95R`\x01`0\x1B\x90\x91\x04\x90\x93\x16\x91\x81\x01\x91\x90\x91R\x84Q\x80Q\x91\x93P\x90\x84\x90\x81\x10a1\xB7Wa1\xB7aK\x15V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a2\xD9Wa2K\x87\x83\x81Q\x81\x10a1\xE0Wa1\xE0aK\x15V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x84`@\x01Q\x84\x81Q\x81\x10a2\x04Wa2\x04aK\x15V[` \x02` \x01\x01Q\x8B\x86` \x01Q\x86\x81Q\x81\x10a2#Wa2#aK\x15V[` \x02` \x01\x01Q\x89\x87\x81Q\x81\x10a2=Wa2=aK\x15V[` \x02` \x01\x01Q\x86a7\x98V[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x87\x83\x81Q\x81\x10a2}Wa2}aK\x15V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x81_\x81Q\x81\x10a2\x9AWa2\x9AaK\x15V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SPa2\xD7\x86\x84\x81Q\x81\x10a2\xC5Wa2\xC5aK\x15V[` \x02` \x01\x01Q` \x01Q\x82a)\xFDV[P[P`\x01\x01a1-V[_a\x13\x1Ca9\x19V[_\x81\x81R` \x83\x90R`@\x81 T\x80\x82\x03a3\tW_\x91PPa\x0C\xD2V[_\x83\x81R` \x85\x90R`@\x90 a3!`\x01\x83aT\xFCV[\x81T\x81\x10a31Wa31aK\x15V[_\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01`\xC0\x1B\x03\x16\x91Pa\x0C\xD2\x90PV[_\x80[\x82\x15a\x0C\xD2Wa3i`\x01\x84aT\xFCV[\x90\x92\x16\x91\x80a3w\x81aU\x0FV[\x91PPa3XV[a\x0CF`\x02\x83\x83a9\x8CV[_\x80\x80_Q` aV\xEA_9_Q\x90_R`\x03_Q` aV\xEA_9_Q\x90_R\x86_Q` aV\xEA_9_Q\x90_R\x88\x89\t\t\x08\x90P_a3\xFB\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R_Q` aV\xEA_9_Q\x90_Ra;EV[\x91\x95\x91\x94P\x90\x92PPPV[_a\x01\0\x82Q\x11\x15a4,W`@Qc}\xA5NG`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81Q_\x03a4;WP_\x91\x90PV[__\x83_\x81Q\x81\x10a4OWa4OaK\x15V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x81\x90\x1B\x92P[\x84Q\x81\x10\x15a4\xB9W\x84\x81\x81Q\x81\x10a4}Wa4}aK\x15V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x1B\x91P\x82\x82\x11a4\xADW`@Qc\x10\x19\x10i`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91\x81\x17\x91`\x01\x01a4bV[P\x90\x93\x92PPPV[`2Ta\x01\0\x90\x04`\xFF\x16a4\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x11\xF4\x90aRBV[`\xCAa4\xF5\x83\x82aUsV[P`\xCBa5\x02\x82\x82aUsV[PP_`\xC8\x81\x90U`\xC9UPV[_`\xFF\x82\x16`\x1F\x81\x11\x15a\x0C\xD2W`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x82R`\x0BT\x81\x16` \x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91cn4\x92\xB5\x91\x90\x81\x01a5\x8D\x85a;\xBEV[\x81RP`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a5\xAC\x91\x90aV-V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a5\xC3W__\xFD[PZ\xF1\x15\x80\x15a\x12\xE9W=__>=_\xFD[_\x81\x81R` \x84\x90R`@\x81 T\x81[\x81\x81\x10\x15a6XW`\x01a5\xF9\x82\x84aT\xFCV[a6\x03\x91\x90aT\xFCV[\x92P\x84c\xFF\xFF\xFF\xFF\x16\x86_\x86\x81R` \x01\x90\x81R` \x01_ \x84c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a63Wa63aK\x15V[_\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a6PWPPa\n\xBAV[`\x01\x01a5\xE5V[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FRegistryCoordinator.getQuorumBit`D\x82\x01R\x7FmapIndexAtBlockNumber: no bitmap`d\x82\x01R\x7F update found for operatorId\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x11\xF4V[` \x80\x82\x01Q_\x90\x81R`\x04\x90\x91R`@\x90 T`\xFF\x16\x15a7\"W`@Qco\xBE\xFE\xC3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[B\x81`@\x01Q\x10\x15a7GW`@Qc\x08\x19\xBD\xCD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x80\x82\x01\x80Q_\x90\x81R`\x04\x90\x92R`@\x91\x82\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U`\x07T\x90Q\x91\x83\x01Qa\x0E1\x92`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91a7\x91\x91\x88\x91\x88\x91\x88\x91\x90a\x15\xB2V[\x83Qa<cV[` \x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x80\x82\x16_\x81\x81R`\x03\x90\x94R`@\x90\x93 T\x91\x92\x90\x87\x16\x03a7\xDBW`@QcV\x16\x8BA`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x87`\xFF\x16\x84_\x01Q`\xFF\x16\x14a8\x04W`@Qc\x8EZ\xEE\xE7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@QcT\x01\xED'`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`\xFF\x89\x16`$\x82\x01R_\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cT\x01\xED'\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8rW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8\x96\x91\x90aViV[\x90Pa8\xA2\x81\x85a<\x8BV[`\x01`\x01``\x1B\x03\x16\x86`\x01`\x01``\x1B\x03\x16\x11a8\xD3W`@QcLD\x99]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a8\xDD\x88\x85a<\xAEV[`\x01`\x01``\x1B\x03\x16\x81`\x01`\x01``\x1B\x03\x16\x10a9\x0EW`@Qc\xB1\x87\xE8i`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPPV[_\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fa9Ca<\xC7V[a9Ka=\x1FV[`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[_\x82\x81R` \x84\x90R`@\x81 T\x90\x81\x90\x03a:0W_\x83\x81R` \x85\x81R`@\x80\x83 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x86\x81R`\x01`\x01`\xC0\x1B\x03\x80\x8A\x16\x95\x84\x01\x95\x86R\x84T`\x01\x81\x01\x86U\x94\x88R\x95\x90\x96 \x91Q\x91\x90\x92\x01\x80T\x95Q\x93Q\x90\x94\x16`\x01`@\x1B\x02`\x01`\x01`@\x1B\x03\x93\x83\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x96\x16\x91\x90\x92\x16\x17\x93\x90\x93\x17\x16\x91\x90\x91\x17\x90Ua\x0E1V[_\x83\x81R` \x85\x90R`@\x81 a:H`\x01\x84aT\xFCV[\x81T\x81\x10a:XWa:XaK\x15V[_\x91\x82R` \x90\x91 \x01\x80T\x90\x91Pc\xFF\xFF\xFF\xFFC\x81\x16\x91\x16\x03a:\x99W\x80T`\x01`\x01`@\x1B\x03\x16`\x01`@\x1B`\x01`\x01`\xC0\x1B\x03\x85\x16\x02\x17\x81Ua\r\xF0V[\x80Tc\xFF\xFF\xFF\xFFC\x81\x16`\x01` \x1B\x81\x81\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x94\x16\x93\x90\x93\x17\x84U_\x87\x81R` \x89\x81R`@\x80\x83 \x81Q``\x81\x01\x83R\x94\x85R\x84\x83\x01\x84\x81R`\x01`\x01`\xC0\x1B\x03\x80\x8C\x16\x93\x87\x01\x93\x84R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x94Q\x94\x01\x80T\x93Q\x91Q\x90\x92\x16`\x01`@\x1B\x02`\x01`\x01`@\x1B\x03\x91\x86\x16\x90\x96\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x93\x90\x94\x16\x92\x90\x92\x17\x17\x91\x90\x91\x16\x91\x90\x91\x17\x90UPPPPPV[__a;Oa?\x7FV[a;Wa?\x9DV[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80a;\x94W\xFE[P\x82a;\xB3W`@Qc\xD5\x1E\xDA\xE3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PQ\x95\x94PPPPPV[``_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a;\xDAWa;\xDAa?\xBBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a<\x03W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83Q\x81\x10\x15a\x1F^W\x83\x81\x81Q\x81\x10a<#Wa<#aK\x15V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16\x82\x82\x81Q\x81\x10a<FWa<FaK\x15V[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a<\x08V[a<n\x83\x83\x83a=OV[a\n\xA4W`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Q_\x90a'\x10\x90a<\xA4\x90a\xFF\xFF\x16\x85aV\x84V[a\n\xBA\x91\x90aV\xA6V[`@\x81\x01Q_\x90a'\x10\x90a<\xA4\x90a\xFF\xFF\x16\x85aV\x84V[__a<\xD1a*\xBFV[\x80Q\x90\x91P\x15a<\xE8W\x80Q` \x90\x91\x01 \x91\x90PV[`\xC8T\x80\x15a<\xF7W\x92\x91PPV[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x92PPP\x90V[__a=)a+OV[\x80Q\x90\x91P\x15a=@W\x80Q` \x90\x91\x01 \x91\x90PV[`\xC9T\x80\x15a<\xF7W\x92\x91PPV[___a=\\\x85\x85a=\xA3V[\x90\x92P\x90P_\x81`\x04\x81\x11\x15a=tWa=taF\xAEV[\x14\x80\x15a=\x92WP\x85`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14[\x80a\x15\xF1WPa\x15\xF1\x86\x86\x86a=\xE5V[__\x82Q`A\x03a=\xD7W` \x83\x01Q`@\x84\x01Q``\x85\x01Q_\x1Aa=\xCB\x87\x82\x85\x85a>\xCCV[\x94P\x94PPPPa=\xDEV[P_\x90P`\x02[\x92P\x92\x90PV[___\x85`\x01`\x01`\xA0\x1B\x03\x16c\x16&\xBA~`\xE0\x1B\x86\x86`@Q`$\x01a>\r\x92\x91\x90aO\xB3V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90Qa>K\x91\x90aV\xD3V[_`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80_\x81\x14a>\x83W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a>\x88V[``\x91P[P\x91P\x91P\x81\x80\x15a>\x9CWP` \x81Q\x10\x15[\x80\x15a\x15\xF1WP\x80Qc\x0B\x13]?`\xE1\x1B\x90a>\xC1\x90\x83\x01` \x90\x81\x01\x90\x84\x01aS`V[\x14\x96\x95PPPPPPV[_\x80o\xA2\xA8\x91\x8C\xA8[\xAF\xE2 \x16\xD0\xB9\x97\xE4\xDF``\x01`\xFF\x1B\x03\x83\x11\x15a>\xF7WP_\x90P`\x03a?vV[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a?HW=__>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a?pW_`\x01\x92P\x92PPa?vV[\x91P_\x90P[\x94P\x94\x92PPPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a?\xF1Wa?\xF1a?\xBBV[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a?\xF1Wa?\xF1a?\xBBV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a@AWa@Aa?\xBBV[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a@aWa@aa?\xBBV[P`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0B W__\xFD[_\x82`\x1F\x83\x01\x12a@\x8EW__\xFD[\x815a@\xA1a@\x9C\x82a@IV[a@\x19V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a@\xC2W__\xFD[` \x85\x01[\x83\x81\x10\x15a@\xE8W\x805a@\xDA\x81a@kV[\x83R` \x92\x83\x01\x92\x01a@\xC7V[P\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15aA\x02W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aA\x17W__\xFD[a\x1CX\x84\x82\x85\x01a@\x7FV[_` \x82\x84\x03\x12\x15aA3W__\xFD[P5\x91\x90PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0B W__\xFD[___``\x84\x86\x03\x12\x15aA]W__\xFD[\x835\x92P` \x84\x015aAo\x81aA:V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[_\x82`\x1F\x83\x01\x12aA\xA3W__\xFD[\x815` \x83\x01__`\x01`\x01`@\x1B\x03\x84\x11\x15aA\xC2WaA\xC2a?\xBBV[P`\x1F\x83\x01`\x1F\x19\x16` \x01aA\xD7\x81a@\x19V[\x91PP\x82\x81R\x85\x83\x83\x01\x11\x15aA\xEBW__\xFD[\x82\x82` \x83\x017_\x92\x81\x01` \x01\x92\x90\x92RP\x93\x92PPPV[_` \x82\x84\x03\x12\x15aB\x15W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aB*W__\xFD[a\x1CX\x84\x82\x85\x01aA\x94V[_` \x82\x84\x03\x12\x15aBFW__\xFD[\x815a\n\xBA\x81a@kV[__`@\x83\x85\x03\x12\x15aBbW__\xFD[PP\x805\x92` \x90\x91\x015\x91PV[\x805`\xFF\x81\x16\x81\x14aB\x81W__\xFD[\x91\x90PV[_` \x82\x84\x03\x12\x15aB\x96W__\xFD[a\n\xBA\x82aBqV[_\x82`\x1F\x83\x01\x12aB\xAEW__\xFD[\x815aB\xBCa@\x9C\x82a@IV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aB\xDDW__\xFD[` \x85\x01[\x83\x81\x10\x15a@\xE8W\x805aB\xF5\x81aA:V[\x83R` \x92\x83\x01\x92\x01aB\xE2V[___``\x84\x86\x03\x12\x15aC\x15W__\xFD[\x835aC \x81a@kV[\x92P` \x84\x015aC0\x81a@kV[\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aCJW__\xFD[aCV\x86\x82\x87\x01aB\x9FV[\x91PP\x92P\x92P\x92V[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x81\x01a\x0C\xD2V[\x805a\xFF\xFF\x81\x16\x81\x14aB\x81W__\xFD[_``\x82\x84\x03\x12\x15aC\x98W__\xFD[aC\xA0a?\xCFV[\x90P\x815aC\xAD\x81aA:V[\x81RaC\xBB` \x83\x01aCwV[` \x82\x01RaC\xCC`@\x83\x01aCwV[`@\x82\x01R\x92\x91PPV[`\x01`\x01``\x1B\x03\x81\x16\x81\x14a\x0B W__\xFD[_\x82`\x1F\x83\x01\x12aC\xFAW__\xFD[\x815aD\x08a@\x9C\x82a@IV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aD)W__\xFD[` \x85\x01[\x83\x81\x10\x15a@\xE8W`@\x81\x88\x03\x12\x15aDEW__\xFD[aDMa?\xF7V[\x815aDX\x81a@kV[\x81R` \x82\x015aDh\x81aC\xD7V[` \x82\x81\x01\x91\x90\x91R\x90\x84R\x92\x90\x92\x01\x91`@\x01aD.V[____`\xC0\x85\x87\x03\x12\x15aD\x94W__\xFD[aD\x9E\x86\x86aC\x88V[\x93P``\x85\x015aD\xAE\x81aC\xD7V[\x92P`\x80\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aD\xC8W__\xFD[aD\xD4\x87\x82\x88\x01aC\xEBV[\x92PP`\xA0\x85\x015aD\xE5\x81aA:V[\x93\x96\x92\x95P\x90\x93PPV[__\x83`\x1F\x84\x01\x12aE\0W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aE\x16W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a=\xDEW__\xFD[___`@\x84\x86\x03\x12\x15aE?W__\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15aETW__\xFD[\x84\x01`\x1F\x81\x01\x86\x13aEdW__\xFD[\x805aEra@\x9C\x82a@IV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x88\x83\x11\x15aE\x93W__\xFD[` \x84\x01[\x83\x81\x10\x15aE\xD3W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aE\xB5W__\xFD[aE\xC4\x8B` \x83\x89\x01\x01a@\x7FV[\x84RP` \x92\x83\x01\x92\x01aE\x98V[P\x95PPPP` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aE\xF1W__\xFD[aE\xFD\x86\x82\x87\x01aD\xF0V[\x94\x97\x90\x96P\x93\x94PPPPV[_____`\xA0\x86\x88\x03\x12\x15aF\x1EW__\xFD[\x855aF)\x81a@kV[\x94P` \x86\x015aF9\x81a@kV[\x93P`@\x86\x015aFI\x81a@kV[\x92P``\x86\x015\x91P`\x80\x86\x015aF`\x81a@kV[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\n\xBA` \x83\x01\x84aFnV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x03\x81\x10aF\xD2WaF\xD2aF\xAEV[\x90RV[\x81Q\x81R` \x80\x83\x01Q`@\x83\x01\x91a\x1F^\x90\x84\x01\x82aF\xC2V[__`\x80\x83\x85\x03\x12\x15aG\x02W__\xFD[aG\x0B\x83aBqV[\x91PaG\x1A\x84` \x85\x01aC\x88V[\x90P\x92P\x92\x90PV[__`@\x83\x85\x03\x12\x15aG4W__\xFD[\x825aG?\x81a@kV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aGYW__\xFD[aGe\x85\x82\x86\x01aA\x94V[\x91PP\x92P\x92\x90PV[___`\xA0\x84\x86\x03\x12\x15aG\x81W__\xFD[aG\x8B\x85\x85aC\x88V[\x92P``\x84\x015aG\x9B\x81aC\xD7V[\x91P`\x80\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aG\xB5W__\xFD[aCV\x86\x82\x87\x01aC\xEBV[`\xFF`\xF8\x1B\x88\x16\x81R`\xE0` \x82\x01R_aG\xDF`\xE0\x83\x01\x89aFnV[\x82\x81\x03`@\x84\x01RaG\xF1\x81\x89aFnV[``\x84\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x85\x01R`\xA0\x84\x01\x86\x90R\x83\x81\x03`\xC0\x85\x01R\x84Q\x80\x82R` \x80\x87\x01\x93P\x90\x91\x01\x90_[\x81\x81\x10\x15aHFW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aH(V[P\x90\x9B\x9APPPPPPPPPPPV[_\x82`\x1F\x83\x01\x12aHfW__\xFD[\x815aHta@\x9C\x82a@IV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aH\x95W__\xFD[` \x85\x01[\x83\x81\x10\x15a@\xE8W`@\x81\x88\x03\x12\x15aH\xB1W__\xFD[aH\xB9a?\xF7V[aH\xC2\x82aBqV[\x81R` \x82\x015aH\xD2\x81a@kV[` \x82\x81\x01\x91\x90\x91R\x90\x84R\x92\x90\x92\x01\x91`@\x01aH\x9AV[_____`\xA0\x86\x88\x03\x12\x15aH\xFFW__\xFD[\x855aI\n\x81a@kV[\x94P` \x86\x015\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aI+W__\xFD[aI7\x88\x82\x89\x01aHWV[\x95\x98\x94\x97P\x94\x95``\x81\x015\x95P`\x80\x015\x93\x92PPPV[__`@\x83\x85\x03\x12\x15aIaW__\xFD[\x825aIl\x81aA:V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aI\x86W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13aI\x96W__\xFD[\x805aI\xA4a@\x9C\x82a@IV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15aI\xC5W__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aI\xE7W\x835\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aI\xCCV[\x80\x94PPPPP\x92P\x92\x90PV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aJ+W\x81Qc\xFF\xFF\xFF\xFF\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aJ\x07V[P\x93\x94\x93PPPPV[` \x81R_a\n\xBA` \x83\x01\x84aI\xF5V[_____`\x80\x86\x88\x03\x12\x15aJ[W__\xFD[\x855aJf\x81a@kV[\x94P` \x86\x015aJv\x81a@kV[\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\x90W__\xFD[aJ\x9C\x88\x82\x89\x01aB\x9FV[\x93PP``\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\xB7W__\xFD[aJ\xC3\x88\x82\x89\x01aD\xF0V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[``\x81\x01a\x0C\xD2\x82\x84c\xFF\xFF\xFF\xFF\x81Q\x16\x82Ra\xFF\xFF` \x82\x01Q\x16` \x83\x01Ra\xFF\xFF`@\x82\x01Q\x16`@\x83\x01RPPV[` \x81\x01a\x0C\xD2\x82\x84aF\xC2V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80Q\x80\x15\x15\x81\x14aB\x81W__\xFD[_` \x82\x84\x03\x12\x15aKHW__\xFD[a\n\xBA\x82aK)V[_` \x82\x84\x03\x12\x15aKaW__\xFD[\x81Qa\n\xBA\x81a@kV[_` \x82\x84\x03\x12\x15aK|W__\xFD[\x81Qa\n\xBA\x81aA:V[_`\xC0\x82\x01\x88\x83R`\x01\x80`\xA0\x1B\x03\x88\x16` \x84\x01R\x86`@\x84\x01R`\xC0``\x84\x01R\x80\x86Q\x80\x83R`\xE0\x85\x01\x91P` \x88\x01\x92P_[\x81\x81\x10\x15aK\xF4W\x83Q\x80Q`\xFF\x16\x84R` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81\x85\x01R\x90\x93\x01\x92`@\x90\x92\x01\x91`\x01\x01aK\xBEV[PP`\x80\x84\x01\x95\x90\x95RPP`\xA0\x01R\x94\x93PPPPV[\x805`\x02\x81\x10aB\x81W__\xFD[_`@\x82\x84\x03\x12\x15aL*W__\xFD[aL2a?\xF7V[\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[_\x82`\x1F\x83\x01\x12aLXW__\xFD[aL`a?\xF7V[\x80`@\x84\x01\x85\x81\x11\x15aLqW__\xFD[\x84[\x81\x81\x10\x15aL\x8BW\x805\x84R` \x93\x84\x01\x93\x01aLsV[P\x90\x95\x94PPPPPV[_\x81\x83\x03a\x01\0\x81\x12\x15aL\xA8W__\xFD[aL\xB0a?\xCFV[\x91PaL\xBC\x84\x84aL\x1AV[\x82RaL\xCB\x84`@\x85\x01aL\x1AV[` \x83\x01R`\x80`\x7F\x19\x82\x01\x12\x15aL\xE1W__\xFD[PaL\xEAa?\xF7V[aL\xF7\x84`\x80\x85\x01aLIV[\x81RaM\x06\x84`\xC0\x85\x01aLIV[` \x82\x01R`@\x82\x01R\x92\x91PPV[___a\x01@\x84\x86\x03\x12\x15aM)W__\xFD[aM2\x84aL\x0CV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aMLW__\xFD[aMX\x86\x82\x87\x01aA\x94V[\x92PPaMh\x85`@\x86\x01aL\x96V[\x90P\x92P\x92P\x92V[_____a\x01\x80\x86\x88\x03\x12\x15aM\x86W__\xFD[aM\x8F\x86aL\x0CV[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aM\xA9W__\xFD[aM\xB5\x88\x82\x89\x01aA\x94V[\x94PPaM\xC5\x87`@\x88\x01aL\x96V[\x92Pa\x01@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aM\xE0W__\xFD[aM\xEC\x88\x82\x89\x01aHWV[\x92PPa\x01`\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aN\x08W__\xFD[\x86\x01``\x81\x89\x03\x12\x15aN\x19W__\xFD[aN!a?\xCFV[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aN6W__\xFD[aNB\x8A\x82\x85\x01aA\x94V[\x82RP` \x82\x81\x015\x90\x82\x01R`@\x91\x82\x015\x91\x81\x01\x91\x90\x91R\x94\x97\x93\x96P\x91\x94P\x92\x91\x90PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_`\x01\x82\x01aN\x8FWaN\x8FaNjV[P`\x01\x01\x90V[``\x80\x82R\x84Q\x90\x82\x01\x81\x90R_\x90` \x86\x01\x90`\x80\x84\x01\x90\x83[\x81\x81\x10\x15aN\xD8W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aN\xB1V[PP\x83\x81\x03` \x80\x86\x01\x91\x90\x91R\x86Q\x80\x83R\x91\x81\x01\x92P\x86\x01\x90_[\x81\x81\x10\x15aO\x13W\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aN\xF5V[PPP`\xFF\x84\x16`@\x84\x01R\x90Pa\x1CXV[_` \x82\x84\x03\x12\x15aO6W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aOKW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aO[W__\xFD[\x80QaOia@\x9C\x82a@IV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15aO\x8AW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a\x15\xF1WaO\xA2\x84aK)V[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aO\x91V[\x82\x81R`@` \x82\x01R_a\n\xB7`@\x83\x01\x84aFnV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x16` \x82\x01R`@\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90a\n\xB7\x90\x83\x01\x84aFnV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82aP*WaP*aP\x08V[P\x06\x90V[`\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x0C\xD2Wa\x0C\xD2aNjV[_`@\x82\x01`\x01\x80`\xA0\x1B\x03\x85\x16\x83R`@` \x84\x01R\x80\x84Q\x80\x83R``\x85\x01\x91P``\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aP\xFDW\x86\x85\x03`_\x19\x01\x84R\x81Q\x80Qc\xFF\xFF\xFF\xFF\x16\x86R` \x90\x81\x01Q`@\x82\x88\x01\x81\x90R\x81Q\x90\x88\x01\x81\x90R\x91\x01\x90_\x90``\x88\x01\x90[\x80\x83\x10\x15aP\xE5W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x93\x84\x01\x93`\x01\x93\x90\x93\x01\x92\x90\x91\x01\x90aP\xBAV[P\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aP|V[P\x92\x97\x96PPPPPPPV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aJ+W\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x87R` \x90\x81\x01Q`\x01`\x01``\x1B\x03\x16\x81\x88\x01R`@\x90\x96\x01\x95\x90\x91\x01\x90`\x01\x01aQ\x1CV[`\xFF\x84\x16\x81R`\x01`\x01``\x1B\x03\x83\x16` \x82\x01R```@\x82\x01R_aQ\x82``\x83\x01\x84aQ\nV[\x95\x94PPPPPV[`\xFF\x85\x16\x81R`\x01`\x01``\x1B\x03\x84\x16` \x82\x01Rc\xFF\xFF\xFF\xFF\x83\x16`@\x82\x01R`\x80``\x82\x01R_a\x15\xF1`\x80\x83\x01\x84aQ\nV[aQ\xF0\x81\x87c\xFF\xFF\xFF\xFF\x81Q\x16\x82Ra\xFF\xFF` \x82\x01Q\x16` \x83\x01Ra\xFF\xFF`@\x82\x01Q\x16`@\x83\x01RPPV[`\x01`\x01``\x1B\x03\x85\x16``\x82\x01R`\xE0`\x80\x82\x01R_aR\x14`\xE0\x83\x01\x86aQ\nV[\x90P`\x02\x84\x10aR&WaR&aF\xAEV[\x83`\xA0\x83\x01Rc\xFF\xFF\xFF\xFF\x83\x16`\xC0\x83\x01R\x96\x95PPPPPPV[` \x80\x82R`+\x90\x82\x01R\x7FInitializable: contract is not i`@\x82\x01Rjnitializing`\xA8\x1B``\x82\x01R`\x80\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80aR\xA1W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aR\xBFWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[\x80_[`\x02\x81\x10\x15a\x0E1W\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01aR\xC8V[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R\x82Q\x80Q` \x80\x84\x01\x91\x90\x91R\x01Q`@\x82\x01Ra\x01`\x81\x01` \x84\x81\x01Q\x80Q``\x85\x01R\x90\x81\x01Q`\x80\x84\x01RP`@\x84\x01QaS6`\xA0\x84\x01\x82QaR\xC5V[` \x01QaSG`\xE0\x84\x01\x82aR\xC5V[P\x82Qa\x01 \x83\x01R` \x83\x01Qa\x01@\x83\x01Ra\x1CXV[_` \x82\x84\x03\x12\x15aSpW__\xFD[PQ\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x0C\xD2Wa\x0C\xD2aNjV[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R_aQ\x82``\x83\x01\x84aFnV[_\x82`\x1F\x83\x01\x12aS\xBFW__\xFD[\x81QaS\xCDa@\x9C\x82a@IV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aS\xEEW__\xFD[` \x85\x01[\x83\x81\x10\x15a@\xE8W\x80QaT\x06\x81aC\xD7V[\x83R` \x92\x83\x01\x92\x01aS\xF3V[__`@\x83\x85\x03\x12\x15aT%W__\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aT:W__\xFD[aTF\x85\x82\x86\x01aS\xB0V[\x92PP` \x83\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aTaW__\xFD[aGe\x85\x82\x86\x01aS\xB0V[_` \x82\x84\x03\x12\x15aT}W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aT\x92W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aT\xA2W__\xFD[\x80QaT\xB0a@\x9C\x82a@IV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15aT\xD1W__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a\x15\xF1W\x83QaT\xEB\x81aA:V[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aT\xD8V[\x81\x81\x03\x81\x81\x11\x15a\x0C\xD2Wa\x0C\xD2aNjV[_a\xFF\xFF\x82\x16a\xFF\xFF\x81\x03aU&WaU&aNjV[`\x01\x01\x92\x91PPV[`\x1F\x82\x11\x15a\n\xA4W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15aUTWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\r\xF0W_\x81U`\x01\x01aU`V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aU\x8CWaU\x8Ca?\xBBV[aU\xA0\x81aU\x9A\x84TaR\x8DV[\x84aU/V[` `\x1F\x82\x11`\x01\x81\x14aU\xD2W_\x83\x15aU\xBBWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\r\xF0V[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15aV\x01W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01aU\xE1V[P\x84\x82\x10\x15aV\x1EW\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[` \x80\x82R\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83\x83\x01R\x90\x83\x01Q\x16`@\x80\x83\x01\x91\x90\x91R\x82\x01Q``\x80\x83\x01R_\x90a\x1CX`\x80\x84\x01\x82aI\xF5V[_` \x82\x84\x03\x12\x15aVyW__\xFD[\x81Qa\n\xBA\x81aC\xD7V[`\x01`\x01``\x1B\x03\x81\x81\x16\x83\x82\x16\x02\x90\x81\x16\x90\x81\x81\x14a\x1F^Wa\x1F^aNjV[_`\x01`\x01``\x1B\x03\x83\x16\x80aV\xBEWaV\xBEaP\x08V[\x80`\x01`\x01``\x1B\x03\x84\x16\x04\x91PP\x92\x91PPV[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 \xA42\xD9\xF0\x89b\xAD\xE1\xF8<W\xC8_\xFE\x1B\x12\xB9\xEB\x84\x98W[9\xE5\tI\xD7s\xE7\xA5R\x8AdsolcC\0\x08\x1B\x003",
    );
    /**Custom error with signature `AlreadyRegisteredForQuorums()` and selector `0x0c6816cd`.
    ```solidity
    error AlreadyRegisteredForQuorums();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AlreadyRegisteredForQuorums {}
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<AlreadyRegisteredForQuorums> for UnderlyingRustTuple<'_> {
            fn from(value: AlreadyRegisteredForQuorums) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AlreadyRegisteredForQuorums {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AlreadyRegisteredForQuorums {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AlreadyRegisteredForQuorums()";
            const SELECTOR: [u8; 4] = [12u8, 104u8, 22u8, 205u8];
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
    /**Custom error with signature `BitmapCannotBeZero()` and selector `0xd16d50ea`.
    ```solidity
    error BitmapCannotBeZero();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BitmapCannotBeZero {}
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<BitmapCannotBeZero> for UnderlyingRustTuple<'_> {
            fn from(value: BitmapCannotBeZero) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BitmapCannotBeZero {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BitmapCannotBeZero {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BitmapCannotBeZero()";
            const SELECTOR: [u8; 4] = [209u8, 109u8, 80u8, 234u8];
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
    /**Custom error with signature `BitmapEmpty()` and selector `0x13ca4657`.
    ```solidity
    error BitmapEmpty();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BitmapEmpty {}
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<BitmapEmpty> for UnderlyingRustTuple<'_> {
            fn from(value: BitmapEmpty) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BitmapEmpty {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BitmapEmpty {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BitmapEmpty()";
            const SELECTOR: [u8; 4] = [19u8, 202u8, 70u8, 87u8];
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
    /**Custom error with signature `BitmapUpdateIsAfterBlockNumber()` and selector `0x6cb19aff`.
    ```solidity
    error BitmapUpdateIsAfterBlockNumber();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BitmapUpdateIsAfterBlockNumber {}
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<BitmapUpdateIsAfterBlockNumber> for UnderlyingRustTuple<'_> {
            fn from(value: BitmapUpdateIsAfterBlockNumber) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BitmapUpdateIsAfterBlockNumber {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BitmapUpdateIsAfterBlockNumber {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BitmapUpdateIsAfterBlockNumber()";
            const SELECTOR: [u8; 4] = [108u8, 177u8, 154u8, 255u8];
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
    /**Custom error with signature `BitmapValueTooLarge()` and selector `0xca957333`.
    ```solidity
    error BitmapValueTooLarge();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BitmapValueTooLarge {}
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<BitmapValueTooLarge> for UnderlyingRustTuple<'_> {
            fn from(value: BitmapValueTooLarge) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BitmapValueTooLarge {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BitmapValueTooLarge {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BitmapValueTooLarge()";
            const SELECTOR: [u8; 4] = [202u8, 149u8, 115u8, 51u8];
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
    /**Custom error with signature `BytesArrayLengthTooLong()` and selector `0xfb4a9c8e`.
    ```solidity
    error BytesArrayLengthTooLong();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BytesArrayLengthTooLong {}
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<BytesArrayLengthTooLong> for UnderlyingRustTuple<'_> {
            fn from(value: BytesArrayLengthTooLong) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BytesArrayLengthTooLong {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BytesArrayLengthTooLong {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BytesArrayLengthTooLong()";
            const SELECTOR: [u8; 4] = [251u8, 74u8, 156u8, 142u8];
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
    /**Custom error with signature `BytesArrayNotOrdered()` and selector `0x80c88348`.
    ```solidity
    error BytesArrayNotOrdered();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BytesArrayNotOrdered {}
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<BytesArrayNotOrdered> for UnderlyingRustTuple<'_> {
            fn from(value: BytesArrayNotOrdered) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BytesArrayNotOrdered {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BytesArrayNotOrdered {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BytesArrayNotOrdered()";
            const SELECTOR: [u8; 4] = [128u8, 200u8, 131u8, 72u8];
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
    /**Custom error with signature `CannotChurnSelf()` and selector `0xac2d1682`.
    ```solidity
    error CannotChurnSelf();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CannotChurnSelf {}
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<CannotChurnSelf> for UnderlyingRustTuple<'_> {
            fn from(value: CannotChurnSelf) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CannotChurnSelf {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CannotChurnSelf {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CannotChurnSelf()";
            const SELECTOR: [u8; 4] = [172u8, 45u8, 22u8, 130u8];
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
    /**Custom error with signature `CannotKickOperatorAboveThreshold()` and selector `0xb187e869`.
    ```solidity
    error CannotKickOperatorAboveThreshold();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CannotKickOperatorAboveThreshold {}
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<CannotKickOperatorAboveThreshold> for UnderlyingRustTuple<'_> {
            fn from(value: CannotKickOperatorAboveThreshold) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CannotKickOperatorAboveThreshold {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CannotKickOperatorAboveThreshold {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CannotKickOperatorAboveThreshold()";
            const SELECTOR: [u8; 4] = [177u8, 135u8, 232u8, 105u8];
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
    /**Custom error with signature `CannotReregisterYet()` and selector `0x32d0cefa`.
    ```solidity
    error CannotReregisterYet();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CannotReregisterYet {}
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<CannotReregisterYet> for UnderlyingRustTuple<'_> {
            fn from(value: CannotReregisterYet) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CannotReregisterYet {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CannotReregisterYet {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CannotReregisterYet()";
            const SELECTOR: [u8; 4] = [50u8, 208u8, 206u8, 250u8];
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
    /**Custom error with signature `ChurnApproverSaltUsed()` and selector `0xdf7dfd86`.
    ```solidity
    error ChurnApproverSaltUsed();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ChurnApproverSaltUsed {}
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ChurnApproverSaltUsed> for UnderlyingRustTuple<'_> {
            fn from(value: ChurnApproverSaltUsed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ChurnApproverSaltUsed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ChurnApproverSaltUsed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ChurnApproverSaltUsed()";
            const SELECTOR: [u8; 4] = [223u8, 125u8, 253u8, 134u8];
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
    /**Custom error with signature `CurrentlyPaused()` and selector `0x840a48d5`.
    ```solidity
    error CurrentlyPaused();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CurrentlyPaused {}
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<CurrentlyPaused> for UnderlyingRustTuple<'_> {
            fn from(value: CurrentlyPaused) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CurrentlyPaused {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CurrentlyPaused {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CurrentlyPaused()";
            const SELECTOR: [u8; 4] = [132u8, 10u8, 72u8, 213u8];
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
    /**Custom error with signature `ExpModFailed()` and selector `0xd51edae3`.
    ```solidity
    error ExpModFailed();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ExpModFailed {}
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ExpModFailed> for UnderlyingRustTuple<'_> {
            fn from(value: ExpModFailed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ExpModFailed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ExpModFailed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ExpModFailed()";
            const SELECTOR: [u8; 4] = [213u8, 30u8, 218u8, 227u8];
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
    /**Custom error with signature `InputAddressZero()` and selector `0x73632176`.
    ```solidity
    error InputAddressZero();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InputAddressZero {}
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InputAddressZero> for UnderlyingRustTuple<'_> {
            fn from(value: InputAddressZero) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InputAddressZero {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InputAddressZero {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InputAddressZero()";
            const SELECTOR: [u8; 4] = [115u8, 99u8, 33u8, 118u8];
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
    /**Custom error with signature `InputLengthMismatch()` and selector `0xaaad13f7`.
    ```solidity
    error InputLengthMismatch();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InputLengthMismatch {}
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InputLengthMismatch> for UnderlyingRustTuple<'_> {
            fn from(value: InputLengthMismatch) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InputLengthMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InputLengthMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InputLengthMismatch()";
            const SELECTOR: [u8; 4] = [170u8, 173u8, 19u8, 247u8];
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
    /**Custom error with signature `InsufficientStakeForChurn()` and selector `0x4c44995d`.
    ```solidity
    error InsufficientStakeForChurn();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientStakeForChurn {}
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InsufficientStakeForChurn> for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientStakeForChurn) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InsufficientStakeForChurn {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientStakeForChurn {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientStakeForChurn()";
            const SELECTOR: [u8; 4] = [76u8, 68u8, 153u8, 93u8];
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
    /**Custom error with signature `InvalidAVS()` and selector `0x66e565df`.
    ```solidity
    error InvalidAVS();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidAVS {}
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidAVS> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidAVS) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidAVS {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidAVS {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidAVS()";
            const SELECTOR: [u8; 4] = [102u8, 229u8, 101u8, 223u8];
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
    /**Custom error with signature `InvalidNewPausedStatus()` and selector `0xc61dca5d`.
    ```solidity
    error InvalidNewPausedStatus();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidNewPausedStatus {}
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidNewPausedStatus> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidNewPausedStatus) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidNewPausedStatus {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidNewPausedStatus {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidNewPausedStatus()";
            const SELECTOR: [u8; 4] = [198u8, 29u8, 202u8, 93u8];
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
    /**Custom error with signature `InvalidRegistrationType()` and selector `0x354bb8ab`.
    ```solidity
    error InvalidRegistrationType();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidRegistrationType {}
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidRegistrationType> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidRegistrationType) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidRegistrationType {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidRegistrationType {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidRegistrationType()";
            const SELECTOR: [u8; 4] = [53u8, 75u8, 184u8, 171u8];
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
    /**Custom error with signature `InvalidShortString()` and selector `0xb3512b0c`.
    ```solidity
    error InvalidShortString();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidShortString {}
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidShortString> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidShortString) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidShortString {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidShortString {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidShortString()";
            const SELECTOR: [u8; 4] = [179u8, 81u8, 43u8, 12u8];
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
    /**Custom error with signature `InvalidSignature()` and selector `0x8baa579f`.
    ```solidity
    error InvalidSignature();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidSignature {}
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidSignature> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidSignature) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidSignature()";
            const SELECTOR: [u8; 4] = [139u8, 170u8, 87u8, 159u8];
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
    /**Custom error with signature `LookAheadPeriodTooLong()` and selector `0x2f5106e4`.
    ```solidity
    error LookAheadPeriodTooLong();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LookAheadPeriodTooLong {}
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<LookAheadPeriodTooLong> for UnderlyingRustTuple<'_> {
            fn from(value: LookAheadPeriodTooLong) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for LookAheadPeriodTooLong {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LookAheadPeriodTooLong {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LookAheadPeriodTooLong()";
            const SELECTOR: [u8; 4] = [47u8, 81u8, 6u8, 228u8];
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
    /**Custom error with signature `MaxOperatorCountReached()` and selector `0xc6b9e767`.
    ```solidity
    error MaxOperatorCountReached();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MaxOperatorCountReached {}
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<MaxOperatorCountReached> for UnderlyingRustTuple<'_> {
            fn from(value: MaxOperatorCountReached) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MaxOperatorCountReached {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MaxOperatorCountReached {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MaxOperatorCountReached()";
            const SELECTOR: [u8; 4] = [198u8, 185u8, 231u8, 103u8];
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
    /**Custom error with signature `MaxQuorumsReached()` and selector `0x3cb89c97`.
    ```solidity
    error MaxQuorumsReached();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MaxQuorumsReached {}
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<MaxQuorumsReached> for UnderlyingRustTuple<'_> {
            fn from(value: MaxQuorumsReached) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MaxQuorumsReached {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MaxQuorumsReached {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MaxQuorumsReached()";
            const SELECTOR: [u8; 4] = [60u8, 184u8, 156u8, 151u8];
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
    /**Custom error with signature `NextBitmapUpdateIsBeforeBlockNumber()` and selector `0xbbba60cb`.
    ```solidity
    error NextBitmapUpdateIsBeforeBlockNumber();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NextBitmapUpdateIsBeforeBlockNumber {}
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NextBitmapUpdateIsBeforeBlockNumber> for UnderlyingRustTuple<'_> {
            fn from(value: NextBitmapUpdateIsBeforeBlockNumber) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NextBitmapUpdateIsBeforeBlockNumber {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NextBitmapUpdateIsBeforeBlockNumber {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NextBitmapUpdateIsBeforeBlockNumber()";
            const SELECTOR: [u8; 4] = [187u8, 186u8, 96u8, 203u8];
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
    /**Custom error with signature `NotRegistered()` and selector `0xaba47339`.
    ```solidity
    error NotRegistered();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotRegistered {}
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NotRegistered> for UnderlyingRustTuple<'_> {
            fn from(value: NotRegistered) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotRegistered {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotRegistered {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotRegistered()";
            const SELECTOR: [u8; 4] = [171u8, 164u8, 115u8, 57u8];
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
    /**Custom error with signature `NotRegisteredForQuorum()` and selector `0xd053aa21`.
    ```solidity
    error NotRegisteredForQuorum();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotRegisteredForQuorum {}
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NotRegisteredForQuorum> for UnderlyingRustTuple<'_> {
            fn from(value: NotRegisteredForQuorum) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotRegisteredForQuorum {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotRegisteredForQuorum {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotRegisteredForQuorum()";
            const SELECTOR: [u8; 4] = [208u8, 83u8, 170u8, 33u8];
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
    /**Custom error with signature `NotSorted()` and selector `0xba50f911`.
    ```solidity
    error NotSorted();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotSorted {}
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NotSorted> for UnderlyingRustTuple<'_> {
            fn from(value: NotSorted) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotSorted {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotSorted {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotSorted()";
            const SELECTOR: [u8; 4] = [186u8, 80u8, 249u8, 17u8];
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
    /**Custom error with signature `OnlyAllocationManager()` and selector `0x23d871a5`.
    ```solidity
    error OnlyAllocationManager();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyAllocationManager {}
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OnlyAllocationManager> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyAllocationManager) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyAllocationManager {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyAllocationManager {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyAllocationManager()";
            const SELECTOR: [u8; 4] = [35u8, 216u8, 113u8, 165u8];
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
    /**Custom error with signature `OnlyEjector()` and selector `0xedb1562e`.
    ```solidity
    error OnlyEjector();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyEjector {}
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OnlyEjector> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyEjector) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyEjector {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyEjector {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyEjector()";
            const SELECTOR: [u8; 4] = [237u8, 177u8, 86u8, 46u8];
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
    /**Custom error with signature `OnlyPauser()` and selector `0x75df51dc`.
    ```solidity
    error OnlyPauser();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyPauser {}
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OnlyPauser> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyPauser) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyPauser {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyPauser {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyPauser()";
            const SELECTOR: [u8; 4] = [117u8, 223u8, 81u8, 220u8];
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
    /**Custom error with signature `OnlyUnpauser()` and selector `0x794821ff`.
    ```solidity
    error OnlyUnpauser();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyUnpauser {}
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OnlyUnpauser> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyUnpauser) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyUnpauser {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyUnpauser {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyUnpauser()";
            const SELECTOR: [u8; 4] = [121u8, 72u8, 33u8, 255u8];
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
    /**Custom error with signature `OperatorNotRegistered()` and selector `0x25ec6c1f`.
    ```solidity
    error OperatorNotRegistered();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorNotRegistered {}
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OperatorNotRegistered> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorNotRegistered) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorNotRegistered {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OperatorNotRegistered {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OperatorNotRegistered()";
            const SELECTOR: [u8; 4] = [37u8, 236u8, 108u8, 31u8];
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
    /**Custom error with signature `QuorumDoesNotExist()` and selector `0xe6219fea`.
    ```solidity
    error QuorumDoesNotExist();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct QuorumDoesNotExist {}
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<QuorumDoesNotExist> for UnderlyingRustTuple<'_> {
            fn from(value: QuorumDoesNotExist) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for QuorumDoesNotExist {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for QuorumDoesNotExist {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "QuorumDoesNotExist()";
            const SELECTOR: [u8; 4] = [230u8, 33u8, 159u8, 234u8];
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
    /**Custom error with signature `QuorumOperatorCountMismatch()` and selector `0x8e5aeee7`.
    ```solidity
    error QuorumOperatorCountMismatch();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct QuorumOperatorCountMismatch {}
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<QuorumOperatorCountMismatch> for UnderlyingRustTuple<'_> {
            fn from(value: QuorumOperatorCountMismatch) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for QuorumOperatorCountMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for QuorumOperatorCountMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "QuorumOperatorCountMismatch()";
            const SELECTOR: [u8; 4] = [142u8, 90u8, 238u8, 231u8];
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
    /**Custom error with signature `SignatureExpired()` and selector `0x0819bdcd`.
    ```solidity
    error SignatureExpired();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SignatureExpired {}
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<SignatureExpired> for UnderlyingRustTuple<'_> {
            fn from(value: SignatureExpired) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SignatureExpired {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SignatureExpired {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SignatureExpired()";
            const SELECTOR: [u8; 4] = [8u8, 25u8, 189u8, 205u8];
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
    /**Custom error with signature `StringTooLong(string)` and selector `0x305a27a9`.
    ```solidity
    error StringTooLong(string str);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StringTooLong {
        #[allow(missing_docs)]
        pub str: alloy::sol_types::private::String,
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
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<StringTooLong> for UnderlyingRustTuple<'_> {
            fn from(value: StringTooLong) -> Self {
                (value.str,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StringTooLong {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { str: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for StringTooLong {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "StringTooLong(string)";
            const SELECTOR: [u8; 4] = [48u8, 90u8, 39u8, 169u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.str,
                    ),
                )
            }
        }
    };
    /**Event with signature `AVSUpdated(address,address)` and selector `0x9770f3cadfdcbb6f93af935e86047111590c3768271d237e4a2bc0b874bed693`.
    ```solidity
    event AVSUpdated(address prevAVS, address newAVS);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct AVSUpdated {
        #[allow(missing_docs)]
        pub prevAVS: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newAVS: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for AVSUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "AVSUpdated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    151u8, 112u8, 243u8, 202u8, 223u8, 220u8, 187u8, 111u8, 147u8, 175u8, 147u8,
                    94u8, 134u8, 4u8, 113u8, 17u8, 89u8, 12u8, 55u8, 104u8, 39u8, 29u8, 35u8,
                    126u8, 74u8, 43u8, 192u8, 184u8, 116u8, 190u8, 214u8, 147u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    prevAVS: data.0,
                    newAVS: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.prevAVS,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newAVS,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for AVSUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&AVSUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &AVSUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ChurnApproverUpdated(address,address)` and selector `0x315457d8a8fe60f04af17c16e2f5a5e1db612b31648e58030360759ef8f3528c`.
    ```solidity
    event ChurnApproverUpdated(address prevChurnApprover, address newChurnApprover);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ChurnApproverUpdated {
        #[allow(missing_docs)]
        pub prevChurnApprover: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newChurnApprover: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for ChurnApproverUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ChurnApproverUpdated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    49u8, 84u8, 87u8, 216u8, 168u8, 254u8, 96u8, 240u8, 74u8, 241u8, 124u8, 22u8,
                    226u8, 245u8, 165u8, 225u8, 219u8, 97u8, 43u8, 49u8, 100u8, 142u8, 88u8, 3u8,
                    3u8, 96u8, 117u8, 158u8, 248u8, 243u8, 82u8, 140u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    prevChurnApprover: data.0,
                    newChurnApprover: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.prevChurnApprover,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newChurnApprover,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ChurnApproverUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ChurnApproverUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ChurnApproverUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "EIP712DomainChanged()";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    10u8, 99u8, 135u8, 201u8, 234u8, 54u8, 40u8, 184u8, 138u8, 99u8, 59u8, 180u8,
                    243u8, 177u8, 81u8, 119u8, 15u8, 112u8, 8u8, 81u8, 23u8, 161u8, 95u8, 155u8,
                    243u8, 120u8, 124u8, 218u8, 83u8, 241u8, 61u8, 49u8,
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
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
    /**Event with signature `EjectionCooldownUpdated(uint256,uint256)` and selector `0xa77a91bea7b6d95a8eb5a54878a1d9e3c875e26c86a9b70e3420c5c5db193b62`.
    ```solidity
    event EjectionCooldownUpdated(uint256 prevEjectionCooldown, uint256 newEjectionCooldown);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct EjectionCooldownUpdated {
        #[allow(missing_docs)]
        pub prevEjectionCooldown: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newEjectionCooldown: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for EjectionCooldownUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "EjectionCooldownUpdated(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    167u8, 122u8, 145u8, 190u8, 167u8, 182u8, 217u8, 90u8, 142u8, 181u8, 165u8,
                    72u8, 120u8, 161u8, 217u8, 227u8, 200u8, 117u8, 226u8, 108u8, 134u8, 169u8,
                    183u8, 14u8, 52u8, 32u8, 197u8, 197u8, 219u8, 25u8, 59u8, 98u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    prevEjectionCooldown: data.0,
                    newEjectionCooldown: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.prevEjectionCooldown,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.newEjectionCooldown,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for EjectionCooldownUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&EjectionCooldownUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &EjectionCooldownUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `EjectorUpdated(address,address)` and selector `0x8f30ab09f43a6c157d7fce7e0a13c003042c1c95e8a72e7a146a21c0caa24dc9`.
    ```solidity
    event EjectorUpdated(address prevEjector, address newEjector);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct EjectorUpdated {
        #[allow(missing_docs)]
        pub prevEjector: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newEjector: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for EjectorUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "EjectorUpdated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    143u8, 48u8, 171u8, 9u8, 244u8, 58u8, 108u8, 21u8, 125u8, 127u8, 206u8, 126u8,
                    10u8, 19u8, 192u8, 3u8, 4u8, 44u8, 28u8, 149u8, 232u8, 167u8, 46u8, 122u8,
                    20u8, 106u8, 33u8, 192u8, 202u8, 162u8, 77u8, 201u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    prevEjector: data.0,
                    newEjector: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.prevEjector,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newEjector,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for EjectorUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&EjectorUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &EjectorUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Initialized(uint8)` and selector `0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498`.
    ```solidity
    event Initialized(uint8 version);
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
        pub version: u8,
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
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Initialized(uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    127u8, 38u8, 184u8, 63u8, 249u8, 110u8, 31u8, 43u8, 106u8, 104u8, 47u8, 19u8,
                    56u8, 82u8, 246u8, 121u8, 138u8, 9u8, 196u8, 101u8, 218u8, 149u8, 146u8, 20u8,
                    96u8, 206u8, 251u8, 56u8, 71u8, 64u8, 36u8, 152u8,
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
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self.version,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
    /**Event with signature `OperatorDeregistered(address,bytes32)` and selector `0x396fdcb180cb0fea26928113fb0fd1c3549863f9cd563e6a184f1d578116c8e4`.
    ```solidity
    event OperatorDeregistered(address indexed operator, bytes32 indexed operatorId);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorDeregistered {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for OperatorDeregistered {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "OperatorDeregistered(address,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    57u8, 111u8, 220u8, 177u8, 128u8, 203u8, 15u8, 234u8, 38u8, 146u8, 129u8, 19u8,
                    251u8, 15u8, 209u8, 195u8, 84u8, 152u8, 99u8, 249u8, 205u8, 86u8, 62u8, 106u8,
                    24u8, 79u8, 29u8, 87u8, 129u8, 22u8, 200u8, 228u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    operator: topics.1,
                    operatorId: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
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
                    self.operator.clone(),
                    self.operatorId.clone(),
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                out[2usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.operatorId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorDeregistered {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorDeregistered> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorDeregistered) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorRegistered(address,bytes32)` and selector `0xe8e68cef1c3a761ed7be7e8463a375f27f7bc335e51824223cacce636ec5c3fe`.
    ```solidity
    event OperatorRegistered(address indexed operator, bytes32 indexed operatorId);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorRegistered {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for OperatorRegistered {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "OperatorRegistered(address,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    232u8, 230u8, 140u8, 239u8, 28u8, 58u8, 118u8, 30u8, 215u8, 190u8, 126u8,
                    132u8, 99u8, 163u8, 117u8, 242u8, 127u8, 123u8, 195u8, 53u8, 229u8, 24u8, 36u8,
                    34u8, 60u8, 172u8, 206u8, 99u8, 110u8, 197u8, 195u8, 254u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    operator: topics.1,
                    operatorId: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
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
                    self.operator.clone(),
                    self.operatorId.clone(),
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                out[2usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.operatorId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorRegistered {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorRegistered> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorRegistered) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorSetParamsUpdated(uint8,(uint32,uint16,uint16))` and selector `0x3ee6fe8d54610244c3e9d3c066ae4aee997884aa28f10616ae821925401318ac`.
    ```solidity
    event OperatorSetParamsUpdated(uint8 indexed quorumNumber, ISlashingRegistryCoordinatorTypes.OperatorSetParam operatorSetParams);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorSetParamsUpdated {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub operatorSetParams: <ISlashingRegistryCoordinatorTypes::OperatorSetParam as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for OperatorSetParamsUpdated {
            type DataTuple<'a> = (ISlashingRegistryCoordinatorTypes::OperatorSetParam,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            const SIGNATURE: &'static str =
                "OperatorSetParamsUpdated(uint8,(uint32,uint16,uint16))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    62u8, 230u8, 254u8, 141u8, 84u8, 97u8, 2u8, 68u8, 195u8, 233u8, 211u8, 192u8,
                    102u8, 174u8, 74u8, 238u8, 153u8, 120u8, 132u8, 170u8, 40u8, 241u8, 6u8, 22u8,
                    174u8, 130u8, 25u8, 37u8, 64u8, 19u8, 24u8, 172u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    quorumNumber: topics.1,
                    operatorSetParams: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <ISlashingRegistryCoordinatorTypes::OperatorSetParam as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSetParams,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.quorumNumber.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.quorumNumber);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorSetParamsUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorSetParamsUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorSetParamsUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorSocketUpdate(bytes32,string)` and selector `0xec2963ab21c1e50e1e582aa542af2e4bf7bf38e6e1403c27b42e1c5d6e621eaa`.
    ```solidity
    event OperatorSocketUpdate(bytes32 indexed operatorId, string socket);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorSocketUpdate {
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub socket: alloy::sol_types::private::String,
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
        impl alloy_sol_types::SolEvent for OperatorSocketUpdate {
            type DataTuple<'a> = (alloy::sol_types::sol_data::String,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "OperatorSocketUpdate(bytes32,string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    236u8, 41u8, 99u8, 171u8, 33u8, 193u8, 229u8, 14u8, 30u8, 88u8, 42u8, 165u8,
                    66u8, 175u8, 46u8, 75u8, 247u8, 191u8, 56u8, 230u8, 225u8, 64u8, 60u8, 39u8,
                    180u8, 46u8, 28u8, 93u8, 110u8, 98u8, 30u8, 170u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    operatorId: topics.1,
                    socket: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.socket,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.operatorId.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.operatorId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorSocketUpdate {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorSocketUpdate> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorSocketUpdate) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OwnershipTransferred(address,address)` and selector `0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0`.
    ```solidity
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OwnershipTransferred {
        #[allow(missing_docs)]
        pub previousOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for OwnershipTransferred {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferred(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8, 31u8,
                    208u8, 164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8, 218u8,
                    175u8, 227u8, 180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    previousOwner: topics.1,
                    newOwner: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
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
                    self.previousOwner.clone(),
                    self.newOwner.clone(),
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.previousOwner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newOwner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OwnershipTransferred {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OwnershipTransferred> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OwnershipTransferred) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Paused(address,uint256)` and selector `0xab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d`.
    ```solidity
    event Paused(address indexed account, uint256 newPausedStatus);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Paused {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for Paused {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Paused(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    171u8, 64u8, 163u8, 116u8, 188u8, 81u8, 222u8, 55u8, 34u8, 0u8, 168u8, 188u8,
                    152u8, 26u8, 248u8, 201u8, 236u8, 220u8, 8u8, 223u8, 218u8, 239u8, 11u8, 182u8,
                    224u8, 159u8, 136u8, 243u8, 198u8, 22u8, 239u8, 61u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    account: topics.1,
                    newPausedStatus: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.newPausedStatus,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.account.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.account,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Paused {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Paused> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Paused) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `QuorumBlockNumberUpdated(uint8,uint256)` and selector `0x46077d55330763f16269fd75e5761663f4192d2791747c0189b16ad31db07db4`.
    ```solidity
    event QuorumBlockNumberUpdated(uint8 indexed quorumNumber, uint256 blocknumber);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct QuorumBlockNumberUpdated {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub blocknumber: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for QuorumBlockNumberUpdated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            const SIGNATURE: &'static str = "QuorumBlockNumberUpdated(uint8,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    70u8, 7u8, 125u8, 85u8, 51u8, 7u8, 99u8, 241u8, 98u8, 105u8, 253u8, 117u8,
                    229u8, 118u8, 22u8, 99u8, 244u8, 25u8, 45u8, 39u8, 145u8, 116u8, 124u8, 1u8,
                    137u8, 177u8, 106u8, 211u8, 29u8, 176u8, 125u8, 180u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    quorumNumber: topics.1,
                    blocknumber: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.blocknumber,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.quorumNumber.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.quorumNumber);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for QuorumBlockNumberUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&QuorumBlockNumberUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &QuorumBlockNumberUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `QuorumCreated(uint8,(uint32,uint16,uint16),uint96,(address,uint96)[],uint8,uint32)` and selector `0xa34835bc2b673ec37fcf1591a91295b163fc2e181e4ea4e733beb27de1ceac4c`.
    ```solidity
    event QuorumCreated(uint8 indexed quorumNumber, ISlashingRegistryCoordinatorTypes.OperatorSetParam operatorSetParams, uint96 minimumStake, IStakeRegistryTypes.StrategyParams[] strategyParams, IStakeRegistryTypes.StakeType stakeType, uint32 lookAheadPeriod);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct QuorumCreated {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub operatorSetParams: <ISlashingRegistryCoordinatorTypes::OperatorSetParam as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub minimumStake: alloy::sol_types::private::primitives::aliases::U96,
        #[allow(missing_docs)]
        pub strategyParams: alloy::sol_types::private::Vec<
            <IStakeRegistryTypes::StrategyParams as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub stakeType: <IStakeRegistryTypes::StakeType as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub lookAheadPeriod: u32,
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
        impl alloy_sol_types::SolEvent for QuorumCreated {
            type DataTuple<'a> = (
                ISlashingRegistryCoordinatorTypes::OperatorSetParam,
                alloy::sol_types::sol_data::Uint<96>,
                alloy::sol_types::sol_data::Array<IStakeRegistryTypes::StrategyParams>,
                IStakeRegistryTypes::StakeType,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            const SIGNATURE: &'static str = "QuorumCreated(uint8,(uint32,uint16,uint16),uint96,(address,uint96)[],uint8,uint32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    163u8, 72u8, 53u8, 188u8, 43u8, 103u8, 62u8, 195u8, 127u8, 207u8, 21u8, 145u8,
                    169u8, 18u8, 149u8, 177u8, 99u8, 252u8, 46u8, 24u8, 30u8, 78u8, 164u8, 231u8,
                    51u8, 190u8, 178u8, 125u8, 225u8, 206u8, 172u8, 76u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    quorumNumber: topics.1,
                    operatorSetParams: data.0,
                    minimumStake: data.1,
                    strategyParams: data.2,
                    stakeType: data.3,
                    lookAheadPeriod: data.4,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <ISlashingRegistryCoordinatorTypes::OperatorSetParam as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSetParams,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.minimumStake),
                    <alloy::sol_types::sol_data::Array<
                        IStakeRegistryTypes::StrategyParams,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategyParams),
                    <IStakeRegistryTypes::StakeType as alloy_sol_types::SolType>::tokenize(
                        &self.stakeType,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.lookAheadPeriod),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.quorumNumber.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.quorumNumber);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for QuorumCreated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&QuorumCreated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &QuorumCreated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Unpaused(address,uint256)` and selector `0x3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c`.
    ```solidity
    event Unpaused(address indexed account, uint256 newPausedStatus);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Unpaused {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for Unpaused {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Unpaused(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    53u8, 130u8, 209u8, 130u8, 142u8, 38u8, 191u8, 86u8, 189u8, 128u8, 21u8, 2u8,
                    188u8, 2u8, 26u8, 192u8, 188u8, 138u8, 251u8, 87u8, 200u8, 38u8, 228u8, 152u8,
                    107u8, 69u8, 89u8, 60u8, 143u8, 173u8, 56u8, 156u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    account: topics.1,
                    newPausedStatus: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.newPausedStatus,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.account.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.account,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Unpaused {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Unpaused> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Unpaused) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
    ```solidity
    constructor(address _stakeRegistry, address _blsApkRegistry, address _indexRegistry, address _socketRegistry, address _allocationManager, address _pauserRegistry, string _version);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _stakeRegistry: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _blsApkRegistry: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _indexRegistry: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _socketRegistry: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _allocationManager: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _pauserRegistry: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _version: alloy::sol_types::private::String,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::String,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::String,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                        value._stakeRegistry,
                        value._blsApkRegistry,
                        value._indexRegistry,
                        value._socketRegistry,
                        value._allocationManager,
                        value._pauserRegistry,
                        value._version,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _stakeRegistry: tuple.0,
                        _blsApkRegistry: tuple.1,
                        _indexRegistry: tuple.2,
                        _socketRegistry: tuple.3,
                        _allocationManager: tuple.4,
                        _pauserRegistry: tuple.5,
                        _version: tuple.6,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::String,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                        &self._stakeRegistry,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._blsApkRegistry,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._indexRegistry,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._socketRegistry,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._allocationManager,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._pauserRegistry,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self._version,
                    ),
                )
            }
        }
    };
    /**Function with signature `OPERATOR_CHURN_APPROVAL_TYPEHASH()` and selector `0xca0de882`.
    ```solidity
    function OPERATOR_CHURN_APPROVAL_TYPEHASH() external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OPERATOR_CHURN_APPROVAL_TYPEHASHCall {}
    ///Container type for the return parameters of the [`OPERATOR_CHURN_APPROVAL_TYPEHASH()`](OPERATOR_CHURN_APPROVAL_TYPEHASHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OPERATOR_CHURN_APPROVAL_TYPEHASHReturn {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<OPERATOR_CHURN_APPROVAL_TYPEHASHCall> for UnderlyingRustTuple<'_> {
                fn from(value: OPERATOR_CHURN_APPROVAL_TYPEHASHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for OPERATOR_CHURN_APPROVAL_TYPEHASHCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<OPERATOR_CHURN_APPROVAL_TYPEHASHReturn> for UnderlyingRustTuple<'_> {
                fn from(value: OPERATOR_CHURN_APPROVAL_TYPEHASHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for OPERATOR_CHURN_APPROVAL_TYPEHASHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for OPERATOR_CHURN_APPROVAL_TYPEHASHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = OPERATOR_CHURN_APPROVAL_TYPEHASHReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OPERATOR_CHURN_APPROVAL_TYPEHASH()";
            const SELECTOR: [u8; 4] = [202u8, 13u8, 232u8, 130u8];
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `PUBKEY_REGISTRATION_TYPEHASH()` and selector `0x9feab859`.
    ```solidity
    function PUBKEY_REGISTRATION_TYPEHASH() external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PUBKEY_REGISTRATION_TYPEHASHCall {}
    ///Container type for the return parameters of the [`PUBKEY_REGISTRATION_TYPEHASH()`](PUBKEY_REGISTRATION_TYPEHASHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PUBKEY_REGISTRATION_TYPEHASHReturn {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<PUBKEY_REGISTRATION_TYPEHASHCall> for UnderlyingRustTuple<'_> {
                fn from(value: PUBKEY_REGISTRATION_TYPEHASHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for PUBKEY_REGISTRATION_TYPEHASHCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<PUBKEY_REGISTRATION_TYPEHASHReturn> for UnderlyingRustTuple<'_> {
                fn from(value: PUBKEY_REGISTRATION_TYPEHASHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for PUBKEY_REGISTRATION_TYPEHASHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for PUBKEY_REGISTRATION_TYPEHASHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = PUBKEY_REGISTRATION_TYPEHASHReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PUBKEY_REGISTRATION_TYPEHASH()";
            const SELECTOR: [u8; 4] = [159u8, 234u8, 184u8, 89u8];
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<allocationManagerCall> for UnderlyingRustTuple<'_> {
                fn from(value: allocationManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allocationManagerCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<allocationManagerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: allocationManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allocationManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for allocationManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = allocationManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `avs()` and selector `0xde1164bb`.
    ```solidity
    function avs() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct avsCall {}
    ///Container type for the return parameters of the [`avs()`](avsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct avsReturn {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<avsCall> for UnderlyingRustTuple<'_> {
                fn from(value: avsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for avsCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<avsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: avsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for avsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for avsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = avsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "avs()";
            const SELECTOR: [u8; 4] = [222u8, 17u8, 100u8, 187u8];
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `blsApkRegistry()` and selector `0x5df45946`.
    ```solidity
    function blsApkRegistry() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct blsApkRegistryCall {}
    ///Container type for the return parameters of the [`blsApkRegistry()`](blsApkRegistryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct blsApkRegistryReturn {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<blsApkRegistryCall> for UnderlyingRustTuple<'_> {
                fn from(value: blsApkRegistryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for blsApkRegistryCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<blsApkRegistryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: blsApkRegistryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for blsApkRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for blsApkRegistryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = blsApkRegistryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "blsApkRegistry()";
            const SELECTOR: [u8; 4] = [93u8, 244u8, 89u8, 70u8];
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `calculateOperatorChurnApprovalDigestHash(address,bytes32,(uint8,address)[],bytes32,uint256)` and selector `0x84ca5213`.
    ```solidity
    function calculateOperatorChurnApprovalDigestHash(address registeringOperator, bytes32 registeringOperatorId, ISlashingRegistryCoordinatorTypes.OperatorKickParam[] memory operatorKickParams, bytes32 salt, uint256 expiry) external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateOperatorChurnApprovalDigestHashCall {
        #[allow(missing_docs)]
        pub registeringOperator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub registeringOperatorId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub operatorKickParams: alloy::sol_types::private::Vec<
            <ISlashingRegistryCoordinatorTypes::OperatorKickParam as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub salt: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub expiry: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`calculateOperatorChurnApprovalDigestHash(address,bytes32,(uint8,address)[],bytes32,uint256)`](calculateOperatorChurnApprovalDigestHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateOperatorChurnApprovalDigestHashReturn {
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
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<
                    ISlashingRegistryCoordinatorTypes::OperatorKickParam,
                >,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Vec<
                    <ISlashingRegistryCoordinatorTypes::OperatorKickParam as alloy::sol_types::SolType>::RustType,
                >,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<calculateOperatorChurnApprovalDigestHashCall>
                for UnderlyingRustTuple<'_>
            {
                fn from(value: calculateOperatorChurnApprovalDigestHashCall) -> Self {
                    (
                        value.registeringOperator,
                        value.registeringOperatorId,
                        value.operatorKickParams,
                        value.salt,
                        value.expiry,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
                for calculateOperatorChurnApprovalDigestHashCall
            {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        registeringOperator: tuple.0,
                        registeringOperatorId: tuple.1,
                        operatorKickParams: tuple.2,
                        salt: tuple.3,
                        expiry: tuple.4,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<calculateOperatorChurnApprovalDigestHashReturn>
                for UnderlyingRustTuple<'_>
            {
                fn from(value: calculateOperatorChurnApprovalDigestHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
                for calculateOperatorChurnApprovalDigestHashReturn
            {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for calculateOperatorChurnApprovalDigestHashCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<
                    ISlashingRegistryCoordinatorTypes::OperatorKickParam,
                >,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = calculateOperatorChurnApprovalDigestHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "calculateOperatorChurnApprovalDigestHash(address,bytes32,(uint8,address)[],bytes32,uint256)";
            const SELECTOR: [u8; 4] = [132u8, 202u8, 82u8, 19u8];
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
                        &self.registeringOperator,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.registeringOperatorId,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        ISlashingRegistryCoordinatorTypes::OperatorKickParam,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorKickParams),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.expiry),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `calculatePubkeyRegistrationMessageHash(address)` and selector `0x73447992`.
    ```solidity
    function calculatePubkeyRegistrationMessageHash(address operator) external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculatePubkeyRegistrationMessageHashCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`calculatePubkeyRegistrationMessageHash(address)`](calculatePubkeyRegistrationMessageHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculatePubkeyRegistrationMessageHashReturn {
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<calculatePubkeyRegistrationMessageHashCall> for UnderlyingRustTuple<'_> {
                fn from(value: calculatePubkeyRegistrationMessageHashCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for calculatePubkeyRegistrationMessageHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<calculatePubkeyRegistrationMessageHashReturn>
                for UnderlyingRustTuple<'_>
            {
                fn from(value: calculatePubkeyRegistrationMessageHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
                for calculatePubkeyRegistrationMessageHashReturn
            {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for calculatePubkeyRegistrationMessageHashCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = calculatePubkeyRegistrationMessageHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "calculatePubkeyRegistrationMessageHash(address)";
            const SELECTOR: [u8; 4] = [115u8, 68u8, 121u8, 146u8];
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
                        &self.operator,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `churnApprover()` and selector `0x054310e6`.
    ```solidity
    function churnApprover() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct churnApproverCall {}
    ///Container type for the return parameters of the [`churnApprover()`](churnApproverCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct churnApproverReturn {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<churnApproverCall> for UnderlyingRustTuple<'_> {
                fn from(value: churnApproverCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for churnApproverCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<churnApproverReturn> for UnderlyingRustTuple<'_> {
                fn from(value: churnApproverReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for churnApproverReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for churnApproverCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = churnApproverReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "churnApprover()";
            const SELECTOR: [u8; 4] = [5u8, 67u8, 16u8, 230u8];
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `createSlashableStakeQuorum((uint32,uint16,uint16),uint96,(address,uint96)[],uint32)` and selector `0x3eef3a51`.
    ```solidity
    function createSlashableStakeQuorum(ISlashingRegistryCoordinatorTypes.OperatorSetParam memory operatorSetParams, uint96 minimumStake, IStakeRegistryTypes.StrategyParams[] memory strategyParams, uint32 lookAheadPeriod) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createSlashableStakeQuorumCall {
        #[allow(missing_docs)]
        pub operatorSetParams: <ISlashingRegistryCoordinatorTypes::OperatorSetParam as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub minimumStake: alloy::sol_types::private::primitives::aliases::U96,
        #[allow(missing_docs)]
        pub strategyParams: alloy::sol_types::private::Vec<
            <IStakeRegistryTypes::StrategyParams as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub lookAheadPeriod: u32,
    }
    ///Container type for the return parameters of the [`createSlashableStakeQuorum((uint32,uint16,uint16),uint96,(address,uint96)[],uint32)`](createSlashableStakeQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createSlashableStakeQuorumReturn {}
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
                ISlashingRegistryCoordinatorTypes::OperatorSetParam,
                alloy::sol_types::sol_data::Uint<96>,
                alloy::sol_types::sol_data::Array<IStakeRegistryTypes::StrategyParams>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ISlashingRegistryCoordinatorTypes::OperatorSetParam as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::U96,
                alloy::sol_types::private::Vec<
                    <IStakeRegistryTypes::StrategyParams as alloy::sol_types::SolType>::RustType,
                >,
                u32,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<createSlashableStakeQuorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: createSlashableStakeQuorumCall) -> Self {
                    (
                        value.operatorSetParams,
                        value.minimumStake,
                        value.strategyParams,
                        value.lookAheadPeriod,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createSlashableStakeQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSetParams: tuple.0,
                        minimumStake: tuple.1,
                        strategyParams: tuple.2,
                        lookAheadPeriod: tuple.3,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<createSlashableStakeQuorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: createSlashableStakeQuorumReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createSlashableStakeQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createSlashableStakeQuorumCall {
            type Parameters<'a> = (
                ISlashingRegistryCoordinatorTypes::OperatorSetParam,
                alloy::sol_types::sol_data::Uint<96>,
                alloy::sol_types::sol_data::Array<IStakeRegistryTypes::StrategyParams>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = createSlashableStakeQuorumReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createSlashableStakeQuorum((uint32,uint16,uint16),uint96,(address,uint96)[],uint32)";
            const SELECTOR: [u8; 4] = [62u8, 239u8, 58u8, 81u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <ISlashingRegistryCoordinatorTypes::OperatorSetParam as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSetParams,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.minimumStake),
                    <alloy::sol_types::sol_data::Array<
                        IStakeRegistryTypes::StrategyParams,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategyParams),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.lookAheadPeriod),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `createTotalDelegatedStakeQuorum((uint32,uint16,uint16),uint96,(address,uint96)[])` and selector `0x8281ab75`.
    ```solidity
    function createTotalDelegatedStakeQuorum(ISlashingRegistryCoordinatorTypes.OperatorSetParam memory operatorSetParams, uint96 minimumStake, IStakeRegistryTypes.StrategyParams[] memory strategyParams) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createTotalDelegatedStakeQuorumCall {
        #[allow(missing_docs)]
        pub operatorSetParams: <ISlashingRegistryCoordinatorTypes::OperatorSetParam as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub minimumStake: alloy::sol_types::private::primitives::aliases::U96,
        #[allow(missing_docs)]
        pub strategyParams: alloy::sol_types::private::Vec<
            <IStakeRegistryTypes::StrategyParams as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`createTotalDelegatedStakeQuorum((uint32,uint16,uint16),uint96,(address,uint96)[])`](createTotalDelegatedStakeQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createTotalDelegatedStakeQuorumReturn {}
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
                ISlashingRegistryCoordinatorTypes::OperatorSetParam,
                alloy::sol_types::sol_data::Uint<96>,
                alloy::sol_types::sol_data::Array<IStakeRegistryTypes::StrategyParams>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ISlashingRegistryCoordinatorTypes::OperatorSetParam as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::U96,
                alloy::sol_types::private::Vec<
                    <IStakeRegistryTypes::StrategyParams as alloy::sol_types::SolType>::RustType,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<createTotalDelegatedStakeQuorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: createTotalDelegatedStakeQuorumCall) -> Self {
                    (
                        value.operatorSetParams,
                        value.minimumStake,
                        value.strategyParams,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createTotalDelegatedStakeQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSetParams: tuple.0,
                        minimumStake: tuple.1,
                        strategyParams: tuple.2,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<createTotalDelegatedStakeQuorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: createTotalDelegatedStakeQuorumReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createTotalDelegatedStakeQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createTotalDelegatedStakeQuorumCall {
            type Parameters<'a> = (
                ISlashingRegistryCoordinatorTypes::OperatorSetParam,
                alloy::sol_types::sol_data::Uint<96>,
                alloy::sol_types::sol_data::Array<IStakeRegistryTypes::StrategyParams>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = createTotalDelegatedStakeQuorumReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "createTotalDelegatedStakeQuorum((uint32,uint16,uint16),uint96,(address,uint96)[])";
            const SELECTOR: [u8; 4] = [130u8, 129u8, 171u8, 117u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <ISlashingRegistryCoordinatorTypes::OperatorSetParam as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSetParams,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.minimumStake),
                    <alloy::sol_types::sol_data::Array<
                        IStakeRegistryTypes::StrategyParams,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategyParams),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `deregisterOperator(address,address,uint32[])` and selector `0x303ca956`.
    ```solidity
    function deregisterOperator(address operator, address avs, uint32[] memory operatorSetIds) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSetIds: alloy::sol_types::private::Vec<u32>,
    }
    ///Container type for the return parameters of the [`deregisterOperator(address,address,uint32[])`](deregisterOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorReturn {}
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<u32>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deregisterOperatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorCall) -> Self {
                    (value.operator, value.avs, value.operatorSetIds)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        avs: tuple.1,
                        operatorSetIds: tuple.2,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deregisterOperatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deregisterOperatorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deregisterOperator(address,address,uint32[])";
            const SELECTOR: [u8; 4] = [48u8, 60u8, 169u8, 86u8];
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
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.avs,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSetIds),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `domainSeparator()` and selector `0xf698da25`.
    ```solidity
    function domainSeparator() external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct domainSeparatorCall {}
    ///Container type for the return parameters of the [`domainSeparator()`](domainSeparatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct domainSeparatorReturn {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<domainSeparatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: domainSeparatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for domainSeparatorCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<domainSeparatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: domainSeparatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for domainSeparatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for domainSeparatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = domainSeparatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "domainSeparator()";
            const SELECTOR: [u8; 4] = [246u8, 152u8, 218u8, 37u8];
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub extensions:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `ejectOperator(address,bytes)` and selector `0x6e3b17db`.
    ```solidity
    function ejectOperator(address operator, bytes memory quorumNumbers) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ejectOperatorCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub quorumNumbers: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`ejectOperator(address,bytes)`](ejectOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ejectOperatorReturn {}
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ejectOperatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: ejectOperatorCall) -> Self {
                    (value.operator, value.quorumNumbers)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ejectOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        quorumNumbers: tuple.1,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ejectOperatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ejectOperatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ejectOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ejectOperatorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = ejectOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ejectOperator(address,bytes)";
            const SELECTOR: [u8; 4] = [110u8, 59u8, 23u8, 219u8];
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
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumbers,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `ejectionCooldown()` and selector `0xa96f783e`.
    ```solidity
    function ejectionCooldown() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ejectionCooldownCall {}
    ///Container type for the return parameters of the [`ejectionCooldown()`](ejectionCooldownCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ejectionCooldownReturn {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ejectionCooldownCall> for UnderlyingRustTuple<'_> {
                fn from(value: ejectionCooldownCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ejectionCooldownCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ejectionCooldownReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ejectionCooldownReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ejectionCooldownReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ejectionCooldownCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = ejectionCooldownReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ejectionCooldown()";
            const SELECTOR: [u8; 4] = [169u8, 111u8, 120u8, 62u8];
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `ejector()` and selector `0x28f61b31`.
    ```solidity
    function ejector() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ejectorCall {}
    ///Container type for the return parameters of the [`ejector()`](ejectorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ejectorReturn {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ejectorCall> for UnderlyingRustTuple<'_> {
                fn from(value: ejectorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ejectorCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ejectorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ejectorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ejectorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ejectorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = ejectorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ejector()";
            const SELECTOR: [u8; 4] = [40u8, 246u8, 27u8, 49u8];
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getCurrentQuorumBitmap(bytes32)` and selector `0x871ef049`.
    ```solidity
    function getCurrentQuorumBitmap(bytes32 operatorId) external view returns (uint192);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentQuorumBitmapCall {
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`getCurrentQuorumBitmap(bytes32)`](getCurrentQuorumBitmapCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentQuorumBitmapReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U192,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getCurrentQuorumBitmapCall> for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentQuorumBitmapCall) -> Self {
                    (value.operatorId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCurrentQuorumBitmapCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorId: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<192>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U192,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getCurrentQuorumBitmapReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentQuorumBitmapReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCurrentQuorumBitmapReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCurrentQuorumBitmapCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getCurrentQuorumBitmapReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<192>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCurrentQuorumBitmap(bytes32)";
            const SELECTOR: [u8; 4] = [135u8, 30u8, 240u8, 73u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getOperator(address)` and selector `0x5865c60c`.
    ```solidity
    function getOperator(address operator) external view returns (ISlashingRegistryCoordinatorTypes.OperatorInfo memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getOperator(address)`](getOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorReturn {
        #[allow(missing_docs)]
        pub _0: <ISlashingRegistryCoordinatorTypes::OperatorInfo as alloy::sol_types::SolType>::RustType,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (ISlashingRegistryCoordinatorTypes::OperatorInfo,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ISlashingRegistryCoordinatorTypes::OperatorInfo as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorReturn;
            type ReturnTuple<'a> = (ISlashingRegistryCoordinatorTypes::OperatorInfo,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperator(address)";
            const SELECTOR: [u8; 4] = [88u8, 101u8, 198u8, 12u8];
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
                        &self.operator,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getOperatorFromId(bytes32)` and selector `0x296bb064`.
    ```solidity
    function getOperatorFromId(bytes32 operatorId) external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorFromIdCall {
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`getOperatorFromId(bytes32)`](getOperatorFromIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorFromIdReturn {
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorFromIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorFromIdCall) -> Self {
                    (value.operatorId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorFromIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorId: tuple.0,
                    }
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorFromIdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorFromIdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorFromIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorFromIdCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorFromIdReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorFromId(bytes32)";
            const SELECTOR: [u8; 4] = [41u8, 107u8, 176u8, 100u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getOperatorId(address)` and selector `0x13542a4e`.
    ```solidity
    function getOperatorId(address operator) external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorIdCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getOperatorId(address)`](getOperatorIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorIdReturn {
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorIdCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorIdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorIdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorIdCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorIdReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorId(address)";
            const SELECTOR: [u8; 4] = [19u8, 84u8, 42u8, 78u8];
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
                        &self.operator,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getOperatorSetParams(uint8)` and selector `0xe65797ad`.
    ```solidity
    function getOperatorSetParams(uint8 quorumNumber) external view returns (ISlashingRegistryCoordinatorTypes.OperatorSetParam memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorSetParamsCall {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
    }
    ///Container type for the return parameters of the [`getOperatorSetParams(uint8)`](getOperatorSetParamsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorSetParamsReturn {
        #[allow(missing_docs)]
        pub _0: <ISlashingRegistryCoordinatorTypes::OperatorSetParam as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorSetParamsCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSetParamsCall) -> Self {
                    (value.quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorSetParamsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (ISlashingRegistryCoordinatorTypes::OperatorSetParam,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ISlashingRegistryCoordinatorTypes::OperatorSetParam as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorSetParamsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSetParamsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorSetParamsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorSetParamsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorSetParamsReturn;
            type ReturnTuple<'a> = (ISlashingRegistryCoordinatorTypes::OperatorSetParam,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorSetParams(uint8)";
            const SELECTOR: [u8; 4] = [230u8, 87u8, 151u8, 173u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumber,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getOperatorStatus(address)` and selector `0xfd39105a`.
    ```solidity
    function getOperatorStatus(address operator) external view returns (ISlashingRegistryCoordinatorTypes.OperatorStatus);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorStatusCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getOperatorStatus(address)`](getOperatorStatusCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorStatusReturn {
        #[allow(missing_docs)]
        pub _0: <ISlashingRegistryCoordinatorTypes::OperatorStatus as alloy::sol_types::SolType>::RustType,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorStatusCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorStatusCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorStatusCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (ISlashingRegistryCoordinatorTypes::OperatorStatus,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ISlashingRegistryCoordinatorTypes::OperatorStatus as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorStatusReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorStatusReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorStatusReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorStatusCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorStatusReturn;
            type ReturnTuple<'a> = (ISlashingRegistryCoordinatorTypes::OperatorStatus,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorStatus(address)";
            const SELECTOR: [u8; 4] = [253u8, 57u8, 16u8, 90u8];
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
                        &self.operator,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getQuorumBitmapAtBlockNumberByIndex(bytes32,uint32,uint256)` and selector `0x04ec6351`.
    ```solidity
    function getQuorumBitmapAtBlockNumberByIndex(bytes32 operatorId, uint32 blockNumber, uint256 index) external view returns (uint192);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumBitmapAtBlockNumberByIndexCall {
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub blockNumber: u32,
        #[allow(missing_docs)]
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getQuorumBitmapAtBlockNumberByIndex(bytes32,uint32,uint256)`](getQuorumBitmapAtBlockNumberByIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumBitmapAtBlockNumberByIndexReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U192,
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
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                u32,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getQuorumBitmapAtBlockNumberByIndexCall> for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapAtBlockNumberByIndexCall) -> Self {
                    (value.operatorId, value.blockNumber, value.index)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getQuorumBitmapAtBlockNumberByIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorId: tuple.0,
                        blockNumber: tuple.1,
                        index: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<192>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U192,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getQuorumBitmapAtBlockNumberByIndexReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapAtBlockNumberByIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getQuorumBitmapAtBlockNumberByIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getQuorumBitmapAtBlockNumberByIndexCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getQuorumBitmapAtBlockNumberByIndexReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<192>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "getQuorumBitmapAtBlockNumberByIndex(bytes32,uint32,uint256)";
            const SELECTOR: [u8; 4] = [4u8, 236u8, 99u8, 81u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.blockNumber),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.index),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getQuorumBitmapHistoryLength(bytes32)` and selector `0x03fd3492`.
    ```solidity
    function getQuorumBitmapHistoryLength(bytes32 operatorId) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumBitmapHistoryLengthCall {
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`getQuorumBitmapHistoryLength(bytes32)`](getQuorumBitmapHistoryLengthCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumBitmapHistoryLengthReturn {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getQuorumBitmapHistoryLengthCall> for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapHistoryLengthCall) -> Self {
                    (value.operatorId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getQuorumBitmapHistoryLengthCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorId: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getQuorumBitmapHistoryLengthReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapHistoryLengthReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getQuorumBitmapHistoryLengthReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getQuorumBitmapHistoryLengthCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getQuorumBitmapHistoryLengthReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getQuorumBitmapHistoryLength(bytes32)";
            const SELECTOR: [u8; 4] = [3u8, 253u8, 52u8, 146u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getQuorumBitmapIndicesAtBlockNumber(uint32,bytes32[])` and selector `0xc391425e`.
    ```solidity
    function getQuorumBitmapIndicesAtBlockNumber(uint32 blockNumber, bytes32[] memory operatorIds) external view returns (uint32[] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumBitmapIndicesAtBlockNumberCall {
        #[allow(missing_docs)]
        pub blockNumber: u32,
        #[allow(missing_docs)]
        pub operatorIds: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
    }
    ///Container type for the return parameters of the [`getQuorumBitmapIndicesAtBlockNumber(uint32,bytes32[])`](getQuorumBitmapIndicesAtBlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumBitmapIndicesAtBlockNumberReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<u32>,
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
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u32,
                alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getQuorumBitmapIndicesAtBlockNumberCall> for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapIndicesAtBlockNumberCall) -> Self {
                    (value.blockNumber, value.operatorIds)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getQuorumBitmapIndicesAtBlockNumberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        blockNumber: tuple.0,
                        operatorIds: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Vec<u32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getQuorumBitmapIndicesAtBlockNumberReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapIndicesAtBlockNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getQuorumBitmapIndicesAtBlockNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getQuorumBitmapIndicesAtBlockNumberCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getQuorumBitmapIndicesAtBlockNumberReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getQuorumBitmapIndicesAtBlockNumber(uint32,bytes32[])";
            const SELECTOR: [u8; 4] = [195u8, 145u8, 66u8, 94u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.blockNumber),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorIds),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getQuorumBitmapUpdateByIndex(bytes32,uint256)` and selector `0x1eb812da`.
    ```solidity
    function getQuorumBitmapUpdateByIndex(bytes32 operatorId, uint256 index) external view returns (ISlashingRegistryCoordinatorTypes.QuorumBitmapUpdate memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumBitmapUpdateByIndexCall {
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getQuorumBitmapUpdateByIndex(bytes32,uint256)`](getQuorumBitmapUpdateByIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumBitmapUpdateByIndexReturn {
        #[allow(missing_docs)]
        pub _0: <ISlashingRegistryCoordinatorTypes::QuorumBitmapUpdate as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getQuorumBitmapUpdateByIndexCall> for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapUpdateByIndexCall) -> Self {
                    (value.operatorId, value.index)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getQuorumBitmapUpdateByIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorId: tuple.0,
                        index: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (ISlashingRegistryCoordinatorTypes::QuorumBitmapUpdate,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ISlashingRegistryCoordinatorTypes::QuorumBitmapUpdate as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getQuorumBitmapUpdateByIndexReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapUpdateByIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getQuorumBitmapUpdateByIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getQuorumBitmapUpdateByIndexCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getQuorumBitmapUpdateByIndexReturn;
            type ReturnTuple<'a> = (ISlashingRegistryCoordinatorTypes::QuorumBitmapUpdate,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getQuorumBitmapUpdateByIndex(bytes32,uint256)";
            const SELECTOR: [u8; 4] = [30u8, 184u8, 18u8, 218u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.index),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `indexRegistry()` and selector `0x9e9923c2`.
    ```solidity
    function indexRegistry() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct indexRegistryCall {}
    ///Container type for the return parameters of the [`indexRegistry()`](indexRegistryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct indexRegistryReturn {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<indexRegistryCall> for UnderlyingRustTuple<'_> {
                fn from(value: indexRegistryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for indexRegistryCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<indexRegistryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: indexRegistryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for indexRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for indexRegistryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = indexRegistryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "indexRegistry()";
            const SELECTOR: [u8; 4] = [158u8, 153u8, 35u8, 194u8];
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `initialize(address,address,address,uint256,address)` and selector `0x530b97a4`.
    ```solidity
    function initialize(address initialOwner, address churnApprover, address ejector, uint256 initialPausedStatus, address avs) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        #[allow(missing_docs)]
        pub initialOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub churnApprover: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub ejector: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub initialPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`initialize(address,address,address,uint256,address)`](initializeCall) function.
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                        value.initialOwner,
                        value.churnApprover,
                        value.ejector,
                        value.initialPausedStatus,
                        value.avs,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        initialOwner: tuple.0,
                        churnApprover: tuple.1,
                        ejector: tuple.2,
                        initialPausedStatus: tuple.3,
                        avs: tuple.4,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize(address,address,address,uint256,address)";
            const SELECTOR: [u8; 4] = [83u8, 11u8, 151u8, 164u8];
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
                        &self.initialOwner,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.churnApprover,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.ejector,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.initialPausedStatus,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.avs,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `isChurnApproverSaltUsed(bytes32)` and selector `0x1478851f`.
    ```solidity
    function isChurnApproverSaltUsed(bytes32) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isChurnApproverSaltUsedCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`isChurnApproverSaltUsed(bytes32)`](isChurnApproverSaltUsedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isChurnApproverSaltUsedReturn {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isChurnApproverSaltUsedCall> for UnderlyingRustTuple<'_> {
                fn from(value: isChurnApproverSaltUsedCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isChurnApproverSaltUsedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isChurnApproverSaltUsedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isChurnApproverSaltUsedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isChurnApproverSaltUsedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isChurnApproverSaltUsedCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = isChurnApproverSaltUsedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isChurnApproverSaltUsed(bytes32)";
            const SELECTOR: [u8; 4] = [20u8, 120u8, 133u8, 31u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `lastEjectionTimestamp(address)` and selector `0x125e0584`.
    ```solidity
    function lastEjectionTimestamp(address) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastEjectionTimestampCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`lastEjectionTimestamp(address)`](lastEjectionTimestampCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastEjectionTimestampReturn {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<lastEjectionTimestampCall> for UnderlyingRustTuple<'_> {
                fn from(value: lastEjectionTimestampCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lastEjectionTimestampCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<lastEjectionTimestampReturn> for UnderlyingRustTuple<'_> {
                fn from(value: lastEjectionTimestampReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lastEjectionTimestampReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for lastEjectionTimestampCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = lastEjectionTimestampReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "lastEjectionTimestamp(address)";
            const SELECTOR: [u8; 4] = [18u8, 94u8, 5u8, 132u8];
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
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `owner()` and selector `0x8da5cb5b`.
    ```solidity
    function owner() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerCall {}
    ///Container type for the return parameters of the [`owner()`](ownerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerReturn {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerCall> for UnderlyingRustTuple<'_> {
                fn from(value: ownerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ownerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ownerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = ownerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "owner()";
            const SELECTOR: [u8; 4] = [141u8, 165u8, 203u8, 91u8];
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `pause(uint256)` and selector `0x136439dd`.
    ```solidity
    function pause(uint256 newPausedStatus) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseCall {
        #[allow(missing_docs)]
        pub newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`pause(uint256)`](pauseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseReturn {}
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
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pauseCall> for UnderlyingRustTuple<'_> {
                fn from(value: pauseCall) -> Self {
                    (value.newPausedStatus,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newPausedStatus: tuple.0,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pauseReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pauseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pauseCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = pauseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pause(uint256)";
            const SELECTOR: [u8; 4] = [19u8, 100u8, 57u8, 221u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.newPausedStatus,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `pauseAll()` and selector `0x595c6a67`.
    ```solidity
    function pauseAll() external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseAllCall {}
    ///Container type for the return parameters of the [`pauseAll()`](pauseAllCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseAllReturn {}
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pauseAllCall> for UnderlyingRustTuple<'_> {
                fn from(value: pauseAllCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseAllCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pauseAllReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pauseAllReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseAllReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pauseAllCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = pauseAllReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pauseAll()";
            const SELECTOR: [u8; 4] = [89u8, 92u8, 106u8, 103u8];
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `paused(uint8)` and selector `0x5ac86ab7`.
    ```solidity
    function paused(uint8 index) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paused_0Call {
        #[allow(missing_docs)]
        pub index: u8,
    }
    ///Container type for the return parameters of the [`paused(uint8)`](paused_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paused_0Return {
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<paused_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: paused_0Call) -> Self {
                    (value.index,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for paused_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { index: tuple.0 }
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<paused_0Return> for UnderlyingRustTuple<'_> {
                fn from(value: paused_0Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for paused_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for paused_0Call {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = paused_0Return;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "paused(uint8)";
            const SELECTOR: [u8; 4] = [90u8, 200u8, 106u8, 183u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self.index,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `paused()` and selector `0x5c975abb`.
    ```solidity
    function paused() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paused_1Call {}
    ///Container type for the return parameters of the [`paused()`](paused_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paused_1Return {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<paused_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: paused_1Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for paused_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<paused_1Return> for UnderlyingRustTuple<'_> {
                fn from(value: paused_1Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for paused_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for paused_1Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = paused_1Return;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "paused()";
            const SELECTOR: [u8; 4] = [92u8, 151u8, 90u8, 187u8];
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `pauserRegistry()` and selector `0x886f1195`.
    ```solidity
    function pauserRegistry() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauserRegistryCall {}
    ///Container type for the return parameters of the [`pauserRegistry()`](pauserRegistryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauserRegistryReturn {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pauserRegistryCall> for UnderlyingRustTuple<'_> {
                fn from(value: pauserRegistryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauserRegistryCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pauserRegistryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pauserRegistryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauserRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pauserRegistryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = pauserRegistryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pauserRegistry()";
            const SELECTOR: [u8; 4] = [136u8, 111u8, 17u8, 149u8];
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `pubkeyRegistrationMessageHash(address)` and selector `0x3c2a7f4c`.
    ```solidity
    function pubkeyRegistrationMessageHash(address operator) external view returns (BN254.G1Point memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pubkeyRegistrationMessageHashCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`pubkeyRegistrationMessageHash(address)`](pubkeyRegistrationMessageHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pubkeyRegistrationMessageHashReturn {
        #[allow(missing_docs)]
        pub _0: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pubkeyRegistrationMessageHashCall> for UnderlyingRustTuple<'_> {
                fn from(value: pubkeyRegistrationMessageHashCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pubkeyRegistrationMessageHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (BN254::G1Point,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<BN254::G1Point as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pubkeyRegistrationMessageHashReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pubkeyRegistrationMessageHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pubkeyRegistrationMessageHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pubkeyRegistrationMessageHashCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = pubkeyRegistrationMessageHashReturn;
            type ReturnTuple<'a> = (BN254::G1Point,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pubkeyRegistrationMessageHash(address)";
            const SELECTOR: [u8; 4] = [60u8, 42u8, 127u8, 76u8];
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
                        &self.operator,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `quorumCount()` and selector `0x9aa1653d`.
    ```solidity
    function quorumCount() external view returns (uint8);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct quorumCountCall {}
    ///Container type for the return parameters of the [`quorumCount()`](quorumCountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct quorumCountReturn {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<quorumCountCall> for UnderlyingRustTuple<'_> {
                fn from(value: quorumCountCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for quorumCountCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<quorumCountReturn> for UnderlyingRustTuple<'_> {
                fn from(value: quorumCountReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for quorumCountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for quorumCountCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = quorumCountReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "quorumCount()";
            const SELECTOR: [u8; 4] = [154u8, 161u8, 101u8, 61u8];
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `quorumUpdateBlockNumber(uint8)` and selector `0x249a0c42`.
    ```solidity
    function quorumUpdateBlockNumber(uint8) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct quorumUpdateBlockNumberCall {
        #[allow(missing_docs)]
        pub _0: u8,
    }
    ///Container type for the return parameters of the [`quorumUpdateBlockNumber(uint8)`](quorumUpdateBlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct quorumUpdateBlockNumberReturn {
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<quorumUpdateBlockNumberCall> for UnderlyingRustTuple<'_> {
                fn from(value: quorumUpdateBlockNumberCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for quorumUpdateBlockNumberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<quorumUpdateBlockNumberReturn> for UnderlyingRustTuple<'_> {
                fn from(value: quorumUpdateBlockNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for quorumUpdateBlockNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for quorumUpdateBlockNumberCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = quorumUpdateBlockNumberReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "quorumUpdateBlockNumber(uint8)";
            const SELECTOR: [u8; 4] = [36u8, 154u8, 12u8, 66u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `registerOperator(address,address,uint32[],bytes)` and selector `0xc63fd502`.
    ```solidity
    function registerOperator(address operator, address avs, uint32[] memory operatorSetIds, bytes memory data) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSetIds: alloy::sol_types::private::Vec<u32>,
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`registerOperator(address,address,uint32[],bytes)`](registerOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorReturn {}
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<u32>,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerOperatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorCall) -> Self {
                    (value.operator, value.avs, value.operatorSetIds, value.data)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        avs: tuple.1,
                        operatorSetIds: tuple.2,
                        data: tuple.3,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerOperatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerOperatorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registerOperator(address,address,uint32[],bytes)";
            const SELECTOR: [u8; 4] = [198u8, 63u8, 213u8, 2u8];
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
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.avs,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSetIds),
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `renounceOwnership()` and selector `0x715018a6`.
    ```solidity
    function renounceOwnership() external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipCall {}
    ///Container type for the return parameters of the [`renounceOwnership()`](renounceOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipReturn {}
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipCall> for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for renounceOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipReturn> for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for renounceOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceOwnershipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "renounceOwnership()";
            const SELECTOR: [u8; 4] = [113u8, 80u8, 24u8, 166u8];
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `setAVS(address)` and selector `0xa65497c6`.
    ```solidity
    function setAVS(address _avs) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAVSCall {
        #[allow(missing_docs)]
        pub _avs: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setAVS(address)`](setAVSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAVSReturn {}
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setAVSCall> for UnderlyingRustTuple<'_> {
                fn from(value: setAVSCall) -> Self {
                    (value._avs,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setAVSCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _avs: tuple.0 }
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setAVSReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setAVSReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setAVSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setAVSCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setAVSReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setAVS(address)";
            const SELECTOR: [u8; 4] = [166u8, 84u8, 151u8, 198u8];
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
                        &self._avs,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `setChurnApprover(address)` and selector `0x29d1e0c3`.
    ```solidity
    function setChurnApprover(address _churnApprover) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setChurnApproverCall {
        #[allow(missing_docs)]
        pub _churnApprover: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setChurnApprover(address)`](setChurnApproverCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setChurnApproverReturn {}
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setChurnApproverCall> for UnderlyingRustTuple<'_> {
                fn from(value: setChurnApproverCall) -> Self {
                    (value._churnApprover,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setChurnApproverCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _churnApprover: tuple.0,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setChurnApproverReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setChurnApproverReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setChurnApproverReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setChurnApproverCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setChurnApproverReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setChurnApprover(address)";
            const SELECTOR: [u8; 4] = [41u8, 209u8, 224u8, 195u8];
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
                        &self._churnApprover,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `setEjectionCooldown(uint256)` and selector `0x0d3f2134`.
    ```solidity
    function setEjectionCooldown(uint256 _ejectionCooldown) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setEjectionCooldownCall {
        #[allow(missing_docs)]
        pub _ejectionCooldown: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`setEjectionCooldown(uint256)`](setEjectionCooldownCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setEjectionCooldownReturn {}
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
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setEjectionCooldownCall> for UnderlyingRustTuple<'_> {
                fn from(value: setEjectionCooldownCall) -> Self {
                    (value._ejectionCooldown,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setEjectionCooldownCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _ejectionCooldown: tuple.0,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setEjectionCooldownReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setEjectionCooldownReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setEjectionCooldownReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setEjectionCooldownCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setEjectionCooldownReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setEjectionCooldown(uint256)";
            const SELECTOR: [u8; 4] = [13u8, 63u8, 33u8, 52u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self._ejectionCooldown,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `setEjector(address)` and selector `0x2cdd1e86`.
    ```solidity
    function setEjector(address _ejector) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setEjectorCall {
        #[allow(missing_docs)]
        pub _ejector: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setEjector(address)`](setEjectorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setEjectorReturn {}
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setEjectorCall> for UnderlyingRustTuple<'_> {
                fn from(value: setEjectorCall) -> Self {
                    (value._ejector,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setEjectorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _ejector: tuple.0 }
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setEjectorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setEjectorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setEjectorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setEjectorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setEjectorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setEjector(address)";
            const SELECTOR: [u8; 4] = [44u8, 221u8, 30u8, 134u8];
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
                        &self._ejector,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `setOperatorSetParams(uint8,(uint32,uint16,uint16))` and selector `0x5b0b829f`.
    ```solidity
    function setOperatorSetParams(uint8 quorumNumber, ISlashingRegistryCoordinatorTypes.OperatorSetParam memory operatorSetParams) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setOperatorSetParamsCall {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub operatorSetParams: <ISlashingRegistryCoordinatorTypes::OperatorSetParam as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`setOperatorSetParams(uint8,(uint32,uint16,uint16))`](setOperatorSetParamsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setOperatorSetParamsReturn {}
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
                alloy::sol_types::sol_data::Uint<8>,
                ISlashingRegistryCoordinatorTypes::OperatorSetParam,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
                <ISlashingRegistryCoordinatorTypes::OperatorSetParam as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setOperatorSetParamsCall> for UnderlyingRustTuple<'_> {
                fn from(value: setOperatorSetParamsCall) -> Self {
                    (value.quorumNumber, value.operatorSetParams)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setOperatorSetParamsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        operatorSetParams: tuple.1,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setOperatorSetParamsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setOperatorSetParamsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setOperatorSetParamsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setOperatorSetParamsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                ISlashingRegistryCoordinatorTypes::OperatorSetParam,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setOperatorSetParamsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setOperatorSetParams(uint8,(uint32,uint16,uint16))";
            const SELECTOR: [u8; 4] = [91u8, 11u8, 130u8, 159u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                    <ISlashingRegistryCoordinatorTypes::OperatorSetParam as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSetParams,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `socketRegistry()` and selector `0xea32afae`.
    ```solidity
    function socketRegistry() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct socketRegistryCall {}
    ///Container type for the return parameters of the [`socketRegistry()`](socketRegistryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct socketRegistryReturn {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<socketRegistryCall> for UnderlyingRustTuple<'_> {
                fn from(value: socketRegistryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for socketRegistryCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<socketRegistryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: socketRegistryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for socketRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for socketRegistryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = socketRegistryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "socketRegistry()";
            const SELECTOR: [u8; 4] = [234u8, 50u8, 175u8, 174u8];
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `stakeRegistry()` and selector `0x68304835`.
    ```solidity
    function stakeRegistry() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakeRegistryCall {}
    ///Container type for the return parameters of the [`stakeRegistry()`](stakeRegistryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakeRegistryReturn {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<stakeRegistryCall> for UnderlyingRustTuple<'_> {
                fn from(value: stakeRegistryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stakeRegistryCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<stakeRegistryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: stakeRegistryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stakeRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for stakeRegistryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = stakeRegistryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "stakeRegistry()";
            const SELECTOR: [u8; 4] = [104u8, 48u8, 72u8, 53u8];
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `supportsAVS(address)` and selector `0xb5265787`.
    ```solidity
    function supportsAVS(address _avs) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct supportsAVSCall {
        #[allow(missing_docs)]
        pub _avs: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`supportsAVS(address)`](supportsAVSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct supportsAVSReturn {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<supportsAVSCall> for UnderlyingRustTuple<'_> {
                fn from(value: supportsAVSCall) -> Self {
                    (value._avs,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for supportsAVSCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _avs: tuple.0 }
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<supportsAVSReturn> for UnderlyingRustTuple<'_> {
                fn from(value: supportsAVSReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for supportsAVSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for supportsAVSCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = supportsAVSReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "supportsAVS(address)";
            const SELECTOR: [u8; 4] = [181u8, 38u8, 87u8, 135u8];
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
                        &self._avs,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `transferOwnership(address)` and selector `0xf2fde38b`.
    ```solidity
    function transferOwnership(address newOwner) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipCall {
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`transferOwnership(address)`](transferOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipReturn {}
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipCall> for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipCall) -> Self {
                    (value.newOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newOwner: tuple.0 }
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipReturn> for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferOwnershipCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transferOwnership(address)";
            const SELECTOR: [u8; 4] = [242u8, 253u8, 227u8, 139u8];
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
                        &self.newOwner,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `unpause(uint256)` and selector `0xfabc1cbc`.
    ```solidity
    function unpause(uint256 newPausedStatus) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unpauseCall {
        #[allow(missing_docs)]
        pub newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`unpause(uint256)`](unpauseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unpauseReturn {}
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
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<unpauseCall> for UnderlyingRustTuple<'_> {
                fn from(value: unpauseCall) -> Self {
                    (value.newPausedStatus,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unpauseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newPausedStatus: tuple.0,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<unpauseReturn> for UnderlyingRustTuple<'_> {
                fn from(value: unpauseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unpauseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for unpauseCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = unpauseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "unpause(uint256)";
            const SELECTOR: [u8; 4] = [250u8, 188u8, 28u8, 188u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.newPausedStatus,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `updateOperators(address[])` and selector `0x00cf2ab5`.
    ```solidity
    function updateOperators(address[] memory operators) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorsCall {
        #[allow(missing_docs)]
        pub operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    ///Container type for the return parameters of the [`updateOperators(address[])`](updateOperatorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorsReturn {}
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
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorsCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsCall) -> Self {
                    (value.operators,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operators: tuple.0 }
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateOperatorsCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateOperatorsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateOperators(address[])";
            const SELECTOR: [u8; 4] = [0u8, 207u8, 42u8, 181u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Address,
                > as alloy_sol_types::SolType>::tokenize(
                    &self.operators
                ),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `updateOperatorsForQuorum(address[][],bytes)` and selector `0x5140a548`.
    ```solidity
    function updateOperatorsForQuorum(address[][] memory operatorsPerQuorum, bytes memory quorumNumbers) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorsForQuorumCall {
        #[allow(missing_docs)]
        pub operatorsPerQuorum: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        >,
        #[allow(missing_docs)]
        pub quorumNumbers: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`updateOperatorsForQuorum(address[][],bytes)`](updateOperatorsForQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorsForQuorumReturn {}
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
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                >,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                >,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorsForQuorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsForQuorumCall) -> Self {
                    (value.operatorsPerQuorum, value.quorumNumbers)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorsForQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorsPerQuorum: tuple.0,
                        quorumNumbers: tuple.1,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorsForQuorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsForQuorumReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorsForQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateOperatorsForQuorumCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                >,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateOperatorsForQuorumReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateOperatorsForQuorum(address[][],bytes)";
            const SELECTOR: [u8; 4] = [81u8, 64u8, 165u8, 72u8];
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
                        alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.operatorsPerQuorum
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumbers,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `updateSocket(string)` and selector `0x0cf4b767`.
    ```solidity
    function updateSocket(string memory socket) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateSocketCall {
        #[allow(missing_docs)]
        pub socket: alloy::sol_types::private::String,
    }
    ///Container type for the return parameters of the [`updateSocket(string)`](updateSocketCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateSocketReturn {}
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateSocketCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateSocketCall) -> Self {
                    (value.socket,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateSocketCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { socket: tuple.0 }
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateSocketReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateSocketReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateSocketReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateSocketCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::String,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateSocketReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateSocket(string)";
            const SELECTOR: [u8; 4] = [12u8, 244u8, 183u8, 103u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.socket,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = versionReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    ///Container for all the [`SlashingRegistryCoordinator`](self) function calls.
    pub enum SlashingRegistryCoordinatorCalls {
        #[allow(missing_docs)]
        OPERATOR_CHURN_APPROVAL_TYPEHASH(OPERATOR_CHURN_APPROVAL_TYPEHASHCall),
        #[allow(missing_docs)]
        PUBKEY_REGISTRATION_TYPEHASH(PUBKEY_REGISTRATION_TYPEHASHCall),
        #[allow(missing_docs)]
        allocationManager(allocationManagerCall),
        #[allow(missing_docs)]
        avs(avsCall),
        #[allow(missing_docs)]
        blsApkRegistry(blsApkRegistryCall),
        #[allow(missing_docs)]
        calculateOperatorChurnApprovalDigestHash(calculateOperatorChurnApprovalDigestHashCall),
        #[allow(missing_docs)]
        calculatePubkeyRegistrationMessageHash(calculatePubkeyRegistrationMessageHashCall),
        #[allow(missing_docs)]
        churnApprover(churnApproverCall),
        #[allow(missing_docs)]
        createSlashableStakeQuorum(createSlashableStakeQuorumCall),
        #[allow(missing_docs)]
        createTotalDelegatedStakeQuorum(createTotalDelegatedStakeQuorumCall),
        #[allow(missing_docs)]
        deregisterOperator(deregisterOperatorCall),
        #[allow(missing_docs)]
        domainSeparator(domainSeparatorCall),
        #[allow(missing_docs)]
        eip712Domain(eip712DomainCall),
        #[allow(missing_docs)]
        ejectOperator(ejectOperatorCall),
        #[allow(missing_docs)]
        ejectionCooldown(ejectionCooldownCall),
        #[allow(missing_docs)]
        ejector(ejectorCall),
        #[allow(missing_docs)]
        getCurrentQuorumBitmap(getCurrentQuorumBitmapCall),
        #[allow(missing_docs)]
        getOperator(getOperatorCall),
        #[allow(missing_docs)]
        getOperatorFromId(getOperatorFromIdCall),
        #[allow(missing_docs)]
        getOperatorId(getOperatorIdCall),
        #[allow(missing_docs)]
        getOperatorSetParams(getOperatorSetParamsCall),
        #[allow(missing_docs)]
        getOperatorStatus(getOperatorStatusCall),
        #[allow(missing_docs)]
        getQuorumBitmapAtBlockNumberByIndex(getQuorumBitmapAtBlockNumberByIndexCall),
        #[allow(missing_docs)]
        getQuorumBitmapHistoryLength(getQuorumBitmapHistoryLengthCall),
        #[allow(missing_docs)]
        getQuorumBitmapIndicesAtBlockNumber(getQuorumBitmapIndicesAtBlockNumberCall),
        #[allow(missing_docs)]
        getQuorumBitmapUpdateByIndex(getQuorumBitmapUpdateByIndexCall),
        #[allow(missing_docs)]
        indexRegistry(indexRegistryCall),
        #[allow(missing_docs)]
        initialize(initializeCall),
        #[allow(missing_docs)]
        isChurnApproverSaltUsed(isChurnApproverSaltUsedCall),
        #[allow(missing_docs)]
        lastEjectionTimestamp(lastEjectionTimestampCall),
        #[allow(missing_docs)]
        owner(ownerCall),
        #[allow(missing_docs)]
        pause(pauseCall),
        #[allow(missing_docs)]
        pauseAll(pauseAllCall),
        #[allow(missing_docs)]
        paused_0(paused_0Call),
        #[allow(missing_docs)]
        paused_1(paused_1Call),
        #[allow(missing_docs)]
        pauserRegistry(pauserRegistryCall),
        #[allow(missing_docs)]
        pubkeyRegistrationMessageHash(pubkeyRegistrationMessageHashCall),
        #[allow(missing_docs)]
        quorumCount(quorumCountCall),
        #[allow(missing_docs)]
        quorumUpdateBlockNumber(quorumUpdateBlockNumberCall),
        #[allow(missing_docs)]
        registerOperator(registerOperatorCall),
        #[allow(missing_docs)]
        renounceOwnership(renounceOwnershipCall),
        #[allow(missing_docs)]
        setAVS(setAVSCall),
        #[allow(missing_docs)]
        setChurnApprover(setChurnApproverCall),
        #[allow(missing_docs)]
        setEjectionCooldown(setEjectionCooldownCall),
        #[allow(missing_docs)]
        setEjector(setEjectorCall),
        #[allow(missing_docs)]
        setOperatorSetParams(setOperatorSetParamsCall),
        #[allow(missing_docs)]
        socketRegistry(socketRegistryCall),
        #[allow(missing_docs)]
        stakeRegistry(stakeRegistryCall),
        #[allow(missing_docs)]
        supportsAVS(supportsAVSCall),
        #[allow(missing_docs)]
        transferOwnership(transferOwnershipCall),
        #[allow(missing_docs)]
        unpause(unpauseCall),
        #[allow(missing_docs)]
        updateOperators(updateOperatorsCall),
        #[allow(missing_docs)]
        updateOperatorsForQuorum(updateOperatorsForQuorumCall),
        #[allow(missing_docs)]
        updateSocket(updateSocketCall),
        #[allow(missing_docs)]
        version(versionCall),
    }
    #[automatically_derived]
    impl SlashingRegistryCoordinatorCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [0u8, 207u8, 42u8, 181u8],
            [3u8, 253u8, 52u8, 146u8],
            [4u8, 236u8, 99u8, 81u8],
            [5u8, 67u8, 16u8, 230u8],
            [12u8, 244u8, 183u8, 103u8],
            [13u8, 63u8, 33u8, 52u8],
            [18u8, 94u8, 5u8, 132u8],
            [19u8, 84u8, 42u8, 78u8],
            [19u8, 100u8, 57u8, 221u8],
            [20u8, 120u8, 133u8, 31u8],
            [30u8, 184u8, 18u8, 218u8],
            [36u8, 154u8, 12u8, 66u8],
            [40u8, 246u8, 27u8, 49u8],
            [41u8, 107u8, 176u8, 100u8],
            [41u8, 209u8, 224u8, 195u8],
            [44u8, 221u8, 30u8, 134u8],
            [48u8, 60u8, 169u8, 86u8],
            [60u8, 42u8, 127u8, 76u8],
            [62u8, 239u8, 58u8, 81u8],
            [81u8, 64u8, 165u8, 72u8],
            [83u8, 11u8, 151u8, 164u8],
            [84u8, 253u8, 77u8, 80u8],
            [88u8, 101u8, 198u8, 12u8],
            [89u8, 92u8, 106u8, 103u8],
            [90u8, 200u8, 106u8, 183u8],
            [91u8, 11u8, 130u8, 159u8],
            [92u8, 151u8, 90u8, 187u8],
            [93u8, 244u8, 89u8, 70u8],
            [104u8, 48u8, 72u8, 53u8],
            [110u8, 59u8, 23u8, 219u8],
            [113u8, 80u8, 24u8, 166u8],
            [115u8, 68u8, 121u8, 146u8],
            [130u8, 129u8, 171u8, 117u8],
            [132u8, 176u8, 25u8, 110u8],
            [132u8, 202u8, 82u8, 19u8],
            [135u8, 30u8, 240u8, 73u8],
            [136u8, 111u8, 17u8, 149u8],
            [141u8, 165u8, 203u8, 91u8],
            [154u8, 161u8, 101u8, 61u8],
            [158u8, 153u8, 35u8, 194u8],
            [159u8, 234u8, 184u8, 89u8],
            [166u8, 84u8, 151u8, 198u8],
            [169u8, 111u8, 120u8, 62u8],
            [181u8, 38u8, 87u8, 135u8],
            [195u8, 145u8, 66u8, 94u8],
            [198u8, 63u8, 213u8, 2u8],
            [202u8, 13u8, 232u8, 130u8],
            [202u8, 138u8, 167u8, 199u8],
            [222u8, 17u8, 100u8, 187u8],
            [230u8, 87u8, 151u8, 173u8],
            [234u8, 50u8, 175u8, 174u8],
            [242u8, 253u8, 227u8, 139u8],
            [246u8, 152u8, 218u8, 37u8],
            [250u8, 188u8, 28u8, 188u8],
            [253u8, 57u8, 16u8, 90u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for SlashingRegistryCoordinatorCalls {
        const NAME: &'static str = "SlashingRegistryCoordinatorCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 55usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::OPERATOR_CHURN_APPROVAL_TYPEHASH(_) => {
                    <OPERATOR_CHURN_APPROVAL_TYPEHASHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::PUBKEY_REGISTRATION_TYPEHASH(_) => {
                    <PUBKEY_REGISTRATION_TYPEHASHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::allocationManager(_) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::avs(_) => <avsCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::blsApkRegistry(_) => {
                    <blsApkRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calculateOperatorChurnApprovalDigestHash(_) => {
                    <calculateOperatorChurnApprovalDigestHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calculatePubkeyRegistrationMessageHash(_) => {
                    <calculatePubkeyRegistrationMessageHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::churnApprover(_) => {
                    <churnApproverCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createSlashableStakeQuorum(_) => {
                    <createSlashableStakeQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createTotalDelegatedStakeQuorum(_) => {
                    <createTotalDelegatedStakeQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deregisterOperator(_) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::domainSeparator(_) => {
                    <domainSeparatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::eip712Domain(_) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::ejectOperator(_) => {
                    <ejectOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::ejectionCooldown(_) => {
                    <ejectionCooldownCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::ejector(_) => <ejectorCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getCurrentQuorumBitmap(_) => {
                    <getCurrentQuorumBitmapCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperator(_) => {
                    <getOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorFromId(_) => {
                    <getOperatorFromIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorId(_) => {
                    <getOperatorIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorSetParams(_) => {
                    <getOperatorSetParamsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorStatus(_) => {
                    <getOperatorStatusCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getQuorumBitmapAtBlockNumberByIndex(_) => {
                    <getQuorumBitmapAtBlockNumberByIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getQuorumBitmapHistoryLength(_) => {
                    <getQuorumBitmapHistoryLengthCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getQuorumBitmapIndicesAtBlockNumber(_) => {
                    <getQuorumBitmapIndicesAtBlockNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getQuorumBitmapUpdateByIndex(_) => {
                    <getQuorumBitmapUpdateByIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::indexRegistry(_) => {
                    <indexRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isChurnApproverSaltUsed(_) => {
                    <isChurnApproverSaltUsedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::lastEjectionTimestamp(_) => {
                    <lastEjectionTimestampCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pause(_) => <pauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pauseAll(_) => <pauseAllCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused_0(_) => <paused_0Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused_1(_) => <paused_1Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::pauserRegistry(_) => {
                    <pauserRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::pubkeyRegistrationMessageHash(_) => {
                    <pubkeyRegistrationMessageHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::quorumCount(_) => {
                    <quorumCountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::quorumUpdateBlockNumber(_) => {
                    <quorumUpdateBlockNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registerOperator(_) => {
                    <registerOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setAVS(_) => <setAVSCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::setChurnApprover(_) => {
                    <setChurnApproverCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setEjectionCooldown(_) => {
                    <setEjectionCooldownCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setEjector(_) => {
                    <setEjectorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setOperatorSetParams(_) => {
                    <setOperatorSetParamsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::socketRegistry(_) => {
                    <socketRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::stakeRegistry(_) => {
                    <stakeRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::supportsAVS(_) => {
                    <supportsAVSCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::unpause(_) => <unpauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::updateOperators(_) => {
                    <updateOperatorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateOperatorsForQuorum(_) => {
                    <updateOperatorsForQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateSocket(_) => {
                    <updateSocketCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::version(_) => <versionCall as alloy_sol_types::SolCall>::SELECTOR,
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
            ) -> alloy_sol_types::Result<
                SlashingRegistryCoordinatorCalls,
            >] = &[
                {
                    fn updateOperators(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <updateOperatorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::updateOperators)
                    }
                    updateOperators
                },
                {
                    fn getQuorumBitmapHistoryLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <getQuorumBitmapHistoryLengthCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorCalls::getQuorumBitmapHistoryLength,
                            )
                    }
                    getQuorumBitmapHistoryLength
                },
                {
                    fn getQuorumBitmapAtBlockNumberByIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <getQuorumBitmapAtBlockNumberByIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorCalls::getQuorumBitmapAtBlockNumberByIndex,
                            )
                    }
                    getQuorumBitmapAtBlockNumberByIndex
                },
                {
                    fn churnApprover(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <churnApproverCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::churnApprover)
                    }
                    churnApprover
                },
                {
                    fn updateSocket(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <updateSocketCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::updateSocket)
                    }
                    updateSocket
                },
                {
                    fn setEjectionCooldown(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <setEjectionCooldownCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::setEjectionCooldown)
                    }
                    setEjectionCooldown
                },
                {
                    fn lastEjectionTimestamp(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <lastEjectionTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::lastEjectionTimestamp)
                    }
                    lastEjectionTimestamp
                },
                {
                    fn getOperatorId(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <getOperatorIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::getOperatorId)
                    }
                    getOperatorId
                },
                {
                    fn pause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorCalls::pause)
                    }
                    pause
                },
                {
                    fn isChurnApproverSaltUsed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <isChurnApproverSaltUsedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::isChurnApproverSaltUsed)
                    }
                    isChurnApproverSaltUsed
                },
                {
                    fn getQuorumBitmapUpdateByIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <getQuorumBitmapUpdateByIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorCalls::getQuorumBitmapUpdateByIndex,
                            )
                    }
                    getQuorumBitmapUpdateByIndex
                },
                {
                    fn quorumUpdateBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <quorumUpdateBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::quorumUpdateBlockNumber)
                    }
                    quorumUpdateBlockNumber
                },
                {
                    fn ejector(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <ejectorCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorCalls::ejector)
                    }
                    ejector
                },
                {
                    fn getOperatorFromId(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <getOperatorFromIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::getOperatorFromId)
                    }
                    getOperatorFromId
                },
                {
                    fn setChurnApprover(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <setChurnApproverCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::setChurnApprover)
                    }
                    setChurnApprover
                },
                {
                    fn setEjector(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <setEjectorCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorCalls::setEjector)
                    }
                    setEjector
                },
                {
                    fn deregisterOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::deregisterOperator)
                    }
                    deregisterOperator
                },
                {
                    fn pubkeyRegistrationMessageHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <pubkeyRegistrationMessageHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorCalls::pubkeyRegistrationMessageHash,
                            )
                    }
                    pubkeyRegistrationMessageHash
                },
                {
                    fn createSlashableStakeQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <createSlashableStakeQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorCalls::createSlashableStakeQuorum,
                            )
                    }
                    createSlashableStakeQuorum
                },
                {
                    fn updateOperatorsForQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <updateOperatorsForQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::updateOperatorsForQuorum)
                    }
                    updateOperatorsForQuorum
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorCalls::initialize)
                    }
                    initialize
                },
                {
                    fn version(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorCalls::version)
                    }
                    version
                },
                {
                    fn getOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <getOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::getOperator)
                    }
                    getOperator
                },
                {
                    fn pauseAll(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <pauseAllCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorCalls::pauseAll)
                    }
                    pauseAll
                },
                {
                    fn paused_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <paused_0Call as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorCalls::paused_0)
                    }
                    paused_0
                },
                {
                    fn setOperatorSetParams(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <setOperatorSetParamsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::setOperatorSetParams)
                    }
                    setOperatorSetParams
                },
                {
                    fn paused_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <paused_1Call as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorCalls::paused_1)
                    }
                    paused_1
                },
                {
                    fn blsApkRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <blsApkRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::blsApkRegistry)
                    }
                    blsApkRegistry
                },
                {
                    fn stakeRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <stakeRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::stakeRegistry)
                    }
                    stakeRegistry
                },
                {
                    fn ejectOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <ejectOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::ejectOperator)
                    }
                    ejectOperator
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn calculatePubkeyRegistrationMessageHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <calculatePubkeyRegistrationMessageHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorCalls::calculatePubkeyRegistrationMessageHash,
                            )
                    }
                    calculatePubkeyRegistrationMessageHash
                },
                {
                    fn createTotalDelegatedStakeQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <createTotalDelegatedStakeQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorCalls::createTotalDelegatedStakeQuorum,
                            )
                    }
                    createTotalDelegatedStakeQuorum
                },
                {
                    fn eip712Domain(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <eip712DomainCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::eip712Domain)
                    }
                    eip712Domain
                },
                {
                    fn calculateOperatorChurnApprovalDigestHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <calculateOperatorChurnApprovalDigestHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorCalls::calculateOperatorChurnApprovalDigestHash,
                            )
                    }
                    calculateOperatorChurnApprovalDigestHash
                },
                {
                    fn getCurrentQuorumBitmap(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <getCurrentQuorumBitmapCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::getCurrentQuorumBitmap)
                    }
                    getCurrentQuorumBitmap
                },
                {
                    fn pauserRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <pauserRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::pauserRegistry)
                    }
                    pauserRegistry
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorCalls::owner)
                    }
                    owner
                },
                {
                    fn quorumCount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <quorumCountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::quorumCount)
                    }
                    quorumCount
                },
                {
                    fn indexRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <indexRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::indexRegistry)
                    }
                    indexRegistry
                },
                {
                    fn PUBKEY_REGISTRATION_TYPEHASH(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <PUBKEY_REGISTRATION_TYPEHASHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorCalls::PUBKEY_REGISTRATION_TYPEHASH,
                            )
                    }
                    PUBKEY_REGISTRATION_TYPEHASH
                },
                {
                    fn setAVS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <setAVSCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorCalls::setAVS)
                    }
                    setAVS
                },
                {
                    fn ejectionCooldown(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <ejectionCooldownCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::ejectionCooldown)
                    }
                    ejectionCooldown
                },
                {
                    fn supportsAVS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <supportsAVSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::supportsAVS)
                    }
                    supportsAVS
                },
                {
                    fn getQuorumBitmapIndicesAtBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <getQuorumBitmapIndicesAtBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorCalls::getQuorumBitmapIndicesAtBlockNumber,
                            )
                    }
                    getQuorumBitmapIndicesAtBlockNumber
                },
                {
                    fn registerOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <registerOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::registerOperator)
                    }
                    registerOperator
                },
                {
                    fn OPERATOR_CHURN_APPROVAL_TYPEHASH(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <OPERATOR_CHURN_APPROVAL_TYPEHASHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorCalls::OPERATOR_CHURN_APPROVAL_TYPEHASH,
                            )
                    }
                    OPERATOR_CHURN_APPROVAL_TYPEHASH
                },
                {
                    fn allocationManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <allocationManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::allocationManager)
                    }
                    allocationManager
                },
                {
                    fn avs(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <avsCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorCalls::avs)
                    }
                    avs
                },
                {
                    fn getOperatorSetParams(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <getOperatorSetParamsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::getOperatorSetParams)
                    }
                    getOperatorSetParams
                },
                {
                    fn socketRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <socketRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::socketRegistry)
                    }
                    socketRegistry
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn domainSeparator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <domainSeparatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::domainSeparator)
                    }
                    domainSeparator
                },
                {
                    fn unpause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorCalls::unpause)
                    }
                    unpause
                },
                {
                    fn getOperatorStatus(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorCalls>
                    {
                        <getOperatorStatusCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorCalls::getOperatorStatus)
                    }
                    getOperatorStatus
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(alloy_sol_types::Error::unknown_selector(
                    <Self as alloy_sol_types::SolInterface>::NAME,
                    selector,
                ));
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::OPERATOR_CHURN_APPROVAL_TYPEHASH(inner) => {
                    <OPERATOR_CHURN_APPROVAL_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::PUBKEY_REGISTRATION_TYPEHASH(inner) => {
                    <PUBKEY_REGISTRATION_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::allocationManager(inner) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::avs(inner) => {
                    <avsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::blsApkRegistry(inner) => {
                    <blsApkRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::calculateOperatorChurnApprovalDigestHash(inner) => {
                    <calculateOperatorChurnApprovalDigestHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::calculatePubkeyRegistrationMessageHash(inner) => {
                    <calculatePubkeyRegistrationMessageHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::churnApprover(inner) => {
                    <churnApproverCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::createSlashableStakeQuorum(inner) => {
                    <createSlashableStakeQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::createTotalDelegatedStakeQuorum(inner) => {
                    <createTotalDelegatedStakeQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::deregisterOperator(inner) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::domainSeparator(inner) => {
                    <domainSeparatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::eip712Domain(inner) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ejectOperator(inner) => {
                    <ejectOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ejectionCooldown(inner) => {
                    <ejectionCooldownCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ejector(inner) => {
                    <ejectorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getCurrentQuorumBitmap(inner) => {
                    <getCurrentQuorumBitmapCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperator(inner) => {
                    <getOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorFromId(inner) => {
                    <getOperatorFromIdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorId(inner) => {
                    <getOperatorIdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorSetParams(inner) => {
                    <getOperatorSetParamsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorStatus(inner) => {
                    <getOperatorStatusCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getQuorumBitmapAtBlockNumberByIndex(inner) => {
                    <getQuorumBitmapAtBlockNumberByIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getQuorumBitmapHistoryLength(inner) => {
                    <getQuorumBitmapHistoryLengthCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getQuorumBitmapIndicesAtBlockNumber(inner) => {
                    <getQuorumBitmapIndicesAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getQuorumBitmapUpdateByIndex(inner) => {
                    <getQuorumBitmapUpdateByIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::indexRegistry(inner) => {
                    <indexRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isChurnApproverSaltUsed(inner) => {
                    <isChurnApproverSaltUsedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::lastEjectionTimestamp(inner) => {
                    <lastEjectionTimestampCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::pause(inner) => {
                    <pauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::pauseAll(inner) => {
                    <pauseAllCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::paused_0(inner) => {
                    <paused_0Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::paused_1(inner) => {
                    <paused_1Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::pauserRegistry(inner) => {
                    <pauserRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::pubkeyRegistrationMessageHash(inner) => {
                    <pubkeyRegistrationMessageHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::quorumCount(inner) => {
                    <quorumCountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::quorumUpdateBlockNumber(inner) => {
                    <quorumUpdateBlockNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::registerOperator(inner) => {
                    <registerOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setAVS(inner) => {
                    <setAVSCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::setChurnApprover(inner) => {
                    <setChurnApproverCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setEjectionCooldown(inner) => {
                    <setEjectionCooldownCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setEjector(inner) => {
                    <setEjectorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::setOperatorSetParams(inner) => {
                    <setOperatorSetParamsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::socketRegistry(inner) => {
                    <socketRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::stakeRegistry(inner) => {
                    <stakeRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::supportsAVS(inner) => {
                    <supportsAVSCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::updateOperators(inner) => {
                    <updateOperatorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateOperatorsForQuorum(inner) => {
                    <updateOperatorsForQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateSocket(inner) => {
                    <updateSocketCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::version(inner) => {
                    <versionCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::OPERATOR_CHURN_APPROVAL_TYPEHASH(inner) => {
                    <OPERATOR_CHURN_APPROVAL_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::PUBKEY_REGISTRATION_TYPEHASH(inner) => {
                    <PUBKEY_REGISTRATION_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::allocationManager(inner) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::avs(inner) => {
                    <avsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::blsApkRegistry(inner) => {
                    <blsApkRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::calculateOperatorChurnApprovalDigestHash(inner) => {
                    <calculateOperatorChurnApprovalDigestHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::calculatePubkeyRegistrationMessageHash(inner) => {
                    <calculatePubkeyRegistrationMessageHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::churnApprover(inner) => {
                    <churnApproverCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::createSlashableStakeQuorum(inner) => {
                    <createSlashableStakeQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::createTotalDelegatedStakeQuorum(inner) => {
                    <createTotalDelegatedStakeQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::deregisterOperator(inner) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::domainSeparator(inner) => {
                    <domainSeparatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::eip712Domain(inner) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ejectOperator(inner) => {
                    <ejectOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ejectionCooldown(inner) => {
                    <ejectionCooldownCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ejector(inner) => {
                    <ejectorCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getCurrentQuorumBitmap(inner) => {
                    <getCurrentQuorumBitmapCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperator(inner) => {
                    <getOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorFromId(inner) => {
                    <getOperatorFromIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorId(inner) => {
                    <getOperatorIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorSetParams(inner) => {
                    <getOperatorSetParamsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getQuorumBitmapAtBlockNumberByIndex(inner) => {
                    <getQuorumBitmapAtBlockNumberByIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getQuorumBitmapHistoryLength(inner) => {
                    <getQuorumBitmapHistoryLengthCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getQuorumBitmapIndicesAtBlockNumber(inner) => {
                    <getQuorumBitmapIndicesAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getQuorumBitmapUpdateByIndex(inner) => {
                    <getQuorumBitmapUpdateByIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::indexRegistry(inner) => {
                    <indexRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::isChurnApproverSaltUsed(inner) => {
                    <isChurnApproverSaltUsedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::lastEjectionTimestamp(inner) => {
                    <lastEjectionTimestampCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::pause(inner) => {
                    <pauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::pauseAll(inner) => {
                    <pauseAllCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::paused_0(inner) => {
                    <paused_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::paused_1(inner) => {
                    <paused_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::pauserRegistry(inner) => {
                    <pauserRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::pubkeyRegistrationMessageHash(inner) => {
                    <pubkeyRegistrationMessageHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::quorumCount(inner) => {
                    <quorumCountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::quorumUpdateBlockNumber(inner) => {
                    <quorumUpdateBlockNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::registerOperator(inner) => {
                    <registerOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setAVS(inner) => {
                    <setAVSCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::setChurnApprover(inner) => {
                    <setChurnApproverCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setEjectionCooldown(inner) => {
                    <setEjectionCooldownCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setEjector(inner) => {
                    <setEjectorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setOperatorSetParams(inner) => {
                    <setOperatorSetParamsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::socketRegistry(inner) => {
                    <socketRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::stakeRegistry(inner) => {
                    <stakeRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::supportsAVS(inner) => {
                    <supportsAVSCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::updateOperators(inner) => {
                    <updateOperatorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateOperatorsForQuorum(inner) => {
                    <updateOperatorsForQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateSocket(inner) => {
                    <updateSocketCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::version(inner) => {
                    <versionCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`SlashingRegistryCoordinator`](self) custom errors.
    pub enum SlashingRegistryCoordinatorErrors {
        #[allow(missing_docs)]
        AlreadyRegisteredForQuorums(AlreadyRegisteredForQuorums),
        #[allow(missing_docs)]
        BitmapCannotBeZero(BitmapCannotBeZero),
        #[allow(missing_docs)]
        BitmapEmpty(BitmapEmpty),
        #[allow(missing_docs)]
        BitmapUpdateIsAfterBlockNumber(BitmapUpdateIsAfterBlockNumber),
        #[allow(missing_docs)]
        BitmapValueTooLarge(BitmapValueTooLarge),
        #[allow(missing_docs)]
        BytesArrayLengthTooLong(BytesArrayLengthTooLong),
        #[allow(missing_docs)]
        BytesArrayNotOrdered(BytesArrayNotOrdered),
        #[allow(missing_docs)]
        CannotChurnSelf(CannotChurnSelf),
        #[allow(missing_docs)]
        CannotKickOperatorAboveThreshold(CannotKickOperatorAboveThreshold),
        #[allow(missing_docs)]
        CannotReregisterYet(CannotReregisterYet),
        #[allow(missing_docs)]
        ChurnApproverSaltUsed(ChurnApproverSaltUsed),
        #[allow(missing_docs)]
        CurrentlyPaused(CurrentlyPaused),
        #[allow(missing_docs)]
        ExpModFailed(ExpModFailed),
        #[allow(missing_docs)]
        InputAddressZero(InputAddressZero),
        #[allow(missing_docs)]
        InputLengthMismatch(InputLengthMismatch),
        #[allow(missing_docs)]
        InsufficientStakeForChurn(InsufficientStakeForChurn),
        #[allow(missing_docs)]
        InvalidAVS(InvalidAVS),
        #[allow(missing_docs)]
        InvalidNewPausedStatus(InvalidNewPausedStatus),
        #[allow(missing_docs)]
        InvalidRegistrationType(InvalidRegistrationType),
        #[allow(missing_docs)]
        InvalidShortString(InvalidShortString),
        #[allow(missing_docs)]
        InvalidSignature(InvalidSignature),
        #[allow(missing_docs)]
        LookAheadPeriodTooLong(LookAheadPeriodTooLong),
        #[allow(missing_docs)]
        MaxOperatorCountReached(MaxOperatorCountReached),
        #[allow(missing_docs)]
        MaxQuorumsReached(MaxQuorumsReached),
        #[allow(missing_docs)]
        NextBitmapUpdateIsBeforeBlockNumber(NextBitmapUpdateIsBeforeBlockNumber),
        #[allow(missing_docs)]
        NotRegistered(NotRegistered),
        #[allow(missing_docs)]
        NotRegisteredForQuorum(NotRegisteredForQuorum),
        #[allow(missing_docs)]
        NotSorted(NotSorted),
        #[allow(missing_docs)]
        OnlyAllocationManager(OnlyAllocationManager),
        #[allow(missing_docs)]
        OnlyEjector(OnlyEjector),
        #[allow(missing_docs)]
        OnlyPauser(OnlyPauser),
        #[allow(missing_docs)]
        OnlyUnpauser(OnlyUnpauser),
        #[allow(missing_docs)]
        OperatorNotRegistered(OperatorNotRegistered),
        #[allow(missing_docs)]
        QuorumDoesNotExist(QuorumDoesNotExist),
        #[allow(missing_docs)]
        QuorumOperatorCountMismatch(QuorumOperatorCountMismatch),
        #[allow(missing_docs)]
        SignatureExpired(SignatureExpired),
        #[allow(missing_docs)]
        StringTooLong(StringTooLong),
    }
    #[automatically_derived]
    impl SlashingRegistryCoordinatorErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [8u8, 25u8, 189u8, 205u8],
            [12u8, 104u8, 22u8, 205u8],
            [19u8, 202u8, 70u8, 87u8],
            [35u8, 216u8, 113u8, 165u8],
            [37u8, 236u8, 108u8, 31u8],
            [47u8, 81u8, 6u8, 228u8],
            [48u8, 90u8, 39u8, 169u8],
            [50u8, 208u8, 206u8, 250u8],
            [53u8, 75u8, 184u8, 171u8],
            [60u8, 184u8, 156u8, 151u8],
            [76u8, 68u8, 153u8, 93u8],
            [102u8, 229u8, 101u8, 223u8],
            [108u8, 177u8, 154u8, 255u8],
            [115u8, 99u8, 33u8, 118u8],
            [117u8, 223u8, 81u8, 220u8],
            [121u8, 72u8, 33u8, 255u8],
            [128u8, 200u8, 131u8, 72u8],
            [132u8, 10u8, 72u8, 213u8],
            [139u8, 170u8, 87u8, 159u8],
            [142u8, 90u8, 238u8, 231u8],
            [170u8, 173u8, 19u8, 247u8],
            [171u8, 164u8, 115u8, 57u8],
            [172u8, 45u8, 22u8, 130u8],
            [177u8, 135u8, 232u8, 105u8],
            [179u8, 81u8, 43u8, 12u8],
            [186u8, 80u8, 249u8, 17u8],
            [187u8, 186u8, 96u8, 203u8],
            [198u8, 29u8, 202u8, 93u8],
            [198u8, 185u8, 231u8, 103u8],
            [202u8, 149u8, 115u8, 51u8],
            [208u8, 83u8, 170u8, 33u8],
            [209u8, 109u8, 80u8, 234u8],
            [213u8, 30u8, 218u8, 227u8],
            [223u8, 125u8, 253u8, 134u8],
            [230u8, 33u8, 159u8, 234u8],
            [237u8, 177u8, 86u8, 46u8],
            [251u8, 74u8, 156u8, 142u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for SlashingRegistryCoordinatorErrors {
        const NAME: &'static str = "SlashingRegistryCoordinatorErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 37usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AlreadyRegisteredForQuorums(_) => {
                    <AlreadyRegisteredForQuorums as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BitmapCannotBeZero(_) => {
                    <BitmapCannotBeZero as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BitmapEmpty(_) => <BitmapEmpty as alloy_sol_types::SolError>::SELECTOR,
                Self::BitmapUpdateIsAfterBlockNumber(_) => {
                    <BitmapUpdateIsAfterBlockNumber as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BitmapValueTooLarge(_) => {
                    <BitmapValueTooLarge as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BytesArrayLengthTooLong(_) => {
                    <BytesArrayLengthTooLong as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BytesArrayNotOrdered(_) => {
                    <BytesArrayNotOrdered as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CannotChurnSelf(_) => {
                    <CannotChurnSelf as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CannotKickOperatorAboveThreshold(_) => {
                    <CannotKickOperatorAboveThreshold as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CannotReregisterYet(_) => {
                    <CannotReregisterYet as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ChurnApproverSaltUsed(_) => {
                    <ChurnApproverSaltUsed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CurrentlyPaused(_) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ExpModFailed(_) => <ExpModFailed as alloy_sol_types::SolError>::SELECTOR,
                Self::InputAddressZero(_) => {
                    <InputAddressZero as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputLengthMismatch(_) => {
                    <InputLengthMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientStakeForChurn(_) => {
                    <InsufficientStakeForChurn as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidAVS(_) => <InvalidAVS as alloy_sol_types::SolError>::SELECTOR,
                Self::InvalidNewPausedStatus(_) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidRegistrationType(_) => {
                    <InvalidRegistrationType as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidShortString(_) => {
                    <InvalidShortString as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidSignature(_) => {
                    <InvalidSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::LookAheadPeriodTooLong(_) => {
                    <LookAheadPeriodTooLong as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MaxOperatorCountReached(_) => {
                    <MaxOperatorCountReached as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MaxQuorumsReached(_) => {
                    <MaxQuorumsReached as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NextBitmapUpdateIsBeforeBlockNumber(_) => {
                    <NextBitmapUpdateIsBeforeBlockNumber as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotRegistered(_) => <NotRegistered as alloy_sol_types::SolError>::SELECTOR,
                Self::NotRegisteredForQuorum(_) => {
                    <NotRegisteredForQuorum as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotSorted(_) => <NotSorted as alloy_sol_types::SolError>::SELECTOR,
                Self::OnlyAllocationManager(_) => {
                    <OnlyAllocationManager as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyEjector(_) => <OnlyEjector as alloy_sol_types::SolError>::SELECTOR,
                Self::OnlyPauser(_) => <OnlyPauser as alloy_sol_types::SolError>::SELECTOR,
                Self::OnlyUnpauser(_) => <OnlyUnpauser as alloy_sol_types::SolError>::SELECTOR,
                Self::OperatorNotRegistered(_) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::SELECTOR
                }
                Self::QuorumDoesNotExist(_) => {
                    <QuorumDoesNotExist as alloy_sol_types::SolError>::SELECTOR
                }
                Self::QuorumOperatorCountMismatch(_) => {
                    <QuorumOperatorCountMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SignatureExpired(_) => {
                    <SignatureExpired as alloy_sol_types::SolError>::SELECTOR
                }
                Self::StringTooLong(_) => <StringTooLong as alloy_sol_types::SolError>::SELECTOR,
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
            ) -> alloy_sol_types::Result<
                SlashingRegistryCoordinatorErrors,
            >] = &[
                {
                    fn SignatureExpired(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <SignatureExpired as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::SignatureExpired)
                    }
                    SignatureExpired
                },
                {
                    fn AlreadyRegisteredForQuorums(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <AlreadyRegisteredForQuorums as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::AlreadyRegisteredForQuorums)
                    }
                    AlreadyRegisteredForQuorums
                },
                {
                    fn BitmapEmpty(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <BitmapEmpty as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorErrors::BitmapEmpty)
                    }
                    BitmapEmpty
                },
                {
                    fn OnlyAllocationManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <OnlyAllocationManager as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::OnlyAllocationManager)
                    }
                    OnlyAllocationManager
                },
                {
                    fn OperatorNotRegistered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <OperatorNotRegistered as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::OperatorNotRegistered)
                    }
                    OperatorNotRegistered
                },
                {
                    fn LookAheadPeriodTooLong(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <LookAheadPeriodTooLong as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::LookAheadPeriodTooLong)
                    }
                    LookAheadPeriodTooLong
                },
                {
                    fn StringTooLong(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <StringTooLong as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorErrors::StringTooLong)
                    }
                    StringTooLong
                },
                {
                    fn CannotReregisterYet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <CannotReregisterYet as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::CannotReregisterYet)
                    }
                    CannotReregisterYet
                },
                {
                    fn InvalidRegistrationType(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <InvalidRegistrationType as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::InvalidRegistrationType)
                    }
                    InvalidRegistrationType
                },
                {
                    fn MaxQuorumsReached(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <MaxQuorumsReached as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::MaxQuorumsReached)
                    }
                    MaxQuorumsReached
                },
                {
                    fn InsufficientStakeForChurn(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <InsufficientStakeForChurn as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::InsufficientStakeForChurn)
                    }
                    InsufficientStakeForChurn
                },
                {
                    fn InvalidAVS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <InvalidAVS as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorErrors::InvalidAVS)
                    }
                    InvalidAVS
                },
                {
                    fn BitmapUpdateIsAfterBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <BitmapUpdateIsAfterBlockNumber as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorErrors::BitmapUpdateIsAfterBlockNumber,
                            )
                    }
                    BitmapUpdateIsAfterBlockNumber
                },
                {
                    fn InputAddressZero(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <InputAddressZero as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::InputAddressZero)
                    }
                    InputAddressZero
                },
                {
                    fn OnlyPauser(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <OnlyPauser as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorErrors::OnlyPauser)
                    }
                    OnlyPauser
                },
                {
                    fn OnlyUnpauser(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <OnlyUnpauser as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorErrors::OnlyUnpauser)
                    }
                    OnlyUnpauser
                },
                {
                    fn BytesArrayNotOrdered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <BytesArrayNotOrdered as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::BytesArrayNotOrdered)
                    }
                    BytesArrayNotOrdered
                },
                {
                    fn CurrentlyPaused(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <CurrentlyPaused as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::CurrentlyPaused)
                    }
                    CurrentlyPaused
                },
                {
                    fn InvalidSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <InvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::InvalidSignature)
                    }
                    InvalidSignature
                },
                {
                    fn QuorumOperatorCountMismatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <QuorumOperatorCountMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::QuorumOperatorCountMismatch)
                    }
                    QuorumOperatorCountMismatch
                },
                {
                    fn InputLengthMismatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <InputLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::InputLengthMismatch)
                    }
                    InputLengthMismatch
                },
                {
                    fn NotRegistered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <NotRegistered as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorErrors::NotRegistered)
                    }
                    NotRegistered
                },
                {
                    fn CannotChurnSelf(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <CannotChurnSelf as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::CannotChurnSelf)
                    }
                    CannotChurnSelf
                },
                {
                    fn CannotKickOperatorAboveThreshold(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <CannotKickOperatorAboveThreshold as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorErrors::CannotKickOperatorAboveThreshold,
                            )
                    }
                    CannotKickOperatorAboveThreshold
                },
                {
                    fn InvalidShortString(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <InvalidShortString as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::InvalidShortString)
                    }
                    InvalidShortString
                },
                {
                    fn NotSorted(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <NotSorted as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorErrors::NotSorted)
                    }
                    NotSorted
                },
                {
                    fn NextBitmapUpdateIsBeforeBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <NextBitmapUpdateIsBeforeBlockNumber as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                SlashingRegistryCoordinatorErrors::NextBitmapUpdateIsBeforeBlockNumber,
                            )
                    }
                    NextBitmapUpdateIsBeforeBlockNumber
                },
                {
                    fn InvalidNewPausedStatus(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::InvalidNewPausedStatus)
                    }
                    InvalidNewPausedStatus
                },
                {
                    fn MaxOperatorCountReached(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <MaxOperatorCountReached as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::MaxOperatorCountReached)
                    }
                    MaxOperatorCountReached
                },
                {
                    fn BitmapValueTooLarge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <BitmapValueTooLarge as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::BitmapValueTooLarge)
                    }
                    BitmapValueTooLarge
                },
                {
                    fn NotRegisteredForQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <NotRegisteredForQuorum as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::NotRegisteredForQuorum)
                    }
                    NotRegisteredForQuorum
                },
                {
                    fn BitmapCannotBeZero(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <BitmapCannotBeZero as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::BitmapCannotBeZero)
                    }
                    BitmapCannotBeZero
                },
                {
                    fn ExpModFailed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <ExpModFailed as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorErrors::ExpModFailed)
                    }
                    ExpModFailed
                },
                {
                    fn ChurnApproverSaltUsed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <ChurnApproverSaltUsed as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::ChurnApproverSaltUsed)
                    }
                    ChurnApproverSaltUsed
                },
                {
                    fn QuorumDoesNotExist(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <QuorumDoesNotExist as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::QuorumDoesNotExist)
                    }
                    QuorumDoesNotExist
                },
                {
                    fn OnlyEjector(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <OnlyEjector as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(SlashingRegistryCoordinatorErrors::OnlyEjector)
                    }
                    OnlyEjector
                },
                {
                    fn BytesArrayLengthTooLong(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlashingRegistryCoordinatorErrors>
                    {
                        <BytesArrayLengthTooLong as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SlashingRegistryCoordinatorErrors::BytesArrayLengthTooLong)
                    }
                    BytesArrayLengthTooLong
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(alloy_sol_types::Error::unknown_selector(
                    <Self as alloy_sol_types::SolInterface>::NAME,
                    selector,
                ));
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::AlreadyRegisteredForQuorums(inner) => {
                    <AlreadyRegisteredForQuorums as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BitmapCannotBeZero(inner) => {
                    <BitmapCannotBeZero as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BitmapEmpty(inner) => {
                    <BitmapEmpty as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::BitmapUpdateIsAfterBlockNumber(inner) => {
                    <BitmapUpdateIsAfterBlockNumber as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BitmapValueTooLarge(inner) => {
                    <BitmapValueTooLarge as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BytesArrayLengthTooLong(inner) => {
                    <BytesArrayLengthTooLong as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BytesArrayNotOrdered(inner) => {
                    <BytesArrayNotOrdered as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CannotChurnSelf(inner) => {
                    <CannotChurnSelf as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CannotKickOperatorAboveThreshold(inner) => {
                    <CannotKickOperatorAboveThreshold as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CannotReregisterYet(inner) => {
                    <CannotReregisterYet as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ChurnApproverSaltUsed(inner) => {
                    <ChurnApproverSaltUsed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CurrentlyPaused(inner) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ExpModFailed(inner) => {
                    <ExpModFailed as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InputAddressZero(inner) => {
                    <InputAddressZero as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InputLengthMismatch(inner) => {
                    <InputLengthMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InsufficientStakeForChurn(inner) => {
                    <InsufficientStakeForChurn as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidAVS(inner) => {
                    <InvalidAVS as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidNewPausedStatus(inner) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidRegistrationType(inner) => {
                    <InvalidRegistrationType as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidShortString(inner) => {
                    <InvalidShortString as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::LookAheadPeriodTooLong(inner) => {
                    <LookAheadPeriodTooLong as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MaxOperatorCountReached(inner) => {
                    <MaxOperatorCountReached as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MaxQuorumsReached(inner) => {
                    <MaxQuorumsReached as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NextBitmapUpdateIsBeforeBlockNumber(inner) => {
                    <NextBitmapUpdateIsBeforeBlockNumber as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotRegistered(inner) => {
                    <NotRegistered as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::NotRegisteredForQuorum(inner) => {
                    <NotRegisteredForQuorum as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotSorted(inner) => {
                    <NotSorted as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyAllocationManager(inner) => {
                    <OnlyAllocationManager as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlyEjector(inner) => {
                    <OnlyEjector as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyPauser(inner) => {
                    <OnlyPauser as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyUnpauser(inner) => {
                    <OnlyUnpauser as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OperatorNotRegistered(inner) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::QuorumDoesNotExist(inner) => {
                    <QuorumDoesNotExist as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::QuorumOperatorCountMismatch(inner) => {
                    <QuorumOperatorCountMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SignatureExpired(inner) => {
                    <SignatureExpired as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::StringTooLong(inner) => {
                    <StringTooLong as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::AlreadyRegisteredForQuorums(inner) => {
                    <AlreadyRegisteredForQuorums as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BitmapCannotBeZero(inner) => {
                    <BitmapCannotBeZero as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BitmapEmpty(inner) => {
                    <BitmapEmpty as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BitmapUpdateIsAfterBlockNumber(inner) => {
                    <BitmapUpdateIsAfterBlockNumber as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BitmapValueTooLarge(inner) => {
                    <BitmapValueTooLarge as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BytesArrayLengthTooLong(inner) => {
                    <BytesArrayLengthTooLong as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BytesArrayNotOrdered(inner) => {
                    <BytesArrayNotOrdered as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CannotChurnSelf(inner) => {
                    <CannotChurnSelf as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CannotKickOperatorAboveThreshold(inner) => {
                    <CannotKickOperatorAboveThreshold as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CannotReregisterYet(inner) => {
                    <CannotReregisterYet as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ChurnApproverSaltUsed(inner) => {
                    <ChurnApproverSaltUsed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CurrentlyPaused(inner) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ExpModFailed(inner) => {
                    <ExpModFailed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InputAddressZero(inner) => {
                    <InputAddressZero as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InputLengthMismatch(inner) => {
                    <InputLengthMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InsufficientStakeForChurn(inner) => {
                    <InsufficientStakeForChurn as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidAVS(inner) => {
                    <InvalidAVS as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidNewPausedStatus(inner) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidRegistrationType(inner) => {
                    <InvalidRegistrationType as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidShortString(inner) => {
                    <InvalidShortString as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::LookAheadPeriodTooLong(inner) => {
                    <LookAheadPeriodTooLong as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MaxOperatorCountReached(inner) => {
                    <MaxOperatorCountReached as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MaxQuorumsReached(inner) => {
                    <MaxQuorumsReached as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NextBitmapUpdateIsBeforeBlockNumber(inner) => {
                    <NextBitmapUpdateIsBeforeBlockNumber as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotRegistered(inner) => {
                    <NotRegistered as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotRegisteredForQuorum(inner) => {
                    <NotRegisteredForQuorum as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotSorted(inner) => {
                    <NotSorted as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OnlyAllocationManager(inner) => {
                    <OnlyAllocationManager as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OnlyEjector(inner) => {
                    <OnlyEjector as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OnlyPauser(inner) => {
                    <OnlyPauser as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OnlyUnpauser(inner) => {
                    <OnlyUnpauser as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OperatorNotRegistered(inner) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::QuorumDoesNotExist(inner) => {
                    <QuorumDoesNotExist as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::QuorumOperatorCountMismatch(inner) => {
                    <QuorumOperatorCountMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SignatureExpired(inner) => {
                    <SignatureExpired as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::StringTooLong(inner) => {
                    <StringTooLong as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`SlashingRegistryCoordinator`](self) events.
    pub enum SlashingRegistryCoordinatorEvents {
        #[allow(missing_docs)]
        AVSUpdated(AVSUpdated),
        #[allow(missing_docs)]
        ChurnApproverUpdated(ChurnApproverUpdated),
        #[allow(missing_docs)]
        EIP712DomainChanged(EIP712DomainChanged),
        #[allow(missing_docs)]
        EjectionCooldownUpdated(EjectionCooldownUpdated),
        #[allow(missing_docs)]
        EjectorUpdated(EjectorUpdated),
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        OperatorDeregistered(OperatorDeregistered),
        #[allow(missing_docs)]
        OperatorRegistered(OperatorRegistered),
        #[allow(missing_docs)]
        OperatorSetParamsUpdated(OperatorSetParamsUpdated),
        #[allow(missing_docs)]
        OperatorSocketUpdate(OperatorSocketUpdate),
        #[allow(missing_docs)]
        OwnershipTransferred(OwnershipTransferred),
        #[allow(missing_docs)]
        Paused(Paused),
        #[allow(missing_docs)]
        QuorumBlockNumberUpdated(QuorumBlockNumberUpdated),
        #[allow(missing_docs)]
        QuorumCreated(QuorumCreated),
        #[allow(missing_docs)]
        Unpaused(Unpaused),
    }
    #[automatically_derived]
    impl SlashingRegistryCoordinatorEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                10u8, 99u8, 135u8, 201u8, 234u8, 54u8, 40u8, 184u8, 138u8, 99u8, 59u8, 180u8,
                243u8, 177u8, 81u8, 119u8, 15u8, 112u8, 8u8, 81u8, 23u8, 161u8, 95u8, 155u8, 243u8,
                120u8, 124u8, 218u8, 83u8, 241u8, 61u8, 49u8,
            ],
            [
                49u8, 84u8, 87u8, 216u8, 168u8, 254u8, 96u8, 240u8, 74u8, 241u8, 124u8, 22u8,
                226u8, 245u8, 165u8, 225u8, 219u8, 97u8, 43u8, 49u8, 100u8, 142u8, 88u8, 3u8, 3u8,
                96u8, 117u8, 158u8, 248u8, 243u8, 82u8, 140u8,
            ],
            [
                53u8, 130u8, 209u8, 130u8, 142u8, 38u8, 191u8, 86u8, 189u8, 128u8, 21u8, 2u8,
                188u8, 2u8, 26u8, 192u8, 188u8, 138u8, 251u8, 87u8, 200u8, 38u8, 228u8, 152u8,
                107u8, 69u8, 89u8, 60u8, 143u8, 173u8, 56u8, 156u8,
            ],
            [
                57u8, 111u8, 220u8, 177u8, 128u8, 203u8, 15u8, 234u8, 38u8, 146u8, 129u8, 19u8,
                251u8, 15u8, 209u8, 195u8, 84u8, 152u8, 99u8, 249u8, 205u8, 86u8, 62u8, 106u8,
                24u8, 79u8, 29u8, 87u8, 129u8, 22u8, 200u8, 228u8,
            ],
            [
                62u8, 230u8, 254u8, 141u8, 84u8, 97u8, 2u8, 68u8, 195u8, 233u8, 211u8, 192u8,
                102u8, 174u8, 74u8, 238u8, 153u8, 120u8, 132u8, 170u8, 40u8, 241u8, 6u8, 22u8,
                174u8, 130u8, 25u8, 37u8, 64u8, 19u8, 24u8, 172u8,
            ],
            [
                70u8, 7u8, 125u8, 85u8, 51u8, 7u8, 99u8, 241u8, 98u8, 105u8, 253u8, 117u8, 229u8,
                118u8, 22u8, 99u8, 244u8, 25u8, 45u8, 39u8, 145u8, 116u8, 124u8, 1u8, 137u8, 177u8,
                106u8, 211u8, 29u8, 176u8, 125u8, 180u8,
            ],
            [
                127u8, 38u8, 184u8, 63u8, 249u8, 110u8, 31u8, 43u8, 106u8, 104u8, 47u8, 19u8, 56u8,
                82u8, 246u8, 121u8, 138u8, 9u8, 196u8, 101u8, 218u8, 149u8, 146u8, 20u8, 96u8,
                206u8, 251u8, 56u8, 71u8, 64u8, 36u8, 152u8,
            ],
            [
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8, 31u8, 208u8,
                164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8, 218u8, 175u8, 227u8,
                180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ],
            [
                143u8, 48u8, 171u8, 9u8, 244u8, 58u8, 108u8, 21u8, 125u8, 127u8, 206u8, 126u8,
                10u8, 19u8, 192u8, 3u8, 4u8, 44u8, 28u8, 149u8, 232u8, 167u8, 46u8, 122u8, 20u8,
                106u8, 33u8, 192u8, 202u8, 162u8, 77u8, 201u8,
            ],
            [
                151u8, 112u8, 243u8, 202u8, 223u8, 220u8, 187u8, 111u8, 147u8, 175u8, 147u8, 94u8,
                134u8, 4u8, 113u8, 17u8, 89u8, 12u8, 55u8, 104u8, 39u8, 29u8, 35u8, 126u8, 74u8,
                43u8, 192u8, 184u8, 116u8, 190u8, 214u8, 147u8,
            ],
            [
                163u8, 72u8, 53u8, 188u8, 43u8, 103u8, 62u8, 195u8, 127u8, 207u8, 21u8, 145u8,
                169u8, 18u8, 149u8, 177u8, 99u8, 252u8, 46u8, 24u8, 30u8, 78u8, 164u8, 231u8, 51u8,
                190u8, 178u8, 125u8, 225u8, 206u8, 172u8, 76u8,
            ],
            [
                167u8, 122u8, 145u8, 190u8, 167u8, 182u8, 217u8, 90u8, 142u8, 181u8, 165u8, 72u8,
                120u8, 161u8, 217u8, 227u8, 200u8, 117u8, 226u8, 108u8, 134u8, 169u8, 183u8, 14u8,
                52u8, 32u8, 197u8, 197u8, 219u8, 25u8, 59u8, 98u8,
            ],
            [
                171u8, 64u8, 163u8, 116u8, 188u8, 81u8, 222u8, 55u8, 34u8, 0u8, 168u8, 188u8,
                152u8, 26u8, 248u8, 201u8, 236u8, 220u8, 8u8, 223u8, 218u8, 239u8, 11u8, 182u8,
                224u8, 159u8, 136u8, 243u8, 198u8, 22u8, 239u8, 61u8,
            ],
            [
                232u8, 230u8, 140u8, 239u8, 28u8, 58u8, 118u8, 30u8, 215u8, 190u8, 126u8, 132u8,
                99u8, 163u8, 117u8, 242u8, 127u8, 123u8, 195u8, 53u8, 229u8, 24u8, 36u8, 34u8,
                60u8, 172u8, 206u8, 99u8, 110u8, 197u8, 195u8, 254u8,
            ],
            [
                236u8, 41u8, 99u8, 171u8, 33u8, 193u8, 229u8, 14u8, 30u8, 88u8, 42u8, 165u8, 66u8,
                175u8, 46u8, 75u8, 247u8, 191u8, 56u8, 230u8, 225u8, 64u8, 60u8, 39u8, 180u8, 46u8,
                28u8, 93u8, 110u8, 98u8, 30u8, 170u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for SlashingRegistryCoordinatorEvents {
        const NAME: &'static str = "SlashingRegistryCoordinatorEvents";
        const COUNT: usize = 15usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<AVSUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <AVSUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::AVSUpdated)
                }
                Some(<ChurnApproverUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ChurnApproverUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::ChurnApproverUpdated)
                }
                Some(<EIP712DomainChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <EIP712DomainChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::EIP712DomainChanged)
                }
                Some(<EjectionCooldownUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <EjectionCooldownUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::EjectionCooldownUpdated)
                }
                Some(<EjectorUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <EjectorUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::EjectorUpdated)
                }
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::Initialized)
                }
                Some(<OperatorDeregistered as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorDeregistered as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorDeregistered)
                }
                Some(<OperatorRegistered as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorRegistered as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorRegistered)
                }
                Some(<OperatorSetParamsUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorSetParamsUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorSetParamsUpdated)
                }
                Some(<OperatorSocketUpdate as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorSocketUpdate as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorSocketUpdate)
                }
                Some(<OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OwnershipTransferred)
                }
                Some(<Paused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Paused as alloy_sol_types::SolEvent>::decode_raw_log(topics, data, validate)
                        .map(Self::Paused)
                }
                Some(<QuorumBlockNumberUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <QuorumBlockNumberUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::QuorumBlockNumberUpdated)
                }
                Some(<QuorumCreated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <QuorumCreated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::QuorumCreated)
                }
                Some(<Unpaused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Unpaused as alloy_sol_types::SolEvent>::decode_raw_log(topics, data, validate)
                        .map(Self::Unpaused)
                }
                _ => alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                    name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                    log: alloy_sol_types::private::Box::new(
                        alloy_sol_types::private::LogData::new_unchecked(
                            topics.to_vec(),
                            data.to_vec().into(),
                        ),
                    ),
                }),
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for SlashingRegistryCoordinatorEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::AVSUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ChurnApproverUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::EIP712DomainChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::EjectionCooldownUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::EjectorUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorDeregistered(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorRegistered(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorSetParamsUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorSocketUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Paused(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
                Self::QuorumBlockNumberUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::QuorumCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Unpaused(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::AVSUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ChurnApproverUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::EIP712DomainChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::EjectionCooldownUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::EjectorUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorDeregistered(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorRegistered(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorSetParamsUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorSocketUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Paused(inner) => alloy_sol_types::private::IntoLogData::into_log_data(inner),
                Self::QuorumBlockNumberUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::QuorumCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`SlashingRegistryCoordinator`](self) contract instance.

    See the [wrapper's documentation](`SlashingRegistryCoordinatorInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> SlashingRegistryCoordinatorInstance<T, P, N> {
        SlashingRegistryCoordinatorInstance::<T, P, N>::new(address, provider)
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
        _stakeRegistry: alloy::sol_types::private::Address,
        _blsApkRegistry: alloy::sol_types::private::Address,
        _indexRegistry: alloy::sol_types::private::Address,
        _socketRegistry: alloy::sol_types::private::Address,
        _allocationManager: alloy::sol_types::private::Address,
        _pauserRegistry: alloy::sol_types::private::Address,
        _version: alloy::sol_types::private::String,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<SlashingRegistryCoordinatorInstance<T, P, N>>,
    > {
        SlashingRegistryCoordinatorInstance::<T, P, N>::deploy(
            provider,
            _stakeRegistry,
            _blsApkRegistry,
            _indexRegistry,
            _socketRegistry,
            _allocationManager,
            _pauserRegistry,
            _version,
        )
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
        _stakeRegistry: alloy::sol_types::private::Address,
        _blsApkRegistry: alloy::sol_types::private::Address,
        _indexRegistry: alloy::sol_types::private::Address,
        _socketRegistry: alloy::sol_types::private::Address,
        _allocationManager: alloy::sol_types::private::Address,
        _pauserRegistry: alloy::sol_types::private::Address,
        _version: alloy::sol_types::private::String,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        SlashingRegistryCoordinatorInstance::<T, P, N>::deploy_builder(
            provider,
            _stakeRegistry,
            _blsApkRegistry,
            _indexRegistry,
            _socketRegistry,
            _allocationManager,
            _pauserRegistry,
            _version,
        )
    }
    /**A [`SlashingRegistryCoordinator`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`SlashingRegistryCoordinator`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct SlashingRegistryCoordinatorInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for SlashingRegistryCoordinatorInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("SlashingRegistryCoordinatorInstance")
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
    > SlashingRegistryCoordinatorInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`SlashingRegistryCoordinator`](self) contract instance.

        See the [wrapper's documentation](`SlashingRegistryCoordinatorInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
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
            _stakeRegistry: alloy::sol_types::private::Address,
            _blsApkRegistry: alloy::sol_types::private::Address,
            _indexRegistry: alloy::sol_types::private::Address,
            _socketRegistry: alloy::sol_types::private::Address,
            _allocationManager: alloy::sol_types::private::Address,
            _pauserRegistry: alloy::sol_types::private::Address,
            _version: alloy::sol_types::private::String,
        ) -> alloy_contract::Result<SlashingRegistryCoordinatorInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _stakeRegistry,
                _blsApkRegistry,
                _indexRegistry,
                _socketRegistry,
                _allocationManager,
                _pauserRegistry,
                _version,
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
            _stakeRegistry: alloy::sol_types::private::Address,
            _blsApkRegistry: alloy::sol_types::private::Address,
            _indexRegistry: alloy::sol_types::private::Address,
            _socketRegistry: alloy::sol_types::private::Address,
            _allocationManager: alloy::sol_types::private::Address,
            _pauserRegistry: alloy::sol_types::private::Address,
            _version: alloy::sol_types::private::String,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(&constructorCall {
                        _stakeRegistry,
                        _blsApkRegistry,
                        _indexRegistry,
                        _socketRegistry,
                        _allocationManager,
                        _pauserRegistry,
                        _version,
                    })[..],
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
    impl<T, P: ::core::clone::Clone, N> SlashingRegistryCoordinatorInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> SlashingRegistryCoordinatorInstance<T, P, N> {
            SlashingRegistryCoordinatorInstance {
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
    > SlashingRegistryCoordinatorInstance<T, P, N>
    {
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
        ///Creates a new call builder for the [`OPERATOR_CHURN_APPROVAL_TYPEHASH`] function.
        pub fn OPERATOR_CHURN_APPROVAL_TYPEHASH(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, OPERATOR_CHURN_APPROVAL_TYPEHASHCall, N>
        {
            self.call_builder(&OPERATOR_CHURN_APPROVAL_TYPEHASHCall {})
        }
        ///Creates a new call builder for the [`PUBKEY_REGISTRATION_TYPEHASH`] function.
        pub fn PUBKEY_REGISTRATION_TYPEHASH(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, PUBKEY_REGISTRATION_TYPEHASHCall, N> {
            self.call_builder(&PUBKEY_REGISTRATION_TYPEHASHCall {})
        }
        ///Creates a new call builder for the [`allocationManager`] function.
        pub fn allocationManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, allocationManagerCall, N> {
            self.call_builder(&allocationManagerCall {})
        }
        ///Creates a new call builder for the [`avs`] function.
        pub fn avs(&self) -> alloy_contract::SolCallBuilder<T, &P, avsCall, N> {
            self.call_builder(&avsCall {})
        }
        ///Creates a new call builder for the [`blsApkRegistry`] function.
        pub fn blsApkRegistry(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, blsApkRegistryCall, N> {
            self.call_builder(&blsApkRegistryCall {})
        }
        ///Creates a new call builder for the [`calculateOperatorChurnApprovalDigestHash`] function.
        pub fn calculateOperatorChurnApprovalDigestHash(
            &self,
            registeringOperator: alloy::sol_types::private::Address,
            registeringOperatorId: alloy::sol_types::private::FixedBytes<32>,
            operatorKickParams: alloy::sol_types::private::Vec<
                <ISlashingRegistryCoordinatorTypes::OperatorKickParam as alloy::sol_types::SolType>::RustType,
            >,
            salt: alloy::sol_types::private::FixedBytes<32>,
            expiry: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, calculateOperatorChurnApprovalDigestHashCall, N>
        {
            self.call_builder(&calculateOperatorChurnApprovalDigestHashCall {
                registeringOperator,
                registeringOperatorId,
                operatorKickParams,
                salt,
                expiry,
            })
        }
        ///Creates a new call builder for the [`calculatePubkeyRegistrationMessageHash`] function.
        pub fn calculatePubkeyRegistrationMessageHash(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, calculatePubkeyRegistrationMessageHashCall, N>
        {
            self.call_builder(&calculatePubkeyRegistrationMessageHashCall { operator })
        }
        ///Creates a new call builder for the [`churnApprover`] function.
        pub fn churnApprover(&self) -> alloy_contract::SolCallBuilder<T, &P, churnApproverCall, N> {
            self.call_builder(&churnApproverCall {})
        }
        ///Creates a new call builder for the [`createSlashableStakeQuorum`] function.
        pub fn createSlashableStakeQuorum(
            &self,
            operatorSetParams: <ISlashingRegistryCoordinatorTypes::OperatorSetParam as alloy::sol_types::SolType>::RustType,
            minimumStake: alloy::sol_types::private::primitives::aliases::U96,
            strategyParams: alloy::sol_types::private::Vec<
                <IStakeRegistryTypes::StrategyParams as alloy::sol_types::SolType>::RustType,
            >,
            lookAheadPeriod: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, createSlashableStakeQuorumCall, N> {
            self.call_builder(&createSlashableStakeQuorumCall {
                operatorSetParams,
                minimumStake,
                strategyParams,
                lookAheadPeriod,
            })
        }
        ///Creates a new call builder for the [`createTotalDelegatedStakeQuorum`] function.
        pub fn createTotalDelegatedStakeQuorum(
            &self,
            operatorSetParams: <ISlashingRegistryCoordinatorTypes::OperatorSetParam as alloy::sol_types::SolType>::RustType,
            minimumStake: alloy::sol_types::private::primitives::aliases::U96,
            strategyParams: alloy::sol_types::private::Vec<
                <IStakeRegistryTypes::StrategyParams as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, createTotalDelegatedStakeQuorumCall, N> {
            self.call_builder(&createTotalDelegatedStakeQuorumCall {
                operatorSetParams,
                minimumStake,
                strategyParams,
            })
        }
        ///Creates a new call builder for the [`deregisterOperator`] function.
        pub fn deregisterOperator(
            &self,
            operator: alloy::sol_types::private::Address,
            avs: alloy::sol_types::private::Address,
            operatorSetIds: alloy::sol_types::private::Vec<u32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, deregisterOperatorCall, N> {
            self.call_builder(&deregisterOperatorCall {
                operator,
                avs,
                operatorSetIds,
            })
        }
        ///Creates a new call builder for the [`domainSeparator`] function.
        pub fn domainSeparator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, domainSeparatorCall, N> {
            self.call_builder(&domainSeparatorCall {})
        }
        ///Creates a new call builder for the [`eip712Domain`] function.
        pub fn eip712Domain(&self) -> alloy_contract::SolCallBuilder<T, &P, eip712DomainCall, N> {
            self.call_builder(&eip712DomainCall {})
        }
        ///Creates a new call builder for the [`ejectOperator`] function.
        pub fn ejectOperator(
            &self,
            operator: alloy::sol_types::private::Address,
            quorumNumbers: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, ejectOperatorCall, N> {
            self.call_builder(&ejectOperatorCall {
                operator,
                quorumNumbers,
            })
        }
        ///Creates a new call builder for the [`ejectionCooldown`] function.
        pub fn ejectionCooldown(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, ejectionCooldownCall, N> {
            self.call_builder(&ejectionCooldownCall {})
        }
        ///Creates a new call builder for the [`ejector`] function.
        pub fn ejector(&self) -> alloy_contract::SolCallBuilder<T, &P, ejectorCall, N> {
            self.call_builder(&ejectorCall {})
        }
        ///Creates a new call builder for the [`getCurrentQuorumBitmap`] function.
        pub fn getCurrentQuorumBitmap(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getCurrentQuorumBitmapCall, N> {
            self.call_builder(&getCurrentQuorumBitmapCall { operatorId })
        }
        ///Creates a new call builder for the [`getOperator`] function.
        pub fn getOperator(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorCall, N> {
            self.call_builder(&getOperatorCall { operator })
        }
        ///Creates a new call builder for the [`getOperatorFromId`] function.
        pub fn getOperatorFromId(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorFromIdCall, N> {
            self.call_builder(&getOperatorFromIdCall { operatorId })
        }
        ///Creates a new call builder for the [`getOperatorId`] function.
        pub fn getOperatorId(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorIdCall, N> {
            self.call_builder(&getOperatorIdCall { operator })
        }
        ///Creates a new call builder for the [`getOperatorSetParams`] function.
        pub fn getOperatorSetParams(
            &self,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorSetParamsCall, N> {
            self.call_builder(&getOperatorSetParamsCall { quorumNumber })
        }
        ///Creates a new call builder for the [`getOperatorStatus`] function.
        pub fn getOperatorStatus(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorStatusCall, N> {
            self.call_builder(&getOperatorStatusCall { operator })
        }
        ///Creates a new call builder for the [`getQuorumBitmapAtBlockNumberByIndex`] function.
        pub fn getQuorumBitmapAtBlockNumberByIndex(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            blockNumber: u32,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getQuorumBitmapAtBlockNumberByIndexCall, N>
        {
            self.call_builder(&getQuorumBitmapAtBlockNumberByIndexCall {
                operatorId,
                blockNumber,
                index,
            })
        }
        ///Creates a new call builder for the [`getQuorumBitmapHistoryLength`] function.
        pub fn getQuorumBitmapHistoryLength(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getQuorumBitmapHistoryLengthCall, N> {
            self.call_builder(&getQuorumBitmapHistoryLengthCall { operatorId })
        }
        ///Creates a new call builder for the [`getQuorumBitmapIndicesAtBlockNumber`] function.
        pub fn getQuorumBitmapIndicesAtBlockNumber(
            &self,
            blockNumber: u32,
            operatorIds: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getQuorumBitmapIndicesAtBlockNumberCall, N>
        {
            self.call_builder(&getQuorumBitmapIndicesAtBlockNumberCall {
                blockNumber,
                operatorIds,
            })
        }
        ///Creates a new call builder for the [`getQuorumBitmapUpdateByIndex`] function.
        pub fn getQuorumBitmapUpdateByIndex(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getQuorumBitmapUpdateByIndexCall, N> {
            self.call_builder(&getQuorumBitmapUpdateByIndexCall { operatorId, index })
        }
        ///Creates a new call builder for the [`indexRegistry`] function.
        pub fn indexRegistry(&self) -> alloy_contract::SolCallBuilder<T, &P, indexRegistryCall, N> {
            self.call_builder(&indexRegistryCall {})
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            initialOwner: alloy::sol_types::private::Address,
            churnApprover: alloy::sol_types::private::Address,
            ejector: alloy::sol_types::private::Address,
            initialPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
            avs: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(&initializeCall {
                initialOwner,
                churnApprover,
                ejector,
                initialPausedStatus,
                avs,
            })
        }
        ///Creates a new call builder for the [`isChurnApproverSaltUsed`] function.
        pub fn isChurnApproverSaltUsed(
            &self,
            _0: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, isChurnApproverSaltUsedCall, N> {
            self.call_builder(&isChurnApproverSaltUsedCall { _0 })
        }
        ///Creates a new call builder for the [`lastEjectionTimestamp`] function.
        pub fn lastEjectionTimestamp(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, lastEjectionTimestampCall, N> {
            self.call_builder(&lastEjectionTimestampCall { _0 })
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<T, &P, ownerCall, N> {
            self.call_builder(&ownerCall {})
        }
        ///Creates a new call builder for the [`pause`] function.
        pub fn pause(
            &self,
            newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, pauseCall, N> {
            self.call_builder(&pauseCall { newPausedStatus })
        }
        ///Creates a new call builder for the [`pauseAll`] function.
        pub fn pauseAll(&self) -> alloy_contract::SolCallBuilder<T, &P, pauseAllCall, N> {
            self.call_builder(&pauseAllCall {})
        }
        ///Creates a new call builder for the [`paused_0`] function.
        pub fn paused_0(
            &self,
            index: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, paused_0Call, N> {
            self.call_builder(&paused_0Call { index })
        }
        ///Creates a new call builder for the [`paused_1`] function.
        pub fn paused_1(&self) -> alloy_contract::SolCallBuilder<T, &P, paused_1Call, N> {
            self.call_builder(&paused_1Call {})
        }
        ///Creates a new call builder for the [`pauserRegistry`] function.
        pub fn pauserRegistry(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, pauserRegistryCall, N> {
            self.call_builder(&pauserRegistryCall {})
        }
        ///Creates a new call builder for the [`pubkeyRegistrationMessageHash`] function.
        pub fn pubkeyRegistrationMessageHash(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, pubkeyRegistrationMessageHashCall, N> {
            self.call_builder(&pubkeyRegistrationMessageHashCall { operator })
        }
        ///Creates a new call builder for the [`quorumCount`] function.
        pub fn quorumCount(&self) -> alloy_contract::SolCallBuilder<T, &P, quorumCountCall, N> {
            self.call_builder(&quorumCountCall {})
        }
        ///Creates a new call builder for the [`quorumUpdateBlockNumber`] function.
        pub fn quorumUpdateBlockNumber(
            &self,
            _0: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, quorumUpdateBlockNumberCall, N> {
            self.call_builder(&quorumUpdateBlockNumberCall { _0 })
        }
        ///Creates a new call builder for the [`registerOperator`] function.
        pub fn registerOperator(
            &self,
            operator: alloy::sol_types::private::Address,
            avs: alloy::sol_types::private::Address,
            operatorSetIds: alloy::sol_types::private::Vec<u32>,
            data: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerOperatorCall, N> {
            self.call_builder(&registerOperatorCall {
                operator,
                avs,
                operatorSetIds,
                data,
            })
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall {})
        }
        ///Creates a new call builder for the [`setAVS`] function.
        pub fn setAVS(
            &self,
            _avs: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setAVSCall, N> {
            self.call_builder(&setAVSCall { _avs })
        }
        ///Creates a new call builder for the [`setChurnApprover`] function.
        pub fn setChurnApprover(
            &self,
            _churnApprover: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setChurnApproverCall, N> {
            self.call_builder(&setChurnApproverCall { _churnApprover })
        }
        ///Creates a new call builder for the [`setEjectionCooldown`] function.
        pub fn setEjectionCooldown(
            &self,
            _ejectionCooldown: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, setEjectionCooldownCall, N> {
            self.call_builder(&setEjectionCooldownCall { _ejectionCooldown })
        }
        ///Creates a new call builder for the [`setEjector`] function.
        pub fn setEjector(
            &self,
            _ejector: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setEjectorCall, N> {
            self.call_builder(&setEjectorCall { _ejector })
        }
        ///Creates a new call builder for the [`setOperatorSetParams`] function.
        pub fn setOperatorSetParams(
            &self,
            quorumNumber: u8,
            operatorSetParams: <ISlashingRegistryCoordinatorTypes::OperatorSetParam as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, setOperatorSetParamsCall, N> {
            self.call_builder(&setOperatorSetParamsCall {
                quorumNumber,
                operatorSetParams,
            })
        }
        ///Creates a new call builder for the [`socketRegistry`] function.
        pub fn socketRegistry(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, socketRegistryCall, N> {
            self.call_builder(&socketRegistryCall {})
        }
        ///Creates a new call builder for the [`stakeRegistry`] function.
        pub fn stakeRegistry(&self) -> alloy_contract::SolCallBuilder<T, &P, stakeRegistryCall, N> {
            self.call_builder(&stakeRegistryCall {})
        }
        ///Creates a new call builder for the [`supportsAVS`] function.
        pub fn supportsAVS(
            &self,
            _avs: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, supportsAVSCall, N> {
            self.call_builder(&supportsAVSCall { _avs })
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
        ///Creates a new call builder for the [`unpause`] function.
        pub fn unpause(
            &self,
            newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, unpauseCall, N> {
            self.call_builder(&unpauseCall { newPausedStatus })
        }
        ///Creates a new call builder for the [`updateOperators`] function.
        pub fn updateOperators(
            &self,
            operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateOperatorsCall, N> {
            self.call_builder(&updateOperatorsCall { operators })
        }
        ///Creates a new call builder for the [`updateOperatorsForQuorum`] function.
        pub fn updateOperatorsForQuorum(
            &self,
            operatorsPerQuorum: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            >,
            quorumNumbers: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateOperatorsForQuorumCall, N> {
            self.call_builder(&updateOperatorsForQuorumCall {
                operatorsPerQuorum,
                quorumNumbers,
            })
        }
        ///Creates a new call builder for the [`updateSocket`] function.
        pub fn updateSocket(
            &self,
            socket: alloy::sol_types::private::String,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateSocketCall, N> {
            self.call_builder(&updateSocketCall { socket })
        }
        ///Creates a new call builder for the [`version`] function.
        pub fn version(&self) -> alloy_contract::SolCallBuilder<T, &P, versionCall, N> {
            self.call_builder(&versionCall {})
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > SlashingRegistryCoordinatorInstance<T, P, N>
    {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`AVSUpdated`] event.
        pub fn AVSUpdated_filter(&self) -> alloy_contract::Event<T, &P, AVSUpdated, N> {
            self.event_filter::<AVSUpdated>()
        }
        ///Creates a new event filter for the [`ChurnApproverUpdated`] event.
        pub fn ChurnApproverUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ChurnApproverUpdated, N> {
            self.event_filter::<ChurnApproverUpdated>()
        }
        ///Creates a new event filter for the [`EIP712DomainChanged`] event.
        pub fn EIP712DomainChanged_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, EIP712DomainChanged, N> {
            self.event_filter::<EIP712DomainChanged>()
        }
        ///Creates a new event filter for the [`EjectionCooldownUpdated`] event.
        pub fn EjectionCooldownUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, EjectionCooldownUpdated, N> {
            self.event_filter::<EjectionCooldownUpdated>()
        }
        ///Creates a new event filter for the [`EjectorUpdated`] event.
        pub fn EjectorUpdated_filter(&self) -> alloy_contract::Event<T, &P, EjectorUpdated, N> {
            self.event_filter::<EjectorUpdated>()
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(&self) -> alloy_contract::Event<T, &P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`OperatorDeregistered`] event.
        pub fn OperatorDeregistered_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorDeregistered, N> {
            self.event_filter::<OperatorDeregistered>()
        }
        ///Creates a new event filter for the [`OperatorRegistered`] event.
        pub fn OperatorRegistered_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorRegistered, N> {
            self.event_filter::<OperatorRegistered>()
        }
        ///Creates a new event filter for the [`OperatorSetParamsUpdated`] event.
        pub fn OperatorSetParamsUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorSetParamsUpdated, N> {
            self.event_filter::<OperatorSetParamsUpdated>()
        }
        ///Creates a new event filter for the [`OperatorSocketUpdate`] event.
        pub fn OperatorSocketUpdate_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorSocketUpdate, N> {
            self.event_filter::<OperatorSocketUpdate>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`Paused`] event.
        pub fn Paused_filter(&self) -> alloy_contract::Event<T, &P, Paused, N> {
            self.event_filter::<Paused>()
        }
        ///Creates a new event filter for the [`QuorumBlockNumberUpdated`] event.
        pub fn QuorumBlockNumberUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, QuorumBlockNumberUpdated, N> {
            self.event_filter::<QuorumBlockNumberUpdated>()
        }
        ///Creates a new event filter for the [`QuorumCreated`] event.
        pub fn QuorumCreated_filter(&self) -> alloy_contract::Event<T, &P, QuorumCreated, N> {
            self.event_filter::<QuorumCreated>()
        }
        ///Creates a new event filter for the [`Unpaused`] event.
        pub fn Unpaused_filter(&self) -> alloy_contract::Event<T, &P, Unpaused, N> {
            self.event_filter::<Unpaused>()
        }
    }
}
