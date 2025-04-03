#![allow(clippy::all, clippy::pedantic, clippy::nursery, warnings, unknown_lints, rustdoc::all, elided_lifetimes_in_paths)]
use DelegationManager::*;

///Module containing a contract's types and functions.
/**

```solidity
library IDelegationManagerTypes {
    struct QueuedWithdrawalParams { address[] strategies; uint256[] depositShares; address __deprecated_withdrawer; }
    struct Withdrawal { address staker; address delegatedTo; address withdrawer; uint256 nonce; uint32 startBlock; address[] strategies; uint256[] scaledShares; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IDelegationManagerTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct QueuedWithdrawalParams { address[] strategies; uint256[] depositShares; address __deprecated_withdrawer; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct QueuedWithdrawalParams {
        #[allow(missing_docs)]
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub depositShares: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        #[allow(missing_docs)]
        pub __deprecated_withdrawer: alloy::sol_types::private::Address,
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
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
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
        impl ::core::convert::From<QueuedWithdrawalParams> for UnderlyingRustTuple<'_> {
            fn from(value: QueuedWithdrawalParams) -> Self {
                (value.strategies, value.depositShares, value.__deprecated_withdrawer)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for QueuedWithdrawalParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    strategies: tuple.0,
                    depositShares: tuple.1,
                    __deprecated_withdrawer: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for QueuedWithdrawalParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for QueuedWithdrawalParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.depositShares),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.__deprecated_withdrawer,
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
        impl alloy_sol_types::SolType for QueuedWithdrawalParams {
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
        impl alloy_sol_types::SolStruct for QueuedWithdrawalParams {
            const NAME: &'static str = "QueuedWithdrawalParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "QueuedWithdrawalParams(address[] strategies,uint256[] depositShares,address __deprecated_withdrawer)",
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.strategies)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.depositShares)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.__deprecated_withdrawer,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for QueuedWithdrawalParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategies,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.depositShares,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.__deprecated_withdrawer,
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
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Address,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.strategies,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<256>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.depositShares,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.__deprecated_withdrawer,
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
struct Withdrawal { address staker; address delegatedTo; address withdrawer; uint256 nonce; uint32 startBlock; address[] strategies; uint256[] scaledShares; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Withdrawal {
        #[allow(missing_docs)]
        pub staker: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub delegatedTo: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub withdrawer: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub nonce: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub startBlock: u32,
        #[allow(missing_docs)]
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub scaledShares: alloy::sol_types::private::Vec<
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
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            u32,
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
        impl ::core::convert::From<Withdrawal> for UnderlyingRustTuple<'_> {
            fn from(value: Withdrawal) -> Self {
                (
                    value.staker,
                    value.delegatedTo,
                    value.withdrawer,
                    value.nonce,
                    value.startBlock,
                    value.strategies,
                    value.scaledShares,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Withdrawal {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    staker: tuple.0,
                    delegatedTo: tuple.1,
                    withdrawer: tuple.2,
                    nonce: tuple.3,
                    startBlock: tuple.4,
                    strategies: tuple.5,
                    scaledShares: tuple.6,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Withdrawal {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Withdrawal {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.staker,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.delegatedTo,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.withdrawer,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.startBlock),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.scaledShares),
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
        impl alloy_sol_types::SolType for Withdrawal {
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
        impl alloy_sol_types::SolStruct for Withdrawal {
            const NAME: &'static str = "Withdrawal";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Withdrawal(address staker,address delegatedTo,address withdrawer,uint256 nonce,uint32 startBlock,address[] strategies,uint256[] scaledShares)",
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
                            &self.staker,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.delegatedTo,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.withdrawer,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.nonce)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.startBlock)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.strategies)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.scaledShares)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Withdrawal {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.staker,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.delegatedTo,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.withdrawer,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.nonce)
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.startBlock,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategies,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.scaledShares,
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
                    &rust.staker,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.delegatedTo,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.withdrawer,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonce,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.startBlock,
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
                    &rust.scaledShares,
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
    /**Creates a new wrapper around an on-chain [`IDelegationManagerTypes`](self) contract instance.

See the [wrapper's documentation](`IDelegationManagerTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IDelegationManagerTypesInstance<T, P, N> {
        IDelegationManagerTypesInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IDelegationManagerTypes`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IDelegationManagerTypes`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IDelegationManagerTypesInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IDelegationManagerTypesInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IDelegationManagerTypesInstance")
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
    > IDelegationManagerTypesInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IDelegationManagerTypes`](self) contract instance.

See the [wrapper's documentation](`IDelegationManagerTypesInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IDelegationManagerTypesInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IDelegationManagerTypesInstance<T, P, N> {
            IDelegationManagerTypesInstance {
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
    > IDelegationManagerTypesInstance<T, P, N> {
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
    > IDelegationManagerTypesInstance<T, P, N> {
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
library ISignatureUtilsMixinTypes {
    struct SignatureWithExpiry { bytes signature; uint256 expiry; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod ISignatureUtilsMixinTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct SignatureWithExpiry { bytes signature; uint256 expiry; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SignatureWithExpiry {
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub expiry: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Bytes,
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
        impl ::core::convert::From<SignatureWithExpiry> for UnderlyingRustTuple<'_> {
            fn from(value: SignatureWithExpiry) -> Self {
                (value.signature, value.expiry)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SignatureWithExpiry {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    signature: tuple.0,
                    expiry: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for SignatureWithExpiry {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for SignatureWithExpiry {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.expiry),
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
        impl alloy_sol_types::SolType for SignatureWithExpiry {
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
        impl alloy_sol_types::SolStruct for SignatureWithExpiry {
            const NAME: &'static str = "SignatureWithExpiry";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "SignatureWithExpiry(bytes signature,uint256 expiry)",
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
                            &self.signature,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.expiry)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for SignatureWithExpiry {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signature,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.expiry,
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
                    &rust.signature,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.expiry,
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
    /**Creates a new wrapper around an on-chain [`ISignatureUtilsMixinTypes`](self) contract instance.

See the [wrapper's documentation](`ISignatureUtilsMixinTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ISignatureUtilsMixinTypesInstance<T, P, N> {
        ISignatureUtilsMixinTypesInstance::<T, P, N>::new(address, provider)
    }
    /**A [`ISignatureUtilsMixinTypes`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ISignatureUtilsMixinTypes`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ISignatureUtilsMixinTypesInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ISignatureUtilsMixinTypesInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ISignatureUtilsMixinTypesInstance")
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
    > ISignatureUtilsMixinTypesInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`ISignatureUtilsMixinTypes`](self) contract instance.

See the [wrapper's documentation](`ISignatureUtilsMixinTypesInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> ISignatureUtilsMixinTypesInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ISignatureUtilsMixinTypesInstance<T, P, N> {
            ISignatureUtilsMixinTypesInstance {
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
    > ISignatureUtilsMixinTypesInstance<T, P, N> {
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
    > ISignatureUtilsMixinTypesInstance<T, P, N> {
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
library IDelegationManagerTypes {
    struct QueuedWithdrawalParams {
        address[] strategies;
        uint256[] depositShares;
        address __deprecated_withdrawer;
    }
    struct Withdrawal {
        address staker;
        address delegatedTo;
        address withdrawer;
        uint256 nonce;
        uint32 startBlock;
        address[] strategies;
        uint256[] scaledShares;
    }
}

library ISignatureUtilsMixinTypes {
    struct SignatureWithExpiry {
        bytes signature;
        uint256 expiry;
    }
}

interface DelegationManager {
    error ActivelyDelegated();
    error CallerCannotUndelegate();
    error CurrentlyPaused();
    error FullySlashed();
    error InputAddressZero();
    error InputArrayLengthMismatch();
    error InputArrayLengthZero();
    error InvalidNewPausedStatus();
    error InvalidPermissions();
    error InvalidShortString();
    error InvalidSignature();
    error InvalidSnapshotOrdering();
    error NotActivelyDelegated();
    error OnlyAllocationManager();
    error OnlyEigenPodManager();
    error OnlyPauser();
    error OnlyStrategyManagerOrEigenPodManager();
    error OnlyUnpauser();
    error OperatorNotRegistered();
    error OperatorsCannotUndelegate();
    error SaltSpent();
    error SignatureExpired();
    error StringTooLong(string str);
    error WithdrawalDelayNotElapsed();
    error WithdrawalNotQueued();
    error WithdrawerNotCaller();

    event DelegationApproverUpdated(address indexed operator, address newDelegationApprover);
    event DepositScalingFactorUpdated(address staker, address strategy, uint256 newDepositScalingFactor);
    event Initialized(uint8 version);
    event OperatorMetadataURIUpdated(address indexed operator, string metadataURI);
    event OperatorRegistered(address indexed operator, address delegationApprover);
    event OperatorSharesDecreased(address indexed operator, address staker, address strategy, uint256 shares);
    event OperatorSharesIncreased(address indexed operator, address staker, address strategy, uint256 shares);
    event OperatorSharesSlashed(address indexed operator, address strategy, uint256 totalSlashedShares);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event Paused(address indexed account, uint256 newPausedStatus);
    event SlashingWithdrawalCompleted(bytes32 withdrawalRoot);
    event SlashingWithdrawalQueued(bytes32 withdrawalRoot, IDelegationManagerTypes.Withdrawal withdrawal, uint256[] sharesToWithdraw);
    event StakerDelegated(address indexed staker, address indexed operator);
    event StakerForceUndelegated(address indexed staker, address indexed operator);
    event StakerUndelegated(address indexed staker, address indexed operator);
    event Unpaused(address indexed account, uint256 newPausedStatus);

    constructor(address _strategyManager, address _eigenPodManager, address _allocationManager, address _pauserRegistry, address _permissionController, uint32 _MIN_WITHDRAWAL_DELAY, string _version);

    function DELEGATION_APPROVAL_TYPEHASH() external view returns (bytes32);
    function allocationManager() external view returns (address);
    function beaconChainETHStrategy() external view returns (address);
    function calculateDelegationApprovalDigestHash(address staker, address operator, address approver, bytes32 approverSalt, uint256 expiry) external view returns (bytes32);
    function calculateWithdrawalRoot(IDelegationManagerTypes.Withdrawal memory withdrawal) external pure returns (bytes32);
    function completeQueuedWithdrawal(IDelegationManagerTypes.Withdrawal memory withdrawal, address[] memory tokens, bool receiveAsTokens) external;
    function completeQueuedWithdrawals(IDelegationManagerTypes.Withdrawal[] memory withdrawals, address[][] memory tokens, bool[] memory receiveAsTokens) external;
    function convertToDepositShares(address staker, address[] memory strategies, uint256[] memory withdrawableShares) external view returns (uint256[] memory);
    function cumulativeWithdrawalsQueued(address staker) external view returns (uint256 totalQueued);
    function decreaseDelegatedShares(address staker, uint256 curDepositShares, uint64 beaconChainSlashingFactorDecrease) external;
    function delegateTo(address operator, ISignatureUtilsMixinTypes.SignatureWithExpiry memory approverSignatureAndExpiry, bytes32 approverSalt) external;
    function delegatedTo(address staker) external view returns (address operator);
    function delegationApprover(address operator) external view returns (address);
    function delegationApproverSaltIsSpent(address delegationApprover, bytes32 salt) external view returns (bool spent);
    function depositScalingFactor(address staker, address strategy) external view returns (uint256);
    function domainSeparator() external view returns (bytes32);
    function eigenPodManager() external view returns (address);
    function getDepositedShares(address staker) external view returns (address[] memory, uint256[] memory);
    function getOperatorShares(address operator, address[] memory strategies) external view returns (uint256[] memory);
    function getOperatorsShares(address[] memory operators, address[] memory strategies) external view returns (uint256[][] memory);
    function getQueuedWithdrawal(bytes32 withdrawalRoot) external view returns (IDelegationManagerTypes.Withdrawal memory withdrawal, uint256[] memory shares);
    function getQueuedWithdrawalRoots(address staker) external view returns (bytes32[] memory);
    function getQueuedWithdrawals(address staker) external view returns (IDelegationManagerTypes.Withdrawal[] memory withdrawals, uint256[][] memory shares);
    function getSlashableSharesInQueue(address operator, address strategy) external view returns (uint256);
    function getWithdrawableShares(address staker, address[] memory strategies) external view returns (uint256[] memory withdrawableShares, uint256[] memory depositShares);
    function increaseDelegatedShares(address staker, address strategy, uint256 prevDepositShares, uint256 addedShares) external;
    function initialize(address initialOwner, uint256 initialPausedStatus) external;
    function isDelegated(address staker) external view returns (bool);
    function isOperator(address operator) external view returns (bool);
    function minWithdrawalDelayBlocks() external view returns (uint32);
    function modifyOperatorDetails(address operator, address newDelegationApprover) external;
    function operatorShares(address operator, address strategy) external view returns (uint256 shares);
    function owner() external view returns (address);
    function pause(uint256 newPausedStatus) external;
    function pauseAll() external;
    function paused(uint8 index) external view returns (bool);
    function paused() external view returns (uint256);
    function pauserRegistry() external view returns (address);
    function pendingWithdrawals(bytes32 withdrawalRoot) external view returns (bool pending);
    function permissionController() external view returns (address);
    function queueWithdrawals(IDelegationManagerTypes.QueuedWithdrawalParams[] memory params) external returns (bytes32[] memory);
    function queuedWithdrawals(bytes32 withdrawalRoot) external view returns (IDelegationManagerTypes.Withdrawal memory withdrawal);
    function redelegate(address newOperator, ISignatureUtilsMixinTypes.SignatureWithExpiry memory newOperatorApproverSig, bytes32 approverSalt) external returns (bytes32[] memory withdrawalRoots);
    function registerAsOperator(address initDelegationApprover, uint32 allocationDelay, string memory metadataURI) external;
    function renounceOwnership() external;
    function slashOperatorShares(address operator, address strategy, uint64 prevMaxMagnitude, uint64 newMaxMagnitude) external;
    function strategyManager() external view returns (address);
    function transferOwnership(address newOwner) external;
    function undelegate(address staker) external returns (bytes32[] memory withdrawalRoots);
    function unpause(uint256 newPausedStatus) external;
    function updateOperatorMetadataURI(address operator, string memory metadataURI) external;
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
        "name": "_strategyManager",
        "type": "address",
        "internalType": "contract IStrategyManager"
      },
      {
        "name": "_eigenPodManager",
        "type": "address",
        "internalType": "contract IEigenPodManager"
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
        "name": "_permissionController",
        "type": "address",
        "internalType": "contract IPermissionController"
      },
      {
        "name": "_MIN_WITHDRAWAL_DELAY",
        "type": "uint32",
        "internalType": "uint32"
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
    "name": "DELEGATION_APPROVAL_TYPEHASH",
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
    "name": "beaconChainETHStrategy",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IStrategy"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "calculateDelegationApprovalDigestHash",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "approver",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "approverSalt",
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
    "name": "calculateWithdrawalRoot",
    "inputs": [
      {
        "name": "withdrawal",
        "type": "tuple",
        "internalType": "struct IDelegationManagerTypes.Withdrawal",
        "components": [
          {
            "name": "staker",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delegatedTo",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "withdrawer",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "startBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          },
          {
            "name": "scaledShares",
            "type": "uint256[]",
            "internalType": "uint256[]"
          }
        ]
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
    "name": "completeQueuedWithdrawal",
    "inputs": [
      {
        "name": "withdrawal",
        "type": "tuple",
        "internalType": "struct IDelegationManagerTypes.Withdrawal",
        "components": [
          {
            "name": "staker",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delegatedTo",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "withdrawer",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "startBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          },
          {
            "name": "scaledShares",
            "type": "uint256[]",
            "internalType": "uint256[]"
          }
        ]
      },
      {
        "name": "tokens",
        "type": "address[]",
        "internalType": "contract IERC20[]"
      },
      {
        "name": "receiveAsTokens",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "completeQueuedWithdrawals",
    "inputs": [
      {
        "name": "withdrawals",
        "type": "tuple[]",
        "internalType": "struct IDelegationManagerTypes.Withdrawal[]",
        "components": [
          {
            "name": "staker",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delegatedTo",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "withdrawer",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "startBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          },
          {
            "name": "scaledShares",
            "type": "uint256[]",
            "internalType": "uint256[]"
          }
        ]
      },
      {
        "name": "tokens",
        "type": "address[][]",
        "internalType": "contract IERC20[][]"
      },
      {
        "name": "receiveAsTokens",
        "type": "bool[]",
        "internalType": "bool[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "convertToDepositShares",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "strategies",
        "type": "address[]",
        "internalType": "contract IStrategy[]"
      },
      {
        "name": "withdrawableShares",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "cumulativeWithdrawalsQueued",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "totalQueued",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "decreaseDelegatedShares",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "curDepositShares",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "beaconChainSlashingFactorDecrease",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "delegateTo",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "approverSignatureAndExpiry",
        "type": "tuple",
        "internalType": "struct ISignatureUtilsMixinTypes.SignatureWithExpiry",
        "components": [
          {
            "name": "signature",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "expiry",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "approverSalt",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "delegatedTo",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "delegationApprover",
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
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "delegationApproverSaltIsSpent",
    "inputs": [
      {
        "name": "delegationApprover",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "salt",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "spent",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "depositScalingFactor",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "strategy",
        "type": "address",
        "internalType": "contract IStrategy"
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
    "name": "eigenPodManager",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IEigenPodManager"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getDepositedShares",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address[]",
        "internalType": "contract IStrategy[]"
      },
      {
        "name": "",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorShares",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "strategies",
        "type": "address[]",
        "internalType": "contract IStrategy[]"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorsShares",
    "inputs": [
      {
        "name": "operators",
        "type": "address[]",
        "internalType": "address[]"
      },
      {
        "name": "strategies",
        "type": "address[]",
        "internalType": "contract IStrategy[]"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256[][]",
        "internalType": "uint256[][]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getQueuedWithdrawal",
    "inputs": [
      {
        "name": "withdrawalRoot",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "withdrawal",
        "type": "tuple",
        "internalType": "struct IDelegationManagerTypes.Withdrawal",
        "components": [
          {
            "name": "staker",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delegatedTo",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "withdrawer",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "startBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          },
          {
            "name": "scaledShares",
            "type": "uint256[]",
            "internalType": "uint256[]"
          }
        ]
      },
      {
        "name": "shares",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getQueuedWithdrawalRoots",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getQueuedWithdrawals",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "withdrawals",
        "type": "tuple[]",
        "internalType": "struct IDelegationManagerTypes.Withdrawal[]",
        "components": [
          {
            "name": "staker",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delegatedTo",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "withdrawer",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "startBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          },
          {
            "name": "scaledShares",
            "type": "uint256[]",
            "internalType": "uint256[]"
          }
        ]
      },
      {
        "name": "shares",
        "type": "uint256[][]",
        "internalType": "uint256[][]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getSlashableSharesInQueue",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "strategy",
        "type": "address",
        "internalType": "contract IStrategy"
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
    "name": "getWithdrawableShares",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "strategies",
        "type": "address[]",
        "internalType": "contract IStrategy[]"
      }
    ],
    "outputs": [
      {
        "name": "withdrawableShares",
        "type": "uint256[]",
        "internalType": "uint256[]"
      },
      {
        "name": "depositShares",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "increaseDelegatedShares",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "strategy",
        "type": "address",
        "internalType": "contract IStrategy"
      },
      {
        "name": "prevDepositShares",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "addedShares",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
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
        "name": "initialPausedStatus",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "isDelegated",
    "inputs": [
      {
        "name": "staker",
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
    "name": "isOperator",
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
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "minWithdrawalDelayBlocks",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "modifyOperatorDetails",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "newDelegationApprover",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "operatorShares",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "strategy",
        "type": "address",
        "internalType": "contract IStrategy"
      }
    ],
    "outputs": [
      {
        "name": "shares",
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
    "name": "pendingWithdrawals",
    "inputs": [
      {
        "name": "withdrawalRoot",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "pending",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "permissionController",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IPermissionController"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "queueWithdrawals",
    "inputs": [
      {
        "name": "params",
        "type": "tuple[]",
        "internalType": "struct IDelegationManagerTypes.QueuedWithdrawalParams[]",
        "components": [
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          },
          {
            "name": "depositShares",
            "type": "uint256[]",
            "internalType": "uint256[]"
          },
          {
            "name": "__deprecated_withdrawer",
            "type": "address",
            "internalType": "address"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "queuedWithdrawals",
    "inputs": [
      {
        "name": "withdrawalRoot",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "withdrawal",
        "type": "tuple",
        "internalType": "struct IDelegationManagerTypes.Withdrawal",
        "components": [
          {
            "name": "staker",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delegatedTo",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "withdrawer",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "startBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          },
          {
            "name": "scaledShares",
            "type": "uint256[]",
            "internalType": "uint256[]"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "redelegate",
    "inputs": [
      {
        "name": "newOperator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "newOperatorApproverSig",
        "type": "tuple",
        "internalType": "struct ISignatureUtilsMixinTypes.SignatureWithExpiry",
        "components": [
          {
            "name": "signature",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "expiry",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "approverSalt",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "withdrawalRoots",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "registerAsOperator",
    "inputs": [
      {
        "name": "initDelegationApprover",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "allocationDelay",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "metadataURI",
        "type": "string",
        "internalType": "string"
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
    "name": "slashOperatorShares",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "strategy",
        "type": "address",
        "internalType": "contract IStrategy"
      },
      {
        "name": "prevMaxMagnitude",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "newMaxMagnitude",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "strategyManager",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IStrategyManager"
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
    "name": "undelegate",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "withdrawalRoots",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
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
    "name": "updateOperatorMetadataURI",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "metadataURI",
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
    "name": "DelegationApproverUpdated",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newDelegationApprover",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "DepositScalingFactorUpdated",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "strategy",
        "type": "address",
        "indexed": false,
        "internalType": "contract IStrategy"
      },
      {
        "name": "newDepositScalingFactor",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
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
    "name": "OperatorMetadataURIUpdated",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "metadataURI",
        "type": "string",
        "indexed": false,
        "internalType": "string"
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
        "name": "delegationApprover",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorSharesDecreased",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "staker",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "strategy",
        "type": "address",
        "indexed": false,
        "internalType": "contract IStrategy"
      },
      {
        "name": "shares",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorSharesIncreased",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "staker",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "strategy",
        "type": "address",
        "indexed": false,
        "internalType": "contract IStrategy"
      },
      {
        "name": "shares",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorSharesSlashed",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "strategy",
        "type": "address",
        "indexed": false,
        "internalType": "contract IStrategy"
      },
      {
        "name": "totalSlashedShares",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
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
    "name": "SlashingWithdrawalCompleted",
    "inputs": [
      {
        "name": "withdrawalRoot",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SlashingWithdrawalQueued",
    "inputs": [
      {
        "name": "withdrawalRoot",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      },
      {
        "name": "withdrawal",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IDelegationManagerTypes.Withdrawal",
        "components": [
          {
            "name": "staker",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delegatedTo",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "withdrawer",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "startBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          },
          {
            "name": "scaledShares",
            "type": "uint256[]",
            "internalType": "uint256[]"
          }
        ]
      },
      {
        "name": "sharesToWithdraw",
        "type": "uint256[]",
        "indexed": false,
        "internalType": "uint256[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "StakerDelegated",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "indexed": true,
        "internalType": "address"
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
    "name": "StakerForceUndelegated",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "indexed": true,
        "internalType": "address"
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
    "name": "StakerUndelegated",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "indexed": true,
        "internalType": "address"
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
    "name": "ActivelyDelegated",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CallerCannotUndelegate",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CurrentlyPaused",
    "inputs": []
  },
  {
    "type": "error",
    "name": "FullySlashed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InputAddressZero",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InputArrayLengthMismatch",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InputArrayLengthZero",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidNewPausedStatus",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidPermissions",
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
    "name": "InvalidSnapshotOrdering",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotActivelyDelegated",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyAllocationManager",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyEigenPodManager",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyPauser",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyStrategyManagerOrEigenPodManager",
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
    "name": "OperatorsCannotUndelegate",
    "inputs": []
  },
  {
    "type": "error",
    "name": "SaltSpent",
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
  },
  {
    "type": "error",
    "name": "WithdrawalDelayNotElapsed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "WithdrawalNotQueued",
    "inputs": []
  },
  {
    "type": "error",
    "name": "WithdrawerNotCaller",
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
pub mod DelegationManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x610160604052348015610010575f5ffd5b5060405161626a38038061626a83398101604081905261002f916101fb565b808084898989878a6001600160a01b03811661005e576040516339b190bb60e11b815260040160405180910390fd5b6001600160a01b0390811660805293841660a05291831660c052821660e05263ffffffff16610100521661012052610095816100b0565b61014052506100a490506100f6565b50505050505050610382565b5f5f829050601f815111156100e3578260405163305a27a960e01b81526004016100da919061032a565b60405180910390fd5b80516100ee8261035c565b179392505050565b5f54610100900460ff161561015d5760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b60648201526084016100da565b5f5460ff908116146101ac575f805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b6001600160a01b03811681146101c2575f5ffd5b50565b634e487b7160e01b5f52604160045260245ffd5b5f5b838110156101f35781810151838201526020016101db565b50505f910152565b5f5f5f5f5f5f5f60e0888a031215610211575f5ffd5b875161021c816101ae565b602089015190975061022d816101ae565b604089015190965061023e816101ae565b606089015190955061024f816101ae565b6080890151909450610260816101ae565b60a089015190935063ffffffff81168114610279575f5ffd5b60c08901519092506001600160401b03811115610294575f5ffd5b88015f601f82018b136102a5575f5ffd5b81516001600160401b038111156102be576102be6101c5565b604051601f8201601f19908116603f011681016001600160401b03811182821017156102ec576102ec6101c5565b6040528181528382016020018d1015610303575f5ffd5b6103148260208301602087016101d9565b8092508094505050505092959891949750929550565b602081525f82518060208401526103488160408501602087016101d9565b601f01601f19169190910160400192915050565b8051602080830151919081101561037c575f198160200360031b1b821691505b50919050565b60805160a05160c05160e051610100516101205161014051615def61047b5f395f81816110cf015261407201525f81816103fe015261335801525f818161070101528181613568015281816136aa015261398601525f818161075101528181610dc301528181610f7a0152818161132d01528181611525015281816119590152818161271e015261429c01525f818161042501528181610f07015281816114a5015281816116c901528181612f4f0152818161313101526137e901525f818161038101528181610ed50152818161163701526137c301525f81816105d601528181610b500152818161110f01526125140152615def5ff3fe608060405234801561000f575f5ffd5b50600436106102c3575f3560e01c8063715018a611610177578063bfae3fd2116100d5578063e4cc3f901161008f578063e4cc3f9014610799578063eea9064b146107ac578063f0e0e676146107bf578063f2fde38b146107df578063f698da25146107f2578063fabc1cbc146107fa578063fd8aa88d1461080d575f5ffd5b8063bfae3fd2146106e4578063c448feb8146106f7578063c978f7ac1461072b578063ca8aa7c71461074c578063cd6dc68714610773578063da8be86414610786575f5ffd5b80639104c319116101315780639104c3191461061c5780639435bb431461063057806399f5371b14610643578063a178848414610663578063a33a343314610682578063b7f06ebe14610695578063bb45fef2146106b7575f5ffd5b8063715018a61461058c578063778e55f31461059457806378296ec5146105be578063886f1195146105d15780638da5cb5b146105f85780639004134714610609575f5ffd5b806354fd4d50116102245780635dd68579116101de5780635dd68579146104d6578063601bb36f146104f757806360a0d1ce1461050a57806365da12641461051d57806366d5ba93146105455780636d70f7ae146105665780636e17444814610579575f5ffd5b806354fd4d501461045a578063595c6a671461046f578063597b36da146104775780635ac86ab71461048a5780635c975abb146104ad5780635d975e88146104b5575f5ffd5b806339b70e381161028057806339b70e381461037c5780633c651cf2146103b05780633cdeb5e0146103c35780633e28391d146103d65780634657e26a146103f95780634665bcda1461042057806354b7c96c14610447575f5ffd5b806304a4f979146102c75780630b9f487a146103015780630dd8dd0214610314578063136439dd1461033457806325df922e146103495780632aa6d88814610369575b5f5ffd5b6102ee7f14bde674c9f64b2ad00eaaee4a8bed1fabef35c7507e3c5b9cfc9436909a2dad81565b6040519081526020015b60405180910390f35b6102ee61030f366004614c76565b610820565b610327610322366004614d0d565b6108a8565b6040516102f89190614d4b565b610347610342366004614d82565b610b39565b005b61035c610357366004614f17565b610c13565b6040516102f89190614fc5565b610347610377366004615027565b610d73565b6103a37f000000000000000000000000000000000000000000000000000000000000000081565b6040516102f89190615085565b6103476103be366004615099565b610eca565b6103a36103d13660046150dc565b61101b565b6103e96103e43660046150dc565b61103b565b60405190151581526020016102f8565b6103a37f000000000000000000000000000000000000000000000000000000000000000081565b6103a37f000000000000000000000000000000000000000000000000000000000000000081565b6103476104553660046150f7565b61105a565b6104626110c8565b6040516102f8919061517b565b6103476110f8565b6102ee610485366004615249565b6111ac565b6103e961049836600461527a565b606654600160ff9092169190911b9081161490565b6066546102ee565b6104c86104c3366004614d82565b6111db565b6040516102f8929190615351565b6104e96104e43660046150dc565b6111f8565b6040516102f89291906153c3565b610347610505366004615444565b611322565b61034761051836600461549d565b61149a565b6103a361052b3660046150dc565b609a6020525f90815260409020546001600160a01b031681565b6105586105533660046150dc565b611630565b6040516102f89291906154dc565b6103e96105743660046150dc565b61191d565b6102ee6105873660046150f7565b611955565b6103476119fa565b6102ee6105a23660046150f7565b609860209081525f928352604080842090915290825290205481565b6103476105cc3660046154ee565b611a0b565b6103a37f000000000000000000000000000000000000000000000000000000000000000081565b6033546001600160a01b03166103a3565b61035c61061736600461553e565b611aa1565b6103a35f516020615d9a5f395f51905f5281565b61034761063e36600461558a565b611b77565b610656610651366004614d82565b611c50565b6040516102f89190615626565b6102ee6106713660046150dc565b609f6020525f908152604090205481565b610327610690366004615638565b611d6c565b6103e96106a3366004614d82565b609e6020525f908152604090205460ff1681565b6103e96106c536600461571f565b609c60209081525f928352604080842090915290825290205460ff1681565b6102ee6106f23660046150f7565b611d84565b60405163ffffffff7f00000000000000000000000000000000000000000000000000000000000000001681526020016102f8565b61073e61073936600461553e565b611dc0565b6040516102f8929190615749565b6103a37f000000000000000000000000000000000000000000000000000000000000000081565b61034761078136600461571f565b612038565b6103276107943660046150dc565b612153565b6103476107a7366004615768565b61227a565b6103476107ba366004615638565b6122d0565b6107d26107cd3660046157db565b61233b565b6040516102f89190615888565b6103476107ed3660046150dc565b6123e0565b6102ee612459565b610347610808366004614d82565b612512565b61032761081b3660046150dc565b612629565b604080517f14bde674c9f64b2ad00eaaee4a8bed1fabef35c7507e3c5b9cfc9436909a2dad60208201526001600160a01b03808616928201929092528187166060820152908516608082015260a0810183905260c081018290525f9061089e9060e0016040516020818303038152906040528051906020012061264c565b9695505050505050565b6066546060906001906002908116036108d45760405163840a48d560e01b815260040160405180910390fd5b6108dc61267a565b5f836001600160401b038111156108f5576108f5614d99565b60405190808252806020026020018201604052801561091e578160200160208202803683370190505b50335f908152609a60205260408120549192506001600160a01b03909116905b85811015610b2a578686828181106109585761095861589a565b905060200281019061096a91906158ae565b6109789060208101906158cc565b905087878381811061098c5761098c61589a565b905060200281019061099e91906158ae565b6109a890806158cc565b9050146109c8576040516343714afd60e01b815260040160405180910390fd5b5f610a3233848a8a868181106109e0576109e061589a565b90506020028101906109f291906158ae565b6109fc90806158cc565b808060200260200160405190810160405280939291908181526020018383602002808284375f920191909152506126d392505050565b9050610b0433848a8a86818110610a4b57610a4b61589a565b9050602002810190610a5d91906158ae565b610a6790806158cc565b808060200260200160405190810160405280939291908181526020018383602002808284375f920191909152508e92508d9150889050818110610aac57610aac61589a565b9050602002810190610abe91906158ae565b610acc9060208101906158cc565b808060200260200160405190810160405280939291908181526020018383602002808284375f92019190915250889250612825915050565b848381518110610b1657610b1661589a565b60209081029190910101525060010161093e565b5050600160c955949350505050565b60405163237dfb4760e11b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906346fbf68e90610b85903390600401615085565b602060405180830381865afa158015610ba0573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610bc49190615911565b610be157604051631d77d47760e21b815260040160405180910390fd5b6066548181168114610c065760405163c61dca5d60e01b815260040160405180910390fd5b610c0f82612d94565b5050565b6001600160a01b038084165f908152609a60205260408120546060921690610c3c8683876126d3565b90505f85516001600160401b03811115610c5857610c58614d99565b604051908082528060200260200182016040528015610c81578160200160208202803683370190505b5090505f5b8651811015610d66576001600160a01b0388165f90815260a260205260408120885182908a9085908110610cbc57610cbc61589a565b60200260200101516001600160a01b03166001600160a01b031681526020019081526020015f206040518060200160405290815f820154815250509050610d40878381518110610d0e57610d0e61589a565b6020026020010151858481518110610d2857610d2861589a565b602002602001015183612dd19092919063ffffffff16565b838381518110610d5257610d5261589a565b602090810291909101015250600101610c86565b50925050505b9392505050565b610d7b61267a565b610d843361103b565b15610da257604051633bf2b50360e11b815260040160405180910390fd5b604051632b6241f360e11b815233600482015263ffffffff841660248201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906356c483e6906044015f604051808303815f87803b158015610e0c575f5ffd5b505af1158015610e1e573d5f5f3e3d5ffd5b50505050610e2c3385612def565b610e363333612e51565b336001600160a01b03167fa453db612af59e5521d6ab9284dc3e2d06af286eb1b1b7b771fce4716c19f2c185604051610e6f9190615085565b60405180910390a2336001600160a01b03167f02a919ed0e2acad1dd90f17ef2fa4ae5462ee1339170034a8531cca4b67080908383604051610eb292919061592c565b60405180910390a2610ec4600160c955565b50505050565b336001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161480610f295750336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016145b610f465760405163045206a560e21b815260040160405180910390fd5b610f4e61267a565b6001600160a01b038085165f908152609a602052604080822054905163152667d960e31b8152908316927f0000000000000000000000000000000000000000000000000000000000000000169063a9333ec890610fb1908590899060040161595a565b602060405180830381865afa158015610fcc573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ff09190615974565b90505f610ffe8787846130f2565b905061100e8388888888866131d5565b505050610ec4600160c955565b6001600160a01b039081165f908152609960205260409020600101541690565b6001600160a01b039081165f908152609a602052604090205416151590565b816110648161331a565b6110815760405163932d94f760e01b815260040160405180910390fd5b61108961267a565b6110928361191d565b6110af576040516325ec6c1f60e01b815260040160405180910390fd5b6110b98383612def565b6110c3600160c955565b505050565b60606110f37f00000000000000000000000000000000000000000000000000000000000000006133c4565b905090565b60405163237dfb4760e11b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906346fbf68e90611144903390600401615085565b602060405180830381865afa15801561115f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906111839190615911565b6111a057604051631d77d47760e21b815260040160405180910390fd5b6111aa5f19612d94565b565b5f816040516020016111be9190615626565b604051602081830303815290604052805190602001209050919050565b6111e3614b37565b60606111ee83613401565b9094909350915050565b6060805f61120584612629565b8051909150806001600160401b0381111561122257611222614d99565b60405190808252806020026020018201604052801561125b57816020015b611248614b37565b8152602001906001900390816112405790505b509350806001600160401b0381111561127657611276614d99565b6040519080825280602002602001820160405280156112a957816020015b60608152602001906001900390816112945790505b5092505f5b8181101561131a576112d88382815181106112cb576112cb61589a565b6020026020010151613401565b8683815181106112ea576112ea61589a565b602002602001018684815181106113035761130361589a565b6020908102919091010191909152526001016112ae565b505050915091565b336001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161461136b576040516323d871a560e01b815260040160405180910390fd5b61137361267a565b6001600160a01b038085165f9081526098602090815260408083209387168352929052908120546113b1906001600160401b03808616908516613654565b90505f6113c08686868661366c565b90505f6113cd82846159a3565b90506113db875f8886613729565b604080516001600160a01b038881168252602082018490528916917fdd611f4ef63f4385f1756c86ce1f1f389a9013ba6fa07daba8528291bc2d3c30910160405180910390a25f61142b876137a3565b60405163debe1eab60e01b81526001600160a01b038981166004830152602482018590529192509082169063debe1eab906044015f604051808303815f87803b158015611476575f5ffd5b505af1158015611488573d5f5f3e3d5ffd5b5050505050505050610ec4600160c955565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146114e357604051633213a66160e21b815260040160405180910390fd5b6114eb61267a565b6114f48361103b565b156110b9576001600160a01b038084165f908152609a602052604080822054905163152667d960e31b8152908316927f0000000000000000000000000000000000000000000000000000000000000000169063a9333ec8906115699085905f516020615d9a5f395f51905f529060040161595a565b602060405180830381865afa158015611584573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906115a89190615974565b6001600160a01b0386165f90815260a2602090815260408083205f516020615d9a5f395f51905f52845282528083208151928301909152548152919250611607866115ff6001600160401b0380871690891661380e565b849190613822565b905061162284885f516020615d9a5f395f51905f5284613729565b505050506110c3600160c955565b6060805f5f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166394f649dd866040518263ffffffff1660e01b81526004016116819190615085565b5f60405180830381865afa15801561169b573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526116c29190810190615a11565b915091505f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663fe243a17875f516020615d9a5f395f51905f526040518363ffffffff1660e01b815260040161172292919061595a565b602060405180830381865afa15801561173d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906117619190615acc565b9050805f0361177557509094909350915050565b5f8351600161178491906159a3565b6001600160401b0381111561179b5761179b614d99565b6040519080825280602002602001820160405280156117c4578160200160208202803683370190505b5090505f845160016117d691906159a3565b6001600160401b038111156117ed576117ed614d99565b604051908082528060200260200182016040528015611816578160200160208202803683370190505b5090505f516020615d9a5f395f51905f528286518151811061183a5761183a61589a565b60200260200101906001600160a01b031690816001600160a01b031681525050828186518151811061186e5761186e61589a565b60209081029190910101525f5b855181101561190f578581815181106118965761189661589a565b60200260200101518382815181106118b0576118b061589a565b60200260200101906001600160a01b031690816001600160a01b0316815250508481815181106118e2576118e261589a565b60200260200101518282815181106118fc576118fc61589a565b602090810291909101015260010161187b565b509097909650945050505050565b5f6001600160a01b0382161580159061194f57506001600160a01b038083165f818152609a6020526040902054909116145b92915050565b5f5f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663a9333ec885856040518363ffffffff1660e01b81526004016119a592919061595a565b602060405180830381865afa1580156119c0573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119e49190615974565b90506119f28484835f61366c565b949350505050565b611a02613840565b6111aa5f61389a565b82611a158161331a565b611a325760405163932d94f760e01b815260040160405180910390fd5b611a3b8461191d565b611a58576040516325ec6c1f60e01b815260040160405180910390fd5b836001600160a01b03167f02a919ed0e2acad1dd90f17ef2fa4ae5462ee1339170034a8531cca4b67080908484604051611a9392919061592c565b60405180910390a250505050565b60605f82516001600160401b03811115611abd57611abd614d99565b604051908082528060200260200182016040528015611ae6578160200160208202803683370190505b5090505f5b8351811015611b6f576001600160a01b0385165f9081526098602052604081208551909190869084908110611b2257611b2261589a565b60200260200101516001600160a01b03166001600160a01b031681526020019081526020015f2054828281518110611b5c57611b5c61589a565b6020908102919091010152600101611aeb565b509392505050565b606654600290600490811603611ba05760405163840a48d560e01b815260040160405180910390fd5b611ba861267a565b855f5b81811015611c3b57611c33898983818110611bc857611bc861589a565b9050602002810190611bda9190615ae3565b611be390615af7565b888884818110611bf557611bf561589a565b9050602002810190611c0791906158cc565b888886818110611c1957611c1961589a565b9050602002016020810190611c2e9190615b02565b6138eb565b600101611bab565b5050611c47600160c955565b50505050505050565b611c58614b37565b5f82815260a46020908152604091829020825160e08101845281546001600160a01b03908116825260018301548116828501526002830154168185015260038201546060820152600482015463ffffffff1660808201526005820180548551818602810186019096528086529194929360a08601939290830182828015611d0657602002820191905f5260205f20905b81546001600160a01b03168152600190910190602001808311611ce8575b5050505050815260200160068201805480602002602001604051908101604052809291908181526020018280548015611d5c57602002820191905f5260205f20905b815481526020019060010190808311611d48575b5050505050815250509050919050565b6060611d7733612153565b9050610d6c8484846122d0565b6001600160a01b038083165f90815260a260209081526040808320938516835292815282822083519182019093529154825290610d6c90613d2d565b60608082516001600160401b03811115611ddc57611ddc614d99565b604051908082528060200260200182016040528015611e05578160200160208202803683370190505b50915082516001600160401b03811115611e2157611e21614d99565b604051908082528060200260200182016040528015611e4a578160200160208202803683370190505b506001600160a01b038086165f908152609a6020526040812054929350911690611e758683876126d3565b90505f5b855181101561202d575f611ea5878381518110611e9857611e9861589a565b60200260200101516137a3565b9050806001600160a01b031663fe243a1789898581518110611ec957611ec961589a565b60200260200101516040518363ffffffff1660e01b8152600401611eee92919061595a565b602060405180830381865afa158015611f09573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611f2d9190615acc565b858381518110611f3f57611f3f61589a565b6020026020010181815250505f60a25f8a6001600160a01b03166001600160a01b031681526020019081526020015f205f898581518110611f8257611f8261589a565b60200260200101516001600160a01b03166001600160a01b031681526020019081526020015f206040518060200160405290815f820154815250509050612006868481518110611fd457611fd461589a565b6020026020010151858581518110611fee57611fee61589a565b6020026020010151836138229092919063ffffffff16565b8784815181106120185761201861589a565b60209081029190910101525050600101611e79565b5050505b9250929050565b5f54610100900460ff161580801561205657505f54600160ff909116105b8061206f5750303b15801561206f57505f5460ff166001145b6120d75760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b5f805460ff1916600117905580156120f8575f805461ff0019166101001790555b61210182612d94565b61210a8361389a565b80156110c3575f805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a1505050565b606061215d61267a565b6121668261103b565b6121835760405163a5c7c44560e01b815260040160405180910390fd5b61218c8261191d565b156121aa576040516311ca333560e31b815260040160405180910390fd5b336001600160a01b03831614612260576001600160a01b038083165f908152609a6020526040902054166121dd8161331a565b8061220157506121ec8161101b565b6001600160a01b0316336001600160a01b0316145b61221e57604051631e499a2360e11b815260040160405180910390fd5b806001600160a01b0316836001600160a01b03167ff0eddf07e6ea14f388b47e1e94a0f464ecbd9eed4171130e0fc0e99fb4030a8a60405160405180910390a3505b61226982613d4c565b9050612275600160c955565b919050565b6066546002906004908116036122a35760405163840a48d560e01b815260040160405180910390fd5b6122ab61267a565b6122bf6122b786615af7565b8585856138eb565b6122c9600160c955565b5050505050565b6122d861267a565b6122e13361103b565b156122ff57604051633bf2b50360e11b815260040160405180910390fd5b6123088361191d565b612325576040516325ec6c1f60e01b815260040160405180910390fd5b61233133848484613fab565b6110b93384612e51565b60605f83516001600160401b0381111561235757612357614d99565b60405190808252806020026020018201604052801561238a57816020015b60608152602001906001900390816123755790505b5090505f5b8451811015611b6f576123bb8582815181106123ad576123ad61589a565b602002602001015185611aa1565b8282815181106123cd576123cd61589a565b602090810291909101015260010161238f565b6123e8613840565b6001600160a01b03811661244d5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b60648201526084016120ce565b6124568161389a565b50565b60408051808201909152600a81526922b4b3b2b72630bcb2b960b11b6020909101525f7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f7f71b625cfad44bac63b13dba07f2e1d6084ee04b6f8752101ece6126d584ee6ea6124c661406a565b805160209182012060408051928301949094529281019190915260608101919091524660808201523060a082015260c00160405160208183030381529060405280519060200120905090565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561256e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906125929190615b1d565b6001600160a01b0316336001600160a01b0316146125c35760405163794821ff60e01b815260040160405180910390fd5b606654801982198116146125ea5760405163c61dca5d60e01b815260040160405180910390fd5b606682905560405182815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c906020015b60405180910390a25050565b6001600160a01b0381165f90815260a36020526040902060609061194f90614106565b5f612655612459565b60405161190160f01b60208201526022810191909152604281018390526062016111be565b600260c954036126cc5760405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c0060448201526064016120ce565b600260c955565b60605f82516001600160401b038111156126ef576126ef614d99565b604051908082528060200260200182016040528015612718578160200160208202803683370190505b5090505f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663547afb8786866040518363ffffffff1660e01b815260040161276a929190615b38565b5f60405180830381865afa158015612784573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526127ab9190810190615b5b565b90505f5b845181101561281a576127f5878683815181106127ce576127ce61589a565b60200260200101518484815181106127e8576127e861589a565b60200260200101516130f2565b8382815181106128075761280761589a565b60209081029190910101526001016127af565b509095945050505050565b5f6001600160a01b03861661284d576040516339b190bb60e11b815260040160405180910390fd5b83515f0361286e5760405163796cc52560e01b815260040160405180910390fd5b5f84516001600160401b0381111561288857612888614d99565b6040519080825280602002602001820160405280156128b1578160200160208202803683370190505b5090505f85516001600160401b038111156128ce576128ce614d99565b6040519080825280602002602001820160405280156128f7578160200160208202803683370190505b5090505f5b8651811015612bc7575f61291b888381518110611e9857611e9861589a565b90505f60a25f8c6001600160a01b03166001600160a01b031681526020019081526020015f205f8a85815181106129545761295461589a565b60200260200101516001600160a01b03166001600160a01b031681526020019081526020015f206040518060200160405290815f8201548152505090506129c08884815181106129a6576129a661589a565b6020026020010151888581518110611fee57611fee61589a565b8484815181106129d2576129d261589a565b602002602001018181525050612a0a8884815181106129f3576129f361589a565b60200260200101518261411290919063ffffffff16565b858481518110612a1c57612a1c61589a565b60209081029190910101526001600160a01b038a1615612ab157612a738a8a8581518110612a4c57612a4c61589a565b6020026020010151878681518110612a6657612a6661589a565b6020026020010151614126565b612ab18a8c8b8681518110612a8a57612a8a61589a565b6020026020010151878781518110612aa457612aa461589a565b6020026020010151613729565b5f826001600160a01b031663724af4238d8c8781518110612ad457612ad461589a565b60200260200101518c8881518110612aee57612aee61589a565b60200260200101516040518463ffffffff1660e01b8152600401612b1493929190615bea565b6020604051808303815f875af1158015612b30573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612b549190615acc565b9050805f03612bb9576001600160a01b038c165f90815260a2602052604081208b51612bb992908d9088908110612b8d57612b8d61589a565b60200260200101516001600160a01b03166001600160a01b031681526020019081526020015f205f9055565b5050508060010190506128fc565b506001600160a01b0388165f908152609f60205260408120805491829190612bee83615c0e565b91905055505f6040518060e001604052808b6001600160a01b031681526020018a6001600160a01b031681526020018b6001600160a01b031681526020018381526020014363ffffffff1681526020018981526020018581525090505f612c54826111ac565b5f818152609e602090815260408083208054600160ff19909116811790915560a4835292819020865181546001600160a01b03199081166001600160a01b039283161783558885015195830180548216968316969096179095559187015160028201805490951692169190911790925560608501516003830155608085015160048301805463ffffffff191663ffffffff90921691909117905560a085015180519394508593612d0a9260058501920190614b90565b5060c08201518051612d26916006840191602090910190614bf3565b5050506001600160a01b038b165f90815260a360205260409020612d4a9082614190565b507f26b2aae26516e8719ef50ea2f6831a2efbd4e37dccdf0f6936b27bc08e793e30818386604051612d7e93929190615c26565b60405180910390a19a9950505050505050505050565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a250565b5f6119f282612de9612de287613d2d565b869061419b565b9061419b565b6001600160a01b038281165f818152609960205260409081902060010180546001600160a01b0319169385169390931790925590517f773b54c04d756fcc5e678111f7d730de3be98192000799eee3d63716055a87c69061261d908490615085565b6066545f90600190811603612e795760405163840a48d560e01b815260040160405180910390fd5b5f5f612e8485611630565b915091505f612e945f86856126d3565b6001600160a01b038781165f818152609a602052604080822080546001600160a01b031916948b16948517905551939450919290917fc3ee9f2e5fda98e8066a1f745b2df9285f416fe98cf2559cd21484b3d874330491a35f5b8351811015611c47575f516020615d9a5f395f51905f526001600160a01b0316848281518110612f2057612f2061589a565b60200260200101516001600160a01b0316036130915760405163a3d75e0960e01b81525f906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063a3d75e0990612f84908b90600401615085565b602060405180830381865afa158015612f9f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612fc39190615974565b90505f60a25f8a6001600160a01b03166001600160a01b031681526020019081526020015f205f878581518110612ffc57612ffc61589a565b60200260200101516001600160a01b03166001600160a01b031681526020019081526020015f206040518060200160405290815f82015481525050905061307085848151811061304e5761304e61589a565b6020026020010151836001600160401b0316836138229092919063ffffffff16565b8584815181106130825761308261589a565b60200260200101818152505050505b6130ea86888684815181106130a8576130a861589a565b60200260200101515f8786815181106130c3576130c361589a565b60200260200101518787815181106130dd576130dd61589a565b60200260200101516131d5565b600101612eee565b5f73beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeabf196001600160a01b038416016131c55760405163a3d75e0960e01b81525f906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063a3d75e0990613166908890600401615085565b602060405180830381865afa158015613181573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906131a59190615974565b90506131bd6001600160401b0384811690831661380e565b915050610d6c565b506001600160401b031692915050565b805f036131f557604051630a33bc6960e21b815260040160405180910390fd5b8115613312576001600160a01b038086165f90815260a26020908152604080832093881683529290522061322b818585856141af565b6040805160208101909152815481527f8be932bac54561f27260f95463d9b8ab37e06b2842e5ee2404157cc13df6eb8f908790879061326990613d2d565b60405161327893929190615bea565b60405180910390a16132898661103b565b15611c47576001600160a01b038088165f908152609860209081526040808320938916835292905290812080548592906132c49084906159a3565b92505081905550866001600160a01b03167f1ec042c965e2edd7107b51188ee0f383e22e76179041ab3a9d18ff151405166c87878660405161330893929190615bea565b60405180910390a2505b505050505050565b604051631beb2b9760e31b81526001600160a01b0382811660048301523360248301523060448301525f80356001600160e01b0319166064840152917f00000000000000000000000000000000000000000000000000000000000000009091169063df595cb8906084016020604051808303815f875af11580156133a0573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061194f9190615911565b60605f6133d08361422a565b6040805160208082528183019092529192505f91906020820181803683375050509182525060208101929092525090565b613409614b37565b5f82815260a46020908152604091829020825160e08101845281546001600160a01b0390811682526001830154811682850152600283015416818501526003820154606082810191909152600483015463ffffffff1660808301526005830180548651818702810187019097528087529195929460a086019392908301828280156134bb57602002820191905f5260205f20905b81546001600160a01b0316815260019091019060200180831161349d575b505050505081526020016006820180548060200260200160405190810160405280929190818152602001828054801561351157602002820191905f5260205f20905b8154815260200190600101908083116134fd575b50505050508152505091508160a00151516001600160401b0381111561353957613539614d99565b604051908082528060200260200182016040528015613562578160200160208202803683370190505b5090505f7f000000000000000000000000000000000000000000000000000000000000000083608001516135969190615c50565b90505f4363ffffffff168263ffffffff16106135c7576135c2845f015185602001518660a001516126d3565b6135de565b6135de845f015185602001518660a0015185614251565b90505f5b8460a001515181101561131a5761362f8560c0015182815181106136085761360861589a565b60200260200101518383815181106136225761362261589a565b602002602001015161437f565b8482815181106136415761364161589a565b60209081029190910101526001016135e2565b5f613662848385600161438a565b6119f29085615c6c565b6001600160a01b038085165f90815260a5602090815260408083209387168352929052908120819061369d906143e5565b90505f61370360016136cf7f000000000000000000000000000000000000000000000000000000000000000043615c7f565b6136d99190615c7f565b6001600160a01b03808a165f90815260a560209081526040808320938c16835292905220906143ff565b90505f6137108284615c6c565b905061371d81878761441b565b98975050505050505050565b6001600160a01b038085165f9081526098602090815260408083209386168352929052908120805483929061375f908490615c6c565b92505081905550836001600160a01b03167f6909600037b75d7b4733aedd815442b5ec018a827751c832aaff64eba5d6d2dd848484604051611a9393929190615bea565b5f6001600160a01b0382165f516020615d9a5f395f51905f52146137e7577f000000000000000000000000000000000000000000000000000000000000000061194f565b7f000000000000000000000000000000000000000000000000000000000000000092915050565b5f610d6c8383670de0b6b3a7640000614439565b5f6119f28261383a61383387613d2d565b869061380e565b9061380e565b6033546001600160a01b031633146111aa5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016120ce565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a35050565b60a0840151518214613910576040516343714afd60e01b815260040160405180910390fd5b83604001516001600160a01b0316336001600160a01b031614613946576040516316110d3560e21b815260040160405180910390fd5b5f613950856111ac565b5f818152609e602052604090205490915060ff16613981576040516387c9d21960e01b815260040160405180910390fd5b60605f7f000000000000000000000000000000000000000000000000000000000000000087608001516139b49190615c50565b90508063ffffffff164363ffffffff16116139e2576040516378f67ae160e11b815260040160405180910390fd5b6139f9875f015188602001518960a0015184614251565b87516001600160a01b03165f90815260a360205260409020909250613a1f91508361451e565b505f82815260a46020526040812080546001600160a01b031990811682556001820180548216905560028201805490911690556003810182905560048101805463ffffffff1916905590613a766005830182614c2c565b613a83600683015f614c2c565b50505f828152609e602052604090819020805460ff19169055517f1f40400889274ed07b24845e5054a87a0cab969eb1277aafe61ae352e7c32a0090613acc9084815260200190565b60405180910390a185516001600160a01b039081165f908152609a6020526040812054885160a08a01519190931692613b069184906126d3565b90505f5b8860a0015151811015613d22575f613b318a60a001518381518110611e9857611e9861589a565b90505f613b678b60c001518481518110613b4d57613b4d61589a565b60200260200101518785815181106136225761362261589a565b9050805f03613b77575050613d1a565b8715613c4557816001600160a01b0316632eae418c8c5f01518d60a001518681518110613ba657613ba661589a565b60200260200101518d8d88818110613bc057613bc061589a565b9050602002016020810190613bd591906150dc565b60405160e085901b6001600160e01b03191681526001600160a01b03938416600482015291831660248301529091166044820152606481018490526084015f604051808303815f87803b158015613c2a575f5ffd5b505af1158015613c3c573d5f5f3e3d5ffd5b50505050613d17565b5f5f836001600160a01b03166350ff72258e5f01518f60a001518881518110613c7057613c7061589a565b6020026020010151866040518463ffffffff1660e01b8152600401613c9793929190615bea565b60408051808303815f875af1158015613cb2573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613cd69190615c9b565b91509150613d14878e5f01518f60a001518881518110613cf857613cf861589a565b602002602001015185858b8b815181106130dd576130dd61589a565b50505b50505b600101613b0a565b505050505050505050565b80515f9015613d3d57815161194f565b670de0b6b3a764000092915050565b606654606090600190600290811603613d785760405163840a48d560e01b815260040160405180910390fd5b6001600160a01b038084165f818152609a602052604080822080546001600160a01b0319811690915590519316928392917ffee30966a256b71e14bc0ebfc94315e28ef4a97a7131a9e2b7a310a73af4467691a35f5f613dd786611630565b9150915081515f03613deb57505050613fa5565b81516001600160401b03811115613e0457613e04614d99565b604051908082528060200260200182016040528015613e2d578160200160208202803683370190505b5094505f613e3c8785856126d3565b90505f5b8351811015613f9f576040805160018082528183019092525f916020808301908036833750506040805160018082528183019092529293505f9291506020808301908036833750506040805160018082528183019092529293505f92915060208083019080368337019050509050868481518110613ec057613ec061589a565b6020026020010151835f81518110613eda57613eda61589a565b60200260200101906001600160a01b031690816001600160a01b031681525050858481518110613f0c57613f0c61589a565b6020026020010151825f81518110613f2657613f2661589a565b602002602001018181525050848481518110613f4457613f4461589a565b6020026020010151815f81518110613f5e57613f5e61589a565b602002602001018181525050613f778b89858585612825565b8a8581518110613f8957613f8961589a565b6020908102919091010152505050600101613e40565b50505050505b50919050565b6001600160a01b038084165f908152609960205260409020600101541680613fd35750610ec4565b6001600160a01b0381165f908152609c6020908152604080832085845290915290205460ff161561401757604051630d4c4c9160e21b815260040160405180910390fd5b6001600160a01b0381165f908152609c602090815260408083208584528252909120805460ff191660011790558301516122c990829061405e908890889084908890610820565b85516020870151614529565b60605f6140967f00000000000000000000000000000000000000000000000000000000000000006133c4565b9050805f815181106140aa576140aa61589a565b602001015160f81c60f81b816001815181106140c8576140c861589a565b016020908101516040516001600160f81b03199384169281019290925291909116602182015260220160405160208183030381529060405291505090565b60605f610d6c8361457b565b5f610d6c61411f84613d2d565b839061380e565b6001600160a01b038084165f90815260a5602090815260408083209386168352929052908120614155906143e5565b9050610ec44361416584846159a3565b6001600160a01b038088165f90815260a560209081526040808320938a1683529290522091906145d4565b5f610d6c83836145df565b5f610d6c83670de0b6b3a764000084614439565b825f036141db576040805160208101909152845481526141d4908290612de990613d2d565b8455610ec4565b6040805160208101909152845481525f906141f7908584613822565b90505f61420484836159a3565b90505f61421f84612de9614218888a6159a3565b859061419b565b875550505050505050565b5f60ff8216601f81111561194f57604051632cd44ac360e21b815260040160405180910390fd5b60605f83516001600160401b0381111561426d5761426d614d99565b604051908082528060200260200182016040528015614296578160200160208202803683370190505b5090505f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166394d7d00c8787876040518463ffffffff1660e01b81526004016142ea93929190615cbd565b5f60405180830381865afa158015614304573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261432b9190810190615b5b565b90505f5b85518110156143735761434e888783815181106127ce576127ce61589a565b8382815181106143605761436061589a565b602090810291909101015260010161432f565b50909695505050505050565b5f610d6c838361380e565b5f5f614397868686614439565b905060018360028111156143ad576143ad615cf6565b1480156143c957505f84806143c4576143c4615d0a565b868809115b156143dc576143d96001826159a3565b90505b95945050505050565b5f6143f0828261462b565b6001600160e01b031692915050565b5f61440b83838361466f565b6001600160e01b03169392505050565b5f6119f26144298385615d1e565b85906001600160401b031661380e565b5f80805f19858709858702925082811083820303915050805f036144705783828161446657614466615d0a565b0492505050610d6c565b8084116144b75760405162461bcd60e51b81526020600482015260156024820152744d6174683a206d756c446976206f766572666c6f7760581b60448201526064016120ce565b5f8486880960026001871981018816978890046003810283188082028403028082028403028082028403028082028403028082028403029081029092039091025f889003889004909101858311909403939093029303949094049190911702949350505050565b5f610d6c83836146b7565b4281101561454a57604051630819bdcd60e01b815260040160405180910390fd5b61455e6001600160a01b038516848461479a565b610ec457604051638baa579f60e01b815260040160405180910390fd5b6060815f018054806020026020016040519081016040528092919081815260200182805480156145c857602002820191905f5260205f20905b8154815260200190600101908083116145b4575b50505050509050919050565b6110c38383836147ee565b5f81815260018301602052604081205461462457508154600181810184555f84815260208082209093018490558454848252828601909352604090209190915561194f565b505f61194f565b81545f9080156146675761465184614644600184615c6c565b5f91825260209091200190565b54600160201b90046001600160e01b03166119f2565b509092915050565b82545f9081614680868683856148f1565b905080156146ad5761469786614644600184615c6c565b54600160201b90046001600160e01b031661089e565b5091949350505050565b5f8181526001830160205260408120548015614791575f6146d9600183615c6c565b85549091505f906146ec90600190615c6c565b905081811461474b575f865f01828154811061470a5761470a61589a565b905f5260205f200154905080875f01848154811061472a5761472a61589a565b5f918252602080832090910192909255918252600188019052604090208390555b855486908061475c5761475c615d3d565b600190038181905f5260205f20015f90559055856001015f8681526020019081526020015f205f90556001935050505061194f565b5f91505061194f565b5f5f5f6147a78585614944565b90925090505f8160048111156147bf576147bf615cf6565b1480156147dd5750856001600160a01b0316826001600160a01b0316145b8061089e575061089e868686614983565b825480156148a4575f61480685614644600185615c6c565b60408051808201909152905463ffffffff808216808452600160201b9092046001600160e01b0316602084015291925090851610156148585760405163151b8e3f60e11b815260040160405180910390fd5b805163ffffffff8086169116036148a2578261487986614644600186615c6c565b80546001600160e01b0392909216600160201b0263ffffffff9092169190911790555050505050565b505b506040805180820190915263ffffffff92831681526001600160e01b03918216602080830191825285546001810187555f968752952091519051909216600160201b029190921617910155565b5f5b81831015611b6f575f6149068484614a6a565b5f8781526020902090915063ffffffff86169082015463ffffffff1611156149305780925061493e565b61493b8160016159a3565b93505b506148f3565b5f5f8251604103614978576020830151604084015160608501515f1a61496c87828585614a84565b94509450505050612031565b505f90506002612031565b5f5f5f856001600160a01b0316631626ba7e60e01b86866040516024016149ab929190615d51565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b03199094169390931790925290516149e99190615d69565b5f60405180830381855afa9150503d805f8114614a21576040519150601f19603f3d011682016040523d82523d5f602084013e614a26565b606091505b5091509150818015614a3a57506020815110155b801561089e57508051630b135d3f60e11b90614a5f9083016020908101908401615acc565b149695505050505050565b5f614a786002848418615d7a565b610d6c908484166159a3565b5f806fa2a8918ca85bafe22016d0b997e4df60600160ff1b03831115614aaf57505f90506003614b2e565b604080515f8082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa158015614b00573d5f5f3e3d5ffd5b5050604051601f1901519150506001600160a01b038116614b28575f60019250925050614b2e565b91505f90505b94509492505050565b6040518060e001604052805f6001600160a01b031681526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f63ffffffff16815260200160608152602001606081525090565b828054828255905f5260205f20908101928215614be3579160200282015b82811115614be357825182546001600160a01b0319166001600160a01b03909116178255602090920191600190910190614bae565b50614bef929150614c43565b5090565b828054828255905f5260205f20908101928215614be3579160200282015b82811115614be3578251825591602001919060010190614c11565b5080545f8255905f5260205f209081019061245691905b5b80821115614bef575f8155600101614c44565b6001600160a01b0381168114612456575f5ffd5b803561227581614c57565b5f5f5f5f5f60a08688031215614c8a575f5ffd5b8535614c9581614c57565b94506020860135614ca581614c57565b93506040860135614cb581614c57565b94979396509394606081013594506080013592915050565b5f5f83601f840112614cdd575f5ffd5b5081356001600160401b03811115614cf3575f5ffd5b6020830191508360208260051b8501011115612031575f5ffd5b5f5f60208385031215614d1e575f5ffd5b82356001600160401b03811115614d33575f5ffd5b614d3f85828601614ccd565b90969095509350505050565b602080825282518282018190525f918401906040840190835b8181101561281a578351835260209384019390920191600101614d64565b5f60208284031215614d92575f5ffd5b5035919050565b634e487b7160e01b5f52604160045260245ffd5b60405160e081016001600160401b0381118282101715614dcf57614dcf614d99565b60405290565b604080519081016001600160401b0381118282101715614dcf57614dcf614d99565b604051601f8201601f191681016001600160401b0381118282101715614e1f57614e1f614d99565b604052919050565b5f6001600160401b03821115614e3f57614e3f614d99565b5060051b60200190565b5f82601f830112614e58575f5ffd5b8135614e6b614e6682614e27565b614df7565b8082825260208201915060208360051b860101925085831115614e8c575f5ffd5b602085015b83811015614eb2578035614ea481614c57565b835260209283019201614e91565b5095945050505050565b5f82601f830112614ecb575f5ffd5b8135614ed9614e6682614e27565b8082825260208201915060208360051b860101925085831115614efa575f5ffd5b602085015b83811015614eb2578035835260209283019201614eff565b5f5f5f60608486031215614f29575f5ffd5b8335614f3481614c57565b925060208401356001600160401b03811115614f4e575f5ffd5b614f5a86828701614e49565b92505060408401356001600160401b03811115614f75575f5ffd5b614f8186828701614ebc565b9150509250925092565b5f8151808452602084019350602083015f5b82811015614fbb578151865260209586019590910190600101614f9d565b5093949350505050565b602081525f610d6c6020830184614f8b565b803563ffffffff81168114612275575f5ffd5b5f5f83601f840112614ffa575f5ffd5b5081356001600160401b03811115615010575f5ffd5b602083019150836020828501011115612031575f5ffd5b5f5f5f5f6060858703121561503a575f5ffd5b843561504581614c57565b935061505360208601614fd7565b925060408501356001600160401b0381111561506d575f5ffd5b61507987828801614fea565b95989497509550505050565b6001600160a01b0391909116815260200190565b5f5f5f5f608085870312156150ac575f5ffd5b84356150b781614c57565b935060208501356150c781614c57565b93969395505050506040820135916060013590565b5f602082840312156150ec575f5ffd5b8135610d6c81614c57565b5f5f60408385031215615108575f5ffd5b823561511381614c57565b9150602083013561512381614c57565b809150509250929050565b5f5b83811015615148578181015183820152602001615130565b50505f910152565b5f815180845261516781602086016020860161512e565b601f01601f19169290920160200192915050565b602081525f610d6c6020830184615150565b5f60e0828403121561519d575f5ffd5b6151a5614dad565b90506151b082614c6b565b81526151be60208301614c6b565b60208201526151cf60408301614c6b565b6040820152606082810135908201526151ea60808301614fd7565b608082015260a08201356001600160401b03811115615207575f5ffd5b61521384828501614e49565b60a08301525060c08201356001600160401b03811115615231575f5ffd5b61523d84828501614ebc565b60c08301525092915050565b5f60208284031215615259575f5ffd5b81356001600160401b0381111561526e575f5ffd5b6119f28482850161518d565b5f6020828403121561528a575f5ffd5b813560ff81168114610d6c575f5ffd5b5f8151808452602084019350602083015f5b82811015614fbb5781516001600160a01b03168652602095860195909101906001016152ac565b80516001600160a01b03908116835260208083015182169084015260408083015190911690830152606080820151908301526080808201515f9161531e9085018263ffffffff169052565b5060a082015160e060a085015261533860e085018261529a565b905060c083015184820360c08601526143dc8282614f8b565b604081525f61536360408301856152d3565b82810360208401526143dc8185614f8b565b5f82825180855260208501945060208160051b830101602085015f5b8381101561437357601f198584030188526153ad838351614f8b565b6020988901989093509190910190600101615391565b5f604082016040835280855180835260608501915060608160051b8601019250602087015f5b8281101561541a57605f198786030184526154058583516152d3565b945060209384019391909101906001016153e9565b5050505082810360208401526143dc8185615375565b6001600160401b0381168114612456575f5ffd5b5f5f5f5f60808587031215615457575f5ffd5b843561546281614c57565b9350602085013561547281614c57565b9250604085013561548281615430565b9150606085013561549281615430565b939692955090935050565b5f5f5f606084860312156154af575f5ffd5b83356154ba81614c57565b92506020840135915060408401356154d181615430565b809150509250925092565b604081525f615363604083018561529a565b5f5f5f60408486031215615500575f5ffd5b833561550b81614c57565b925060208401356001600160401b03811115615525575f5ffd5b61553186828701614fea565b9497909650939450505050565b5f5f6040838503121561554f575f5ffd5b823561555a81614c57565b915060208301356001600160401b03811115615574575f5ffd5b61558085828601614e49565b9150509250929050565b5f5f5f5f5f5f6060878903121561559f575f5ffd5b86356001600160401b038111156155b4575f5ffd5b6155c089828a01614ccd565b90975095505060208701356001600160401b038111156155de575f5ffd5b6155ea89828a01614ccd565b90955093505060408701356001600160401b03811115615608575f5ffd5b61561489828a01614ccd565b979a9699509497509295939492505050565b602081525f610d6c60208301846152d3565b5f5f5f6060848603121561564a575f5ffd5b833561565581614c57565b925060208401356001600160401b0381111561566f575f5ffd5b840160408187031215615680575f5ffd5b615688614dd5565b81356001600160401b0381111561569d575f5ffd5b8201601f810188136156ad575f5ffd5b80356001600160401b038111156156c6576156c6614d99565b6156d9601f8201601f1916602001614df7565b8181528960208385010111156156ed575f5ffd5b816020840160208301375f60209282018301528352928301359282019290925293969395505050506040919091013590565b5f5f60408385031215615730575f5ffd5b823561573b81614c57565b946020939093013593505050565b604081525f6153636040830185614f8b565b8015158114612456575f5ffd5b5f5f5f5f6060858703121561577b575f5ffd5b84356001600160401b03811115615790575f5ffd5b850160e081880312156157a1575f5ffd5b935060208501356001600160401b038111156157bb575f5ffd5b6157c787828801614ccd565b90945092505060408501356154928161575b565b5f5f604083850312156157ec575f5ffd5b82356001600160401b03811115615801575f5ffd5b8301601f81018513615811575f5ffd5b803561581f614e6682614e27565b8082825260208201915060208360051b850101925087831115615840575f5ffd5b6020840193505b8284101561586b57833561585a81614c57565b825260209384019390910190615847565b945050505060208301356001600160401b03811115615574575f5ffd5b602081525f610d6c6020830184615375565b634e487b7160e01b5f52603260045260245ffd5b5f8235605e198336030181126158c2575f5ffd5b9190910192915050565b5f5f8335601e198436030181126158e1575f5ffd5b8301803591506001600160401b038211156158fa575f5ffd5b6020019150600581901b3603821315612031575f5ffd5b5f60208284031215615921575f5ffd5b8151610d6c8161575b565b60208152816020820152818360408301375f818301604090810191909152601f909201601f19160101919050565b6001600160a01b0392831681529116602082015260400190565b5f60208284031215615984575f5ffd5b8151610d6c81615430565b634e487b7160e01b5f52601160045260245ffd5b8082018082111561194f5761194f61598f565b5f82601f8301126159c5575f5ffd5b81516159d3614e6682614e27565b8082825260208201915060208360051b8601019250858311156159f4575f5ffd5b602085015b83811015614eb25780518352602092830192016159f9565b5f5f60408385031215615a22575f5ffd5b82516001600160401b03811115615a37575f5ffd5b8301601f81018513615a47575f5ffd5b8051615a55614e6682614e27565b8082825260208201915060208360051b850101925087831115615a76575f5ffd5b6020840193505b82841015615aa1578351615a9081614c57565b825260209384019390910190615a7d565b8095505050505060208301516001600160401b03811115615ac0575f5ffd5b615580858286016159b6565b5f60208284031215615adc575f5ffd5b5051919050565b5f823560de198336030181126158c2575f5ffd5b5f61194f368361518d565b5f60208284031215615b12575f5ffd5b8135610d6c8161575b565b5f60208284031215615b2d575f5ffd5b8151610d6c81614c57565b6001600160a01b03831681526040602082018190525f906119f29083018461529a565b5f60208284031215615b6b575f5ffd5b81516001600160401b03811115615b80575f5ffd5b8201601f81018413615b90575f5ffd5b8051615b9e614e6682614e27565b8082825260208201915060208360051b850101925086831115615bbf575f5ffd5b6020840193505b8284101561089e578351615bd981615430565b825260209384019390910190615bc6565b6001600160a01b039384168152919092166020820152604081019190915260600190565b5f60018201615c1f57615c1f61598f565b5060010190565b838152606060208201525f615c3e60608301856152d3565b828103604084015261089e8185614f8b565b63ffffffff818116838216019081111561194f5761194f61598f565b8181038181111561194f5761194f61598f565b63ffffffff828116828216039081111561194f5761194f61598f565b5f5f60408385031215615cac575f5ffd5b505080516020909101519092909150565b6001600160a01b03841681526060602082018190525f90615ce09083018561529a565b905063ffffffff83166040830152949350505050565b634e487b7160e01b5f52602160045260245ffd5b634e487b7160e01b5f52601260045260245ffd5b6001600160401b03828116828216039081111561194f5761194f61598f565b634e487b7160e01b5f52603160045260245ffd5b828152604060208201525f6119f26040830184615150565b5f82516158c281846020870161512e565b5f82615d9457634e487b7160e01b5f52601260045260245ffd5b50049056fe000000000000000000000000beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac0a2646970667358221220e1c0613b67b7d291dbd51e9b97ad592681c391bc5ea883c66bc8728ebbdeedd464736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01``@R4\x80\x15a\0\x10W__\xFD[P`@Qabj8\x03\x80abj\x839\x81\x01`@\x81\x90Ra\0/\x91a\x01\xFBV[\x80\x80\x84\x89\x89\x89\x87\x8A`\x01`\x01`\xA0\x1B\x03\x81\x16a\0^W`@Qc9\xB1\x90\xBB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x80R\x93\x84\x16`\xA0R\x91\x83\x16`\xC0R\x82\x16`\xE0Rc\xFF\xFF\xFF\xFF\x16a\x01\0R\x16a\x01 Ra\0\x95\x81a\0\xB0V[a\x01@RPa\0\xA4\x90Pa\0\xF6V[PPPPPPPa\x03\x82V[__\x82\x90P`\x1F\x81Q\x11\x15a\0\xE3W\x82`@Qc0Z'\xA9`\xE0\x1B\x81R`\x04\x01a\0\xDA\x91\x90a\x03*V[`@Q\x80\x91\x03\x90\xFD[\x80Qa\0\xEE\x82a\x03\\V[\x17\x93\x92PPPV[_Ta\x01\0\x90\x04`\xFF\x16\x15a\x01]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01a\0\xDAV[_T`\xFF\x90\x81\x16\x14a\x01\xACW_\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xC2W__\xFD[PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_[\x83\x81\x10\x15a\x01\xF3W\x81\x81\x01Q\x83\x82\x01R` \x01a\x01\xDBV[PP_\x91\x01RV[_______`\xE0\x88\x8A\x03\x12\x15a\x02\x11W__\xFD[\x87Qa\x02\x1C\x81a\x01\xAEV[` \x89\x01Q\x90\x97Pa\x02-\x81a\x01\xAEV[`@\x89\x01Q\x90\x96Pa\x02>\x81a\x01\xAEV[``\x89\x01Q\x90\x95Pa\x02O\x81a\x01\xAEV[`\x80\x89\x01Q\x90\x94Pa\x02`\x81a\x01\xAEV[`\xA0\x89\x01Q\x90\x93Pc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x02yW__\xFD[`\xC0\x89\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02\x94W__\xFD[\x88\x01_`\x1F\x82\x01\x8B\x13a\x02\xA5W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02\xBEWa\x02\xBEa\x01\xC5V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x02\xECWa\x02\xECa\x01\xC5V[`@R\x81\x81R\x83\x82\x01` \x01\x8D\x10\x15a\x03\x03W__\xFD[a\x03\x14\x82` \x83\x01` \x87\x01a\x01\xD9V[\x80\x92P\x80\x94PPPPP\x92\x95\x98\x91\x94\x97P\x92\x95PV[` \x81R_\x82Q\x80` \x84\x01Ra\x03H\x81`@\x85\x01` \x87\x01a\x01\xD9V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\x03|W_\x19\x81` \x03`\x03\x1B\x1B\x82\x16\x91P[P\x91\x90PV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa]\xEFa\x04{_9_\x81\x81a\x10\xCF\x01Ra@r\x01R_\x81\x81a\x03\xFE\x01Ra3X\x01R_\x81\x81a\x07\x01\x01R\x81\x81a5h\x01R\x81\x81a6\xAA\x01Ra9\x86\x01R_\x81\x81a\x07Q\x01R\x81\x81a\r\xC3\x01R\x81\x81a\x0Fz\x01R\x81\x81a\x13-\x01R\x81\x81a\x15%\x01R\x81\x81a\x19Y\x01R\x81\x81a'\x1E\x01RaB\x9C\x01R_\x81\x81a\x04%\x01R\x81\x81a\x0F\x07\x01R\x81\x81a\x14\xA5\x01R\x81\x81a\x16\xC9\x01R\x81\x81a/O\x01R\x81\x81a11\x01Ra7\xE9\x01R_\x81\x81a\x03\x81\x01R\x81\x81a\x0E\xD5\x01R\x81\x81a\x167\x01Ra7\xC3\x01R_\x81\x81a\x05\xD6\x01R\x81\x81a\x0BP\x01R\x81\x81a\x11\x0F\x01Ra%\x14\x01Ra]\xEF_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x02\xC3W_5`\xE0\x1C\x80cqP\x18\xA6\x11a\x01wW\x80c\xBF\xAE?\xD2\x11a\0\xD5W\x80c\xE4\xCC?\x90\x11a\0\x8FW\x80c\xE4\xCC?\x90\x14a\x07\x99W\x80c\xEE\xA9\x06K\x14a\x07\xACW\x80c\xF0\xE0\xE6v\x14a\x07\xBFW\x80c\xF2\xFD\xE3\x8B\x14a\x07\xDFW\x80c\xF6\x98\xDA%\x14a\x07\xF2W\x80c\xFA\xBC\x1C\xBC\x14a\x07\xFAW\x80c\xFD\x8A\xA8\x8D\x14a\x08\rW__\xFD[\x80c\xBF\xAE?\xD2\x14a\x06\xE4W\x80c\xC4H\xFE\xB8\x14a\x06\xF7W\x80c\xC9x\xF7\xAC\x14a\x07+W\x80c\xCA\x8A\xA7\xC7\x14a\x07LW\x80c\xCDm\xC6\x87\x14a\x07sW\x80c\xDA\x8B\xE8d\x14a\x07\x86W__\xFD[\x80c\x91\x04\xC3\x19\x11a\x011W\x80c\x91\x04\xC3\x19\x14a\x06\x1CW\x80c\x945\xBBC\x14a\x060W\x80c\x99\xF57\x1B\x14a\x06CW\x80c\xA1x\x84\x84\x14a\x06cW\x80c\xA3:43\x14a\x06\x82W\x80c\xB7\xF0n\xBE\x14a\x06\x95W\x80c\xBBE\xFE\xF2\x14a\x06\xB7W__\xFD[\x80cqP\x18\xA6\x14a\x05\x8CW\x80cw\x8EU\xF3\x14a\x05\x94W\x80cx)n\xC5\x14a\x05\xBEW\x80c\x88o\x11\x95\x14a\x05\xD1W\x80c\x8D\xA5\xCB[\x14a\x05\xF8W\x80c\x90\x04\x13G\x14a\x06\tW__\xFD[\x80cT\xFDMP\x11a\x02$W\x80c]\xD6\x85y\x11a\x01\xDEW\x80c]\xD6\x85y\x14a\x04\xD6W\x80c`\x1B\xB3o\x14a\x04\xF7W\x80c`\xA0\xD1\xCE\x14a\x05\nW\x80ce\xDA\x12d\x14a\x05\x1DW\x80cf\xD5\xBA\x93\x14a\x05EW\x80cmp\xF7\xAE\x14a\x05fW\x80cn\x17DH\x14a\x05yW__\xFD[\x80cT\xFDMP\x14a\x04ZW\x80cY\\jg\x14a\x04oW\x80cY{6\xDA\x14a\x04wW\x80cZ\xC8j\xB7\x14a\x04\x8AW\x80c\\\x97Z\xBB\x14a\x04\xADW\x80c]\x97^\x88\x14a\x04\xB5W__\xFD[\x80c9\xB7\x0E8\x11a\x02\x80W\x80c9\xB7\x0E8\x14a\x03|W\x80c<e\x1C\xF2\x14a\x03\xB0W\x80c<\xDE\xB5\xE0\x14a\x03\xC3W\x80c>(9\x1D\x14a\x03\xD6W\x80cFW\xE2j\x14a\x03\xF9W\x80cFe\xBC\xDA\x14a\x04 W\x80cT\xB7\xC9l\x14a\x04GW__\xFD[\x80c\x04\xA4\xF9y\x14a\x02\xC7W\x80c\x0B\x9FHz\x14a\x03\x01W\x80c\r\xD8\xDD\x02\x14a\x03\x14W\x80c\x13d9\xDD\x14a\x034W\x80c%\xDF\x92.\x14a\x03IW\x80c*\xA6\xD8\x88\x14a\x03iW[__\xFD[a\x02\xEE\x7F\x14\xBD\xE6t\xC9\xF6K*\xD0\x0E\xAA\xEEJ\x8B\xED\x1F\xAB\xEF5\xC7P~<[\x9C\xFC\x946\x90\x9A-\xAD\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\xEEa\x03\x0F6`\x04aLvV[a\x08 V[a\x03'a\x03\"6`\x04aM\rV[a\x08\xA8V[`@Qa\x02\xF8\x91\x90aMKV[a\x03Ga\x03B6`\x04aM\x82V[a\x0B9V[\0[a\x03\\a\x03W6`\x04aO\x17V[a\x0C\x13V[`@Qa\x02\xF8\x91\x90aO\xC5V[a\x03Ga\x03w6`\x04aP'V[a\rsV[a\x03\xA3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qa\x02\xF8\x91\x90aP\x85V[a\x03Ga\x03\xBE6`\x04aP\x99V[a\x0E\xCAV[a\x03\xA3a\x03\xD16`\x04aP\xDCV[a\x10\x1BV[a\x03\xE9a\x03\xE46`\x04aP\xDCV[a\x10;V[`@Q\x90\x15\x15\x81R` \x01a\x02\xF8V[a\x03\xA3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\xA3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03Ga\x04U6`\x04aP\xF7V[a\x10ZV[a\x04ba\x10\xC8V[`@Qa\x02\xF8\x91\x90aQ{V[a\x03Ga\x10\xF8V[a\x02\xEEa\x04\x856`\x04aRIV[a\x11\xACV[a\x03\xE9a\x04\x986`\x04aRzV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fTa\x02\xEEV[a\x04\xC8a\x04\xC36`\x04aM\x82V[a\x11\xDBV[`@Qa\x02\xF8\x92\x91\x90aSQV[a\x04\xE9a\x04\xE46`\x04aP\xDCV[a\x11\xF8V[`@Qa\x02\xF8\x92\x91\x90aS\xC3V[a\x03Ga\x05\x056`\x04aTDV[a\x13\"V[a\x03Ga\x05\x186`\x04aT\x9DV[a\x14\x9AV[a\x03\xA3a\x05+6`\x04aP\xDCV[`\x9A` R_\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x05Xa\x05S6`\x04aP\xDCV[a\x160V[`@Qa\x02\xF8\x92\x91\x90aT\xDCV[a\x03\xE9a\x05t6`\x04aP\xDCV[a\x19\x1DV[a\x02\xEEa\x05\x876`\x04aP\xF7V[a\x19UV[a\x03Ga\x19\xFAV[a\x02\xEEa\x05\xA26`\x04aP\xF7V[`\x98` \x90\x81R_\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[a\x03Ga\x05\xCC6`\x04aT\xEEV[a\x1A\x0BV[a\x03\xA3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03\xA3V[a\x03\\a\x06\x176`\x04aU>V[a\x1A\xA1V[a\x03\xA3_Q` a]\x9A_9_Q\x90_R\x81V[a\x03Ga\x06>6`\x04aU\x8AV[a\x1BwV[a\x06Va\x06Q6`\x04aM\x82V[a\x1CPV[`@Qa\x02\xF8\x91\x90aV&V[a\x02\xEEa\x06q6`\x04aP\xDCV[`\x9F` R_\x90\x81R`@\x90 T\x81V[a\x03'a\x06\x906`\x04aV8V[a\x1DlV[a\x03\xE9a\x06\xA36`\x04aM\x82V[`\x9E` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x03\xE9a\x06\xC56`\x04aW\x1FV[`\x9C` \x90\x81R_\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[a\x02\xEEa\x06\xF26`\x04aP\xF7V[a\x1D\x84V[`@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R` \x01a\x02\xF8V[a\x07>a\x0796`\x04aU>V[a\x1D\xC0V[`@Qa\x02\xF8\x92\x91\x90aWIV[a\x03\xA3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03Ga\x07\x816`\x04aW\x1FV[a 8V[a\x03'a\x07\x946`\x04aP\xDCV[a!SV[a\x03Ga\x07\xA76`\x04aWhV[a\"zV[a\x03Ga\x07\xBA6`\x04aV8V[a\"\xD0V[a\x07\xD2a\x07\xCD6`\x04aW\xDBV[a#;V[`@Qa\x02\xF8\x91\x90aX\x88V[a\x03Ga\x07\xED6`\x04aP\xDCV[a#\xE0V[a\x02\xEEa$YV[a\x03Ga\x08\x086`\x04aM\x82V[a%\x12V[a\x03'a\x08\x1B6`\x04aP\xDCV[a&)V[`@\x80Q\x7F\x14\xBD\xE6t\xC9\xF6K*\xD0\x0E\xAA\xEEJ\x8B\xED\x1F\xAB\xEF5\xC7P~<[\x9C\xFC\x946\x90\x9A-\xAD` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x92\x82\x01\x92\x90\x92R\x81\x87\x16``\x82\x01R\x90\x85\x16`\x80\x82\x01R`\xA0\x81\x01\x83\x90R`\xC0\x81\x01\x82\x90R_\x90a\x08\x9E\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a&LV[\x96\x95PPPPPPV[`fT``\x90`\x01\x90`\x02\x90\x81\x16\x03a\x08\xD4W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08\xDCa&zV[_\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08\xF5Wa\x08\xF5aM\x99V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\t\x1EW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P3_\x90\x81R`\x9A` R`@\x81 T\x91\x92P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90[\x85\x81\x10\x15a\x0B*W\x86\x86\x82\x81\x81\x10a\tXWa\tXaX\x9AV[\x90P` \x02\x81\x01\x90a\tj\x91\x90aX\xAEV[a\tx\x90` \x81\x01\x90aX\xCCV[\x90P\x87\x87\x83\x81\x81\x10a\t\x8CWa\t\x8CaX\x9AV[\x90P` \x02\x81\x01\x90a\t\x9E\x91\x90aX\xAEV[a\t\xA8\x90\x80aX\xCCV[\x90P\x14a\t\xC8W`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\n23\x84\x8A\x8A\x86\x81\x81\x10a\t\xE0Wa\t\xE0aX\x9AV[\x90P` \x02\x81\x01\x90a\t\xF2\x91\x90aX\xAEV[a\t\xFC\x90\x80aX\xCCV[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RPa&\xD3\x92PPPV[\x90Pa\x0B\x043\x84\x8A\x8A\x86\x81\x81\x10a\nKWa\nKaX\x9AV[\x90P` \x02\x81\x01\x90a\n]\x91\x90aX\xAEV[a\ng\x90\x80aX\xCCV[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RP\x8E\x92P\x8D\x91P\x88\x90P\x81\x81\x10a\n\xACWa\n\xACaX\x9AV[\x90P` \x02\x81\x01\x90a\n\xBE\x91\x90aX\xAEV[a\n\xCC\x90` \x81\x01\x90aX\xCCV[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RP\x88\x92Pa(%\x91PPV[\x84\x83\x81Q\x81\x10a\x0B\x16Wa\x0B\x16aX\x9AV[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\t>V[PP`\x01`\xC9U\x94\x93PPPPV[`@Qc#}\xFBG`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cF\xFB\xF6\x8E\x90a\x0B\x85\x903\x90`\x04\x01aP\x85V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xA0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xC4\x91\x90aY\x11V[a\x0B\xE1W`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`fT\x81\x81\x16\x81\x14a\x0C\x06W`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0C\x0F\x82a-\x94V[PPV[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16_\x90\x81R`\x9A` R`@\x81 T``\x92\x16\x90a\x0C<\x86\x83\x87a&\xD3V[\x90P_\x85Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0CXWa\x0CXaM\x99V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C\x81W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x86Q\x81\x10\x15a\rfW`\x01`\x01`\xA0\x1B\x03\x88\x16_\x90\x81R`\xA2` R`@\x81 \x88Q\x82\x90\x8A\x90\x85\x90\x81\x10a\x0C\xBCWa\x0C\xBCaX\x9AV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80` \x01`@R\x90\x81_\x82\x01T\x81RPP\x90Pa\r@\x87\x83\x81Q\x81\x10a\r\x0EWa\r\x0EaX\x9AV[` \x02` \x01\x01Q\x85\x84\x81Q\x81\x10a\r(Wa\r(aX\x9AV[` \x02` \x01\x01Q\x83a-\xD1\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83\x83\x81Q\x81\x10a\rRWa\rRaX\x9AV[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x0C\x86V[P\x92PPP[\x93\x92PPPV[a\r{a&zV[a\r\x843a\x10;V[\x15a\r\xA2W`@Qc;\xF2\xB5\x03`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc+bA\xF3`\xE1\x1B\x81R3`\x04\x82\x01Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cV\xC4\x83\xE6\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0E\x0CW__\xFD[PZ\xF1\x15\x80\x15a\x0E\x1EW=__>=_\xFD[PPPPa\x0E,3\x85a-\xEFV[a\x0E633a.QV[3`\x01`\x01`\xA0\x1B\x03\x16\x7F\xA4S\xDBa*\xF5\x9EU!\xD6\xAB\x92\x84\xDC>-\x06\xAF(n\xB1\xB1\xB7\xB7q\xFC\xE4ql\x19\xF2\xC1\x85`@Qa\x0Eo\x91\x90aP\x85V[`@Q\x80\x91\x03\x90\xA23`\x01`\x01`\xA0\x1B\x03\x16\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x83\x83`@Qa\x0E\xB2\x92\x91\x90aY,V[`@Q\x80\x91\x03\x90\xA2a\x0E\xC4`\x01`\xC9UV[PPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x0F)WP3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14[a\x0FFW`@Qc\x04R\x06\xA5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0FNa&zV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16_\x90\x81R`\x9A` R`@\x80\x82 T\x90Qc\x15&g\xD9`\xE3\x1B\x81R\x90\x83\x16\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA93>\xC8\x90a\x0F\xB1\x90\x85\x90\x89\x90`\x04\x01aYZV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xCCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xF0\x91\x90aYtV[\x90P_a\x0F\xFE\x87\x87\x84a0\xF2V[\x90Pa\x10\x0E\x83\x88\x88\x88\x88\x86a1\xD5V[PPPa\x0E\xC4`\x01`\xC9UV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`\x99` R`@\x90 `\x01\x01T\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`\x9A` R`@\x90 T\x16\x15\x15\x90V[\x81a\x10d\x81a3\x1AV[a\x10\x81W`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x10\x89a&zV[a\x10\x92\x83a\x19\x1DV[a\x10\xAFW`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x10\xB9\x83\x83a-\xEFV[a\x10\xC3`\x01`\xC9UV[PPPV[``a\x10\xF3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a3\xC4V[\x90P\x90V[`@Qc#}\xFBG`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cF\xFB\xF6\x8E\x90a\x11D\x903\x90`\x04\x01aP\x85V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11_W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x83\x91\x90aY\x11V[a\x11\xA0W`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x11\xAA_\x19a-\x94V[V[_\x81`@Q` \x01a\x11\xBE\x91\x90aV&V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[a\x11\xE3aK7V[``a\x11\xEE\x83a4\x01V[\x90\x94\x90\x93P\x91PPV[``\x80_a\x12\x05\x84a&)V[\x80Q\x90\x91P\x80`\x01`\x01`@\x1B\x03\x81\x11\x15a\x12\"Wa\x12\"aM\x99V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x12[W\x81` \x01[a\x12HaK7V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x12@W\x90P[P\x93P\x80`\x01`\x01`@\x1B\x03\x81\x11\x15a\x12vWa\x12vaM\x99V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x12\xA9W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x12\x94W\x90P[P\x92P_[\x81\x81\x10\x15a\x13\x1AWa\x12\xD8\x83\x82\x81Q\x81\x10a\x12\xCBWa\x12\xCBaX\x9AV[` \x02` \x01\x01Qa4\x01V[\x86\x83\x81Q\x81\x10a\x12\xEAWa\x12\xEAaX\x9AV[` \x02` \x01\x01\x86\x84\x81Q\x81\x10a\x13\x03Wa\x13\x03aX\x9AV[` \x90\x81\x02\x91\x90\x91\x01\x01\x91\x90\x91RR`\x01\x01a\x12\xAEV[PPP\x91P\x91V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x13kW`@Qc#\xD8q\xA5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x13sa&zV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16_\x90\x81R`\x98` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R\x90\x81 Ta\x13\xB1\x90`\x01`\x01`@\x1B\x03\x80\x86\x16\x90\x85\x16a6TV[\x90P_a\x13\xC0\x86\x86\x86\x86a6lV[\x90P_a\x13\xCD\x82\x84aY\xA3V[\x90Pa\x13\xDB\x87_\x88\x86a7)V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x88\x81\x16\x82R` \x82\x01\x84\x90R\x89\x16\x91\x7F\xDDa\x1FN\xF6?C\x85\xF1ul\x86\xCE\x1F\x1F8\x9A\x90\x13\xBAo\xA0}\xAB\xA8R\x82\x91\xBC-<0\x91\x01`@Q\x80\x91\x03\x90\xA2_a\x14+\x87a7\xA3V[`@Qc\xDE\xBE\x1E\xAB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16`\x04\x83\x01R`$\x82\x01\x85\x90R\x91\x92P\x90\x82\x16\x90c\xDE\xBE\x1E\xAB\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x14vW__\xFD[PZ\xF1\x15\x80\x15a\x14\x88W=__>=_\xFD[PPPPPPPPa\x0E\xC4`\x01`\xC9UV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x14\xE3W`@Qc2\x13\xA6a`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x14\xEBa&zV[a\x14\xF4\x83a\x10;V[\x15a\x10\xB9W`\x01`\x01`\xA0\x1B\x03\x80\x84\x16_\x90\x81R`\x9A` R`@\x80\x82 T\x90Qc\x15&g\xD9`\xE3\x1B\x81R\x90\x83\x16\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA93>\xC8\x90a\x15i\x90\x85\x90_Q` a]\x9A_9_Q\x90_R\x90`\x04\x01aYZV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\x84W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xA8\x91\x90aYtV[`\x01`\x01`\xA0\x1B\x03\x86\x16_\x90\x81R`\xA2` \x90\x81R`@\x80\x83 _Q` a]\x9A_9_Q\x90_R\x84R\x82R\x80\x83 \x81Q\x92\x83\x01\x90\x91RT\x81R\x91\x92Pa\x16\x07\x86a\x15\xFF`\x01`\x01`@\x1B\x03\x80\x87\x16\x90\x89\x16a8\x0EV[\x84\x91\x90a8\"V[\x90Pa\x16\"\x84\x88_Q` a]\x9A_9_Q\x90_R\x84a7)V[PPPPa\x10\xC3`\x01`\xC9UV[``\x80__\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x94\xF6I\xDD\x86`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16\x81\x91\x90aP\x85V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\x9BW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x16\xC2\x91\x90\x81\x01\x90aZ\x11V[\x91P\x91P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xFE$:\x17\x87_Q` a]\x9A_9_Q\x90_R`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\"\x92\x91\x90aYZV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17=W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17a\x91\x90aZ\xCCV[\x90P\x80_\x03a\x17uWP\x90\x94\x90\x93P\x91PPV[_\x83Q`\x01a\x17\x84\x91\x90aY\xA3V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17\x9BWa\x17\x9BaM\x99V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x17\xC4W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x84Q`\x01a\x17\xD6\x91\x90aY\xA3V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17\xEDWa\x17\xEDaM\x99V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x18\x16W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_Q` a]\x9A_9_Q\x90_R\x82\x86Q\x81Q\x81\x10a\x18:Wa\x18:aX\x9AV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x82\x81\x86Q\x81Q\x81\x10a\x18nWa\x18naX\x9AV[` \x90\x81\x02\x91\x90\x91\x01\x01R_[\x85Q\x81\x10\x15a\x19\x0FW\x85\x81\x81Q\x81\x10a\x18\x96Wa\x18\x96aX\x9AV[` \x02` \x01\x01Q\x83\x82\x81Q\x81\x10a\x18\xB0Wa\x18\xB0aX\x9AV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x84\x81\x81Q\x81\x10a\x18\xE2Wa\x18\xE2aX\x9AV[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a\x18\xFCWa\x18\xFCaX\x9AV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x18{V[P\x90\x97\x90\x96P\x94PPPPPV[_`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x80\x15\x90a\x19OWP`\x01`\x01`\xA0\x1B\x03\x80\x83\x16_\x81\x81R`\x9A` R`@\x90 T\x90\x91\x16\x14[\x92\x91PPV[__\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA93>\xC8\x85\x85`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19\xA5\x92\x91\x90aYZV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\xC0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\xE4\x91\x90aYtV[\x90Pa\x19\xF2\x84\x84\x83_a6lV[\x94\x93PPPPV[a\x1A\x02a8@V[a\x11\xAA_a8\x9AV[\x82a\x1A\x15\x81a3\x1AV[a\x1A2W`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1A;\x84a\x19\x1DV[a\x1AXW`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x84\x84`@Qa\x1A\x93\x92\x91\x90aY,V[`@Q\x80\x91\x03\x90\xA2PPPPV[``_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1A\xBDWa\x1A\xBDaM\x99V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1A\xE6W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83Q\x81\x10\x15a\x1BoW`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x98` R`@\x81 \x85Q\x90\x91\x90\x86\x90\x84\x90\x81\x10a\x1B\"Wa\x1B\"aX\x9AV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ T\x82\x82\x81Q\x81\x10a\x1B\\Wa\x1B\\aX\x9AV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x1A\xEBV[P\x93\x92PPPV[`fT`\x02\x90`\x04\x90\x81\x16\x03a\x1B\xA0W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1B\xA8a&zV[\x85_[\x81\x81\x10\x15a\x1C;Wa\x1C3\x89\x89\x83\x81\x81\x10a\x1B\xC8Wa\x1B\xC8aX\x9AV[\x90P` \x02\x81\x01\x90a\x1B\xDA\x91\x90aZ\xE3V[a\x1B\xE3\x90aZ\xF7V[\x88\x88\x84\x81\x81\x10a\x1B\xF5Wa\x1B\xF5aX\x9AV[\x90P` \x02\x81\x01\x90a\x1C\x07\x91\x90aX\xCCV[\x88\x88\x86\x81\x81\x10a\x1C\x19Wa\x1C\x19aX\x9AV[\x90P` \x02\x01` \x81\x01\x90a\x1C.\x91\x90a[\x02V[a8\xEBV[`\x01\x01a\x1B\xABV[PPa\x1CG`\x01`\xC9UV[PPPPPPPV[a\x1CXaK7V[_\x82\x81R`\xA4` \x90\x81R`@\x91\x82\x90 \x82Q`\xE0\x81\x01\x84R\x81T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R`\x01\x83\x01T\x81\x16\x82\x85\x01R`\x02\x83\x01T\x16\x81\x85\x01R`\x03\x82\x01T``\x82\x01R`\x04\x82\x01Tc\xFF\xFF\xFF\xFF\x16`\x80\x82\x01R`\x05\x82\x01\x80T\x85Q\x81\x86\x02\x81\x01\x86\x01\x90\x96R\x80\x86R\x91\x94\x92\x93`\xA0\x86\x01\x93\x92\x90\x83\x01\x82\x82\x80\x15a\x1D\x06W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x1C\xE8W[PPPPP\x81R` \x01`\x06\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x1D\\W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x1DHW[PPPPP\x81RPP\x90P\x91\x90PV[``a\x1Dw3a!SV[\x90Pa\rl\x84\x84\x84a\"\xD0V[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16_\x90\x81R`\xA2` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x81R\x82\x82 \x83Q\x91\x82\x01\x90\x93R\x91T\x82R\x90a\rl\x90a=-V[``\x80\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\xDCWa\x1D\xDCaM\x99V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1E\x05W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E!Wa\x1E!aM\x99V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1EJW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\x9A` R`@\x81 T\x92\x93P\x91\x16\x90a\x1Eu\x86\x83\x87a&\xD3V[\x90P_[\x85Q\x81\x10\x15a -W_a\x1E\xA5\x87\x83\x81Q\x81\x10a\x1E\x98Wa\x1E\x98aX\x9AV[` \x02` \x01\x01Qa7\xA3V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xFE$:\x17\x89\x89\x85\x81Q\x81\x10a\x1E\xC9Wa\x1E\xC9aX\x9AV[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1E\xEE\x92\x91\x90aYZV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\tW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F-\x91\x90aZ\xCCV[\x85\x83\x81Q\x81\x10a\x1F?Wa\x1F?aX\x9AV[` \x02` \x01\x01\x81\x81RPP_`\xA2_\x8A`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _\x89\x85\x81Q\x81\x10a\x1F\x82Wa\x1F\x82aX\x9AV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80` \x01`@R\x90\x81_\x82\x01T\x81RPP\x90Pa \x06\x86\x84\x81Q\x81\x10a\x1F\xD4Wa\x1F\xD4aX\x9AV[` \x02` \x01\x01Q\x85\x85\x81Q\x81\x10a\x1F\xEEWa\x1F\xEEaX\x9AV[` \x02` \x01\x01Q\x83a8\"\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x87\x84\x81Q\x81\x10a \x18Wa \x18aX\x9AV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x1EyV[PPP[\x92P\x92\x90PV[_Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a VWP_T`\x01`\xFF\x90\x91\x16\x10[\x80a oWP0;\x15\x80\x15a oWP_T`\xFF\x16`\x01\x14[a \xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a \xF8W_\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a!\x01\x82a-\x94V[a!\n\x83a8\x9AV[\x80\x15a\x10\xC3W_\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPV[``a!]a&zV[a!f\x82a\x10;V[a!\x83W`@Qc\xA5\xC7\xC4E`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a!\x8C\x82a\x19\x1DV[\x15a!\xAAW`@Qc\x11\xCA35`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3`\x01`\x01`\xA0\x1B\x03\x83\x16\x14a\"`W`\x01`\x01`\xA0\x1B\x03\x80\x83\x16_\x90\x81R`\x9A` R`@\x90 T\x16a!\xDD\x81a3\x1AV[\x80a\"\x01WPa!\xEC\x81a\x10\x1BV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14[a\"\x1EW`@Qc\x1EI\x9A#`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xF0\xED\xDF\x07\xE6\xEA\x14\xF3\x88\xB4~\x1E\x94\xA0\xF4d\xEC\xBD\x9E\xEDAq\x13\x0E\x0F\xC0\xE9\x9F\xB4\x03\n\x8A`@Q`@Q\x80\x91\x03\x90\xA3P[a\"i\x82a=LV[\x90Pa\"u`\x01`\xC9UV[\x91\x90PV[`fT`\x02\x90`\x04\x90\x81\x16\x03a\"\xA3W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\"\xABa&zV[a\"\xBFa\"\xB7\x86aZ\xF7V[\x85\x85\x85a8\xEBV[a\"\xC9`\x01`\xC9UV[PPPPPV[a\"\xD8a&zV[a\"\xE13a\x10;V[\x15a\"\xFFW`@Qc;\xF2\xB5\x03`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a#\x08\x83a\x19\x1DV[a#%W`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a#13\x84\x84\x84a?\xABV[a\x10\xB93\x84a.QV[``_\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15a#WWa#WaM\x99V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a#\x8AW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a#uW\x90P[P\x90P_[\x84Q\x81\x10\x15a\x1BoWa#\xBB\x85\x82\x81Q\x81\x10a#\xADWa#\xADaX\x9AV[` \x02` \x01\x01Q\x85a\x1A\xA1V[\x82\x82\x81Q\x81\x10a#\xCDWa#\xCDaX\x9AV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a#\x8FV[a#\xE8a8@V[`\x01`\x01`\xA0\x1B\x03\x81\x16a$MW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a \xCEV[a$V\x81a8\x9AV[PV[`@\x80Q\x80\x82\x01\x90\x91R`\n\x81Ri\"\xB4\xB3\xB2\xB7&0\xBC\xB2\xB9`\xB1\x1B` \x90\x91\x01R_\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x7Fq\xB6%\xCF\xADD\xBA\xC6;\x13\xDB\xA0\x7F.\x1D`\x84\xEE\x04\xB6\xF8u!\x01\xEC\xE6\x12mXN\xE6\xEAa$\xC6a@jV[\x80Q` \x91\x82\x01 `@\x80Q\x92\x83\x01\x94\x90\x94R\x92\x81\x01\x91\x90\x91R``\x81\x01\x91\x90\x91RF`\x80\x82\x01R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%nW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%\x92\x91\x90a[\x1DV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a%\xC3W`@QcyH!\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`fT\x80\x19\x82\x19\x81\x16\x14a%\xEAW`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`f\x82\x90U`@Q\x82\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01[`@Q\x80\x91\x03\x90\xA2PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\xA3` R`@\x90 ``\x90a\x19O\x90aA\x06V[_a&Ua$YV[`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x91\x90\x91R`B\x81\x01\x83\x90R`b\x01a\x11\xBEV[`\x02`\xC9T\x03a&\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a \xCEV[`\x02`\xC9UV[``_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a&\xEFWa&\xEFaM\x99V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a'\x18W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cTz\xFB\x87\x86\x86`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a'j\x92\x91\x90a[8V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'\x84W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra'\xAB\x91\x90\x81\x01\x90a[[V[\x90P_[\x84Q\x81\x10\x15a(\x1AWa'\xF5\x87\x86\x83\x81Q\x81\x10a'\xCEWa'\xCEaX\x9AV[` \x02` \x01\x01Q\x84\x84\x81Q\x81\x10a'\xE8Wa'\xE8aX\x9AV[` \x02` \x01\x01Qa0\xF2V[\x83\x82\x81Q\x81\x10a(\x07Wa(\x07aX\x9AV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a'\xAFV[P\x90\x95\x94PPPPPV[_`\x01`\x01`\xA0\x1B\x03\x86\x16a(MW`@Qc9\xB1\x90\xBB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83Q_\x03a(nW`@Qcyl\xC5%`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84Q`\x01`\x01`@\x1B\x03\x81\x11\x15a(\x88Wa(\x88aM\x99V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a(\xB1W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x85Q`\x01`\x01`@\x1B\x03\x81\x11\x15a(\xCEWa(\xCEaM\x99V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a(\xF7W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x86Q\x81\x10\x15a+\xC7W_a)\x1B\x88\x83\x81Q\x81\x10a\x1E\x98Wa\x1E\x98aX\x9AV[\x90P_`\xA2_\x8C`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _\x8A\x85\x81Q\x81\x10a)TWa)TaX\x9AV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80` \x01`@R\x90\x81_\x82\x01T\x81RPP\x90Pa)\xC0\x88\x84\x81Q\x81\x10a)\xA6Wa)\xA6aX\x9AV[` \x02` \x01\x01Q\x88\x85\x81Q\x81\x10a\x1F\xEEWa\x1F\xEEaX\x9AV[\x84\x84\x81Q\x81\x10a)\xD2Wa)\xD2aX\x9AV[` \x02` \x01\x01\x81\x81RPPa*\n\x88\x84\x81Q\x81\x10a)\xF3Wa)\xF3aX\x9AV[` \x02` \x01\x01Q\x82aA\x12\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x85\x84\x81Q\x81\x10a*\x1CWa*\x1CaX\x9AV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x15a*\xB1Wa*s\x8A\x8A\x85\x81Q\x81\x10a*LWa*LaX\x9AV[` \x02` \x01\x01Q\x87\x86\x81Q\x81\x10a*fWa*faX\x9AV[` \x02` \x01\x01QaA&V[a*\xB1\x8A\x8C\x8B\x86\x81Q\x81\x10a*\x8AWa*\x8AaX\x9AV[` \x02` \x01\x01Q\x87\x87\x81Q\x81\x10a*\xA4Wa*\xA4aX\x9AV[` \x02` \x01\x01Qa7)V[_\x82`\x01`\x01`\xA0\x1B\x03\x16crJ\xF4#\x8D\x8C\x87\x81Q\x81\x10a*\xD4Wa*\xD4aX\x9AV[` \x02` \x01\x01Q\x8C\x88\x81Q\x81\x10a*\xEEWa*\xEEaX\x9AV[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a+\x14\x93\x92\x91\x90a[\xEAV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a+0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+T\x91\x90aZ\xCCV[\x90P\x80_\x03a+\xB9W`\x01`\x01`\xA0\x1B\x03\x8C\x16_\x90\x81R`\xA2` R`@\x81 \x8BQa+\xB9\x92\x90\x8D\x90\x88\x90\x81\x10a+\x8DWa+\x8DaX\x9AV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _\x90UV[PPP\x80`\x01\x01\x90Pa(\xFCV[P`\x01`\x01`\xA0\x1B\x03\x88\x16_\x90\x81R`\x9F` R`@\x81 \x80T\x91\x82\x91\x90a+\xEE\x83a\\\x0EV[\x91\x90PUP_`@Q\x80`\xE0\x01`@R\x80\x8B`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8A`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8B`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x89\x81R` \x01\x85\x81RP\x90P_a,T\x82a\x11\xACV[_\x81\x81R`\x9E` \x90\x81R`@\x80\x83 \x80T`\x01`\xFF\x19\x90\x91\x16\x81\x17\x90\x91U`\xA4\x83R\x92\x81\x90 \x86Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x83U\x88\x85\x01Q\x95\x83\x01\x80T\x82\x16\x96\x83\x16\x96\x90\x96\x17\x90\x95U\x91\x87\x01Q`\x02\x82\x01\x80T\x90\x95\x16\x92\x16\x91\x90\x91\x17\x90\x92U``\x85\x01Q`\x03\x83\x01U`\x80\x85\x01Q`\x04\x83\x01\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U`\xA0\x85\x01Q\x80Q\x93\x94P\x85\x93a-\n\x92`\x05\x85\x01\x92\x01\x90aK\x90V[P`\xC0\x82\x01Q\x80Qa-&\x91`\x06\x84\x01\x91` \x90\x91\x01\x90aK\xF3V[PPP`\x01`\x01`\xA0\x1B\x03\x8B\x16_\x90\x81R`\xA3` R`@\x90 a-J\x90\x82aA\x90V[P\x7F&\xB2\xAA\xE2e\x16\xE8q\x9E\xF5\x0E\xA2\xF6\x83\x1A.\xFB\xD4\xE3}\xCC\xDF\x0Fi6\xB2{\xC0\x8Ey>0\x81\x83\x86`@Qa-~\x93\x92\x91\x90a\\&V[`@Q\x80\x91\x03\x90\xA1\x9A\x99PPPPPPPPPPV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2PV[_a\x19\xF2\x82a-\xE9a-\xE2\x87a=-V[\x86\x90aA\x9BV[\x90aA\x9BV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x81\x81R`\x99` R`@\x90\x81\x90 `\x01\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x93\x85\x16\x93\x90\x93\x17\x90\x92U\x90Q\x7Fw;T\xC0Muo\xCC^g\x81\x11\xF7\xD70\xDE;\xE9\x81\x92\0\x07\x99\xEE\xE3\xD67\x16\x05Z\x87\xC6\x90a&\x1D\x90\x84\x90aP\x85V[`fT_\x90`\x01\x90\x81\x16\x03a.yW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__a.\x84\x85a\x160V[\x91P\x91P_a.\x94_\x86\x85a&\xD3V[`\x01`\x01`\xA0\x1B\x03\x87\x81\x16_\x81\x81R`\x9A` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x94\x8B\x16\x94\x85\x17\x90UQ\x93\x94P\x91\x92\x90\x91\x7F\xC3\xEE\x9F._\xDA\x98\xE8\x06j\x1Ft[-\xF9(_Ao\xE9\x8C\xF2U\x9C\xD2\x14\x84\xB3\xD8t3\x04\x91\xA3_[\x83Q\x81\x10\x15a\x1CGW_Q` a]\x9A_9_Q\x90_R`\x01`\x01`\xA0\x1B\x03\x16\x84\x82\x81Q\x81\x10a/ Wa/ aX\x9AV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x03a0\x91W`@Qc\xA3\xD7^\t`\xE0\x1B\x81R_\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA3\xD7^\t\x90a/\x84\x90\x8B\x90`\x04\x01aP\x85V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/\x9FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a/\xC3\x91\x90aYtV[\x90P_`\xA2_\x8A`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _\x87\x85\x81Q\x81\x10a/\xFCWa/\xFCaX\x9AV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80` \x01`@R\x90\x81_\x82\x01T\x81RPP\x90Pa0p\x85\x84\x81Q\x81\x10a0NWa0NaX\x9AV[` \x02` \x01\x01Q\x83`\x01`\x01`@\x1B\x03\x16\x83a8\"\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x85\x84\x81Q\x81\x10a0\x82Wa0\x82aX\x9AV[` \x02` \x01\x01\x81\x81RPPPP[a0\xEA\x86\x88\x86\x84\x81Q\x81\x10a0\xA8Wa0\xA8aX\x9AV[` \x02` \x01\x01Q_\x87\x86\x81Q\x81\x10a0\xC3Wa0\xC3aX\x9AV[` \x02` \x01\x01Q\x87\x87\x81Q\x81\x10a0\xDDWa0\xDDaX\x9AV[` \x02` \x01\x01Qa1\xD5V[`\x01\x01a.\xEEV[_s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xBF\x19`\x01`\x01`\xA0\x1B\x03\x84\x16\x01a1\xC5W`@Qc\xA3\xD7^\t`\xE0\x1B\x81R_\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA3\xD7^\t\x90a1f\x90\x88\x90`\x04\x01aP\x85V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a1\x81W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a1\xA5\x91\x90aYtV[\x90Pa1\xBD`\x01`\x01`@\x1B\x03\x84\x81\x16\x90\x83\x16a8\x0EV[\x91PPa\rlV[P`\x01`\x01`@\x1B\x03\x16\x92\x91PPV[\x80_\x03a1\xF5W`@Qc\n3\xBCi`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x15a3\x12W`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\xA2` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R a2+\x81\x85\x85\x85aA\xAFV[`@\x80Q` \x81\x01\x90\x91R\x81T\x81R\x7F\x8B\xE92\xBA\xC5Ea\xF2r`\xF9Tc\xD9\xB8\xAB7\xE0k(B\xE5\xEE$\x04\x15|\xC1=\xF6\xEB\x8F\x90\x87\x90\x87\x90a2i\x90a=-V[`@Qa2x\x93\x92\x91\x90a[\xEAV[`@Q\x80\x91\x03\x90\xA1a2\x89\x86a\x10;V[\x15a\x1CGW`\x01`\x01`\xA0\x1B\x03\x80\x88\x16_\x90\x81R`\x98` \x90\x81R`@\x80\x83 \x93\x89\x16\x83R\x92\x90R\x90\x81 \x80T\x85\x92\x90a2\xC4\x90\x84\x90aY\xA3V[\x92PP\x81\x90UP\x86`\x01`\x01`\xA0\x1B\x03\x16\x7F\x1E\xC0B\xC9e\xE2\xED\xD7\x10{Q\x18\x8E\xE0\xF3\x83\xE2.v\x17\x90A\xAB:\x9D\x18\xFF\x15\x14\x05\x16l\x87\x87\x86`@Qa3\x08\x93\x92\x91\x90a[\xEAV[`@Q\x80\x91\x03\x90\xA2P[PPPPPPV[`@Qc\x1B\xEB+\x97`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R3`$\x83\x01R0`D\x83\x01R_\x805`\x01`\x01`\xE0\x1B\x03\x19\x16`d\x84\x01R\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xDFY\\\xB8\x90`\x84\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a3\xA0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19O\x91\x90aY\x11V[``_a3\xD0\x83aB*V[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P_\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[a4\taK7V[_\x82\x81R`\xA4` \x90\x81R`@\x91\x82\x90 \x82Q`\xE0\x81\x01\x84R\x81T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R`\x01\x83\x01T\x81\x16\x82\x85\x01R`\x02\x83\x01T\x16\x81\x85\x01R`\x03\x82\x01T``\x82\x81\x01\x91\x90\x91R`\x04\x83\x01Tc\xFF\xFF\xFF\xFF\x16`\x80\x83\x01R`\x05\x83\x01\x80T\x86Q\x81\x87\x02\x81\x01\x87\x01\x90\x97R\x80\x87R\x91\x95\x92\x94`\xA0\x86\x01\x93\x92\x90\x83\x01\x82\x82\x80\x15a4\xBBW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a4\x9DW[PPPPP\x81R` \x01`\x06\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a5\x11W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a4\xFDW[PPPPP\x81RPP\x91P\x81`\xA0\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a59Wa59aM\x99V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a5bW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83`\x80\x01Qa5\x96\x91\x90a\\PV[\x90P_Cc\xFF\xFF\xFF\xFF\x16\x82c\xFF\xFF\xFF\xFF\x16\x10a5\xC7Wa5\xC2\x84_\x01Q\x85` \x01Q\x86`\xA0\x01Qa&\xD3V[a5\xDEV[a5\xDE\x84_\x01Q\x85` \x01Q\x86`\xA0\x01Q\x85aBQV[\x90P_[\x84`\xA0\x01QQ\x81\x10\x15a\x13\x1AWa6/\x85`\xC0\x01Q\x82\x81Q\x81\x10a6\x08Wa6\x08aX\x9AV[` \x02` \x01\x01Q\x83\x83\x81Q\x81\x10a6\"Wa6\"aX\x9AV[` \x02` \x01\x01QaC\x7FV[\x84\x82\x81Q\x81\x10a6AWa6AaX\x9AV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a5\xE2V[_a6b\x84\x83\x85`\x01aC\x8AV[a\x19\xF2\x90\x85a\\lV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16_\x90\x81R`\xA5` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R\x90\x81 \x81\x90a6\x9D\x90aC\xE5V[\x90P_a7\x03`\x01a6\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Ca\\\x7FV[a6\xD9\x91\x90a\\\x7FV[`\x01`\x01`\xA0\x1B\x03\x80\x8A\x16_\x90\x81R`\xA5` \x90\x81R`@\x80\x83 \x93\x8C\x16\x83R\x92\x90R \x90aC\xFFV[\x90P_a7\x10\x82\x84a\\lV[\x90Pa7\x1D\x81\x87\x87aD\x1BV[\x98\x97PPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16_\x90\x81R`\x98` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a7_\x90\x84\x90a\\lV[\x92PP\x81\x90UP\x83`\x01`\x01`\xA0\x1B\x03\x16\x7Fi\t`\x007\xB7]{G3\xAE\xDD\x81TB\xB5\xEC\x01\x8A\x82wQ\xC82\xAA\xFFd\xEB\xA5\xD6\xD2\xDD\x84\x84\x84`@Qa\x1A\x93\x93\x92\x91\x90a[\xEAV[_`\x01`\x01`\xA0\x1B\x03\x82\x16_Q` a]\x9A_9_Q\x90_R\x14a7\xE7W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x19OV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x91PPV[_a\rl\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0aD9V[_a\x19\xF2\x82a8:a83\x87a=-V[\x86\x90a8\x0EV[\x90a8\x0EV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x11\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a \xCEV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPV[`\xA0\x84\x01QQ\x82\x14a9\x10W`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83`@\x01Q`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a9FW`@Qc\x16\x11\r5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a9P\x85a\x11\xACV[_\x81\x81R`\x9E` R`@\x90 T\x90\x91P`\xFF\x16a9\x81W`@Qc\x87\xC9\xD2\x19`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87`\x80\x01Qa9\xB4\x91\x90a\\PV[\x90P\x80c\xFF\xFF\xFF\xFF\x16Cc\xFF\xFF\xFF\xFF\x16\x11a9\xE2W`@Qcx\xF6z\xE1`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a9\xF9\x87_\x01Q\x88` \x01Q\x89`\xA0\x01Q\x84aBQV[\x87Q`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\xA3` R`@\x90 \x90\x92Pa:\x1F\x91P\x83aE\x1EV[P_\x82\x81R`\xA4` R`@\x81 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x82U`\x01\x82\x01\x80T\x82\x16\x90U`\x02\x82\x01\x80T\x90\x91\x16\x90U`\x03\x81\x01\x82\x90U`\x04\x81\x01\x80Tc\xFF\xFF\xFF\xFF\x19\x16\x90U\x90a:v`\x05\x83\x01\x82aL,V[a:\x83`\x06\x83\x01_aL,V[PP_\x82\x81R`\x9E` R`@\x90\x81\x90 \x80T`\xFF\x19\x16\x90UQ\x7F\x1F@@\x08\x89'N\xD0{$\x84^PT\xA8z\x0C\xAB\x96\x9E\xB1'z\xAF\xE6\x1A\xE3R\xE7\xC3*\0\x90a:\xCC\x90\x84\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1\x85Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`\x9A` R`@\x81 T\x88Q`\xA0\x8A\x01Q\x91\x90\x93\x16\x92a;\x06\x91\x84\x90a&\xD3V[\x90P_[\x88`\xA0\x01QQ\x81\x10\x15a=\"W_a;1\x8A`\xA0\x01Q\x83\x81Q\x81\x10a\x1E\x98Wa\x1E\x98aX\x9AV[\x90P_a;g\x8B`\xC0\x01Q\x84\x81Q\x81\x10a;MWa;MaX\x9AV[` \x02` \x01\x01Q\x87\x85\x81Q\x81\x10a6\"Wa6\"aX\x9AV[\x90P\x80_\x03a;wWPPa=\x1AV[\x87\x15a<EW\x81`\x01`\x01`\xA0\x1B\x03\x16c.\xAEA\x8C\x8C_\x01Q\x8D`\xA0\x01Q\x86\x81Q\x81\x10a;\xA6Wa;\xA6aX\x9AV[` \x02` \x01\x01Q\x8D\x8D\x88\x81\x81\x10a;\xC0Wa;\xC0aX\x9AV[\x90P` \x02\x01` \x81\x01\x90a;\xD5\x91\x90aP\xDCV[`@Q`\xE0\x85\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x91\x83\x16`$\x83\x01R\x90\x91\x16`D\x82\x01R`d\x81\x01\x84\x90R`\x84\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a<*W__\xFD[PZ\xF1\x15\x80\x15a<<W=__>=_\xFD[PPPPa=\x17V[__\x83`\x01`\x01`\xA0\x1B\x03\x16cP\xFFr%\x8E_\x01Q\x8F`\xA0\x01Q\x88\x81Q\x81\x10a<pWa<paX\x9AV[` \x02` \x01\x01Q\x86`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a<\x97\x93\x92\x91\x90a[\xEAV[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a<\xB2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<\xD6\x91\x90a\\\x9BV[\x91P\x91Pa=\x14\x87\x8E_\x01Q\x8F`\xA0\x01Q\x88\x81Q\x81\x10a<\xF8Wa<\xF8aX\x9AV[` \x02` \x01\x01Q\x85\x85\x8B\x8B\x81Q\x81\x10a0\xDDWa0\xDDaX\x9AV[PP[PP[`\x01\x01a;\nV[PPPPPPPPPV[\x80Q_\x90\x15a==W\x81Qa\x19OV[g\r\xE0\xB6\xB3\xA7d\0\0\x92\x91PPV[`fT``\x90`\x01\x90`\x02\x90\x81\x16\x03a=xW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16_\x81\x81R`\x9A` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U\x90Q\x93\x16\x92\x83\x92\x91\x7F\xFE\xE3\tf\xA2V\xB7\x1E\x14\xBC\x0E\xBF\xC9C\x15\xE2\x8E\xF4\xA9zq1\xA9\xE2\xB7\xA3\x10\xA7:\xF4Fv\x91\xA3__a=\xD7\x86a\x160V[\x91P\x91P\x81Q_\x03a=\xEBWPPPa?\xA5V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a>\x04Wa>\x04aM\x99V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a>-W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x94P_a><\x87\x85\x85a&\xD3V[\x90P_[\x83Q\x81\x10\x15a?\x9FW`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91` \x80\x83\x01\x90\x806\x837PP`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x92\x93P_\x92\x91P` \x80\x83\x01\x90\x806\x837PP`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x92\x93P_\x92\x91P` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x86\x84\x81Q\x81\x10a>\xC0Wa>\xC0aX\x9AV[` \x02` \x01\x01Q\x83_\x81Q\x81\x10a>\xDAWa>\xDAaX\x9AV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x85\x84\x81Q\x81\x10a?\x0CWa?\x0CaX\x9AV[` \x02` \x01\x01Q\x82_\x81Q\x81\x10a?&Wa?&aX\x9AV[` \x02` \x01\x01\x81\x81RPP\x84\x84\x81Q\x81\x10a?DWa?DaX\x9AV[` \x02` \x01\x01Q\x81_\x81Q\x81\x10a?^Wa?^aX\x9AV[` \x02` \x01\x01\x81\x81RPPa?w\x8B\x89\x85\x85\x85a(%V[\x8A\x85\x81Q\x81\x10a?\x89Wa?\x89aX\x9AV[` \x90\x81\x02\x91\x90\x91\x01\x01RPPP`\x01\x01a>@V[PPPPP[P\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16_\x90\x81R`\x99` R`@\x90 `\x01\x01T\x16\x80a?\xD3WPa\x0E\xC4V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x9C` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x90 T`\xFF\x16\x15a@\x17W`@Qc\rLL\x91`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x9C` \x90\x81R`@\x80\x83 \x85\x84R\x82R\x90\x91 \x80T`\xFF\x19\x16`\x01\x17\x90U\x83\x01Qa\"\xC9\x90\x82\x90a@^\x90\x88\x90\x88\x90\x84\x90\x88\x90a\x08 V[\x85Q` \x87\x01QaE)V[``_a@\x96\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a3\xC4V[\x90P\x80_\x81Q\x81\x10a@\xAAWa@\xAAaX\x9AV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x81`\x01\x81Q\x81\x10a@\xC8Wa@\xC8aX\x9AV[\x01` \x90\x81\x01Q`@Q`\x01`\x01`\xF8\x1B\x03\x19\x93\x84\x16\x92\x81\x01\x92\x90\x92R\x91\x90\x91\x16`!\x82\x01R`\"\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x90V[``_a\rl\x83aE{V[_a\rlaA\x1F\x84a=-V[\x83\x90a8\x0EV[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16_\x90\x81R`\xA5` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 aAU\x90aC\xE5V[\x90Pa\x0E\xC4CaAe\x84\x84aY\xA3V[`\x01`\x01`\xA0\x1B\x03\x80\x88\x16_\x90\x81R`\xA5` \x90\x81R`@\x80\x83 \x93\x8A\x16\x83R\x92\x90R \x91\x90aE\xD4V[_a\rl\x83\x83aE\xDFV[_a\rl\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84aD9V[\x82_\x03aA\xDBW`@\x80Q` \x81\x01\x90\x91R\x84T\x81RaA\xD4\x90\x82\x90a-\xE9\x90a=-V[\x84Ua\x0E\xC4V[`@\x80Q` \x81\x01\x90\x91R\x84T\x81R_\x90aA\xF7\x90\x85\x84a8\"V[\x90P_aB\x04\x84\x83aY\xA3V[\x90P_aB\x1F\x84a-\xE9aB\x18\x88\x8AaY\xA3V[\x85\x90aA\x9BV[\x87UPPPPPPPV[_`\xFF\x82\x16`\x1F\x81\x11\x15a\x19OW`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``_\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15aBmWaBmaM\x99V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15aB\x96W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x94\xD7\xD0\x0C\x87\x87\x87`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aB\xEA\x93\x92\x91\x90a\\\xBDV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aC\x04W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@RaC+\x91\x90\x81\x01\x90a[[V[\x90P_[\x85Q\x81\x10\x15aCsWaCN\x88\x87\x83\x81Q\x81\x10a'\xCEWa'\xCEaX\x9AV[\x83\x82\x81Q\x81\x10aC`WaC`aX\x9AV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01aC/V[P\x90\x96\x95PPPPPPV[_a\rl\x83\x83a8\x0EV[__aC\x97\x86\x86\x86aD9V[\x90P`\x01\x83`\x02\x81\x11\x15aC\xADWaC\xADa\\\xF6V[\x14\x80\x15aC\xC9WP_\x84\x80aC\xC4WaC\xC4a]\nV[\x86\x88\t\x11[\x15aC\xDCWaC\xD9`\x01\x82aY\xA3V[\x90P[\x95\x94PPPPPV[_aC\xF0\x82\x82aF+V[`\x01`\x01`\xE0\x1B\x03\x16\x92\x91PPV[_aD\x0B\x83\x83\x83aFoV[`\x01`\x01`\xE0\x1B\x03\x16\x93\x92PPPV[_a\x19\xF2aD)\x83\x85a]\x1EV[\x85\x90`\x01`\x01`@\x1B\x03\x16a8\x0EV[_\x80\x80_\x19\x85\x87\t\x85\x87\x02\x92P\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03aDpW\x83\x82\x81aDfWaDfa]\nV[\x04\x92PPPa\rlV[\x80\x84\x11aD\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtMath: mulDiv overflow`X\x1B`D\x82\x01R`d\x01a \xCEV[_\x84\x86\x88\t`\x02`\x01\x87\x19\x81\x01\x88\x16\x97\x88\x90\x04`\x03\x81\x02\x83\x18\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x90\x81\x02\x90\x92\x03\x90\x91\x02_\x88\x90\x03\x88\x90\x04\x90\x91\x01\x85\x83\x11\x90\x94\x03\x93\x90\x93\x02\x93\x03\x94\x90\x94\x04\x91\x90\x91\x17\x02\x94\x93PPPPV[_a\rl\x83\x83aF\xB7V[B\x81\x10\x15aEJW`@Qc\x08\x19\xBD\xCD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aE^`\x01`\x01`\xA0\x1B\x03\x85\x16\x84\x84aG\x9AV[a\x0E\xC4W`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x81_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15aE\xC8W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11aE\xB4W[PPPPP\x90P\x91\x90PV[a\x10\xC3\x83\x83\x83aG\xEEV[_\x81\x81R`\x01\x83\x01` R`@\x81 TaF$WP\x81T`\x01\x81\x81\x01\x84U_\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x19OV[P_a\x19OV[\x81T_\x90\x80\x15aFgWaFQ\x84aFD`\x01\x84a\\lV[_\x91\x82R` \x90\x91 \x01\x90V[T`\x01` \x1B\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x19\xF2V[P\x90\x92\x91PPV[\x82T_\x90\x81aF\x80\x86\x86\x83\x85aH\xF1V[\x90P\x80\x15aF\xADWaF\x97\x86aFD`\x01\x84a\\lV[T`\x01` \x1B\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x08\x9EV[P\x91\x94\x93PPPPV[_\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15aG\x91W_aF\xD9`\x01\x83a\\lV[\x85T\x90\x91P_\x90aF\xEC\x90`\x01\x90a\\lV[\x90P\x81\x81\x14aGKW_\x86_\x01\x82\x81T\x81\x10aG\nWaG\naX\x9AV[\x90_R` _ \x01T\x90P\x80\x87_\x01\x84\x81T\x81\x10aG*WaG*aX\x9AV[_\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80aG\\WaG\\a]=V[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90U\x85`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x90U`\x01\x93PPPPa\x19OV[_\x91PPa\x19OV[___aG\xA7\x85\x85aIDV[\x90\x92P\x90P_\x81`\x04\x81\x11\x15aG\xBFWaG\xBFa\\\xF6V[\x14\x80\x15aG\xDDWP\x85`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14[\x80a\x08\x9EWPa\x08\x9E\x86\x86\x86aI\x83V[\x82T\x80\x15aH\xA4W_aH\x06\x85aFD`\x01\x85a\\lV[`@\x80Q\x80\x82\x01\x90\x91R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\x01` \x1B\x90\x92\x04`\x01`\x01`\xE0\x1B\x03\x16` \x84\x01R\x91\x92P\x90\x85\x16\x10\x15aHXW`@Qc\x15\x1B\x8E?`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Qc\xFF\xFF\xFF\xFF\x80\x86\x16\x91\x16\x03aH\xA2W\x82aHy\x86aFD`\x01\x86a\\lV[\x80T`\x01`\x01`\xE0\x1B\x03\x92\x90\x92\x16`\x01` \x1B\x02c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UPPPPPV[P[P`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x92\x83\x16\x81R`\x01`\x01`\xE0\x1B\x03\x91\x82\x16` \x80\x83\x01\x91\x82R\x85T`\x01\x81\x01\x87U_\x96\x87R\x95 \x91Q\x90Q\x90\x92\x16`\x01` \x1B\x02\x91\x90\x92\x16\x17\x91\x01UV[_[\x81\x83\x10\x15a\x1BoW_aI\x06\x84\x84aJjV[_\x87\x81R` \x90 \x90\x91Pc\xFF\xFF\xFF\xFF\x86\x16\x90\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15aI0W\x80\x92PaI>V[aI;\x81`\x01aY\xA3V[\x93P[PaH\xF3V[__\x82Q`A\x03aIxW` \x83\x01Q`@\x84\x01Q``\x85\x01Q_\x1AaIl\x87\x82\x85\x85aJ\x84V[\x94P\x94PPPPa 1V[P_\x90P`\x02a 1V[___\x85`\x01`\x01`\xA0\x1B\x03\x16c\x16&\xBA~`\xE0\x1B\x86\x86`@Q`$\x01aI\xAB\x92\x91\x90a]QV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90QaI\xE9\x91\x90a]iV[_`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80_\x81\x14aJ!W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>aJ&V[``\x91P[P\x91P\x91P\x81\x80\x15aJ:WP` \x81Q\x10\x15[\x80\x15a\x08\x9EWP\x80Qc\x0B\x13]?`\xE1\x1B\x90aJ_\x90\x83\x01` \x90\x81\x01\x90\x84\x01aZ\xCCV[\x14\x96\x95PPPPPPV[_aJx`\x02\x84\x84\x18a]zV[a\rl\x90\x84\x84\x16aY\xA3V[_\x80o\xA2\xA8\x91\x8C\xA8[\xAF\xE2 \x16\xD0\xB9\x97\xE4\xDF``\x01`\xFF\x1B\x03\x83\x11\x15aJ\xAFWP_\x90P`\x03aK.V[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aK\0W=__>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16aK(W_`\x01\x92P\x92PPaK.V[\x91P_\x90P[\x94P\x94\x92PPPV[`@Q\x80`\xE0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01``\x81R` \x01``\x81RP\x90V[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15aK\xE3W\x91` \x02\x82\x01[\x82\x81\x11\x15aK\xE3W\x82Q\x82T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x82U` \x90\x92\x01\x91`\x01\x90\x91\x01\x90aK\xAEV[PaK\xEF\x92\x91PaLCV[P\x90V[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15aK\xE3W\x91` \x02\x82\x01[\x82\x81\x11\x15aK\xE3W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90aL\x11V[P\x80T_\x82U\x90_R` _ \x90\x81\x01\x90a$V\x91\x90[[\x80\x82\x11\x15aK\xEFW_\x81U`\x01\x01aLDV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a$VW__\xFD[\x805a\"u\x81aLWV[_____`\xA0\x86\x88\x03\x12\x15aL\x8AW__\xFD[\x855aL\x95\x81aLWV[\x94P` \x86\x015aL\xA5\x81aLWV[\x93P`@\x86\x015aL\xB5\x81aLWV[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[__\x83`\x1F\x84\x01\x12aL\xDDW__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aL\xF3W__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a 1W__\xFD[__` \x83\x85\x03\x12\x15aM\x1EW__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aM3W__\xFD[aM?\x85\x82\x86\x01aL\xCDV[\x90\x96\x90\x95P\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a(\x1AW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aMdV[_` \x82\x84\x03\x12\x15aM\x92W__\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aM\xCFWaM\xCFaM\x99V[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aM\xCFWaM\xCFaM\x99V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aN\x1FWaN\x1FaM\x99V[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15aN?WaN?aM\x99V[P`\x05\x1B` \x01\x90V[_\x82`\x1F\x83\x01\x12aNXW__\xFD[\x815aNkaNf\x82aN'V[aM\xF7V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aN\x8CW__\xFD[` \x85\x01[\x83\x81\x10\x15aN\xB2W\x805aN\xA4\x81aLWV[\x83R` \x92\x83\x01\x92\x01aN\x91V[P\x95\x94PPPPPV[_\x82`\x1F\x83\x01\x12aN\xCBW__\xFD[\x815aN\xD9aNf\x82aN'V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aN\xFAW__\xFD[` \x85\x01[\x83\x81\x10\x15aN\xB2W\x805\x83R` \x92\x83\x01\x92\x01aN\xFFV[___``\x84\x86\x03\x12\x15aO)W__\xFD[\x835aO4\x81aLWV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aONW__\xFD[aOZ\x86\x82\x87\x01aNIV[\x92PP`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aOuW__\xFD[aO\x81\x86\x82\x87\x01aN\xBCV[\x91PP\x92P\x92P\x92V[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aO\xBBW\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aO\x9DV[P\x93\x94\x93PPPPV[` \x81R_a\rl` \x83\x01\x84aO\x8BV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\"uW__\xFD[__\x83`\x1F\x84\x01\x12aO\xFAW__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aP\x10W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a 1W__\xFD[____``\x85\x87\x03\x12\x15aP:W__\xFD[\x845aPE\x81aLWV[\x93PaPS` \x86\x01aO\xD7V[\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aPmW__\xFD[aPy\x87\x82\x88\x01aO\xEAV[\x95\x98\x94\x97P\x95PPPPV[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[____`\x80\x85\x87\x03\x12\x15aP\xACW__\xFD[\x845aP\xB7\x81aLWV[\x93P` \x85\x015aP\xC7\x81aLWV[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[_` \x82\x84\x03\x12\x15aP\xECW__\xFD[\x815a\rl\x81aLWV[__`@\x83\x85\x03\x12\x15aQ\x08W__\xFD[\x825aQ\x13\x81aLWV[\x91P` \x83\x015aQ#\x81aLWV[\x80\x91PP\x92P\x92\x90PV[_[\x83\x81\x10\x15aQHW\x81\x81\x01Q\x83\x82\x01R` \x01aQ0V[PP_\x91\x01RV[_\x81Q\x80\x84RaQg\x81` \x86\x01` \x86\x01aQ.V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R_a\rl` \x83\x01\x84aQPV[_`\xE0\x82\x84\x03\x12\x15aQ\x9DW__\xFD[aQ\xA5aM\xADV[\x90PaQ\xB0\x82aLkV[\x81RaQ\xBE` \x83\x01aLkV[` \x82\x01RaQ\xCF`@\x83\x01aLkV[`@\x82\x01R``\x82\x81\x015\x90\x82\x01RaQ\xEA`\x80\x83\x01aO\xD7V[`\x80\x82\x01R`\xA0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aR\x07W__\xFD[aR\x13\x84\x82\x85\x01aNIV[`\xA0\x83\x01RP`\xC0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aR1W__\xFD[aR=\x84\x82\x85\x01aN\xBCV[`\xC0\x83\x01RP\x92\x91PPV[_` \x82\x84\x03\x12\x15aRYW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aRnW__\xFD[a\x19\xF2\x84\x82\x85\x01aQ\x8DV[_` \x82\x84\x03\x12\x15aR\x8AW__\xFD[\x815`\xFF\x81\x16\x81\x14a\rlW__\xFD[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aO\xBBW\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aR\xACV[\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x80\x83\x01Q\x82\x16\x90\x84\x01R`@\x80\x83\x01Q\x90\x91\x16\x90\x83\x01R``\x80\x82\x01Q\x90\x83\x01R`\x80\x80\x82\x01Q_\x91aS\x1E\x90\x85\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P`\xA0\x82\x01Q`\xE0`\xA0\x85\x01RaS8`\xE0\x85\x01\x82aR\x9AV[\x90P`\xC0\x83\x01Q\x84\x82\x03`\xC0\x86\x01RaC\xDC\x82\x82aO\x8BV[`@\x81R_aSc`@\x83\x01\x85aR\xD3V[\x82\x81\x03` \x84\x01RaC\xDC\x81\x85aO\x8BV[_\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01_[\x83\x81\x10\x15aCsW`\x1F\x19\x85\x84\x03\x01\x88RaS\xAD\x83\x83QaO\x8BV[` \x98\x89\x01\x98\x90\x93P\x91\x90\x91\x01\x90`\x01\x01aS\x91V[_`@\x82\x01`@\x83R\x80\x85Q\x80\x83R``\x85\x01\x91P``\x81`\x05\x1B\x86\x01\x01\x92P` \x87\x01_[\x82\x81\x10\x15aT\x1AW`_\x19\x87\x86\x03\x01\x84RaT\x05\x85\x83QaR\xD3V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aS\xE9V[PPPP\x82\x81\x03` \x84\x01RaC\xDC\x81\x85aSuV[`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a$VW__\xFD[____`\x80\x85\x87\x03\x12\x15aTWW__\xFD[\x845aTb\x81aLWV[\x93P` \x85\x015aTr\x81aLWV[\x92P`@\x85\x015aT\x82\x81aT0V[\x91P``\x85\x015aT\x92\x81aT0V[\x93\x96\x92\x95P\x90\x93PPV[___``\x84\x86\x03\x12\x15aT\xAFW__\xFD[\x835aT\xBA\x81aLWV[\x92P` \x84\x015\x91P`@\x84\x015aT\xD1\x81aT0V[\x80\x91PP\x92P\x92P\x92V[`@\x81R_aSc`@\x83\x01\x85aR\x9AV[___`@\x84\x86\x03\x12\x15aU\0W__\xFD[\x835aU\x0B\x81aLWV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aU%W__\xFD[aU1\x86\x82\x87\x01aO\xEAV[\x94\x97\x90\x96P\x93\x94PPPPV[__`@\x83\x85\x03\x12\x15aUOW__\xFD[\x825aUZ\x81aLWV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aUtW__\xFD[aU\x80\x85\x82\x86\x01aNIV[\x91PP\x92P\x92\x90PV[______``\x87\x89\x03\x12\x15aU\x9FW__\xFD[\x865`\x01`\x01`@\x1B\x03\x81\x11\x15aU\xB4W__\xFD[aU\xC0\x89\x82\x8A\x01aL\xCDV[\x90\x97P\x95PP` \x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aU\xDEW__\xFD[aU\xEA\x89\x82\x8A\x01aL\xCDV[\x90\x95P\x93PP`@\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aV\x08W__\xFD[aV\x14\x89\x82\x8A\x01aL\xCDV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[` \x81R_a\rl` \x83\x01\x84aR\xD3V[___``\x84\x86\x03\x12\x15aVJW__\xFD[\x835aVU\x81aLWV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aVoW__\xFD[\x84\x01`@\x81\x87\x03\x12\x15aV\x80W__\xFD[aV\x88aM\xD5V[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aV\x9DW__\xFD[\x82\x01`\x1F\x81\x01\x88\x13aV\xADW__\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aV\xC6WaV\xC6aM\x99V[aV\xD9`\x1F\x82\x01`\x1F\x19\x16` \x01aM\xF7V[\x81\x81R\x89` \x83\x85\x01\x01\x11\x15aV\xEDW__\xFD[\x81` \x84\x01` \x83\x017_` \x92\x82\x01\x83\x01R\x83R\x92\x83\x015\x92\x82\x01\x92\x90\x92R\x93\x96\x93\x95PPPP`@\x91\x90\x91\x015\x90V[__`@\x83\x85\x03\x12\x15aW0W__\xFD[\x825aW;\x81aLWV[\x94` \x93\x90\x93\x015\x93PPPV[`@\x81R_aSc`@\x83\x01\x85aO\x8BV[\x80\x15\x15\x81\x14a$VW__\xFD[____``\x85\x87\x03\x12\x15aW{W__\xFD[\x845`\x01`\x01`@\x1B\x03\x81\x11\x15aW\x90W__\xFD[\x85\x01`\xE0\x81\x88\x03\x12\x15aW\xA1W__\xFD[\x93P` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aW\xBBW__\xFD[aW\xC7\x87\x82\x88\x01aL\xCDV[\x90\x94P\x92PP`@\x85\x015aT\x92\x81aW[V[__`@\x83\x85\x03\x12\x15aW\xECW__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aX\x01W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13aX\x11W__\xFD[\x805aX\x1FaNf\x82aN'V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15aX@W__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aXkW\x835aXZ\x81aLWV[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aXGV[\x94PPPP` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aUtW__\xFD[` \x81R_a\rl` \x83\x01\x84aSuV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x825`^\x19\x836\x03\x01\x81\x12aX\xC2W__\xFD[\x91\x90\x91\x01\x92\x91PPV[__\x835`\x1E\x19\x846\x03\x01\x81\x12aX\xE1W__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aX\xFAW__\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a 1W__\xFD[_` \x82\x84\x03\x12\x15aY!W__\xFD[\x81Qa\rl\x81aW[V[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017_\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x16` \x82\x01R`@\x01\x90V[_` \x82\x84\x03\x12\x15aY\x84W__\xFD[\x81Qa\rl\x81aT0V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x19OWa\x19OaY\x8FV[_\x82`\x1F\x83\x01\x12aY\xC5W__\xFD[\x81QaY\xD3aNf\x82aN'V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aY\xF4W__\xFD[` \x85\x01[\x83\x81\x10\x15aN\xB2W\x80Q\x83R` \x92\x83\x01\x92\x01aY\xF9V[__`@\x83\x85\x03\x12\x15aZ\"W__\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aZ7W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13aZGW__\xFD[\x80QaZUaNf\x82aN'V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15aZvW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aZ\xA1W\x83QaZ\x90\x81aLWV[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aZ}V[\x80\x95PPPPP` \x83\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aZ\xC0W__\xFD[aU\x80\x85\x82\x86\x01aY\xB6V[_` \x82\x84\x03\x12\x15aZ\xDCW__\xFD[PQ\x91\x90PV[_\x825`\xDE\x19\x836\x03\x01\x81\x12aX\xC2W__\xFD[_a\x19O6\x83aQ\x8DV[_` \x82\x84\x03\x12\x15a[\x12W__\xFD[\x815a\rl\x81aW[V[_` \x82\x84\x03\x12\x15a[-W__\xFD[\x81Qa\rl\x81aLWV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90a\x19\xF2\x90\x83\x01\x84aR\x9AV[_` \x82\x84\x03\x12\x15a[kW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a[\x80W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a[\x90W__\xFD[\x80Qa[\x9EaNf\x82aN'V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15a[\xBFW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a\x08\x9EW\x83Qa[\xD9\x81aT0V[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a[\xC6V[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[_`\x01\x82\x01a\\\x1FWa\\\x1FaY\x8FV[P`\x01\x01\x90V[\x83\x81R``` \x82\x01R_a\\>``\x83\x01\x85aR\xD3V[\x82\x81\x03`@\x84\x01Ra\x08\x9E\x81\x85aO\x8BV[c\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x19OWa\x19OaY\x8FV[\x81\x81\x03\x81\x81\x11\x15a\x19OWa\x19OaY\x8FV[c\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x19OWa\x19OaY\x8FV[__`@\x83\x85\x03\x12\x15a\\\xACW__\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R``` \x82\x01\x81\x90R_\x90a\\\xE0\x90\x83\x01\x85aR\x9AV[\x90Pc\xFF\xFF\xFF\xFF\x83\x16`@\x83\x01R\x94\x93PPPPV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[`\x01`\x01`@\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x19OWa\x19OaY\x8FV[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[\x82\x81R`@` \x82\x01R_a\x19\xF2`@\x83\x01\x84aQPV[_\x82QaX\xC2\x81\x84` \x87\x01aQ.V[_\x82a]\x94WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V\xFE\0\0\0\0\0\0\0\0\0\0\0\0\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\xA2dipfsX\"\x12 \xE1\xC0a;g\xB7\xD2\x91\xDB\xD5\x1E\x9B\x97\xADY&\x81\xC3\x91\xBC^\xA8\x83\xC6k\xC8r\x8E\xBB\xDE\xED\xD4dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b50600436106102c3575f3560e01c8063715018a611610177578063bfae3fd2116100d5578063e4cc3f901161008f578063e4cc3f9014610799578063eea9064b146107ac578063f0e0e676146107bf578063f2fde38b146107df578063f698da25146107f2578063fabc1cbc146107fa578063fd8aa88d1461080d575f5ffd5b8063bfae3fd2146106e4578063c448feb8146106f7578063c978f7ac1461072b578063ca8aa7c71461074c578063cd6dc68714610773578063da8be86414610786575f5ffd5b80639104c319116101315780639104c3191461061c5780639435bb431461063057806399f5371b14610643578063a178848414610663578063a33a343314610682578063b7f06ebe14610695578063bb45fef2146106b7575f5ffd5b8063715018a61461058c578063778e55f31461059457806378296ec5146105be578063886f1195146105d15780638da5cb5b146105f85780639004134714610609575f5ffd5b806354fd4d50116102245780635dd68579116101de5780635dd68579146104d6578063601bb36f146104f757806360a0d1ce1461050a57806365da12641461051d57806366d5ba93146105455780636d70f7ae146105665780636e17444814610579575f5ffd5b806354fd4d501461045a578063595c6a671461046f578063597b36da146104775780635ac86ab71461048a5780635c975abb146104ad5780635d975e88146104b5575f5ffd5b806339b70e381161028057806339b70e381461037c5780633c651cf2146103b05780633cdeb5e0146103c35780633e28391d146103d65780634657e26a146103f95780634665bcda1461042057806354b7c96c14610447575f5ffd5b806304a4f979146102c75780630b9f487a146103015780630dd8dd0214610314578063136439dd1461033457806325df922e146103495780632aa6d88814610369575b5f5ffd5b6102ee7f14bde674c9f64b2ad00eaaee4a8bed1fabef35c7507e3c5b9cfc9436909a2dad81565b6040519081526020015b60405180910390f35b6102ee61030f366004614c76565b610820565b610327610322366004614d0d565b6108a8565b6040516102f89190614d4b565b610347610342366004614d82565b610b39565b005b61035c610357366004614f17565b610c13565b6040516102f89190614fc5565b610347610377366004615027565b610d73565b6103a37f000000000000000000000000000000000000000000000000000000000000000081565b6040516102f89190615085565b6103476103be366004615099565b610eca565b6103a36103d13660046150dc565b61101b565b6103e96103e43660046150dc565b61103b565b60405190151581526020016102f8565b6103a37f000000000000000000000000000000000000000000000000000000000000000081565b6103a37f000000000000000000000000000000000000000000000000000000000000000081565b6103476104553660046150f7565b61105a565b6104626110c8565b6040516102f8919061517b565b6103476110f8565b6102ee610485366004615249565b6111ac565b6103e961049836600461527a565b606654600160ff9092169190911b9081161490565b6066546102ee565b6104c86104c3366004614d82565b6111db565b6040516102f8929190615351565b6104e96104e43660046150dc565b6111f8565b6040516102f89291906153c3565b610347610505366004615444565b611322565b61034761051836600461549d565b61149a565b6103a361052b3660046150dc565b609a6020525f90815260409020546001600160a01b031681565b6105586105533660046150dc565b611630565b6040516102f89291906154dc565b6103e96105743660046150dc565b61191d565b6102ee6105873660046150f7565b611955565b6103476119fa565b6102ee6105a23660046150f7565b609860209081525f928352604080842090915290825290205481565b6103476105cc3660046154ee565b611a0b565b6103a37f000000000000000000000000000000000000000000000000000000000000000081565b6033546001600160a01b03166103a3565b61035c61061736600461553e565b611aa1565b6103a35f516020615d9a5f395f51905f5281565b61034761063e36600461558a565b611b77565b610656610651366004614d82565b611c50565b6040516102f89190615626565b6102ee6106713660046150dc565b609f6020525f908152604090205481565b610327610690366004615638565b611d6c565b6103e96106a3366004614d82565b609e6020525f908152604090205460ff1681565b6103e96106c536600461571f565b609c60209081525f928352604080842090915290825290205460ff1681565b6102ee6106f23660046150f7565b611d84565b60405163ffffffff7f00000000000000000000000000000000000000000000000000000000000000001681526020016102f8565b61073e61073936600461553e565b611dc0565b6040516102f8929190615749565b6103a37f000000000000000000000000000000000000000000000000000000000000000081565b61034761078136600461571f565b612038565b6103276107943660046150dc565b612153565b6103476107a7366004615768565b61227a565b6103476107ba366004615638565b6122d0565b6107d26107cd3660046157db565b61233b565b6040516102f89190615888565b6103476107ed3660046150dc565b6123e0565b6102ee612459565b610347610808366004614d82565b612512565b61032761081b3660046150dc565b612629565b604080517f14bde674c9f64b2ad00eaaee4a8bed1fabef35c7507e3c5b9cfc9436909a2dad60208201526001600160a01b03808616928201929092528187166060820152908516608082015260a0810183905260c081018290525f9061089e9060e0016040516020818303038152906040528051906020012061264c565b9695505050505050565b6066546060906001906002908116036108d45760405163840a48d560e01b815260040160405180910390fd5b6108dc61267a565b5f836001600160401b038111156108f5576108f5614d99565b60405190808252806020026020018201604052801561091e578160200160208202803683370190505b50335f908152609a60205260408120549192506001600160a01b03909116905b85811015610b2a578686828181106109585761095861589a565b905060200281019061096a91906158ae565b6109789060208101906158cc565b905087878381811061098c5761098c61589a565b905060200281019061099e91906158ae565b6109a890806158cc565b9050146109c8576040516343714afd60e01b815260040160405180910390fd5b5f610a3233848a8a868181106109e0576109e061589a565b90506020028101906109f291906158ae565b6109fc90806158cc565b808060200260200160405190810160405280939291908181526020018383602002808284375f920191909152506126d392505050565b9050610b0433848a8a86818110610a4b57610a4b61589a565b9050602002810190610a5d91906158ae565b610a6790806158cc565b808060200260200160405190810160405280939291908181526020018383602002808284375f920191909152508e92508d9150889050818110610aac57610aac61589a565b9050602002810190610abe91906158ae565b610acc9060208101906158cc565b808060200260200160405190810160405280939291908181526020018383602002808284375f92019190915250889250612825915050565b848381518110610b1657610b1661589a565b60209081029190910101525060010161093e565b5050600160c955949350505050565b60405163237dfb4760e11b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906346fbf68e90610b85903390600401615085565b602060405180830381865afa158015610ba0573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610bc49190615911565b610be157604051631d77d47760e21b815260040160405180910390fd5b6066548181168114610c065760405163c61dca5d60e01b815260040160405180910390fd5b610c0f82612d94565b5050565b6001600160a01b038084165f908152609a60205260408120546060921690610c3c8683876126d3565b90505f85516001600160401b03811115610c5857610c58614d99565b604051908082528060200260200182016040528015610c81578160200160208202803683370190505b5090505f5b8651811015610d66576001600160a01b0388165f90815260a260205260408120885182908a9085908110610cbc57610cbc61589a565b60200260200101516001600160a01b03166001600160a01b031681526020019081526020015f206040518060200160405290815f820154815250509050610d40878381518110610d0e57610d0e61589a565b6020026020010151858481518110610d2857610d2861589a565b602002602001015183612dd19092919063ffffffff16565b838381518110610d5257610d5261589a565b602090810291909101015250600101610c86565b50925050505b9392505050565b610d7b61267a565b610d843361103b565b15610da257604051633bf2b50360e11b815260040160405180910390fd5b604051632b6241f360e11b815233600482015263ffffffff841660248201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906356c483e6906044015f604051808303815f87803b158015610e0c575f5ffd5b505af1158015610e1e573d5f5f3e3d5ffd5b50505050610e2c3385612def565b610e363333612e51565b336001600160a01b03167fa453db612af59e5521d6ab9284dc3e2d06af286eb1b1b7b771fce4716c19f2c185604051610e6f9190615085565b60405180910390a2336001600160a01b03167f02a919ed0e2acad1dd90f17ef2fa4ae5462ee1339170034a8531cca4b67080908383604051610eb292919061592c565b60405180910390a2610ec4600160c955565b50505050565b336001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161480610f295750336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016145b610f465760405163045206a560e21b815260040160405180910390fd5b610f4e61267a565b6001600160a01b038085165f908152609a602052604080822054905163152667d960e31b8152908316927f0000000000000000000000000000000000000000000000000000000000000000169063a9333ec890610fb1908590899060040161595a565b602060405180830381865afa158015610fcc573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ff09190615974565b90505f610ffe8787846130f2565b905061100e8388888888866131d5565b505050610ec4600160c955565b6001600160a01b039081165f908152609960205260409020600101541690565b6001600160a01b039081165f908152609a602052604090205416151590565b816110648161331a565b6110815760405163932d94f760e01b815260040160405180910390fd5b61108961267a565b6110928361191d565b6110af576040516325ec6c1f60e01b815260040160405180910390fd5b6110b98383612def565b6110c3600160c955565b505050565b60606110f37f00000000000000000000000000000000000000000000000000000000000000006133c4565b905090565b60405163237dfb4760e11b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906346fbf68e90611144903390600401615085565b602060405180830381865afa15801561115f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906111839190615911565b6111a057604051631d77d47760e21b815260040160405180910390fd5b6111aa5f19612d94565b565b5f816040516020016111be9190615626565b604051602081830303815290604052805190602001209050919050565b6111e3614b37565b60606111ee83613401565b9094909350915050565b6060805f61120584612629565b8051909150806001600160401b0381111561122257611222614d99565b60405190808252806020026020018201604052801561125b57816020015b611248614b37565b8152602001906001900390816112405790505b509350806001600160401b0381111561127657611276614d99565b6040519080825280602002602001820160405280156112a957816020015b60608152602001906001900390816112945790505b5092505f5b8181101561131a576112d88382815181106112cb576112cb61589a565b6020026020010151613401565b8683815181106112ea576112ea61589a565b602002602001018684815181106113035761130361589a565b6020908102919091010191909152526001016112ae565b505050915091565b336001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161461136b576040516323d871a560e01b815260040160405180910390fd5b61137361267a565b6001600160a01b038085165f9081526098602090815260408083209387168352929052908120546113b1906001600160401b03808616908516613654565b90505f6113c08686868661366c565b90505f6113cd82846159a3565b90506113db875f8886613729565b604080516001600160a01b038881168252602082018490528916917fdd611f4ef63f4385f1756c86ce1f1f389a9013ba6fa07daba8528291bc2d3c30910160405180910390a25f61142b876137a3565b60405163debe1eab60e01b81526001600160a01b038981166004830152602482018590529192509082169063debe1eab906044015f604051808303815f87803b158015611476575f5ffd5b505af1158015611488573d5f5f3e3d5ffd5b5050505050505050610ec4600160c955565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146114e357604051633213a66160e21b815260040160405180910390fd5b6114eb61267a565b6114f48361103b565b156110b9576001600160a01b038084165f908152609a602052604080822054905163152667d960e31b8152908316927f0000000000000000000000000000000000000000000000000000000000000000169063a9333ec8906115699085905f516020615d9a5f395f51905f529060040161595a565b602060405180830381865afa158015611584573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906115a89190615974565b6001600160a01b0386165f90815260a2602090815260408083205f516020615d9a5f395f51905f52845282528083208151928301909152548152919250611607866115ff6001600160401b0380871690891661380e565b849190613822565b905061162284885f516020615d9a5f395f51905f5284613729565b505050506110c3600160c955565b6060805f5f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166394f649dd866040518263ffffffff1660e01b81526004016116819190615085565b5f60405180830381865afa15801561169b573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526116c29190810190615a11565b915091505f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663fe243a17875f516020615d9a5f395f51905f526040518363ffffffff1660e01b815260040161172292919061595a565b602060405180830381865afa15801561173d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906117619190615acc565b9050805f0361177557509094909350915050565b5f8351600161178491906159a3565b6001600160401b0381111561179b5761179b614d99565b6040519080825280602002602001820160405280156117c4578160200160208202803683370190505b5090505f845160016117d691906159a3565b6001600160401b038111156117ed576117ed614d99565b604051908082528060200260200182016040528015611816578160200160208202803683370190505b5090505f516020615d9a5f395f51905f528286518151811061183a5761183a61589a565b60200260200101906001600160a01b031690816001600160a01b031681525050828186518151811061186e5761186e61589a565b60209081029190910101525f5b855181101561190f578581815181106118965761189661589a565b60200260200101518382815181106118b0576118b061589a565b60200260200101906001600160a01b031690816001600160a01b0316815250508481815181106118e2576118e261589a565b60200260200101518282815181106118fc576118fc61589a565b602090810291909101015260010161187b565b509097909650945050505050565b5f6001600160a01b0382161580159061194f57506001600160a01b038083165f818152609a6020526040902054909116145b92915050565b5f5f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663a9333ec885856040518363ffffffff1660e01b81526004016119a592919061595a565b602060405180830381865afa1580156119c0573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119e49190615974565b90506119f28484835f61366c565b949350505050565b611a02613840565b6111aa5f61389a565b82611a158161331a565b611a325760405163932d94f760e01b815260040160405180910390fd5b611a3b8461191d565b611a58576040516325ec6c1f60e01b815260040160405180910390fd5b836001600160a01b03167f02a919ed0e2acad1dd90f17ef2fa4ae5462ee1339170034a8531cca4b67080908484604051611a9392919061592c565b60405180910390a250505050565b60605f82516001600160401b03811115611abd57611abd614d99565b604051908082528060200260200182016040528015611ae6578160200160208202803683370190505b5090505f5b8351811015611b6f576001600160a01b0385165f9081526098602052604081208551909190869084908110611b2257611b2261589a565b60200260200101516001600160a01b03166001600160a01b031681526020019081526020015f2054828281518110611b5c57611b5c61589a565b6020908102919091010152600101611aeb565b509392505050565b606654600290600490811603611ba05760405163840a48d560e01b815260040160405180910390fd5b611ba861267a565b855f5b81811015611c3b57611c33898983818110611bc857611bc861589a565b9050602002810190611bda9190615ae3565b611be390615af7565b888884818110611bf557611bf561589a565b9050602002810190611c0791906158cc565b888886818110611c1957611c1961589a565b9050602002016020810190611c2e9190615b02565b6138eb565b600101611bab565b5050611c47600160c955565b50505050505050565b611c58614b37565b5f82815260a46020908152604091829020825160e08101845281546001600160a01b03908116825260018301548116828501526002830154168185015260038201546060820152600482015463ffffffff1660808201526005820180548551818602810186019096528086529194929360a08601939290830182828015611d0657602002820191905f5260205f20905b81546001600160a01b03168152600190910190602001808311611ce8575b5050505050815260200160068201805480602002602001604051908101604052809291908181526020018280548015611d5c57602002820191905f5260205f20905b815481526020019060010190808311611d48575b5050505050815250509050919050565b6060611d7733612153565b9050610d6c8484846122d0565b6001600160a01b038083165f90815260a260209081526040808320938516835292815282822083519182019093529154825290610d6c90613d2d565b60608082516001600160401b03811115611ddc57611ddc614d99565b604051908082528060200260200182016040528015611e05578160200160208202803683370190505b50915082516001600160401b03811115611e2157611e21614d99565b604051908082528060200260200182016040528015611e4a578160200160208202803683370190505b506001600160a01b038086165f908152609a6020526040812054929350911690611e758683876126d3565b90505f5b855181101561202d575f611ea5878381518110611e9857611e9861589a565b60200260200101516137a3565b9050806001600160a01b031663fe243a1789898581518110611ec957611ec961589a565b60200260200101516040518363ffffffff1660e01b8152600401611eee92919061595a565b602060405180830381865afa158015611f09573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611f2d9190615acc565b858381518110611f3f57611f3f61589a565b6020026020010181815250505f60a25f8a6001600160a01b03166001600160a01b031681526020019081526020015f205f898581518110611f8257611f8261589a565b60200260200101516001600160a01b03166001600160a01b031681526020019081526020015f206040518060200160405290815f820154815250509050612006868481518110611fd457611fd461589a565b6020026020010151858581518110611fee57611fee61589a565b6020026020010151836138229092919063ffffffff16565b8784815181106120185761201861589a565b60209081029190910101525050600101611e79565b5050505b9250929050565b5f54610100900460ff161580801561205657505f54600160ff909116105b8061206f5750303b15801561206f57505f5460ff166001145b6120d75760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b5f805460ff1916600117905580156120f8575f805461ff0019166101001790555b61210182612d94565b61210a8361389a565b80156110c3575f805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a1505050565b606061215d61267a565b6121668261103b565b6121835760405163a5c7c44560e01b815260040160405180910390fd5b61218c8261191d565b156121aa576040516311ca333560e31b815260040160405180910390fd5b336001600160a01b03831614612260576001600160a01b038083165f908152609a6020526040902054166121dd8161331a565b8061220157506121ec8161101b565b6001600160a01b0316336001600160a01b0316145b61221e57604051631e499a2360e11b815260040160405180910390fd5b806001600160a01b0316836001600160a01b03167ff0eddf07e6ea14f388b47e1e94a0f464ecbd9eed4171130e0fc0e99fb4030a8a60405160405180910390a3505b61226982613d4c565b9050612275600160c955565b919050565b6066546002906004908116036122a35760405163840a48d560e01b815260040160405180910390fd5b6122ab61267a565b6122bf6122b786615af7565b8585856138eb565b6122c9600160c955565b5050505050565b6122d861267a565b6122e13361103b565b156122ff57604051633bf2b50360e11b815260040160405180910390fd5b6123088361191d565b612325576040516325ec6c1f60e01b815260040160405180910390fd5b61233133848484613fab565b6110b93384612e51565b60605f83516001600160401b0381111561235757612357614d99565b60405190808252806020026020018201604052801561238a57816020015b60608152602001906001900390816123755790505b5090505f5b8451811015611b6f576123bb8582815181106123ad576123ad61589a565b602002602001015185611aa1565b8282815181106123cd576123cd61589a565b602090810291909101015260010161238f565b6123e8613840565b6001600160a01b03811661244d5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b60648201526084016120ce565b6124568161389a565b50565b60408051808201909152600a81526922b4b3b2b72630bcb2b960b11b6020909101525f7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f7f71b625cfad44bac63b13dba07f2e1d6084ee04b6f8752101ece6126d584ee6ea6124c661406a565b805160209182012060408051928301949094529281019190915260608101919091524660808201523060a082015260c00160405160208183030381529060405280519060200120905090565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561256e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906125929190615b1d565b6001600160a01b0316336001600160a01b0316146125c35760405163794821ff60e01b815260040160405180910390fd5b606654801982198116146125ea5760405163c61dca5d60e01b815260040160405180910390fd5b606682905560405182815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c906020015b60405180910390a25050565b6001600160a01b0381165f90815260a36020526040902060609061194f90614106565b5f612655612459565b60405161190160f01b60208201526022810191909152604281018390526062016111be565b600260c954036126cc5760405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c0060448201526064016120ce565b600260c955565b60605f82516001600160401b038111156126ef576126ef614d99565b604051908082528060200260200182016040528015612718578160200160208202803683370190505b5090505f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663547afb8786866040518363ffffffff1660e01b815260040161276a929190615b38565b5f60405180830381865afa158015612784573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526127ab9190810190615b5b565b90505f5b845181101561281a576127f5878683815181106127ce576127ce61589a565b60200260200101518484815181106127e8576127e861589a565b60200260200101516130f2565b8382815181106128075761280761589a565b60209081029190910101526001016127af565b509095945050505050565b5f6001600160a01b03861661284d576040516339b190bb60e11b815260040160405180910390fd5b83515f0361286e5760405163796cc52560e01b815260040160405180910390fd5b5f84516001600160401b0381111561288857612888614d99565b6040519080825280602002602001820160405280156128b1578160200160208202803683370190505b5090505f85516001600160401b038111156128ce576128ce614d99565b6040519080825280602002602001820160405280156128f7578160200160208202803683370190505b5090505f5b8651811015612bc7575f61291b888381518110611e9857611e9861589a565b90505f60a25f8c6001600160a01b03166001600160a01b031681526020019081526020015f205f8a85815181106129545761295461589a565b60200260200101516001600160a01b03166001600160a01b031681526020019081526020015f206040518060200160405290815f8201548152505090506129c08884815181106129a6576129a661589a565b6020026020010151888581518110611fee57611fee61589a565b8484815181106129d2576129d261589a565b602002602001018181525050612a0a8884815181106129f3576129f361589a565b60200260200101518261411290919063ffffffff16565b858481518110612a1c57612a1c61589a565b60209081029190910101526001600160a01b038a1615612ab157612a738a8a8581518110612a4c57612a4c61589a565b6020026020010151878681518110612a6657612a6661589a565b6020026020010151614126565b612ab18a8c8b8681518110612a8a57612a8a61589a565b6020026020010151878781518110612aa457612aa461589a565b6020026020010151613729565b5f826001600160a01b031663724af4238d8c8781518110612ad457612ad461589a565b60200260200101518c8881518110612aee57612aee61589a565b60200260200101516040518463ffffffff1660e01b8152600401612b1493929190615bea565b6020604051808303815f875af1158015612b30573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612b549190615acc565b9050805f03612bb9576001600160a01b038c165f90815260a2602052604081208b51612bb992908d9088908110612b8d57612b8d61589a565b60200260200101516001600160a01b03166001600160a01b031681526020019081526020015f205f9055565b5050508060010190506128fc565b506001600160a01b0388165f908152609f60205260408120805491829190612bee83615c0e565b91905055505f6040518060e001604052808b6001600160a01b031681526020018a6001600160a01b031681526020018b6001600160a01b031681526020018381526020014363ffffffff1681526020018981526020018581525090505f612c54826111ac565b5f818152609e602090815260408083208054600160ff19909116811790915560a4835292819020865181546001600160a01b03199081166001600160a01b039283161783558885015195830180548216968316969096179095559187015160028201805490951692169190911790925560608501516003830155608085015160048301805463ffffffff191663ffffffff90921691909117905560a085015180519394508593612d0a9260058501920190614b90565b5060c08201518051612d26916006840191602090910190614bf3565b5050506001600160a01b038b165f90815260a360205260409020612d4a9082614190565b507f26b2aae26516e8719ef50ea2f6831a2efbd4e37dccdf0f6936b27bc08e793e30818386604051612d7e93929190615c26565b60405180910390a19a9950505050505050505050565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a250565b5f6119f282612de9612de287613d2d565b869061419b565b9061419b565b6001600160a01b038281165f818152609960205260409081902060010180546001600160a01b0319169385169390931790925590517f773b54c04d756fcc5e678111f7d730de3be98192000799eee3d63716055a87c69061261d908490615085565b6066545f90600190811603612e795760405163840a48d560e01b815260040160405180910390fd5b5f5f612e8485611630565b915091505f612e945f86856126d3565b6001600160a01b038781165f818152609a602052604080822080546001600160a01b031916948b16948517905551939450919290917fc3ee9f2e5fda98e8066a1f745b2df9285f416fe98cf2559cd21484b3d874330491a35f5b8351811015611c47575f516020615d9a5f395f51905f526001600160a01b0316848281518110612f2057612f2061589a565b60200260200101516001600160a01b0316036130915760405163a3d75e0960e01b81525f906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063a3d75e0990612f84908b90600401615085565b602060405180830381865afa158015612f9f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612fc39190615974565b90505f60a25f8a6001600160a01b03166001600160a01b031681526020019081526020015f205f878581518110612ffc57612ffc61589a565b60200260200101516001600160a01b03166001600160a01b031681526020019081526020015f206040518060200160405290815f82015481525050905061307085848151811061304e5761304e61589a565b6020026020010151836001600160401b0316836138229092919063ffffffff16565b8584815181106130825761308261589a565b60200260200101818152505050505b6130ea86888684815181106130a8576130a861589a565b60200260200101515f8786815181106130c3576130c361589a565b60200260200101518787815181106130dd576130dd61589a565b60200260200101516131d5565b600101612eee565b5f73beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeabf196001600160a01b038416016131c55760405163a3d75e0960e01b81525f906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063a3d75e0990613166908890600401615085565b602060405180830381865afa158015613181573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906131a59190615974565b90506131bd6001600160401b0384811690831661380e565b915050610d6c565b506001600160401b031692915050565b805f036131f557604051630a33bc6960e21b815260040160405180910390fd5b8115613312576001600160a01b038086165f90815260a26020908152604080832093881683529290522061322b818585856141af565b6040805160208101909152815481527f8be932bac54561f27260f95463d9b8ab37e06b2842e5ee2404157cc13df6eb8f908790879061326990613d2d565b60405161327893929190615bea565b60405180910390a16132898661103b565b15611c47576001600160a01b038088165f908152609860209081526040808320938916835292905290812080548592906132c49084906159a3565b92505081905550866001600160a01b03167f1ec042c965e2edd7107b51188ee0f383e22e76179041ab3a9d18ff151405166c87878660405161330893929190615bea565b60405180910390a2505b505050505050565b604051631beb2b9760e31b81526001600160a01b0382811660048301523360248301523060448301525f80356001600160e01b0319166064840152917f00000000000000000000000000000000000000000000000000000000000000009091169063df595cb8906084016020604051808303815f875af11580156133a0573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061194f9190615911565b60605f6133d08361422a565b6040805160208082528183019092529192505f91906020820181803683375050509182525060208101929092525090565b613409614b37565b5f82815260a46020908152604091829020825160e08101845281546001600160a01b0390811682526001830154811682850152600283015416818501526003820154606082810191909152600483015463ffffffff1660808301526005830180548651818702810187019097528087529195929460a086019392908301828280156134bb57602002820191905f5260205f20905b81546001600160a01b0316815260019091019060200180831161349d575b505050505081526020016006820180548060200260200160405190810160405280929190818152602001828054801561351157602002820191905f5260205f20905b8154815260200190600101908083116134fd575b50505050508152505091508160a00151516001600160401b0381111561353957613539614d99565b604051908082528060200260200182016040528015613562578160200160208202803683370190505b5090505f7f000000000000000000000000000000000000000000000000000000000000000083608001516135969190615c50565b90505f4363ffffffff168263ffffffff16106135c7576135c2845f015185602001518660a001516126d3565b6135de565b6135de845f015185602001518660a0015185614251565b90505f5b8460a001515181101561131a5761362f8560c0015182815181106136085761360861589a565b60200260200101518383815181106136225761362261589a565b602002602001015161437f565b8482815181106136415761364161589a565b60209081029190910101526001016135e2565b5f613662848385600161438a565b6119f29085615c6c565b6001600160a01b038085165f90815260a5602090815260408083209387168352929052908120819061369d906143e5565b90505f61370360016136cf7f000000000000000000000000000000000000000000000000000000000000000043615c7f565b6136d99190615c7f565b6001600160a01b03808a165f90815260a560209081526040808320938c16835292905220906143ff565b90505f6137108284615c6c565b905061371d81878761441b565b98975050505050505050565b6001600160a01b038085165f9081526098602090815260408083209386168352929052908120805483929061375f908490615c6c565b92505081905550836001600160a01b03167f6909600037b75d7b4733aedd815442b5ec018a827751c832aaff64eba5d6d2dd848484604051611a9393929190615bea565b5f6001600160a01b0382165f516020615d9a5f395f51905f52146137e7577f000000000000000000000000000000000000000000000000000000000000000061194f565b7f000000000000000000000000000000000000000000000000000000000000000092915050565b5f610d6c8383670de0b6b3a7640000614439565b5f6119f28261383a61383387613d2d565b869061380e565b9061380e565b6033546001600160a01b031633146111aa5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016120ce565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a35050565b60a0840151518214613910576040516343714afd60e01b815260040160405180910390fd5b83604001516001600160a01b0316336001600160a01b031614613946576040516316110d3560e21b815260040160405180910390fd5b5f613950856111ac565b5f818152609e602052604090205490915060ff16613981576040516387c9d21960e01b815260040160405180910390fd5b60605f7f000000000000000000000000000000000000000000000000000000000000000087608001516139b49190615c50565b90508063ffffffff164363ffffffff16116139e2576040516378f67ae160e11b815260040160405180910390fd5b6139f9875f015188602001518960a0015184614251565b87516001600160a01b03165f90815260a360205260409020909250613a1f91508361451e565b505f82815260a46020526040812080546001600160a01b031990811682556001820180548216905560028201805490911690556003810182905560048101805463ffffffff1916905590613a766005830182614c2c565b613a83600683015f614c2c565b50505f828152609e602052604090819020805460ff19169055517f1f40400889274ed07b24845e5054a87a0cab969eb1277aafe61ae352e7c32a0090613acc9084815260200190565b60405180910390a185516001600160a01b039081165f908152609a6020526040812054885160a08a01519190931692613b069184906126d3565b90505f5b8860a0015151811015613d22575f613b318a60a001518381518110611e9857611e9861589a565b90505f613b678b60c001518481518110613b4d57613b4d61589a565b60200260200101518785815181106136225761362261589a565b9050805f03613b77575050613d1a565b8715613c4557816001600160a01b0316632eae418c8c5f01518d60a001518681518110613ba657613ba661589a565b60200260200101518d8d88818110613bc057613bc061589a565b9050602002016020810190613bd591906150dc565b60405160e085901b6001600160e01b03191681526001600160a01b03938416600482015291831660248301529091166044820152606481018490526084015f604051808303815f87803b158015613c2a575f5ffd5b505af1158015613c3c573d5f5f3e3d5ffd5b50505050613d17565b5f5f836001600160a01b03166350ff72258e5f01518f60a001518881518110613c7057613c7061589a565b6020026020010151866040518463ffffffff1660e01b8152600401613c9793929190615bea565b60408051808303815f875af1158015613cb2573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613cd69190615c9b565b91509150613d14878e5f01518f60a001518881518110613cf857613cf861589a565b602002602001015185858b8b815181106130dd576130dd61589a565b50505b50505b600101613b0a565b505050505050505050565b80515f9015613d3d57815161194f565b670de0b6b3a764000092915050565b606654606090600190600290811603613d785760405163840a48d560e01b815260040160405180910390fd5b6001600160a01b038084165f818152609a602052604080822080546001600160a01b0319811690915590519316928392917ffee30966a256b71e14bc0ebfc94315e28ef4a97a7131a9e2b7a310a73af4467691a35f5f613dd786611630565b9150915081515f03613deb57505050613fa5565b81516001600160401b03811115613e0457613e04614d99565b604051908082528060200260200182016040528015613e2d578160200160208202803683370190505b5094505f613e3c8785856126d3565b90505f5b8351811015613f9f576040805160018082528183019092525f916020808301908036833750506040805160018082528183019092529293505f9291506020808301908036833750506040805160018082528183019092529293505f92915060208083019080368337019050509050868481518110613ec057613ec061589a565b6020026020010151835f81518110613eda57613eda61589a565b60200260200101906001600160a01b031690816001600160a01b031681525050858481518110613f0c57613f0c61589a565b6020026020010151825f81518110613f2657613f2661589a565b602002602001018181525050848481518110613f4457613f4461589a565b6020026020010151815f81518110613f5e57613f5e61589a565b602002602001018181525050613f778b89858585612825565b8a8581518110613f8957613f8961589a565b6020908102919091010152505050600101613e40565b50505050505b50919050565b6001600160a01b038084165f908152609960205260409020600101541680613fd35750610ec4565b6001600160a01b0381165f908152609c6020908152604080832085845290915290205460ff161561401757604051630d4c4c9160e21b815260040160405180910390fd5b6001600160a01b0381165f908152609c602090815260408083208584528252909120805460ff191660011790558301516122c990829061405e908890889084908890610820565b85516020870151614529565b60605f6140967f00000000000000000000000000000000000000000000000000000000000000006133c4565b9050805f815181106140aa576140aa61589a565b602001015160f81c60f81b816001815181106140c8576140c861589a565b016020908101516040516001600160f81b03199384169281019290925291909116602182015260220160405160208183030381529060405291505090565b60605f610d6c8361457b565b5f610d6c61411f84613d2d565b839061380e565b6001600160a01b038084165f90815260a5602090815260408083209386168352929052908120614155906143e5565b9050610ec44361416584846159a3565b6001600160a01b038088165f90815260a560209081526040808320938a1683529290522091906145d4565b5f610d6c83836145df565b5f610d6c83670de0b6b3a764000084614439565b825f036141db576040805160208101909152845481526141d4908290612de990613d2d565b8455610ec4565b6040805160208101909152845481525f906141f7908584613822565b90505f61420484836159a3565b90505f61421f84612de9614218888a6159a3565b859061419b565b875550505050505050565b5f60ff8216601f81111561194f57604051632cd44ac360e21b815260040160405180910390fd5b60605f83516001600160401b0381111561426d5761426d614d99565b604051908082528060200260200182016040528015614296578160200160208202803683370190505b5090505f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166394d7d00c8787876040518463ffffffff1660e01b81526004016142ea93929190615cbd565b5f60405180830381865afa158015614304573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261432b9190810190615b5b565b90505f5b85518110156143735761434e888783815181106127ce576127ce61589a565b8382815181106143605761436061589a565b602090810291909101015260010161432f565b50909695505050505050565b5f610d6c838361380e565b5f5f614397868686614439565b905060018360028111156143ad576143ad615cf6565b1480156143c957505f84806143c4576143c4615d0a565b868809115b156143dc576143d96001826159a3565b90505b95945050505050565b5f6143f0828261462b565b6001600160e01b031692915050565b5f61440b83838361466f565b6001600160e01b03169392505050565b5f6119f26144298385615d1e565b85906001600160401b031661380e565b5f80805f19858709858702925082811083820303915050805f036144705783828161446657614466615d0a565b0492505050610d6c565b8084116144b75760405162461bcd60e51b81526020600482015260156024820152744d6174683a206d756c446976206f766572666c6f7760581b60448201526064016120ce565b5f8486880960026001871981018816978890046003810283188082028403028082028403028082028403028082028403028082028403029081029092039091025f889003889004909101858311909403939093029303949094049190911702949350505050565b5f610d6c83836146b7565b4281101561454a57604051630819bdcd60e01b815260040160405180910390fd5b61455e6001600160a01b038516848461479a565b610ec457604051638baa579f60e01b815260040160405180910390fd5b6060815f018054806020026020016040519081016040528092919081815260200182805480156145c857602002820191905f5260205f20905b8154815260200190600101908083116145b4575b50505050509050919050565b6110c38383836147ee565b5f81815260018301602052604081205461462457508154600181810184555f84815260208082209093018490558454848252828601909352604090209190915561194f565b505f61194f565b81545f9080156146675761465184614644600184615c6c565b5f91825260209091200190565b54600160201b90046001600160e01b03166119f2565b509092915050565b82545f9081614680868683856148f1565b905080156146ad5761469786614644600184615c6c565b54600160201b90046001600160e01b031661089e565b5091949350505050565b5f8181526001830160205260408120548015614791575f6146d9600183615c6c565b85549091505f906146ec90600190615c6c565b905081811461474b575f865f01828154811061470a5761470a61589a565b905f5260205f200154905080875f01848154811061472a5761472a61589a565b5f918252602080832090910192909255918252600188019052604090208390555b855486908061475c5761475c615d3d565b600190038181905f5260205f20015f90559055856001015f8681526020019081526020015f205f90556001935050505061194f565b5f91505061194f565b5f5f5f6147a78585614944565b90925090505f8160048111156147bf576147bf615cf6565b1480156147dd5750856001600160a01b0316826001600160a01b0316145b8061089e575061089e868686614983565b825480156148a4575f61480685614644600185615c6c565b60408051808201909152905463ffffffff808216808452600160201b9092046001600160e01b0316602084015291925090851610156148585760405163151b8e3f60e11b815260040160405180910390fd5b805163ffffffff8086169116036148a2578261487986614644600186615c6c565b80546001600160e01b0392909216600160201b0263ffffffff9092169190911790555050505050565b505b506040805180820190915263ffffffff92831681526001600160e01b03918216602080830191825285546001810187555f968752952091519051909216600160201b029190921617910155565b5f5b81831015611b6f575f6149068484614a6a565b5f8781526020902090915063ffffffff86169082015463ffffffff1611156149305780925061493e565b61493b8160016159a3565b93505b506148f3565b5f5f8251604103614978576020830151604084015160608501515f1a61496c87828585614a84565b94509450505050612031565b505f90506002612031565b5f5f5f856001600160a01b0316631626ba7e60e01b86866040516024016149ab929190615d51565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b03199094169390931790925290516149e99190615d69565b5f60405180830381855afa9150503d805f8114614a21576040519150601f19603f3d011682016040523d82523d5f602084013e614a26565b606091505b5091509150818015614a3a57506020815110155b801561089e57508051630b135d3f60e11b90614a5f9083016020908101908401615acc565b149695505050505050565b5f614a786002848418615d7a565b610d6c908484166159a3565b5f806fa2a8918ca85bafe22016d0b997e4df60600160ff1b03831115614aaf57505f90506003614b2e565b604080515f8082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa158015614b00573d5f5f3e3d5ffd5b5050604051601f1901519150506001600160a01b038116614b28575f60019250925050614b2e565b91505f90505b94509492505050565b6040518060e001604052805f6001600160a01b031681526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f63ffffffff16815260200160608152602001606081525090565b828054828255905f5260205f20908101928215614be3579160200282015b82811115614be357825182546001600160a01b0319166001600160a01b03909116178255602090920191600190910190614bae565b50614bef929150614c43565b5090565b828054828255905f5260205f20908101928215614be3579160200282015b82811115614be3578251825591602001919060010190614c11565b5080545f8255905f5260205f209081019061245691905b5b80821115614bef575f8155600101614c44565b6001600160a01b0381168114612456575f5ffd5b803561227581614c57565b5f5f5f5f5f60a08688031215614c8a575f5ffd5b8535614c9581614c57565b94506020860135614ca581614c57565b93506040860135614cb581614c57565b94979396509394606081013594506080013592915050565b5f5f83601f840112614cdd575f5ffd5b5081356001600160401b03811115614cf3575f5ffd5b6020830191508360208260051b8501011115612031575f5ffd5b5f5f60208385031215614d1e575f5ffd5b82356001600160401b03811115614d33575f5ffd5b614d3f85828601614ccd565b90969095509350505050565b602080825282518282018190525f918401906040840190835b8181101561281a578351835260209384019390920191600101614d64565b5f60208284031215614d92575f5ffd5b5035919050565b634e487b7160e01b5f52604160045260245ffd5b60405160e081016001600160401b0381118282101715614dcf57614dcf614d99565b60405290565b604080519081016001600160401b0381118282101715614dcf57614dcf614d99565b604051601f8201601f191681016001600160401b0381118282101715614e1f57614e1f614d99565b604052919050565b5f6001600160401b03821115614e3f57614e3f614d99565b5060051b60200190565b5f82601f830112614e58575f5ffd5b8135614e6b614e6682614e27565b614df7565b8082825260208201915060208360051b860101925085831115614e8c575f5ffd5b602085015b83811015614eb2578035614ea481614c57565b835260209283019201614e91565b5095945050505050565b5f82601f830112614ecb575f5ffd5b8135614ed9614e6682614e27565b8082825260208201915060208360051b860101925085831115614efa575f5ffd5b602085015b83811015614eb2578035835260209283019201614eff565b5f5f5f60608486031215614f29575f5ffd5b8335614f3481614c57565b925060208401356001600160401b03811115614f4e575f5ffd5b614f5a86828701614e49565b92505060408401356001600160401b03811115614f75575f5ffd5b614f8186828701614ebc565b9150509250925092565b5f8151808452602084019350602083015f5b82811015614fbb578151865260209586019590910190600101614f9d565b5093949350505050565b602081525f610d6c6020830184614f8b565b803563ffffffff81168114612275575f5ffd5b5f5f83601f840112614ffa575f5ffd5b5081356001600160401b03811115615010575f5ffd5b602083019150836020828501011115612031575f5ffd5b5f5f5f5f6060858703121561503a575f5ffd5b843561504581614c57565b935061505360208601614fd7565b925060408501356001600160401b0381111561506d575f5ffd5b61507987828801614fea565b95989497509550505050565b6001600160a01b0391909116815260200190565b5f5f5f5f608085870312156150ac575f5ffd5b84356150b781614c57565b935060208501356150c781614c57565b93969395505050506040820135916060013590565b5f602082840312156150ec575f5ffd5b8135610d6c81614c57565b5f5f60408385031215615108575f5ffd5b823561511381614c57565b9150602083013561512381614c57565b809150509250929050565b5f5b83811015615148578181015183820152602001615130565b50505f910152565b5f815180845261516781602086016020860161512e565b601f01601f19169290920160200192915050565b602081525f610d6c6020830184615150565b5f60e0828403121561519d575f5ffd5b6151a5614dad565b90506151b082614c6b565b81526151be60208301614c6b565b60208201526151cf60408301614c6b565b6040820152606082810135908201526151ea60808301614fd7565b608082015260a08201356001600160401b03811115615207575f5ffd5b61521384828501614e49565b60a08301525060c08201356001600160401b03811115615231575f5ffd5b61523d84828501614ebc565b60c08301525092915050565b5f60208284031215615259575f5ffd5b81356001600160401b0381111561526e575f5ffd5b6119f28482850161518d565b5f6020828403121561528a575f5ffd5b813560ff81168114610d6c575f5ffd5b5f8151808452602084019350602083015f5b82811015614fbb5781516001600160a01b03168652602095860195909101906001016152ac565b80516001600160a01b03908116835260208083015182169084015260408083015190911690830152606080820151908301526080808201515f9161531e9085018263ffffffff169052565b5060a082015160e060a085015261533860e085018261529a565b905060c083015184820360c08601526143dc8282614f8b565b604081525f61536360408301856152d3565b82810360208401526143dc8185614f8b565b5f82825180855260208501945060208160051b830101602085015f5b8381101561437357601f198584030188526153ad838351614f8b565b6020988901989093509190910190600101615391565b5f604082016040835280855180835260608501915060608160051b8601019250602087015f5b8281101561541a57605f198786030184526154058583516152d3565b945060209384019391909101906001016153e9565b5050505082810360208401526143dc8185615375565b6001600160401b0381168114612456575f5ffd5b5f5f5f5f60808587031215615457575f5ffd5b843561546281614c57565b9350602085013561547281614c57565b9250604085013561548281615430565b9150606085013561549281615430565b939692955090935050565b5f5f5f606084860312156154af575f5ffd5b83356154ba81614c57565b92506020840135915060408401356154d181615430565b809150509250925092565b604081525f615363604083018561529a565b5f5f5f60408486031215615500575f5ffd5b833561550b81614c57565b925060208401356001600160401b03811115615525575f5ffd5b61553186828701614fea565b9497909650939450505050565b5f5f6040838503121561554f575f5ffd5b823561555a81614c57565b915060208301356001600160401b03811115615574575f5ffd5b61558085828601614e49565b9150509250929050565b5f5f5f5f5f5f6060878903121561559f575f5ffd5b86356001600160401b038111156155b4575f5ffd5b6155c089828a01614ccd565b90975095505060208701356001600160401b038111156155de575f5ffd5b6155ea89828a01614ccd565b90955093505060408701356001600160401b03811115615608575f5ffd5b61561489828a01614ccd565b979a9699509497509295939492505050565b602081525f610d6c60208301846152d3565b5f5f5f6060848603121561564a575f5ffd5b833561565581614c57565b925060208401356001600160401b0381111561566f575f5ffd5b840160408187031215615680575f5ffd5b615688614dd5565b81356001600160401b0381111561569d575f5ffd5b8201601f810188136156ad575f5ffd5b80356001600160401b038111156156c6576156c6614d99565b6156d9601f8201601f1916602001614df7565b8181528960208385010111156156ed575f5ffd5b816020840160208301375f60209282018301528352928301359282019290925293969395505050506040919091013590565b5f5f60408385031215615730575f5ffd5b823561573b81614c57565b946020939093013593505050565b604081525f6153636040830185614f8b565b8015158114612456575f5ffd5b5f5f5f5f6060858703121561577b575f5ffd5b84356001600160401b03811115615790575f5ffd5b850160e081880312156157a1575f5ffd5b935060208501356001600160401b038111156157bb575f5ffd5b6157c787828801614ccd565b90945092505060408501356154928161575b565b5f5f604083850312156157ec575f5ffd5b82356001600160401b03811115615801575f5ffd5b8301601f81018513615811575f5ffd5b803561581f614e6682614e27565b8082825260208201915060208360051b850101925087831115615840575f5ffd5b6020840193505b8284101561586b57833561585a81614c57565b825260209384019390910190615847565b945050505060208301356001600160401b03811115615574575f5ffd5b602081525f610d6c6020830184615375565b634e487b7160e01b5f52603260045260245ffd5b5f8235605e198336030181126158c2575f5ffd5b9190910192915050565b5f5f8335601e198436030181126158e1575f5ffd5b8301803591506001600160401b038211156158fa575f5ffd5b6020019150600581901b3603821315612031575f5ffd5b5f60208284031215615921575f5ffd5b8151610d6c8161575b565b60208152816020820152818360408301375f818301604090810191909152601f909201601f19160101919050565b6001600160a01b0392831681529116602082015260400190565b5f60208284031215615984575f5ffd5b8151610d6c81615430565b634e487b7160e01b5f52601160045260245ffd5b8082018082111561194f5761194f61598f565b5f82601f8301126159c5575f5ffd5b81516159d3614e6682614e27565b8082825260208201915060208360051b8601019250858311156159f4575f5ffd5b602085015b83811015614eb25780518352602092830192016159f9565b5f5f60408385031215615a22575f5ffd5b82516001600160401b03811115615a37575f5ffd5b8301601f81018513615a47575f5ffd5b8051615a55614e6682614e27565b8082825260208201915060208360051b850101925087831115615a76575f5ffd5b6020840193505b82841015615aa1578351615a9081614c57565b825260209384019390910190615a7d565b8095505050505060208301516001600160401b03811115615ac0575f5ffd5b615580858286016159b6565b5f60208284031215615adc575f5ffd5b5051919050565b5f823560de198336030181126158c2575f5ffd5b5f61194f368361518d565b5f60208284031215615b12575f5ffd5b8135610d6c8161575b565b5f60208284031215615b2d575f5ffd5b8151610d6c81614c57565b6001600160a01b03831681526040602082018190525f906119f29083018461529a565b5f60208284031215615b6b575f5ffd5b81516001600160401b03811115615b80575f5ffd5b8201601f81018413615b90575f5ffd5b8051615b9e614e6682614e27565b8082825260208201915060208360051b850101925086831115615bbf575f5ffd5b6020840193505b8284101561089e578351615bd981615430565b825260209384019390910190615bc6565b6001600160a01b039384168152919092166020820152604081019190915260600190565b5f60018201615c1f57615c1f61598f565b5060010190565b838152606060208201525f615c3e60608301856152d3565b828103604084015261089e8185614f8b565b63ffffffff818116838216019081111561194f5761194f61598f565b8181038181111561194f5761194f61598f565b63ffffffff828116828216039081111561194f5761194f61598f565b5f5f60408385031215615cac575f5ffd5b505080516020909101519092909150565b6001600160a01b03841681526060602082018190525f90615ce09083018561529a565b905063ffffffff83166040830152949350505050565b634e487b7160e01b5f52602160045260245ffd5b634e487b7160e01b5f52601260045260245ffd5b6001600160401b03828116828216039081111561194f5761194f61598f565b634e487b7160e01b5f52603160045260245ffd5b828152604060208201525f6119f26040830184615150565b5f82516158c281846020870161512e565b5f82615d9457634e487b7160e01b5f52601260045260245ffd5b50049056fe000000000000000000000000beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac0a2646970667358221220e1c0613b67b7d291dbd51e9b97ad592681c391bc5ea883c66bc8728ebbdeedd464736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x02\xC3W_5`\xE0\x1C\x80cqP\x18\xA6\x11a\x01wW\x80c\xBF\xAE?\xD2\x11a\0\xD5W\x80c\xE4\xCC?\x90\x11a\0\x8FW\x80c\xE4\xCC?\x90\x14a\x07\x99W\x80c\xEE\xA9\x06K\x14a\x07\xACW\x80c\xF0\xE0\xE6v\x14a\x07\xBFW\x80c\xF2\xFD\xE3\x8B\x14a\x07\xDFW\x80c\xF6\x98\xDA%\x14a\x07\xF2W\x80c\xFA\xBC\x1C\xBC\x14a\x07\xFAW\x80c\xFD\x8A\xA8\x8D\x14a\x08\rW__\xFD[\x80c\xBF\xAE?\xD2\x14a\x06\xE4W\x80c\xC4H\xFE\xB8\x14a\x06\xF7W\x80c\xC9x\xF7\xAC\x14a\x07+W\x80c\xCA\x8A\xA7\xC7\x14a\x07LW\x80c\xCDm\xC6\x87\x14a\x07sW\x80c\xDA\x8B\xE8d\x14a\x07\x86W__\xFD[\x80c\x91\x04\xC3\x19\x11a\x011W\x80c\x91\x04\xC3\x19\x14a\x06\x1CW\x80c\x945\xBBC\x14a\x060W\x80c\x99\xF57\x1B\x14a\x06CW\x80c\xA1x\x84\x84\x14a\x06cW\x80c\xA3:43\x14a\x06\x82W\x80c\xB7\xF0n\xBE\x14a\x06\x95W\x80c\xBBE\xFE\xF2\x14a\x06\xB7W__\xFD[\x80cqP\x18\xA6\x14a\x05\x8CW\x80cw\x8EU\xF3\x14a\x05\x94W\x80cx)n\xC5\x14a\x05\xBEW\x80c\x88o\x11\x95\x14a\x05\xD1W\x80c\x8D\xA5\xCB[\x14a\x05\xF8W\x80c\x90\x04\x13G\x14a\x06\tW__\xFD[\x80cT\xFDMP\x11a\x02$W\x80c]\xD6\x85y\x11a\x01\xDEW\x80c]\xD6\x85y\x14a\x04\xD6W\x80c`\x1B\xB3o\x14a\x04\xF7W\x80c`\xA0\xD1\xCE\x14a\x05\nW\x80ce\xDA\x12d\x14a\x05\x1DW\x80cf\xD5\xBA\x93\x14a\x05EW\x80cmp\xF7\xAE\x14a\x05fW\x80cn\x17DH\x14a\x05yW__\xFD[\x80cT\xFDMP\x14a\x04ZW\x80cY\\jg\x14a\x04oW\x80cY{6\xDA\x14a\x04wW\x80cZ\xC8j\xB7\x14a\x04\x8AW\x80c\\\x97Z\xBB\x14a\x04\xADW\x80c]\x97^\x88\x14a\x04\xB5W__\xFD[\x80c9\xB7\x0E8\x11a\x02\x80W\x80c9\xB7\x0E8\x14a\x03|W\x80c<e\x1C\xF2\x14a\x03\xB0W\x80c<\xDE\xB5\xE0\x14a\x03\xC3W\x80c>(9\x1D\x14a\x03\xD6W\x80cFW\xE2j\x14a\x03\xF9W\x80cFe\xBC\xDA\x14a\x04 W\x80cT\xB7\xC9l\x14a\x04GW__\xFD[\x80c\x04\xA4\xF9y\x14a\x02\xC7W\x80c\x0B\x9FHz\x14a\x03\x01W\x80c\r\xD8\xDD\x02\x14a\x03\x14W\x80c\x13d9\xDD\x14a\x034W\x80c%\xDF\x92.\x14a\x03IW\x80c*\xA6\xD8\x88\x14a\x03iW[__\xFD[a\x02\xEE\x7F\x14\xBD\xE6t\xC9\xF6K*\xD0\x0E\xAA\xEEJ\x8B\xED\x1F\xAB\xEF5\xC7P~<[\x9C\xFC\x946\x90\x9A-\xAD\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\xEEa\x03\x0F6`\x04aLvV[a\x08 V[a\x03'a\x03\"6`\x04aM\rV[a\x08\xA8V[`@Qa\x02\xF8\x91\x90aMKV[a\x03Ga\x03B6`\x04aM\x82V[a\x0B9V[\0[a\x03\\a\x03W6`\x04aO\x17V[a\x0C\x13V[`@Qa\x02\xF8\x91\x90aO\xC5V[a\x03Ga\x03w6`\x04aP'V[a\rsV[a\x03\xA3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qa\x02\xF8\x91\x90aP\x85V[a\x03Ga\x03\xBE6`\x04aP\x99V[a\x0E\xCAV[a\x03\xA3a\x03\xD16`\x04aP\xDCV[a\x10\x1BV[a\x03\xE9a\x03\xE46`\x04aP\xDCV[a\x10;V[`@Q\x90\x15\x15\x81R` \x01a\x02\xF8V[a\x03\xA3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\xA3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03Ga\x04U6`\x04aP\xF7V[a\x10ZV[a\x04ba\x10\xC8V[`@Qa\x02\xF8\x91\x90aQ{V[a\x03Ga\x10\xF8V[a\x02\xEEa\x04\x856`\x04aRIV[a\x11\xACV[a\x03\xE9a\x04\x986`\x04aRzV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fTa\x02\xEEV[a\x04\xC8a\x04\xC36`\x04aM\x82V[a\x11\xDBV[`@Qa\x02\xF8\x92\x91\x90aSQV[a\x04\xE9a\x04\xE46`\x04aP\xDCV[a\x11\xF8V[`@Qa\x02\xF8\x92\x91\x90aS\xC3V[a\x03Ga\x05\x056`\x04aTDV[a\x13\"V[a\x03Ga\x05\x186`\x04aT\x9DV[a\x14\x9AV[a\x03\xA3a\x05+6`\x04aP\xDCV[`\x9A` R_\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x05Xa\x05S6`\x04aP\xDCV[a\x160V[`@Qa\x02\xF8\x92\x91\x90aT\xDCV[a\x03\xE9a\x05t6`\x04aP\xDCV[a\x19\x1DV[a\x02\xEEa\x05\x876`\x04aP\xF7V[a\x19UV[a\x03Ga\x19\xFAV[a\x02\xEEa\x05\xA26`\x04aP\xF7V[`\x98` \x90\x81R_\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[a\x03Ga\x05\xCC6`\x04aT\xEEV[a\x1A\x0BV[a\x03\xA3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03\xA3V[a\x03\\a\x06\x176`\x04aU>V[a\x1A\xA1V[a\x03\xA3_Q` a]\x9A_9_Q\x90_R\x81V[a\x03Ga\x06>6`\x04aU\x8AV[a\x1BwV[a\x06Va\x06Q6`\x04aM\x82V[a\x1CPV[`@Qa\x02\xF8\x91\x90aV&V[a\x02\xEEa\x06q6`\x04aP\xDCV[`\x9F` R_\x90\x81R`@\x90 T\x81V[a\x03'a\x06\x906`\x04aV8V[a\x1DlV[a\x03\xE9a\x06\xA36`\x04aM\x82V[`\x9E` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x03\xE9a\x06\xC56`\x04aW\x1FV[`\x9C` \x90\x81R_\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[a\x02\xEEa\x06\xF26`\x04aP\xF7V[a\x1D\x84V[`@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R` \x01a\x02\xF8V[a\x07>a\x0796`\x04aU>V[a\x1D\xC0V[`@Qa\x02\xF8\x92\x91\x90aWIV[a\x03\xA3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03Ga\x07\x816`\x04aW\x1FV[a 8V[a\x03'a\x07\x946`\x04aP\xDCV[a!SV[a\x03Ga\x07\xA76`\x04aWhV[a\"zV[a\x03Ga\x07\xBA6`\x04aV8V[a\"\xD0V[a\x07\xD2a\x07\xCD6`\x04aW\xDBV[a#;V[`@Qa\x02\xF8\x91\x90aX\x88V[a\x03Ga\x07\xED6`\x04aP\xDCV[a#\xE0V[a\x02\xEEa$YV[a\x03Ga\x08\x086`\x04aM\x82V[a%\x12V[a\x03'a\x08\x1B6`\x04aP\xDCV[a&)V[`@\x80Q\x7F\x14\xBD\xE6t\xC9\xF6K*\xD0\x0E\xAA\xEEJ\x8B\xED\x1F\xAB\xEF5\xC7P~<[\x9C\xFC\x946\x90\x9A-\xAD` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x92\x82\x01\x92\x90\x92R\x81\x87\x16``\x82\x01R\x90\x85\x16`\x80\x82\x01R`\xA0\x81\x01\x83\x90R`\xC0\x81\x01\x82\x90R_\x90a\x08\x9E\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a&LV[\x96\x95PPPPPPV[`fT``\x90`\x01\x90`\x02\x90\x81\x16\x03a\x08\xD4W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08\xDCa&zV[_\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08\xF5Wa\x08\xF5aM\x99V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\t\x1EW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P3_\x90\x81R`\x9A` R`@\x81 T\x91\x92P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90[\x85\x81\x10\x15a\x0B*W\x86\x86\x82\x81\x81\x10a\tXWa\tXaX\x9AV[\x90P` \x02\x81\x01\x90a\tj\x91\x90aX\xAEV[a\tx\x90` \x81\x01\x90aX\xCCV[\x90P\x87\x87\x83\x81\x81\x10a\t\x8CWa\t\x8CaX\x9AV[\x90P` \x02\x81\x01\x90a\t\x9E\x91\x90aX\xAEV[a\t\xA8\x90\x80aX\xCCV[\x90P\x14a\t\xC8W`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\n23\x84\x8A\x8A\x86\x81\x81\x10a\t\xE0Wa\t\xE0aX\x9AV[\x90P` \x02\x81\x01\x90a\t\xF2\x91\x90aX\xAEV[a\t\xFC\x90\x80aX\xCCV[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RPa&\xD3\x92PPPV[\x90Pa\x0B\x043\x84\x8A\x8A\x86\x81\x81\x10a\nKWa\nKaX\x9AV[\x90P` \x02\x81\x01\x90a\n]\x91\x90aX\xAEV[a\ng\x90\x80aX\xCCV[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RP\x8E\x92P\x8D\x91P\x88\x90P\x81\x81\x10a\n\xACWa\n\xACaX\x9AV[\x90P` \x02\x81\x01\x90a\n\xBE\x91\x90aX\xAEV[a\n\xCC\x90` \x81\x01\x90aX\xCCV[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RP\x88\x92Pa(%\x91PPV[\x84\x83\x81Q\x81\x10a\x0B\x16Wa\x0B\x16aX\x9AV[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\t>V[PP`\x01`\xC9U\x94\x93PPPPV[`@Qc#}\xFBG`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cF\xFB\xF6\x8E\x90a\x0B\x85\x903\x90`\x04\x01aP\x85V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xA0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xC4\x91\x90aY\x11V[a\x0B\xE1W`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`fT\x81\x81\x16\x81\x14a\x0C\x06W`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0C\x0F\x82a-\x94V[PPV[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16_\x90\x81R`\x9A` R`@\x81 T``\x92\x16\x90a\x0C<\x86\x83\x87a&\xD3V[\x90P_\x85Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0CXWa\x0CXaM\x99V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C\x81W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x86Q\x81\x10\x15a\rfW`\x01`\x01`\xA0\x1B\x03\x88\x16_\x90\x81R`\xA2` R`@\x81 \x88Q\x82\x90\x8A\x90\x85\x90\x81\x10a\x0C\xBCWa\x0C\xBCaX\x9AV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80` \x01`@R\x90\x81_\x82\x01T\x81RPP\x90Pa\r@\x87\x83\x81Q\x81\x10a\r\x0EWa\r\x0EaX\x9AV[` \x02` \x01\x01Q\x85\x84\x81Q\x81\x10a\r(Wa\r(aX\x9AV[` \x02` \x01\x01Q\x83a-\xD1\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83\x83\x81Q\x81\x10a\rRWa\rRaX\x9AV[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x0C\x86V[P\x92PPP[\x93\x92PPPV[a\r{a&zV[a\r\x843a\x10;V[\x15a\r\xA2W`@Qc;\xF2\xB5\x03`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc+bA\xF3`\xE1\x1B\x81R3`\x04\x82\x01Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cV\xC4\x83\xE6\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0E\x0CW__\xFD[PZ\xF1\x15\x80\x15a\x0E\x1EW=__>=_\xFD[PPPPa\x0E,3\x85a-\xEFV[a\x0E633a.QV[3`\x01`\x01`\xA0\x1B\x03\x16\x7F\xA4S\xDBa*\xF5\x9EU!\xD6\xAB\x92\x84\xDC>-\x06\xAF(n\xB1\xB1\xB7\xB7q\xFC\xE4ql\x19\xF2\xC1\x85`@Qa\x0Eo\x91\x90aP\x85V[`@Q\x80\x91\x03\x90\xA23`\x01`\x01`\xA0\x1B\x03\x16\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x83\x83`@Qa\x0E\xB2\x92\x91\x90aY,V[`@Q\x80\x91\x03\x90\xA2a\x0E\xC4`\x01`\xC9UV[PPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x0F)WP3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14[a\x0FFW`@Qc\x04R\x06\xA5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0FNa&zV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16_\x90\x81R`\x9A` R`@\x80\x82 T\x90Qc\x15&g\xD9`\xE3\x1B\x81R\x90\x83\x16\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA93>\xC8\x90a\x0F\xB1\x90\x85\x90\x89\x90`\x04\x01aYZV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xCCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xF0\x91\x90aYtV[\x90P_a\x0F\xFE\x87\x87\x84a0\xF2V[\x90Pa\x10\x0E\x83\x88\x88\x88\x88\x86a1\xD5V[PPPa\x0E\xC4`\x01`\xC9UV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`\x99` R`@\x90 `\x01\x01T\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`\x9A` R`@\x90 T\x16\x15\x15\x90V[\x81a\x10d\x81a3\x1AV[a\x10\x81W`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x10\x89a&zV[a\x10\x92\x83a\x19\x1DV[a\x10\xAFW`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x10\xB9\x83\x83a-\xEFV[a\x10\xC3`\x01`\xC9UV[PPPV[``a\x10\xF3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a3\xC4V[\x90P\x90V[`@Qc#}\xFBG`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cF\xFB\xF6\x8E\x90a\x11D\x903\x90`\x04\x01aP\x85V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11_W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x83\x91\x90aY\x11V[a\x11\xA0W`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x11\xAA_\x19a-\x94V[V[_\x81`@Q` \x01a\x11\xBE\x91\x90aV&V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[a\x11\xE3aK7V[``a\x11\xEE\x83a4\x01V[\x90\x94\x90\x93P\x91PPV[``\x80_a\x12\x05\x84a&)V[\x80Q\x90\x91P\x80`\x01`\x01`@\x1B\x03\x81\x11\x15a\x12\"Wa\x12\"aM\x99V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x12[W\x81` \x01[a\x12HaK7V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x12@W\x90P[P\x93P\x80`\x01`\x01`@\x1B\x03\x81\x11\x15a\x12vWa\x12vaM\x99V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x12\xA9W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x12\x94W\x90P[P\x92P_[\x81\x81\x10\x15a\x13\x1AWa\x12\xD8\x83\x82\x81Q\x81\x10a\x12\xCBWa\x12\xCBaX\x9AV[` \x02` \x01\x01Qa4\x01V[\x86\x83\x81Q\x81\x10a\x12\xEAWa\x12\xEAaX\x9AV[` \x02` \x01\x01\x86\x84\x81Q\x81\x10a\x13\x03Wa\x13\x03aX\x9AV[` \x90\x81\x02\x91\x90\x91\x01\x01\x91\x90\x91RR`\x01\x01a\x12\xAEV[PPP\x91P\x91V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x13kW`@Qc#\xD8q\xA5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x13sa&zV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16_\x90\x81R`\x98` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R\x90\x81 Ta\x13\xB1\x90`\x01`\x01`@\x1B\x03\x80\x86\x16\x90\x85\x16a6TV[\x90P_a\x13\xC0\x86\x86\x86\x86a6lV[\x90P_a\x13\xCD\x82\x84aY\xA3V[\x90Pa\x13\xDB\x87_\x88\x86a7)V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x88\x81\x16\x82R` \x82\x01\x84\x90R\x89\x16\x91\x7F\xDDa\x1FN\xF6?C\x85\xF1ul\x86\xCE\x1F\x1F8\x9A\x90\x13\xBAo\xA0}\xAB\xA8R\x82\x91\xBC-<0\x91\x01`@Q\x80\x91\x03\x90\xA2_a\x14+\x87a7\xA3V[`@Qc\xDE\xBE\x1E\xAB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16`\x04\x83\x01R`$\x82\x01\x85\x90R\x91\x92P\x90\x82\x16\x90c\xDE\xBE\x1E\xAB\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x14vW__\xFD[PZ\xF1\x15\x80\x15a\x14\x88W=__>=_\xFD[PPPPPPPPa\x0E\xC4`\x01`\xC9UV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x14\xE3W`@Qc2\x13\xA6a`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x14\xEBa&zV[a\x14\xF4\x83a\x10;V[\x15a\x10\xB9W`\x01`\x01`\xA0\x1B\x03\x80\x84\x16_\x90\x81R`\x9A` R`@\x80\x82 T\x90Qc\x15&g\xD9`\xE3\x1B\x81R\x90\x83\x16\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA93>\xC8\x90a\x15i\x90\x85\x90_Q` a]\x9A_9_Q\x90_R\x90`\x04\x01aYZV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\x84W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xA8\x91\x90aYtV[`\x01`\x01`\xA0\x1B\x03\x86\x16_\x90\x81R`\xA2` \x90\x81R`@\x80\x83 _Q` a]\x9A_9_Q\x90_R\x84R\x82R\x80\x83 \x81Q\x92\x83\x01\x90\x91RT\x81R\x91\x92Pa\x16\x07\x86a\x15\xFF`\x01`\x01`@\x1B\x03\x80\x87\x16\x90\x89\x16a8\x0EV[\x84\x91\x90a8\"V[\x90Pa\x16\"\x84\x88_Q` a]\x9A_9_Q\x90_R\x84a7)V[PPPPa\x10\xC3`\x01`\xC9UV[``\x80__\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x94\xF6I\xDD\x86`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16\x81\x91\x90aP\x85V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\x9BW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x16\xC2\x91\x90\x81\x01\x90aZ\x11V[\x91P\x91P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xFE$:\x17\x87_Q` a]\x9A_9_Q\x90_R`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\"\x92\x91\x90aYZV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17=W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17a\x91\x90aZ\xCCV[\x90P\x80_\x03a\x17uWP\x90\x94\x90\x93P\x91PPV[_\x83Q`\x01a\x17\x84\x91\x90aY\xA3V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17\x9BWa\x17\x9BaM\x99V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x17\xC4W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x84Q`\x01a\x17\xD6\x91\x90aY\xA3V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17\xEDWa\x17\xEDaM\x99V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x18\x16W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_Q` a]\x9A_9_Q\x90_R\x82\x86Q\x81Q\x81\x10a\x18:Wa\x18:aX\x9AV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x82\x81\x86Q\x81Q\x81\x10a\x18nWa\x18naX\x9AV[` \x90\x81\x02\x91\x90\x91\x01\x01R_[\x85Q\x81\x10\x15a\x19\x0FW\x85\x81\x81Q\x81\x10a\x18\x96Wa\x18\x96aX\x9AV[` \x02` \x01\x01Q\x83\x82\x81Q\x81\x10a\x18\xB0Wa\x18\xB0aX\x9AV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x84\x81\x81Q\x81\x10a\x18\xE2Wa\x18\xE2aX\x9AV[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a\x18\xFCWa\x18\xFCaX\x9AV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x18{V[P\x90\x97\x90\x96P\x94PPPPPV[_`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x80\x15\x90a\x19OWP`\x01`\x01`\xA0\x1B\x03\x80\x83\x16_\x81\x81R`\x9A` R`@\x90 T\x90\x91\x16\x14[\x92\x91PPV[__\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA93>\xC8\x85\x85`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19\xA5\x92\x91\x90aYZV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\xC0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\xE4\x91\x90aYtV[\x90Pa\x19\xF2\x84\x84\x83_a6lV[\x94\x93PPPPV[a\x1A\x02a8@V[a\x11\xAA_a8\x9AV[\x82a\x1A\x15\x81a3\x1AV[a\x1A2W`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1A;\x84a\x19\x1DV[a\x1AXW`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x84\x84`@Qa\x1A\x93\x92\x91\x90aY,V[`@Q\x80\x91\x03\x90\xA2PPPPV[``_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1A\xBDWa\x1A\xBDaM\x99V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1A\xE6W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83Q\x81\x10\x15a\x1BoW`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x98` R`@\x81 \x85Q\x90\x91\x90\x86\x90\x84\x90\x81\x10a\x1B\"Wa\x1B\"aX\x9AV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ T\x82\x82\x81Q\x81\x10a\x1B\\Wa\x1B\\aX\x9AV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x1A\xEBV[P\x93\x92PPPV[`fT`\x02\x90`\x04\x90\x81\x16\x03a\x1B\xA0W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1B\xA8a&zV[\x85_[\x81\x81\x10\x15a\x1C;Wa\x1C3\x89\x89\x83\x81\x81\x10a\x1B\xC8Wa\x1B\xC8aX\x9AV[\x90P` \x02\x81\x01\x90a\x1B\xDA\x91\x90aZ\xE3V[a\x1B\xE3\x90aZ\xF7V[\x88\x88\x84\x81\x81\x10a\x1B\xF5Wa\x1B\xF5aX\x9AV[\x90P` \x02\x81\x01\x90a\x1C\x07\x91\x90aX\xCCV[\x88\x88\x86\x81\x81\x10a\x1C\x19Wa\x1C\x19aX\x9AV[\x90P` \x02\x01` \x81\x01\x90a\x1C.\x91\x90a[\x02V[a8\xEBV[`\x01\x01a\x1B\xABV[PPa\x1CG`\x01`\xC9UV[PPPPPPPV[a\x1CXaK7V[_\x82\x81R`\xA4` \x90\x81R`@\x91\x82\x90 \x82Q`\xE0\x81\x01\x84R\x81T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R`\x01\x83\x01T\x81\x16\x82\x85\x01R`\x02\x83\x01T\x16\x81\x85\x01R`\x03\x82\x01T``\x82\x01R`\x04\x82\x01Tc\xFF\xFF\xFF\xFF\x16`\x80\x82\x01R`\x05\x82\x01\x80T\x85Q\x81\x86\x02\x81\x01\x86\x01\x90\x96R\x80\x86R\x91\x94\x92\x93`\xA0\x86\x01\x93\x92\x90\x83\x01\x82\x82\x80\x15a\x1D\x06W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x1C\xE8W[PPPPP\x81R` \x01`\x06\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x1D\\W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x1DHW[PPPPP\x81RPP\x90P\x91\x90PV[``a\x1Dw3a!SV[\x90Pa\rl\x84\x84\x84a\"\xD0V[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16_\x90\x81R`\xA2` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x81R\x82\x82 \x83Q\x91\x82\x01\x90\x93R\x91T\x82R\x90a\rl\x90a=-V[``\x80\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\xDCWa\x1D\xDCaM\x99V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1E\x05W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E!Wa\x1E!aM\x99V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1EJW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\x9A` R`@\x81 T\x92\x93P\x91\x16\x90a\x1Eu\x86\x83\x87a&\xD3V[\x90P_[\x85Q\x81\x10\x15a -W_a\x1E\xA5\x87\x83\x81Q\x81\x10a\x1E\x98Wa\x1E\x98aX\x9AV[` \x02` \x01\x01Qa7\xA3V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xFE$:\x17\x89\x89\x85\x81Q\x81\x10a\x1E\xC9Wa\x1E\xC9aX\x9AV[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1E\xEE\x92\x91\x90aYZV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\tW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F-\x91\x90aZ\xCCV[\x85\x83\x81Q\x81\x10a\x1F?Wa\x1F?aX\x9AV[` \x02` \x01\x01\x81\x81RPP_`\xA2_\x8A`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _\x89\x85\x81Q\x81\x10a\x1F\x82Wa\x1F\x82aX\x9AV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80` \x01`@R\x90\x81_\x82\x01T\x81RPP\x90Pa \x06\x86\x84\x81Q\x81\x10a\x1F\xD4Wa\x1F\xD4aX\x9AV[` \x02` \x01\x01Q\x85\x85\x81Q\x81\x10a\x1F\xEEWa\x1F\xEEaX\x9AV[` \x02` \x01\x01Q\x83a8\"\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x87\x84\x81Q\x81\x10a \x18Wa \x18aX\x9AV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x1EyV[PPP[\x92P\x92\x90PV[_Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a VWP_T`\x01`\xFF\x90\x91\x16\x10[\x80a oWP0;\x15\x80\x15a oWP_T`\xFF\x16`\x01\x14[a \xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a \xF8W_\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a!\x01\x82a-\x94V[a!\n\x83a8\x9AV[\x80\x15a\x10\xC3W_\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPV[``a!]a&zV[a!f\x82a\x10;V[a!\x83W`@Qc\xA5\xC7\xC4E`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a!\x8C\x82a\x19\x1DV[\x15a!\xAAW`@Qc\x11\xCA35`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3`\x01`\x01`\xA0\x1B\x03\x83\x16\x14a\"`W`\x01`\x01`\xA0\x1B\x03\x80\x83\x16_\x90\x81R`\x9A` R`@\x90 T\x16a!\xDD\x81a3\x1AV[\x80a\"\x01WPa!\xEC\x81a\x10\x1BV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14[a\"\x1EW`@Qc\x1EI\x9A#`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xF0\xED\xDF\x07\xE6\xEA\x14\xF3\x88\xB4~\x1E\x94\xA0\xF4d\xEC\xBD\x9E\xEDAq\x13\x0E\x0F\xC0\xE9\x9F\xB4\x03\n\x8A`@Q`@Q\x80\x91\x03\x90\xA3P[a\"i\x82a=LV[\x90Pa\"u`\x01`\xC9UV[\x91\x90PV[`fT`\x02\x90`\x04\x90\x81\x16\x03a\"\xA3W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\"\xABa&zV[a\"\xBFa\"\xB7\x86aZ\xF7V[\x85\x85\x85a8\xEBV[a\"\xC9`\x01`\xC9UV[PPPPPV[a\"\xD8a&zV[a\"\xE13a\x10;V[\x15a\"\xFFW`@Qc;\xF2\xB5\x03`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a#\x08\x83a\x19\x1DV[a#%W`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a#13\x84\x84\x84a?\xABV[a\x10\xB93\x84a.QV[``_\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15a#WWa#WaM\x99V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a#\x8AW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a#uW\x90P[P\x90P_[\x84Q\x81\x10\x15a\x1BoWa#\xBB\x85\x82\x81Q\x81\x10a#\xADWa#\xADaX\x9AV[` \x02` \x01\x01Q\x85a\x1A\xA1V[\x82\x82\x81Q\x81\x10a#\xCDWa#\xCDaX\x9AV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a#\x8FV[a#\xE8a8@V[`\x01`\x01`\xA0\x1B\x03\x81\x16a$MW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a \xCEV[a$V\x81a8\x9AV[PV[`@\x80Q\x80\x82\x01\x90\x91R`\n\x81Ri\"\xB4\xB3\xB2\xB7&0\xBC\xB2\xB9`\xB1\x1B` \x90\x91\x01R_\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x7Fq\xB6%\xCF\xADD\xBA\xC6;\x13\xDB\xA0\x7F.\x1D`\x84\xEE\x04\xB6\xF8u!\x01\xEC\xE6\x12mXN\xE6\xEAa$\xC6a@jV[\x80Q` \x91\x82\x01 `@\x80Q\x92\x83\x01\x94\x90\x94R\x92\x81\x01\x91\x90\x91R``\x81\x01\x91\x90\x91RF`\x80\x82\x01R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%nW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%\x92\x91\x90a[\x1DV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a%\xC3W`@QcyH!\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`fT\x80\x19\x82\x19\x81\x16\x14a%\xEAW`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`f\x82\x90U`@Q\x82\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01[`@Q\x80\x91\x03\x90\xA2PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\xA3` R`@\x90 ``\x90a\x19O\x90aA\x06V[_a&Ua$YV[`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x91\x90\x91R`B\x81\x01\x83\x90R`b\x01a\x11\xBEV[`\x02`\xC9T\x03a&\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a \xCEV[`\x02`\xC9UV[``_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a&\xEFWa&\xEFaM\x99V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a'\x18W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cTz\xFB\x87\x86\x86`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a'j\x92\x91\x90a[8V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'\x84W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra'\xAB\x91\x90\x81\x01\x90a[[V[\x90P_[\x84Q\x81\x10\x15a(\x1AWa'\xF5\x87\x86\x83\x81Q\x81\x10a'\xCEWa'\xCEaX\x9AV[` \x02` \x01\x01Q\x84\x84\x81Q\x81\x10a'\xE8Wa'\xE8aX\x9AV[` \x02` \x01\x01Qa0\xF2V[\x83\x82\x81Q\x81\x10a(\x07Wa(\x07aX\x9AV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a'\xAFV[P\x90\x95\x94PPPPPV[_`\x01`\x01`\xA0\x1B\x03\x86\x16a(MW`@Qc9\xB1\x90\xBB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83Q_\x03a(nW`@Qcyl\xC5%`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84Q`\x01`\x01`@\x1B\x03\x81\x11\x15a(\x88Wa(\x88aM\x99V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a(\xB1W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x85Q`\x01`\x01`@\x1B\x03\x81\x11\x15a(\xCEWa(\xCEaM\x99V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a(\xF7W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x86Q\x81\x10\x15a+\xC7W_a)\x1B\x88\x83\x81Q\x81\x10a\x1E\x98Wa\x1E\x98aX\x9AV[\x90P_`\xA2_\x8C`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _\x8A\x85\x81Q\x81\x10a)TWa)TaX\x9AV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80` \x01`@R\x90\x81_\x82\x01T\x81RPP\x90Pa)\xC0\x88\x84\x81Q\x81\x10a)\xA6Wa)\xA6aX\x9AV[` \x02` \x01\x01Q\x88\x85\x81Q\x81\x10a\x1F\xEEWa\x1F\xEEaX\x9AV[\x84\x84\x81Q\x81\x10a)\xD2Wa)\xD2aX\x9AV[` \x02` \x01\x01\x81\x81RPPa*\n\x88\x84\x81Q\x81\x10a)\xF3Wa)\xF3aX\x9AV[` \x02` \x01\x01Q\x82aA\x12\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x85\x84\x81Q\x81\x10a*\x1CWa*\x1CaX\x9AV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x15a*\xB1Wa*s\x8A\x8A\x85\x81Q\x81\x10a*LWa*LaX\x9AV[` \x02` \x01\x01Q\x87\x86\x81Q\x81\x10a*fWa*faX\x9AV[` \x02` \x01\x01QaA&V[a*\xB1\x8A\x8C\x8B\x86\x81Q\x81\x10a*\x8AWa*\x8AaX\x9AV[` \x02` \x01\x01Q\x87\x87\x81Q\x81\x10a*\xA4Wa*\xA4aX\x9AV[` \x02` \x01\x01Qa7)V[_\x82`\x01`\x01`\xA0\x1B\x03\x16crJ\xF4#\x8D\x8C\x87\x81Q\x81\x10a*\xD4Wa*\xD4aX\x9AV[` \x02` \x01\x01Q\x8C\x88\x81Q\x81\x10a*\xEEWa*\xEEaX\x9AV[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a+\x14\x93\x92\x91\x90a[\xEAV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a+0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+T\x91\x90aZ\xCCV[\x90P\x80_\x03a+\xB9W`\x01`\x01`\xA0\x1B\x03\x8C\x16_\x90\x81R`\xA2` R`@\x81 \x8BQa+\xB9\x92\x90\x8D\x90\x88\x90\x81\x10a+\x8DWa+\x8DaX\x9AV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _\x90UV[PPP\x80`\x01\x01\x90Pa(\xFCV[P`\x01`\x01`\xA0\x1B\x03\x88\x16_\x90\x81R`\x9F` R`@\x81 \x80T\x91\x82\x91\x90a+\xEE\x83a\\\x0EV[\x91\x90PUP_`@Q\x80`\xE0\x01`@R\x80\x8B`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8A`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8B`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x89\x81R` \x01\x85\x81RP\x90P_a,T\x82a\x11\xACV[_\x81\x81R`\x9E` \x90\x81R`@\x80\x83 \x80T`\x01`\xFF\x19\x90\x91\x16\x81\x17\x90\x91U`\xA4\x83R\x92\x81\x90 \x86Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x83U\x88\x85\x01Q\x95\x83\x01\x80T\x82\x16\x96\x83\x16\x96\x90\x96\x17\x90\x95U\x91\x87\x01Q`\x02\x82\x01\x80T\x90\x95\x16\x92\x16\x91\x90\x91\x17\x90\x92U``\x85\x01Q`\x03\x83\x01U`\x80\x85\x01Q`\x04\x83\x01\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U`\xA0\x85\x01Q\x80Q\x93\x94P\x85\x93a-\n\x92`\x05\x85\x01\x92\x01\x90aK\x90V[P`\xC0\x82\x01Q\x80Qa-&\x91`\x06\x84\x01\x91` \x90\x91\x01\x90aK\xF3V[PPP`\x01`\x01`\xA0\x1B\x03\x8B\x16_\x90\x81R`\xA3` R`@\x90 a-J\x90\x82aA\x90V[P\x7F&\xB2\xAA\xE2e\x16\xE8q\x9E\xF5\x0E\xA2\xF6\x83\x1A.\xFB\xD4\xE3}\xCC\xDF\x0Fi6\xB2{\xC0\x8Ey>0\x81\x83\x86`@Qa-~\x93\x92\x91\x90a\\&V[`@Q\x80\x91\x03\x90\xA1\x9A\x99PPPPPPPPPPV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2PV[_a\x19\xF2\x82a-\xE9a-\xE2\x87a=-V[\x86\x90aA\x9BV[\x90aA\x9BV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x81\x81R`\x99` R`@\x90\x81\x90 `\x01\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x93\x85\x16\x93\x90\x93\x17\x90\x92U\x90Q\x7Fw;T\xC0Muo\xCC^g\x81\x11\xF7\xD70\xDE;\xE9\x81\x92\0\x07\x99\xEE\xE3\xD67\x16\x05Z\x87\xC6\x90a&\x1D\x90\x84\x90aP\x85V[`fT_\x90`\x01\x90\x81\x16\x03a.yW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__a.\x84\x85a\x160V[\x91P\x91P_a.\x94_\x86\x85a&\xD3V[`\x01`\x01`\xA0\x1B\x03\x87\x81\x16_\x81\x81R`\x9A` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x94\x8B\x16\x94\x85\x17\x90UQ\x93\x94P\x91\x92\x90\x91\x7F\xC3\xEE\x9F._\xDA\x98\xE8\x06j\x1Ft[-\xF9(_Ao\xE9\x8C\xF2U\x9C\xD2\x14\x84\xB3\xD8t3\x04\x91\xA3_[\x83Q\x81\x10\x15a\x1CGW_Q` a]\x9A_9_Q\x90_R`\x01`\x01`\xA0\x1B\x03\x16\x84\x82\x81Q\x81\x10a/ Wa/ aX\x9AV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x03a0\x91W`@Qc\xA3\xD7^\t`\xE0\x1B\x81R_\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA3\xD7^\t\x90a/\x84\x90\x8B\x90`\x04\x01aP\x85V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/\x9FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a/\xC3\x91\x90aYtV[\x90P_`\xA2_\x8A`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _\x87\x85\x81Q\x81\x10a/\xFCWa/\xFCaX\x9AV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80` \x01`@R\x90\x81_\x82\x01T\x81RPP\x90Pa0p\x85\x84\x81Q\x81\x10a0NWa0NaX\x9AV[` \x02` \x01\x01Q\x83`\x01`\x01`@\x1B\x03\x16\x83a8\"\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x85\x84\x81Q\x81\x10a0\x82Wa0\x82aX\x9AV[` \x02` \x01\x01\x81\x81RPPPP[a0\xEA\x86\x88\x86\x84\x81Q\x81\x10a0\xA8Wa0\xA8aX\x9AV[` \x02` \x01\x01Q_\x87\x86\x81Q\x81\x10a0\xC3Wa0\xC3aX\x9AV[` \x02` \x01\x01Q\x87\x87\x81Q\x81\x10a0\xDDWa0\xDDaX\x9AV[` \x02` \x01\x01Qa1\xD5V[`\x01\x01a.\xEEV[_s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xBF\x19`\x01`\x01`\xA0\x1B\x03\x84\x16\x01a1\xC5W`@Qc\xA3\xD7^\t`\xE0\x1B\x81R_\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA3\xD7^\t\x90a1f\x90\x88\x90`\x04\x01aP\x85V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a1\x81W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a1\xA5\x91\x90aYtV[\x90Pa1\xBD`\x01`\x01`@\x1B\x03\x84\x81\x16\x90\x83\x16a8\x0EV[\x91PPa\rlV[P`\x01`\x01`@\x1B\x03\x16\x92\x91PPV[\x80_\x03a1\xF5W`@Qc\n3\xBCi`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x15a3\x12W`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\xA2` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R a2+\x81\x85\x85\x85aA\xAFV[`@\x80Q` \x81\x01\x90\x91R\x81T\x81R\x7F\x8B\xE92\xBA\xC5Ea\xF2r`\xF9Tc\xD9\xB8\xAB7\xE0k(B\xE5\xEE$\x04\x15|\xC1=\xF6\xEB\x8F\x90\x87\x90\x87\x90a2i\x90a=-V[`@Qa2x\x93\x92\x91\x90a[\xEAV[`@Q\x80\x91\x03\x90\xA1a2\x89\x86a\x10;V[\x15a\x1CGW`\x01`\x01`\xA0\x1B\x03\x80\x88\x16_\x90\x81R`\x98` \x90\x81R`@\x80\x83 \x93\x89\x16\x83R\x92\x90R\x90\x81 \x80T\x85\x92\x90a2\xC4\x90\x84\x90aY\xA3V[\x92PP\x81\x90UP\x86`\x01`\x01`\xA0\x1B\x03\x16\x7F\x1E\xC0B\xC9e\xE2\xED\xD7\x10{Q\x18\x8E\xE0\xF3\x83\xE2.v\x17\x90A\xAB:\x9D\x18\xFF\x15\x14\x05\x16l\x87\x87\x86`@Qa3\x08\x93\x92\x91\x90a[\xEAV[`@Q\x80\x91\x03\x90\xA2P[PPPPPPV[`@Qc\x1B\xEB+\x97`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R3`$\x83\x01R0`D\x83\x01R_\x805`\x01`\x01`\xE0\x1B\x03\x19\x16`d\x84\x01R\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xDFY\\\xB8\x90`\x84\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a3\xA0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19O\x91\x90aY\x11V[``_a3\xD0\x83aB*V[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P_\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[a4\taK7V[_\x82\x81R`\xA4` \x90\x81R`@\x91\x82\x90 \x82Q`\xE0\x81\x01\x84R\x81T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R`\x01\x83\x01T\x81\x16\x82\x85\x01R`\x02\x83\x01T\x16\x81\x85\x01R`\x03\x82\x01T``\x82\x81\x01\x91\x90\x91R`\x04\x83\x01Tc\xFF\xFF\xFF\xFF\x16`\x80\x83\x01R`\x05\x83\x01\x80T\x86Q\x81\x87\x02\x81\x01\x87\x01\x90\x97R\x80\x87R\x91\x95\x92\x94`\xA0\x86\x01\x93\x92\x90\x83\x01\x82\x82\x80\x15a4\xBBW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a4\x9DW[PPPPP\x81R` \x01`\x06\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a5\x11W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a4\xFDW[PPPPP\x81RPP\x91P\x81`\xA0\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a59Wa59aM\x99V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a5bW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83`\x80\x01Qa5\x96\x91\x90a\\PV[\x90P_Cc\xFF\xFF\xFF\xFF\x16\x82c\xFF\xFF\xFF\xFF\x16\x10a5\xC7Wa5\xC2\x84_\x01Q\x85` \x01Q\x86`\xA0\x01Qa&\xD3V[a5\xDEV[a5\xDE\x84_\x01Q\x85` \x01Q\x86`\xA0\x01Q\x85aBQV[\x90P_[\x84`\xA0\x01QQ\x81\x10\x15a\x13\x1AWa6/\x85`\xC0\x01Q\x82\x81Q\x81\x10a6\x08Wa6\x08aX\x9AV[` \x02` \x01\x01Q\x83\x83\x81Q\x81\x10a6\"Wa6\"aX\x9AV[` \x02` \x01\x01QaC\x7FV[\x84\x82\x81Q\x81\x10a6AWa6AaX\x9AV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a5\xE2V[_a6b\x84\x83\x85`\x01aC\x8AV[a\x19\xF2\x90\x85a\\lV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16_\x90\x81R`\xA5` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R\x90\x81 \x81\x90a6\x9D\x90aC\xE5V[\x90P_a7\x03`\x01a6\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Ca\\\x7FV[a6\xD9\x91\x90a\\\x7FV[`\x01`\x01`\xA0\x1B\x03\x80\x8A\x16_\x90\x81R`\xA5` \x90\x81R`@\x80\x83 \x93\x8C\x16\x83R\x92\x90R \x90aC\xFFV[\x90P_a7\x10\x82\x84a\\lV[\x90Pa7\x1D\x81\x87\x87aD\x1BV[\x98\x97PPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16_\x90\x81R`\x98` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a7_\x90\x84\x90a\\lV[\x92PP\x81\x90UP\x83`\x01`\x01`\xA0\x1B\x03\x16\x7Fi\t`\x007\xB7]{G3\xAE\xDD\x81TB\xB5\xEC\x01\x8A\x82wQ\xC82\xAA\xFFd\xEB\xA5\xD6\xD2\xDD\x84\x84\x84`@Qa\x1A\x93\x93\x92\x91\x90a[\xEAV[_`\x01`\x01`\xA0\x1B\x03\x82\x16_Q` a]\x9A_9_Q\x90_R\x14a7\xE7W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x19OV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x91PPV[_a\rl\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0aD9V[_a\x19\xF2\x82a8:a83\x87a=-V[\x86\x90a8\x0EV[\x90a8\x0EV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x11\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a \xCEV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPV[`\xA0\x84\x01QQ\x82\x14a9\x10W`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83`@\x01Q`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a9FW`@Qc\x16\x11\r5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a9P\x85a\x11\xACV[_\x81\x81R`\x9E` R`@\x90 T\x90\x91P`\xFF\x16a9\x81W`@Qc\x87\xC9\xD2\x19`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87`\x80\x01Qa9\xB4\x91\x90a\\PV[\x90P\x80c\xFF\xFF\xFF\xFF\x16Cc\xFF\xFF\xFF\xFF\x16\x11a9\xE2W`@Qcx\xF6z\xE1`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a9\xF9\x87_\x01Q\x88` \x01Q\x89`\xA0\x01Q\x84aBQV[\x87Q`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\xA3` R`@\x90 \x90\x92Pa:\x1F\x91P\x83aE\x1EV[P_\x82\x81R`\xA4` R`@\x81 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x82U`\x01\x82\x01\x80T\x82\x16\x90U`\x02\x82\x01\x80T\x90\x91\x16\x90U`\x03\x81\x01\x82\x90U`\x04\x81\x01\x80Tc\xFF\xFF\xFF\xFF\x19\x16\x90U\x90a:v`\x05\x83\x01\x82aL,V[a:\x83`\x06\x83\x01_aL,V[PP_\x82\x81R`\x9E` R`@\x90\x81\x90 \x80T`\xFF\x19\x16\x90UQ\x7F\x1F@@\x08\x89'N\xD0{$\x84^PT\xA8z\x0C\xAB\x96\x9E\xB1'z\xAF\xE6\x1A\xE3R\xE7\xC3*\0\x90a:\xCC\x90\x84\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1\x85Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`\x9A` R`@\x81 T\x88Q`\xA0\x8A\x01Q\x91\x90\x93\x16\x92a;\x06\x91\x84\x90a&\xD3V[\x90P_[\x88`\xA0\x01QQ\x81\x10\x15a=\"W_a;1\x8A`\xA0\x01Q\x83\x81Q\x81\x10a\x1E\x98Wa\x1E\x98aX\x9AV[\x90P_a;g\x8B`\xC0\x01Q\x84\x81Q\x81\x10a;MWa;MaX\x9AV[` \x02` \x01\x01Q\x87\x85\x81Q\x81\x10a6\"Wa6\"aX\x9AV[\x90P\x80_\x03a;wWPPa=\x1AV[\x87\x15a<EW\x81`\x01`\x01`\xA0\x1B\x03\x16c.\xAEA\x8C\x8C_\x01Q\x8D`\xA0\x01Q\x86\x81Q\x81\x10a;\xA6Wa;\xA6aX\x9AV[` \x02` \x01\x01Q\x8D\x8D\x88\x81\x81\x10a;\xC0Wa;\xC0aX\x9AV[\x90P` \x02\x01` \x81\x01\x90a;\xD5\x91\x90aP\xDCV[`@Q`\xE0\x85\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x91\x83\x16`$\x83\x01R\x90\x91\x16`D\x82\x01R`d\x81\x01\x84\x90R`\x84\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a<*W__\xFD[PZ\xF1\x15\x80\x15a<<W=__>=_\xFD[PPPPa=\x17V[__\x83`\x01`\x01`\xA0\x1B\x03\x16cP\xFFr%\x8E_\x01Q\x8F`\xA0\x01Q\x88\x81Q\x81\x10a<pWa<paX\x9AV[` \x02` \x01\x01Q\x86`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a<\x97\x93\x92\x91\x90a[\xEAV[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a<\xB2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<\xD6\x91\x90a\\\x9BV[\x91P\x91Pa=\x14\x87\x8E_\x01Q\x8F`\xA0\x01Q\x88\x81Q\x81\x10a<\xF8Wa<\xF8aX\x9AV[` \x02` \x01\x01Q\x85\x85\x8B\x8B\x81Q\x81\x10a0\xDDWa0\xDDaX\x9AV[PP[PP[`\x01\x01a;\nV[PPPPPPPPPV[\x80Q_\x90\x15a==W\x81Qa\x19OV[g\r\xE0\xB6\xB3\xA7d\0\0\x92\x91PPV[`fT``\x90`\x01\x90`\x02\x90\x81\x16\x03a=xW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16_\x81\x81R`\x9A` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U\x90Q\x93\x16\x92\x83\x92\x91\x7F\xFE\xE3\tf\xA2V\xB7\x1E\x14\xBC\x0E\xBF\xC9C\x15\xE2\x8E\xF4\xA9zq1\xA9\xE2\xB7\xA3\x10\xA7:\xF4Fv\x91\xA3__a=\xD7\x86a\x160V[\x91P\x91P\x81Q_\x03a=\xEBWPPPa?\xA5V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a>\x04Wa>\x04aM\x99V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a>-W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x94P_a><\x87\x85\x85a&\xD3V[\x90P_[\x83Q\x81\x10\x15a?\x9FW`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91` \x80\x83\x01\x90\x806\x837PP`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x92\x93P_\x92\x91P` \x80\x83\x01\x90\x806\x837PP`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x92\x93P_\x92\x91P` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x86\x84\x81Q\x81\x10a>\xC0Wa>\xC0aX\x9AV[` \x02` \x01\x01Q\x83_\x81Q\x81\x10a>\xDAWa>\xDAaX\x9AV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x85\x84\x81Q\x81\x10a?\x0CWa?\x0CaX\x9AV[` \x02` \x01\x01Q\x82_\x81Q\x81\x10a?&Wa?&aX\x9AV[` \x02` \x01\x01\x81\x81RPP\x84\x84\x81Q\x81\x10a?DWa?DaX\x9AV[` \x02` \x01\x01Q\x81_\x81Q\x81\x10a?^Wa?^aX\x9AV[` \x02` \x01\x01\x81\x81RPPa?w\x8B\x89\x85\x85\x85a(%V[\x8A\x85\x81Q\x81\x10a?\x89Wa?\x89aX\x9AV[` \x90\x81\x02\x91\x90\x91\x01\x01RPPP`\x01\x01a>@V[PPPPP[P\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16_\x90\x81R`\x99` R`@\x90 `\x01\x01T\x16\x80a?\xD3WPa\x0E\xC4V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x9C` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x90 T`\xFF\x16\x15a@\x17W`@Qc\rLL\x91`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x9C` \x90\x81R`@\x80\x83 \x85\x84R\x82R\x90\x91 \x80T`\xFF\x19\x16`\x01\x17\x90U\x83\x01Qa\"\xC9\x90\x82\x90a@^\x90\x88\x90\x88\x90\x84\x90\x88\x90a\x08 V[\x85Q` \x87\x01QaE)V[``_a@\x96\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a3\xC4V[\x90P\x80_\x81Q\x81\x10a@\xAAWa@\xAAaX\x9AV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x81`\x01\x81Q\x81\x10a@\xC8Wa@\xC8aX\x9AV[\x01` \x90\x81\x01Q`@Q`\x01`\x01`\xF8\x1B\x03\x19\x93\x84\x16\x92\x81\x01\x92\x90\x92R\x91\x90\x91\x16`!\x82\x01R`\"\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x90V[``_a\rl\x83aE{V[_a\rlaA\x1F\x84a=-V[\x83\x90a8\x0EV[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16_\x90\x81R`\xA5` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 aAU\x90aC\xE5V[\x90Pa\x0E\xC4CaAe\x84\x84aY\xA3V[`\x01`\x01`\xA0\x1B\x03\x80\x88\x16_\x90\x81R`\xA5` \x90\x81R`@\x80\x83 \x93\x8A\x16\x83R\x92\x90R \x91\x90aE\xD4V[_a\rl\x83\x83aE\xDFV[_a\rl\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84aD9V[\x82_\x03aA\xDBW`@\x80Q` \x81\x01\x90\x91R\x84T\x81RaA\xD4\x90\x82\x90a-\xE9\x90a=-V[\x84Ua\x0E\xC4V[`@\x80Q` \x81\x01\x90\x91R\x84T\x81R_\x90aA\xF7\x90\x85\x84a8\"V[\x90P_aB\x04\x84\x83aY\xA3V[\x90P_aB\x1F\x84a-\xE9aB\x18\x88\x8AaY\xA3V[\x85\x90aA\x9BV[\x87UPPPPPPPV[_`\xFF\x82\x16`\x1F\x81\x11\x15a\x19OW`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``_\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15aBmWaBmaM\x99V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15aB\x96W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x94\xD7\xD0\x0C\x87\x87\x87`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aB\xEA\x93\x92\x91\x90a\\\xBDV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aC\x04W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@RaC+\x91\x90\x81\x01\x90a[[V[\x90P_[\x85Q\x81\x10\x15aCsWaCN\x88\x87\x83\x81Q\x81\x10a'\xCEWa'\xCEaX\x9AV[\x83\x82\x81Q\x81\x10aC`WaC`aX\x9AV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01aC/V[P\x90\x96\x95PPPPPPV[_a\rl\x83\x83a8\x0EV[__aC\x97\x86\x86\x86aD9V[\x90P`\x01\x83`\x02\x81\x11\x15aC\xADWaC\xADa\\\xF6V[\x14\x80\x15aC\xC9WP_\x84\x80aC\xC4WaC\xC4a]\nV[\x86\x88\t\x11[\x15aC\xDCWaC\xD9`\x01\x82aY\xA3V[\x90P[\x95\x94PPPPPV[_aC\xF0\x82\x82aF+V[`\x01`\x01`\xE0\x1B\x03\x16\x92\x91PPV[_aD\x0B\x83\x83\x83aFoV[`\x01`\x01`\xE0\x1B\x03\x16\x93\x92PPPV[_a\x19\xF2aD)\x83\x85a]\x1EV[\x85\x90`\x01`\x01`@\x1B\x03\x16a8\x0EV[_\x80\x80_\x19\x85\x87\t\x85\x87\x02\x92P\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03aDpW\x83\x82\x81aDfWaDfa]\nV[\x04\x92PPPa\rlV[\x80\x84\x11aD\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtMath: mulDiv overflow`X\x1B`D\x82\x01R`d\x01a \xCEV[_\x84\x86\x88\t`\x02`\x01\x87\x19\x81\x01\x88\x16\x97\x88\x90\x04`\x03\x81\x02\x83\x18\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x90\x81\x02\x90\x92\x03\x90\x91\x02_\x88\x90\x03\x88\x90\x04\x90\x91\x01\x85\x83\x11\x90\x94\x03\x93\x90\x93\x02\x93\x03\x94\x90\x94\x04\x91\x90\x91\x17\x02\x94\x93PPPPV[_a\rl\x83\x83aF\xB7V[B\x81\x10\x15aEJW`@Qc\x08\x19\xBD\xCD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[aE^`\x01`\x01`\xA0\x1B\x03\x85\x16\x84\x84aG\x9AV[a\x0E\xC4W`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x81_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15aE\xC8W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11aE\xB4W[PPPPP\x90P\x91\x90PV[a\x10\xC3\x83\x83\x83aG\xEEV[_\x81\x81R`\x01\x83\x01` R`@\x81 TaF$WP\x81T`\x01\x81\x81\x01\x84U_\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x19OV[P_a\x19OV[\x81T_\x90\x80\x15aFgWaFQ\x84aFD`\x01\x84a\\lV[_\x91\x82R` \x90\x91 \x01\x90V[T`\x01` \x1B\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x19\xF2V[P\x90\x92\x91PPV[\x82T_\x90\x81aF\x80\x86\x86\x83\x85aH\xF1V[\x90P\x80\x15aF\xADWaF\x97\x86aFD`\x01\x84a\\lV[T`\x01` \x1B\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x08\x9EV[P\x91\x94\x93PPPPV[_\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15aG\x91W_aF\xD9`\x01\x83a\\lV[\x85T\x90\x91P_\x90aF\xEC\x90`\x01\x90a\\lV[\x90P\x81\x81\x14aGKW_\x86_\x01\x82\x81T\x81\x10aG\nWaG\naX\x9AV[\x90_R` _ \x01T\x90P\x80\x87_\x01\x84\x81T\x81\x10aG*WaG*aX\x9AV[_\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80aG\\WaG\\a]=V[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90U\x85`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x90U`\x01\x93PPPPa\x19OV[_\x91PPa\x19OV[___aG\xA7\x85\x85aIDV[\x90\x92P\x90P_\x81`\x04\x81\x11\x15aG\xBFWaG\xBFa\\\xF6V[\x14\x80\x15aG\xDDWP\x85`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14[\x80a\x08\x9EWPa\x08\x9E\x86\x86\x86aI\x83V[\x82T\x80\x15aH\xA4W_aH\x06\x85aFD`\x01\x85a\\lV[`@\x80Q\x80\x82\x01\x90\x91R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\x01` \x1B\x90\x92\x04`\x01`\x01`\xE0\x1B\x03\x16` \x84\x01R\x91\x92P\x90\x85\x16\x10\x15aHXW`@Qc\x15\x1B\x8E?`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Qc\xFF\xFF\xFF\xFF\x80\x86\x16\x91\x16\x03aH\xA2W\x82aHy\x86aFD`\x01\x86a\\lV[\x80T`\x01`\x01`\xE0\x1B\x03\x92\x90\x92\x16`\x01` \x1B\x02c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UPPPPPV[P[P`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x92\x83\x16\x81R`\x01`\x01`\xE0\x1B\x03\x91\x82\x16` \x80\x83\x01\x91\x82R\x85T`\x01\x81\x01\x87U_\x96\x87R\x95 \x91Q\x90Q\x90\x92\x16`\x01` \x1B\x02\x91\x90\x92\x16\x17\x91\x01UV[_[\x81\x83\x10\x15a\x1BoW_aI\x06\x84\x84aJjV[_\x87\x81R` \x90 \x90\x91Pc\xFF\xFF\xFF\xFF\x86\x16\x90\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15aI0W\x80\x92PaI>V[aI;\x81`\x01aY\xA3V[\x93P[PaH\xF3V[__\x82Q`A\x03aIxW` \x83\x01Q`@\x84\x01Q``\x85\x01Q_\x1AaIl\x87\x82\x85\x85aJ\x84V[\x94P\x94PPPPa 1V[P_\x90P`\x02a 1V[___\x85`\x01`\x01`\xA0\x1B\x03\x16c\x16&\xBA~`\xE0\x1B\x86\x86`@Q`$\x01aI\xAB\x92\x91\x90a]QV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90QaI\xE9\x91\x90a]iV[_`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80_\x81\x14aJ!W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>aJ&V[``\x91P[P\x91P\x91P\x81\x80\x15aJ:WP` \x81Q\x10\x15[\x80\x15a\x08\x9EWP\x80Qc\x0B\x13]?`\xE1\x1B\x90aJ_\x90\x83\x01` \x90\x81\x01\x90\x84\x01aZ\xCCV[\x14\x96\x95PPPPPPV[_aJx`\x02\x84\x84\x18a]zV[a\rl\x90\x84\x84\x16aY\xA3V[_\x80o\xA2\xA8\x91\x8C\xA8[\xAF\xE2 \x16\xD0\xB9\x97\xE4\xDF``\x01`\xFF\x1B\x03\x83\x11\x15aJ\xAFWP_\x90P`\x03aK.V[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aK\0W=__>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16aK(W_`\x01\x92P\x92PPaK.V[\x91P_\x90P[\x94P\x94\x92PPPV[`@Q\x80`\xE0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01``\x81R` \x01``\x81RP\x90V[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15aK\xE3W\x91` \x02\x82\x01[\x82\x81\x11\x15aK\xE3W\x82Q\x82T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x82U` \x90\x92\x01\x91`\x01\x90\x91\x01\x90aK\xAEV[PaK\xEF\x92\x91PaLCV[P\x90V[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15aK\xE3W\x91` \x02\x82\x01[\x82\x81\x11\x15aK\xE3W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90aL\x11V[P\x80T_\x82U\x90_R` _ \x90\x81\x01\x90a$V\x91\x90[[\x80\x82\x11\x15aK\xEFW_\x81U`\x01\x01aLDV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a$VW__\xFD[\x805a\"u\x81aLWV[_____`\xA0\x86\x88\x03\x12\x15aL\x8AW__\xFD[\x855aL\x95\x81aLWV[\x94P` \x86\x015aL\xA5\x81aLWV[\x93P`@\x86\x015aL\xB5\x81aLWV[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[__\x83`\x1F\x84\x01\x12aL\xDDW__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aL\xF3W__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a 1W__\xFD[__` \x83\x85\x03\x12\x15aM\x1EW__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aM3W__\xFD[aM?\x85\x82\x86\x01aL\xCDV[\x90\x96\x90\x95P\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a(\x1AW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aMdV[_` \x82\x84\x03\x12\x15aM\x92W__\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aM\xCFWaM\xCFaM\x99V[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aM\xCFWaM\xCFaM\x99V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aN\x1FWaN\x1FaM\x99V[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15aN?WaN?aM\x99V[P`\x05\x1B` \x01\x90V[_\x82`\x1F\x83\x01\x12aNXW__\xFD[\x815aNkaNf\x82aN'V[aM\xF7V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aN\x8CW__\xFD[` \x85\x01[\x83\x81\x10\x15aN\xB2W\x805aN\xA4\x81aLWV[\x83R` \x92\x83\x01\x92\x01aN\x91V[P\x95\x94PPPPPV[_\x82`\x1F\x83\x01\x12aN\xCBW__\xFD[\x815aN\xD9aNf\x82aN'V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aN\xFAW__\xFD[` \x85\x01[\x83\x81\x10\x15aN\xB2W\x805\x83R` \x92\x83\x01\x92\x01aN\xFFV[___``\x84\x86\x03\x12\x15aO)W__\xFD[\x835aO4\x81aLWV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aONW__\xFD[aOZ\x86\x82\x87\x01aNIV[\x92PP`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aOuW__\xFD[aO\x81\x86\x82\x87\x01aN\xBCV[\x91PP\x92P\x92P\x92V[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aO\xBBW\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aO\x9DV[P\x93\x94\x93PPPPV[` \x81R_a\rl` \x83\x01\x84aO\x8BV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\"uW__\xFD[__\x83`\x1F\x84\x01\x12aO\xFAW__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aP\x10W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a 1W__\xFD[____``\x85\x87\x03\x12\x15aP:W__\xFD[\x845aPE\x81aLWV[\x93PaPS` \x86\x01aO\xD7V[\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aPmW__\xFD[aPy\x87\x82\x88\x01aO\xEAV[\x95\x98\x94\x97P\x95PPPPV[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[____`\x80\x85\x87\x03\x12\x15aP\xACW__\xFD[\x845aP\xB7\x81aLWV[\x93P` \x85\x015aP\xC7\x81aLWV[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[_` \x82\x84\x03\x12\x15aP\xECW__\xFD[\x815a\rl\x81aLWV[__`@\x83\x85\x03\x12\x15aQ\x08W__\xFD[\x825aQ\x13\x81aLWV[\x91P` \x83\x015aQ#\x81aLWV[\x80\x91PP\x92P\x92\x90PV[_[\x83\x81\x10\x15aQHW\x81\x81\x01Q\x83\x82\x01R` \x01aQ0V[PP_\x91\x01RV[_\x81Q\x80\x84RaQg\x81` \x86\x01` \x86\x01aQ.V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R_a\rl` \x83\x01\x84aQPV[_`\xE0\x82\x84\x03\x12\x15aQ\x9DW__\xFD[aQ\xA5aM\xADV[\x90PaQ\xB0\x82aLkV[\x81RaQ\xBE` \x83\x01aLkV[` \x82\x01RaQ\xCF`@\x83\x01aLkV[`@\x82\x01R``\x82\x81\x015\x90\x82\x01RaQ\xEA`\x80\x83\x01aO\xD7V[`\x80\x82\x01R`\xA0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aR\x07W__\xFD[aR\x13\x84\x82\x85\x01aNIV[`\xA0\x83\x01RP`\xC0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aR1W__\xFD[aR=\x84\x82\x85\x01aN\xBCV[`\xC0\x83\x01RP\x92\x91PPV[_` \x82\x84\x03\x12\x15aRYW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aRnW__\xFD[a\x19\xF2\x84\x82\x85\x01aQ\x8DV[_` \x82\x84\x03\x12\x15aR\x8AW__\xFD[\x815`\xFF\x81\x16\x81\x14a\rlW__\xFD[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aO\xBBW\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aR\xACV[\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x80\x83\x01Q\x82\x16\x90\x84\x01R`@\x80\x83\x01Q\x90\x91\x16\x90\x83\x01R``\x80\x82\x01Q\x90\x83\x01R`\x80\x80\x82\x01Q_\x91aS\x1E\x90\x85\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P`\xA0\x82\x01Q`\xE0`\xA0\x85\x01RaS8`\xE0\x85\x01\x82aR\x9AV[\x90P`\xC0\x83\x01Q\x84\x82\x03`\xC0\x86\x01RaC\xDC\x82\x82aO\x8BV[`@\x81R_aSc`@\x83\x01\x85aR\xD3V[\x82\x81\x03` \x84\x01RaC\xDC\x81\x85aO\x8BV[_\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01_[\x83\x81\x10\x15aCsW`\x1F\x19\x85\x84\x03\x01\x88RaS\xAD\x83\x83QaO\x8BV[` \x98\x89\x01\x98\x90\x93P\x91\x90\x91\x01\x90`\x01\x01aS\x91V[_`@\x82\x01`@\x83R\x80\x85Q\x80\x83R``\x85\x01\x91P``\x81`\x05\x1B\x86\x01\x01\x92P` \x87\x01_[\x82\x81\x10\x15aT\x1AW`_\x19\x87\x86\x03\x01\x84RaT\x05\x85\x83QaR\xD3V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aS\xE9V[PPPP\x82\x81\x03` \x84\x01RaC\xDC\x81\x85aSuV[`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a$VW__\xFD[____`\x80\x85\x87\x03\x12\x15aTWW__\xFD[\x845aTb\x81aLWV[\x93P` \x85\x015aTr\x81aLWV[\x92P`@\x85\x015aT\x82\x81aT0V[\x91P``\x85\x015aT\x92\x81aT0V[\x93\x96\x92\x95P\x90\x93PPV[___``\x84\x86\x03\x12\x15aT\xAFW__\xFD[\x835aT\xBA\x81aLWV[\x92P` \x84\x015\x91P`@\x84\x015aT\xD1\x81aT0V[\x80\x91PP\x92P\x92P\x92V[`@\x81R_aSc`@\x83\x01\x85aR\x9AV[___`@\x84\x86\x03\x12\x15aU\0W__\xFD[\x835aU\x0B\x81aLWV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aU%W__\xFD[aU1\x86\x82\x87\x01aO\xEAV[\x94\x97\x90\x96P\x93\x94PPPPV[__`@\x83\x85\x03\x12\x15aUOW__\xFD[\x825aUZ\x81aLWV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aUtW__\xFD[aU\x80\x85\x82\x86\x01aNIV[\x91PP\x92P\x92\x90PV[______``\x87\x89\x03\x12\x15aU\x9FW__\xFD[\x865`\x01`\x01`@\x1B\x03\x81\x11\x15aU\xB4W__\xFD[aU\xC0\x89\x82\x8A\x01aL\xCDV[\x90\x97P\x95PP` \x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aU\xDEW__\xFD[aU\xEA\x89\x82\x8A\x01aL\xCDV[\x90\x95P\x93PP`@\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aV\x08W__\xFD[aV\x14\x89\x82\x8A\x01aL\xCDV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[` \x81R_a\rl` \x83\x01\x84aR\xD3V[___``\x84\x86\x03\x12\x15aVJW__\xFD[\x835aVU\x81aLWV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aVoW__\xFD[\x84\x01`@\x81\x87\x03\x12\x15aV\x80W__\xFD[aV\x88aM\xD5V[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aV\x9DW__\xFD[\x82\x01`\x1F\x81\x01\x88\x13aV\xADW__\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aV\xC6WaV\xC6aM\x99V[aV\xD9`\x1F\x82\x01`\x1F\x19\x16` \x01aM\xF7V[\x81\x81R\x89` \x83\x85\x01\x01\x11\x15aV\xEDW__\xFD[\x81` \x84\x01` \x83\x017_` \x92\x82\x01\x83\x01R\x83R\x92\x83\x015\x92\x82\x01\x92\x90\x92R\x93\x96\x93\x95PPPP`@\x91\x90\x91\x015\x90V[__`@\x83\x85\x03\x12\x15aW0W__\xFD[\x825aW;\x81aLWV[\x94` \x93\x90\x93\x015\x93PPPV[`@\x81R_aSc`@\x83\x01\x85aO\x8BV[\x80\x15\x15\x81\x14a$VW__\xFD[____``\x85\x87\x03\x12\x15aW{W__\xFD[\x845`\x01`\x01`@\x1B\x03\x81\x11\x15aW\x90W__\xFD[\x85\x01`\xE0\x81\x88\x03\x12\x15aW\xA1W__\xFD[\x93P` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aW\xBBW__\xFD[aW\xC7\x87\x82\x88\x01aL\xCDV[\x90\x94P\x92PP`@\x85\x015aT\x92\x81aW[V[__`@\x83\x85\x03\x12\x15aW\xECW__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aX\x01W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13aX\x11W__\xFD[\x805aX\x1FaNf\x82aN'V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15aX@W__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aXkW\x835aXZ\x81aLWV[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aXGV[\x94PPPP` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aUtW__\xFD[` \x81R_a\rl` \x83\x01\x84aSuV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x825`^\x19\x836\x03\x01\x81\x12aX\xC2W__\xFD[\x91\x90\x91\x01\x92\x91PPV[__\x835`\x1E\x19\x846\x03\x01\x81\x12aX\xE1W__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aX\xFAW__\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a 1W__\xFD[_` \x82\x84\x03\x12\x15aY!W__\xFD[\x81Qa\rl\x81aW[V[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017_\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x16` \x82\x01R`@\x01\x90V[_` \x82\x84\x03\x12\x15aY\x84W__\xFD[\x81Qa\rl\x81aT0V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x19OWa\x19OaY\x8FV[_\x82`\x1F\x83\x01\x12aY\xC5W__\xFD[\x81QaY\xD3aNf\x82aN'V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aY\xF4W__\xFD[` \x85\x01[\x83\x81\x10\x15aN\xB2W\x80Q\x83R` \x92\x83\x01\x92\x01aY\xF9V[__`@\x83\x85\x03\x12\x15aZ\"W__\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aZ7W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13aZGW__\xFD[\x80QaZUaNf\x82aN'V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15aZvW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aZ\xA1W\x83QaZ\x90\x81aLWV[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aZ}V[\x80\x95PPPPP` \x83\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aZ\xC0W__\xFD[aU\x80\x85\x82\x86\x01aY\xB6V[_` \x82\x84\x03\x12\x15aZ\xDCW__\xFD[PQ\x91\x90PV[_\x825`\xDE\x19\x836\x03\x01\x81\x12aX\xC2W__\xFD[_a\x19O6\x83aQ\x8DV[_` \x82\x84\x03\x12\x15a[\x12W__\xFD[\x815a\rl\x81aW[V[_` \x82\x84\x03\x12\x15a[-W__\xFD[\x81Qa\rl\x81aLWV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90a\x19\xF2\x90\x83\x01\x84aR\x9AV[_` \x82\x84\x03\x12\x15a[kW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a[\x80W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a[\x90W__\xFD[\x80Qa[\x9EaNf\x82aN'V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15a[\xBFW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a\x08\x9EW\x83Qa[\xD9\x81aT0V[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a[\xC6V[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[_`\x01\x82\x01a\\\x1FWa\\\x1FaY\x8FV[P`\x01\x01\x90V[\x83\x81R``` \x82\x01R_a\\>``\x83\x01\x85aR\xD3V[\x82\x81\x03`@\x84\x01Ra\x08\x9E\x81\x85aO\x8BV[c\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x19OWa\x19OaY\x8FV[\x81\x81\x03\x81\x81\x11\x15a\x19OWa\x19OaY\x8FV[c\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x19OWa\x19OaY\x8FV[__`@\x83\x85\x03\x12\x15a\\\xACW__\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R``` \x82\x01\x81\x90R_\x90a\\\xE0\x90\x83\x01\x85aR\x9AV[\x90Pc\xFF\xFF\xFF\xFF\x83\x16`@\x83\x01R\x94\x93PPPPV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[`\x01`\x01`@\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x19OWa\x19OaY\x8FV[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[\x82\x81R`@` \x82\x01R_a\x19\xF2`@\x83\x01\x84aQPV[_\x82QaX\xC2\x81\x84` \x87\x01aQ.V[_\x82a]\x94WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V\xFE\0\0\0\0\0\0\0\0\0\0\0\0\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\xA2dipfsX\"\x12 \xE1\xC0a;g\xB7\xD2\x91\xDB\xD5\x1E\x9B\x97\xADY&\x81\xC3\x91\xBC^\xA8\x83\xC6k\xC8r\x8E\xBB\xDE\xED\xD4dsolcC\0\x08\x1B\x003",
    );
    /**Custom error with signature `ActivelyDelegated()` and selector `0x77e56a06`.
```solidity
error ActivelyDelegated();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ActivelyDelegated {}
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
        impl ::core::convert::From<ActivelyDelegated> for UnderlyingRustTuple<'_> {
            fn from(value: ActivelyDelegated) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ActivelyDelegated {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ActivelyDelegated {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ActivelyDelegated()";
            const SELECTOR: [u8; 4] = [119u8, 229u8, 106u8, 6u8];
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
    /**Custom error with signature `CallerCannotUndelegate()` and selector `0x3c933446`.
```solidity
error CallerCannotUndelegate();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CallerCannotUndelegate {}
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
        impl ::core::convert::From<CallerCannotUndelegate> for UnderlyingRustTuple<'_> {
            fn from(value: CallerCannotUndelegate) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CallerCannotUndelegate {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CallerCannotUndelegate {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CallerCannotUndelegate()";
            const SELECTOR: [u8; 4] = [60u8, 147u8, 52u8, 70u8];
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
    /**Custom error with signature `FullySlashed()` and selector `0x28cef1a4`.
```solidity
error FullySlashed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FullySlashed {}
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
        impl ::core::convert::From<FullySlashed> for UnderlyingRustTuple<'_> {
            fn from(value: FullySlashed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FullySlashed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for FullySlashed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FullySlashed()";
            const SELECTOR: [u8; 4] = [40u8, 206u8, 241u8, 164u8];
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
    /**Custom error with signature `InputArrayLengthMismatch()` and selector `0x43714afd`.
```solidity
error InputArrayLengthMismatch();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InputArrayLengthMismatch {}
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
        impl ::core::convert::From<InputArrayLengthMismatch>
        for UnderlyingRustTuple<'_> {
            fn from(value: InputArrayLengthMismatch) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InputArrayLengthMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InputArrayLengthMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InputArrayLengthMismatch()";
            const SELECTOR: [u8; 4] = [67u8, 113u8, 74u8, 253u8];
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
    /**Custom error with signature `InputArrayLengthZero()` and selector `0x796cc525`.
```solidity
error InputArrayLengthZero();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InputArrayLengthZero {}
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
        impl ::core::convert::From<InputArrayLengthZero> for UnderlyingRustTuple<'_> {
            fn from(value: InputArrayLengthZero) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InputArrayLengthZero {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InputArrayLengthZero {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InputArrayLengthZero()";
            const SELECTOR: [u8; 4] = [121u8, 108u8, 197u8, 37u8];
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
    /**Custom error with signature `InvalidPermissions()` and selector `0x932d94f7`.
```solidity
error InvalidPermissions();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidPermissions {}
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
        impl ::core::convert::From<InvalidPermissions> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidPermissions) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidPermissions {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidPermissions {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidPermissions()";
            const SELECTOR: [u8; 4] = [147u8, 45u8, 148u8, 247u8];
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
    /**Custom error with signature `InvalidSnapshotOrdering()` and selector `0x2a371c7e`.
```solidity
error InvalidSnapshotOrdering();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidSnapshotOrdering {}
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
        impl ::core::convert::From<InvalidSnapshotOrdering> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidSnapshotOrdering) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidSnapshotOrdering {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidSnapshotOrdering {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidSnapshotOrdering()";
            const SELECTOR: [u8; 4] = [42u8, 55u8, 28u8, 126u8];
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
    /**Custom error with signature `NotActivelyDelegated()` and selector `0xa5c7c445`.
```solidity
error NotActivelyDelegated();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotActivelyDelegated {}
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
        impl ::core::convert::From<NotActivelyDelegated> for UnderlyingRustTuple<'_> {
            fn from(value: NotActivelyDelegated) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotActivelyDelegated {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotActivelyDelegated {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotActivelyDelegated()";
            const SELECTOR: [u8; 4] = [165u8, 199u8, 196u8, 69u8];
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
    /**Custom error with signature `OnlyEigenPodManager()` and selector `0xc84e9984`.
```solidity
error OnlyEigenPodManager();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyEigenPodManager {}
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
        impl ::core::convert::From<OnlyEigenPodManager> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyEigenPodManager) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyEigenPodManager {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyEigenPodManager {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyEigenPodManager()";
            const SELECTOR: [u8; 4] = [200u8, 78u8, 153u8, 132u8];
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
    /**Custom error with signature `OnlyStrategyManagerOrEigenPodManager()` and selector `0x11481a94`.
```solidity
error OnlyStrategyManagerOrEigenPodManager();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyStrategyManagerOrEigenPodManager {}
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
        impl ::core::convert::From<OnlyStrategyManagerOrEigenPodManager>
        for UnderlyingRustTuple<'_> {
            fn from(value: OnlyStrategyManagerOrEigenPodManager) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OnlyStrategyManagerOrEigenPodManager {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyStrategyManagerOrEigenPodManager {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyStrategyManagerOrEigenPodManager()";
            const SELECTOR: [u8; 4] = [17u8, 72u8, 26u8, 148u8];
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
    /**Custom error with signature `OperatorsCannotUndelegate()` and selector `0x8e5199a8`.
```solidity
error OperatorsCannotUndelegate();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorsCannotUndelegate {}
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
        impl ::core::convert::From<OperatorsCannotUndelegate>
        for UnderlyingRustTuple<'_> {
            fn from(value: OperatorsCannotUndelegate) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OperatorsCannotUndelegate {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OperatorsCannotUndelegate {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OperatorsCannotUndelegate()";
            const SELECTOR: [u8; 4] = [142u8, 81u8, 153u8, 168u8];
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
    /**Custom error with signature `SaltSpent()` and selector `0x35313244`.
```solidity
error SaltSpent();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SaltSpent {}
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
        impl ::core::convert::From<SaltSpent> for UnderlyingRustTuple<'_> {
            fn from(value: SaltSpent) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SaltSpent {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SaltSpent {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SaltSpent()";
            const SELECTOR: [u8; 4] = [53u8, 49u8, 50u8, 68u8];
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
    /**Custom error with signature `WithdrawalDelayNotElapsed()` and selector `0xf1ecf5c2`.
```solidity
error WithdrawalDelayNotElapsed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WithdrawalDelayNotElapsed {}
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
        impl ::core::convert::From<WithdrawalDelayNotElapsed>
        for UnderlyingRustTuple<'_> {
            fn from(value: WithdrawalDelayNotElapsed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for WithdrawalDelayNotElapsed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for WithdrawalDelayNotElapsed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "WithdrawalDelayNotElapsed()";
            const SELECTOR: [u8; 4] = [241u8, 236u8, 245u8, 194u8];
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
    /**Custom error with signature `WithdrawalNotQueued()` and selector `0x87c9d219`.
```solidity
error WithdrawalNotQueued();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WithdrawalNotQueued {}
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
        impl ::core::convert::From<WithdrawalNotQueued> for UnderlyingRustTuple<'_> {
            fn from(value: WithdrawalNotQueued) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for WithdrawalNotQueued {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for WithdrawalNotQueued {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "WithdrawalNotQueued()";
            const SELECTOR: [u8; 4] = [135u8, 201u8, 210u8, 25u8];
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
    /**Custom error with signature `WithdrawerNotCaller()` and selector `0x584434d4`.
```solidity
error WithdrawerNotCaller();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WithdrawerNotCaller {}
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
        impl ::core::convert::From<WithdrawerNotCaller> for UnderlyingRustTuple<'_> {
            fn from(value: WithdrawerNotCaller) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for WithdrawerNotCaller {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for WithdrawerNotCaller {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "WithdrawerNotCaller()";
            const SELECTOR: [u8; 4] = [88u8, 68u8, 52u8, 212u8];
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
    /**Event with signature `DelegationApproverUpdated(address,address)` and selector `0x773b54c04d756fcc5e678111f7d730de3be98192000799eee3d63716055a87c6`.
```solidity
event DelegationApproverUpdated(address indexed operator, address newDelegationApprover);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct DelegationApproverUpdated {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newDelegationApprover: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for DelegationApproverUpdated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "DelegationApproverUpdated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                119u8,
                59u8,
                84u8,
                192u8,
                77u8,
                117u8,
                111u8,
                204u8,
                94u8,
                103u8,
                129u8,
                17u8,
                247u8,
                215u8,
                48u8,
                222u8,
                59u8,
                233u8,
                129u8,
                146u8,
                0u8,
                7u8,
                153u8,
                238u8,
                227u8,
                214u8,
                55u8,
                22u8,
                5u8,
                90u8,
                135u8,
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
                    operator: topics.1,
                    newDelegationApprover: data.0,
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
                        &self.newDelegationApprover,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.operator.clone())
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
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for DelegationApproverUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&DelegationApproverUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &DelegationApproverUpdated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `DepositScalingFactorUpdated(address,address,uint256)` and selector `0x8be932bac54561f27260f95463d9b8ab37e06b2842e5ee2404157cc13df6eb8f`.
```solidity
event DepositScalingFactorUpdated(address staker, address strategy, uint256 newDepositScalingFactor);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct DepositScalingFactorUpdated {
        #[allow(missing_docs)]
        pub staker: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newDepositScalingFactor: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for DepositScalingFactorUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "DepositScalingFactorUpdated(address,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                139u8,
                233u8,
                50u8,
                186u8,
                197u8,
                69u8,
                97u8,
                242u8,
                114u8,
                96u8,
                249u8,
                84u8,
                99u8,
                217u8,
                184u8,
                171u8,
                55u8,
                224u8,
                107u8,
                40u8,
                66u8,
                229u8,
                238u8,
                36u8,
                4u8,
                21u8,
                124u8,
                193u8,
                61u8,
                246u8,
                235u8,
                143u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    staker: data.0,
                    strategy: data.1,
                    newDepositScalingFactor: data.2,
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
                        &self.staker,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.newDepositScalingFactor,
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
        impl alloy_sol_types::private::IntoLogData for DepositScalingFactorUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&DepositScalingFactorUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &DepositScalingFactorUpdated,
            ) -> alloy_sol_types::private::LogData {
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
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Initialized(uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                127u8,
                38u8,
                184u8,
                63u8,
                249u8,
                110u8,
                31u8,
                43u8,
                106u8,
                104u8,
                47u8,
                19u8,
                56u8,
                82u8,
                246u8,
                121u8,
                138u8,
                9u8,
                196u8,
                101u8,
                218u8,
                149u8,
                146u8,
                20u8,
                96u8,
                206u8,
                251u8,
                56u8,
                71u8,
                64u8,
                36u8,
                152u8,
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
                        8,
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
    /**Event with signature `OperatorMetadataURIUpdated(address,string)` and selector `0x02a919ed0e2acad1dd90f17ef2fa4ae5462ee1339170034a8531cca4b6708090`.
```solidity
event OperatorMetadataURIUpdated(address indexed operator, string metadataURI);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorMetadataURIUpdated {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub metadataURI: alloy::sol_types::private::String,
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
        impl alloy_sol_types::SolEvent for OperatorMetadataURIUpdated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::String,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorMetadataURIUpdated(address,string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                2u8,
                169u8,
                25u8,
                237u8,
                14u8,
                42u8,
                202u8,
                209u8,
                221u8,
                144u8,
                241u8,
                126u8,
                242u8,
                250u8,
                74u8,
                229u8,
                70u8,
                46u8,
                225u8,
                51u8,
                145u8,
                112u8,
                3u8,
                74u8,
                133u8,
                49u8,
                204u8,
                164u8,
                182u8,
                112u8,
                128u8,
                144u8,
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
                    metadataURI: data.0,
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
                        &self.metadataURI,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.operator.clone())
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
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorMetadataURIUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorMetadataURIUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &OperatorMetadataURIUpdated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorRegistered(address,address)` and selector `0xa453db612af59e5521d6ab9284dc3e2d06af286eb1b1b7b771fce4716c19f2c1`.
```solidity
event OperatorRegistered(address indexed operator, address delegationApprover);
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
        pub delegationApprover: alloy::sol_types::private::Address,
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
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorRegistered(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                164u8,
                83u8,
                219u8,
                97u8,
                42u8,
                245u8,
                158u8,
                85u8,
                33u8,
                214u8,
                171u8,
                146u8,
                132u8,
                220u8,
                62u8,
                45u8,
                6u8,
                175u8,
                40u8,
                110u8,
                177u8,
                177u8,
                183u8,
                183u8,
                113u8,
                252u8,
                228u8,
                113u8,
                108u8,
                25u8,
                242u8,
                193u8,
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
                    delegationApprover: data.0,
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
                        &self.delegationApprover,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.operator.clone())
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
                    &self.operator,
                );
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
    /**Event with signature `OperatorSharesDecreased(address,address,address,uint256)` and selector `0x6909600037b75d7b4733aedd815442b5ec018a827751c832aaff64eba5d6d2dd`.
```solidity
event OperatorSharesDecreased(address indexed operator, address staker, address strategy, uint256 shares);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorSharesDecreased {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub staker: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub shares: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for OperatorSharesDecreased {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorSharesDecreased(address,address,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                105u8,
                9u8,
                96u8,
                0u8,
                55u8,
                183u8,
                93u8,
                123u8,
                71u8,
                51u8,
                174u8,
                221u8,
                129u8,
                84u8,
                66u8,
                181u8,
                236u8,
                1u8,
                138u8,
                130u8,
                119u8,
                81u8,
                200u8,
                50u8,
                170u8,
                255u8,
                100u8,
                235u8,
                165u8,
                214u8,
                210u8,
                221u8,
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
                    staker: data.0,
                    strategy: data.1,
                    shares: data.2,
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
                        &self.staker,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.shares),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.operator.clone())
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
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorSharesDecreased {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorSharesDecreased> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &OperatorSharesDecreased,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorSharesIncreased(address,address,address,uint256)` and selector `0x1ec042c965e2edd7107b51188ee0f383e22e76179041ab3a9d18ff151405166c`.
```solidity
event OperatorSharesIncreased(address indexed operator, address staker, address strategy, uint256 shares);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorSharesIncreased {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub staker: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub shares: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for OperatorSharesIncreased {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorSharesIncreased(address,address,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                30u8,
                192u8,
                66u8,
                201u8,
                101u8,
                226u8,
                237u8,
                215u8,
                16u8,
                123u8,
                81u8,
                24u8,
                142u8,
                224u8,
                243u8,
                131u8,
                226u8,
                46u8,
                118u8,
                23u8,
                144u8,
                65u8,
                171u8,
                58u8,
                157u8,
                24u8,
                255u8,
                21u8,
                20u8,
                5u8,
                22u8,
                108u8,
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
                    staker: data.0,
                    strategy: data.1,
                    shares: data.2,
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
                        &self.staker,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.shares),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.operator.clone())
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
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorSharesIncreased {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorSharesIncreased> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &OperatorSharesIncreased,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorSharesSlashed(address,address,uint256)` and selector `0xdd611f4ef63f4385f1756c86ce1f1f389a9013ba6fa07daba8528291bc2d3c30`.
```solidity
event OperatorSharesSlashed(address indexed operator, address strategy, uint256 totalSlashedShares);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorSharesSlashed {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub totalSlashedShares: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for OperatorSharesSlashed {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorSharesSlashed(address,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                221u8,
                97u8,
                31u8,
                78u8,
                246u8,
                63u8,
                67u8,
                133u8,
                241u8,
                117u8,
                108u8,
                134u8,
                206u8,
                31u8,
                31u8,
                56u8,
                154u8,
                144u8,
                19u8,
                186u8,
                111u8,
                160u8,
                125u8,
                171u8,
                168u8,
                82u8,
                130u8,
                145u8,
                188u8,
                45u8,
                60u8,
                48u8,
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
                    strategy: data.0,
                    totalSlashedShares: data.1,
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
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalSlashedShares),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.operator.clone())
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
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorSharesSlashed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorSharesSlashed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorSharesSlashed) -> alloy_sol_types::private::LogData {
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
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferred(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                139u8,
                224u8,
                7u8,
                156u8,
                83u8,
                22u8,
                89u8,
                20u8,
                19u8,
                68u8,
                205u8,
                31u8,
                208u8,
                164u8,
                242u8,
                132u8,
                25u8,
                73u8,
                127u8,
                151u8,
                34u8,
                163u8,
                218u8,
                175u8,
                227u8,
                180u8,
                24u8,
                111u8,
                107u8,
                100u8,
                87u8,
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
                    previousOwner: topics.1,
                    newOwner: topics.2,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
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
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Paused(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                171u8,
                64u8,
                163u8,
                116u8,
                188u8,
                81u8,
                222u8,
                55u8,
                34u8,
                0u8,
                168u8,
                188u8,
                152u8,
                26u8,
                248u8,
                201u8,
                236u8,
                220u8,
                8u8,
                223u8,
                218u8,
                239u8,
                11u8,
                182u8,
                224u8,
                159u8,
                136u8,
                243u8,
                198u8,
                22u8,
                239u8,
                61u8,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.newPausedStatus),
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
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
    /**Event with signature `SlashingWithdrawalCompleted(bytes32)` and selector `0x1f40400889274ed07b24845e5054a87a0cab969eb1277aafe61ae352e7c32a00`.
```solidity
event SlashingWithdrawalCompleted(bytes32 withdrawalRoot);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SlashingWithdrawalCompleted {
        #[allow(missing_docs)]
        pub withdrawalRoot: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for SlashingWithdrawalCompleted {
            type DataTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "SlashingWithdrawalCompleted(bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                31u8,
                64u8,
                64u8,
                8u8,
                137u8,
                39u8,
                78u8,
                208u8,
                123u8,
                36u8,
                132u8,
                94u8,
                80u8,
                84u8,
                168u8,
                122u8,
                12u8,
                171u8,
                150u8,
                158u8,
                177u8,
                39u8,
                122u8,
                175u8,
                230u8,
                26u8,
                227u8,
                82u8,
                231u8,
                195u8,
                42u8,
                0u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { withdrawalRoot: data.0 }
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
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawalRoot),
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
        impl alloy_sol_types::private::IntoLogData for SlashingWithdrawalCompleted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SlashingWithdrawalCompleted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &SlashingWithdrawalCompleted,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `SlashingWithdrawalQueued(bytes32,(address,address,address,uint256,uint32,address[],uint256[]),uint256[])` and selector `0x26b2aae26516e8719ef50ea2f6831a2efbd4e37dccdf0f6936b27bc08e793e30`.
```solidity
event SlashingWithdrawalQueued(bytes32 withdrawalRoot, IDelegationManagerTypes.Withdrawal withdrawal, uint256[] sharesToWithdraw);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SlashingWithdrawalQueued {
        #[allow(missing_docs)]
        pub withdrawalRoot: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub withdrawal: <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub sharesToWithdraw: alloy::sol_types::private::Vec<
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for SlashingWithdrawalQueued {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                IDelegationManagerTypes::Withdrawal,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "SlashingWithdrawalQueued(bytes32,(address,address,address,uint256,uint32,address[],uint256[]),uint256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                38u8,
                178u8,
                170u8,
                226u8,
                101u8,
                22u8,
                232u8,
                113u8,
                158u8,
                245u8,
                14u8,
                162u8,
                246u8,
                131u8,
                26u8,
                46u8,
                251u8,
                212u8,
                227u8,
                125u8,
                204u8,
                223u8,
                15u8,
                105u8,
                54u8,
                178u8,
                123u8,
                192u8,
                142u8,
                121u8,
                62u8,
                48u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    withdrawalRoot: data.0,
                    withdrawal: data.1,
                    sharesToWithdraw: data.2,
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawalRoot),
                    <IDelegationManagerTypes::Withdrawal as alloy_sol_types::SolType>::tokenize(
                        &self.withdrawal,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.sharesToWithdraw),
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
        impl alloy_sol_types::private::IntoLogData for SlashingWithdrawalQueued {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SlashingWithdrawalQueued> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &SlashingWithdrawalQueued,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `StakerDelegated(address,address)` and selector `0xc3ee9f2e5fda98e8066a1f745b2df9285f416fe98cf2559cd21484b3d8743304`.
```solidity
event StakerDelegated(address indexed staker, address indexed operator);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct StakerDelegated {
        #[allow(missing_docs)]
        pub staker: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for StakerDelegated {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "StakerDelegated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                195u8,
                238u8,
                159u8,
                46u8,
                95u8,
                218u8,
                152u8,
                232u8,
                6u8,
                106u8,
                31u8,
                116u8,
                91u8,
                45u8,
                249u8,
                40u8,
                95u8,
                65u8,
                111u8,
                233u8,
                140u8,
                242u8,
                85u8,
                156u8,
                210u8,
                20u8,
                132u8,
                179u8,
                216u8,
                116u8,
                51u8,
                4u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    staker: topics.1,
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
                (Self::SIGNATURE_HASH.into(), self.staker.clone(), self.operator.clone())
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
                    &self.staker,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for StakerDelegated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&StakerDelegated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &StakerDelegated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `StakerForceUndelegated(address,address)` and selector `0xf0eddf07e6ea14f388b47e1e94a0f464ecbd9eed4171130e0fc0e99fb4030a8a`.
```solidity
event StakerForceUndelegated(address indexed staker, address indexed operator);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct StakerForceUndelegated {
        #[allow(missing_docs)]
        pub staker: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for StakerForceUndelegated {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "StakerForceUndelegated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                240u8,
                237u8,
                223u8,
                7u8,
                230u8,
                234u8,
                20u8,
                243u8,
                136u8,
                180u8,
                126u8,
                30u8,
                148u8,
                160u8,
                244u8,
                100u8,
                236u8,
                189u8,
                158u8,
                237u8,
                65u8,
                113u8,
                19u8,
                14u8,
                15u8,
                192u8,
                233u8,
                159u8,
                180u8,
                3u8,
                10u8,
                138u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    staker: topics.1,
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
                (Self::SIGNATURE_HASH.into(), self.staker.clone(), self.operator.clone())
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
                    &self.staker,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for StakerForceUndelegated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&StakerForceUndelegated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &StakerForceUndelegated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `StakerUndelegated(address,address)` and selector `0xfee30966a256b71e14bc0ebfc94315e28ef4a97a7131a9e2b7a310a73af44676`.
```solidity
event StakerUndelegated(address indexed staker, address indexed operator);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct StakerUndelegated {
        #[allow(missing_docs)]
        pub staker: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for StakerUndelegated {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "StakerUndelegated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                254u8,
                227u8,
                9u8,
                102u8,
                162u8,
                86u8,
                183u8,
                30u8,
                20u8,
                188u8,
                14u8,
                191u8,
                201u8,
                67u8,
                21u8,
                226u8,
                142u8,
                244u8,
                169u8,
                122u8,
                113u8,
                49u8,
                169u8,
                226u8,
                183u8,
                163u8,
                16u8,
                167u8,
                58u8,
                244u8,
                70u8,
                118u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    staker: topics.1,
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
                (Self::SIGNATURE_HASH.into(), self.staker.clone(), self.operator.clone())
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
                    &self.staker,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for StakerUndelegated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&StakerUndelegated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &StakerUndelegated) -> alloy_sol_types::private::LogData {
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
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Unpaused(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                53u8,
                130u8,
                209u8,
                130u8,
                142u8,
                38u8,
                191u8,
                86u8,
                189u8,
                128u8,
                21u8,
                2u8,
                188u8,
                2u8,
                26u8,
                192u8,
                188u8,
                138u8,
                251u8,
                87u8,
                200u8,
                38u8,
                228u8,
                152u8,
                107u8,
                69u8,
                89u8,
                60u8,
                143u8,
                173u8,
                56u8,
                156u8,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.newPausedStatus),
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
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
constructor(address _strategyManager, address _eigenPodManager, address _allocationManager, address _pauserRegistry, address _permissionController, uint32 _MIN_WITHDRAWAL_DELAY, string _version);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _strategyManager: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _eigenPodManager: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _allocationManager: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _pauserRegistry: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _permissionController: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _MIN_WITHDRAWAL_DELAY: u32,
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
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::String,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                u32,
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (
                        value._strategyManager,
                        value._eigenPodManager,
                        value._allocationManager,
                        value._pauserRegistry,
                        value._permissionController,
                        value._MIN_WITHDRAWAL_DELAY,
                        value._version,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _strategyManager: tuple.0,
                        _eigenPodManager: tuple.1,
                        _allocationManager: tuple.2,
                        _pauserRegistry: tuple.3,
                        _permissionController: tuple.4,
                        _MIN_WITHDRAWAL_DELAY: tuple.5,
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
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::String,
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
                        &self._strategyManager,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._eigenPodManager,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._allocationManager,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._pauserRegistry,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._permissionController,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self._MIN_WITHDRAWAL_DELAY,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self._version,
                    ),
                )
            }
        }
    };
    /**Function with signature `DELEGATION_APPROVAL_TYPEHASH()` and selector `0x04a4f979`.
```solidity
function DELEGATION_APPROVAL_TYPEHASH() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DELEGATION_APPROVAL_TYPEHASHCall {}
    ///Container type for the return parameters of the [`DELEGATION_APPROVAL_TYPEHASH()`](DELEGATION_APPROVAL_TYPEHASHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DELEGATION_APPROVAL_TYPEHASHReturn {
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
            impl ::core::convert::From<DELEGATION_APPROVAL_TYPEHASHCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: DELEGATION_APPROVAL_TYPEHASHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DELEGATION_APPROVAL_TYPEHASHCall {
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
            impl ::core::convert::From<DELEGATION_APPROVAL_TYPEHASHReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: DELEGATION_APPROVAL_TYPEHASHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DELEGATION_APPROVAL_TYPEHASHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for DELEGATION_APPROVAL_TYPEHASHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = DELEGATION_APPROVAL_TYPEHASHReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DELEGATION_APPROVAL_TYPEHASH()";
            const SELECTOR: [u8; 4] = [4u8, 164u8, 249u8, 121u8];
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
    /**Function with signature `beaconChainETHStrategy()` and selector `0x9104c319`.
```solidity
function beaconChainETHStrategy() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct beaconChainETHStrategyCall {}
    ///Container type for the return parameters of the [`beaconChainETHStrategy()`](beaconChainETHStrategyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct beaconChainETHStrategyReturn {
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
            impl ::core::convert::From<beaconChainETHStrategyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: beaconChainETHStrategyCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for beaconChainETHStrategyCall {
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
            impl ::core::convert::From<beaconChainETHStrategyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: beaconChainETHStrategyReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for beaconChainETHStrategyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for beaconChainETHStrategyCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = beaconChainETHStrategyReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "beaconChainETHStrategy()";
            const SELECTOR: [u8; 4] = [145u8, 4u8, 195u8, 25u8];
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
    /**Function with signature `calculateDelegationApprovalDigestHash(address,address,address,bytes32,uint256)` and selector `0x0b9f487a`.
```solidity
function calculateDelegationApprovalDigestHash(address staker, address operator, address approver, bytes32 approverSalt, uint256 expiry) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateDelegationApprovalDigestHashCall {
        #[allow(missing_docs)]
        pub staker: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub approver: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub approverSalt: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub expiry: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`calculateDelegationApprovalDigestHash(address,address,address,bytes32,uint256)`](calculateDelegationApprovalDigestHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateDelegationApprovalDigestHashReturn {
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<calculateDelegationApprovalDigestHashCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: calculateDelegationApprovalDigestHashCall) -> Self {
                    (
                        value.staker,
                        value.operator,
                        value.approver,
                        value.approverSalt,
                        value.expiry,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calculateDelegationApprovalDigestHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        staker: tuple.0,
                        operator: tuple.1,
                        approver: tuple.2,
                        approverSalt: tuple.3,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<calculateDelegationApprovalDigestHashReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: calculateDelegationApprovalDigestHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calculateDelegationApprovalDigestHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for calculateDelegationApprovalDigestHashCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = calculateDelegationApprovalDigestHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "calculateDelegationApprovalDigestHash(address,address,address,bytes32,uint256)";
            const SELECTOR: [u8; 4] = [11u8, 159u8, 72u8, 122u8];
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
                        &self.staker,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.approver,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.approverSalt),
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `calculateWithdrawalRoot((address,address,address,uint256,uint32,address[],uint256[]))` and selector `0x597b36da`.
```solidity
function calculateWithdrawalRoot(IDelegationManagerTypes.Withdrawal memory withdrawal) external pure returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateWithdrawalRootCall {
        #[allow(missing_docs)]
        pub withdrawal: <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`calculateWithdrawalRoot((address,address,address,uint256,uint32,address[],uint256[]))`](calculateWithdrawalRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateWithdrawalRootReturn {
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
            type UnderlyingSolTuple<'a> = (IDelegationManagerTypes::Withdrawal,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<calculateWithdrawalRootCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: calculateWithdrawalRootCall) -> Self {
                    (value.withdrawal,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calculateWithdrawalRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { withdrawal: tuple.0 }
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
            impl ::core::convert::From<calculateWithdrawalRootReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: calculateWithdrawalRootReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calculateWithdrawalRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for calculateWithdrawalRootCall {
            type Parameters<'a> = (IDelegationManagerTypes::Withdrawal,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = calculateWithdrawalRootReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "calculateWithdrawalRoot((address,address,address,uint256,uint32,address[],uint256[]))";
            const SELECTOR: [u8; 4] = [89u8, 123u8, 54u8, 218u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IDelegationManagerTypes::Withdrawal as alloy_sol_types::SolType>::tokenize(
                        &self.withdrawal,
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
    /**Function with signature `completeQueuedWithdrawal((address,address,address,uint256,uint32,address[],uint256[]),address[],bool)` and selector `0xe4cc3f90`.
```solidity
function completeQueuedWithdrawal(IDelegationManagerTypes.Withdrawal memory withdrawal, address[] memory tokens, bool receiveAsTokens) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct completeQueuedWithdrawalCall {
        #[allow(missing_docs)]
        pub withdrawal: <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub tokens: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        #[allow(missing_docs)]
        pub receiveAsTokens: bool,
    }
    ///Container type for the return parameters of the [`completeQueuedWithdrawal((address,address,address,uint256,uint32,address[],uint256[]),address[],bool)`](completeQueuedWithdrawalCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct completeQueuedWithdrawalReturn {}
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
                IDelegationManagerTypes::Withdrawal,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<completeQueuedWithdrawalCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: completeQueuedWithdrawalCall) -> Self {
                    (value.withdrawal, value.tokens, value.receiveAsTokens)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for completeQueuedWithdrawalCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        withdrawal: tuple.0,
                        tokens: tuple.1,
                        receiveAsTokens: tuple.2,
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
            impl ::core::convert::From<completeQueuedWithdrawalReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: completeQueuedWithdrawalReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for completeQueuedWithdrawalReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for completeQueuedWithdrawalCall {
            type Parameters<'a> = (
                IDelegationManagerTypes::Withdrawal,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Bool,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = completeQueuedWithdrawalReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "completeQueuedWithdrawal((address,address,address,uint256,uint32,address[],uint256[]),address[],bool)";
            const SELECTOR: [u8; 4] = [228u8, 204u8, 63u8, 144u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IDelegationManagerTypes::Withdrawal as alloy_sol_types::SolType>::tokenize(
                        &self.withdrawal,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.tokens),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.receiveAsTokens,
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
    /**Function with signature `completeQueuedWithdrawals((address,address,address,uint256,uint32,address[],uint256[])[],address[][],bool[])` and selector `0x9435bb43`.
```solidity
function completeQueuedWithdrawals(IDelegationManagerTypes.Withdrawal[] memory withdrawals, address[][] memory tokens, bool[] memory receiveAsTokens) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct completeQueuedWithdrawalsCall {
        #[allow(missing_docs)]
        pub withdrawals: alloy::sol_types::private::Vec<
            <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub tokens: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        >,
        #[allow(missing_docs)]
        pub receiveAsTokens: alloy::sol_types::private::Vec<bool>,
    }
    ///Container type for the return parameters of the [`completeQueuedWithdrawals((address,address,address,uint256,uint32,address[],uint256[])[],address[][],bool[])`](completeQueuedWithdrawalsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct completeQueuedWithdrawalsReturn {}
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
                alloy::sol_types::sol_data::Array<IDelegationManagerTypes::Withdrawal>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    >,
                >,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bool>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
                >,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                >,
                alloy::sol_types::private::Vec<bool>,
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
            impl ::core::convert::From<completeQueuedWithdrawalsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: completeQueuedWithdrawalsCall) -> Self {
                    (value.withdrawals, value.tokens, value.receiveAsTokens)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for completeQueuedWithdrawalsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        withdrawals: tuple.0,
                        tokens: tuple.1,
                        receiveAsTokens: tuple.2,
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
            impl ::core::convert::From<completeQueuedWithdrawalsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: completeQueuedWithdrawalsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for completeQueuedWithdrawalsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for completeQueuedWithdrawalsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<IDelegationManagerTypes::Withdrawal>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    >,
                >,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bool>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = completeQueuedWithdrawalsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "completeQueuedWithdrawals((address,address,address,uint256,uint32,address[],uint256[])[],address[][],bool[])";
            const SELECTOR: [u8; 4] = [148u8, 53u8, 187u8, 67u8];
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
                        IDelegationManagerTypes::Withdrawal,
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawals),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::Address,
                        >,
                    > as alloy_sol_types::SolType>::tokenize(&self.tokens),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bool,
                    > as alloy_sol_types::SolType>::tokenize(&self.receiveAsTokens),
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
    /**Function with signature `convertToDepositShares(address,address[],uint256[])` and selector `0x25df922e`.
```solidity
function convertToDepositShares(address staker, address[] memory strategies, uint256[] memory withdrawableShares) external view returns (uint256[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct convertToDepositSharesCall {
        #[allow(missing_docs)]
        pub staker: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub withdrawableShares: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
    }
    ///Container type for the return parameters of the [`convertToDepositShares(address,address[],uint256[])`](convertToDepositSharesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct convertToDepositSharesReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<convertToDepositSharesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: convertToDepositSharesCall) -> Self {
                    (value.staker, value.strategies, value.withdrawableShares)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for convertToDepositSharesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        staker: tuple.0,
                        strategies: tuple.1,
                        withdrawableShares: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<convertToDepositSharesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: convertToDepositSharesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for convertToDepositSharesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for convertToDepositSharesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = convertToDepositSharesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "convertToDepositShares(address,address[],uint256[])";
            const SELECTOR: [u8; 4] = [37u8, 223u8, 146u8, 46u8];
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
                        &self.staker,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawableShares),
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
    /**Function with signature `cumulativeWithdrawalsQueued(address)` and selector `0xa1788484`.
```solidity
function cumulativeWithdrawalsQueued(address staker) external view returns (uint256 totalQueued);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cumulativeWithdrawalsQueuedCall {
        #[allow(missing_docs)]
        pub staker: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`cumulativeWithdrawalsQueued(address)`](cumulativeWithdrawalsQueuedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cumulativeWithdrawalsQueuedReturn {
        #[allow(missing_docs)]
        pub totalQueued: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<cumulativeWithdrawalsQueuedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: cumulativeWithdrawalsQueuedCall) -> Self {
                    (value.staker,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for cumulativeWithdrawalsQueuedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { staker: tuple.0 }
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
            impl ::core::convert::From<cumulativeWithdrawalsQueuedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: cumulativeWithdrawalsQueuedReturn) -> Self {
                    (value.totalQueued,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for cumulativeWithdrawalsQueuedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { totalQueued: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for cumulativeWithdrawalsQueuedCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = cumulativeWithdrawalsQueuedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "cumulativeWithdrawalsQueued(address)";
            const SELECTOR: [u8; 4] = [161u8, 120u8, 132u8, 132u8];
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
                        &self.staker,
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
    /**Function with signature `decreaseDelegatedShares(address,uint256,uint64)` and selector `0x60a0d1ce`.
```solidity
function decreaseDelegatedShares(address staker, uint256 curDepositShares, uint64 beaconChainSlashingFactorDecrease) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct decreaseDelegatedSharesCall {
        #[allow(missing_docs)]
        pub staker: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub curDepositShares: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub beaconChainSlashingFactorDecrease: u64,
    }
    ///Container type for the return parameters of the [`decreaseDelegatedShares(address,uint256,uint64)`](decreaseDelegatedSharesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct decreaseDelegatedSharesReturn {}
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
                alloy::sol_types::sol_data::Uint<64>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<decreaseDelegatedSharesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: decreaseDelegatedSharesCall) -> Self {
                    (
                        value.staker,
                        value.curDepositShares,
                        value.beaconChainSlashingFactorDecrease,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for decreaseDelegatedSharesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        staker: tuple.0,
                        curDepositShares: tuple.1,
                        beaconChainSlashingFactorDecrease: tuple.2,
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
            impl ::core::convert::From<decreaseDelegatedSharesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: decreaseDelegatedSharesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for decreaseDelegatedSharesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for decreaseDelegatedSharesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = decreaseDelegatedSharesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "decreaseDelegatedShares(address,uint256,uint64)";
            const SELECTOR: [u8; 4] = [96u8, 160u8, 209u8, 206u8];
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
                        &self.staker,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.curDepositShares),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.beaconChainSlashingFactorDecrease,
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
    /**Function with signature `delegateTo(address,(bytes,uint256),bytes32)` and selector `0xeea9064b`.
```solidity
function delegateTo(address operator, ISignatureUtilsMixinTypes.SignatureWithExpiry memory approverSignatureAndExpiry, bytes32 approverSalt) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegateToCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub approverSignatureAndExpiry: <ISignatureUtilsMixinTypes::SignatureWithExpiry as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub approverSalt: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`delegateTo(address,(bytes,uint256),bytes32)`](delegateToCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegateToReturn {}
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
                ISignatureUtilsMixinTypes::SignatureWithExpiry,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <ISignatureUtilsMixinTypes::SignatureWithExpiry as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<delegateToCall> for UnderlyingRustTuple<'_> {
                fn from(value: delegateToCall) -> Self {
                    (
                        value.operator,
                        value.approverSignatureAndExpiry,
                        value.approverSalt,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegateToCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        approverSignatureAndExpiry: tuple.1,
                        approverSalt: tuple.2,
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
            impl ::core::convert::From<delegateToReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delegateToReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegateToReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegateToCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                ISignatureUtilsMixinTypes::SignatureWithExpiry,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegateToReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegateTo(address,(bytes,uint256),bytes32)";
            const SELECTOR: [u8; 4] = [238u8, 169u8, 6u8, 75u8];
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
                    <ISignatureUtilsMixinTypes::SignatureWithExpiry as alloy_sol_types::SolType>::tokenize(
                        &self.approverSignatureAndExpiry,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.approverSalt),
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
    /**Function with signature `delegatedTo(address)` and selector `0x65da1264`.
```solidity
function delegatedTo(address staker) external view returns (address operator);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegatedToCall {
        #[allow(missing_docs)]
        pub staker: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`delegatedTo(address)`](delegatedToCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegatedToReturn {
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
            impl ::core::convert::From<delegatedToCall> for UnderlyingRustTuple<'_> {
                fn from(value: delegatedToCall) -> Self {
                    (value.staker,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegatedToCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { staker: tuple.0 }
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
            impl ::core::convert::From<delegatedToReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delegatedToReturn) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegatedToReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegatedToCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegatedToReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegatedTo(address)";
            const SELECTOR: [u8; 4] = [101u8, 218u8, 18u8, 100u8];
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
                        &self.staker,
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
    /**Function with signature `delegationApprover(address)` and selector `0x3cdeb5e0`.
```solidity
function delegationApprover(address operator) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegationApproverCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`delegationApprover(address)`](delegationApproverCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegationApproverReturn {
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
            impl ::core::convert::From<delegationApproverCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: delegationApproverCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for delegationApproverCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
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
            impl ::core::convert::From<delegationApproverReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: delegationApproverReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for delegationApproverReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegationApproverCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegationApproverReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegationApprover(address)";
            const SELECTOR: [u8; 4] = [60u8, 222u8, 181u8, 224u8];
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `delegationApproverSaltIsSpent(address,bytes32)` and selector `0xbb45fef2`.
```solidity
function delegationApproverSaltIsSpent(address delegationApprover, bytes32 salt) external view returns (bool spent);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegationApproverSaltIsSpentCall {
        #[allow(missing_docs)]
        pub delegationApprover: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub salt: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`delegationApproverSaltIsSpent(address,bytes32)`](delegationApproverSaltIsSpentCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegationApproverSaltIsSpentReturn {
        #[allow(missing_docs)]
        pub spent: bool,
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
            impl ::core::convert::From<delegationApproverSaltIsSpentCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: delegationApproverSaltIsSpentCall) -> Self {
                    (value.delegationApprover, value.salt)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for delegationApproverSaltIsSpentCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        delegationApprover: tuple.0,
                        salt: tuple.1,
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
            impl ::core::convert::From<delegationApproverSaltIsSpentReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: delegationApproverSaltIsSpentReturn) -> Self {
                    (value.spent,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for delegationApproverSaltIsSpentReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { spent: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegationApproverSaltIsSpentCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegationApproverSaltIsSpentReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegationApproverSaltIsSpent(address,bytes32)";
            const SELECTOR: [u8; 4] = [187u8, 69u8, 254u8, 242u8];
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
                        &self.delegationApprover,
                    ),
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
    /**Function with signature `depositScalingFactor(address,address)` and selector `0xbfae3fd2`.
```solidity
function depositScalingFactor(address staker, address strategy) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct depositScalingFactorCall {
        #[allow(missing_docs)]
        pub staker: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`depositScalingFactor(address,address)`](depositScalingFactorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct depositScalingFactorReturn {
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
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<depositScalingFactorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: depositScalingFactorCall) -> Self {
                    (value.staker, value.strategy)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for depositScalingFactorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        staker: tuple.0,
                        strategy: tuple.1,
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
            impl ::core::convert::From<depositScalingFactorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: depositScalingFactorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for depositScalingFactorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for depositScalingFactorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = depositScalingFactorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "depositScalingFactor(address,address)";
            const SELECTOR: [u8; 4] = [191u8, 174u8, 63u8, 210u8];
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
                        &self.staker,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<domainSeparatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: domainSeparatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for domainSeparatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for domainSeparatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = domainSeparatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `eigenPodManager()` and selector `0x4665bcda`.
```solidity
function eigenPodManager() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eigenPodManagerCall {}
    ///Container type for the return parameters of the [`eigenPodManager()`](eigenPodManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eigenPodManagerReturn {
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
            impl ::core::convert::From<eigenPodManagerCall> for UnderlyingRustTuple<'_> {
                fn from(value: eigenPodManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eigenPodManagerCall {
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
            impl ::core::convert::From<eigenPodManagerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: eigenPodManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for eigenPodManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for eigenPodManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = eigenPodManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "eigenPodManager()";
            const SELECTOR: [u8; 4] = [70u8, 101u8, 188u8, 218u8];
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
    /**Function with signature `getDepositedShares(address)` and selector `0x66d5ba93`.
```solidity
function getDepositedShares(address staker) external view returns (address[] memory, uint256[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getDepositedSharesCall {
        #[allow(missing_docs)]
        pub staker: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getDepositedShares(address)`](getDepositedSharesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getDepositedSharesReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Vec<
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
            impl ::core::convert::From<getDepositedSharesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getDepositedSharesCall) -> Self {
                    (value.staker,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getDepositedSharesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { staker: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<getDepositedSharesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getDepositedSharesReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getDepositedSharesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getDepositedSharesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getDepositedSharesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getDepositedShares(address)";
            const SELECTOR: [u8; 4] = [102u8, 213u8, 186u8, 147u8];
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
                        &self.staker,
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
    /**Function with signature `getOperatorShares(address,address[])` and selector `0x90041347`.
```solidity
function getOperatorShares(address operator, address[] memory strategies) external view returns (uint256[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorSharesCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    ///Container type for the return parameters of the [`getOperatorShares(address,address[])`](getOperatorSharesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorSharesReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<getOperatorSharesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSharesCall) -> Self {
                    (value.operator, value.strategies)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorSharesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        strategies: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<getOperatorSharesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSharesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorSharesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorSharesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorSharesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorShares(address,address[])";
            const SELECTOR: [u8; 4] = [144u8, 4u8, 19u8, 71u8];
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
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
    /**Function with signature `getOperatorsShares(address[],address[])` and selector `0xf0e0e676`.
```solidity
function getOperatorsShares(address[] memory operators, address[] memory strategies) external view returns (uint256[][] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorsSharesCall {
        #[allow(missing_docs)]
        pub operators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    ///Container type for the return parameters of the [`getOperatorsShares(address[],address[])`](getOperatorsSharesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorsSharesReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<getOperatorsSharesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorsSharesCall) -> Self {
                    (value.operators, value.strategies)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorsSharesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operators: tuple.0,
                        strategies: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    >,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::Vec<
                        alloy::sol_types::private::primitives::aliases::U256,
                    >,
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
            impl ::core::convert::From<getOperatorsSharesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorsSharesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorsSharesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorsSharesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorsSharesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    >,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorsShares(address[],address[])";
            const SELECTOR: [u8; 4] = [240u8, 224u8, 230u8, 118u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.operators),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
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
    /**Function with signature `getQueuedWithdrawal(bytes32)` and selector `0x5d975e88`.
```solidity
function getQueuedWithdrawal(bytes32 withdrawalRoot) external view returns (IDelegationManagerTypes.Withdrawal memory withdrawal, uint256[] memory shares);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQueuedWithdrawalCall {
        #[allow(missing_docs)]
        pub withdrawalRoot: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`getQueuedWithdrawal(bytes32)`](getQueuedWithdrawalCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQueuedWithdrawalReturn {
        #[allow(missing_docs)]
        pub withdrawal: <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub shares: alloy::sol_types::private::Vec<
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
            impl ::core::convert::From<getQueuedWithdrawalCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getQueuedWithdrawalCall) -> Self {
                    (value.withdrawalRoot,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getQueuedWithdrawalCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { withdrawalRoot: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                IDelegationManagerTypes::Withdrawal,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getQueuedWithdrawalReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getQueuedWithdrawalReturn) -> Self {
                    (value.withdrawal, value.shares)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getQueuedWithdrawalReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        withdrawal: tuple.0,
                        shares: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getQueuedWithdrawalCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getQueuedWithdrawalReturn;
            type ReturnTuple<'a> = (
                IDelegationManagerTypes::Withdrawal,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getQueuedWithdrawal(bytes32)";
            const SELECTOR: [u8; 4] = [93u8, 151u8, 94u8, 136u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawalRoot),
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
    /**Function with signature `getQueuedWithdrawalRoots(address)` and selector `0xfd8aa88d`.
```solidity
function getQueuedWithdrawalRoots(address staker) external view returns (bytes32[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQueuedWithdrawalRootsCall {
        #[allow(missing_docs)]
        pub staker: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getQueuedWithdrawalRoots(address)`](getQueuedWithdrawalRootsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQueuedWithdrawalRootsReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<getQueuedWithdrawalRootsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getQueuedWithdrawalRootsCall) -> Self {
                    (value.staker,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getQueuedWithdrawalRootsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { staker: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<getQueuedWithdrawalRootsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getQueuedWithdrawalRootsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getQueuedWithdrawalRootsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getQueuedWithdrawalRootsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getQueuedWithdrawalRootsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getQueuedWithdrawalRoots(address)";
            const SELECTOR: [u8; 4] = [253u8, 138u8, 168u8, 141u8];
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
                        &self.staker,
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
    /**Function with signature `getQueuedWithdrawals(address)` and selector `0x5dd68579`.
```solidity
function getQueuedWithdrawals(address staker) external view returns (IDelegationManagerTypes.Withdrawal[] memory withdrawals, uint256[][] memory shares);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQueuedWithdrawalsCall {
        #[allow(missing_docs)]
        pub staker: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getQueuedWithdrawals(address)`](getQueuedWithdrawalsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQueuedWithdrawalsReturn {
        #[allow(missing_docs)]
        pub withdrawals: alloy::sol_types::private::Vec<
            <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub shares: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
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
            impl ::core::convert::From<getQueuedWithdrawalsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getQueuedWithdrawalsCall) -> Self {
                    (value.staker,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getQueuedWithdrawalsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { staker: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<IDelegationManagerTypes::Withdrawal>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    >,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
                >,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::Vec<
                        alloy::sol_types::private::primitives::aliases::U256,
                    >,
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
            impl ::core::convert::From<getQueuedWithdrawalsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getQueuedWithdrawalsReturn) -> Self {
                    (value.withdrawals, value.shares)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getQueuedWithdrawalsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        withdrawals: tuple.0,
                        shares: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getQueuedWithdrawalsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getQueuedWithdrawalsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<IDelegationManagerTypes::Withdrawal>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    >,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getQueuedWithdrawals(address)";
            const SELECTOR: [u8; 4] = [93u8, 214u8, 133u8, 121u8];
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
                        &self.staker,
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
    /**Function with signature `getSlashableSharesInQueue(address,address)` and selector `0x6e174448`.
```solidity
function getSlashableSharesInQueue(address operator, address strategy) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSlashableSharesInQueueCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getSlashableSharesInQueue(address,address)`](getSlashableSharesInQueueCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSlashableSharesInQueueReturn {
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
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<getSlashableSharesInQueueCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getSlashableSharesInQueueCall) -> Self {
                    (value.operator, value.strategy)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getSlashableSharesInQueueCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        strategy: tuple.1,
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
            impl ::core::convert::From<getSlashableSharesInQueueReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getSlashableSharesInQueueReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getSlashableSharesInQueueReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getSlashableSharesInQueueCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getSlashableSharesInQueueReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getSlashableSharesInQueue(address,address)";
            const SELECTOR: [u8; 4] = [110u8, 23u8, 68u8, 72u8];
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
                        &self.strategy,
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
    /**Function with signature `getWithdrawableShares(address,address[])` and selector `0xc978f7ac`.
```solidity
function getWithdrawableShares(address staker, address[] memory strategies) external view returns (uint256[] memory withdrawableShares, uint256[] memory depositShares);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getWithdrawableSharesCall {
        #[allow(missing_docs)]
        pub staker: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    ///Container type for the return parameters of the [`getWithdrawableShares(address,address[])`](getWithdrawableSharesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getWithdrawableSharesReturn {
        #[allow(missing_docs)]
        pub withdrawableShares: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        #[allow(missing_docs)]
        pub depositShares: alloy::sol_types::private::Vec<
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<getWithdrawableSharesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getWithdrawableSharesCall) -> Self {
                    (value.staker, value.strategies)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getWithdrawableSharesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        staker: tuple.0,
                        strategies: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
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
            impl ::core::convert::From<getWithdrawableSharesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getWithdrawableSharesReturn) -> Self {
                    (value.withdrawableShares, value.depositShares)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getWithdrawableSharesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        withdrawableShares: tuple.0,
                        depositShares: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getWithdrawableSharesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getWithdrawableSharesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getWithdrawableShares(address,address[])";
            const SELECTOR: [u8; 4] = [201u8, 120u8, 247u8, 172u8];
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
                        &self.staker,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
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
    /**Function with signature `increaseDelegatedShares(address,address,uint256,uint256)` and selector `0x3c651cf2`.
```solidity
function increaseDelegatedShares(address staker, address strategy, uint256 prevDepositShares, uint256 addedShares) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct increaseDelegatedSharesCall {
        #[allow(missing_docs)]
        pub staker: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub prevDepositShares: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub addedShares: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`increaseDelegatedShares(address,address,uint256,uint256)`](increaseDelegatedSharesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct increaseDelegatedSharesReturn {}
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
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<increaseDelegatedSharesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: increaseDelegatedSharesCall) -> Self {
                    (
                        value.staker,
                        value.strategy,
                        value.prevDepositShares,
                        value.addedShares,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for increaseDelegatedSharesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        staker: tuple.0,
                        strategy: tuple.1,
                        prevDepositShares: tuple.2,
                        addedShares: tuple.3,
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
            impl ::core::convert::From<increaseDelegatedSharesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: increaseDelegatedSharesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for increaseDelegatedSharesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for increaseDelegatedSharesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = increaseDelegatedSharesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "increaseDelegatedShares(address,address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [60u8, 101u8, 28u8, 242u8];
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
                        &self.staker,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.prevDepositShares),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.addedShares),
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
    /**Function with signature `initialize(address,uint256)` and selector `0xcd6dc687`.
```solidity
function initialize(address initialOwner, uint256 initialPausedStatus) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        #[allow(missing_docs)]
        pub initialOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub initialPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`initialize(address,uint256)`](initializeCall) function.
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
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (value.initialOwner, value.initialPausedStatus)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        initialOwner: tuple.0,
                        initialPausedStatus: tuple.1,
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
            const SIGNATURE: &'static str = "initialize(address,uint256)";
            const SELECTOR: [u8; 4] = [205u8, 109u8, 198u8, 135u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.initialPausedStatus),
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
    /**Function with signature `isDelegated(address)` and selector `0x3e28391d`.
```solidity
function isDelegated(address staker) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isDelegatedCall {
        #[allow(missing_docs)]
        pub staker: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`isDelegated(address)`](isDelegatedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isDelegatedReturn {
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
            impl ::core::convert::From<isDelegatedCall> for UnderlyingRustTuple<'_> {
                fn from(value: isDelegatedCall) -> Self {
                    (value.staker,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isDelegatedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { staker: tuple.0 }
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
            impl ::core::convert::From<isDelegatedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isDelegatedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isDelegatedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isDelegatedCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isDelegatedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isDelegated(address)";
            const SELECTOR: [u8; 4] = [62u8, 40u8, 57u8, 29u8];
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
                        &self.staker,
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
    /**Function with signature `isOperator(address)` and selector `0x6d70f7ae`.
```solidity
function isOperator(address operator) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperatorCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`isOperator(address)`](isOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperatorReturn {
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
            impl ::core::convert::From<isOperatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: isOperatorCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
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
            impl ::core::convert::From<isOperatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isOperatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isOperatorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isOperatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isOperator(address)";
            const SELECTOR: [u8; 4] = [109u8, 112u8, 247u8, 174u8];
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `minWithdrawalDelayBlocks()` and selector `0xc448feb8`.
```solidity
function minWithdrawalDelayBlocks() external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct minWithdrawalDelayBlocksCall {}
    ///Container type for the return parameters of the [`minWithdrawalDelayBlocks()`](minWithdrawalDelayBlocksCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct minWithdrawalDelayBlocksReturn {
        #[allow(missing_docs)]
        pub _0: u32,
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
            impl ::core::convert::From<minWithdrawalDelayBlocksCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: minWithdrawalDelayBlocksCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for minWithdrawalDelayBlocksCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
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
            impl ::core::convert::From<minWithdrawalDelayBlocksReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: minWithdrawalDelayBlocksReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for minWithdrawalDelayBlocksReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for minWithdrawalDelayBlocksCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = minWithdrawalDelayBlocksReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "minWithdrawalDelayBlocks()";
            const SELECTOR: [u8; 4] = [196u8, 72u8, 254u8, 184u8];
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
    /**Function with signature `modifyOperatorDetails(address,address)` and selector `0x54b7c96c`.
```solidity
function modifyOperatorDetails(address operator, address newDelegationApprover) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct modifyOperatorDetailsCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newDelegationApprover: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`modifyOperatorDetails(address,address)`](modifyOperatorDetailsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct modifyOperatorDetailsReturn {}
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
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<modifyOperatorDetailsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: modifyOperatorDetailsCall) -> Self {
                    (value.operator, value.newDelegationApprover)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for modifyOperatorDetailsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        newDelegationApprover: tuple.1,
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
            impl ::core::convert::From<modifyOperatorDetailsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: modifyOperatorDetailsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for modifyOperatorDetailsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for modifyOperatorDetailsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = modifyOperatorDetailsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "modifyOperatorDetails(address,address)";
            const SELECTOR: [u8; 4] = [84u8, 183u8, 201u8, 108u8];
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
                        &self.newDelegationApprover,
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
    /**Function with signature `operatorShares(address,address)` and selector `0x778e55f3`.
```solidity
function operatorShares(address operator, address strategy) external view returns (uint256 shares);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorSharesCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`operatorShares(address,address)`](operatorSharesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorSharesReturn {
        #[allow(missing_docs)]
        pub shares: alloy::sol_types::private::primitives::aliases::U256,
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
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<operatorSharesCall> for UnderlyingRustTuple<'_> {
                fn from(value: operatorSharesCall) -> Self {
                    (value.operator, value.strategy)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for operatorSharesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        strategy: tuple.1,
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
            impl ::core::convert::From<operatorSharesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: operatorSharesReturn) -> Self {
                    (value.shares,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for operatorSharesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { shares: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for operatorSharesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = operatorSharesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "operatorShares(address,address)";
            const SELECTOR: [u8; 4] = [119u8, 142u8, 85u8, 243u8];
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
                        &self.strategy,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = ownerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
            impl ::core::convert::From<pauseCall> for UnderlyingRustTuple<'_> {
                fn from(value: pauseCall) -> Self {
                    (value.newPausedStatus,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newPausedStatus: tuple.0 }
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = pauseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newPausedStatus),
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = pauseAllReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = paused_0Return;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.index),
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = paused_1Return;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pauserRegistryReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: pauserRegistryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for pauserRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pauserRegistryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = pauserRegistryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `pendingWithdrawals(bytes32)` and selector `0xb7f06ebe`.
```solidity
function pendingWithdrawals(bytes32 withdrawalRoot) external view returns (bool pending);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pendingWithdrawalsCall {
        #[allow(missing_docs)]
        pub withdrawalRoot: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`pendingWithdrawals(bytes32)`](pendingWithdrawalsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pendingWithdrawalsReturn {
        #[allow(missing_docs)]
        pub pending: bool,
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
            impl ::core::convert::From<pendingWithdrawalsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: pendingWithdrawalsCall) -> Self {
                    (value.withdrawalRoot,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for pendingWithdrawalsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { withdrawalRoot: tuple.0 }
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
            impl ::core::convert::From<pendingWithdrawalsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: pendingWithdrawalsReturn) -> Self {
                    (value.pending,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for pendingWithdrawalsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { pending: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pendingWithdrawalsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = pendingWithdrawalsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pendingWithdrawals(bytes32)";
            const SELECTOR: [u8; 4] = [183u8, 240u8, 110u8, 190u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawalRoot),
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
    /**Function with signature `permissionController()` and selector `0x4657e26a`.
```solidity
function permissionController() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct permissionControllerCall {}
    ///Container type for the return parameters of the [`permissionController()`](permissionControllerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct permissionControllerReturn {
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
            impl ::core::convert::From<permissionControllerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: permissionControllerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for permissionControllerCall {
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
            impl ::core::convert::From<permissionControllerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: permissionControllerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for permissionControllerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for permissionControllerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = permissionControllerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "permissionController()";
            const SELECTOR: [u8; 4] = [70u8, 87u8, 226u8, 106u8];
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
    /**Function with signature `queueWithdrawals((address[],uint256[],address)[])` and selector `0x0dd8dd02`.
```solidity
function queueWithdrawals(IDelegationManagerTypes.QueuedWithdrawalParams[] memory params) external returns (bytes32[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct queueWithdrawalsCall {
        #[allow(missing_docs)]
        pub params: alloy::sol_types::private::Vec<
            <IDelegationManagerTypes::QueuedWithdrawalParams as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`queueWithdrawals((address[],uint256[],address)[])`](queueWithdrawalsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct queueWithdrawalsReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    IDelegationManagerTypes::QueuedWithdrawalParams,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IDelegationManagerTypes::QueuedWithdrawalParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<queueWithdrawalsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: queueWithdrawalsCall) -> Self {
                    (value.params,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for queueWithdrawalsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { params: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<queueWithdrawalsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: queueWithdrawalsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for queueWithdrawalsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for queueWithdrawalsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    IDelegationManagerTypes::QueuedWithdrawalParams,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = queueWithdrawalsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "queueWithdrawals((address[],uint256[],address)[])";
            const SELECTOR: [u8; 4] = [13u8, 216u8, 221u8, 2u8];
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
                        IDelegationManagerTypes::QueuedWithdrawalParams,
                    > as alloy_sol_types::SolType>::tokenize(&self.params),
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
    /**Function with signature `queuedWithdrawals(bytes32)` and selector `0x99f5371b`.
```solidity
function queuedWithdrawals(bytes32 withdrawalRoot) external view returns (IDelegationManagerTypes.Withdrawal memory withdrawal);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct queuedWithdrawalsCall {
        #[allow(missing_docs)]
        pub withdrawalRoot: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`queuedWithdrawals(bytes32)`](queuedWithdrawalsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct queuedWithdrawalsReturn {
        #[allow(missing_docs)]
        pub withdrawal: <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<queuedWithdrawalsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: queuedWithdrawalsCall) -> Self {
                    (value.withdrawalRoot,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for queuedWithdrawalsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { withdrawalRoot: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IDelegationManagerTypes::Withdrawal,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<queuedWithdrawalsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: queuedWithdrawalsReturn) -> Self {
                    (value.withdrawal,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for queuedWithdrawalsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { withdrawal: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for queuedWithdrawalsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = queuedWithdrawalsReturn;
            type ReturnTuple<'a> = (IDelegationManagerTypes::Withdrawal,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "queuedWithdrawals(bytes32)";
            const SELECTOR: [u8; 4] = [153u8, 245u8, 55u8, 27u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawalRoot),
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
    /**Function with signature `redelegate(address,(bytes,uint256),bytes32)` and selector `0xa33a3433`.
```solidity
function redelegate(address newOperator, ISignatureUtilsMixinTypes.SignatureWithExpiry memory newOperatorApproverSig, bytes32 approverSalt) external returns (bytes32[] memory withdrawalRoots);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct redelegateCall {
        #[allow(missing_docs)]
        pub newOperator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newOperatorApproverSig: <ISignatureUtilsMixinTypes::SignatureWithExpiry as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub approverSalt: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`redelegate(address,(bytes,uint256),bytes32)`](redelegateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct redelegateReturn {
        #[allow(missing_docs)]
        pub withdrawalRoots: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                ISignatureUtilsMixinTypes::SignatureWithExpiry,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <ISignatureUtilsMixinTypes::SignatureWithExpiry as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<redelegateCall> for UnderlyingRustTuple<'_> {
                fn from(value: redelegateCall) -> Self {
                    (value.newOperator, value.newOperatorApproverSig, value.approverSalt)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for redelegateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newOperator: tuple.0,
                        newOperatorApproverSig: tuple.1,
                        approverSalt: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<redelegateReturn> for UnderlyingRustTuple<'_> {
                fn from(value: redelegateReturn) -> Self {
                    (value.withdrawalRoots,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for redelegateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { withdrawalRoots: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for redelegateCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                ISignatureUtilsMixinTypes::SignatureWithExpiry,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = redelegateReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "redelegate(address,(bytes,uint256),bytes32)";
            const SELECTOR: [u8; 4] = [163u8, 58u8, 52u8, 51u8];
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
                        &self.newOperator,
                    ),
                    <ISignatureUtilsMixinTypes::SignatureWithExpiry as alloy_sol_types::SolType>::tokenize(
                        &self.newOperatorApproverSig,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.approverSalt),
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
    /**Function with signature `registerAsOperator(address,uint32,string)` and selector `0x2aa6d888`.
```solidity
function registerAsOperator(address initDelegationApprover, uint32 allocationDelay, string memory metadataURI) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerAsOperatorCall {
        #[allow(missing_docs)]
        pub initDelegationApprover: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub allocationDelay: u32,
        #[allow(missing_docs)]
        pub metadataURI: alloy::sol_types::private::String,
    }
    ///Container type for the return parameters of the [`registerAsOperator(address,uint32,string)`](registerAsOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerAsOperatorReturn {}
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
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::String,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                u32,
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
            impl ::core::convert::From<registerAsOperatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerAsOperatorCall) -> Self {
                    (
                        value.initDelegationApprover,
                        value.allocationDelay,
                        value.metadataURI,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerAsOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        initDelegationApprover: tuple.0,
                        allocationDelay: tuple.1,
                        metadataURI: tuple.2,
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
            impl ::core::convert::From<registerAsOperatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerAsOperatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerAsOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerAsOperatorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::String,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerAsOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registerAsOperator(address,uint32,string)";
            const SELECTOR: [u8; 4] = [42u8, 166u8, 216u8, 136u8];
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
                        &self.initDelegationApprover,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.allocationDelay),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.metadataURI,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipCall {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceOwnershipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `slashOperatorShares(address,address,uint64,uint64)` and selector `0x601bb36f`.
```solidity
function slashOperatorShares(address operator, address strategy, uint64 prevMaxMagnitude, uint64 newMaxMagnitude) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashOperatorSharesCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub prevMaxMagnitude: u64,
        #[allow(missing_docs)]
        pub newMaxMagnitude: u64,
    }
    ///Container type for the return parameters of the [`slashOperatorShares(address,address,uint64,uint64)`](slashOperatorSharesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashOperatorSharesReturn {}
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
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                u64,
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
            impl ::core::convert::From<slashOperatorSharesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: slashOperatorSharesCall) -> Self {
                    (
                        value.operator,
                        value.strategy,
                        value.prevMaxMagnitude,
                        value.newMaxMagnitude,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for slashOperatorSharesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        strategy: tuple.1,
                        prevMaxMagnitude: tuple.2,
                        newMaxMagnitude: tuple.3,
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
            impl ::core::convert::From<slashOperatorSharesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: slashOperatorSharesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for slashOperatorSharesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for slashOperatorSharesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = slashOperatorSharesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "slashOperatorShares(address,address,uint64,uint64)";
            const SELECTOR: [u8; 4] = [96u8, 27u8, 179u8, 111u8];
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
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.prevMaxMagnitude),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.newMaxMagnitude),
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
    /**Function with signature `strategyManager()` and selector `0x39b70e38`.
```solidity
function strategyManager() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategyManagerCall {}
    ///Container type for the return parameters of the [`strategyManager()`](strategyManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategyManagerReturn {
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
            impl ::core::convert::From<strategyManagerCall> for UnderlyingRustTuple<'_> {
                fn from(value: strategyManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for strategyManagerCall {
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
            impl ::core::convert::From<strategyManagerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: strategyManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for strategyManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for strategyManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = strategyManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "strategyManager()";
            const SELECTOR: [u8; 4] = [57u8, 183u8, 14u8, 56u8];
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipCall) -> Self {
                    (value.newOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipCall {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferOwnershipCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `undelegate(address)` and selector `0xda8be864`.
```solidity
function undelegate(address staker) external returns (bytes32[] memory withdrawalRoots);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct undelegateCall {
        #[allow(missing_docs)]
        pub staker: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`undelegate(address)`](undelegateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct undelegateReturn {
        #[allow(missing_docs)]
        pub withdrawalRoots: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<undelegateCall> for UnderlyingRustTuple<'_> {
                fn from(value: undelegateCall) -> Self {
                    (value.staker,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for undelegateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { staker: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<undelegateReturn> for UnderlyingRustTuple<'_> {
                fn from(value: undelegateReturn) -> Self {
                    (value.withdrawalRoots,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for undelegateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { withdrawalRoots: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for undelegateCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = undelegateReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "undelegate(address)";
            const SELECTOR: [u8; 4] = [218u8, 139u8, 232u8, 100u8];
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
                        &self.staker,
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
            impl ::core::convert::From<unpauseCall> for UnderlyingRustTuple<'_> {
                fn from(value: unpauseCall) -> Self {
                    (value.newPausedStatus,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unpauseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newPausedStatus: tuple.0 }
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = unpauseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newPausedStatus),
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
    /**Function with signature `updateOperatorMetadataURI(address,string)` and selector `0x78296ec5`.
```solidity
function updateOperatorMetadataURI(address operator, string memory metadataURI) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorMetadataURICall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub metadataURI: alloy::sol_types::private::String,
    }
    ///Container type for the return parameters of the [`updateOperatorMetadataURI(address,string)`](updateOperatorMetadataURICall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorMetadataURIReturn {}
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
                alloy::sol_types::sol_data::String,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<updateOperatorMetadataURICall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorMetadataURICall) -> Self {
                    (value.operator, value.metadataURI)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateOperatorMetadataURICall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        metadataURI: tuple.1,
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
            impl ::core::convert::From<updateOperatorMetadataURIReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorMetadataURIReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateOperatorMetadataURIReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateOperatorMetadataURICall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::String,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateOperatorMetadataURIReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateOperatorMetadataURI(address,string)";
            const SELECTOR: [u8; 4] = [120u8, 41u8, 110u8, 197u8];
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.metadataURI,
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
    ///Container for all the [`DelegationManager`](self) function calls.
    pub enum DelegationManagerCalls {
        #[allow(missing_docs)]
        DELEGATION_APPROVAL_TYPEHASH(DELEGATION_APPROVAL_TYPEHASHCall),
        #[allow(missing_docs)]
        allocationManager(allocationManagerCall),
        #[allow(missing_docs)]
        beaconChainETHStrategy(beaconChainETHStrategyCall),
        #[allow(missing_docs)]
        calculateDelegationApprovalDigestHash(calculateDelegationApprovalDigestHashCall),
        #[allow(missing_docs)]
        calculateWithdrawalRoot(calculateWithdrawalRootCall),
        #[allow(missing_docs)]
        completeQueuedWithdrawal(completeQueuedWithdrawalCall),
        #[allow(missing_docs)]
        completeQueuedWithdrawals(completeQueuedWithdrawalsCall),
        #[allow(missing_docs)]
        convertToDepositShares(convertToDepositSharesCall),
        #[allow(missing_docs)]
        cumulativeWithdrawalsQueued(cumulativeWithdrawalsQueuedCall),
        #[allow(missing_docs)]
        decreaseDelegatedShares(decreaseDelegatedSharesCall),
        #[allow(missing_docs)]
        delegateTo(delegateToCall),
        #[allow(missing_docs)]
        delegatedTo(delegatedToCall),
        #[allow(missing_docs)]
        delegationApprover(delegationApproverCall),
        #[allow(missing_docs)]
        delegationApproverSaltIsSpent(delegationApproverSaltIsSpentCall),
        #[allow(missing_docs)]
        depositScalingFactor(depositScalingFactorCall),
        #[allow(missing_docs)]
        domainSeparator(domainSeparatorCall),
        #[allow(missing_docs)]
        eigenPodManager(eigenPodManagerCall),
        #[allow(missing_docs)]
        getDepositedShares(getDepositedSharesCall),
        #[allow(missing_docs)]
        getOperatorShares(getOperatorSharesCall),
        #[allow(missing_docs)]
        getOperatorsShares(getOperatorsSharesCall),
        #[allow(missing_docs)]
        getQueuedWithdrawal(getQueuedWithdrawalCall),
        #[allow(missing_docs)]
        getQueuedWithdrawalRoots(getQueuedWithdrawalRootsCall),
        #[allow(missing_docs)]
        getQueuedWithdrawals(getQueuedWithdrawalsCall),
        #[allow(missing_docs)]
        getSlashableSharesInQueue(getSlashableSharesInQueueCall),
        #[allow(missing_docs)]
        getWithdrawableShares(getWithdrawableSharesCall),
        #[allow(missing_docs)]
        increaseDelegatedShares(increaseDelegatedSharesCall),
        #[allow(missing_docs)]
        initialize(initializeCall),
        #[allow(missing_docs)]
        isDelegated(isDelegatedCall),
        #[allow(missing_docs)]
        isOperator(isOperatorCall),
        #[allow(missing_docs)]
        minWithdrawalDelayBlocks(minWithdrawalDelayBlocksCall),
        #[allow(missing_docs)]
        modifyOperatorDetails(modifyOperatorDetailsCall),
        #[allow(missing_docs)]
        operatorShares(operatorSharesCall),
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
        pendingWithdrawals(pendingWithdrawalsCall),
        #[allow(missing_docs)]
        permissionController(permissionControllerCall),
        #[allow(missing_docs)]
        queueWithdrawals(queueWithdrawalsCall),
        #[allow(missing_docs)]
        queuedWithdrawals(queuedWithdrawalsCall),
        #[allow(missing_docs)]
        redelegate(redelegateCall),
        #[allow(missing_docs)]
        registerAsOperator(registerAsOperatorCall),
        #[allow(missing_docs)]
        renounceOwnership(renounceOwnershipCall),
        #[allow(missing_docs)]
        slashOperatorShares(slashOperatorSharesCall),
        #[allow(missing_docs)]
        strategyManager(strategyManagerCall),
        #[allow(missing_docs)]
        transferOwnership(transferOwnershipCall),
        #[allow(missing_docs)]
        undelegate(undelegateCall),
        #[allow(missing_docs)]
        unpause(unpauseCall),
        #[allow(missing_docs)]
        updateOperatorMetadataURI(updateOperatorMetadataURICall),
        #[allow(missing_docs)]
        version(versionCall),
    }
    #[automatically_derived]
    impl DelegationManagerCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [4u8, 164u8, 249u8, 121u8],
            [11u8, 159u8, 72u8, 122u8],
            [13u8, 216u8, 221u8, 2u8],
            [19u8, 100u8, 57u8, 221u8],
            [37u8, 223u8, 146u8, 46u8],
            [42u8, 166u8, 216u8, 136u8],
            [57u8, 183u8, 14u8, 56u8],
            [60u8, 101u8, 28u8, 242u8],
            [60u8, 222u8, 181u8, 224u8],
            [62u8, 40u8, 57u8, 29u8],
            [70u8, 87u8, 226u8, 106u8],
            [70u8, 101u8, 188u8, 218u8],
            [84u8, 183u8, 201u8, 108u8],
            [84u8, 253u8, 77u8, 80u8],
            [89u8, 92u8, 106u8, 103u8],
            [89u8, 123u8, 54u8, 218u8],
            [90u8, 200u8, 106u8, 183u8],
            [92u8, 151u8, 90u8, 187u8],
            [93u8, 151u8, 94u8, 136u8],
            [93u8, 214u8, 133u8, 121u8],
            [96u8, 27u8, 179u8, 111u8],
            [96u8, 160u8, 209u8, 206u8],
            [101u8, 218u8, 18u8, 100u8],
            [102u8, 213u8, 186u8, 147u8],
            [109u8, 112u8, 247u8, 174u8],
            [110u8, 23u8, 68u8, 72u8],
            [113u8, 80u8, 24u8, 166u8],
            [119u8, 142u8, 85u8, 243u8],
            [120u8, 41u8, 110u8, 197u8],
            [136u8, 111u8, 17u8, 149u8],
            [141u8, 165u8, 203u8, 91u8],
            [144u8, 4u8, 19u8, 71u8],
            [145u8, 4u8, 195u8, 25u8],
            [148u8, 53u8, 187u8, 67u8],
            [153u8, 245u8, 55u8, 27u8],
            [161u8, 120u8, 132u8, 132u8],
            [163u8, 58u8, 52u8, 51u8],
            [183u8, 240u8, 110u8, 190u8],
            [187u8, 69u8, 254u8, 242u8],
            [191u8, 174u8, 63u8, 210u8],
            [196u8, 72u8, 254u8, 184u8],
            [201u8, 120u8, 247u8, 172u8],
            [202u8, 138u8, 167u8, 199u8],
            [205u8, 109u8, 198u8, 135u8],
            [218u8, 139u8, 232u8, 100u8],
            [228u8, 204u8, 63u8, 144u8],
            [238u8, 169u8, 6u8, 75u8],
            [240u8, 224u8, 230u8, 118u8],
            [242u8, 253u8, 227u8, 139u8],
            [246u8, 152u8, 218u8, 37u8],
            [250u8, 188u8, 28u8, 188u8],
            [253u8, 138u8, 168u8, 141u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for DelegationManagerCalls {
        const NAME: &'static str = "DelegationManagerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 52usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::DELEGATION_APPROVAL_TYPEHASH(_) => {
                    <DELEGATION_APPROVAL_TYPEHASHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::allocationManager(_) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::beaconChainETHStrategy(_) => {
                    <beaconChainETHStrategyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calculateDelegationApprovalDigestHash(_) => {
                    <calculateDelegationApprovalDigestHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calculateWithdrawalRoot(_) => {
                    <calculateWithdrawalRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::completeQueuedWithdrawal(_) => {
                    <completeQueuedWithdrawalCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::completeQueuedWithdrawals(_) => {
                    <completeQueuedWithdrawalsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::convertToDepositShares(_) => {
                    <convertToDepositSharesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::cumulativeWithdrawalsQueued(_) => {
                    <cumulativeWithdrawalsQueuedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::decreaseDelegatedShares(_) => {
                    <decreaseDelegatedSharesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegateTo(_) => {
                    <delegateToCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegatedTo(_) => {
                    <delegatedToCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegationApprover(_) => {
                    <delegationApproverCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegationApproverSaltIsSpent(_) => {
                    <delegationApproverSaltIsSpentCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::depositScalingFactor(_) => {
                    <depositScalingFactorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::domainSeparator(_) => {
                    <domainSeparatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::eigenPodManager(_) => {
                    <eigenPodManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getDepositedShares(_) => {
                    <getDepositedSharesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorShares(_) => {
                    <getOperatorSharesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorsShares(_) => {
                    <getOperatorsSharesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getQueuedWithdrawal(_) => {
                    <getQueuedWithdrawalCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getQueuedWithdrawalRoots(_) => {
                    <getQueuedWithdrawalRootsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getQueuedWithdrawals(_) => {
                    <getQueuedWithdrawalsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getSlashableSharesInQueue(_) => {
                    <getSlashableSharesInQueueCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getWithdrawableShares(_) => {
                    <getWithdrawableSharesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::increaseDelegatedShares(_) => {
                    <increaseDelegatedSharesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isDelegated(_) => {
                    <isDelegatedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isOperator(_) => {
                    <isOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::minWithdrawalDelayBlocks(_) => {
                    <minWithdrawalDelayBlocksCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::modifyOperatorDetails(_) => {
                    <modifyOperatorDetailsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::operatorShares(_) => {
                    <operatorSharesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pause(_) => <pauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pauseAll(_) => <pauseAllCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused_0(_) => <paused_0Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused_1(_) => <paused_1Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::pauserRegistry(_) => {
                    <pauserRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::pendingWithdrawals(_) => {
                    <pendingWithdrawalsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::permissionController(_) => {
                    <permissionControllerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::queueWithdrawals(_) => {
                    <queueWithdrawalsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::queuedWithdrawals(_) => {
                    <queuedWithdrawalsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::redelegate(_) => {
                    <redelegateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registerAsOperator(_) => {
                    <registerAsOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::slashOperatorShares(_) => {
                    <slashOperatorSharesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::strategyManager(_) => {
                    <strategyManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::undelegate(_) => {
                    <undelegateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::unpause(_) => <unpauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::updateOperatorMetadataURI(_) => {
                    <updateOperatorMetadataURICall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<DelegationManagerCalls>] = &[
                {
                    fn DELEGATION_APPROVAL_TYPEHASH(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <DELEGATION_APPROVAL_TYPEHASHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::DELEGATION_APPROVAL_TYPEHASH)
                    }
                    DELEGATION_APPROVAL_TYPEHASH
                },
                {
                    fn calculateDelegationApprovalDigestHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <calculateDelegationApprovalDigestHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                DelegationManagerCalls::calculateDelegationApprovalDigestHash,
                            )
                    }
                    calculateDelegationApprovalDigestHash
                },
                {
                    fn queueWithdrawals(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <queueWithdrawalsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::queueWithdrawals)
                    }
                    queueWithdrawals
                },
                {
                    fn pause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::pause)
                    }
                    pause
                },
                {
                    fn convertToDepositShares(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <convertToDepositSharesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::convertToDepositShares)
                    }
                    convertToDepositShares
                },
                {
                    fn registerAsOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <registerAsOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::registerAsOperator)
                    }
                    registerAsOperator
                },
                {
                    fn strategyManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <strategyManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::strategyManager)
                    }
                    strategyManager
                },
                {
                    fn increaseDelegatedShares(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <increaseDelegatedSharesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::increaseDelegatedShares)
                    }
                    increaseDelegatedShares
                },
                {
                    fn delegationApprover(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <delegationApproverCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::delegationApprover)
                    }
                    delegationApprover
                },
                {
                    fn isDelegated(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <isDelegatedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::isDelegated)
                    }
                    isDelegated
                },
                {
                    fn permissionController(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <permissionControllerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::permissionController)
                    }
                    permissionController
                },
                {
                    fn eigenPodManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <eigenPodManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::eigenPodManager)
                    }
                    eigenPodManager
                },
                {
                    fn modifyOperatorDetails(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <modifyOperatorDetailsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::modifyOperatorDetails)
                    }
                    modifyOperatorDetails
                },
                {
                    fn version(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::version)
                    }
                    version
                },
                {
                    fn pauseAll(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <pauseAllCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::pauseAll)
                    }
                    pauseAll
                },
                {
                    fn calculateWithdrawalRoot(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <calculateWithdrawalRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::calculateWithdrawalRoot)
                    }
                    calculateWithdrawalRoot
                },
                {
                    fn paused_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <paused_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::paused_0)
                    }
                    paused_0
                },
                {
                    fn paused_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <paused_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::paused_1)
                    }
                    paused_1
                },
                {
                    fn getQueuedWithdrawal(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <getQueuedWithdrawalCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::getQueuedWithdrawal)
                    }
                    getQueuedWithdrawal
                },
                {
                    fn getQueuedWithdrawals(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <getQueuedWithdrawalsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::getQueuedWithdrawals)
                    }
                    getQueuedWithdrawals
                },
                {
                    fn slashOperatorShares(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <slashOperatorSharesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::slashOperatorShares)
                    }
                    slashOperatorShares
                },
                {
                    fn decreaseDelegatedShares(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <decreaseDelegatedSharesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::decreaseDelegatedShares)
                    }
                    decreaseDelegatedShares
                },
                {
                    fn delegatedTo(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <delegatedToCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::delegatedTo)
                    }
                    delegatedTo
                },
                {
                    fn getDepositedShares(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <getDepositedSharesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::getDepositedShares)
                    }
                    getDepositedShares
                },
                {
                    fn isOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <isOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::isOperator)
                    }
                    isOperator
                },
                {
                    fn getSlashableSharesInQueue(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <getSlashableSharesInQueueCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::getSlashableSharesInQueue)
                    }
                    getSlashableSharesInQueue
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn operatorShares(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <operatorSharesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::operatorShares)
                    }
                    operatorShares
                },
                {
                    fn updateOperatorMetadataURI(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <updateOperatorMetadataURICall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::updateOperatorMetadataURI)
                    }
                    updateOperatorMetadataURI
                },
                {
                    fn pauserRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <pauserRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::pauserRegistry)
                    }
                    pauserRegistry
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::owner)
                    }
                    owner
                },
                {
                    fn getOperatorShares(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <getOperatorSharesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::getOperatorShares)
                    }
                    getOperatorShares
                },
                {
                    fn beaconChainETHStrategy(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <beaconChainETHStrategyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::beaconChainETHStrategy)
                    }
                    beaconChainETHStrategy
                },
                {
                    fn completeQueuedWithdrawals(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <completeQueuedWithdrawalsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::completeQueuedWithdrawals)
                    }
                    completeQueuedWithdrawals
                },
                {
                    fn queuedWithdrawals(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <queuedWithdrawalsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::queuedWithdrawals)
                    }
                    queuedWithdrawals
                },
                {
                    fn cumulativeWithdrawalsQueued(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <cumulativeWithdrawalsQueuedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::cumulativeWithdrawalsQueued)
                    }
                    cumulativeWithdrawalsQueued
                },
                {
                    fn redelegate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <redelegateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::redelegate)
                    }
                    redelegate
                },
                {
                    fn pendingWithdrawals(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <pendingWithdrawalsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::pendingWithdrawals)
                    }
                    pendingWithdrawals
                },
                {
                    fn delegationApproverSaltIsSpent(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <delegationApproverSaltIsSpentCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::delegationApproverSaltIsSpent)
                    }
                    delegationApproverSaltIsSpent
                },
                {
                    fn depositScalingFactor(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <depositScalingFactorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::depositScalingFactor)
                    }
                    depositScalingFactor
                },
                {
                    fn minWithdrawalDelayBlocks(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <minWithdrawalDelayBlocksCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::minWithdrawalDelayBlocks)
                    }
                    minWithdrawalDelayBlocks
                },
                {
                    fn getWithdrawableShares(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <getWithdrawableSharesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::getWithdrawableShares)
                    }
                    getWithdrawableShares
                },
                {
                    fn allocationManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <allocationManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::allocationManager)
                    }
                    allocationManager
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::initialize)
                    }
                    initialize
                },
                {
                    fn undelegate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <undelegateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::undelegate)
                    }
                    undelegate
                },
                {
                    fn completeQueuedWithdrawal(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <completeQueuedWithdrawalCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::completeQueuedWithdrawal)
                    }
                    completeQueuedWithdrawal
                },
                {
                    fn delegateTo(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <delegateToCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::delegateTo)
                    }
                    delegateTo
                },
                {
                    fn getOperatorsShares(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <getOperatorsSharesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::getOperatorsShares)
                    }
                    getOperatorsShares
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn domainSeparator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <domainSeparatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::domainSeparator)
                    }
                    domainSeparator
                },
                {
                    fn unpause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::unpause)
                    }
                    unpause
                },
                {
                    fn getQueuedWithdrawalRoots(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <getQueuedWithdrawalRootsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::getQueuedWithdrawalRoots)
                    }
                    getQueuedWithdrawalRoots
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
                Self::DELEGATION_APPROVAL_TYPEHASH(inner) => {
                    <DELEGATION_APPROVAL_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::allocationManager(inner) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::beaconChainETHStrategy(inner) => {
                    <beaconChainETHStrategyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::calculateDelegationApprovalDigestHash(inner) => {
                    <calculateDelegationApprovalDigestHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::calculateWithdrawalRoot(inner) => {
                    <calculateWithdrawalRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::completeQueuedWithdrawal(inner) => {
                    <completeQueuedWithdrawalCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::completeQueuedWithdrawals(inner) => {
                    <completeQueuedWithdrawalsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::convertToDepositShares(inner) => {
                    <convertToDepositSharesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::cumulativeWithdrawalsQueued(inner) => {
                    <cumulativeWithdrawalsQueuedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::decreaseDelegatedShares(inner) => {
                    <decreaseDelegatedSharesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delegateTo(inner) => {
                    <delegateToCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::delegatedTo(inner) => {
                    <delegatedToCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delegationApprover(inner) => {
                    <delegationApproverCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delegationApproverSaltIsSpent(inner) => {
                    <delegationApproverSaltIsSpentCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::depositScalingFactor(inner) => {
                    <depositScalingFactorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::domainSeparator(inner) => {
                    <domainSeparatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::eigenPodManager(inner) => {
                    <eigenPodManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getDepositedShares(inner) => {
                    <getDepositedSharesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorShares(inner) => {
                    <getOperatorSharesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorsShares(inner) => {
                    <getOperatorsSharesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getQueuedWithdrawal(inner) => {
                    <getQueuedWithdrawalCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getQueuedWithdrawalRoots(inner) => {
                    <getQueuedWithdrawalRootsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getQueuedWithdrawals(inner) => {
                    <getQueuedWithdrawalsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getSlashableSharesInQueue(inner) => {
                    <getSlashableSharesInQueueCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getWithdrawableShares(inner) => {
                    <getWithdrawableSharesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::increaseDelegatedShares(inner) => {
                    <increaseDelegatedSharesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isDelegated(inner) => {
                    <isDelegatedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isOperator(inner) => {
                    <isOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::minWithdrawalDelayBlocks(inner) => {
                    <minWithdrawalDelayBlocksCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::modifyOperatorDetails(inner) => {
                    <modifyOperatorDetailsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::operatorShares(inner) => {
                    <operatorSharesCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::pendingWithdrawals(inner) => {
                    <pendingWithdrawalsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::permissionController(inner) => {
                    <permissionControllerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::queueWithdrawals(inner) => {
                    <queueWithdrawalsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::queuedWithdrawals(inner) => {
                    <queuedWithdrawalsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::redelegate(inner) => {
                    <redelegateCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::registerAsOperator(inner) => {
                    <registerAsOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::slashOperatorShares(inner) => {
                    <slashOperatorSharesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::strategyManager(inner) => {
                    <strategyManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::undelegate(inner) => {
                    <undelegateCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::updateOperatorMetadataURI(inner) => {
                    <updateOperatorMetadataURICall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::DELEGATION_APPROVAL_TYPEHASH(inner) => {
                    <DELEGATION_APPROVAL_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::beaconChainETHStrategy(inner) => {
                    <beaconChainETHStrategyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::calculateDelegationApprovalDigestHash(inner) => {
                    <calculateDelegationApprovalDigestHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::calculateWithdrawalRoot(inner) => {
                    <calculateWithdrawalRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::completeQueuedWithdrawal(inner) => {
                    <completeQueuedWithdrawalCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::completeQueuedWithdrawals(inner) => {
                    <completeQueuedWithdrawalsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::convertToDepositShares(inner) => {
                    <convertToDepositSharesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::cumulativeWithdrawalsQueued(inner) => {
                    <cumulativeWithdrawalsQueuedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::decreaseDelegatedShares(inner) => {
                    <decreaseDelegatedSharesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delegateTo(inner) => {
                    <delegateToCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delegatedTo(inner) => {
                    <delegatedToCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delegationApprover(inner) => {
                    <delegationApproverCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delegationApproverSaltIsSpent(inner) => {
                    <delegationApproverSaltIsSpentCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::depositScalingFactor(inner) => {
                    <depositScalingFactorCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::eigenPodManager(inner) => {
                    <eigenPodManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getDepositedShares(inner) => {
                    <getDepositedSharesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorShares(inner) => {
                    <getOperatorSharesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorsShares(inner) => {
                    <getOperatorsSharesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getQueuedWithdrawal(inner) => {
                    <getQueuedWithdrawalCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getQueuedWithdrawalRoots(inner) => {
                    <getQueuedWithdrawalRootsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getQueuedWithdrawals(inner) => {
                    <getQueuedWithdrawalsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getSlashableSharesInQueue(inner) => {
                    <getSlashableSharesInQueueCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getWithdrawableShares(inner) => {
                    <getWithdrawableSharesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::increaseDelegatedShares(inner) => {
                    <increaseDelegatedSharesCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::isDelegated(inner) => {
                    <isDelegatedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isOperator(inner) => {
                    <isOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::minWithdrawalDelayBlocks(inner) => {
                    <minWithdrawalDelayBlocksCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::modifyOperatorDetails(inner) => {
                    <modifyOperatorDetailsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::operatorShares(inner) => {
                    <operatorSharesCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::pendingWithdrawals(inner) => {
                    <pendingWithdrawalsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::permissionController(inner) => {
                    <permissionControllerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::queueWithdrawals(inner) => {
                    <queueWithdrawalsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::queuedWithdrawals(inner) => {
                    <queuedWithdrawalsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::redelegate(inner) => {
                    <redelegateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::registerAsOperator(inner) => {
                    <registerAsOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::slashOperatorShares(inner) => {
                    <slashOperatorSharesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::strategyManager(inner) => {
                    <strategyManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::undelegate(inner) => {
                    <undelegateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::updateOperatorMetadataURI(inner) => {
                    <updateOperatorMetadataURICall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`DelegationManager`](self) custom errors.
    pub enum DelegationManagerErrors {
        #[allow(missing_docs)]
        ActivelyDelegated(ActivelyDelegated),
        #[allow(missing_docs)]
        CallerCannotUndelegate(CallerCannotUndelegate),
        #[allow(missing_docs)]
        CurrentlyPaused(CurrentlyPaused),
        #[allow(missing_docs)]
        FullySlashed(FullySlashed),
        #[allow(missing_docs)]
        InputAddressZero(InputAddressZero),
        #[allow(missing_docs)]
        InputArrayLengthMismatch(InputArrayLengthMismatch),
        #[allow(missing_docs)]
        InputArrayLengthZero(InputArrayLengthZero),
        #[allow(missing_docs)]
        InvalidNewPausedStatus(InvalidNewPausedStatus),
        #[allow(missing_docs)]
        InvalidPermissions(InvalidPermissions),
        #[allow(missing_docs)]
        InvalidShortString(InvalidShortString),
        #[allow(missing_docs)]
        InvalidSignature(InvalidSignature),
        #[allow(missing_docs)]
        InvalidSnapshotOrdering(InvalidSnapshotOrdering),
        #[allow(missing_docs)]
        NotActivelyDelegated(NotActivelyDelegated),
        #[allow(missing_docs)]
        OnlyAllocationManager(OnlyAllocationManager),
        #[allow(missing_docs)]
        OnlyEigenPodManager(OnlyEigenPodManager),
        #[allow(missing_docs)]
        OnlyPauser(OnlyPauser),
        #[allow(missing_docs)]
        OnlyStrategyManagerOrEigenPodManager(OnlyStrategyManagerOrEigenPodManager),
        #[allow(missing_docs)]
        OnlyUnpauser(OnlyUnpauser),
        #[allow(missing_docs)]
        OperatorNotRegistered(OperatorNotRegistered),
        #[allow(missing_docs)]
        OperatorsCannotUndelegate(OperatorsCannotUndelegate),
        #[allow(missing_docs)]
        SaltSpent(SaltSpent),
        #[allow(missing_docs)]
        SignatureExpired(SignatureExpired),
        #[allow(missing_docs)]
        StringTooLong(StringTooLong),
        #[allow(missing_docs)]
        WithdrawalDelayNotElapsed(WithdrawalDelayNotElapsed),
        #[allow(missing_docs)]
        WithdrawalNotQueued(WithdrawalNotQueued),
        #[allow(missing_docs)]
        WithdrawerNotCaller(WithdrawerNotCaller),
    }
    #[automatically_derived]
    impl DelegationManagerErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [8u8, 25u8, 189u8, 205u8],
            [17u8, 72u8, 26u8, 148u8],
            [35u8, 216u8, 113u8, 165u8],
            [37u8, 236u8, 108u8, 31u8],
            [40u8, 206u8, 241u8, 164u8],
            [42u8, 55u8, 28u8, 126u8],
            [48u8, 90u8, 39u8, 169u8],
            [53u8, 49u8, 50u8, 68u8],
            [60u8, 147u8, 52u8, 70u8],
            [67u8, 113u8, 74u8, 253u8],
            [88u8, 68u8, 52u8, 212u8],
            [115u8, 99u8, 33u8, 118u8],
            [117u8, 223u8, 81u8, 220u8],
            [119u8, 229u8, 106u8, 6u8],
            [121u8, 72u8, 33u8, 255u8],
            [121u8, 108u8, 197u8, 37u8],
            [132u8, 10u8, 72u8, 213u8],
            [135u8, 201u8, 210u8, 25u8],
            [139u8, 170u8, 87u8, 159u8],
            [142u8, 81u8, 153u8, 168u8],
            [147u8, 45u8, 148u8, 247u8],
            [165u8, 199u8, 196u8, 69u8],
            [179u8, 81u8, 43u8, 12u8],
            [198u8, 29u8, 202u8, 93u8],
            [200u8, 78u8, 153u8, 132u8],
            [241u8, 236u8, 245u8, 194u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for DelegationManagerErrors {
        const NAME: &'static str = "DelegationManagerErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 26usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::ActivelyDelegated(_) => {
                    <ActivelyDelegated as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CallerCannotUndelegate(_) => {
                    <CallerCannotUndelegate as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CurrentlyPaused(_) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FullySlashed(_) => {
                    <FullySlashed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputAddressZero(_) => {
                    <InputAddressZero as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputArrayLengthMismatch(_) => {
                    <InputArrayLengthMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputArrayLengthZero(_) => {
                    <InputArrayLengthZero as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidNewPausedStatus(_) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidPermissions(_) => {
                    <InvalidPermissions as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidShortString(_) => {
                    <InvalidShortString as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidSignature(_) => {
                    <InvalidSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidSnapshotOrdering(_) => {
                    <InvalidSnapshotOrdering as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotActivelyDelegated(_) => {
                    <NotActivelyDelegated as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyAllocationManager(_) => {
                    <OnlyAllocationManager as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyEigenPodManager(_) => {
                    <OnlyEigenPodManager as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyPauser(_) => {
                    <OnlyPauser as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyStrategyManagerOrEigenPodManager(_) => {
                    <OnlyStrategyManagerOrEigenPodManager as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyUnpauser(_) => {
                    <OnlyUnpauser as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OperatorNotRegistered(_) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OperatorsCannotUndelegate(_) => {
                    <OperatorsCannotUndelegate as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SaltSpent(_) => <SaltSpent as alloy_sol_types::SolError>::SELECTOR,
                Self::SignatureExpired(_) => {
                    <SignatureExpired as alloy_sol_types::SolError>::SELECTOR
                }
                Self::StringTooLong(_) => {
                    <StringTooLong as alloy_sol_types::SolError>::SELECTOR
                }
                Self::WithdrawalDelayNotElapsed(_) => {
                    <WithdrawalDelayNotElapsed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::WithdrawalNotQueued(_) => {
                    <WithdrawalNotQueued as alloy_sol_types::SolError>::SELECTOR
                }
                Self::WithdrawerNotCaller(_) => {
                    <WithdrawerNotCaller as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<DelegationManagerErrors>] = &[
                {
                    fn SignatureExpired(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <SignatureExpired as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::SignatureExpired)
                    }
                    SignatureExpired
                },
                {
                    fn OnlyStrategyManagerOrEigenPodManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <OnlyStrategyManagerOrEigenPodManager as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                DelegationManagerErrors::OnlyStrategyManagerOrEigenPodManager,
                            )
                    }
                    OnlyStrategyManagerOrEigenPodManager
                },
                {
                    fn OnlyAllocationManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <OnlyAllocationManager as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::OnlyAllocationManager)
                    }
                    OnlyAllocationManager
                },
                {
                    fn OperatorNotRegistered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <OperatorNotRegistered as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::OperatorNotRegistered)
                    }
                    OperatorNotRegistered
                },
                {
                    fn FullySlashed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <FullySlashed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::FullySlashed)
                    }
                    FullySlashed
                },
                {
                    fn InvalidSnapshotOrdering(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <InvalidSnapshotOrdering as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::InvalidSnapshotOrdering)
                    }
                    InvalidSnapshotOrdering
                },
                {
                    fn StringTooLong(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <StringTooLong as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::StringTooLong)
                    }
                    StringTooLong
                },
                {
                    fn SaltSpent(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <SaltSpent as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::SaltSpent)
                    }
                    SaltSpent
                },
                {
                    fn CallerCannotUndelegate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <CallerCannotUndelegate as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::CallerCannotUndelegate)
                    }
                    CallerCannotUndelegate
                },
                {
                    fn InputArrayLengthMismatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <InputArrayLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::InputArrayLengthMismatch)
                    }
                    InputArrayLengthMismatch
                },
                {
                    fn WithdrawerNotCaller(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <WithdrawerNotCaller as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::WithdrawerNotCaller)
                    }
                    WithdrawerNotCaller
                },
                {
                    fn InputAddressZero(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <InputAddressZero as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::InputAddressZero)
                    }
                    InputAddressZero
                },
                {
                    fn OnlyPauser(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <OnlyPauser as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::OnlyPauser)
                    }
                    OnlyPauser
                },
                {
                    fn ActivelyDelegated(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <ActivelyDelegated as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::ActivelyDelegated)
                    }
                    ActivelyDelegated
                },
                {
                    fn OnlyUnpauser(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <OnlyUnpauser as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::OnlyUnpauser)
                    }
                    OnlyUnpauser
                },
                {
                    fn InputArrayLengthZero(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <InputArrayLengthZero as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::InputArrayLengthZero)
                    }
                    InputArrayLengthZero
                },
                {
                    fn CurrentlyPaused(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <CurrentlyPaused as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::CurrentlyPaused)
                    }
                    CurrentlyPaused
                },
                {
                    fn WithdrawalNotQueued(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <WithdrawalNotQueued as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::WithdrawalNotQueued)
                    }
                    WithdrawalNotQueued
                },
                {
                    fn InvalidSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <InvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::InvalidSignature)
                    }
                    InvalidSignature
                },
                {
                    fn OperatorsCannotUndelegate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <OperatorsCannotUndelegate as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::OperatorsCannotUndelegate)
                    }
                    OperatorsCannotUndelegate
                },
                {
                    fn InvalidPermissions(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <InvalidPermissions as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::InvalidPermissions)
                    }
                    InvalidPermissions
                },
                {
                    fn NotActivelyDelegated(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <NotActivelyDelegated as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::NotActivelyDelegated)
                    }
                    NotActivelyDelegated
                },
                {
                    fn InvalidShortString(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <InvalidShortString as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::InvalidShortString)
                    }
                    InvalidShortString
                },
                {
                    fn InvalidNewPausedStatus(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::InvalidNewPausedStatus)
                    }
                    InvalidNewPausedStatus
                },
                {
                    fn OnlyEigenPodManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <OnlyEigenPodManager as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::OnlyEigenPodManager)
                    }
                    OnlyEigenPodManager
                },
                {
                    fn WithdrawalDelayNotElapsed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerErrors> {
                        <WithdrawalDelayNotElapsed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerErrors::WithdrawalDelayNotElapsed)
                    }
                    WithdrawalDelayNotElapsed
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
                Self::ActivelyDelegated(inner) => {
                    <ActivelyDelegated as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CallerCannotUndelegate(inner) => {
                    <CallerCannotUndelegate as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CurrentlyPaused(inner) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FullySlashed(inner) => {
                    <FullySlashed as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InputAddressZero(inner) => {
                    <InputAddressZero as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InputArrayLengthMismatch(inner) => {
                    <InputArrayLengthMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InputArrayLengthZero(inner) => {
                    <InputArrayLengthZero as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidNewPausedStatus(inner) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidPermissions(inner) => {
                    <InvalidPermissions as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::InvalidSnapshotOrdering(inner) => {
                    <InvalidSnapshotOrdering as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotActivelyDelegated(inner) => {
                    <NotActivelyDelegated as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlyAllocationManager(inner) => {
                    <OnlyAllocationManager as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlyEigenPodManager(inner) => {
                    <OnlyEigenPodManager as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlyPauser(inner) => {
                    <OnlyPauser as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyStrategyManagerOrEigenPodManager(inner) => {
                    <OnlyStrategyManagerOrEigenPodManager as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlyUnpauser(inner) => {
                    <OnlyUnpauser as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OperatorNotRegistered(inner) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OperatorsCannotUndelegate(inner) => {
                    <OperatorsCannotUndelegate as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SaltSpent(inner) => {
                    <SaltSpent as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::SignatureExpired(inner) => {
                    <SignatureExpired as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::StringTooLong(inner) => {
                    <StringTooLong as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::WithdrawalDelayNotElapsed(inner) => {
                    <WithdrawalDelayNotElapsed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::WithdrawalNotQueued(inner) => {
                    <WithdrawalNotQueued as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::WithdrawerNotCaller(inner) => {
                    <WithdrawerNotCaller as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::ActivelyDelegated(inner) => {
                    <ActivelyDelegated as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CallerCannotUndelegate(inner) => {
                    <CallerCannotUndelegate as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::FullySlashed(inner) => {
                    <FullySlashed as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::InputArrayLengthMismatch(inner) => {
                    <InputArrayLengthMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InputArrayLengthZero(inner) => {
                    <InputArrayLengthZero as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidNewPausedStatus(inner) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidPermissions(inner) => {
                    <InvalidPermissions as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::InvalidSnapshotOrdering(inner) => {
                    <InvalidSnapshotOrdering as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotActivelyDelegated(inner) => {
                    <NotActivelyDelegated as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OnlyAllocationManager(inner) => {
                    <OnlyAllocationManager as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OnlyEigenPodManager(inner) => {
                    <OnlyEigenPodManager as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OnlyPauser(inner) => {
                    <OnlyPauser as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OnlyStrategyManagerOrEigenPodManager(inner) => {
                    <OnlyStrategyManagerOrEigenPodManager as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
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
                Self::OperatorsCannotUndelegate(inner) => {
                    <OperatorsCannotUndelegate as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SaltSpent(inner) => {
                    <SaltSpent as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
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
                Self::WithdrawalDelayNotElapsed(inner) => {
                    <WithdrawalDelayNotElapsed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::WithdrawalNotQueued(inner) => {
                    <WithdrawalNotQueued as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::WithdrawerNotCaller(inner) => {
                    <WithdrawerNotCaller as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`DelegationManager`](self) events.
    pub enum DelegationManagerEvents {
        #[allow(missing_docs)]
        DelegationApproverUpdated(DelegationApproverUpdated),
        #[allow(missing_docs)]
        DepositScalingFactorUpdated(DepositScalingFactorUpdated),
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        OperatorMetadataURIUpdated(OperatorMetadataURIUpdated),
        #[allow(missing_docs)]
        OperatorRegistered(OperatorRegistered),
        #[allow(missing_docs)]
        OperatorSharesDecreased(OperatorSharesDecreased),
        #[allow(missing_docs)]
        OperatorSharesIncreased(OperatorSharesIncreased),
        #[allow(missing_docs)]
        OperatorSharesSlashed(OperatorSharesSlashed),
        #[allow(missing_docs)]
        OwnershipTransferred(OwnershipTransferred),
        #[allow(missing_docs)]
        Paused(Paused),
        #[allow(missing_docs)]
        SlashingWithdrawalCompleted(SlashingWithdrawalCompleted),
        #[allow(missing_docs)]
        SlashingWithdrawalQueued(SlashingWithdrawalQueued),
        #[allow(missing_docs)]
        StakerDelegated(StakerDelegated),
        #[allow(missing_docs)]
        StakerForceUndelegated(StakerForceUndelegated),
        #[allow(missing_docs)]
        StakerUndelegated(StakerUndelegated),
        #[allow(missing_docs)]
        Unpaused(Unpaused),
    }
    #[automatically_derived]
    impl DelegationManagerEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                2u8,
                169u8,
                25u8,
                237u8,
                14u8,
                42u8,
                202u8,
                209u8,
                221u8,
                144u8,
                241u8,
                126u8,
                242u8,
                250u8,
                74u8,
                229u8,
                70u8,
                46u8,
                225u8,
                51u8,
                145u8,
                112u8,
                3u8,
                74u8,
                133u8,
                49u8,
                204u8,
                164u8,
                182u8,
                112u8,
                128u8,
                144u8,
            ],
            [
                30u8,
                192u8,
                66u8,
                201u8,
                101u8,
                226u8,
                237u8,
                215u8,
                16u8,
                123u8,
                81u8,
                24u8,
                142u8,
                224u8,
                243u8,
                131u8,
                226u8,
                46u8,
                118u8,
                23u8,
                144u8,
                65u8,
                171u8,
                58u8,
                157u8,
                24u8,
                255u8,
                21u8,
                20u8,
                5u8,
                22u8,
                108u8,
            ],
            [
                31u8,
                64u8,
                64u8,
                8u8,
                137u8,
                39u8,
                78u8,
                208u8,
                123u8,
                36u8,
                132u8,
                94u8,
                80u8,
                84u8,
                168u8,
                122u8,
                12u8,
                171u8,
                150u8,
                158u8,
                177u8,
                39u8,
                122u8,
                175u8,
                230u8,
                26u8,
                227u8,
                82u8,
                231u8,
                195u8,
                42u8,
                0u8,
            ],
            [
                38u8,
                178u8,
                170u8,
                226u8,
                101u8,
                22u8,
                232u8,
                113u8,
                158u8,
                245u8,
                14u8,
                162u8,
                246u8,
                131u8,
                26u8,
                46u8,
                251u8,
                212u8,
                227u8,
                125u8,
                204u8,
                223u8,
                15u8,
                105u8,
                54u8,
                178u8,
                123u8,
                192u8,
                142u8,
                121u8,
                62u8,
                48u8,
            ],
            [
                53u8,
                130u8,
                209u8,
                130u8,
                142u8,
                38u8,
                191u8,
                86u8,
                189u8,
                128u8,
                21u8,
                2u8,
                188u8,
                2u8,
                26u8,
                192u8,
                188u8,
                138u8,
                251u8,
                87u8,
                200u8,
                38u8,
                228u8,
                152u8,
                107u8,
                69u8,
                89u8,
                60u8,
                143u8,
                173u8,
                56u8,
                156u8,
            ],
            [
                105u8,
                9u8,
                96u8,
                0u8,
                55u8,
                183u8,
                93u8,
                123u8,
                71u8,
                51u8,
                174u8,
                221u8,
                129u8,
                84u8,
                66u8,
                181u8,
                236u8,
                1u8,
                138u8,
                130u8,
                119u8,
                81u8,
                200u8,
                50u8,
                170u8,
                255u8,
                100u8,
                235u8,
                165u8,
                214u8,
                210u8,
                221u8,
            ],
            [
                119u8,
                59u8,
                84u8,
                192u8,
                77u8,
                117u8,
                111u8,
                204u8,
                94u8,
                103u8,
                129u8,
                17u8,
                247u8,
                215u8,
                48u8,
                222u8,
                59u8,
                233u8,
                129u8,
                146u8,
                0u8,
                7u8,
                153u8,
                238u8,
                227u8,
                214u8,
                55u8,
                22u8,
                5u8,
                90u8,
                135u8,
                198u8,
            ],
            [
                127u8,
                38u8,
                184u8,
                63u8,
                249u8,
                110u8,
                31u8,
                43u8,
                106u8,
                104u8,
                47u8,
                19u8,
                56u8,
                82u8,
                246u8,
                121u8,
                138u8,
                9u8,
                196u8,
                101u8,
                218u8,
                149u8,
                146u8,
                20u8,
                96u8,
                206u8,
                251u8,
                56u8,
                71u8,
                64u8,
                36u8,
                152u8,
            ],
            [
                139u8,
                224u8,
                7u8,
                156u8,
                83u8,
                22u8,
                89u8,
                20u8,
                19u8,
                68u8,
                205u8,
                31u8,
                208u8,
                164u8,
                242u8,
                132u8,
                25u8,
                73u8,
                127u8,
                151u8,
                34u8,
                163u8,
                218u8,
                175u8,
                227u8,
                180u8,
                24u8,
                111u8,
                107u8,
                100u8,
                87u8,
                224u8,
            ],
            [
                139u8,
                233u8,
                50u8,
                186u8,
                197u8,
                69u8,
                97u8,
                242u8,
                114u8,
                96u8,
                249u8,
                84u8,
                99u8,
                217u8,
                184u8,
                171u8,
                55u8,
                224u8,
                107u8,
                40u8,
                66u8,
                229u8,
                238u8,
                36u8,
                4u8,
                21u8,
                124u8,
                193u8,
                61u8,
                246u8,
                235u8,
                143u8,
            ],
            [
                164u8,
                83u8,
                219u8,
                97u8,
                42u8,
                245u8,
                158u8,
                85u8,
                33u8,
                214u8,
                171u8,
                146u8,
                132u8,
                220u8,
                62u8,
                45u8,
                6u8,
                175u8,
                40u8,
                110u8,
                177u8,
                177u8,
                183u8,
                183u8,
                113u8,
                252u8,
                228u8,
                113u8,
                108u8,
                25u8,
                242u8,
                193u8,
            ],
            [
                171u8,
                64u8,
                163u8,
                116u8,
                188u8,
                81u8,
                222u8,
                55u8,
                34u8,
                0u8,
                168u8,
                188u8,
                152u8,
                26u8,
                248u8,
                201u8,
                236u8,
                220u8,
                8u8,
                223u8,
                218u8,
                239u8,
                11u8,
                182u8,
                224u8,
                159u8,
                136u8,
                243u8,
                198u8,
                22u8,
                239u8,
                61u8,
            ],
            [
                195u8,
                238u8,
                159u8,
                46u8,
                95u8,
                218u8,
                152u8,
                232u8,
                6u8,
                106u8,
                31u8,
                116u8,
                91u8,
                45u8,
                249u8,
                40u8,
                95u8,
                65u8,
                111u8,
                233u8,
                140u8,
                242u8,
                85u8,
                156u8,
                210u8,
                20u8,
                132u8,
                179u8,
                216u8,
                116u8,
                51u8,
                4u8,
            ],
            [
                221u8,
                97u8,
                31u8,
                78u8,
                246u8,
                63u8,
                67u8,
                133u8,
                241u8,
                117u8,
                108u8,
                134u8,
                206u8,
                31u8,
                31u8,
                56u8,
                154u8,
                144u8,
                19u8,
                186u8,
                111u8,
                160u8,
                125u8,
                171u8,
                168u8,
                82u8,
                130u8,
                145u8,
                188u8,
                45u8,
                60u8,
                48u8,
            ],
            [
                240u8,
                237u8,
                223u8,
                7u8,
                230u8,
                234u8,
                20u8,
                243u8,
                136u8,
                180u8,
                126u8,
                30u8,
                148u8,
                160u8,
                244u8,
                100u8,
                236u8,
                189u8,
                158u8,
                237u8,
                65u8,
                113u8,
                19u8,
                14u8,
                15u8,
                192u8,
                233u8,
                159u8,
                180u8,
                3u8,
                10u8,
                138u8,
            ],
            [
                254u8,
                227u8,
                9u8,
                102u8,
                162u8,
                86u8,
                183u8,
                30u8,
                20u8,
                188u8,
                14u8,
                191u8,
                201u8,
                67u8,
                21u8,
                226u8,
                142u8,
                244u8,
                169u8,
                122u8,
                113u8,
                49u8,
                169u8,
                226u8,
                183u8,
                163u8,
                16u8,
                167u8,
                58u8,
                244u8,
                70u8,
                118u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for DelegationManagerEvents {
        const NAME: &'static str = "DelegationManagerEvents";
        const COUNT: usize = 16usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <DelegationApproverUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <DelegationApproverUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::DelegationApproverUpdated)
                }
                Some(
                    <DepositScalingFactorUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <DepositScalingFactorUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::DepositScalingFactorUpdated)
                }
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Initialized)
                }
                Some(
                    <OperatorMetadataURIUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorMetadataURIUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorMetadataURIUpdated)
                }
                Some(
                    <OperatorRegistered as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorRegistered as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorRegistered)
                }
                Some(
                    <OperatorSharesDecreased as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorSharesDecreased as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorSharesDecreased)
                }
                Some(
                    <OperatorSharesIncreased as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorSharesIncreased as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorSharesIncreased)
                }
                Some(
                    <OperatorSharesSlashed as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorSharesSlashed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorSharesSlashed)
                }
                Some(
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OwnershipTransferred)
                }
                Some(<Paused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Paused as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Paused)
                }
                Some(
                    <SlashingWithdrawalCompleted as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <SlashingWithdrawalCompleted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::SlashingWithdrawalCompleted)
                }
                Some(
                    <SlashingWithdrawalQueued as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <SlashingWithdrawalQueued as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::SlashingWithdrawalQueued)
                }
                Some(<StakerDelegated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <StakerDelegated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::StakerDelegated)
                }
                Some(
                    <StakerForceUndelegated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <StakerForceUndelegated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::StakerForceUndelegated)
                }
                Some(
                    <StakerUndelegated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <StakerUndelegated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::StakerUndelegated)
                }
                Some(<Unpaused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Unpaused as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Unpaused)
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
    impl alloy_sol_types::private::IntoLogData for DelegationManagerEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::DelegationApproverUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::DepositScalingFactorUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorMetadataURIUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorRegistered(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorSharesDecreased(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorSharesIncreased(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorSharesSlashed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Paused(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SlashingWithdrawalCompleted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SlashingWithdrawalQueued(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::StakerDelegated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::StakerForceUndelegated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::StakerUndelegated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::DelegationApproverUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::DepositScalingFactorUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorMetadataURIUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorRegistered(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorSharesDecreased(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorSharesIncreased(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorSharesSlashed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Paused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SlashingWithdrawalCompleted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SlashingWithdrawalQueued(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::StakerDelegated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::StakerForceUndelegated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::StakerUndelegated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`DelegationManager`](self) contract instance.

See the [wrapper's documentation](`DelegationManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> DelegationManagerInstance<T, P, N> {
        DelegationManagerInstance::<T, P, N>::new(address, provider)
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
        _strategyManager: alloy::sol_types::private::Address,
        _eigenPodManager: alloy::sol_types::private::Address,
        _allocationManager: alloy::sol_types::private::Address,
        _pauserRegistry: alloy::sol_types::private::Address,
        _permissionController: alloy::sol_types::private::Address,
        _MIN_WITHDRAWAL_DELAY: u32,
        _version: alloy::sol_types::private::String,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<DelegationManagerInstance<T, P, N>>,
    > {
        DelegationManagerInstance::<
            T,
            P,
            N,
        >::deploy(
            provider,
            _strategyManager,
            _eigenPodManager,
            _allocationManager,
            _pauserRegistry,
            _permissionController,
            _MIN_WITHDRAWAL_DELAY,
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
        _strategyManager: alloy::sol_types::private::Address,
        _eigenPodManager: alloy::sol_types::private::Address,
        _allocationManager: alloy::sol_types::private::Address,
        _pauserRegistry: alloy::sol_types::private::Address,
        _permissionController: alloy::sol_types::private::Address,
        _MIN_WITHDRAWAL_DELAY: u32,
        _version: alloy::sol_types::private::String,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        DelegationManagerInstance::<
            T,
            P,
            N,
        >::deploy_builder(
            provider,
            _strategyManager,
            _eigenPodManager,
            _allocationManager,
            _pauserRegistry,
            _permissionController,
            _MIN_WITHDRAWAL_DELAY,
            _version,
        )
    }
    /**A [`DelegationManager`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`DelegationManager`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct DelegationManagerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for DelegationManagerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("DelegationManagerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > DelegationManagerInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`DelegationManager`](self) contract instance.

See the [wrapper's documentation](`DelegationManagerInstance`) for more details.*/
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
            _strategyManager: alloy::sol_types::private::Address,
            _eigenPodManager: alloy::sol_types::private::Address,
            _allocationManager: alloy::sol_types::private::Address,
            _pauserRegistry: alloy::sol_types::private::Address,
            _permissionController: alloy::sol_types::private::Address,
            _MIN_WITHDRAWAL_DELAY: u32,
            _version: alloy::sol_types::private::String,
        ) -> alloy_contract::Result<DelegationManagerInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _strategyManager,
                _eigenPodManager,
                _allocationManager,
                _pauserRegistry,
                _permissionController,
                _MIN_WITHDRAWAL_DELAY,
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
            _strategyManager: alloy::sol_types::private::Address,
            _eigenPodManager: alloy::sol_types::private::Address,
            _allocationManager: alloy::sol_types::private::Address,
            _pauserRegistry: alloy::sol_types::private::Address,
            _permissionController: alloy::sol_types::private::Address,
            _MIN_WITHDRAWAL_DELAY: u32,
            _version: alloy::sol_types::private::String,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _strategyManager,
                            _eigenPodManager,
                            _allocationManager,
                            _pauserRegistry,
                            _permissionController,
                            _MIN_WITHDRAWAL_DELAY,
                            _version,
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
    impl<T, P: ::core::clone::Clone, N> DelegationManagerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> DelegationManagerInstance<T, P, N> {
            DelegationManagerInstance {
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
    > DelegationManagerInstance<T, P, N> {
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
        ///Creates a new call builder for the [`DELEGATION_APPROVAL_TYPEHASH`] function.
        pub fn DELEGATION_APPROVAL_TYPEHASH(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, DELEGATION_APPROVAL_TYPEHASHCall, N> {
            self.call_builder(
                &DELEGATION_APPROVAL_TYPEHASHCall {
                },
            )
        }
        ///Creates a new call builder for the [`allocationManager`] function.
        pub fn allocationManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, allocationManagerCall, N> {
            self.call_builder(&allocationManagerCall {})
        }
        ///Creates a new call builder for the [`beaconChainETHStrategy`] function.
        pub fn beaconChainETHStrategy(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, beaconChainETHStrategyCall, N> {
            self.call_builder(&beaconChainETHStrategyCall {})
        }
        ///Creates a new call builder for the [`calculateDelegationApprovalDigestHash`] function.
        pub fn calculateDelegationApprovalDigestHash(
            &self,
            staker: alloy::sol_types::private::Address,
            operator: alloy::sol_types::private::Address,
            approver: alloy::sol_types::private::Address,
            approverSalt: alloy::sol_types::private::FixedBytes<32>,
            expiry: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            calculateDelegationApprovalDigestHashCall,
            N,
        > {
            self.call_builder(
                &calculateDelegationApprovalDigestHashCall {
                    staker,
                    operator,
                    approver,
                    approverSalt,
                    expiry,
                },
            )
        }
        ///Creates a new call builder for the [`calculateWithdrawalRoot`] function.
        pub fn calculateWithdrawalRoot(
            &self,
            withdrawal: <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, calculateWithdrawalRootCall, N> {
            self.call_builder(
                &calculateWithdrawalRootCall {
                    withdrawal,
                },
            )
        }
        ///Creates a new call builder for the [`completeQueuedWithdrawal`] function.
        pub fn completeQueuedWithdrawal(
            &self,
            withdrawal: <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
            tokens: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            receiveAsTokens: bool,
        ) -> alloy_contract::SolCallBuilder<T, &P, completeQueuedWithdrawalCall, N> {
            self.call_builder(
                &completeQueuedWithdrawalCall {
                    withdrawal,
                    tokens,
                    receiveAsTokens,
                },
            )
        }
        ///Creates a new call builder for the [`completeQueuedWithdrawals`] function.
        pub fn completeQueuedWithdrawals(
            &self,
            withdrawals: alloy::sol_types::private::Vec<
                <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
            >,
            tokens: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            >,
            receiveAsTokens: alloy::sol_types::private::Vec<bool>,
        ) -> alloy_contract::SolCallBuilder<T, &P, completeQueuedWithdrawalsCall, N> {
            self.call_builder(
                &completeQueuedWithdrawalsCall {
                    withdrawals,
                    tokens,
                    receiveAsTokens,
                },
            )
        }
        ///Creates a new call builder for the [`convertToDepositShares`] function.
        pub fn convertToDepositShares(
            &self,
            staker: alloy::sol_types::private::Address,
            strategies: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            withdrawableShares: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, convertToDepositSharesCall, N> {
            self.call_builder(
                &convertToDepositSharesCall {
                    staker,
                    strategies,
                    withdrawableShares,
                },
            )
        }
        ///Creates a new call builder for the [`cumulativeWithdrawalsQueued`] function.
        pub fn cumulativeWithdrawalsQueued(
            &self,
            staker: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, cumulativeWithdrawalsQueuedCall, N> {
            self.call_builder(
                &cumulativeWithdrawalsQueuedCall {
                    staker,
                },
            )
        }
        ///Creates a new call builder for the [`decreaseDelegatedShares`] function.
        pub fn decreaseDelegatedShares(
            &self,
            staker: alloy::sol_types::private::Address,
            curDepositShares: alloy::sol_types::private::primitives::aliases::U256,
            beaconChainSlashingFactorDecrease: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, decreaseDelegatedSharesCall, N> {
            self.call_builder(
                &decreaseDelegatedSharesCall {
                    staker,
                    curDepositShares,
                    beaconChainSlashingFactorDecrease,
                },
            )
        }
        ///Creates a new call builder for the [`delegateTo`] function.
        pub fn delegateTo(
            &self,
            operator: alloy::sol_types::private::Address,
            approverSignatureAndExpiry: <ISignatureUtilsMixinTypes::SignatureWithExpiry as alloy::sol_types::SolType>::RustType,
            approverSalt: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, delegateToCall, N> {
            self.call_builder(
                &delegateToCall {
                    operator,
                    approverSignatureAndExpiry,
                    approverSalt,
                },
            )
        }
        ///Creates a new call builder for the [`delegatedTo`] function.
        pub fn delegatedTo(
            &self,
            staker: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, delegatedToCall, N> {
            self.call_builder(&delegatedToCall { staker })
        }
        ///Creates a new call builder for the [`delegationApprover`] function.
        pub fn delegationApprover(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, delegationApproverCall, N> {
            self.call_builder(&delegationApproverCall { operator })
        }
        ///Creates a new call builder for the [`delegationApproverSaltIsSpent`] function.
        pub fn delegationApproverSaltIsSpent(
            &self,
            delegationApprover: alloy::sol_types::private::Address,
            salt: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            delegationApproverSaltIsSpentCall,
            N,
        > {
            self.call_builder(
                &delegationApproverSaltIsSpentCall {
                    delegationApprover,
                    salt,
                },
            )
        }
        ///Creates a new call builder for the [`depositScalingFactor`] function.
        pub fn depositScalingFactor(
            &self,
            staker: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, depositScalingFactorCall, N> {
            self.call_builder(
                &depositScalingFactorCall {
                    staker,
                    strategy,
                },
            )
        }
        ///Creates a new call builder for the [`domainSeparator`] function.
        pub fn domainSeparator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, domainSeparatorCall, N> {
            self.call_builder(&domainSeparatorCall {})
        }
        ///Creates a new call builder for the [`eigenPodManager`] function.
        pub fn eigenPodManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, eigenPodManagerCall, N> {
            self.call_builder(&eigenPodManagerCall {})
        }
        ///Creates a new call builder for the [`getDepositedShares`] function.
        pub fn getDepositedShares(
            &self,
            staker: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getDepositedSharesCall, N> {
            self.call_builder(&getDepositedSharesCall { staker })
        }
        ///Creates a new call builder for the [`getOperatorShares`] function.
        pub fn getOperatorShares(
            &self,
            operator: alloy::sol_types::private::Address,
            strategies: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorSharesCall, N> {
            self.call_builder(
                &getOperatorSharesCall {
                    operator,
                    strategies,
                },
            )
        }
        ///Creates a new call builder for the [`getOperatorsShares`] function.
        pub fn getOperatorsShares(
            &self,
            operators: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            strategies: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorsSharesCall, N> {
            self.call_builder(
                &getOperatorsSharesCall {
                    operators,
                    strategies,
                },
            )
        }
        ///Creates a new call builder for the [`getQueuedWithdrawal`] function.
        pub fn getQueuedWithdrawal(
            &self,
            withdrawalRoot: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getQueuedWithdrawalCall, N> {
            self.call_builder(
                &getQueuedWithdrawalCall {
                    withdrawalRoot,
                },
            )
        }
        ///Creates a new call builder for the [`getQueuedWithdrawalRoots`] function.
        pub fn getQueuedWithdrawalRoots(
            &self,
            staker: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getQueuedWithdrawalRootsCall, N> {
            self.call_builder(
                &getQueuedWithdrawalRootsCall {
                    staker,
                },
            )
        }
        ///Creates a new call builder for the [`getQueuedWithdrawals`] function.
        pub fn getQueuedWithdrawals(
            &self,
            staker: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getQueuedWithdrawalsCall, N> {
            self.call_builder(&getQueuedWithdrawalsCall { staker })
        }
        ///Creates a new call builder for the [`getSlashableSharesInQueue`] function.
        pub fn getSlashableSharesInQueue(
            &self,
            operator: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getSlashableSharesInQueueCall, N> {
            self.call_builder(
                &getSlashableSharesInQueueCall {
                    operator,
                    strategy,
                },
            )
        }
        ///Creates a new call builder for the [`getWithdrawableShares`] function.
        pub fn getWithdrawableShares(
            &self,
            staker: alloy::sol_types::private::Address,
            strategies: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, getWithdrawableSharesCall, N> {
            self.call_builder(
                &getWithdrawableSharesCall {
                    staker,
                    strategies,
                },
            )
        }
        ///Creates a new call builder for the [`increaseDelegatedShares`] function.
        pub fn increaseDelegatedShares(
            &self,
            staker: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
            prevDepositShares: alloy::sol_types::private::primitives::aliases::U256,
            addedShares: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, increaseDelegatedSharesCall, N> {
            self.call_builder(
                &increaseDelegatedSharesCall {
                    staker,
                    strategy,
                    prevDepositShares,
                    addedShares,
                },
            )
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            initialOwner: alloy::sol_types::private::Address,
            initialPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(
                &initializeCall {
                    initialOwner,
                    initialPausedStatus,
                },
            )
        }
        ///Creates a new call builder for the [`isDelegated`] function.
        pub fn isDelegated(
            &self,
            staker: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, isDelegatedCall, N> {
            self.call_builder(&isDelegatedCall { staker })
        }
        ///Creates a new call builder for the [`isOperator`] function.
        pub fn isOperator(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, isOperatorCall, N> {
            self.call_builder(&isOperatorCall { operator })
        }
        ///Creates a new call builder for the [`minWithdrawalDelayBlocks`] function.
        pub fn minWithdrawalDelayBlocks(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, minWithdrawalDelayBlocksCall, N> {
            self.call_builder(&minWithdrawalDelayBlocksCall {})
        }
        ///Creates a new call builder for the [`modifyOperatorDetails`] function.
        pub fn modifyOperatorDetails(
            &self,
            operator: alloy::sol_types::private::Address,
            newDelegationApprover: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, modifyOperatorDetailsCall, N> {
            self.call_builder(
                &modifyOperatorDetailsCall {
                    operator,
                    newDelegationApprover,
                },
            )
        }
        ///Creates a new call builder for the [`operatorShares`] function.
        pub fn operatorShares(
            &self,
            operator: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, operatorSharesCall, N> {
            self.call_builder(
                &operatorSharesCall {
                    operator,
                    strategy,
                },
            )
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
        pub fn pauseAll(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, pauseAllCall, N> {
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
        pub fn paused_1(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, paused_1Call, N> {
            self.call_builder(&paused_1Call {})
        }
        ///Creates a new call builder for the [`pauserRegistry`] function.
        pub fn pauserRegistry(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, pauserRegistryCall, N> {
            self.call_builder(&pauserRegistryCall {})
        }
        ///Creates a new call builder for the [`pendingWithdrawals`] function.
        pub fn pendingWithdrawals(
            &self,
            withdrawalRoot: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, pendingWithdrawalsCall, N> {
            self.call_builder(
                &pendingWithdrawalsCall {
                    withdrawalRoot,
                },
            )
        }
        ///Creates a new call builder for the [`permissionController`] function.
        pub fn permissionController(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, permissionControllerCall, N> {
            self.call_builder(&permissionControllerCall {})
        }
        ///Creates a new call builder for the [`queueWithdrawals`] function.
        pub fn queueWithdrawals(
            &self,
            params: alloy::sol_types::private::Vec<
                <IDelegationManagerTypes::QueuedWithdrawalParams as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, queueWithdrawalsCall, N> {
            self.call_builder(&queueWithdrawalsCall { params })
        }
        ///Creates a new call builder for the [`queuedWithdrawals`] function.
        pub fn queuedWithdrawals(
            &self,
            withdrawalRoot: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, queuedWithdrawalsCall, N> {
            self.call_builder(
                &queuedWithdrawalsCall {
                    withdrawalRoot,
                },
            )
        }
        ///Creates a new call builder for the [`redelegate`] function.
        pub fn redelegate(
            &self,
            newOperator: alloy::sol_types::private::Address,
            newOperatorApproverSig: <ISignatureUtilsMixinTypes::SignatureWithExpiry as alloy::sol_types::SolType>::RustType,
            approverSalt: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, redelegateCall, N> {
            self.call_builder(
                &redelegateCall {
                    newOperator,
                    newOperatorApproverSig,
                    approverSalt,
                },
            )
        }
        ///Creates a new call builder for the [`registerAsOperator`] function.
        pub fn registerAsOperator(
            &self,
            initDelegationApprover: alloy::sol_types::private::Address,
            allocationDelay: u32,
            metadataURI: alloy::sol_types::private::String,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerAsOperatorCall, N> {
            self.call_builder(
                &registerAsOperatorCall {
                    initDelegationApprover,
                    allocationDelay,
                    metadataURI,
                },
            )
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall {})
        }
        ///Creates a new call builder for the [`slashOperatorShares`] function.
        pub fn slashOperatorShares(
            &self,
            operator: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
            prevMaxMagnitude: u64,
            newMaxMagnitude: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, slashOperatorSharesCall, N> {
            self.call_builder(
                &slashOperatorSharesCall {
                    operator,
                    strategy,
                    prevMaxMagnitude,
                    newMaxMagnitude,
                },
            )
        }
        ///Creates a new call builder for the [`strategyManager`] function.
        pub fn strategyManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, strategyManagerCall, N> {
            self.call_builder(&strategyManagerCall {})
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
        ///Creates a new call builder for the [`undelegate`] function.
        pub fn undelegate(
            &self,
            staker: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, undelegateCall, N> {
            self.call_builder(&undelegateCall { staker })
        }
        ///Creates a new call builder for the [`unpause`] function.
        pub fn unpause(
            &self,
            newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, unpauseCall, N> {
            self.call_builder(&unpauseCall { newPausedStatus })
        }
        ///Creates a new call builder for the [`updateOperatorMetadataURI`] function.
        pub fn updateOperatorMetadataURI(
            &self,
            operator: alloy::sol_types::private::Address,
            metadataURI: alloy::sol_types::private::String,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateOperatorMetadataURICall, N> {
            self.call_builder(
                &updateOperatorMetadataURICall {
                    operator,
                    metadataURI,
                },
            )
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
    > DelegationManagerInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`DelegationApproverUpdated`] event.
        pub fn DelegationApproverUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, DelegationApproverUpdated, N> {
            self.event_filter::<DelegationApproverUpdated>()
        }
        ///Creates a new event filter for the [`DepositScalingFactorUpdated`] event.
        pub fn DepositScalingFactorUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, DepositScalingFactorUpdated, N> {
            self.event_filter::<DepositScalingFactorUpdated>()
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`OperatorMetadataURIUpdated`] event.
        pub fn OperatorMetadataURIUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorMetadataURIUpdated, N> {
            self.event_filter::<OperatorMetadataURIUpdated>()
        }
        ///Creates a new event filter for the [`OperatorRegistered`] event.
        pub fn OperatorRegistered_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorRegistered, N> {
            self.event_filter::<OperatorRegistered>()
        }
        ///Creates a new event filter for the [`OperatorSharesDecreased`] event.
        pub fn OperatorSharesDecreased_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorSharesDecreased, N> {
            self.event_filter::<OperatorSharesDecreased>()
        }
        ///Creates a new event filter for the [`OperatorSharesIncreased`] event.
        pub fn OperatorSharesIncreased_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorSharesIncreased, N> {
            self.event_filter::<OperatorSharesIncreased>()
        }
        ///Creates a new event filter for the [`OperatorSharesSlashed`] event.
        pub fn OperatorSharesSlashed_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorSharesSlashed, N> {
            self.event_filter::<OperatorSharesSlashed>()
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
        ///Creates a new event filter for the [`SlashingWithdrawalCompleted`] event.
        pub fn SlashingWithdrawalCompleted_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, SlashingWithdrawalCompleted, N> {
            self.event_filter::<SlashingWithdrawalCompleted>()
        }
        ///Creates a new event filter for the [`SlashingWithdrawalQueued`] event.
        pub fn SlashingWithdrawalQueued_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, SlashingWithdrawalQueued, N> {
            self.event_filter::<SlashingWithdrawalQueued>()
        }
        ///Creates a new event filter for the [`StakerDelegated`] event.
        pub fn StakerDelegated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, StakerDelegated, N> {
            self.event_filter::<StakerDelegated>()
        }
        ///Creates a new event filter for the [`StakerForceUndelegated`] event.
        pub fn StakerForceUndelegated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, StakerForceUndelegated, N> {
            self.event_filter::<StakerForceUndelegated>()
        }
        ///Creates a new event filter for the [`StakerUndelegated`] event.
        pub fn StakerUndelegated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, StakerUndelegated, N> {
            self.event_filter::<StakerUndelegated>()
        }
        ///Creates a new event filter for the [`Unpaused`] event.
        pub fn Unpaused_filter(&self) -> alloy_contract::Event<T, &P, Unpaused, N> {
            self.event_filter::<Unpaused>()
        }
    }
}
